use uv_core::source_text::SourceFile;
use uv_core::span::{span_of, Span};
use uv_core::spec_rule;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TokenKind {
    Identifier,
    Keyword,
    IntLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,
    BoolLiteral,
    NullLiteral,
    Operator,
    Punctuator,
    Newline,
    Eof,
    #[default]
    Unknown,
}

impl TokenKind {
    pub fn name(self) -> &'static str {
        match self {
            TokenKind::Identifier => "Identifier",
            TokenKind::Keyword => "Keyword",
            TokenKind::IntLiteral => "IntLiteral",
            TokenKind::FloatLiteral => "FloatLiteral",
            TokenKind::StringLiteral => "StringLiteral",
            TokenKind::CharLiteral => "CharLiteral",
            TokenKind::BoolLiteral => "BoolLiteral",
            TokenKind::NullLiteral => "NullLiteral",
            TokenKind::Operator => "Operator",
            TokenKind::Punctuator => "Punctuator",
            TokenKind::Newline => "Newline",
            TokenKind::Eof => "Eof",
            TokenKind::Unknown => "Unknown",
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RawToken {
    pub kind: TokenKind,
    pub lexeme: String,
    pub start_offset: usize,
    pub end_offset: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DocKind {
    #[default]
    LineDoc,
    ModuleDoc,
}

impl DocKind {
    pub fn name(self) -> &'static str {
        match self {
            DocKind::LineDoc => "LineDoc",
            DocKind::ModuleDoc => "ModuleDoc",
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DocComment {
    pub kind: DocKind,
    pub text: String,
    pub span: Span,
}

pub fn no_unknown_ok(tokens: &[Token]) -> bool {
    spec_rule!("No-Unknown-Ok");
    tokens.iter().all(|tok| tok.kind != TokenKind::Unknown)
}

pub fn attach_span(source: &SourceFile, raw: RawToken) -> Token {
    spec_rule!("Attach-Token-Ok");
    Token {
        kind: raw.kind,
        span: span_of(source, raw.start_offset, raw.end_offset),
        lexeme: raw.lexeme,
    }
}

pub fn attach_spans(source: &SourceFile, raws: Vec<RawToken>) -> Vec<Token> {
    spec_rule!("Attach-Tokens-Ok");
    raws.into_iter().map(|raw| attach_span(source, raw)).collect()
}

/// Scalar index range of a token, when its span lies on scalar boundaries of `source`.
pub fn token_range(source: &SourceFile, token: &Token) -> Option<(usize, usize)> {
    let offsets = &source.offsets;
    let i = offsets.binary_search(&token.span.start_offset).ok()?;
    let j = offsets.binary_search(&token.span.end_offset).ok()?;
    let expected = span_of(source, offsets[i], offsets[j]);
    (token.span == expected).then_some((i, j))
}

pub fn make_eof_token(source: &SourceFile) -> Token {
    Token {
        kind: TokenKind::Eof,
        lexeme: String::new(),
        span: span_of(source, source.byte_len, source.byte_len),
    }
}
