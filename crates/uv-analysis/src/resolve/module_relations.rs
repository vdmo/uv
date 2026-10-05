//! Which module declares a nominal type or class, by bare name.

use uv_source::ast::*;

use super::scopes::id_eq;
use crate::context::Sigma;

pub fn impl_module(sigma: &Sigma, type_name: &str) -> Option<Vec<String>> {
    sigma.mods.iter().find_map(|module| {
        module
            .items
            .iter()
            .any(|item| match item {
                ASTItem::RecordDecl(node) => id_eq(&node.name, type_name),
                ASTItem::EnumDecl(node) => id_eq(&node.name, type_name),
                ASTItem::ModalDecl(node) => id_eq(&node.name, type_name),
                ASTItem::TypeAliasDecl(node) => id_eq(&node.name, type_name),
                _ => false,
            })
            .then(|| module.path.clone())
    })
}

pub fn class_module(sigma: &Sigma, class_name: &str) -> Option<Vec<String>> {
    sigma.mods.iter().find_map(|module| {
        module
            .items
            .iter()
            .any(|item| matches!(item, ASTItem::ClassDecl(cls) if id_eq(&cls.name, class_name)))
            .then(|| module.path.clone())
    })
}
