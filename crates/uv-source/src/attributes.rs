//! Registry of language-defined attributes and validation of attribute lists.

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use uv_core::span::Span;
use uv_core::spec_rule;
use uv_core::spec_trace::Conformance;
use uv_project::language_profile::active_language_profile;

use crate::ast::{AttributeArg, AttributeArgValue, AttributeItem};
use crate::lexer::{Token, TokenKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttributeTarget {
    Procedure,
    ExternBlock,
    Record,
    Enum,
    Modal,
    Field,
    Method,
    TypeAlias,
    Binding,
    Statement,
    Expression,
    KeyBlock,
}

pub struct AttributeArgSpec {
    pub name: &'static str,
    pub required: bool,
    pub accepts_expr: bool,
    pub default_value: Option<&'static str>,
}

pub struct AttributeSpec {
    pub name: &'static str,
    pub valid_targets: &'static [AttributeTarget],
    pub args: &'static [AttributeArgSpec],
}

pub mod attrs {
    pub const DYNAMIC: &str = "dynamic";
    pub const ORDER: &str = "order";
    pub const DEBUG_CONTRACT: &str = "debug_contract";
    pub const RELEASE_CONTRACT: &str = "release_contract";
    pub const LAYOUT: &str = "layout";
    pub const ALIGN: &str = "align";
    pub const PACKED: &str = "packed";
    pub const RELAXED: &str = "relaxed";
    pub const ACQUIRE: &str = "acquire";
    pub const RELEASE: &str = "release";
    pub const ACQ_REL: &str = "acqrel";
    pub const SEQ_CST: &str = "seqcst";
    pub const REFLECT: &str = "reflect";
    pub const DERIVE: &str = "derive";
    pub const EMIT: &str = "emit";
    pub const FILES: &str = "files";
    pub const TEST: &str = "test";
    pub const ALLOW: &str = "allow";
    pub const WARN: &str = "warn";
    pub const DENY: &str = "deny";
    pub const FORBID: &str = "forbid";
    pub const DEPRECATED: &str = "deprecated";
    pub const STALE_OK: &str = "stale_ok";
    pub const INLINE: &str = "inline";
    pub const COLD: &str = "cold";
    pub const HOT: &str = "hot";
    pub const MANGLE: &str = "mangle";
    pub const EXPORT: &str = "export";
    pub const HOST_EXPORT: &str = "host_export";
    pub const LIBRARY: &str = "library";
    pub const UNWIND: &str = "unwind";
    pub const WEAK: &str = "weak";
    pub const FFI_PASS_BY_VALUE: &str = "ffi_pass_by_value";
    pub const STATIC: &str = "static";
}

use AttributeTarget as T;

const fn required_arg(name: &'static str) -> AttributeArgSpec {
    AttributeArgSpec { name, required: true, accepts_expr: false, default_value: None }
}

const ORDERING_TARGETS: &[T] = &[T::Expression, T::KeyBlock];
const TYPE_TARGETS: &[T] = &[T::Record, T::Enum, T::Modal];
const EMIT_TARGETS: &[T] = &[T::Statement, T::Expression];
const PROCEDURE_ONLY: &[T] = &[T::Procedure];

const fn spec(name: &'static str, valid_targets: &'static [T]) -> AttributeSpec {
    AttributeSpec { name, valid_targets, args: &[] }
}

static REGISTRY: &[AttributeSpec] = &[
    spec(
        attrs::DYNAMIC,
        &[T::Procedure, T::Record, T::Enum, T::Modal, T::Expression, T::KeyBlock],
    ),
    AttributeSpec {
        name: attrs::LAYOUT,
        valid_targets: &[T::Record, T::Enum],
        args: &[required_arg("kind")],
    },
    spec(attrs::RELAXED, ORDERING_TARGETS),
    spec(attrs::ACQUIRE, ORDERING_TARGETS),
    spec(attrs::RELEASE, ORDERING_TARGETS),
    spec(attrs::ACQ_REL, ORDERING_TARGETS),
    spec(attrs::SEQ_CST, ORDERING_TARGETS),
    spec(attrs::REFLECT, TYPE_TARGETS),
    spec(attrs::DERIVE, TYPE_TARGETS),
    spec(attrs::EMIT, EMIT_TARGETS),
    spec(attrs::FILES, EMIT_TARGETS),
    spec(attrs::TEST, PROCEDURE_ONLY),
    spec(
        attrs::DEPRECATED,
        &[T::Procedure, T::Record, T::Enum, T::Modal, T::Field, T::Binding, T::TypeAlias],
    ),
    spec(attrs::STALE_OK, &[T::Binding]),
    spec(attrs::INLINE, PROCEDURE_ONLY),
    spec(attrs::COLD, PROCEDURE_ONLY),
    AttributeSpec { name: attrs::MANGLE, valid_targets: PROCEDURE_ONLY, args: &[required_arg("mode")] },
    spec(attrs::EXPORT, PROCEDURE_ONLY),
    spec(attrs::HOST_EXPORT, PROCEDURE_ONLY),
    AttributeSpec {
        name: attrs::LIBRARY,
        valid_targets: &[T::ExternBlock],
        args: &[
            required_arg("name"),
            AttributeArgSpec {
                name: "kind",
                required: false,
                accepts_expr: false,
                default_value: Some("dylib"),
            },
        ],
    },
    spec(attrs::UNWIND, PROCEDURE_ONLY),
    spec(attrs::FFI_PASS_BY_VALUE, &[T::Record, T::Enum]),
    spec(attrs::STATIC, PROCEDURE_ONLY),
];

pub fn lookup_attribute(name: &str) -> Option<&'static AttributeSpec> {
    static INIT: OnceLock<()> = OnceLock::new();
    INIT.get_or_init(|| spec_rule!("AttrRegistry-Init"));
    REGISTRY.iter().find(|spec| spec.name == name)
}

pub fn is_valid_for_target(name: &str, target: AttributeTarget) -> bool {
    lookup_attribute(name).is_some_and(|spec| spec.valid_targets.contains(&target))
}

#[derive(Debug, Clone)]
pub struct AttributeValidationResult {
    pub ok: bool,
    pub diag_id: Option<&'static str>,
    pub span: Option<Span>,
    pub message: String,
}

impl Default for AttributeValidationResult {
    fn default() -> Self {
        AttributeValidationResult { ok: true, diag_id: None, span: None, message: String::new() }
    }
}

type Rejected = Box<AttributeValidationResult>;

fn reject(attr: &AttributeItem, diag_id: &'static str, message: impl Into<String>) -> Rejected {
    Box::new(AttributeValidationResult {
        ok: false,
        diag_id: Some(diag_id),
        span: Some(attr.span.clone()),
        message: message.into(),
    })
}

fn reserved_attribute_prefix() -> String {
    format!("{}::", active_language_profile().runtime_root)
}

fn normalize_attr_literal(value: &str) -> &str {
    let bytes = value.as_bytes();
    if bytes.len() >= 2 {
        let (first, last) = (bytes[0], bytes[bytes.len() - 1]);
        if (first == b'"' && last == b'"') || (first == b'\'' && last == b'\'') {
            return &value[1..value.len() - 1];
        }
    }
    value
}

fn token_arg(arg: &AttributeArg) -> Option<&Token> {
    match &arg.value {
        AttributeArgValue::Token(token) => Some(token),
        AttributeArgValue::AttributeArgList(_) => None,
    }
}

fn nested_args(arg: &AttributeArg) -> Option<&[AttributeArg]> {
    match &arg.value {
        AttributeArgValue::AttributeArgList(nested) => Some(nested),
        AttributeArgValue::Token(_) => None,
    }
}

fn is_string_literal(token: &Token) -> bool {
    token.kind == TokenKind::StringLiteral
}

fn is_non_empty_string_literal(token: &Token) -> bool {
    is_string_literal(token) && !normalize_attr_literal(&token.lexeme).is_empty()
}

fn has_only_positional_token_arg(attr: &AttributeItem) -> bool {
    matches!(attr.args.as_slice(), [arg] if arg.key.is_none() && token_arg(arg).is_some())
}

fn validate_derive_args(attr: &AttributeItem) -> Result<(), Rejected> {
    const MESSAGE: &str = "#derive(... ) requires one or more identifier arguments";
    if attr.args.is_empty() {
        return Err(reject(attr, "E-MOD-2450", MESSAGE));
    }
    let mut seen_targets: Vec<&str> = Vec::with_capacity(attr.args.len());
    for arg in &attr.args {
        let token = match token_arg(arg) {
            Some(token) if arg.key.is_none() && token.kind == TokenKind::Identifier => token,
            _ => return Err(reject(attr, "E-MOD-2450", MESSAGE)),
        };
        if seen_targets.contains(&token.lexeme.as_str()) {
            return Err(reject(attr, "E-CTE-0312", "Duplicate derive target in one derive attribute"));
        }
        seen_targets.push(&token.lexeme);
    }
    Conformance::record_at(
        "requirement.22.DeriveAttributeParsingReference",
        Some(&attr.span),
        "source=ValidateDeriveAttributeArgs;attribute=derive;args=validated",
    );
    Ok(())
}

fn looks_like_coverage_reference(value: &str) -> bool {
    match value.rfind("@L") {
        Some(marker) if marker != 0 && marker + 2 < value.len() => {
            value[marker + 2..].bytes().all(|b| b.is_ascii_digit())
        }
        _ => false,
    }
}

fn is_decimal_text(text: &str) -> bool {
    !text.is_empty() && text.bytes().all(|b| b.is_ascii_digit())
}

/// `<number>,<id>,...,<line>` rows of the obligation ledger name the reference `<id>@L<line>`.
fn obligation_reference_from_csv_row(row: &str) -> Option<String> {
    let first = row.find(',')?;
    if !is_decimal_text(&row[..first]) {
        return None;
    }
    let second = first + 1 + row[first + 1..].find(',')?;
    let last = row.rfind(',')?;
    if second <= first + 1 || last <= second + 1 || last + 1 >= row.len() {
        return None;
    }
    let id = &row[first + 1..second];
    let line = row[last + 1..].trim_end_matches(['\r', '\n']);
    if id.is_empty() || !is_decimal_text(line) {
        return None;
    }
    Some(format!("{id}@L{line}"))
}

/// The ledger is looked up under `Docs/Internal` of the working directory and its
/// ancestors. The reference also consults a build-time default path and the compiler
/// support bundle; neither exists for this build yet.
fn find_obligation_ledger_path() -> Option<PathBuf> {
    let cwd = std::env::current_dir().ok()?;
    let mut current: Option<&Path> = Some(cwd.as_path());
    while let Some(dir) = current {
        let candidate = dir.join("Docs").join("Internal").join("UltravioletObligations.csv");
        if candidate.is_file() {
            return Some(candidate);
        }
        current = dir.parent();
    }
    None
}

fn coverage_reference_names_known_obligation_row(value: &str) -> bool {
    static KNOWN: OnceLock<HashSet<String>> = OnceLock::new();
    KNOWN
        .get_or_init(|| {
            find_obligation_ledger_path()
                .and_then(|path| std::fs::read(path).ok())
                .map(|bytes| {
                    String::from_utf8_lossy(&bytes)
                        .split('\n')
                        .filter_map(obligation_reference_from_csv_row)
                        .collect()
                })
                .unwrap_or_default()
        })
        .contains(value)
}

fn validate_test_args(attr: &AttributeItem) -> Result<(), Rejected> {
    spec_rule!("def.TestAttributeArgsOk");
    let mut saw_name = false;
    for arg in &attr.args {
        match arg.key.as_deref() {
            Some("name") => {
                if saw_name {
                    return Err(reject(attr, "E-TST-0102", "Duplicate #test name argument"));
                }
                if !token_arg(arg).is_some_and(is_non_empty_string_literal) {
                    return Err(reject(attr, "E-TST-0101", "Malformed #test argument"));
                }
                saw_name = true;
            }
            Some("covers") => {
                let malformed = || reject(attr, "E-TST-0103", "Malformed covers(...) argument");
                let token = match nested_args(arg) {
                    Some([inner]) if inner.key.is_none() => token_arg(inner),
                    _ => None,
                };
                let token = token.filter(|token| is_non_empty_string_literal(token)).ok_or_else(malformed)?;
                let reference = normalize_attr_literal(&token.lexeme);
                if !looks_like_coverage_reference(reference) {
                    return Err(malformed());
                }
                if !coverage_reference_names_known_obligation_row(reference) {
                    return Err(reject(attr, "E-TST-0107", "Unknown audit coverage reference"));
                }
            }
            _ => return Err(reject(attr, "E-TST-0101", "Malformed #test argument")),
        }
    }
    Ok(())
}

fn parse_u64_literal(token: &Token) -> Option<u64> {
    let text: String = token.lexeme.chars().filter(|c| *c != '_').collect();
    if !is_decimal_text(&text) {
        return None;
    }
    text.parse().ok()
}

fn is_int_layout_kind(value: &str) -> bool {
    matches!(value, "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64")
}

fn validate_layout_args(attr: &AttributeItem, target: AttributeTarget) -> Result<(), Rejected> {
    let malformed = |message: &str| {
        spec_rule!("req.LayoutAttributeConstraints");
        reject(attr, "E-MOD-2450", message)
    };
    let conflicting = || {
        spec_rule!("def.InvalidLayoutAttributeCombinations");
        spec_rule!("req.LayoutAttributeConstraints");
        reject(attr, "E-MOD-2455", "Conflicting layout arguments")
    };
    let (mut saw_c, mut saw_packed, mut saw_align, mut saw_discriminant) = (false, false, false, false);
    for arg in &attr.args {
        if let Some(key) = &arg.key {
            if key != "align" {
                return Err(malformed("Malformed layout argument"));
            }
            let inner = match nested_args(arg) {
                Some([inner]) if inner.key.is_none() => inner,
                _ => return Err(malformed("Malformed layout align argument")),
            };
            let alignment = token_arg(inner)
                .filter(|token| token.kind == TokenKind::IntLiteral)
                .and_then(parse_u64_literal);
            if !alignment.is_some_and(u64::is_power_of_two) {
                spec_rule!("req.LayoutAttributeConstraints");
                return Err(reject(attr, "E-MOD-2453", "Invalid layout align argument"));
            }
            if saw_align {
                return Err(conflicting());
            }
            saw_align = true;
            continue;
        }
        let Some(token) = token_arg(arg) else {
            return Err(malformed("Malformed layout argument"));
        };
        let value = normalize_attr_literal(&token.lexeme);
        let seen = if value == "C" {
            &mut saw_c
        } else if value == "packed" {
            &mut saw_packed
        } else if is_int_layout_kind(value) {
            &mut saw_discriminant
        } else {
            return Err(malformed("Malformed layout argument"));
        };
        if *seen {
            return Err(conflicting());
        }
        *seen = true;
    }
    if !saw_c && !saw_packed && !saw_align && !saw_discriminant {
        return Err(malformed("Malformed #layout syntax: missing required layout kind"));
    }
    if saw_packed && target != AttributeTarget::Record {
        spec_rule!("def.LayoutAttributeApplicability");
        spec_rule!("req.LayoutAttributeConstraints");
        return Err(reject(attr, "E-MOD-2454", "layout(packed) is valid only on records"));
    }
    if saw_discriminant && target != AttributeTarget::Enum {
        spec_rule!("def.LayoutAttributeApplicability");
        spec_rule!("req.LayoutAttributeConstraints");
        return Err(reject(attr, "E-MOD-2455", "Conflicting layout arguments"));
    }
    if (saw_packed && saw_align) || (saw_discriminant && (saw_c || saw_packed || saw_align)) {
        return Err(conflicting());
    }
    spec_rule!("def.ValidLayoutAttributeCombinations");
    if saw_discriminant {
        spec_rule!("req.LayoutExplicitEnumDiscriminant");
    }
    if saw_packed {
        spec_rule!("req.LayoutPackedRecordSemantics");
    }
    if saw_align {
        spec_rule!("req.LayoutAlignSemantics");
    }
    Ok(())
}

fn validate_mangle_args(attr: &AttributeItem) -> Result<(), Rejected> {
    let invalid = || reject(attr, "E-SYS-3341", "Invalid #mangle(mode) argument");
    let [arg] = attr.args.as_slice() else {
        return Err(invalid());
    };
    if arg.key.as_deref().is_some_and(|key| key != "mode") {
        return Err(invalid());
    }
    let token = token_arg(arg).ok_or_else(invalid)?;
    let mode = normalize_attr_literal(&token.lexeme);
    if mode.is_empty() {
        return Err(invalid());
    }
    let bare_none = mode == "none" && token.kind == TokenKind::Identifier;
    if bare_none || is_string_literal(token) {
        Ok(())
    } else {
        Err(invalid())
    }
}

fn validate_inline_args(attr: &AttributeItem) -> Result<(), Rejected> {
    let malformed = || reject(attr, "E-MOD-2450", "Malformed #inline syntax");
    match attr.args.as_slice() {
        [] => Ok(()),
        [arg] if arg.key.is_none() => {
            let token = token_arg(arg)
                .filter(|token| matches!(token.kind, TokenKind::Identifier | TokenKind::Keyword))
                .ok_or_else(malformed)?;
            match normalize_attr_literal(&token.lexeme) {
                "always" | "never" | "default" => Ok(()),
                _ => Err(malformed()),
            }
        }
        _ => Err(malformed()),
    }
}

fn validate_deprecated_args(attr: &AttributeItem) -> Result<(), Rejected> {
    match attr.args.as_slice() {
        [] => Ok(()),
        [arg] if arg.key.is_none() && token_arg(arg).is_some_and(is_string_literal) => Ok(()),
        _ => Err(reject(attr, "E-MOD-2450", "Malformed #deprecated syntax")),
    }
}

fn validate_export_args(attr: &AttributeItem) -> Result<(), Rejected> {
    match attr.args.as_slice() {
        [arg] if arg.key.is_none() && token_arg(arg).is_some_and(is_non_empty_string_literal) => Ok(()),
        _ => Err(reject(
            attr,
            "E-MOD-2450",
            if attr.name.full_name == attrs::EXPORT { "Malformed #export syntax" } else { "Malformed #host_export syntax" },
        )),
    }
}

fn validate_library_args(attr: &AttributeItem) -> Result<(), Rejected> {
    let malformed = || reject(attr, "E-MOD-2450", "Malformed #library syntax");
    let (mut saw_name, mut saw_kind) = (false, false);
    for (arg_index, arg) in attr.args.iter().enumerate() {
        let key = arg.key.as_deref().filter(|key| matches!(*key, "name" | "kind")).ok_or_else(malformed)?;
        let token = token_arg(arg).filter(|token| is_string_literal(token)).ok_or_else(malformed)?;
        let normalized = normalize_attr_literal(&token.lexeme);
        if key == "name" {
            if saw_name || normalized.is_empty() || arg_index != 0 {
                return Err(malformed());
            }
            saw_name = true;
        } else {
            if saw_kind || !saw_name || arg_index != 1 {
                return Err(malformed());
            }
            saw_kind = true;
            if !matches!(normalized, "dylib" | "static" | "framework" | "raw-dylib") {
                return Err(reject(attr, "E-SYS-3346", "Unknown or unsupported library kind"));
            }
        }
    }
    if !saw_name {
        return Err(reject(
            attr,
            "E-MOD-2450",
            "Malformed #library syntax: missing required `name` argument",
        ));
    }
    Ok(())
}

fn validate_required_args(attr: &AttributeItem, spec: &AttributeSpec) -> Result<(), Rejected> {
    for (spec_index, arg_spec) in spec.args.iter().enumerate() {
        if !arg_spec.required {
            continue;
        }
        let found = attr.args.iter().any(|arg| match arg.key.as_deref() {
            Some(key) => {
                key == arg_spec.name
                    || (attr.name.full_name == attrs::LAYOUT && arg_spec.name == "kind" && key == "align")
            }
            None => spec_index == 0,
        });
        if !found {
            return Err(reject(
                attr,
                "E-MOD-2450",
                format!("Malformed #{} syntax: missing required argument `{}`", attr.name.full_name, arg_spec.name),
            ));
        }
    }
    Ok(())
}

fn validate_attribute(attr: &AttributeItem, target: AttributeTarget) -> Result<(), Rejected> {
    let name = attr.name.full_name.as_str();
    let Some(spec) = lookup_attribute(name) else {
        let diag_id = if name.starts_with(&reserved_attribute_prefix()) {
            Conformance::record_at(
                "conformance.VendorAttributeLowering",
                Some(&attr.span),
                "source=ValidateAttributes;registered_vendor_attributes=0;lowering_rules=none;rejected_before_lowering=true",
            );
            "E-CNF-0402"
        } else {
            spec_rule!("AttrList-Unknown");
            "E-MOD-2451"
        };
        return Err(reject(attr, diag_id, format!("Unknown attribute: {name}")));
    };
    // Methods accept the procedure-level attributes that have the same meaning on them.
    let method_equivalent =
        matches!(name, attrs::INLINE | attrs::COLD | attrs::DEPRECATED | attrs::DYNAMIC);
    let effective_target = if target == AttributeTarget::Method && method_equivalent {
        AttributeTarget::Procedure
    } else {
        target
    };
    if !spec.valid_targets.contains(&effective_target) {
        let diag_id = match (name, effective_target) {
            (attrs::DYNAMIC, AttributeTarget::TypeAlias) => "E-CON-0411",
            (attrs::DYNAMIC, AttributeTarget::Field) => "E-CON-0412",
            (attrs::LIBRARY, _) => "E-SYS-3345",
            (attrs::TEST, _) => {
                spec_rule!("req.TestAttributeProcedureTarget");
                "E-TST-0109"
            }
            _ => {
                if name == attrs::LAYOUT {
                    spec_rule!("def.LayoutAttributeApplicability");
                }
                spec_rule!("AttrList-Target-Err");
                "E-MOD-2452"
            }
        };
        return Err(reject(attr, diag_id, format!("Attribute '{name}' cannot be applied here")));
    }
    match name {
        attrs::DERIVE => validate_derive_args(attr)?,
        attrs::TEST => validate_test_args(attr)?,
        attrs::COLD
        | attrs::REFLECT
        | attrs::DYNAMIC
        | attrs::STALE_OK
        | attrs::EMIT
        | attrs::FILES
        | attrs::FFI_PASS_BY_VALUE
        | attrs::STATIC
        | attrs::RELAXED
        | attrs::ACQUIRE
        | attrs::RELEASE
        | attrs::ACQ_REL
        | attrs::SEQ_CST => {
            if !attr.args.is_empty() {
                return Err(reject(attr, "E-MOD-2450", format!("Malformed #{name} syntax")));
            }
        }
        attrs::LAYOUT => validate_layout_args(attr, target)?,
        attrs::MANGLE => validate_mangle_args(attr)?,
        attrs::INLINE => validate_inline_args(attr)?,
        attrs::DEPRECATED => validate_deprecated_args(attr)?,
        attrs::EXPORT | attrs::HOST_EXPORT => validate_export_args(attr)?,
        attrs::LIBRARY if target == AttributeTarget::ExternBlock => validate_library_args(attr)?,
        attrs::UNWIND if !has_only_positional_token_arg(attr) => {
            return Err(reject(attr, "E-MOD-2450", "Malformed #unwind syntax"));
        }
        _ => {}
    }
    validate_required_args(attr, spec)
}

/// Validates every attribute of a list against the registry; the first failure wins.
pub fn validate_attributes(attrs: &[AttributeItem], target: AttributeTarget) -> AttributeValidationResult {
    spec_rule!("AttrListJudg");
    spec_rule!("AttrListWf");
    spec_rule!("AttrValidation");
    for attr in attrs {
        if let Err(rejected) = validate_attribute(attr, target) {
            return *rejected;
        }
    }
    AttributeValidationResult::default()
}

/// Rejects any attribute on a construct that accepts none.
pub fn validate_unsupported_attribute_target(
    attrs: &[AttributeItem],
    target_name: &str,
) -> AttributeValidationResult {
    let Some(attr) = attrs.first() else {
        return AttributeValidationResult::default();
    };
    let name = attr.name.full_name.as_str();
    if lookup_attribute(name).is_none() {
        let diag_id =
            if name.starts_with(&reserved_attribute_prefix()) { "E-CNF-0402" } else { "E-MOD-2451" };
        return *reject(attr, diag_id, format!("Unknown attribute: {name}"));
    }
    let diag_id = match name {
        attrs::LIBRARY => "E-SYS-3345",
        attrs::TEST => "E-TST-0109",
        _ => "E-MOD-2452",
    };
    *reject(attr, diag_id, format!("Attribute '{name}' cannot be applied to {target_name}"))
}

pub fn has_attribute(attrs: &[AttributeItem], name: &str) -> bool {
    attrs.iter().any(|attr| attr.name.full_name == name)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationModeAttribute {
    Static,
    Dynamic,
}

pub fn resolve_verification_mode_attribute(attrs: &[AttributeItem]) -> Option<VerificationModeAttribute> {
    if has_attribute(attrs, attrs::STATIC) {
        Some(VerificationModeAttribute::Static)
    } else if has_attribute(attrs, attrs::DYNAMIC) {
        Some(VerificationModeAttribute::Dynamic)
    } else {
        None
    }
}

/// Lexeme of the first token argument with the given key (positional when the key is empty).
pub fn get_attribute_value(attrs: &[AttributeItem], name: &str, arg_name: &str) -> Option<String> {
    attrs
        .iter()
        .filter(|attr| attr.name.full_name == name)
        .flat_map(|attr| &attr.args)
        .filter(|arg| match &arg.key {
            Some(key) => !arg_name.is_empty() && key == arg_name,
            None => arg_name.is_empty(),
        })
        .find_map(|arg| token_arg(arg).map(|token| token.lexeme.clone()))
}
