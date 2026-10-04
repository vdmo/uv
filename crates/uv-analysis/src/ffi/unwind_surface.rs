//! FFI surface of a module: imported extern procedures, exported procedures and how each
//! treats unwinding across the boundary.

use uv_core::span::Span;
use uv_core::spec_rule;
use uv_source::ast::{
    ASTFile, ASTItem, ASTModule, AttributeArgValue, AttributeItem, ExternAbi, ExternItem,
    ExternProcDecl, ProcedureDecl,
};
use uv_source::attributes::{attrs, has_attribute};
use uv_source::lexer::TokenKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UnwindMode {
    /// Abort when an unwind reaches the boundary.
    #[default]
    Abort,
    /// Catch and convert unwinds at the boundary.
    Catch,
}

pub fn unwind_mode_to_string(mode: UnwindMode) -> &'static str {
    match mode {
        UnwindMode::Abort => "abort",
        UnwindMode::Catch => "catch",
    }
}

pub fn parse_unwind_mode(text: &str) -> Option<UnwindMode> {
    match text {
        "abort" => Some(UnwindMode::Abort),
        "catch" => Some(UnwindMode::Catch),
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub struct FfiImportInfo {
    pub name: String,
    /// ABI string, such as `C` or `C-unwind`.
    pub abi: String,
    pub unwind_mode: UnwindMode,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ExportProcInfo {
    pub name: String,
    pub unwind_mode: UnwindMode,
    pub span: Span,
}

#[derive(Debug, Clone, Default)]
pub struct FfiSurfaceInfo {
    /// Extern procedure declarations.
    pub imports: Vec<FfiImportInfo>,
    /// Procedures carrying `#export`.
    pub exports: Vec<ExportProcInfo>,
}

fn strip_double_quotes(text: &str) -> &str {
    text.strip_prefix('"')
        .and_then(|rest| rest.strip_suffix('"'))
        .filter(|_| text.len() >= 2)
        .unwrap_or(text)
}

fn abi_string(abi_opt: &Option<ExternAbi>) -> String {
    match abi_opt {
        None => "C".to_string(),
        Some(ExternAbi::ExternAbiString(abi)) => strip_double_quotes(&abi.literal.lexeme).to_string(),
        Some(ExternAbi::ExternAbiIdent(abi)) => abi.name.clone(),
    }
}

fn unwind_attribute(attr_list: &[AttributeItem]) -> Option<&AttributeItem> {
    attr_list.iter().find(|attr| attr.name.full_name == attrs::UNWIND)
}

/// The mode named by `#unwind("...")`, when the attribute is present and well formed.
fn extract_unwind_mode(attr_list: &[AttributeItem]) -> Option<UnwindMode> {
    let [arg] = unwind_attribute(attr_list)?.args.as_slice() else {
        return None;
    };
    match (&arg.key, &arg.value) {
        (None, AttributeArgValue::Token(token)) if token.kind == TokenKind::StringLiteral => {
            parse_unwind_mode(strip_double_quotes(&token.lexeme))
        }
        _ => None,
    }
}

fn unwind_mode_of(attr_list: &[AttributeItem]) -> UnwindMode {
    match extract_unwind_mode(attr_list) {
        Some(mode) => {
            spec_rule!("UnwindMode-Explicit");
            mode
        }
        None => {
            spec_rule!("UnwindMode-Default");
            UnwindMode::Abort
        }
    }
}

pub fn has_export_attribute(proc: &ProcedureDecl) -> bool {
    has_attribute(&proc.attrs, attrs::EXPORT)
}

/// `None` when the attribute list has no `#unwind`; otherwise the mode it names, if valid.
pub fn unwind_attribute_mode(attr_list: &[AttributeItem]) -> Option<Option<UnwindMode>> {
    unwind_attribute(attr_list).map(|_| extract_unwind_mode(attr_list))
}

pub fn procedure_unwind_mode(proc: &ProcedureDecl) -> UnwindMode {
    unwind_mode_of(&proc.attrs)
}

pub fn extern_procedure_unwind_mode(proc: &ExternProcDecl) -> UnwindMode {
    unwind_mode_of(&proc.attrs)
}

fn process_item(item: &ASTItem, surface: &mut FfiSurfaceInfo) {
    match item {
        ASTItem::ProcedureDecl(decl) if has_export_attribute(decl) => {
            spec_rule!("FFIBoundary");
            surface.exports.push(ExportProcInfo {
                name: decl.name.clone(),
                unwind_mode: procedure_unwind_mode(decl),
                span: decl.span.clone(),
            });
        }
        ASTItem::ExternBlock(block) => {
            for ExternItem::ExternProcDecl(extern_decl) in &block.items {
                spec_rule!("FFIBoundary");
                surface.imports.push(FfiImportInfo {
                    name: extern_decl.name.clone(),
                    abi: abi_string(&block.abi_opt),
                    unwind_mode: extern_procedure_unwind_mode(extern_decl),
                    span: extern_decl.span.clone(),
                });
            }
        }
        _ => {}
    }
}

fn collect(items: &[ASTItem]) -> FfiSurfaceInfo {
    let mut surface = FfiSurfaceInfo::default();
    for item in items {
        process_item(item, &mut surface);
    }
    surface
}

pub fn collect_ffi_surface(module: &ASTModule) -> FfiSurfaceInfo {
    collect(&module.items)
}

pub fn collect_ffi_surface_of_file(file: &ASTFile) -> FfiSurfaceInfo {
    collect(&file.items)
}
