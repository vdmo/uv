#include <cstdint>
#include <cstdlib>
#include <new>
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
#include "04_analysis/generics/monomorphize.h"
#include "04_analysis/resolve/scopes.h"
#include "04_analysis/layout/layout.h"
#include "04_analysis/typing/type_equiv.h"
#include "04_analysis/composite/enums.h"
#include "04_analysis/modal/modal_widen.h"
#include "04_analysis/typing/type_lookup.h"
#include "04_analysis/typing/type_lower.h"
#include "04_analysis/typing/type_stmt.h"
#include "04_analysis/typing/types.h"
#include "04_analysis/typing/variance.h"
#include "04_analysis/composite/classes.h"
#include "04_analysis/caps/cap_system.h"
#include "04_analysis/modal/modal_fields.h"
#include "04_analysis/modal/modal_transitions.h"
#include "04_analysis/composite/record_methods.h"
#include "04_analysis/typing/type_expr.h"
#include "04_analysis/contracts/verification.h"
#include "04_analysis/generics/generic_params.h"
#include "04_analysis/generics/where_bounds.h"
#include "04_analysis/modal/modal.h"
#include "04_analysis/typing/subtyping.h"
#include "04_analysis/typing/type_decls.h"
#include "04_analysis/typing/type_predicates.h"
#include "04_analysis/typing/type_wf.h"
#include "04_analysis/typing/literals.h"
#include "04_analysis/typing/type_infer.h"
#include "04_analysis/memory/regions.h"
#include "04_analysis/contracts/contract_check.h"
#include "04_analysis/typing/expr/path.h"
#include "04_analysis/typing/type_pattern.h"
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

// Every expression node of a piece of syntax, by address; the same generated walk as
// the port's `ExprWalk`.
using ExprSet = std::unordered_set<const ast::Expr*>;
template <typename E> requires std::is_enum_v<E> void W(ExprSet&, E) {}
void W(ExprSet&, const std::string&) {}
template <std::same_as<bool> B> void W(ExprSet&, B) {}
template <std::same_as<std::size_t> N> void W(ExprSet&, N) {}
void W(ExprSet&, const ast::TupleIndex&) {}
void W(ExprSet&, const core::Span&) {}
void W(ExprSet&, const lexer::Token&) {}
void W(ExprSet&, const lexer::DocComment&) {}
template <typename T> void W(ExprSet& s, const std::optional<T>& v);
template <typename T> void W(ExprSet& s, const std::shared_ptr<T>& v);
template <typename T> void W(ExprSet& s, const std::vector<T>& v);
template <typename... Ts> void W(ExprSet& s, const std::variant<Ts...>& v);

#include "ast_walk_generated.inc"

template <typename T> void W(ExprSet& s, const std::optional<T>& v) {
  if (v.has_value()) W(s, *v);
}
template <typename T> void W(ExprSet& s, const std::shared_ptr<T>& v) {
  if (v) W(s, *v);
}
template <typename T> void W(ExprSet& s, const std::vector<T>& v) {
  for (const auto& item : v) W(s, item);
}
template <typename... Ts> void W(ExprSet& s, const std::variant<Ts...>& v) {
  std::visit([&s](const auto& alt) { W(s, alt); }, v);
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

// ---- type core ------------------------------------------------------------------
// Every type written in a declaration (fields, parameters, returns, payloads, aliases,
// module-level bindings) is lowered and printed with what the type core derives from it.

bool g_types_mode = false;
bool g_rel_mode = false;
bool g_val_mode = false;
bool g_pat_mode = false;
bool g_body_mode = false;

void PrintKey(const analysis::TypeKey& key);
void PrintAtom(const analysis::KeyAtom& atom) {
  switch (atom.kind) {
    case analysis::KeyAtom::Kind::Number: std::cout << 'n' << atom.number; break;
    case analysis::KeyAtom::Kind::String: std::cout << "s\"" << Escape(atom.text) << '"'; break;
    case analysis::KeyAtom::Kind::Key:
      if (atom.key) PrintKey(*atom.key); else std::cout << "null";
      break;
    case analysis::KeyAtom::Kind::KeyList:
      std::cout << '[';
      for (std::size_t i = 0; i < atom.key_list.size(); ++i) {
        if (i != 0) std::cout << ' ';
        if (atom.key_list[i]) PrintKey(*atom.key_list[i]); else std::cout << "null";
      }
      std::cout << ']';
      break;
  }
}
void PrintKey(const analysis::TypeKey& key) {
  std::cout << '(';
  for (std::size_t i = 0; i < key.atoms.size(); ++i) {
    if (i != 0) std::cout << ' ';
    PrintAtom(key.atoms[i]);
  }
  std::cout << ')';
}

struct WrittenType {
  std::string label;
  std::shared_ptr<ast::Type> type;
};

void CollectParams(const std::string& owner, const std::vector<ast::Param>& params,
                   const std::shared_ptr<ast::Type>& ret, std::vector<WrittenType>& out) {
  for (const auto& param : params) out.push_back({owner + "(" + param.name + ")", param.type});
  if (ret) out.push_back({owner + "->", ret});
}

void CollectGenerics(const std::string& owner, const std::optional<ast::GenericParams>& params,
                     std::vector<WrittenType>& out) {
  if (!params) return;
  for (const auto& param : params->params) {
    if (param.default_type) out.push_back({owner + "<" + param.name + "=>", param.default_type});
  }
}

std::vector<WrittenType> CollectWrittenTypes(const ast::ASTModule& module) {
  std::vector<WrittenType> out;
  for (const auto& item : module.items) {
    std::visit(
        [&](const auto& node) {
          using T = std::decay_t<decltype(node)>;
          if constexpr (std::is_same_v<T, ast::StaticDecl>) {
            if (node.binding.type_opt) out.push_back({"static", node.binding.type_opt});
          } else if constexpr (std::is_same_v<T, ast::ProcedureDecl> ||
                               std::is_same_v<T, ast::ComptimeProcedureDecl>) {
            CollectGenerics(node.name, node.generic_params, out);
            CollectParams(node.name, node.params, node.return_type_opt, out);
          } else if constexpr (std::is_same_v<T, ast::ExternBlock>) {
            for (const auto& ext : node.items) {
              const auto& proc = std::get<ast::ExternProcDecl>(ext);
              CollectParams(proc.name, proc.params, proc.return_type_opt, out);
            }
          } else if constexpr (std::is_same_v<T, ast::RecordDecl>) {
            CollectGenerics(node.name, node.generic_params, out);
            for (const auto& member : node.members) {
              if (const auto* field = std::get_if<ast::FieldDecl>(&member)) {
                out.push_back({node.name + "." + field->name, field->type});
              } else if (const auto* method = std::get_if<ast::MethodDecl>(&member)) {
                CollectParams(node.name + "::" + method->name, method->params, method->return_type_opt, out);
                if (const auto* recv = std::get_if<ast::ReceiverExplicit>(&method->receiver)) {
                  out.push_back({node.name + "::" + method->name + "(self)", recv->type});
                }
              } else if (const auto* assoc = std::get_if<ast::AssociatedTypeDecl>(&member)) {
                if (assoc->default_type) out.push_back({node.name + "::" + assoc->name, assoc->default_type});
              }
            }
          } else if constexpr (std::is_same_v<T, ast::EnumDecl>) {
            CollectGenerics(node.name, node.generic_params, out);
            for (const auto& variant : node.variants) {
              if (!variant.payload_opt) continue;
              if (const auto* tuple = std::get_if<ast::VariantPayloadTuple>(&*variant.payload_opt)) {
                for (const auto& element : tuple->elements) {
                  out.push_back({node.name + "::" + variant.name, element});
                }
              } else {
                for (const auto& field : std::get<ast::VariantPayloadRecord>(*variant.payload_opt).fields) {
                  out.push_back({node.name + "::" + variant.name + "." + field.name, field.type});
                }
              }
            }
          } else if constexpr (std::is_same_v<T, ast::ModalDecl>) {
            CollectGenerics(node.name, node.generic_params, out);
            for (const auto& state : node.states) {
              const std::string owner = node.name + "@" + state.name;
              for (const auto& member : state.members) {
                if (const auto* field = std::get_if<ast::StateFieldDecl>(&member)) {
                  out.push_back({owner + "." + field->name, field->type});
                } else if (const auto* method = std::get_if<ast::StateMethodDecl>(&member)) {
                  CollectParams(owner + "::" + method->name, method->params, method->return_type_opt, out);
                } else if (const auto* trans = std::get_if<ast::TransitionDecl>(&member)) {
                  CollectParams(owner + "::" + trans->name, trans->params, nullptr, out);
                }
              }
            }
          } else if constexpr (std::is_same_v<T, ast::ClassDecl>) {
            CollectGenerics(node.name, node.generic_params, out);
            for (const auto& class_item : node.items) {
              if (const auto* field = std::get_if<ast::ClassFieldDecl>(&class_item)) {
                out.push_back({node.name + "." + field->name, field->type});
              } else if (const auto* method = std::get_if<ast::ClassMethodDecl>(&class_item)) {
                CollectParams(node.name + "::" + method->name, method->params, method->return_type_opt, out);
              } else if (const auto* assoc = std::get_if<ast::AssociatedTypeDecl>(&class_item)) {
                if (assoc->default_type) out.push_back({node.name + "::" + assoc->name, assoc->default_type});
              } else if (const auto* abstract_field = std::get_if<ast::AbstractFieldDecl>(&class_item)) {
                out.push_back({node.name + "." + abstract_field->name, abstract_field->type});
              } else if (const auto* abstract_state = std::get_if<ast::AbstractStateDecl>(&class_item)) {
                for (const auto& field : abstract_state->fields) {
                  out.push_back({node.name + "@" + abstract_state->name + "." + field.name, field.type});
                }
              }
            }
          } else if constexpr (std::is_same_v<T, ast::TypeAliasDecl>) {
            CollectGenerics(node.name, node.generic_params, out);
            out.push_back({node.name, node.type});
          }
        },
        item);
  }
  return out;
}

const char* VarianceName(analysis::Variance variance) {
  switch (variance) {
    case analysis::Variance::Covariant: return "+";
    case analysis::Variance::Contravariant: return "-";
    case analysis::Variance::Invariant: return "=";
    case analysis::Variance::Bivariant: return "*";
  }
  return "?";
}

void PrintPathList(const std::vector<analysis::TypePath>& paths) {
  for (std::size_t i = 0; i < paths.size(); ++i) {
    if (i != 0) std::cout << ' ';
    std::cout << core::StringOfPath(paths[i]);
  }
}

// Instantiations of a generic declaration's member types: with no arguments (defaults
// only), with one argument, and with an argument for every parameter.
void DumpInstantiations(const std::string& name, const std::vector<ast::TypeParam>& params,
                        const std::vector<analysis::TypeRef>& members) {
  const std::vector<analysis::TypeRef> pool = {
      analysis::MakeTypePrim("i32"), analysis::MakeTypePrim("bool"),
      analysis::MakeTypeString(analysis::StringState::View),
      analysis::MakeTypeTuple({analysis::MakeTypePrim("u8"), analysis::MakeTypePrim("u8")})};
  for (const std::size_t count : {std::size_t{0}, std::size_t{1}, params.size()}) {
    std::vector<analysis::TypeRef> args;
    for (std::size_t i = 0; i < count; ++i) args.push_back(pool[i % pool.size()]);
    const auto subst = analysis::BuildSubstitution(params, args);
    std::cout << "S\t" << name << '\t' << count;
    for (const auto& [param, type] : subst) {
      std::cout << '\t' << param << '=' << Escape(analysis::TypeToString(type));
    }
    std::cout << '\n';
    for (const auto& member : members) {
      std::cout << "I\t" << name << '\t' << count << '\t'
                << Escape(analysis::TypeToString(analysis::InstantiateType(member, subst))) << '\n';
    }
  }
  const auto variance = analysis::ComputeVarianceContext(params, members);
  std::cout << "V\t" << name;
  for (const auto& [param, value] : variance.param_variance) {
    std::cout << '\t' << param << VarianceName(value);
  }
  std::cout << '\n';
}

void PrintOpt(const std::optional<std::uint64_t>& value) {
  if (value) std::cout << *value; else std::cout << '-';
}
void PrintLayout(const std::optional<analysis::layout::Layout>& layout) {
  if (layout) std::cout << layout->size << '/' << layout->align; else std::cout << '-';
}
void PrintOffsets(const std::vector<std::uint64_t>& offsets) {
  for (std::size_t i = 0; i < offsets.size(); ++i) {
    if (i != 0) std::cout << ',';
    std::cout << offsets[i];
  }
}

// Layouts of a declaration, with its own parameters as arguments left out (defaults
// apply) and with every parameter given a concrete argument.
void DumpDeclLayouts(const analysis::ScopeContext& ctx, const ast::ASTItem& item) {
  namespace layout = analysis::layout;
  const std::vector<analysis::TypeRef> pool = {
      analysis::MakeTypePrim("i32"), analysis::MakeTypePrim("bool"),
      analysis::MakeTypeString(analysis::StringState::View),
      analysis::MakeTypeTuple({analysis::MakeTypePrim("u8"), analysis::MakeTypePrim("u8")})};
  const auto arg_lists = [&](const std::optional<ast::GenericParams>& params) {
    std::vector<std::vector<analysis::TypeRef>> lists = {{}};
    if (params && !params->params.empty()) {
      std::vector<analysis::TypeRef> all;
      for (std::size_t i = 0; i < params->params.size(); ++i) all.push_back(pool[i % pool.size()]);
      lists.push_back(all);
      all.push_back(pool[0]);
      lists.push_back(all);
    }
    return lists;
  };
  std::visit(
      [&](const auto& node) {
        using T = std::decay_t<decltype(node)>;
        if constexpr (std::is_same_v<T, ast::RecordDecl>) {
          const auto options = layout::ResolveRecordLayoutOptions(node.attrs);
          std::cout << "RO\t" << node.name << '\t' << (options.packed ? "packed" : "-") << '\t';
          PrintOpt(options.min_align);
          std::cout << '\n';
          std::vector<analysis::TypeRef> fields;
          bool ok = true;
          for (const auto* field : analysis::RecordFields(node)) {
            const auto lowered = layout::LowerTypeForLayout(ctx, field->type);
            if (!lowered) { ok = false; break; }
            fields.push_back(*lowered);
          }
          std::cout << "RL\t" << node.name << '\t';
          const auto record = ok ? layout::RecordLayoutOf(ctx, fields, options) : std::nullopt;
          if (record) {
            PrintLayout(record->layout);
            std::cout << '\t';
            PrintOffsets(record->offsets);
          } else {
            std::cout << "-\t-";
          }
          std::cout << '\n';
        } else if constexpr (std::is_same_v<T, ast::EnumDecl>) {
          const auto options = layout::ResolveEnumLayoutOptions(node.attrs);
          std::cout << "EO\t" << node.name << '\t' << options.disc_type.value_or("-") << '\t';
          PrintOpt(options.min_align);
          std::cout << '\n';
          const auto discs = analysis::EnumDiscriminants(node);
          std::cout << "ED\t" << node.name << '\t';
          if (discs.ok) {
            std::cout << discs.max_disc << '\t';
            PrintOffsets(discs.discs);
          } else {
            std::cout << "fail\t" << (discs.diag_id ? std::string(*discs.diag_id) : "-") << '\t';
            if (discs.span) PrintSpan(*discs.span); else std::cout << '-';
          }
          std::cout << '\n';
          for (const auto& args : arg_lists(node.generic_params)) {
            const auto enum_layout = layout::EnumLayoutOf(ctx, node, args, options);
            std::cout << "EL\t" << node.name << '\t' << args.size() << '\t';
            if (enum_layout) {
              PrintLayout(enum_layout->layout);
              std::cout << '\t' << enum_layout->disc_type << '\t' << enum_layout->payload_size << '/'
                        << enum_layout->payload_align;
            } else {
              std::cout << "-\t-\t-";
            }
            std::cout << '\n';
            for (const auto& variant : node.variants) {
              if (!variant.payload_opt) continue;
              const auto print_member = [&](const std::string& label,
                                            const std::optional<layout::EnumPayloadMemberLayout>& member) {
                std::cout << "EM\t" << node.name << "::" << variant.name << label << '\t' << args.size() << '\t';
                if (member) {
                  std::cout << Escape(analysis::TypeToString(member->type)) << '\t' << member->offset << '\t'
                            << member->payload_size << '/' << member->payload_align;
                } else {
                  std::cout << "-\t-\t-";
                }
                std::cout << '\n';
              };
              if (const auto* tuple = std::get_if<ast::VariantPayloadTuple>(&*variant.payload_opt)) {
                for (std::size_t i = 0; i <= tuple->elements.size(); ++i) {
                  print_member("." + std::to_string(i),
                               layout::EnumTuplePayloadMemberLayout(ctx, node, variant, args, i));
                }
                print_member(".named", layout::EnumRecordPayloadMemberLayout(ctx, node, variant, args, "x"));
              } else {
                for (const auto& field : std::get<ast::VariantPayloadRecord>(*variant.payload_opt).fields) {
                  print_member("." + field.name,
                               layout::EnumRecordPayloadMemberLayout(ctx, node, variant, args, field.name));
                }
                print_member(".missing",
                             layout::EnumRecordPayloadMemberLayout(ctx, node, variant, args, "no_such_field"));
                print_member(".0", layout::EnumTuplePayloadMemberLayout(ctx, node, variant, args, 0));
              }
            }
          }
        } else if constexpr (std::is_same_v<T, ast::ModalDecl>) {
          const auto payload_state = analysis::PayloadState(ctx, node);
          for (const auto& args : arg_lists(node.generic_params)) {
            const auto modal = layout::ModalLayoutOf(ctx, node, args);
            std::cout << "ML\t" << node.name << '\t' << args.size() << '\t'
                      << (payload_state ? std::string(*payload_state) : "-") << '\t';
            if (modal) {
              PrintLayout(modal->layout);
              std::cout << '\t' << (modal->niche ? "niche" : "tagged") << '\t';
              PrintLayout(modal->niche_payload_layout);
              std::cout << '\t' << modal->disc_type.value_or("-") << '\t' << modal->payload_size << '/'
                        << modal->payload_align;
            } else {
              std::cout << "-\t-\t-\t-\t-";
            }
            std::cout << '\n';
          }
        }
      },
      item);
}

void DumpTypes(analysis::ScopeContext& ctx, const analysis::NameMapTable& name_maps) {
  for (const auto& module : ctx.sigma.mods) {
    std::cout << "X\t";
    D(std::cout, module.path);
    std::cout << '\n';
    ctx.current_module = module.path;
    const auto names = name_maps.find(analysis::PathKeyOf(module.path));
    ctx.scopes = {analysis::Scope{},
                  names == name_maps.end() ? analysis::Scope{} : names->second,
                  analysis::UniverseBindings()};
    std::vector<analysis::TypeRef> lowered_types;
    for (const auto& written : CollectWrittenTypes(module)) {
      const auto lowered = analysis::LowerType(ctx, written.type);
      std::cout << "L\t" << Escape(written.label) << '\t';
      if (!lowered.ok) {
        std::cout << "fail\t" << (lowered.diag_id ? std::string(*lowered.diag_id) : "-") << '\n';
        continue;
      }
      lowered_types.push_back(lowered.type);
      std::cout << "ok\t" << Escape(analysis::TypeToString(lowered.type)) << '\t';
      PrintKey(analysis::TypeKeyOf(lowered.type));
      std::cout << '\t';
      const auto paths = analysis::TypePaths(lowered.type);
      PrintPathList(paths);
      std::cout << '\t' << (analysis::IsRangeType(lowered.type) ? 'r' : '-')
                << (analysis::IsRangeIndexType(lowered.type) ? 'i' : '-');
      std::cout << '\t';
      PrintLayout(analysis::layout::LayoutOf(ctx, lowered.type));
      std::cout << ' ';
      PrintOpt(analysis::layout::SizeOf(ctx, lowered.type));
      std::cout << ' ';
      PrintOpt(analysis::layout::AlignOf(ctx, lowered.type));
      if (const auto for_layout = analysis::layout::LowerTypeForLayout(ctx, written.type)) {
        std::cout << ' ' << Escape(analysis::TypeToString(*for_layout)) << ' ';
        PrintLayout(analysis::layout::LayoutOf(ctx, *for_layout));
      } else {
        std::cout << " nolower";
      }
      if (const auto sig = analysis::AsyncSigOf(ctx, lowered.type)) {
        std::cout << "\tasync " << Escape(analysis::TypeToString(sig->out)) << " / "
                  << Escape(analysis::TypeToString(sig->in)) << " / "
                  << Escape(analysis::TypeToString(sig->result)) << " / "
                  << Escape(analysis::TypeToString(sig->err));
      } else {
        std::cout << "\t-";
      }
      std::cout << '\n';
      if (const auto* uni = std::get_if<analysis::TypeUnion>(&lowered.type->node)) {
        std::cout << "UL\t";
        if (const auto union_layout = analysis::layout::UnionLayoutOf(ctx, *uni)) {
          PrintLayout(union_layout->layout);
          std::cout << '\t' << (union_layout->niche ? "niche" : "tagged") << '\t';
          PrintLayout(union_layout->niche_payload_layout);
          std::cout << '\t' << union_layout->disc_type.value_or("-") << '\t' << union_layout->payload_size
                    << '/' << union_layout->payload_align;
          for (const auto& member : union_layout->member_list) {
            std::cout << '\t' << Escape(analysis::TypeToString(member));
          }
        } else {
          std::cout << '-';
        }
        std::cout << '\n';
      }
      if (analysis::IsRangeType(lowered.type)) {
        std::cout << "GL\t";
        if (const auto range = analysis::layout::RangeLayoutOf(ctx, lowered.type)) {
          PrintLayout(range->layout);
          std::cout << '\t';
          PrintOffsets(range->offsets);
        } else {
          std::cout << '-';
        }
        std::cout << '\n';
      }
      if (const auto* tuple = std::get_if<analysis::TypeTuple>(&lowered.type->node)) {
        std::cout << "TL\t";
        if (const auto tuple_layout = analysis::layout::TupleLayoutOf(ctx, tuple->elements)) {
          PrintLayout(tuple_layout->layout);
          std::cout << '\t';
          PrintOffsets(tuple_layout->offsets);
        } else {
          std::cout << '-';
        }
        std::cout << '\n';
      }
      if (const auto lowered_async = analysis::layout::LowerAsyncType(lowered.type)) {
        std::cout << "AL";
        for (const auto& state : lowered_async->states) std::cout << '\t' << state;
        std::cout << '\t' << Escape(analysis::TypeToString(lowered_async->resume_type)) << '\t';
        PrintLayout(analysis::layout::LayoutOf(ctx, lowered_async->resume_type));
        std::cout << '\n';
      }
      for (const auto& path : paths) {
        analysis::TypePath resolved;
        const auto* decl = analysis::LookupTypeDecl(ctx, path, &resolved);
        std::cout << "P\t" << core::StringOfPath(path) << '\t';
        if (decl) {
          std::cout << decl->index() << '\t' << core::StringOfPath(resolved);
        } else {
          std::cout << "-\t-";
        }
        std::cout << '\n';
      }
    }
    // Equivalence between the first types of the module, as a matrix.
    const std::size_t count = std::min<std::size_t>(lowered_types.size(), 48);
    for (std::size_t i = 0; i < count; ++i) {
      std::cout << "Q\t" << i << '\t';
      for (std::size_t j = 0; j < count; ++j) {
        const auto equiv = analysis::TypeEquiv(lowered_types[i], lowered_types[j]);
        std::cout << (equiv.ok ? (equiv.equiv ? '1' : '0') : 'e');
      }
      std::cout << '\n';
    }
    {
      const auto dyn = analysis::layout::DynLayoutOf(ctx);
      std::cout << "DL\t";
      PrintLayout(dyn.layout);
      for (const auto& field : dyn.fields) std::cout << '\t' << Escape(analysis::TypeToString(field));
      std::cout << '\n';
    }
    for (const auto& item : module.items) DumpDeclLayouts(ctx, item);
    for (const auto& item : module.items) {
      std::visit(
          [&](const auto& node) {
            using T = std::decay_t<decltype(node)>;
            if constexpr (std::is_same_v<T, ast::RecordDecl>) {
              std::vector<analysis::TypeRef> members;
              for (const auto* field : analysis::RecordFields(node)) {
                const auto type = analysis::FieldType(node, field->name, ctx, {});
                std::cout << "FT\t" << node.name << '.' << field->name << '\t'
                          << (type ? Escape(analysis::TypeToString(*type)) : "-") << '\t'
                          << (analysis::FieldVisible(ctx, node, field->name,
                                                     analysis::TypePath{"Elsewhere", node.name})
                                  ? "visible" : "hidden")
                          << '\n';
                const auto lowered = analysis::LowerType(ctx, field->type);
                if (lowered.ok) members.push_back(lowered.type);
              }
              if (node.generic_params) DumpInstantiations(node.name, node.generic_params->params, members);
            } else if constexpr (std::is_same_v<T, ast::TypeAliasDecl>) {
              if (!node.generic_params) return;
              const auto lowered = analysis::LowerType(ctx, node.type);
              if (lowered.ok) DumpInstantiations(node.name, node.generic_params->params, {lowered.type});
            }
          },
          item);
    }
  }
}

// ---- relations between types: well-formedness, predicates, subtyping, classes ----

std::string DiagOr(const std::optional<std::string_view>& diag_id) {
  return diag_id ? std::string(*diag_id) : "-";
}

void PrintSignature(const analysis::SignatureResult& sig) {
  if (!sig.ok) {
    std::cout << "fail\t" << DiagOr(sig.diag_id);
    return;
  }
  std::cout << "ok\t" << Escape(analysis::TypeToString(sig.func_type)) << '\t'
            << Escape(analysis::TypeToString(sig.return_type)) << '\t';
  for (const auto& [name, type] : sig.bindings) {
    std::cout << name << ':' << Escape(analysis::TypeToString(type)) << ';';
  }
}

void PrintProof(const analysis::StaticProofResult& proof) {
  std::cout << (proof.provable ? '1' : '0') << '/' << DiagOr(proof.diag_id) << '/' << Escape(proof.explanation);
}

void PrintConst(const analysis::ConstValue& value) {
  if (!value.known) std::cout << '?';
  else if (value.is_bool) std::cout << (value.bool_value ? "true" : "false");
  else std::cout << value.value;
}

// A contract's clauses: what each proves alone, and the postcondition given the
// precondition.
void DumpContract(const std::string& owner, const std::optional<ast::ContractClause>& contract) {
  if (!contract) return;
  std::cout << "PC\t" << owner;
  for (const auto& clause : {contract->precondition, contract->postcondition}) {
    std::cout << '\t';
    if (!clause) {
      std::cout << '-';
      continue;
    }
    PrintProof(analysis::StaticProof(analysis::StaticProofContext{}, clause));
    std::cout << ' ';
    PrintConst(analysis::EvaluateConstant(clause));
    std::cout << ' ' << (analysis::EntTrue(clause) ? 't' : '-');
    const auto negated = analysis::NegatedPredicate(clause);
    std::cout << (negated && *negated ? 'n' : '-');
  }
  std::cout << '\t';
  if (contract->precondition && contract->postcondition) {
    analysis::StaticProofContext proof_ctx;
    analysis::AddPredicateFacts(proof_ctx, contract->precondition);
    std::cout << proof_ctx.facts.size() << ' ';
    PrintProof(analysis::StaticProof(proof_ctx, contract->postcondition));
    std::cout << ' ' << (analysis::EntFact(proof_ctx, contract->postcondition) ? 'f' : '-')
              << (analysis::EntLinear(proof_ctx, contract->postcondition) ? 'l' : '-');
    // And the precondition refuted: its negation as the only fact.
    if (const auto negated = analysis::NegatedPredicate(contract->precondition); negated && *negated) {
      analysis::StaticProofContext neg_ctx;
      analysis::AddPredicateFacts(neg_ctx, *negated);
      std::cout << ' ';
      PrintProof(analysis::StaticProof(neg_ctx, contract->postcondition));
    }
  } else {
    std::cout << '-';
  }
  std::cout << '\n';
}

void PrintDiagList(const core::DiagnosticStream& diags) {
  for (const auto& diag : diags) {
    std::cout << " [" << diag.code << '|' << Escape(diag.message) << '|';
    if (diag.span) std::cout << diag.span->start_offset; else std::cout << '-';
    std::cout << ']';
  }
}

std::string TypeText(const analysis::TypeRef& type) {
  return type ? Escape(analysis::TypeToString(type)) : "-";
}

// A generic parameter list: how it validates, the scope it binds, and how argument
// lists drawn from the module's types fit it.
void DumpGenericParams(analysis::ScopeContext& ctx, const std::string& name,
                       const std::optional<ast::GenericParams>& generic_params,
                       const std::vector<analysis::TypeRef>& types) {
  const auto& params = generic_params->params;
  std::cout << "GP\t" << name << '\t' << analysis::RequiredParamCount(generic_params) << '/'
            << analysis::TotalParamCount(generic_params)
            << (analysis::HasDefaultParams(generic_params) ? 'd' : '-') << '\t';
  const auto valid = analysis::ValidateGenericParams(ctx, generic_params);
  std::cout << (valid.ok ? "ok" : "fail") << ' ' << DiagOr(valid.diag_id);
  for (const auto& info : valid.type_params) {
    std::cout << ' ' << info.name << '/' << info.class_bounds.size() << '/'
              << (info.default_type ? TypeText(*info.default_type) : "none");
  }
  PrintDiagList(valid.diagnostics);
  std::cout << '\t';
  const auto processed = analysis::ProcessGenericParams(ctx, generic_params);
  std::cout << (processed.ok ? "ok" : "fail") << ' ' << DiagOr(processed.diag_id);
  if (processed.ok) {
    for (const auto& info : processed.params) {
      std::cout << ' ' << info.name << '/' << info.class_bounds.size() << '/' << TypeText(info.default_type);
    }
  }
  std::cout << '\t' << analysis::BindTypeParams(ctx, generic_params).size();
  const auto scope = analysis::BuildParamScope(ctx, generic_params);
  std::vector<std::string> keys;
  for (const auto& [key, entity] : scope) keys.push_back(key);
  std::sort(keys.begin(), keys.end());
  for (const auto& key : keys) {
    const auto& entity = scope.at(key);
    std::cout << ' ' << key << '=' << entity.target_opt.value_or("-") << '/' << entity.type_param_class_bounds.size();
  }
  std::cout << '\t';
  for (const auto& param : params) {
    const auto info = analysis::ParseConstParam(ctx, param, analysis::MakeTypePrim("u8"));
    std::cout << info.name << '=';
    if (info.default_value) std::cout << *info.default_value; else std::cout << '-';
    std::cout << (analysis::ParseConstParam(ctx, param, analysis::MakeTypePrim("bool")).type ? '!' : ' ');
  }
  std::cout << '\n';
  for (std::size_t n = 0; n <= params.size() + 1; ++n) {
    for (const std::size_t shift : {std::size_t{0}, std::size_t{7}}) {
      if (types.empty() && (n != 0 || shift != 0)) continue;
      std::vector<analysis::TypeRef> args;
      for (std::size_t i = 0; i < n; ++i) args.push_back(types[(shift + i * 3) % types.size()]);
      std::cout << "GB\t" << name << '\t' << n << '.' << shift << '\t';
      const auto bounds = analysis::CheckBoundsSatisfied(params, args);
      if (bounds.ok) {
        std::cout << "ok";
      } else {
        std::cout << "fail " << DiagOr(bounds.diag_id) << '|' << bounds.param_name << '|' << Escape(bounds.type_name)
                  << '|' << bounds.bound_name;
      }
      PrintDiagList(bounds.diagnostics);
      std::cout << '\t';
      if (processed.ok) {
        const auto checked = analysis::CheckGenericArgs(ctx, processed.params, args);
        std::cout << (checked.ok ? "ok" : "fail") << ' ' << DiagOr(checked.diag_id);
      } else {
        std::cout << '-';
      }
      std::cout << '\n';
    }
  }
}

// Type arguments inferred for a generic procedure: from its own parameter types, from
// those types instantiated with the module's types, and from the module's types as they
// come.
void DumpInference(analysis::ScopeContext& ctx, const std::string& name,
                   const std::optional<ast::GenericParams>& generic_params,
                   const std::vector<ast::Param>& proc_params,
                   const std::vector<analysis::TypeRef>& types) {
  const auto& params = generic_params->params;
  const auto saved = ctx.scopes;
  ctx.scopes = analysis::BindTypeParams(ctx, generic_params);
  std::vector<analysis::TypeRef> expected;
  for (const auto& param : proc_params) {
    const auto lowered = analysis::LowerType(ctx, param.type);
    expected.push_back(lowered.ok ? lowered.type : nullptr);
  }
  ctx.scopes = saved;
  for (int variant = 0; variant < 3; ++variant) {
    if (variant != 0 && types.empty()) continue;
    std::vector<analysis::TypeRef> actual;
    if (variant == 0) {
      actual = expected;
    } else if (variant == 1) {
      std::vector<analysis::TypeRef> args;
      for (std::size_t j = 0; j < params.size(); ++j) args.push_back(types[(j * 5 + 1) % types.size()]);
      const auto subst = analysis::BuildSubstitution(params, args);
      for (const auto& type : expected) actual.push_back(type ? analysis::InstantiateType(type, subst) : nullptr);
    } else {
      for (std::size_t i = 0; i < expected.size(); ++i) actual.push_back(types[i % types.size()]);
    }
    const auto inferred = analysis::InferTypeArguments(params, expected, actual, std::nullopt);
    std::cout << "IN\t" << name << '\t' << variant << '\t' << (inferred.ok ? "ok" : "fail") << ' '
              << DiagOr(inferred.diag_id);
    for (const auto& arg : inferred.inferred_args) std::cout << '\t' << TypeText(arg);
    std::cout << '\n';
  }
}

void DumpInstantiationSet(const std::vector<analysis::TypeRef>& types) {
  analysis::MonomorphizeContext mono;
  const std::size_t count = std::min<std::size_t>(types.size(), 12);
  const auto key = [&](const char* name, std::vector<analysis::TypeRef> args) {
    return analysis::InstantiationKey{analysis::TypePath{"M", name}, std::move(args)};
  };
  for (std::size_t i = 0; i < count; ++i) mono.Demand(key("G", {types[i]}));
  for (std::size_t i = 0; i < count; ++i) mono.Demand(key("G", {types[i], types[count - 1 - i]}));
  for (std::size_t i = 0; i < count; ++i) mono.Demand(key("F", {types[count - 1 - i]}));
  mono.Demand(key("A", {}));
  mono.Demand(key("A", {}));
  std::cout << "MI\t" << mono.Instantiations().size();
  for (const auto& [inst, entry] : mono.Instantiations()) {
    std::cout << '\t' << core::StringOfPath(inst.decl_path) << '<';
    for (const auto& arg : inst.args) std::cout << TypeText(arg) << ';';
    std::cout << '>' << (entry.processed ? 'p' : '-');
  }
  std::cout << '\t' << (mono.HasInstantiation(key("A", {})) ? '1' : '0')
            << (mono.HasInstantiation(key("B", {})) ? '1' : '0') << (mono.ProcessToFixedPoint() ? 'T' : 'F');
  std::size_t processed = 0;
  for (const auto& [inst, entry] : mono.Instantiations()) processed += entry.processed ? 1 : 0;
  std::cout << processed << '\n';
}

// Constraint sets built from two of the module's types and fresh type variables, and
// what solving each yields.
void DumpSolving(const analysis::ScopeContext& ctx, const std::vector<analysis::TypeRef>& types) {
  using namespace analysis;
  const std::size_t count = std::min<std::size_t>(types.size(), 24);
  const auto var = [](std::uint32_t id) { return MakeTypeVar(id); };
  const auto eq = [](TypeRef lhs, TypeRef rhs) { return Constraint{std::move(lhs), std::move(rhs), false}; };
  const auto sub = [](TypeRef lhs, TypeRef rhs) { return Constraint{std::move(lhs), std::move(rhs), true}; };
  for (std::size_t i = 0; i < count; ++i) {
    const TypeRef a = types[i];
    const TypeRef b = types[(i + 1) % types.size()];
    const std::vector<ConstraintSet> sets = {
        {eq(var(0), a)},
        {eq(MakeTypeTuple({var(0), var(1)}), MakeTypeTuple({a, b}))},
        {eq(var(0), a), eq(var(0), b)},
        {sub(var(0), a), sub(b, var(1)), eq(var(1), var(0))},
        {eq(var(0), MakeTypeTuple({var(0), a}))},
        {sub(a, b)},
        {eq(MakeTypeSlice(var(0)), MakeTypeSlice(a)),
         eq(MakeTypeRawPtr(RawPtrQual::Imm, var(1)), MakeTypeRawPtr(RawPtrQual::Imm, b)),
         eq(MakeTypePtr(var(2), PtrState::Valid), MakeTypePtr(a, PtrState::Valid))},
        {eq(MakeTypeFunc({TypeFuncParam{std::nullopt, var(0)}}, var(1)), MakeTypeFunc({TypeFuncParam{std::nullopt, a}}, b)),
         eq(var(0), var(1))},
        {eq(var(0), var(1)), eq(var(1), var(2)), eq(var(2), a),
         eq(MakeTypePerm(Permission::Unique, var(3)), MakeTypePerm(Permission::Unique, var(0)))},
        {eq(MakeTypeUnion({var(0), a}), MakeTypeUnion({b, a}))},
        {eq(MakeTypeRange(var(0)), MakeTypeRange(a)), eq(a, var(1)), eq(MakeTypeArray(var(0), 3), MakeTypeArray(b, 3))},
        {eq(MakeTypeClosure({{false, var(0)}}, var(1), std::nullopt), MakeTypeClosure({{false, a}}, b, std::nullopt))},
        {eq(var(1), var(0)), eq(var(0), var(1)), eq(var(0), a)},
        {eq(MakeTypePerm(Permission::Unique, var(0)), a), eq(MakeTypeArray(var(1), 2), b), eq(MakeTypeTuple({var(2), var(3)}), a)},
    };
    std::cout << "SV\t" << i;
    for (const auto& set : sets) {
      const auto solved = Solve(ctx, set);
      std::cout << '\t';
      if (!solved.ok) {
        std::cout << "fail " << DiagOr(solved.diag_id);
        continue;
      }
      std::cout << "ok";
      for (std::uint32_t id = 0; id < 4; ++id) {
        const auto it = solved.subst.find(id);
        if (it != solved.subst.end()) std::cout << ' ' << id << '=' << TypeText(it->second);
      }
      std::cout << " => "
                << TypeText(ApplySubstitution(MakeTypeTuple({var(0), var(1), var(2), var(3)}), solved.subst));
    }
    std::cout << '\n';
    std::cout << "SU\t" << i << '\t';
    for (std::size_t j = 0; j < count; ++j) {
      const auto solved = Solve(ctx, {eq(a, types[j])});
      std::cout << (solved.ok ? '1' : (solved.diag_id == std::optional<std::string_view>{"Syn-Call-Err"} ? '0' : '?'));
    }
    std::cout << '\n';
  }
}

// Purity of the clauses of a contract and of the expressions written directly in a
// body: one mark each, in source order.
void DumpPurity(const analysis::ScopeContext& ctx, const std::string& name,
                const std::optional<ast::ContractClause>& contract, const std::shared_ptr<ast::Block>& body) {
  analysis::ContractContext contract_ctx;
  contract_ctx.scope_ctx = &ctx;
  std::string marks;
  const auto mark = [&](const ast::ExprPtr& expr) {
    if (!expr) {
      marks.push_back('-');
      return;
    }
    const auto purity = analysis::CheckPurity(contract_ctx, expr);
    marks.push_back(purity.ok ? '1' : '0');
  };
  if (contract) {
    mark(contract->precondition);
    mark(contract->postcondition);
  }
  marks.push_back('|');
  if (body) {
    for (const auto& stmt : body->stmts) {
      if (const auto* node = std::get_if<ast::LetStmt>(&stmt)) mark(node->binding.init);
      else if (const auto* node = std::get_if<ast::VarStmt>(&stmt)) mark(node->binding.init);
      else if (const auto* node = std::get_if<ast::ExprStmt>(&stmt)) mark(node->value);
      else if (const auto* node = std::get_if<ast::ReturnStmt>(&stmt)) mark(node->value_opt);
      else if (const auto* node = std::get_if<ast::AssignStmt>(&stmt)) mark(node->value);
      else marks.push_back('.');
    }
    mark(body->tail_opt);
  }
  std::cout << "PU\t" << name << '\t' << marks << '\n';
}

void DumpRelations(analysis::ScopeContext& ctx, const analysis::NameMapTable& name_maps) {
  static const char* const kFoundational[] = {"Bitcopy", "Clone", "Drop", "FfiSafe", "GpuSafe", "Eq",
                                               "Discrete", "Hash", "Iterator"};
  for (const auto& module : ctx.sigma.mods) {
    std::cout << "X\t";
    D(std::cout, module.path);
    std::cout << '\n';
    ctx.current_module = module.path;
    const auto names = name_maps.find(analysis::PathKeyOf(module.path));
    ctx.scopes = {analysis::Scope{},
                  names == name_maps.end() ? analysis::Scope{} : names->second,
                  analysis::UniverseBindings()};
    std::vector<analysis::TypeRef> types;
    for (const auto& written : CollectWrittenTypes(module)) {
      const auto lowered = analysis::LowerType(ctx, written.type);
      if (!lowered.ok) continue;
      const auto& type = lowered.type;
      types.push_back(type);
      const std::string text = analysis::TypeToString(type);
      std::cout << "W\t" << Escape(written.label) << '\t' << Escape(text) << '\t';
      // A refinement's predicate is typed as an expression, which the port cannot do yet.
      if (text.find(" where { ... }") != std::string::npos) {
        std::cout << "pending";
      } else {
        const auto wf = analysis::TypeWF(ctx, type);
        std::cout << (wf.ok ? "ok" : "fail:" + DiagOr(wf.diag_id));
      }
      std::cout << '\t' << (analysis::BitcopyType(ctx, type) ? 'b' : '-')
                << (analysis::CloneType(ctx, type) ? 'c' : '-')
                << (analysis::DropType(ctx, type) ? 'd' : '-')
                << (analysis::ZeroableType(ctx, type) ? 'z' : '-')
                << (analysis::EqType(type) ? 'e' : '-')
                << (analysis::EqType(ctx, type) ? 'E' : '-')
                << (analysis::BuiltinDiscreteType(type) ? 'i' : '-')
                << (analysis::OrdType(type) ? 'o' : '-')
                << (analysis::IsCapabilityType(type) ? 'C' : '-')
                << (analysis::FfiSafeType(ctx, type) ? 'f' : '-')
                << (analysis::IsValidConstParamType(type) ? 'k' : '-')
                << static_cast<int>(analysis::PermOfType(type));
      std::cout << '\t' << DiagOr(analysis::FfiSafeDiagForType(ctx, type)) << '\t'
                << DiagOr(analysis::GpuSafeDiagForType(ctx, type)) << '\t'
                << Escape(analysis::TypeToString(analysis::StripPerm(type)));
      for (const char* method : {"eq", "successor", "predecessor"}) {
        std::cout << '\t';
        if (const auto sig = analysis::LookupFoundationalBuiltinMethodSig(ctx, type, method)) {
          std::cout << Escape(analysis::TypeToString(sig->recv_type)) << '(';
          for (const auto& param : sig->params) std::cout << Escape(analysis::TypeToString(param.type));
          std::cout << ")->" << Escape(analysis::TypeToString(sig->ret));
        } else {
          std::cout << '-';
        }
        std::cout << (analysis::LookupFoundationalBuiltinMethodSig(type, method) ? '+' : '-');
      }
      std::cout << '\n';
      if (const auto* refine = std::get_if<analysis::TypeRefine>(&type->node); refine && refine->predicate) {
        std::cout << "PR\t";
        PrintProof(analysis::StaticProof(analysis::StaticProofContext{}, refine->predicate));
        std::cout << '\t';
        PrintConst(analysis::EvaluateConstant(refine->predicate));
        analysis::StaticProofContext self_ctx;
        analysis::AddPredicateFacts(self_ctx, refine->predicate);
        std::cout << '\t' << self_ctx.facts.size() << ' ';
        PrintProof(analysis::StaticProof(self_ctx, refine->predicate));
        const auto bounds = analysis::GetTypeBounds(refine->base);
        std::cout << '\t' << bounds.has_min << bounds.has_max << ' ' << bounds.min << ' ' << bounds.max << '\n';
      }
    }
    const std::size_t count = std::min<std::size_t>(types.size(), 40);
    for (std::size_t i = 0; i < count; ++i) {
      std::string sub, arg, cast, diags;
      for (std::size_t j = 0; j < count; ++j) {
        const auto res = analysis::Subtyping(ctx, types[i], types[j]);
        sub.push_back(res.ok ? (res.subtype ? '1' : '0') : 'e');
        if (res.diag_id) diags += " " + std::to_string(j) + "=" + std::string(*res.diag_id);
        const auto compatible = analysis::ArgumentTypeCompatible(ctx, types[i], types[j], std::nullopt);
        arg.push_back(compatible.ok ? (compatible.subtype ? '1' : '0') : 'e');
        cast.push_back(analysis::CastValid(types[i], types[j]) ? '1' : '0');
      }
      std::cout << "SB\t" << i << '\t' << sub << '\t' << arg << '\t' << cast << '\t' << diags << '\n';
    }
    DumpInstantiationSet(types);
    DumpSolving(ctx, types);
    for (const char* name : kFoundational) {
      std::cout << "FI\t" << name << '\t';
      for (std::size_t i = 0; i < count; ++i) {
        std::cout << (analysis::TypeImplementsClass(ctx, types[i], ast::ClassPath{name}) ? '1' : '0');
      }
      std::cout << '\n';
    }
    std::vector<ast::ClassPath> classes;
    for (const auto& item : module.items) {
      if (const auto* decl = std::get_if<ast::ClassDecl>(&item)) {
        ast::ClassPath path = module.path;
        path.push_back(decl->name);
        classes.push_back(path);
      }
    }
    for (const auto& item : module.items) {
      std::visit(
          [&](const auto& node) {
            using T = std::decay_t<decltype(node)>;
            ast::Path path = module.path;
            if constexpr (requires { node.name; }) path.push_back(node.name);
            if constexpr (requires { node.generic_params; node.name; }) {
              if (node.generic_params) DumpGenericParams(ctx, node.name, node.generic_params, types);
            }
            if constexpr (requires { node.generic_params; node.params; node.name; }) {
              if (node.generic_params) DumpInference(ctx, node.name, node.generic_params, node.params, types);
            }
            if constexpr (std::is_same_v<T, ast::ProcedureDecl>) {
              DumpContract(node.name, node.contract);
              DumpPurity(ctx, node.name, node.contract, node.body);
            } else if constexpr (std::is_same_v<T, ast::ClassDecl>) {
              std::cout << "CL\t" << node.name << '\t';
              const auto order = analysis::LinearizeClass(ctx, path);
              if (order.ok) {
                std::cout << "ok";
                for (const auto& entry : order.order) std::cout << ' ' << core::StringOfPath(entry);
              } else {
                std::cout << "fail " << DiagOr(order.diag_id);
              }
              std::cout << '\t';
              const auto methods = analysis::ClassMethodTable(ctx, path);
              if (methods.ok) {
                std::cout << "ok";
                for (const auto& entry : methods.methods) {
                  std::cout << ' ' << core::StringOfPath(entry.owner) << "::" << entry.method->name
                            << (analysis::VTableEligible(*entry.method) ? "+" : "-");
                }
              } else {
                std::cout << "fail " << DiagOr(methods.diag_id);
              }
              std::cout << '\t';
              const auto fields = analysis::ClassFieldTable(ctx, path);
              if (fields.ok) {
                std::cout << "ok";
                for (const auto* field : fields.fields) std::cout << ' ' << field->name;
              } else {
                std::cout << "fail " << DiagOr(fields.diag_id);
              }
              std::cout << '\t' << DiagOr(analysis::ClassDispatchabilityDiagnostic(ctx, path)) << '\t'
                        << (analysis::ClassDispatchable(ctx, path) ? 'd' : '-')
                        << (analysis::Dispatchable(ctx, node) ? 'D' : '-')
                        << (analysis::IsModalClass(node) ? 'm' : '-')
                        << (analysis::IsCapabilityClass(ctx, path) ? 'c' : '-') << '\t'
                        << analysis::ClassAssociatedTypes(node).size() << '/'
                        << analysis::ClassAbstractStates(node).size() << '\t';
              for (const auto& other : classes) std::cout << (analysis::ClassSubtypes(ctx, path, other) ? '1' : '0');
              std::cout << '\t';
              for (std::size_t i = 0; i < count; ++i) {
                std::cout << (analysis::TypeImplementsClass(ctx, types[i], path) ? '1' : '0');
              }
              std::cout << '\t';
              for (const auto& item : node.items) {
                if (const auto* method = std::get_if<ast::ClassMethodDecl>(&item)) {
                  const auto* found = analysis::LookupClassMethod(ctx, path, method->name);
                  std::cout << (found ? (found == method ? '=' : '^') : '!');
                }
              }
              std::cout << '\n';
            } else if constexpr (std::is_same_v<T, ast::RecordDecl>) {
              const auto self_type = analysis::MakeTypePath(path);
              for (const auto& class_path : node.implements) {
                const auto complete = analysis::CheckImplCompleteness(ctx, class_path, node);
                std::cout << "IM\t" << node.name << '\t' << core::StringOfPath(class_path) << '\t'
                          << (complete.ok ? "ok" : "fail") << ' ' << DiagOr(complete.diag_id);
                for (const auto& missing : complete.missing_methods) std::cout << ' ' << missing;
                std::cout << '\t' << (analysis::CheckOrphanRule(ctx, path, class_path, module.path) ? '1' : '0')
                          << (analysis::CheckOrphanRule(ctx, path, class_path, ast::ModulePath{"Elsewhere"}) ? '1' : '0')
                          << (analysis::TypeImplementsClass(ctx, self_type, class_path) ? '1' : '0') << '\n';
              }
              for (const auto& member : node.members) {
                if (const auto* method = std::get_if<ast::MethodDecl>(&member)) {
                  std::cout << "MS\t" << node.name << "::" << method->name << '\t';
                  PrintSignature(analysis::BuildMethodSignature(ctx, self_type, method->receiver, method->params,
                                                                method->return_type_opt));
                  const auto mode = analysis::RecvModeOf(method->receiver);
                  std::cout << '\t' << (mode ? "move" : "-") << '\n';
                  DumpContract(node.name + "::" + method->name, method->contract);
                  DumpPurity(ctx, node.name + "::" + method->name, method->contract, method->body);
                }
              }
            } else if constexpr (std::is_same_v<T, ast::ModalDecl>) {
              for (const auto& state : node.states) {
                const auto state_type = analysis::MakeTypeModalState(path, state.name);
                std::cout << "MW\t" << node.name << '@' << state.name << '\t'
                          << (analysis::NicheCompatible(ctx, path, state.name) ? 'n' : '-')
                          << (analysis::WidenWarnCond(ctx, path, state.name) ? 'w' : '-')
                          << (analysis::HasState(node, state.name) ? 's' : '-') << '\n';
                for (const auto& member : state.members) {
                  if (const auto* method = std::get_if<ast::StateMethodDecl>(&member)) {
                    std::cout << "MS\t" << node.name << '@' << state.name << "::" << method->name << '\t';
                    PrintSignature(analysis::BuildMethodSignature(ctx, state_type, method->receiver, method->params,
                                                                  method->return_type_opt));
                    std::cout << '\t' << (analysis::LookupStateMethodDecl(node, state.name, method->name) == method ? '=' : '!')
                              << '\n';
                  } else if (const auto* transition = std::get_if<ast::TransitionDecl>(&member)) {
                    std::cout << "TS\t" << node.name << '@' << state.name << "::" << transition->name << '\t';
                    PrintSignature(analysis::BuildTransitionSignature(
                        ctx, state_type, analysis::MakeTypeModalState(path, transition->target_state),
                        transition->params));
                    std::cout << '\t' << (analysis::LookupTransitionDecl(node, state.name, transition->name) == transition ? '=' : '!')
                              << '\n';
                  } else if (const auto* field = std::get_if<ast::StateFieldDecl>(&member)) {
                    std::cout << "MF\t" << node.name << '@' << state.name << '.' << field->name << '\t'
                              << (analysis::LookupModalFieldDecl(node, state.name, field->name) == field ? '=' : '!')
                              << '\n';
                  }
                }
              }
            }
          },
          item);
    }
  }
}

// ---- the bytes of values; see `layout_value_bits.cpp` ----

void PrintHex(const std::optional<std::vector<std::uint8_t>>& bits) {
  if (!bits) {
    std::cout << '-';
    return;
  }
  static const char* const kDigits = "0123456789abcdef";
  std::cout << 'x';
  for (const auto byte : *bits) std::cout << kDigits[byte >> 4] << kDigits[byte & 15];
}

// Every literal token of a file, encoded as each primitive type and as a raw pointer.
int DumpConsts(const std::string& label, const std::string& path) {
  static const char* const kPrims[] = {"i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64",
                                       "u128", "usize", "f16", "f32", "f64", "bool", "char", "()", "!", "string"};
  std::ifstream in(path, std::ios::binary);
  if (!in) {
    std::cerr << "cannot read " << path << "\n";
    return 2;
  }
  const std::vector<std::uint8_t> bytes((std::istreambuf_iterator<char>(in)), std::istreambuf_iterator<char>());
  std::cout << "F\t" << label << '\n';
  const core::SourceLoadResult loaded = core::LoadSource(label, bytes);
  if (!loaded.source.has_value()) {
    std::cout << "NOSOURCE\n";
    return 0;
  }
  const lexer::TokenizeDiagnosticResult result = lexer::TokenizeWithDiagnostics(*loaded.source);
  if (!result.output.has_value()) {
    std::cout << "NOTOKENS\n";
    return 0;
  }
  const auto raw = analysis::MakeTypeRawPtr(analysis::RawPtrQual::Imm, analysis::MakeTypePrim("u8"));
  for (const auto& token : result.output->tokens) {
    switch (token.kind) {
      case lexer::TokenKind::IntLiteral:
      case lexer::TokenKind::FloatLiteral:
      case lexer::TokenKind::CharLiteral:
      case lexer::TokenKind::BoolLiteral:
      case lexer::TokenKind::NullLiteral:
      case lexer::TokenKind::StringLiteral:
        break;
      default:
        continue;
    }
    std::cout << "C\t" << KindName(token.kind) << '\t' << Escape(token.lexeme);
    for (const char* prim : kPrims) {
      const auto bits = analysis::layout::EncodeConst(analysis::MakeTypePrim(prim), token);
      if (bits) {
        std::cout << '\t' << prim << '=';
        PrintHex(bits);
      }
    }
    if (const auto bits = analysis::layout::EncodeConst(raw, token)) {
      std::cout << "\traw=";
      PrintHex(bits);
    }
    {
      // The literal's own type, and where it may stand: the first field lists the
      // expected types that accept it, the second those that reject it with a rule.
      static const analysis::ScopeContext empty_ctx;
      ast::LiteralExpr literal;
      literal.literal = token;
      const auto typed = analysis::TypeLiteralExpr(empty_ctx, literal);
      std::cout << "\ttype=" << (typed.ok ? Escape(analysis::TypeToString(typed.type)) : "fail:" + DiagOr(typed.diag_id));
      std::vector<std::pair<std::string, analysis::TypeRef>> expected;
      for (const char* prim : kPrims) expected.emplace_back(prim, analysis::MakeTypePrim(prim));
      expected.emplace_back("view", analysis::MakeTypeString(analysis::StringState::View));
      expected.emplace_back("managed", analysis::MakeTypeString(analysis::StringState::Managed));
      expected.emplace_back("text", analysis::MakeTypeString(std::nullopt));
      expected.emplace_back("raw", raw);
      expected.emplace_back("uniq-raw", analysis::MakeTypePerm(analysis::Permission::Unique, raw));
      expected.emplace_back("ptr", analysis::MakeTypePtr(analysis::MakeTypePrim("u8"), std::nullopt));
      expected.emplace_back("uniq-i64", analysis::MakeTypePerm(analysis::Permission::Unique, analysis::MakeTypePrim("i64")));
      expected.emplace_back("const-f64", analysis::MakeTypePerm(analysis::Permission::Const, analysis::MakeTypePrim("f64")));
      expected.emplace_back("const-view", analysis::MakeTypePerm(analysis::Permission::Const,
                                                                 analysis::MakeTypeString(analysis::StringState::View)));
      expected.emplace_back("none", nullptr);
      std::string accepted, rejected;
      for (const auto& [name, type] : expected) {
        const auto checked = analysis::CheckLiteralExpr(empty_ctx, literal, type);
        if (checked.ok) accepted += " " + name;
        else if (checked.diag_id) rejected += " " + name + ":" + std::string(*checked.diag_id);
      }
      std::cout << "\tok=" << accepted << "\tno=" << rejected << "\tnull="
                << (analysis::NullLiteralExpected(raw) ? '1' : '0')
                << (analysis::NullLiteralExpected(expected[0].second) ? '1' : '0');
    }
    if (token.kind == lexer::TokenKind::StringLiteral) {
      std::cout << "\tstr=";
      PrintHex(analysis::layout::DecodeStringLiteralBytes(token.lexeme));
    }
    std::cout << '\n';
  }
  return 0;
}

// A value of the type, chosen by the seed. The port's dump generates the same values.
analysis::layout::Value GenValue(const analysis::ScopeContext& ctx, const analysis::TypeRef& type,
                                 std::uint64_t seed, int depth) {
  namespace L = analysis::layout;
  using analysis::PtrState;
  L::Value out;
  out.node = L::UnitVal{};
  if (!type || depth > 6) return out;
  const std::uint64_t mixed = seed * 0x9E3779B97F4A7C15ull + 1;
  if (const auto* prim = std::get_if<analysis::TypePrim>(&type->node)) {
    const std::string& name = prim->name;
    if (name == "bool") out.node = L::BoolVal{(seed & 1) != 0};
    else if (name == "char") out.node = L::CharVal{static_cast<std::uint32_t>(0x41 + seed % 26)};
    else if (name == "f16" || name == "f32" || name == "f64") out.node = L::FloatVal{name, mixed};
    else if (name != "()" && name != "!") out.node = L::IntVal{name, core::UInt128FromU64(mixed)};
    return out;
  }
  if (const auto* perm = std::get_if<analysis::TypePerm>(&type->node)) return GenValue(ctx, perm->base, seed, depth + 1);
  if (const auto* refine = std::get_if<analysis::TypeRefine>(&type->node)) return GenValue(ctx, refine->base, seed, depth + 1);
  if (const auto* ptr = std::get_if<analysis::TypePtr>(&type->node)) {
    static const PtrState kStates[] = {PtrState::Valid, PtrState::Null, PtrState::Expired};
    const PtrState state = ptr->state ? *ptr->state : kStates[seed % 3];
    out.node = L::PtrVal{state, state == PtrState::Null ? 0 : 0x1000 + seed};
    return out;
  }
  if (const auto* raw = std::get_if<analysis::TypeRawPtr>(&type->node)) {
    out.node = L::RawPtrVal{raw->qual, 0x2000 + seed};
    return out;
  }
  if (const auto* tuple = std::get_if<analysis::TypeTuple>(&type->node)) {
    L::TupleVal value;
    for (std::size_t i = 0; i < tuple->elements.size(); ++i) {
      value.elements.push_back(GenValue(ctx, tuple->elements[i], seed + i + 1, depth + 1));
    }
    out.node = value;
    return out;
  }
  if (const auto* array = std::get_if<analysis::TypeArray>(&type->node)) {
    if (array->length > 16) return out;
    L::ArrayVal value;
    for (std::uint64_t i = 0; i < array->length; ++i) value.elements.push_back(GenValue(ctx, array->element, seed + i, depth + 1));
    out.node = value;
    return out;
  }
  if (std::holds_alternative<analysis::TypeSlice>(type->node)) {
    out.node = L::SliceVal{L::RawPtrVal{analysis::RawPtrQual::Imm, 0x3000 + seed}, seed};
    return out;
  }
  const auto range = [&](L::ValueRangeKind kind) {
    L::ValueRangeVal value;
    value.kind = kind;
    value.lo = seed % 100;
    value.hi = seed % 100 + 5;
    out.node = value;
    return out;
  };
  if (std::holds_alternative<analysis::TypeRange>(type->node)) return range(L::ValueRangeKind::Exclusive);
  if (std::holds_alternative<analysis::TypeRangeInclusive>(type->node)) return range(L::ValueRangeKind::Inclusive);
  if (std::holds_alternative<analysis::TypeRangeFrom>(type->node)) return range(L::ValueRangeKind::From);
  if (std::holds_alternative<analysis::TypeRangeTo>(type->node)) return range(L::ValueRangeKind::To);
  if (std::holds_alternative<analysis::TypeRangeToInclusive>(type->node)) return range(L::ValueRangeKind::ToInclusive);
  if (std::holds_alternative<analysis::TypeRangeFull>(type->node)) return range(L::ValueRangeKind::Full);
  if (const auto* path = analysis::AppliedTypePath(*type)) {
    const auto* args = analysis::AppliedTypeArgs(*type);
    const auto* decl = analysis::LookupEnumDecl(ctx, *path);
    if (!decl || decl->variants.empty() || !args) return out;
    const auto& variant = decl->variants[seed % decl->variants.size()];
    L::EnumVal value;
    value.variant = variant.name;
    if (variant.payload_opt) {
      if (const auto* tuple = std::get_if<ast::VariantPayloadTuple>(&*variant.payload_opt)) {
        L::EnumPayloadTupleVal payload;
        for (std::size_t i = 0; i < tuple->elements.size(); ++i) {
          const auto member = L::EnumTuplePayloadMemberLayout(ctx, *decl, variant, *args, i);
          payload.elements.push_back(GenValue(ctx, member ? member->type : nullptr, seed + i + 1, depth + 1));
        }
        value.payload = payload;
      } else if (const auto* record = std::get_if<ast::VariantPayloadRecord>(&*variant.payload_opt)) {
        L::EnumPayloadRecordVal payload;
        std::uint64_t i = 0;
        for (const auto& field : record->fields) {
          const auto member = L::EnumRecordPayloadMemberLayout(ctx, *decl, variant, *args, field.name);
          payload.fields.emplace_back(field.name, GenValue(ctx, member ? member->type : nullptr, seed + ++i, depth + 1));
        }
        value.payload = payload;
      }
    }
    out.node = value;
    return out;
  }
  if (std::holds_alternative<analysis::TypeDynamic>(type->node)) out.node = L::DynamicVal{seed, seed + 1};
  else if (std::holds_alternative<analysis::TypeString>(type->node)) out.node = L::StringVal{};
  else if (std::holds_alternative<analysis::TypeBytes>(type->node)) out.node = L::BytesVal{};
  return out;
}

void DumpValues(analysis::ScopeContext& ctx, const analysis::NameMapTable& name_maps) {
  namespace L = analysis::layout;
  for (const auto& module : ctx.sigma.mods) {
    std::cout << "X\t";
    D(std::cout, module.path);
    std::cout << '\n';
    ctx.current_module = module.path;
    const auto names = name_maps.find(analysis::PathKeyOf(module.path));
    ctx.scopes = {analysis::Scope{},
                  names == name_maps.end() ? analysis::Scope{} : names->second,
                  analysis::UniverseBindings()};
    std::vector<analysis::TypeRef> types;
    for (const auto& written : CollectWrittenTypes(module)) {
      const auto lowered = analysis::LowerType(ctx, written.type);
      if (lowered.ok) types.push_back(lowered.type);
    }
    const std::size_t count = std::min<std::size_t>(types.size(), 80);
    for (std::size_t i = 0; i < count; ++i) {
      const auto& type = types[i];
      std::cout << "V\t" << i << '\t' << Escape(analysis::TypeToString(type));
      for (const std::uint64_t seed : {std::uint64_t{i}, std::uint64_t{i + 17}}) {
        const auto bits = L::ValueBits(ctx, type, GenValue(ctx, type, seed, 0));
        std::cout << '\t';
        PrintHex(bits);
        if (bits) std::cout << (L::ValidValue(ctx, type, *bits) ? " v" : " n");
      }
      // A value of the next type, which mostly is not a value of this one.
      std::cout << '\t';
      PrintHex(L::ValueBits(ctx, type, GenValue(ctx, types[(i + 1) % types.size()], i, 0)));
      std::cout << '\t';
      const auto size = L::SizeOf(ctx, type);
      if (size && *size <= 4096) {
        std::vector<std::uint8_t> zeros(*size, 0), ones(*size, 0xFF), mixed(*size), one(*size, 1);
        for (std::size_t j = 0; j < mixed.size(); ++j) mixed[j] = static_cast<std::uint8_t>(j * 7 + 1);
        std::vector<std::uint8_t> longer(*size + 1, 0);
        std::cout << *size << (L::ValidValue(ctx, type, zeros) ? 'z' : '-') << (L::ValidValue(ctx, type, ones) ? 'f' : '-')
                  << (L::ValidValue(ctx, type, mixed) ? 'm' : '-') << (L::ValidValue(ctx, type, one) ? 'o' : '-')
                  << (L::ValidValue(ctx, type, longer) ? 'l' : '-');
      } else {
        std::cout << '-';
      }
      std::cout << '\n';
    }
  }
}

// ---- patterns; see `pattern_common.cpp` ----

// The patterns written directly in a body: those of its bindings, and of the pattern
// forms that are a statement, a binding's initialiser or the block's tail.
void CollectExprPatterns(const ast::ExprPtr& expr, std::vector<ast::PatternPtr>& out) {
  if (!expr) return;
  if (const auto* node = std::get_if<ast::IfCaseExpr>(&expr->node)) {
    for (const auto& clause : node->cases) out.push_back(clause.pattern);
  } else if (const auto* node = std::get_if<ast::IfIsExpr>(&expr->node)) {
    out.push_back(node->pattern);
  } else if (const auto* node = std::get_if<ast::LoopIterExpr>(&expr->node)) {
    out.push_back(node->pattern);
  }
}

void CollectBlockPatterns(const std::shared_ptr<ast::Block>& block, std::vector<ast::PatternPtr>& out) {
  if (!block) return;
  for (const auto& stmt : block->stmts) {
    if (const auto* node = std::get_if<ast::LetStmt>(&stmt)) {
      out.push_back(node->binding.pat);
      CollectExprPatterns(node->binding.init, out);
    } else if (const auto* node = std::get_if<ast::VarStmt>(&stmt)) {
      out.push_back(node->binding.pat);
      CollectExprPatterns(node->binding.init, out);
    } else if (const auto* node = std::get_if<ast::ExprStmt>(&stmt)) {
      CollectExprPatterns(node->value, out);
    }
  }
  CollectExprPatterns(block->tail_opt, out);
}

std::string IntroText(const analysis::IntroResult& res) {
  return res.ok ? "ok" : "fail:" + DiagOr(res.diag_id);
}

// The environment operations, on the bindings a pattern introduces.
void DumpEnvScript(std::size_t i, std::size_t j, const std::vector<std::pair<std::string, analysis::TypeRef>>& binds) {
  using namespace analysis;
  const TypeEnv e0 = PushScope(TypeEnv{});
  const auto r1 = IntroAll(e0, binds, ast::Mutability::Let, false);
  const TypeEnv e1 = PushScope(r1.ok ? r1.env : e0);
  const auto r2 = IntroAll(e1, binds, ast::Mutability::Var, true);
  const auto r3 = IntroAll(e1, binds, ast::Mutability::Let, false);
  const TypeEnv env = r2.ok ? r2.env : e1;
  const auto r4 = IntroAll(env, binds, ast::Mutability::Var, true);
  const auto r5 = IntroAll(TypeEnv{}, binds, ast::Mutability::Let, false);
  std::cout << "PE\t" << i << '\t' << j << '\t' << IntroText(r1) << ' ' << IntroText(r2) << ' ' << IntroText(r3) << ' '
            << IntroText(r4) << ' ' << IntroText(r5) << '\t';
  if (!binds.empty()) {
    const auto& name = binds[0].first;
    if (const auto bound = BindOf(env, name)) {
      std::cout << (bound->mut == ast::Mutability::Var ? 'V' : 'L') << ' ' << TypeText(bound->type) << ' '
                << TypeText(StableBindingType(*bound));
    } else {
      std::cout << "unbound";
    }
    const auto outer = MutOf(r1.ok ? r1.env : e0, name);
    std::cout << ' ' << (outer ? (*outer == ast::Mutability::Var ? 'V' : 'L') : '-')
              << (HasHeapProvenance(env, name) ? 'h' : '-');
  }
  std::cout << '\t' << PopScope(env).scopes.size() << ProjectTypeEnvToDepth(env, 1).scopes.size()
            << ProjectTypeEnvToDepth(env, 5).scopes.size() << PopScope(PopScope(PopScope(env))).scopes.size()
            << (GpuContext(env) ? 'g' : '-') << '\n';
}

// Reserved names, provenance seeds and staleness: independent of the module.
void DumpEnvFixed() {
  using namespace analysis;
  const std::vector<std::pair<std::string, TypeRef>> reserved = {{"gen_tmp", MakeTypePrim("i32")}};
  const TypeEnv e0 = PushScope(TypeEnv{});
  std::cout << "PG\t" << IntroText(IntroAll(e0, reserved, ast::Mutability::Let, false)) << ' '
            << IntroText(IntroAll(e0, reserved, ast::Mutability::Let, true)) << ' '
            << IntroText(IntroAll(e0, {}, ast::Mutability::Var, true)) << '\t';
  for (const auto kind : {ProvenanceKind::Global, ProvenanceKind::Stack, ProvenanceKind::Heap, ProvenanceKind::Region,
                          ProvenanceKind::Bottom, ProvenanceKind::Param}) {
    TypeBinding binding;
    ApplyBindingProvenanceSeed(binding, kind, std::string("Arena"));
    std::cout << static_cast<int>(binding.provenance_kind) << (binding.provenance_region ? *binding.provenance_region : "-")
              << static_cast<int>(NormalizeBindingProvenanceSeed(kind)) << ' ';
  }
  TypeEnv env = PushScope(e0);
  TypeBinding derived;
  derived.derived_from_shared = true;
  ApplyBindingProvenanceSeed(derived, ProvenanceKind::Heap);
  env.scopes[0].emplace(IdKeyOf("outer"), derived);
  env.scopes[1].emplace(IdKeyOf("inner"), TypeBinding{});
  env.parallel_context = ParallelContextKind::Gpu;
  MarkSharedDerivedBindingsStale(env);
  std::cout << '\t' << (BindOf(env, "outer")->stale_after_release ? 's' : '-')
            << (BindOf(env, "inner")->stale_after_release ? 's' : '-') << (HasHeapProvenance(env, "outer") ? 'h' : '-')
            << (HasHeapProvenance(env, "inner") ? 'h' : '-') << (HasHeapProvenance(env, "none") ? 'h' : '-')
            << (GpuContext(env) ? 'g' : '-') << (ParallelContext(env) ? 'p' : '-') << '\n';
}

void DumpPatterns(analysis::ScopeContext& ctx, const analysis::NameMapTable& name_maps) {
  DumpEnvFixed();
  for (const auto& module : ctx.sigma.mods) {
    std::cout << "X\t";
    D(std::cout, module.path);
    std::cout << '\n';
    ctx.current_module = module.path;
    const auto names = name_maps.find(analysis::PathKeyOf(module.path));
    ctx.scopes = {analysis::Scope{},
                  names == name_maps.end() ? analysis::Scope{} : names->second,
                  analysis::UniverseBindings()};
    std::vector<analysis::TypeRef> types;
    for (const auto& written : CollectWrittenTypes(module)) {
      const auto lowered = analysis::LowerType(ctx, written.type);
      if (lowered.ok) types.push_back(lowered.type);
    }
    std::vector<ast::PatternPtr> patterns;
    for (const auto& item : module.items) {
      if (const auto* node = std::get_if<ast::StaticDecl>(&item)) {
        patterns.push_back(node->binding.pat);
      } else if (const auto* node = std::get_if<ast::ProcedureDecl>(&item)) {
        CollectBlockPatterns(node->body, patterns);
      } else if (const auto* node = std::get_if<ast::RecordDecl>(&item)) {
        for (const auto& member : node->members) {
          if (const auto* method = std::get_if<ast::MethodDecl>(&member)) CollectBlockPatterns(method->body, patterns);
        }
      }
    }
    const std::size_t type_count = std::min<std::size_t>(types.size(), 48);
    const std::size_t pattern_count = std::min<std::size_t>(patterns.size(), 96);
    for (std::size_t j = 0; j < type_count; ++j) {
      std::cout << "PY\t" << j << '\t' << Escape(analysis::TypeToString(types[j])) << '\n';
    }
    for (std::size_t i = 0; i < pattern_count; ++i) {
      const auto& pattern = patterns[i];
      std::cout << "PT\t" << i << '\t' << (pattern ? pattern->node.index() : 99);
      for (std::size_t j = 0; j < type_count; ++j) {
        const auto typed = analysis::TypePatternAgainstType(ctx, pattern, types[j]);
        const bool irrefutable = analysis::IrrefutablePattern(ctx, pattern, types[j]);
        const bool covers = analysis::EnumPatternCoversVariant(ctx, pattern, types[j]);
        const bool covers_state = analysis::ModalPatternCoversState(ctx, pattern, types[j]);
        // The common outcome, a pattern that does not fit and names no rule, is left out.
        if (!typed.ok && !typed.diag_id && !irrefutable && !covers && !covers_state) continue;
        std::cout << '\t' << j << ':';
        if (typed.ok) {
          std::cout << "ok";
          for (const auto& [name, type] : typed.bindings) {
            std::cout << ' ' << name << '=' << Escape(analysis::TypeToString(type));
          }
        } else {
          std::cout << "fail " << DiagOr(typed.diag_id);
        }
        std::cout << " /" << (irrefutable ? 'i' : '-') << (covers ? 'e' : '-') << (covers_state ? 'm' : '-');
      }
      std::cout << '\n';
      std::vector<analysis::IdKey> names;
      if (pattern) analysis::CollectPatNames(*pattern, names);
      std::cout << "PN\t" << i << '\t';
      for (const auto& name : names) std::cout << name << ',';
      std::cout << '\t' << (analysis::DistinctNames(names) ? 'd' : '-') << '\n';
      // The same pattern as a binding. Its commonest rejection is printed once.
      std::cout << "PS\t" << i;
      std::optional<std::size_t> first_ok;
      std::vector<std::pair<std::string, analysis::TypeRef>> first_binds;
      for (std::size_t j = 0; j < type_count; ++j) {
        const auto typed = analysis::TypePattern(ctx, pattern, types[j]);
        if (!typed.ok && (!typed.diag_id || (*typed.diag_id == "Let-Refutable-Pattern-Err" && j != 0))) continue;
        std::cout << '\t' << j << ':';
        if (typed.ok) {
          std::cout << "ok";
          for (const auto& [name, type] : typed.bindings) std::cout << ' ' << name << '=' << Escape(analysis::TypeToString(type));
          if (!first_ok) {
            first_ok = j;
            first_binds = typed.bindings;
          }
        } else {
          std::cout << "fail " << DiagOr(typed.diag_id);
        }
      }
      std::cout << '\n';
      if (first_ok) DumpEnvScript(i, *first_ok, first_binds);
    }
  }
}

// ---- body typing; see `TypeProcedureDeclBody` ----

void DumpTypedBody(analysis::ScopeContext& proc_ctx, analysis::TypeEnv& env, const analysis::TypeRef& return_type,
                   const std::optional<ast::ContractClause>* contract, const std::optional<analysis::TypePath>& class_path,
                   const std::shared_ptr<analysis::StaticProofContext>& proof_ctx, const ast::Block& body, bool with_env);

// Types the body of a procedure as declaration typing does, up to and including the
// block: type parameters and parameters in scope, the declared return type expected.
// One line per body, so that a port that cannot type a body yet can say so in its place.
void DumpBody(const analysis::ScopeContext& ctx, const std::string& name,
              const std::optional<ast::GenericParams>& generic_params, const std::vector<ast::Param>& params,
              const std::shared_ptr<ast::Type>& return_type_opt, const std::optional<ast::ContractClause>& contract,
              const std::shared_ptr<ast::Block>& body) {
  using namespace analysis;
  if (!body) return;
  std::cout << "B\t" << name << '\t';
  ScopeContext proc_ctx = ctx;
  proc_ctx.sigma_source = ctx.sigma_source ? ctx.sigma_source : &ctx.sigma;
  proc_ctx.scopes = BindTypeParams(ctx, generic_params);
  TypeEnv env;
  env.scopes.emplace_back();
  for (const auto& param : params) {
    const auto lowered = LowerType(proc_ctx, param.type);
    if (!lowered.ok) {
      std::cout << "param-fail\t" << DiagOr(lowered.diag_id) << '\n';
      return;
    }
    TypeBinding binding;
    binding.mut = ast::Mutability::Let;
    binding.type = lowered.type;
    binding.storage_type = lowered.type;
    binding.provenance_kind = BindingProvenanceSeedKind::Param;
    env.scopes.back()[IdKeyOf(param.name)] = std::move(binding);
  }
  TypeRef return_type = MakeTypePrim("()");
  if (return_type_opt) {
    const auto lowered = LowerType(proc_ctx, return_type_opt);
    if (!lowered.ok) {
      std::cout << "return-fail\t" << DiagOr(lowered.diag_id) << '\n';
      return;
    }
    return_type = lowered.type;
  }
  DumpTypedBody(proc_ctx, env, return_type, &contract, std::nullopt, nullptr, *body, true);
}

// One store of facts recorded while a body is typed, as its size and a hash of its
// entries in order; `UV_STORE_VERBOSE` prints the entries themselves after the line.
// The reference keys its expression stores by the address of the syntax node, also for
// nodes it synthesizes and frees while typing. A later node at the same address would
// then be answered from the stale entry, as the allocator happens to decide. While
// bodies are typed the memory of expression nodes is therefore not given back, so that
// no two nodes ever share an address.
bool g_keep_expr_memory = false;
const std::size_t kSharedExprSize =
    sizeof(std::_Sp_counted_ptr_inplace<ast::Expr, std::allocator<void>, __gnu_cxx::__default_lock_policy>);

std::vector<std::string>* g_store_lines = nullptr;
// The expressions of the project's modules. An entry for any other expression is one
// the checker synthesized and has freed since; its address says nothing.
ExprSet g_syntax_exprs;

void PrintStore(char tag, std::vector<std::string> entries) {
  std::sort(entries.begin(), entries.end());
  std::uint64_t hash = 1469598103934665603ull;
  for (const auto& entry : entries) {
    for (const unsigned char byte : entry) hash = (hash ^ byte) * 1099511628211ull;
    hash = (hash ^ 0x0a) * 1099511628211ull;
    if (g_store_lines) g_store_lines->push_back(std::string("S\t") + tag + '\t' + entry);
  }
  std::cout << entries.size() << ':' << std::hex << hash << std::dec;
}

std::string SpanKey(const ast::Expr* expr) {
  return std::to_string(expr->span.start_offset) + '-' + std::to_string(expr->span.end_offset);
}

// The block of a body under its typing context, and the line that reports it. A
// transition is typed without the environment reference and the diagnostic stream.
void DumpTypedBody(analysis::ScopeContext& proc_ctx, analysis::TypeEnv& env, const analysis::TypeRef& return_type,
                   const std::optional<ast::ContractClause>* contract, const std::optional<analysis::TypePath>& class_path,
                   const std::shared_ptr<analysis::StaticProofContext>& proof_ctx, const ast::Block& body, bool with_env) {
  using namespace analysis;
  core::DiagnosticStream diags;
  StmtTypeContext type_ctx;
  type_ctx.return_type = return_type;
  proc_ctx.diagnostics = &diags;
  // The stores the type checker's entry point sets up; they are also read back.
  ExprTypeMap expr_types;
  ExprValueTypeMap expr_value_types;
  DynamicRefineExprMap dynamic_refine_checks;
  GenericCallSubstMap generic_call_substs;
  SelectedCallTargetMap selected_call_targets;
  proc_ctx.expr_types = &expr_types;
  proc_ctx.expr_value_types = &expr_value_types;
  proc_ctx.dynamic_refine_checks = &dynamic_refine_checks;
  proc_ctx.generic_call_substs = &generic_call_substs;
  proc_ctx.selected_call_targets = &selected_call_targets;
  if (with_env) {
    type_ctx.diags = &diags;
    type_ctx.env_ref = &env;
  }
  if (contract && contract->has_value()) type_ctx.contract = &**contract;
  type_ctx.current_class_path = class_path;
  type_ctx.proof_ctx = proof_ctx;
  ExprTypeFn type_expr = [&](const ast::ExprPtr& inner) { return TypeExpr(proc_ctx, type_ctx, inner, env); };
  IdentTypeFn type_ident = [&](std::string_view ident) -> ExprTypeResult {
    return expr::TypeIdentifierExprImpl(proc_ctx, ast::IdentifierExpr{std::string(ident)}, env);
  };
  PlaceTypeFn type_place = [&](const ast::ExprPtr& inner) { return TypePlace(proc_ctx, type_ctx, inner, env); };
  const auto result = TypeBlock(proc_ctx, type_ctx, body, env, type_expr, type_ident, type_place, with_env ? &env : nullptr);
  std::cout << (result.ok ? "ok" : "fail") << '\t' << DiagOr(result.diag_id) << '\t' << Escape(result.diag_detail) << '\t'
            << TypeText(result.type) << '\t';
  if (result.diag_span) std::cout << result.diag_span->start_offset << '-' << result.diag_span->end_offset;
  else std::cout << '-';
  std::cout << '\t' << env.scopes.size() << '\t';
  for (const auto& id : result.diagnostic_obligation_ids) std::cout << id << ',';
  std::cout << '\t';
  PrintDiagList(diags);
  std::vector<std::string> store_lines;
  if (std::getenv("UV_STORE_VERBOSE")) g_store_lines = &store_lines;
  std::vector<std::string> entries;
  for (const auto& [expr, type] : expr_types) if (g_syntax_exprs.count(expr)) entries.push_back(SpanKey(expr) + '=' + TypeText(type));
  std::cout << '\t';
  PrintStore('T', std::move(entries));
  entries.clear();
  for (const auto& [expr, type] : expr_value_types) if (g_syntax_exprs.count(expr)) entries.push_back(SpanKey(expr) + '=' + TypeText(type));
  std::cout << ' ';
  PrintStore('V', std::move(entries));
  entries.clear();
  for (const auto& [expr, types] : dynamic_refine_checks) {
    if (!g_syntax_exprs.count(expr)) continue;
    std::string entry = SpanKey(expr) + '=';
    for (const auto& type : types) entry += TypeText(type) + ';';
    entries.push_back(std::move(entry));
  }
  std::cout << ' ';
  PrintStore('D', std::move(entries));
  entries.clear();
  for (const auto& [call, subst] : generic_call_substs) {
    std::string entry;
    for (const auto& [name, type] : subst) entry += name + '=' + TypeText(type) + ';';
    entries.push_back(std::move(entry));
  }
  std::cout << ' ';
  PrintStore('G', std::move(entries));
  entries.clear();
  for (const auto& [call, target] : selected_call_targets) {
    std::string entry;
    for (const auto& seg : target.module_path) entry += seg + "::";
    entries.push_back(entry + (target.proc ? target.proc->name : std::string("-")));
  }
  std::cout << ' ';
  PrintStore('C', std::move(entries));
  std::cout << '\n';
  g_store_lines = nullptr;
  for (const auto& line : store_lines) std::cout << line << '\n';
}

// The body of a method or transition under the bindings of its signature, as the typing
// of record, class and modal declarations sets it up; `self` of a `unique` shorthand
// receiver is mutable.
void DumpSignatureBody(const analysis::ScopeContext& ctx, const std::string& name, const analysis::SignatureResult& sig,
                       bool self_var, const analysis::TypeRef& return_type,
                       const std::optional<ast::ContractClause>* contract, const std::optional<analysis::TypePath>& class_path,
                       const std::shared_ptr<analysis::StaticProofContext>& proof_ctx,
                       const std::shared_ptr<ast::Block>& body, bool with_env) {
  using namespace analysis;
  if (!body) return;
  std::cout << "B\t" << name << '\t';
  if (!sig.ok) {
    std::cout << "sig-fail\t" << DiagOr(sig.diag_id) << '\n';
    return;
  }
  ScopeContext method_ctx = ctx;
  method_ctx.sigma_source = ctx.sigma_source ? ctx.sigma_source : &ctx.sigma;
  TypeEnv env;
  env.scopes.emplace_back();
  for (const auto& binding : sig.bindings) {
    const bool is_var = self_var && binding.first == "self";
    env.scopes.back()[IdKeyOf(binding.first)] = {is_var ? ast::Mutability::Var : ast::Mutability::Let, binding.second};
  }
  DumpTypedBody(method_ctx, env, return_type, contract, class_path, proof_ctx, *body, with_env);
}

bool UniqueShorthand(const ast::Receiver& receiver) {
  const auto* shorthand = std::get_if<ast::ReceiverShorthand>(&receiver);
  return shorthand && shorthand->perm == ast::ReceiverPerm::Unique;
}

bool ConstShorthand(const ast::Receiver& receiver) {
  const auto* shorthand = std::get_if<ast::ReceiverShorthand>(&receiver);
  return shorthand && shorthand->perm == ast::ReceiverPerm::Const && !shorthand->mode_opt.has_value();
}

analysis::TypePath DeclPath(const ast::ModulePath& module_path, const std::string& name) {
  analysis::TypePath path(module_path.begin(), module_path.end());
  path.push_back(name);
  return path;
}

// See `TypeRecordDecl`: the record's associated types stand in for `Self::Name`, and a
// method with a plain `~` receiver may assume the record's invariant.
void DumpRecordMethodBodies(const analysis::ScopeContext& ctx, const ast::RecordDecl& decl, const ast::ModulePath& module_path) {
  using namespace analysis;
  const TypeRef self_type = MakeTypePath(DeclPath(module_path, decl.name));
  TypeSubst assoc_subst;
  std::optional<std::string_view> assoc_diag;
  bool assoc_ok = true;
  for (const auto& member : decl.members) {
    const auto* assoc = std::get_if<ast::AssociatedTypeDecl>(&member);
    if (!assoc || !assoc_ok) continue;
    if (!assoc->default_type) {
      if (!decl.implements.empty()) {
        assoc_ok = false;
        assoc_diag = "E-TYP-2503";
      }
      continue;
    }
    auto lowered = LowerType(ctx, assoc->default_type);
    if (lowered.ok) {
      const auto wf = TypeWF(ctx, lowered.type);
      if (!wf.ok) lowered = {false, wf.diag_id, {}};
    }
    if (!lowered.ok) {
      assoc_ok = false;
      assoc_diag = lowered.diag_id;
      continue;
    }
    assoc_subst[assoc->name] = SubstSelfType(self_type, lowered.type, &assoc_subst);
  }
  for (const auto& member : decl.members) {
    const auto* method = std::get_if<ast::MethodDecl>(&member);
    if (!method || !method->body) continue;
    const std::string name = decl.name + "::" + method->name;
    if (!assoc_ok) {
      std::cout << "B\t" << name << "\tassoc-fail\t" << DiagOr(assoc_diag) << '\n';
      continue;
    }
    const auto sig = BuildMethodSignature(ctx, self_type, method->receiver, method->params, method->return_type_opt, &assoc_subst);
    std::shared_ptr<StaticProofContext> proof_ctx;
    if (decl.invariant_opt.has_value() && ConstShorthand(method->receiver)) {
      proof_ctx = ExtendProofContextWithPredicateAt(proof_ctx, decl.invariant_opt->predicate, method->body->span);
    }
    DumpSignatureBody(ctx, name, sig, UniqueShorthand(method->receiver), sig.return_type, &method->contract, std::nullopt,
                      proof_ctx, method->body, true);
  }
}

// See `TypeClassDecl`: `Self` stays a variable, and the class is the current one.
void DumpClassMethodBodies(const analysis::ScopeContext& ctx, const ast::ClassDecl& decl, const ast::ModulePath& module_path) {
  using namespace analysis;
  const TypePath class_path = DeclPath(module_path, decl.name);
  for (const auto& item : decl.items) {
    const auto* method = std::get_if<ast::ClassMethodDecl>(&item);
    if (!method || !method->body_opt) continue;
    const auto sig = BuildMethodSignature(ctx, SelfVarType(), method->receiver, method->params, method->return_type_opt);
    DumpSignatureBody(ctx, decl.name + "::" + method->name, sig, UniqueShorthand(method->receiver), sig.return_type,
                      &method->contract, class_path, nullptr, method->body_opt, true);
  }
}

// The part of a modal invariant that speaks of one state; see `StateInvariantPredicateFor`.
ast::ExprPtr StateInvariantOf(const ast::TypeInvariant& invariant, std::string_view state_name) {
  if (!invariant.predicate) return nullptr;
  const auto* if_case = std::get_if<ast::IfCaseExpr>(&invariant.predicate->node);
  if (!if_case) return invariant.predicate;
  const auto* ident = if_case->scrutinee ? std::get_if<ast::IdentifierExpr>(&if_case->scrutinee->node) : nullptr;
  if (!ident || !analysis::IdEq(ident->name, "self")) return invariant.predicate;
  for (const auto& arm : if_case->cases) {
    if (!arm.pattern) continue;
    const auto* modal_pattern = std::get_if<ast::ModalPattern>(&arm.pattern->node);
    if (modal_pattern && analysis::IdEq(modal_pattern->state, state_name)) return arm.body;
  }
  return invariant.predicate;
}

// See `TypeModalDecl`: the receiver is the state, applied to the modal's own parameters;
// a transition returns its target state and is typed without the environment reference.
void DumpModalBodies(const analysis::ScopeContext& ctx, const ast::ModalDecl& decl, const ast::ModulePath& module_path) {
  using namespace analysis;
  const TypePath type_path = DeclPath(module_path, decl.name);
  std::vector<TypeRef> self_args;
  if (decl.generic_params) {
    for (const auto& param : decl.generic_params->params) self_args.push_back(MakeTypePath(TypePath{param.name}));
  }
  for (const auto& state : decl.states) {
    const TypeRef state_type = MakeTypeModalState(type_path, state.name, self_args);
    for (const auto& member : state.members) {
      const auto* method = std::get_if<ast::StateMethodDecl>(&member);
      if (!method || !method->body) continue;
      const auto sig = BuildMethodSignature(ctx, state_type, method->receiver, method->params, method->return_type_opt);
      std::shared_ptr<StaticProofContext> proof_ctx;
      if (decl.invariant_opt.has_value() && ConstShorthand(method->receiver)) {
        proof_ctx = ExtendProofContextWithPredicateAt(proof_ctx, StateInvariantOf(*decl.invariant_opt, state.name),
                                                      method->body->span);
      }
      DumpSignatureBody(ctx, decl.name + "@" + state.name + "::" + method->name, sig, UniqueShorthand(method->receiver),
                        sig.return_type, &method->contract, std::nullopt, proof_ctx, method->body, true);
    }
    for (const auto& member : state.members) {
      const auto* transition = std::get_if<ast::TransitionDecl>(&member);
      if (!transition || !transition->body) continue;
      const TypeRef target_type = MakeTypeModalState(type_path, transition->target_state, self_args);
      const auto sig = BuildTransitionSignature(ctx, state_type, target_type, transition->params);
      DumpSignatureBody(ctx, decl.name + "@" + state.name + "->" + transition->target_state + "::" + transition->name, sig,
                        false, target_type, nullptr, std::nullopt, nullptr, transition->body, false);
    }
  }
}

void DumpBodies(analysis::ScopeContext& ctx, const analysis::NameMapTable& name_maps) {
  g_keep_expr_memory = true;
  g_syntax_exprs.clear();
  for (const auto& module : ctx.sigma.mods) W(g_syntax_exprs, module.items);
  for (const auto& module : ctx.sigma.mods) {
    std::cout << "X\t";
    D(std::cout, module.path);
    std::cout << '\n';
    ctx.current_module = module.path;
    const auto names = name_maps.find(analysis::PathKeyOf(module.path));
    ctx.scopes = {analysis::Scope{},
                  names == name_maps.end() ? analysis::Scope{} : names->second,
                  analysis::UniverseBindings()};
    for (const auto& item : module.items) {
      if (const auto* node = std::get_if<ast::ProcedureDecl>(&item)) {
        DumpBody(ctx, node->name, node->generic_params, node->params, node->return_type_opt, node->contract, node->body);
      } else if (const auto* record = std::get_if<ast::RecordDecl>(&item)) {
        DumpRecordMethodBodies(ctx, *record, module.path);
      } else if (const auto* class_decl = std::get_if<ast::ClassDecl>(&item)) {
        DumpClassMethodBodies(ctx, *class_decl, module.path);
      } else if (const auto* modal = std::get_if<ast::ModalDecl>(&item)) {
        DumpModalBodies(ctx, *modal, module.path);
      }
    }
  }
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
  if (!g_types_mode) {
    PrintDiagsFull(diags);
    DumpNameMaps(name_maps.name_maps);
  }
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
  if (g_types_mode) {
    if (!resolved.ok) {
      std::cout << "STOP\tresolve\n";
      return 0;
    }
    // As the driver does: the declaration tables are rebuilt from the resolved modules.
    ctx.sigma.mods = resolved.modules;
    analysis::PopulateSigma(ctx);
    if (g_body_mode) {
      DumpBodies(ctx, name_maps.name_maps);
    } else if (g_pat_mode) {
      DumpPatterns(ctx, name_maps.name_maps);
    } else if (g_val_mode) {
      DumpValues(ctx, name_maps.name_maps);
    } else if (g_rel_mode) {
      DumpRelations(ctx, name_maps.name_maps);
    } else {
      DumpTypes(ctx, name_maps.name_maps);
    }
    return 0;
  }
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
  g_rel_mode = argc >= 3 && std::string_view(argv[1]) == "relations";
  g_val_mode = argc >= 3 && std::string_view(argv[1]) == "values";
  g_pat_mode = argc >= 3 && std::string_view(argv[1]) == "patterns";
  g_body_mode = argc >= 3 && std::string_view(argv[1]) == "bodies";
  g_types_mode = g_rel_mode || g_val_mode || g_pat_mode || g_body_mode || (argc >= 3 && std::string_view(argv[1]) == "types");
  const bool resolve_mode =
      g_types_mode || (argc >= 3 && std::string_view(argv[1]) == "resolve");
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
  const bool consts_mode = argc >= 3 && std::string_view(argv[1]) == "consts";
  if (ast_mode || consts_mode || (argc >= 3 && std::string_view(argv[1]) == "tokens")) {
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
        // Encoding every literal of a large file takes longer than lexing it.
        alarm(consts_mode ? 300 : 3);
        int rc = 0;
        try {
          rc = ast_mode ? DumpAst(label, path) : consts_mode ? DumpConsts(label, path) : DumpTokens(label, path);
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

// See `g_keep_expr_memory`. Sized deallocation is what `std::make_shared` uses.
void* operator new(std::size_t size) {
  if (void* p = std::malloc(size ? size : 1)) return p;
  throw std::bad_alloc();
}
void operator delete(void* p) noexcept { std::free(p); }
void operator delete(void* p, std::size_t size) noexcept {
  if (g_keep_expr_memory && size == kSharedExprSize) return;
  std::free(p);
}
