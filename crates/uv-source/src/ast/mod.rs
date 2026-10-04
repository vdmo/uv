//! Abstract syntax tree. Node types are generated from the reference headers.

pub mod dump;
#[allow(clippy::derivable_impls)]
mod generated;

pub use generated::*;

pub type Identifier = String;
pub type Path = Vec<Identifier>;
pub type ExprPtr = Option<std::sync::Arc<Expr>>;
pub type TypePtr = Option<std::sync::Arc<Type>>;
pub type PatternPtr = Option<std::sync::Arc<Pattern>>;
pub type BlockPtr = Option<std::sync::Arc<Block>>;
pub type AttributeList = Vec<AttributeItem>;
pub type AttrOpt = Option<AttributeList>;
pub type DocList = Vec<crate::lexer::DocComment>;

/// Attaches attributes to an expression, merging with attributes it already carries.
pub fn attach_expr_attrs(expr: &ExprPtr, attrs: AttributeList, span: &uv_core::span::Span) -> ExprPtr {
    let node = match expr {
        Some(node) if !attrs.is_empty() => node,
        _ => return expr.clone(),
    };
    let make = |node: ExprNode| Some(std::sync::Arc::new(Expr { span: span.clone(), node }));
    match &node.node {
        ExprNode::ComptimeExpr(comptime) => {
            let mut merged = comptime.clone();
            let mut combined = attrs;
            combined.extend(comptime.attrs_opt.clone().unwrap_or_default());
            merged.attrs_opt = Some(combined);
            make(ExprNode::ComptimeExpr(merged))
        }
        ExprNode::AttributedExpr(attributed) => {
            let mut combined = attrs;
            combined.extend(attributed.attrs.iter().cloned());
            let inner_is_comptime = matches!(
                attributed.expr.as_deref(),
                Some(Expr { node: ExprNode::ComptimeExpr(_), .. })
            );
            if inner_is_comptime {
                return attach_expr_attrs(&attributed.expr, combined, span);
            }
            make(ExprNode::AttributedExpr(AttributedExpr {
                attrs: combined,
                expr: attributed.expr.clone(),
            }))
        }
        _ => make(ExprNode::AttributedExpr(AttributedExpr { attrs, expr: expr.clone() })),
    }
}

pub fn item_span(item: &ASTItem) -> &uv_core::span::Span {
    match item {
        ASTItem::UsingDecl(d) => &d.span,
        ASTItem::ImportDecl(d) => &d.span,
        ASTItem::ExternBlock(d) => &d.span,
        ASTItem::StaticDecl(d) => &d.span,
        ASTItem::ProcedureDecl(d) => &d.span,
        ASTItem::ComptimeProcedureDecl(d) => &d.span,
        ASTItem::RecordDecl(d) => &d.span,
        ASTItem::EnumDecl(d) => &d.span,
        ASTItem::ModalDecl(d) => &d.span,
        ASTItem::ClassDecl(d) => &d.span,
        ASTItem::TypeAliasDecl(d) => &d.span,
        ASTItem::DeriveTargetDecl(d) => &d.span,
        ASTItem::ErrorItem(d) => &d.span,
    }
}

pub fn item_kind(item: &ASTItem) -> &'static str {
    match item {
        ASTItem::UsingDecl(_) => "UsingDecl",
        ASTItem::ImportDecl(_) => "ImportDecl",
        ASTItem::ExternBlock(_) => "ExternBlock",
        ASTItem::StaticDecl(_) => "StaticDecl",
        ASTItem::ProcedureDecl(_) => "ProcedureDecl",
        ASTItem::ComptimeProcedureDecl(_) => "ComptimeProcedureDecl",
        ASTItem::RecordDecl(_) => "RecordDecl",
        ASTItem::EnumDecl(_) => "EnumDecl",
        ASTItem::ModalDecl(_) => "ModalDecl",
        ASTItem::ClassDecl(_) => "ClassDecl",
        ASTItem::TypeAliasDecl(_) => "TypeAliasDecl",
        ASTItem::DeriveTargetDecl(_) => "DeriveTargetDecl",
        ASTItem::ErrorItem(_) => "ErrorItem",
    }
}

fn visibility_keyword(vis: Visibility) -> &'static str {
    match vis {
        Visibility::Public => "public",
        Visibility::Internal => "internal",
        Visibility::Private => "private",
    }
}

/// One-line item summary printed by `--dump-ast`.
pub fn item_summary(item: &ASTItem, include_spans: bool) -> String {
    let mut out = item_kind(item).to_string();
    if include_spans {
        let span = item_span(item);
        out.push_str(&format!(
            " [{}:{}-{}:{}]",
            span.start_line, span.start_col, span.end_line, span.end_col
        ));
    }
    let named = |vis: Visibility, keyword: &str, name: &str| {
        format!(" {} {keyword} {name}", visibility_keyword(vis))
    };
    out.push_str(&match item {
        ASTItem::ProcedureDecl(d) => named(d.vis, "procedure", &d.name),
        ASTItem::ComptimeProcedureDecl(d) => named(d.vis, "comptime procedure", &d.name),
        ASTItem::RecordDecl(d) => named(d.vis, "record", &d.name),
        ASTItem::EnumDecl(d) => named(d.vis, "enum", &d.name),
        ASTItem::ModalDecl(d) => named(d.vis, "modal", &d.name),
        ASTItem::ClassDecl(d) => {
            named(d.vis, if d.modal { "modal class" } else { "class" }, &d.name)
        }
        ASTItem::TypeAliasDecl(d) => named(d.vis, "type", &d.name),
        ASTItem::DeriveTargetDecl(d) => format!(" derive target {}", d.name),
        ASTItem::StaticDecl(d) => format!(
            " {} {}",
            visibility_keyword(d.vis),
            match d.r#mut {
                Mutability::Let => "let",
                Mutability::Var => "var",
            }
        ),
        ASTItem::ErrorItem(_) => " <error>".to_string(),
        ASTItem::UsingDecl(_) | ASTItem::ImportDecl(_) | ASTItem::ExternBlock(_) => String::new(),
    });
    out
}
