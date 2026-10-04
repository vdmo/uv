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
  std::cerr << "usage: uv-oracle unicode | tokens <list-file> | ast <list-file>\n";
  return 2;
}
