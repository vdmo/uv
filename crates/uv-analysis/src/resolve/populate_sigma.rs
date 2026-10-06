//! The table of nominal declarations: the built-in ones, then every module's own.

use uv_source::ast::*;

use super::resolver::full_path;
use super::scopes::path_key_of;
use crate::caps::builtin_decls::*;
use crate::context::*;

fn foundational_classes() -> Vec<ClassDecl> {
    let method = |name: &str, perm: ReceiverPerm, params: Vec<Param>, ret: TypePtr| {
        class_method(name, None, perm, params, ret)
    };
    let self_type = || path(&["Self"]);
    let discrete_step = |name: &str| method(name, ReceiverPerm::Const, vec![], union(vec![self_type(), prim("()")]));
    let associated_item =
        ClassItem::AssociatedTypeDecl(AssociatedTypeDecl { vis: Visibility::Public, name: "Item".to_string(), ..Default::default() });
    vec![
        class("Drop", None, vec![method("drop", ReceiverPerm::Unique, vec![], prim("()"))]),
        class("Bitcopy", None, vec![]),
        class("Clone", None, vec![method("clone", ReceiverPerm::Const, vec![], self_type())]),
        class(
            "Eq",
            None,
            vec![method(
                "eq",
                ReceiverPerm::Const,
                vec![param("other", perm(TypePerm::Const, self_type()))],
                prim("bool"),
            )],
        ),
        class(
            "Hasher",
            None,
            vec![
                method("write", ReceiverPerm::Unique, vec![param("data", bytes(BytesState::View))], prim("()")),
                method("finish", ReceiverPerm::Const, vec![], prim("u64")),
            ],
        ),
        class(
            "Hash",
            None,
            vec![method(
                "hash",
                ReceiverPerm::Const,
                vec![param("hasher", perm(TypePerm::Unique, dynamic(&["Hasher"])))],
                prim("()"),
            )],
        ),
        class("FfiSafe", None, vec![]),
        class("GpuSafe", None, vec![]),
        class(
            "Iterator",
            None,
            vec![
                associated_item,
                method("next", ReceiverPerm::Unique, vec![], union(vec![path(&["Self", "Item"]), prim("()")])),
            ],
        ),
        class("Discrete", None, vec![discrete_step("successor"), discrete_step("predecessor")]),
    ]
}

fn capability_classes() -> Vec<ClassDecl> {
    vec![
        class("IO", None, vec![]),
        class("Network", None, vec![]),
        class("HeapAllocator", None, vec![]),
        build_execution_domain_class_decl(),
        build_system_class_decl(),
        build_reactor_class_decl(),
        class("Time", None, vec![]),
        class("MonotonicTime", None, vec![]),
        class("WallTime", None, vec![]),
        build_async_class_decl(),
    ]
}

fn builtin_types() -> Vec<TypeDecl> {
    use TypeDecl::*;
    vec![
        Modal(build_region_modal_decl()),
        Record(build_region_options_record_decl()),
        Record(build_context_record_decl()),
        Record(build_test_authority_record_decl()),
        Record(build_panic_record_decl()),
        TypeAlias(build_cpu_set_alias_decl()),
        Enum(build_priority_enum_decl()),
        Record(build_duration_record_decl()),
        Record(build_monotonic_instant_record_decl()),
        Record(build_utc_instant_record_decl()),
        Enum(build_allocation_error_enum_decl()),
        Enum(build_io_error_enum_decl()),
        Enum(build_time_error_enum_decl()),
        Enum(build_file_kind_enum_decl()),
        Record(build_dir_entry_record_decl()),
        Enum(build_type_category_enum_decl()),
        Record(build_field_info_record_decl()),
        Record(build_variant_info_record_decl()),
        Record(build_state_info_record_decl()),
        Record(build_source_span_record_decl()),
        Modal(build_file_modal_decl()),
        Modal(build_dir_iter_modal_decl()),
        Modal(build_spawned_modal_decl()),
        Modal(build_cancel_token_modal_decl()),
        Modal(build_async_modal_decl()),
        Enum(build_outcome_enum_decl()),
        Modal(build_tracked_modal_decl()),
        TypeAlias(build_future_alias_decl()),
        TypeAlias(build_sequence_alias_decl()),
        TypeAlias(build_stream_alias_decl()),
        TypeAlias(build_pipe_alias_decl()),
        TypeAlias(build_exchange_alias_decl()),
    ]
}

fn type_decl_name(decl: &TypeDecl) -> &str {
    match decl {
        TypeDecl::Record(decl) => &decl.name,
        TypeDecl::Enum(decl) => &decl.name,
        TypeDecl::Modal(decl) => &decl.name,
        TypeDecl::TypeAlias(decl) => &decl.name,
    }
}

/// Rebuilds the declaration tables. Built-ins are keyed by bare name, module
/// declarations by module path and name.
pub fn populate_sigma(ctx: &mut ScopeContext<'_>) {
    let sigma = std::sync::Arc::make_mut(&mut ctx.sigma);
    sigma.types.clear();
    sigma.classes.clear();
    for decl in foundational_classes().into_iter().chain(capability_classes()) {
        sigma.classes.insert(path_key_of(&[&decl.name]), decl);
    }
    for decl in builtin_types() {
        sigma.types.insert(path_key_of(&[type_decl_name(&decl)]), decl);
    }
    for module in &sigma.mods {
        for item in &module.items {
            let (name, decl) = match item {
                ASTItem::RecordDecl(node) => (&node.name, TypeDecl::Record(node.clone())),
                ASTItem::EnumDecl(node) => (&node.name, TypeDecl::Enum(node.clone())),
                ASTItem::ModalDecl(node) => (&node.name, TypeDecl::Modal(node.clone())),
                ASTItem::TypeAliasDecl(node) => (&node.name, TypeDecl::TypeAlias(node.clone())),
                ASTItem::ClassDecl(node) => {
                    sigma.classes.insert(path_key_of(&full_path(&module.path, &node.name)), node.clone());
                    continue;
                }
                _ => continue,
            };
            sigma.types.insert(path_key_of(&full_path(&module.path, name)), decl);
        }
    }
}
