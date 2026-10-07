//! Literal data (`globals/literal_emit.cpp`): the global each literal value is kept in.

use super::*;
use uv_core::hash::{fnv1a64, hex64};
use uv_core::symbols::mangle;
use uv_project::language_profile::runtime_path_sig;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiteralKind {
    String,
    Bytes,
    Char,
    Int,
    Float,
}

impl LiteralKind {
    fn name(self) -> &'static str {
        match self {
            LiteralKind::String => "string",
            LiteralKind::Bytes => "bytes",
            LiteralKind::Char => "char",
            LiteralKind::Int => "int",
            LiteralKind::Float => "float",
        }
    }
}

/// `LiteralSym`: `ultraviolet::runtime::literal::<kind>_<hash of the bytes>`, mangled.
pub fn literal_symbol(kind: LiteralKind, bytes: &[u8]) -> String {
    let id = format!("{}_{}", mangle(kind.name()), hex64(fnv1a64(bytes)));
    runtime_path_sig(&["literal", &id])
}

/// `IsLiteralSymbol`.
pub fn is_literal_symbol(symbol: &str) -> bool {
    let prefix = runtime_path_sig(&["literal"]);
    symbol.starts_with(&prefix)
}

/// `LiteralKindOfImmediate`: what kind of literal an immediate is, when it is one.
pub fn literal_kind_of_immediate(value: &IrValue) -> Option<LiteralKind> {
    if value.kind != IrValueKind::Immediate || value.bytes.is_empty() {
        return None;
    }
    if let Some(kind) = value.literal_kind {
        return Some(match kind {
            IrImmediateLiteralKind::String => LiteralKind::String,
            IrImmediateLiteralKind::Bytes => LiteralKind::Bytes,
            IrImmediateLiteralKind::Char => LiteralKind::Char,
            IrImmediateLiteralKind::Int => LiteralKind::Int,
            IrImmediateLiteralKind::Float => LiteralKind::Float,
        });
    }
    let lexeme = value.name.as_str();
    if matches!(lexeme, "true" | "false" | "null") {
        return None;
    }
    if lexeme.len() >= 2 && lexeme.starts_with('"') && lexeme.ends_with('"') {
        return Some(LiteralKind::String);
    }
    if lexeme.len() >= 2 && lexeme.starts_with('\'') && lexeme.ends_with('\'') {
        return Some(LiteralKind::Char);
    }
    let looks_float = lexeme.contains('.') || lexeme.contains('e') || lexeme.contains('E') || lexeme.ends_with('f') || lexeme.ends_with("f16") || lexeme.ends_with("f32") || lexeme.ends_with("f64");
    Some(if looks_float { LiteralKind::Float } else { LiteralKind::Int })
}

/// One literal an IR refers to.
#[derive(Debug, Clone)]
pub struct LiteralRef {
    pub kind: LiteralKind,
    pub bytes: Vec<u8>,
}

fn collect_value(value: &IrValue, out: &mut Vec<LiteralRef>) {
    if let Some(kind) = literal_kind_of_immediate(value) {
        out.push(LiteralRef { kind, bytes: value.bytes.clone() });
    }
}

fn collect_opt_value(value: &Option<IrValue>, out: &mut Vec<LiteralRef>) {
    if let Some(value) = value {
        collect_value(value, out);
    }
}

fn collect_range(range: &IrRange, out: &mut Vec<LiteralRef>) {
    collect_opt_value(&range.lo, out);
    collect_opt_value(&range.hi, out);
}

fn collect_opt_ir(ir: &Option<IrPtr>, out: &mut Vec<LiteralRef>) {
    if let Some(ir) = ir {
        collect_ir(ir, out);
    }
}

/// `CollectLiteralRefsFromIR`: the immediates of the forms the reference looks at.
fn collect_ir(ir: &IrPtr, out: &mut Vec<LiteralRef>) {
    match ir.as_ref() {
        Ir::Seq { items } => items.iter().for_each(|item| collect_ir(item, out)),
        Ir::Call { callee, args, result } => {
            collect_value(callee, out);
            args.iter().for_each(|arg| collect_value(arg, out));
            collect_value(result, out);
        }
        Ir::CallVTable { base, args, result, .. } => {
            collect_value(base, out);
            args.iter().for_each(|arg| collect_value(arg, out));
            collect_value(result, out);
        }
        Ir::BindVar { value, .. } | Ir::StoreVar { value, .. } | Ir::StoreVarNoDrop { value, .. } | Ir::StoreGlobal { value, .. } | Ir::WritePlace { value, .. } => collect_value(value, out),
        Ir::ReadPtr { ptr, result } => {
            collect_value(ptr, out);
            collect_value(result, out);
        }
        Ir::WritePtr { ptr, value } => {
            collect_value(ptr, out);
            collect_value(value, out);
        }
        Ir::UnaryOp { operand, result, .. } => {
            collect_value(operand, out);
            collect_value(result, out);
        }
        Ir::Fence { result, .. } => collect_value(result, out),
        Ir::BinaryOp { lhs, rhs, result, .. } => {
            collect_value(lhs, out);
            collect_value(rhs, out);
            collect_value(result, out);
        }
        Ir::Cast { value, result, .. } | Ir::Transmute { value, result, .. } => {
            collect_value(value, out);
            collect_value(result, out);
        }
        Ir::CheckIndex { base, index } => {
            collect_value(base, out);
            collect_value(index, out);
        }
        Ir::CheckRange { base, range, range_value } => {
            collect_value(base, out);
            collect_range(range, out);
            collect_opt_value(range_value, out);
        }
        Ir::CheckSliceLen { base, range, range_value, value } => {
            collect_value(base, out);
            collect_range(range, out);
            collect_opt_value(range_value, out);
            collect_value(value, out);
        }
        Ir::CheckOp { lhs, rhs, .. } => {
            collect_value(lhs, out);
            collect_opt_value(rhs, out);
        }
        Ir::CheckCast { value, .. } => collect_value(value, out),
        Ir::Alloc { region, value, result, .. } => {
            collect_opt_value(region, out);
            collect_value(value, out);
            collect_value(result, out);
        }
        Ir::ContextBundleBuild { root_ctx, result, .. } => {
            collect_value(root_ctx, out);
            collect_value(result, out);
        }
        Ir::Return { value } | Ir::Result { value } => collect_value(value, out),
        Ir::Break { value } => collect_opt_value(value, out),
        Ir::If { cond, then_ir, then_value, else_ir, else_value, result } => {
            collect_value(cond, out);
            collect_opt_ir(then_ir, out);
            collect_opt_ir(else_ir, out);
            collect_value(then_value, out);
            collect_value(else_value, out);
            collect_value(result, out);
        }
        Ir::Loop { iter_ir, iter_value, cond_ir, cond_value, body_ir, invariant_entry_ir, invariant_backedge_ir, body_value, result, .. } => {
            collect_opt_ir(iter_ir, out);
            collect_opt_ir(cond_ir, out);
            collect_opt_ir(body_ir, out);
            collect_opt_ir(invariant_entry_ir, out);
            collect_opt_ir(invariant_backedge_ir, out);
            collect_opt_value(iter_value, out);
            collect_opt_value(cond_value, out);
            collect_value(body_value, out);
            collect_value(result, out);
        }
        Ir::IfCase { scrutinee, arms, result, .. } => {
            collect_value(scrutinee, out);
            for arm in arms {
                collect_opt_ir(&arm.body, out);
                collect_opt_ir(&arm.cleanup_ir, out);
                collect_value(&arm.value, out);
            }
            collect_value(result, out);
        }
        Ir::Block { setup, body, value } => {
            collect_opt_ir(setup, out);
            collect_opt_ir(body, out);
            collect_value(value, out);
        }
        Ir::Region { owner, body, value, .. } => {
            collect_value(owner, out);
            collect_opt_ir(body, out);
            collect_value(value, out);
        }
        Ir::Frame { region, body, value } => {
            collect_opt_value(region, out);
            collect_opt_ir(body, out);
            collect_value(value, out);
        }
        Ir::Branch { cond, .. } => collect_opt_value(cond, out),
        Ir::Phi { incoming, value, .. } => {
            incoming.iter().for_each(|incoming| collect_value(&incoming.value, out));
            collect_value(value, out);
        }
        Ir::CleanupPanicCheck { cleanup_ir } | Ir::InitPanicHandle { cleanup_ir, .. } | Ir::InitPanicRaise { cleanup_ir, .. } | Ir::LowerPanic { cleanup_ir, .. } => collect_opt_ir(cleanup_ir, out),
        Ir::Parallel { domain, body, result, cancel_token, .. } => {
            collect_value(domain, out);
            collect_opt_value(cancel_token, out);
            collect_opt_ir(body, out);
            collect_value(result, out);
        }
        Ir::Spawn { captured_env, body, body_result, result, env_ptr, env_size, body_fn, result_size, affinity_mask, priority, .. } => {
            collect_opt_ir(captured_env, out);
            collect_opt_ir(body, out);
            for value in [body_result, result, env_ptr, env_size, body_fn, result_size] {
                collect_value(value, out);
            }
            collect_opt_value(affinity_mask, out);
            collect_opt_value(priority, out);
        }
        Ir::Wait { handle, result, .. } => {
            collect_value(handle, out);
            collect_value(result, out);
        }
        Ir::CancelCreate { result } | Ir::GpuBarrier { result, .. } | Ir::SpecRetry { result } | Ir::SpecSnapshot { result, .. } | Ir::SpecValidate { result, .. } => collect_value(result, out),
        Ir::CancelRequest { token, result } | Ir::CancelWait { token, result } | Ir::CancelCheck { token, result } => {
            collect_value(token, out);
            collect_value(result, out);
        }
        Ir::Dispatch { range, body, body_result, captured_env, env_ptr, body_fn, elem_size, result_size, result_ptr, reduce_fn, result, chunk_size, workgroup_size, .. } => {
            collect_value(range, out);
            collect_opt_ir(body, out);
            collect_value(body_result, out);
            collect_opt_ir(captured_env, out);
            for value in [env_ptr, body_fn, elem_size, result_size, result_ptr] {
                collect_value(value, out);
            }
            collect_opt_value(reduce_fn, out);
            collect_value(result, out);
            collect_opt_value(chunk_size, out);
            collect_value(workgroup_size, out);
        }
        Ir::Yield { value, result, keys_record, .. } => {
            collect_value(value, out);
            collect_value(result, out);
            collect_value(keys_record, out);
        }
        Ir::YieldFrom { source, result, .. } => {
            collect_value(source, out);
            collect_value(result, out);
        }
        Ir::SpecCommit { value, result, .. } => {
            collect_value(value, out);
            collect_value(result, out);
        }
        Ir::SpecFallback { body, result } => {
            collect_opt_ir(body, out);
            collect_value(result, out);
        }
        Ir::SpecLoop { snapshot_ir, body_ir, validate_ir, commit_ir, retry_ir, fallback_ir, result } => {
            for ir in [snapshot_ir, body_ir, validate_ir, commit_ir, retry_ir, fallback_ir] {
                collect_opt_ir(ir, out);
            }
            collect_value(result, out);
        }
        Ir::Sync { async_value, runtime_receiver, result, .. } => {
            collect_value(async_value, out);
            collect_opt_value(runtime_receiver, out);
            collect_value(result, out);
        }
        Ir::RaceReturn { arms, result, .. } | Ir::RaceYield { arms, result, .. } => {
            for arm in arms {
                collect_opt_ir(&arm.async_ir, out);
                collect_value(&arm.async_value, out);
                collect_value(&arm.match_value, out);
                collect_opt_ir(&arm.handler_ir, out);
                collect_value(&arm.handler_result, out);
            }
            collect_value(result, out);
        }
        Ir::All { async_irs, async_values, result, .. } => {
            async_irs.iter().for_each(|ir| collect_ir(ir, out));
            async_values.iter().for_each(|value| collect_value(value, out));
            collect_value(result, out);
        }
        Ir::AsyncComplete { value, result, .. } | Ir::AsyncFail { value, result, .. } => {
            collect_value(value, out);
            collect_value(result, out);
        }
        _ => {}
    }
}

/// `LiteralRefs(decls)` with the duplicates removed (`DedupLiteralRefs`).
pub fn literal_refs(decls: &IrDecls) -> Vec<LiteralRef> {
    let mut refs = Vec::new();
    for decl in decls {
        if let IrDecl::Proc(proc) = decl {
            if let Some(body) = &proc.body {
                collect_ir(body, &mut refs);
            }
        }
    }
    let mut seen = std::collections::HashSet::new();
    refs.retain(|lit| seen.insert(literal_symbol(lit.kind, &lit.bytes)));
    refs
}
