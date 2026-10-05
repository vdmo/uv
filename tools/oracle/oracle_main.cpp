#include <cstdint>
#include <cstdio>
#include <fstream>
#include <iostream>
#include <iterator>
#include <string>
#include <string_view>
#include <vector>

#include "00_core/diagnostics.h"
#include "00_core/source_load.h"
#include "00_core/unicode.h"
#include <sys/wait.h>
#include <unistd.h>
#include <csignal>
#include <concepts>
#include <memory>
#include <optional>
#include <ostream>
#include <sstream>
#include <variant>

#include "02_source/ast/ast.h"
#include "02_source/lexer/lexer.h"
#include "02_source/parser/parser.h"
#include "03_comptime/comptime.h"
#include "00_core/symbols.h"
#include "01_project/project.h"
#include "04_analysis/resolve/resolve_items.h"
#include "04_analysis/resolve/resolver.h"
#include "04_analysis/resolve/scopes_lookup.h"
#include "04_analysis/resolve/visibility.h"
#include <algorithm>
#include <unordered_map>

using namespace ultraviolet;

namespace {

std::string Escape(std::string_view text) {
  std::string out;
  for (unsigned char c : text) {
    switch (c) {
      case '\\': out += "\\\\"; break;
      case '\t': out += "\\t"; break;
      case '\n': out += "\\n"; break;
      case '\r': out += "\\r"; break;
      default: out.push_back(static_cast<char>(c)); break;
    }
  }
  return out;
}

const char* KindName(lexer::TokenKind kind) {
  switch (kind) {
    case lexer::TokenKind::Identifier: return "Identifier";
    case lexer::TokenKind::Keyword: return "Keyword";
    case lexer::TokenKind::IntLiteral: return "IntLiteral";
    case lexer::TokenKind::FloatLiteral: return "FloatLiteral";
    case lexer::TokenKind::StringLiteral: return "StringLiteral";
    case lexer::TokenKind::CharLiteral: return "CharLiteral";
    case lexer::TokenKind::BoolLiteral: return "BoolLiteral";
    case lexer::TokenKind::NullLiteral: return "NullLiteral";
    case lexer::TokenKind::Operator: return "Operator";
    case lexer::TokenKind::Punctuator: return "Punctuator";
    case lexer::TokenKind::Newline: return "Newline";
    case lexer::TokenKind::Eof: return "Eof";
    case lexer::TokenKind::Unknown: return "Unknown";
  }
  return "?";
}

const char* SeverityName(core::Severity severity) {
  switch (severity) {
    case core::Severity::Error: return "error";
    case core::Severity::Warning: return "warning";
    case core::Severity::Info: return "info";
    case core::Severity::Panic: return "panic";
    case core::Severity::Note: return "note";
  }
  return "?";
}

void PrintSpan(const core::Span& span) {
  std::cout << span.start_offset << '\t' << span.end_offset << '\t'
            << span.start_line << '\t' << span.start_col << '\t'
            << span.end_line << '\t' << span.end_col;
}

void PrintDiags(const core::DiagnosticStream& diags) {
  for (const auto& diag : diags) {
    std::cout << "G\t" << diag.code << '\t' << SeverityName(diag.severity)
              << '\t' << Escape(diag.message) << '\t';
    if (diag.span.has_value()) {
      PrintSpan(*diag.span);
    } else {
      std::cout << "-";
    }
    std::cout << '\t';
    for (std::size_t i = 0; i < diag.obligation_ids.size(); ++i) {
      if (i != 0) std::cout << ',';
      std::cout << diag.obligation_ids[i];
    }
    std::cout << '\n';
  }
}

// ---- AST dump -------------------------------------------------------------

void D(std::ostream& o, const std::string& v) {
  o << '"';
  for (char c : v) {
    switch (c) {
      case '\\': o << "\\\\"; break;
      case '"': o << "\\\""; break;
      case '\t': o << "\\t"; break;
      case '\n': o << "\\n"; break;
      case '\r': o << "\\r"; break;
      default: o << c; break;
    }
  }
  o << '"';
}
template <std::same_as<bool> B>
void D(std::ostream& o, B v) { o << (v ? "true" : "false"); }
template <std::same_as<std::size_t> N>
void D(std::ostream& o, N v) { o << v; }
void D(std::ostream& o, const ast::TupleIndex& v) { o << ast::FormatTupleIndex(v); }
void D(std::ostream& o, const core::Span& v) {
  o << '@' << v.start_offset << ':' << v.end_offset << ':' << v.start_line << ':'
    << v.start_col << ':' << v.end_line << ':' << v.end_col;
}
void D(std::ostream& o, const lexer::Token& v) {
  o << "(Token " << KindName(v.kind) << ' ';
  D(o, v.lexeme);
  o << ' ';
  D(o, v.span);
  o << ')';
}
void D(std::ostream& o, const lexer::DocComment& v) {
  o << "(Doc " << (v.kind == lexer::DocKind::LineDoc ? "LineDoc" : "ModuleDoc") << ' ';
  D(o, v.text);
  o << ' ';
  D(o, v.span);
  o << ')';
}
template <typename T> void D(std::ostream& o, const std::optional<T>& v);
template <typename T> void D(std::ostream& o, const std::shared_ptr<T>& v);
template <typename T> void D(std::ostream& o, const std::vector<T>& v);
template <typename... Ts> void D(std::ostream& o, const std::variant<Ts...>& v);

#include "ast_dump_generated.inc"

template <typename T> void D(std::ostream& o, const std::optional<T>& v) {
  if (v.has_value()) { D(o, *v); } else { o << "none"; }
}
template <typename T> void D(std::ostream& o, const std::shared_ptr<T>& v) {
  if (v) { D(o, *v); } else { o << "none"; }
}
template <typename T> void D(std::ostream& o, const std::vector<T>& v) {
  o << '[';
  for (std::size_t i = 0; i < v.size(); ++i) {
    if (i != 0) o << ' ';
    D(o, v[i]);
  }
  o << ']';
}
template <typename... Ts> void D(std::ostream& o, const std::variant<Ts...>& v) {
  std::visit([&o](const auto& alt) { D(o, alt); }, v);
}

int DumpAst(const std::string& label, const std::string& path) {
  std::ifstream in(path, std::ios::binary);
  if (!in) {
    std::cerr << "cannot read " << path << "\n";
    return 2;
  }
  const std::vector<std::uint8_t> bytes((std::istreambuf_iterator<char>(in)),
                                        std::istreambuf_iterator<char>());
  std::cout << "F\t" << label << '\n';
  const core::SourceLoadResult loaded = core::LoadSource(label, bytes);
  PrintDiags(loaded.diags);
  if (!loaded.source.has_value()) {
    std::cout << "NOSOURCE\n";
    return 0;
  }
  const ast::ParseFileResult parsed = ast::ParseFile(*loaded.source);
  PrintDiags(parsed.diags);
  for (const auto& span : parsed.unsafe_spans) {
    std::cout << "U\t";
    PrintSpan(span);
    std::cout << '\n';
  }
  if (!parsed.file.has_value()) {
    std::cout << "NOFILE\n";
    return 0;
  }
  for (const auto& doc : parsed.file->module_doc) {
    std::cout << "M\t";
    D(std::cout, doc);
    std::cout << '\n';
  }
  for (const auto& item : parsed.file->items) {
    std::cout << "I\t";
    D(std::cout, item);
    std::cout << '\n';
  }
  return 0;
}

int DumpTokens(const std::string& label, const std::string& path) {
  std::ifstream in(path, std::ios::binary);
  if (!in) {
    std::cerr << "cannot read " << path << "\n";
    return 2;
  }
  const std::vector<std::uint8_t> bytes((std::istreambuf_iterator<char>(in)),
                                        std::istreambuf_iterator<char>());
  std::cout << "F\t" << label << '\n';
  const core::SourceLoadResult loaded = core::LoadSource(label, bytes);
  PrintDiags(loaded.diags);
  if (!loaded.source.has_value()) {
    std::cout << "NOSOURCE\n";
    return 0;
  }
  const lexer::TokenizeDiagnosticResult result =
      lexer::TokenizeWithDiagnostics(*loaded.source);
  PrintDiags(result.diags);
  if (!result.output.has_value()) {
    std::cout << "NOTOKENS\n";
    return 0;
  }
  for (const auto& token : result.output->tokens) {
    std::cout << "T\t" << KindName(token.kind) << '\t' << Escape(token.lexeme)
              << '\t';
    PrintSpan(token.span);
    std::cout << '\n';
  }
  for (const auto& doc : result.output->docs) {
    std::cout << "D\t"
              << (doc.kind == lexer::DocKind::LineDoc ? "LineDoc" : "ModuleDoc")
              << '\t' << Escape(doc.text) << '\t';
    PrintSpan(doc.span);
    std::cout << '\n';
  }
  return 0;
}

// ---- compile-time pass ------------------------------------------------------

std::vector<std::string> SplitTabs(const std::string& line) {
  std::vector<std::string> out;
  std::size_t start = 0;
  while (true) {
    const auto tab = line.find('\t', start);
    if (tab == std::string::npos) {
      out.push_back(line.substr(start));
      return out;
    }
    out.push_back(line.substr(start, tab - start));
    start = tab + 1;
  }
}

std::vector<std::string> SplitModulePath(const std::string& path) {
  std::vector<std::string> out;
  std::size_t start = 0;
  while (true) {
    const auto sep = path.find("::", start);
    if (sep == std::string::npos) {
      out.push_back(path.substr(start));
      return out;
    }
    out.push_back(path.substr(start, sep - start));
    start = sep + 2;
  }
}

// Diagnostics with the span's file and the attached notes, which the compile-time pass
// uses and the token and syntax-tree dumps do not need.
void PrintDiagsFull(const core::DiagnosticStream& diags) {
  for (const auto& diag : diags) {
    std::cout << "G\t" << diag.code << '\t' << SeverityName(diag.severity)
              << '\t' << Escape(diag.message) << '\t';
    if (diag.span.has_value()) {
      std::cout << diag.span->file << '\t';
      PrintSpan(*diag.span);
    } else {
      std::cout << "-";
    }
    std::cout << '\t';
    for (std::size_t i = 0; i < diag.obligation_ids.size(); ++i) {
      if (i != 0) std::cout << ',';
      std::cout << diag.obligation_ids[i];
    }
    std::cout << '\t' << (diag.label.has_value() ? Escape(*diag.label) : "-") << '\n';
    for (const auto& child : diag.children) {
      std::cout << "N\t" << static_cast<int>(child.kind) << '\t' << Escape(child.message)
                << '\t';
      if (child.span.has_value()) {
        std::cout << child.span->file << '\t';
        PrintSpan(*child.span);
      } else {
        std::cout << "-";
      }
      std::cout << '\t' << (child.fix_text.has_value() ? Escape(*child.fix_text) : "-")
                << '\t' << (child.label.has_value() ? Escape(*child.label) : "-") << '\n';
    }
  }
}

// One project block of a comptime list:
//   P <label> <project root> <fallback source root>
//   A <assembly name> <source root>        (repeated)
//   M <module path> <file> <file> ...      (repeated, in phase-1 order)
//   E
struct ProjectBlock {
  std::string label;
  frontend::ComptimePassOptions options;
  std::vector<ast::ASTModule> modules;
  std::size_t reachable = 0;
  std::unordered_map<std::string, std::vector<core::Span>> unsafe_spans_by_file;
  std::string failure;
};

// One project block of a project list:
//   P <label> <project root> <fallback source root>
//   A <assembly name> <source root>        (repeated)
//   R <number of leading modules reachable from the selected assembly>
//   M <module path> <file> <file> ...      (repeated, in phase-1 order)
//   E
ProjectBlock LoadProjectBlock(const std::vector<std::string>& block) {
  ProjectBlock out;
  for (const auto& line : block) {
    const auto fields = SplitTabs(line);
    if (fields[0] == "P" && fields.size() >= 4) {
      out.label = fields[1];
      out.options.project_root = fields[2];
      out.options.fallback_source_root = std::filesystem::path(fields[3]);
    } else if (fields[0] == "A" && fields.size() >= 3) {
      out.options.source_roots_by_assembly[fields[1]] = fields[2];
    } else if (fields[0] == "R" && fields.size() >= 2) {
      out.reachable = static_cast<std::size_t>(std::stoull(fields[1]));
    } else if (fields[0] == "M" && fields.size() >= 2) {
      ast::ASTModule module;
      module.path = SplitModulePath(fields[1]);
      for (std::size_t i = 2; i < fields.size(); ++i) {
        std::ifstream in(fields[i], std::ios::binary);
        const std::vector<std::uint8_t> bytes((std::istreambuf_iterator<char>(in)),
                                              std::istreambuf_iterator<char>());
        const core::SourceLoadResult loaded = core::LoadSource(fields[i], bytes);
        if (!loaded.source.has_value()) {
          out.failure = "NOSOURCE\t" + fields[i];
          return out;
        }
        ast::ParseFileResult parsed = ast::ParseFile(*loaded.source);
        if (!parsed.file.has_value()) {
          out.failure = "NOFILE\t" + fields[i];
          return out;
        }
        for (auto& item : parsed.file->items) module.items.push_back(std::move(item));
        for (auto& doc : parsed.file->module_doc) module.module_doc.push_back(std::move(doc));
        out.unsafe_spans_by_file[loaded.source->path] = std::move(parsed.unsafe_spans);
      }
      out.modules.push_back(std::move(module));
    }
  }
  return out;
}

void DumpModules(const std::vector<ast::ASTModule>& modules) {
  for (const auto& module : modules) {
    std::cout << "X\t";
    D(std::cout, module.path);
    std::cout << '\n';
    for (const auto& doc : module.module_doc) {
      std::cout << "M\t";
      D(std::cout, doc);
      std::cout << '\n';
    }
    for (const auto& item : module.items) {
      std::cout << "I\t";
      D(std::cout, item);
      std::cout << '\n';
    }
    for (const auto& proc : module.comptime_procedures) {
      std::cout << "C\t";
      D(std::cout, proc);
      std::cout << '\n';
    }
  }
}

int DumpComptime(const std::vector<std::string>& block) {
  const ProjectBlock project = LoadProjectBlock(block);
  std::cout << "F\t" << project.label << '\n';
  if (!project.failure.empty()) {
    std::cout << project.failure << '\n';
    return 0;
  }
  const frontend::ComptimeResult result =
      frontend::ExecuteComptime(project.modules, project.options);
  PrintDiagsFull(result.diags);
  if (!result.modules.has_value()) {
    std::cout << "NOMODULES\n";
    return 0;
  }
  DumpModules(*result.modules);
  return 0;
}

const char* EntityKindName(analysis::EntityKind kind) {
  switch (kind) {
    case analysis::EntityKind::Value: return "Value";
    case analysis::EntityKind::Type: return "Type";
    case analysis::EntityKind::Class: return "Class";
    case analysis::EntityKind::ModuleAlias: return "ModuleAlias";
  }
  return "?";
}

const char* EntitySourceName(analysis::EntitySource source) {
  switch (source) {
    case analysis::EntitySource::Decl: return "Decl";
    case analysis::EntitySource::Using: return "Using";
    case analysis::EntitySource::RegionAlias: return "RegionAlias";
    case analysis::EntitySource::Import: return "Import";
  }
  return "?";
}

void DumpNameMaps(const analysis::NameMapTable& table) {
  for (const auto& [module_key, name_map] : table) {
    std::cout << "NM\t";
    D(std::cout, module_key);
    std::cout << '\n';
    std::vector<const analysis::NameMap::value_type*> entries;
    entries.reserve(name_map.size());
    for (const auto& entry : name_map) entries.push_back(&entry);
    std::sort(entries.begin(), entries.end(),
              [](const auto* a, const auto* b) { return a->first < b->first; });
    for (const auto* entry : entries) {
      const analysis::Entity& ent = entry->second;
      std::cout << "NE\t";
      D(std::cout, entry->first);
      std::cout << '\t' << EntityKindName(ent.kind) << '\t' << EntitySourceName(ent.source)
                << '\t';
      D(std::cout, ent.origin_opt);
      std::cout << '\t';
      D(std::cout, ent.target_opt);
      std::cout << '\t';
      D(std::cout, ent.declaration_span);
      std::cout << '\t';
      D(std::cout, ent.language_symbol_id);
      std::cout << '\t';
      D(std::cout, ent.type_param_class_bounds);
      std::cout << '\t';
      D(std::cout, ent.visibility);
      std::cout << '\n';
    }
  }
}

bool HasErrorDiag(const core::DiagnosticStream& diags) {
  for (const auto& diag : diags) {
    if (diag.severity == core::Severity::Error) return true;
  }
  return false;
}

// Runs the reference front end on a project through name resolution, in the order the
// driver does: compile-time pass, module visibility, name maps, module resolution. The
// driver's check of compile-time procedure signatures (which needs the type checker) is
// not part of this mode.
int DumpResolve(const std::vector<std::string>& block) {
  ProjectBlock input = LoadProjectBlock(block);
  std::cout << "F\t" << input.label << '\n';
  if (!input.failure.empty()) {
    std::cout << input.failure << '\n';
    return 0;
  }
  const project::LoadProjectResult loaded =
      project::LoadProject(input.options.project_root, project::AssemblyTarget{});
  if (!loaded.project.has_value()) {
    std::cout << "NOPROJECT\n";
    return 0;
  }
  frontend::ComptimeResult comptime =
      frontend::ExecuteComptime(input.modules, input.options);
  if (!comptime.modules.has_value() || HasErrorDiag(comptime.diags)) {
    std::cout << "STOP\tcomptime\n";
    return 0;
  }
  core::DiagnosticStream diags;
  for (const auto& diag : comptime.diags) core::Emit(diags, diag);
  std::vector<ast::ASTModule> parsed_modules(
      comptime.modules->begin(),
      comptime.modules->begin() +
          static_cast<std::ptrdiff_t>(std::min(input.reachable, comptime.modules->size())));

  project::Project sema_project = *loaded.project;
  std::vector<project::ModuleInfo> reachable_infos;
  for (const auto& module : parsed_modules) {
    const std::string path = core::StringOfPath(module.path);
    for (const auto& assembly : sema_project.assemblies) {
      for (const auto& info : assembly.modules) {
        if (info.path == path) reachable_infos.push_back(info);
      }
    }
  }
  sema_project.modules = std::move(reachable_infos);

  analysis::ScopeContext ctx;
  ctx.project = &sema_project;
  ctx.target_profile = project::TargetProfile::X86_64SysV;
  ctx.sigma.mods = parsed_modules;
  ctx.sigma.unsafe_spans_by_file = input.unsafe_spans_by_file;
  ctx.scopes = {analysis::Scope{}, analysis::Scope{}, analysis::Scope{}};
  for (const auto& module : parsed_modules) {
    ctx.current_module = module.path;
    for (const auto& diag : analysis::CheckModuleVisibility(ctx, module)) {
      core::Emit(diags, diag);
    }
  }
  const auto name_maps = analysis::CollectNameMaps(ctx);
  for (const auto& diag : name_maps.diags) core::Emit(diags, diag);
  const std::size_t before_resolve = diags.size();
  PrintDiagsFull(diags);
  DumpNameMaps(name_maps.name_maps);
  if (HasErrorDiag(diags)) {
    std::cout << "STOP\tnames\n";
    return 0;
  }
  analysis::PopulateSigma(ctx);
  const auto module_names = analysis::ModuleNamesOf(sema_project);
  core::DiagnosticStream no_parse_diags;
  analysis::ResolveContext res_ctx;
  res_ctx.ctx = &ctx;
  res_ctx.name_maps = &name_maps.name_maps;
  res_ctx.module_names = &module_names;
  res_ctx.can_access = analysis::CanAccess;
  res_ctx.parse_ok = true;
  res_ctx.parse_diags = &no_parse_diags;
  const auto resolved = analysis::ResolveModules(res_ctx);
  (void)before_resolve;
  std::cout << "RESOLVE\t" << (resolved.ok ? "ok" : "failed") << '\n';
  PrintDiagsFull(resolved.diags);
  DumpModules(resolved.modules);
  return 0;
}

// The declarations analysis registers for built-in types and classes, by path.
int DumpSigma() {
  analysis::ScopeContext ctx;
  analysis::PopulateSigma(ctx);
  for (const auto& [key, decl] : ctx.sigma.types) {
    std::cout << "T\t";
    D(std::cout, key);
    std::cout << '\t';
    std::visit([](const auto& node) { D(std::cout, node); }, decl);
    std::cout << '\n';
  }
  for (const auto& [key, decl] : ctx.sigma.classes) {
    std::cout << "K\t";
    D(std::cout, key);
    std::cout << '\t';
    D(std::cout, decl);
    std::cout << '\n';
  }
  return 0;
}

std::string Utf8(std::uint32_t scalar) {
  std::string out;
  core::AppendUtf8(out, core::UnicodeScalar(scalar));
  return out;
}

int DumpUnicode() {
  for (std::uint32_t c = 0; c <= 0x10FFFFu; ++c) {
    if (c >= 0xD800u && c <= 0xDFFFu) {
      continue;
    }
    const std::string text = Utf8(c);
    const bool xid_start = core::IsXidStart(core::UnicodeScalar(c));
    const bool xid_continue = core::IsXidContinue(core::UnicodeScalar(c));
    const std::string nfc = core::NFC(text);
    const std::string fold = core::CaseFold(text);
    const core::IdentifierSecurityInfo info =
        core::AnalyzeIdentifierSecurity("a" + text);
    const bool trivial = !xid_start && !xid_continue && nfc == text &&
                         fold == text && info.skeleton == "a" + text &&
                         !info.mixed_script;
    if (trivial) {
      continue;
    }
    std::printf("%X\t%d\t%d\t%s\t%s\t%s\t%d\n", c, xid_start ? 1 : 0,
                xid_continue ? 1 : 0, Escape(nfc).c_str(), Escape(fold).c_str(),
                Escape(info.skeleton).c_str(), info.mixed_script ? 1 : 0);
  }
  return 0;
}

}  // namespace

int main(int argc, char** argv) {
  std::ios::sync_with_stdio(false);
  if (argc >= 2 && std::string_view(argv[1]) == "unicode") {
    return DumpUnicode();
  }
  if (argc >= 2 && std::string_view(argv[1]) == "sigma") {
    return DumpSigma();
  }
  const bool resolve_mode = argc >= 3 && std::string_view(argv[1]) == "resolve";
  if (resolve_mode || (argc >= 3 && std::string_view(argv[1]) == "comptime")) {
    // argv[2] is a list of project blocks; each runs in a child process.
    std::ifstream list(argv[2]);
    std::string line;
    std::vector<std::string> block;
    while (std::getline(list, line)) {
      if (line != "E") {
        block.push_back(line);
        continue;
      }
      const std::string label = block.empty() ? "" : SplitTabs(block[0])[1];
      std::cout.flush();
      const pid_t child = fork();
      if (child == 0) {
        alarm(1800);
        int rc = 0;
        try {
          rc = resolve_mode ? DumpResolve(block) : DumpComptime(block);
        } catch (const char* message) {
          std::cout << "\nCRASH\t" << message << '\n';
        }
        std::cout.flush();
        _exit(rc);
      }
      int status = 0;
      waitpid(child, &status, 0);
      if (WIFSIGNALED(status)) {
        std::cout << "F\t" << label << '\n'
                  << (WTERMSIG(status) == SIGALRM ? "HANG" : "CRASH\tsignal") << '\n';
      }
      block.clear();
    }
    return 0;
  }
  const bool ast_mode = argc >= 3 && std::string_view(argv[1]) == "ast";
  if (ast_mode || (argc >= 3 && std::string_view(argv[1]) == "tokens")) {
    // argv[2] is a list file: one "label<TAB>path" per line.
    std::ifstream list(argv[2]);
    std::string line;
    while (std::getline(list, line)) {
      const auto tab = line.find('\t');
      if (tab == std::string::npos) continue;
      // Each file runs in a child process: the reference throws a `const char*` from
      // UnicodeScalar's validating constructor on some malformed inputs, and its parser
      // does not terminate on others. The parent records CRASH or HANG and moves on.
      const std::string label = line.substr(0, tab);
      const std::string path = line.substr(tab + 1);
      std::cout.flush();
      const pid_t child = fork();
      if (child == 0) {
        alarm(3);
        int rc = 0;
        try {
          rc = ast_mode ? DumpAst(label, path) : DumpTokens(label, path);
        } catch (const char* message) {
          std::cout << "\nCRASH\t" << message << '\n';
        }
        std::cout.flush();
        _exit(rc);
      }
      int status = 0;
      waitpid(child, &status, 0);
      if (WIFSIGNALED(status)) {
        std::cout << "F\t" << label << '\n'
                  << (WTERMSIG(status) == SIGALRM ? "HANG" : "CRASH\tsignal") << '\n';
      } else if (WEXITSTATUS(status) != 0) {
        return WEXITSTATUS(status);
      }
    }
    return 0;
  }
  std::cerr << "usage: uv-oracle unicode | tokens <list-file> | ast <list-file> | comptime <list-file> | resolve <list-file>\n";
  return 2;
}
