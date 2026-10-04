use super::token::{Token, TokenKind};

pub fn is_ident_tok(tok: &Token) -> bool {
    tok.kind == TokenKind::Identifier
}

pub fn is_kw_tok(tok: &Token, s: &str) -> bool {
    tok.kind == TokenKind::Keyword && tok.lexeme == s
}

pub fn is_op_tok(tok: &Token, s: &str) -> bool {
    tok.kind == TokenKind::Operator && tok.lexeme == s
}

pub fn is_punc_tok(tok: &Token, s: &str) -> bool {
    tok.kind == TokenKind::Punctuator && tok.lexeme == s
}

pub fn lexeme_text(tok: &Token) -> &str {
    &tok.lexeme
}

pub fn is_keyword(s: &str) -> bool {
    uv_core::keywords::is_keyword(s)
}

pub fn is_fixed_identifier(s: &str) -> bool {
    uv_core::keywords::is_fixed_identifier(s)
}

pub fn is_fixed_ident_tok(tok: &Token, s: &str) -> bool {
    is_ident_tok(tok) && tok.lexeme == s && is_fixed_identifier(s)
}

pub fn is_ctx_keyword(s: &str) -> bool {
    matches!(s, "in" | "key" | "wait" | "new")
}

pub fn ctx(tok: &Token, s: &str) -> bool {
    is_ident_tok(tok) && tok.lexeme == s && is_ctx_keyword(s)
}

pub fn union_prop_tok(tok: &Token) -> bool {
    is_op_tok(tok, "?")
}

pub fn type_where_tok(tok: &Token) -> bool {
    tok.kind == TokenKind::Keyword && tok.lexeme == "where"
}

pub fn opaque_type_tok(tok: &Token) -> bool {
    is_ident_tok(tok) && tok.lexeme == "opaque"
}
