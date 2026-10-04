//! File-level entry point: tokenize, parse items, attach doc comments.

use uv_core::diagnostics::{emit, has_error, DiagnosticStream};
use uv_core::process_config::is_debug_enabled;
use uv_core::source_text::SourceFile;
use uv_core::span::Span;
use uv_core::spec_rule;

use super::expr::skip_newlines;
use super::item::parse_item;
use super::state::Parser;
use crate::ast::*;
use crate::lexer::{filter_newlines, tokenize_with_diagnostics, unsafe_spans, DocComment, DocKind};

#[derive(Default)]
pub struct ParseFileResult {
    pub file: Option<ASTFile>,
    pub unsafe_spans: Vec<Span>,
    pub diags: DiagnosticStream,
}

/// Comptime procedures and derive targets do not carry line docs.
fn set_item_doc(item: &mut ASTItem, docs: DocList) {
    match item {
        ASTItem::UsingDecl(d) => d.doc = docs,
        ASTItem::ImportDecl(d) => d.doc = docs,
        ASTItem::ExternBlock(d) => d.doc = docs,
        ASTItem::StaticDecl(d) => d.doc = docs,
        ASTItem::ProcedureDecl(d) => d.doc = docs,
        ASTItem::RecordDecl(d) => d.doc = docs,
        ASTItem::EnumDecl(d) => d.doc = docs,
        ASTItem::ModalDecl(d) => d.doc = docs,
        ASTItem::ClassDecl(d) => d.doc = docs,
        ASTItem::TypeAliasDecl(d) => d.doc = docs,
        ASTItem::ErrorItem(d) => d.doc = docs,
        ASTItem::ComptimeProcedureDecl(_) | ASTItem::DeriveTargetDecl(_) => {}
    }
}

fn module_docs(docs: &[DocComment]) -> Vec<DocComment> {
    docs.iter()
        .filter(|doc| doc.kind == DocKind::ModuleDoc)
        .inspect(|_| spec_rule!("Attach-Doc-Module"))
        .cloned()
        .collect()
}

/// Attaches each line doc to the first item that starts at or after the doc's end.
fn attach_line_docs(items: &mut [ASTItem], docs: &[DocComment]) {
    if items.is_empty() {
        return;
    }
    let mut item_docs: Vec<DocList> = vec![Vec::new(); items.len()];
    let mut item_index = 0usize;
    for doc in docs.iter().filter(|doc| doc.kind == DocKind::LineDoc) {
        while item_index < items.len()
            && item_span(&items[item_index]).start_offset < doc.span.end_offset
        {
            item_index += 1;
        }
        if item_index >= items.len() {
            continue;
        }
        spec_rule!("Attach-Doc-Line");
        item_docs[item_index].push(doc.clone());
    }
    for (item, docs) in items.iter_mut().zip(item_docs) {
        if !docs.is_empty() {
            set_item_doc(item, docs);
        }
    }
}

pub fn parse_items(parser: Parser) -> (Parser, Vec<ASTItem>, Vec<DocComment>) {
    spec_rule!("DocSeq");
    let module_doc = module_docs(&parser.docs);
    let mut items = Vec::new();
    let mut cur = parser;
    loop {
        skip_newlines(&mut cur);
        if cur.at_eof() {
            spec_rule!("ParseItems-Empty");
            return (cur, items, module_doc);
        }
        spec_rule!("ParseItems-Cons");
        let (next, item) = parse_item(cur);
        items.push(item);
        cur = next;
    }
}

fn first_top_level_error_item_span(items: &[ASTItem]) -> Option<Span> {
    items.iter().find_map(|item| match item {
        ASTItem::ErrorItem(err) => Some(err.span.clone()),
        _ => None,
    })
}

pub fn parse_file(source: &SourceFile) -> ParseFileResult {
    let mut result = ParseFileResult::default();
    let debug_phases = is_debug_enabled("phases");
    if debug_phases {
        eprintln!("[uv] parsefile: tokenize {}", source.path);
    }
    let tok = tokenize_with_diagnostics(source);
    result.diags = tok.diags;
    let Some(output) = tok.output else {
        return result;
    };
    if debug_phases {
        eprintln!("[uv] parsefile: filter-newlines {}", source.path);
    }
    let filtered = filter_newlines(&output.tokens);
    spec_rule!("DocSeq");
    if debug_phases {
        eprintln!("[uv] parsefile: parse-items {}", source.path);
    }
    let unsafe_block_spans = unsafe_spans(&filtered);
    let parser = Parser::new(&filtered, &output.docs, source);
    let (parser, mut items, module_doc) = parse_items(parser);
    if debug_phases {
        eprintln!("[uv] parsefile: attach-docs {}", source.path);
    }
    spec_rule!("ItemSeq(Items)");
    attach_line_docs(&mut items, &output.docs);
    spec_rule!("ParseFile-Ok");
    for diag in parser.diags {
        emit(&mut result.diags, diag);
    }
    if !has_error(&result.diags) {
        if let Some(span) = first_top_level_error_item_span(&items) {
            if let Some(diag) = uv_core::diagnostic_messages::make_diagnostic_by_id("E-SRC-0520", Some(span)) {
                emit(&mut result.diags, diag);
            }
        }
    }
    result.file = Some(ASTFile { path: vec![source.path.to_string()], items, module_doc });
    result.unsafe_spans = unsafe_block_spans;
    spec_rule!("Phase1-File");
    result
}

pub fn parse_file_ok(result: &ParseFileResult) -> bool {
    match &result.file {
        Some(file) if !has_error(&result.diags) => {
            first_top_level_error_item_span(&file.items).is_none()
        }
        _ => false,
    }
}
