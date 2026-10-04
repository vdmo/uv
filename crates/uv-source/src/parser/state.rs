//! Parser state: a cheap-to-copy cursor over the filtered token stream.

use std::rc::Rc;

use uv_core::diagnostic_messages::make_diagnostic_by_id;
use uv_core::diagnostics::{emit, DiagnosticStream};
use uv_core::source_text::SourceFile;
use uv_core::span::Span;

use crate::lexer::token::make_eof_token;
use crate::lexer::{DocComment, Token, TokenKind};

#[derive(Clone)]
pub struct Parser {
    pub tokens: Rc<Vec<Rc<Token>>>,
    /// Underlying indices of `>>` tokens that are read as two `>` tokens.
    pub split_shift_right_indices: Option<Rc<Vec<usize>>>,
    eof: Rc<Token>,
    pub index: usize,
    pub docs: Rc<Vec<DocComment>>,
    pub doc_index: usize,
    pub depth: usize,
    pub quote_mode: bool,
    pub stop_before_parallel_options: bool,
    pub stop_before_contract_post_separator: bool,
    pub diags: DiagnosticStream,
}

/// A parser together with the element it produced.
pub type Parsed<T> = (Parser, T);

fn point_span_at_end(token: &Token) -> Span {
    let mut span = token.span.clone();
    span.start_offset = token.span.end_offset;
    span.start_line = token.span.end_line;
    span.start_col = token.span.end_col;
    span
}

fn split_shift_right_token(original: &Token, is_right: bool) -> Token {
    let shift = usize::from(is_right);
    let span = &original.span;
    Token {
        kind: TokenKind::Operator,
        lexeme: ">".to_string(),
        span: Span {
            file: span.file.clone(),
            start_offset: span.start_offset + shift,
            end_offset: span.start_offset + shift + 1,
            start_line: span.start_line,
            end_line: span.start_line,
            start_col: span.start_col + shift,
            end_col: span.start_col + shift + 1,
        },
    }
}

impl Parser {
    pub fn new(tokens: &[Token], docs: &[DocComment], source: &SourceFile) -> Parser {
        Parser::from_parts(tokens, docs, Some(make_eof_token(source)))
    }

    /// A parser over tokens that have no backing source file.
    pub fn from_tokens(tokens: &[Token]) -> Parser {
        Parser::from_parts(tokens, &[], None)
    }

    fn from_parts(tokens: &[Token], docs: &[DocComment], eof: Option<Token>) -> Parser {
        let eof = eof.unwrap_or_else(|| Token {
            kind: TokenKind::Eof,
            lexeme: String::new(),
            span: tokens.last().map(point_span_at_end).unwrap_or_default(),
        });
        Parser {
            tokens: Rc::new(tokens.iter().cloned().map(Rc::new).collect()),
            split_shift_right_indices: None,
            eof: Rc::new(eof),
            index: 0,
            docs: Rc::new(docs.to_vec()),
            doc_index: 0,
            depth: 0,
            quote_mode: false,
            stop_before_parallel_options: false,
            stop_before_contract_post_separator: false,
            diags: Vec::new(),
        }
    }

    pub fn virtual_token_count(&self) -> usize {
        self.tokens.len() + self.split_shift_right_indices.as_ref().map_or(0, |s| s.len())
    }

    pub fn at_eof(&self) -> bool {
        self.index >= self.virtual_token_count()
    }

    fn token_at_virtual_index(&self, virtual_index: usize) -> Option<Rc<Token>> {
        let mut split_count_before = 0usize;
        if let Some(split_indices) = &self.split_shift_right_indices {
            // First split whose two virtual slots end at or after `virtual_index`.
            let first = split_indices
                .iter()
                .enumerate()
                .position(|(nth, &split_index)| split_index + nth + 1 >= virtual_index)
                .unwrap_or(split_indices.len());
            if first < split_indices.len() {
                let split_index = split_indices[first];
                let split_virtual_index = split_index + first;
                if virtual_index == split_virtual_index || virtual_index == split_virtual_index + 1
                {
                    let original = self.tokens.get(split_index)?;
                    return Some(Rc::new(split_shift_right_token(
                        original,
                        virtual_index == split_virtual_index + 1,
                    )));
                }
                split_count_before =
                    if virtual_index < split_virtual_index { first } else { first + 1 };
            } else {
                split_count_before = split_indices.len();
            }
        }
        self.tokens.get(virtual_index - split_count_before).cloned()
    }

    /// The current token, or the end-of-file token.
    pub fn tok(&self) -> Rc<Token> {
        if self.at_eof() {
            return self.eof.clone();
        }
        self.token_at_virtual_index(self.index).unwrap_or_else(|| self.eof.clone())
    }

    pub fn tok_span(&self) -> Span {
        self.tok().span.clone()
    }

    pub fn advance(&mut self) {
        if !self.at_eof() {
            self.index += 1;
        }
    }

    pub fn advance_or_eof(&self) -> Parser {
        let mut next = self.clone();
        next.advance();
        next
    }

    /// The same cursor without accumulated diagnostics (for speculative parsing).
    pub fn clone_without_diags(&self) -> Parser {
        let diags = Vec::new();
        Parser { diags, ..self.shallow() }
    }

    fn shallow(&self) -> Parser {
        Parser {
            tokens: self.tokens.clone(),
            split_shift_right_indices: self.split_shift_right_indices.clone(),
            eof: self.eof.clone(),
            index: self.index,
            docs: self.docs.clone(),
            doc_index: self.doc_index,
            depth: self.depth,
            quote_mode: self.quote_mode,
            stop_before_parallel_options: self.stop_before_parallel_options,
            stop_before_contract_post_separator: self.stop_before_contract_post_separator,
            diags: Vec::new(),
        }
    }

    pub fn p_state_ok(&self) -> bool {
        self.index <= self.virtual_token_count()
    }

    pub fn emit(&mut self, code: &str, span: Span) {
        if let Some(diag) = make_diagnostic_by_id(code, Some(span)) {
            emit(&mut self.diags, diag);
        }
    }

    // Token predicates on the current token.

    pub fn is_op(&self, op: &str) -> bool {
        let tok = self.tok();
        tok.kind == TokenKind::Operator && tok.lexeme == op
    }

    pub fn is_punct(&self, punct: &str) -> bool {
        let tok = self.tok();
        tok.kind == TokenKind::Punctuator && tok.lexeme == punct
    }

    pub fn is_kw(&self, keyword: &str) -> bool {
        let tok = self.tok();
        tok.kind == TokenKind::Keyword && tok.lexeme == keyword
    }

    pub fn is_ident(&self) -> bool {
        self.tok().kind == TokenKind::Identifier
    }

    pub fn is_newline(&self) -> bool {
        self.tok().kind == TokenKind::Newline
    }
}

/// `src`'s cursor carrying the diagnostics of `base` followed by those of `diag`.
pub fn merge_diag(base: &Parser, diag: &Parser, src: &Parser) -> Parser {
    let mut out = src.clone_without_diags();
    for d in base.diags.iter().chain(diag.diags.iter()) {
        emit(&mut out.diags, d.clone());
    }
    out
}

pub fn span_from(start: &Span, end: &Span) -> Span {
    let mut span = start.clone();
    span.end_offset = end.end_offset;
    span.end_line = end.end_line;
    span.end_col = end.end_col;
    span
}

/// Span from the token at `start` to the last token before `end`.
pub fn span_between(start: &Parser, end: &Parser) -> Span {
    let start_tok = start.tok();
    if end.index > start.index {
        let mut last = end.clone_without_diags();
        last.index = end.index - 1;
        return span_from(&start_tok.span, &last.tok().span);
    }
    start_tok.span.clone()
}
