// External scanner for the Ultraviolet grammar.
//
// `_newline`: a line break that terminates a statement. It is produced only where the
// parse state can take a terminator, and not when the next token continues the line
// (`.`, `::`, `~>` or `else`), which mirrors the reference lexer's newline filter. Line
// breaks after operators, commas and inside parentheses are never terminators because the
// grammar has no terminator in those states. The token is zero-width and sits at the first
// token or comment after the line break.
//
// `block_comment`: `/* ... */`, which nests.
//
// `_option_bracket`: the `[` that opens the option list of `parallel`, `spawn`, `dispatch`
// and key blocks. It is told apart from an index expression by what follows it: an option
// name and then `:`, `,` or `]`.
//
// `_generic_open`: the `<` that opens generic arguments in an expression (`f<T>(x)`,
// `Box<T>@Full { .. }`). As in the reference parser, `<` is a generic bracket only when the
// balanced `<...>` holds nothing but type syntax and is followed by `(` or `@`; otherwise
// it is the comparison operator.

#include "tree_sitter/alloc.h"
#include "tree_sitter/parser.h"

#include <stdbool.h>
#include <stdint.h>

enum TokenType { NEWLINE, BLOCK_COMMENT, OPTION_BRACKET, GENERIC_OPEN, ERROR_SENTINEL };

typedef struct {
  // A line break was seen before a block comment that has since been returned; the
  // terminator decision is made at the next token.
  bool pending_newline;
} Scanner;

void *tree_sitter_ultraviolet_external_scanner_create(void) {
  return ts_calloc(1, sizeof(Scanner));
}

void tree_sitter_ultraviolet_external_scanner_destroy(void *payload) { ts_free(payload); }

unsigned tree_sitter_ultraviolet_external_scanner_serialize(void *payload, char *buffer) {
  buffer[0] = (char)((Scanner *)payload)->pending_newline;
  return 1;
}

void tree_sitter_ultraviolet_external_scanner_deserialize(void *payload, const char *buffer,
                                                          unsigned length) {
  ((Scanner *)payload)->pending_newline = length > 0 && buffer[0] != 0;
}

static bool is_horizontal_space(int32_t c) {
  return c == ' ' || c == '\t' || c == '\r' || c == '\f' || c == 0x0B || c == 0xFEFF;
}

static bool is_identifier_char(int32_t c) {
  return c == '_' || (c >= '0' && c <= '9') || (c >= 'a' && c <= 'z') ||
         (c >= 'A' && c <= 'Z') || c >= 0x80;
}

static void skip_blank(TSLexer *lexer) {
  while (is_horizontal_space(lexer->lookahead) || lexer->lookahead == '\n') {
    lexer->advance(lexer, false);
  }
}

// Consumes the rest of a block comment whose opening `/*` has been consumed.
static void finish_block_comment(TSLexer *lexer) {
  unsigned depth = 1;
  while (depth > 0 && !lexer->eof(lexer)) {
    int32_t c = lexer->lookahead;
    lexer->advance(lexer, false);
    if (c == '/' && lexer->lookahead == '*') {
      lexer->advance(lexer, false);
      depth++;
    } else if (c == '*' && lexer->lookahead == '/') {
      lexer->advance(lexer, false);
      depth--;
    }
  }
}

// Looks past blank space and comments and reports whether the next token continues the
// previous line. Only ever called after the token end has been marked.
static bool next_token_continues_line(TSLexer *lexer) {
  for (;;) {
    skip_blank(lexer);
    if (lexer->lookahead != '/') {
      break;
    }
    lexer->advance(lexer, false);
    if (lexer->lookahead == '/') {
      while (!lexer->eof(lexer) && lexer->lookahead != '\n') {
        lexer->advance(lexer, false);
      }
    } else if (lexer->lookahead == '*') {
      lexer->advance(lexer, false);
      finish_block_comment(lexer);
    } else {
      return false;  // a division operator
    }
  }
  switch (lexer->lookahead) {
    case '.':
      lexer->advance(lexer, false);
      return lexer->lookahead != '.';
    case ':':
      lexer->advance(lexer, false);
      return lexer->lookahead == ':';
    case '~':
      lexer->advance(lexer, false);
      return lexer->lookahead == '>';
    case 'e': {
      const char *rest = "lse";
      lexer->advance(lexer, false);
      for (; *rest != 0; rest++) {
        if (lexer->lookahead != *rest) {
          return false;
        }
        lexer->advance(lexer, false);
      }
      return !is_identifier_char(lexer->lookahead);
    }
    default:
      return false;
  }
}

static const char *const OPTION_NAMES[] = {
    "cancel", "name", "workgroup", "workgroups", "affinity", "priority",
    "reduce", "ordered", "chunk",    NULL,
};

static bool is_option_name(const char *word) {
  for (const char *const *name = OPTION_NAMES; *name != NULL; name++) {
    const char *a = *name;
    const char *b = word;
    while (*a != 0 && *a == *b) {
      a++;
      b++;
    }
    if (*a == 0 && *b == 0) {
      return true;
    }
  }
  return false;
}

// With the lexer at `[`: consumes it, marks the token end, and reports whether an option
// list follows.
static bool scan_option_bracket(TSLexer *lexer) {
  lexer->advance(lexer, false);
  lexer->mark_end(lexer);
  skip_blank(lexer);
  char word[16];
  unsigned length = 0;
  while (is_identifier_char(lexer->lookahead) && lexer->lookahead < 0x80) {
    if (length + 1 >= sizeof(word)) {
      return false;
    }
    word[length++] = (char)lexer->lookahead;
    lexer->advance(lexer, false);
  }
  word[length] = 0;
  if (!is_option_name(word)) {
    return false;
  }
  skip_blank(lexer);
  return lexer->lookahead == ':' || lexer->lookahead == ',' || lexer->lookahead == ']';
}

// With the lexer at `<`: consumes it, marks the token end, and reports whether balanced
// generic arguments followed by `(` or `@` come next.
static bool scan_generic_open(TSLexer *lexer) {
  lexer->advance(lexer, false);
  lexer->mark_end(lexer);
  unsigned depth = 1;
  for (unsigned budget = 512; budget > 0 && depth > 0; budget--) {
    int32_t c = lexer->lookahead;
    if (lexer->eof(lexer)) {
      return false;
    }
    if (c == '<') {
      depth++;
    } else if (c == '>') {
      depth--;
    } else if (c == '-') {
      lexer->advance(lexer, false);
      if (lexer->lookahead != '>') {
        return false;
      }
    } else if (!(is_identifier_char(c) || is_horizontal_space(c) || c == '\n' || c == ',' ||
                 c == ':' || c == ';' || c == '(' || c == ')' || c == '[' || c == ']' ||
                 c == '*' || c == '@' || c == '$' || c == '!' || c == '|')) {
      return false;
    }
    lexer->advance(lexer, false);
  }
  if (depth != 0) {
    return false;
  }
  while (is_horizontal_space(lexer->lookahead)) {
    lexer->advance(lexer, false);
  }
  return lexer->lookahead == '(' || lexer->lookahead == '@';
}

bool tree_sitter_ultraviolet_external_scanner_scan(void *payload, TSLexer *lexer,
                                                   const bool *valid_symbols) {
  Scanner *scanner = (Scanner *)payload;
  bool recovering = valid_symbols[ERROR_SENTINEL];

  bool saw_newline = scanner->pending_newline;
  while (is_horizontal_space(lexer->lookahead) || lexer->lookahead == '\n') {
    saw_newline = saw_newline || lexer->lookahead == '\n';
    lexer->advance(lexer, true);
  }

  // A block comment is returned first; a line break before it stays pending.
  if (lexer->lookahead == '/' && valid_symbols[BLOCK_COMMENT]) {
    lexer->mark_end(lexer);
    lexer->advance(lexer, false);
    if (lexer->lookahead == '*') {
      lexer->advance(lexer, false);
      finish_block_comment(lexer);
      lexer->mark_end(lexer);
      scanner->pending_newline = saw_newline;
      lexer->result_symbol = BLOCK_COMMENT;
      return true;
    }
    scanner->pending_newline = false;
    if (lexer->lookahead != '/' || !saw_newline || !valid_symbols[NEWLINE] || recovering) {
      return false;
    }
    // A line comment after a line break: the decision depends on the token after it.
    while (!lexer->eof(lexer) && lexer->lookahead != '\n') {
      lexer->advance(lexer, false);
    }
    if (next_token_continues_line(lexer)) {
      return false;
    }
    lexer->result_symbol = NEWLINE;
    return true;
  }

  scanner->pending_newline = false;
  if (recovering) {
    return false;
  }

  if (saw_newline && valid_symbols[NEWLINE]) {
    lexer->mark_end(lexer);
    if (next_token_continues_line(lexer)) {
      return false;
    }
    lexer->result_symbol = NEWLINE;
    return true;
  }

  if (lexer->lookahead == '[' && valid_symbols[OPTION_BRACKET]) {
    if (scan_option_bracket(lexer)) {
      lexer->result_symbol = OPTION_BRACKET;
      return true;
    }
    return false;
  }

  if (lexer->lookahead == '<' && valid_symbols[GENERIC_OPEN]) {
    if (scan_generic_open(lexer)) {
      lexer->result_symbol = GENERIC_OPEN;
      return true;
    }
    return false;
  }

  return false;
}
