//! The textual form of the IR that `uvc --emit-ir` prints. See `ir_dump.cpp`.

use std::collections::HashMap;
use std::fmt::Write;

use uv_analysis::typing::types::{type_to_string, TypeRef};

use crate::ir::*;

struct Dumper {
    out: String,
    indent_level: usize,
    display_map: HashMap<String, String>,
    addr_place: HashMap<String, String>,
}

fn type_text(ty: &TypeRef) -> String {
    if ty.is_some() {
        type_to_string(ty)
    } else {
        "<unknown>".to_string()
    }
}

/// A trailing `_<digits>` is how temporaries are numbered; the dump leaves it off.
fn normalize_opaque_name(name: &str) -> &str {
    let Some(underscore) = name.rfind('_') else {
        return name;
    };
    let digits = &name[underscore + 1..];
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return name;
    }
    &name[..underscore]
}

fn is_simple_ident(name: &str) -> bool {
    let mut bytes = name.bytes();
    match bytes.next() {
        Some(first) if first.is_ascii_alphabetic() || first == b'_' => bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_'),
        _ => false,
    }
}

fn should_combine_addr_of(place: &str) -> bool {
    !place.is_empty() && !place.starts_with('*') && !place.contains('.')
}

impl Dumper {
    fn w(&mut self, text: &str) {
        self.out.push_str(text);
    }

    fn indent(&mut self) {
        for _ in 0..self.indent_level {
            self.out.push_str("  ");
        }
    }

    fn dump_immediate_bytes(&mut self, value: &IrValue) {
        let mut hex = String::new();
        for byte in value.bytes.iter().rev() {
            let _ = write!(hex, "{byte:02X}");
        }
        let trimmed = hex.trim_start_matches('0');
        if trimmed.is_empty() {
            self.w("0x0");
        } else {
            let text = format!("0x{trimmed}");
            self.w(&text);
        }
    }

    fn dump_value(&mut self, value: &IrValue) {
        match value.kind {
            IrValueKind::Opaque => {
                if value.name.is_empty() {
                    self.w("opaque");
                } else if let Some(shown) = self.display_map.get(&value.name).cloned() {
                    self.w(&shown);
                } else {
                    self.w(normalize_opaque_name(&value.name));
                }
            }
            IrValueKind::Local => {
                self.w("%");
                self.w(&value.name);
            }
            IrValueKind::Symbol => {
                self.w("@");
                self.w(&value.name);
            }
            IrValueKind::Immediate => {
                if value.bytes.is_empty() {
                    self.w(&value.name);
                } else {
                    self.dump_immediate_bytes(value);
                }
            }
        }
    }

    fn dump_opt_value(&mut self, value: &Option<IrValue>) {
        match value {
            None => self.w("?"),
            Some(value) => self.dump_value(value),
        }
    }

    fn dump_range(&mut self, range: &IrRange) {
        match range.kind {
            IrRangeKind::Full => self.w(".."),
            IrRangeKind::From => {
                self.dump_opt_value(&range.lo);
                self.w("..");
            }
            IrRangeKind::To => {
                self.w("..");
                self.dump_opt_value(&range.hi);
            }
            IrRangeKind::ToInclusive => {
                self.w("..=");
                self.dump_opt_value(&range.hi);
            }
            IrRangeKind::Exclusive => {
                self.dump_opt_value(&range.lo);
                self.w("..");
                self.dump_opt_value(&range.hi);
            }
            IrRangeKind::Inclusive => {
                self.dump_opt_value(&range.lo);
                self.w("..=");
                self.dump_opt_value(&range.hi);
            }
        }
    }

    fn dump_pattern(&mut self, pattern: &IrPattern) {
        match &pattern.node {
            IrPatternNode::Literal(literal) => self.w(&format!("<literal {}>", literal.lexeme)),
            IrPatternNode::Wildcard => self.w("_"),
            IrPatternNode::Identifier { name } | IrPatternNode::Typed { name, .. } => self.w(name),
            IrPatternNode::Modal { state, fields } => {
                self.w("@");
                self.w(state);
                if let Some(fields) = fields {
                    self.w(" {");
                    for (i, field) in fields.iter().enumerate() {
                        if i > 0 {
                            self.w(", ");
                        }
                        self.w(&field.name);
                        if let Some(pattern) = &field.pattern {
                            self.w(": ");
                            self.dump_pattern(pattern);
                        }
                    }
                    self.w("}");
                }
            }
            _ => self.w("<pattern>"),
        }
    }

    fn dump_values(&mut self, values: &[IrValue]) {
        for (i, value) in values.iter().enumerate() {
            if i > 0 {
                self.w(", ");
            }
            self.dump_value(value);
        }
    }

    fn dump_call(&mut self, callee: &IrValue, args: &[IrValue]) {
        self.w("call ");
        self.dump_value(callee);
        if !args.is_empty() {
            self.w(" (");
            self.dump_values(args);
            self.w(")");
        }
    }

    fn dump(&mut self, ir: &Option<IrPtr>) {
        match ir {
            None => self.w("null"),
            Some(ir) => self.dump_ir(ir),
        }
    }

    fn dump_seq(&mut self, items: &[IrPtr]) {
        self.w("seq {\n");
        self.indent_level += 1;
        let mut i = 0;
        while i < items.len() {
            let item = &items[i];
            let next = items.get(i + 1).map(|next| next.as_ref());
            // A call whose result is bound next, an address taken and bound, an allocation
            // and its bind print as one line.
            if let (Ir::Call { callee, args, .. }, Some(Ir::BindVar { name, value, .. })) = (item.as_ref(), next) {
                if value.kind == IrValueKind::Opaque && matches!(value.name.as_str(), "call_result" | "method_call_result" | "dyncall_result") {
                    self.indent();
                    self.w(&format!("bind %{name} = "));
                    self.dump_call(callee, args);
                    self.w("\n");
                    i += 2;
                    continue;
                }
            }
            if let (Ir::AddrOf { place, result, .. }, Some(Ir::BindVar { name, value, .. })) = (item.as_ref(), next) {
                if value.kind == IrValueKind::Opaque && normalize_opaque_name(&value.name) == "addr_of" && should_combine_addr_of(&place.repr) {
                    if result.kind == IrValueKind::Opaque && !result.name.is_empty() {
                        self.display_map.insert(result.name.clone(), "addr_of".to_string());
                        if !place.repr.is_empty() {
                            self.addr_place.insert(result.name.clone(), place.repr.clone());
                        }
                    }
                    self.indent();
                    self.w(&format!("bind %{name} = addr_of"));
                    if !place.repr.is_empty() {
                        self.w(&format!(" {}", place.repr));
                    }
                    self.w("\n");
                    i += 2;
                    continue;
                }
            }
            if let (Ir::Alloc { region, .. }, Some(Ir::BindVar { name, value, .. })) = (item.as_ref(), next) {
                if value.kind == IrValueKind::Opaque && value.name == "alloc_ptr" {
                    self.indent();
                    self.w(&format!("bind %{name} = alloc"));
                    if let Some(region) = region {
                        self.w(" in ");
                        self.dump_value(region);
                    }
                    self.w("\n");
                    i += 2;
                    continue;
                }
            }
            self.indent();
            self.dump_ir(item);
            self.w("\n");
            i += 1;
        }
        self.indent_level -= 1;
        self.indent();
        self.w("}");
    }

    fn field(&mut self, label: &str, value: &IrValue) {
        self.indent();
        self.w(label);
        self.dump_value(value);
        self.w("\n");
    }

    fn field_ir(&mut self, label: &str, ir: &Option<IrPtr>) {
        self.indent();
        self.w(label);
        self.dump(ir);
        self.w("\n");
    }

    fn paths(&mut self, paths: &[String]) {
        self.w(&paths.join(", "));
    }

    fn dump_ir(&mut self, ir: &IrPtr) {
        match ir.as_ref() {
            Ir::Opaque => self.w("nop"),
            Ir::Seq { items } => self.dump_seq(items),
            Ir::Return { value } => {
                self.w("ret ");
                self.dump_value(value);
            }
            Ir::BindVar { name, value, .. } => {
                if value.kind == IrValueKind::Opaque && !value.name.is_empty() && normalize_opaque_name(&value.name) == "addr_of" {
                    if let Some(place) = self.addr_place.get(&value.name).filter(|place| should_combine_addr_of(place)).cloned() {
                        self.w(&format!("bind %{name} = addr_of"));
                        if !place.is_empty() {
                            self.w(&format!(" {place}"));
                        }
                        return;
                    }
                }
                self.w(&format!("bind %{name} = "));
                self.dump_value(value);
            }
            Ir::Call { callee, args, .. } => self.dump_call(callee, args),
            Ir::CallVTable { base, slot, args, check_dynamic_receiver_addr_active, .. } => {
                self.w("call_vtable ");
                self.dump_value(base);
                self.w(&format!(" [{slot}]"));
                if *check_dynamic_receiver_addr_active {
                    self.w(" check_dynamic_receiver_addr_active");
                }
                if !args.is_empty() {
                    self.w(" (");
                    self.dump_values(args);
                    self.w(")");
                }
            }
            Ir::StoreGlobal { symbol, value } => {
                self.w(&format!("store @{symbol} = "));
                self.dump_value(value);
            }
            Ir::ReadVar { name } => self.w(&format!("read %{name}")),
            Ir::ReadPath { path, name } => {
                self.w("read @");
                for part in path {
                    self.w(&format!("{part}::"));
                }
                self.w(name);
            }
            Ir::StoreVar { name, value } => {
                self.w(&format!("store %{name} = "));
                self.dump_value(value);
            }
            Ir::StoreVarNoDrop { name, value } => {
                self.w(&format!("store_nodrop %{name} = "));
                self.dump_value(value);
            }
            Ir::ReadPlace { place } => self.w(&format!("read_place {}", place.repr)),
            Ir::WritePlace { place, .. } => {
                self.w("rec_update");
                if !place.repr.is_empty() {
                    self.w(&format!(" {}", place.repr));
                }
            }
            Ir::AddrOf { place, result, .. } => {
                if result.kind == IrValueKind::Opaque && !result.name.is_empty() {
                    self.display_map.insert(result.name.clone(), "addr_of".to_string());
                    if !place.repr.is_empty() {
                        self.addr_place.insert(result.name.clone(), place.repr.clone());
                    }
                }
                self.w("addr_of");
                if !place.repr.is_empty() {
                    self.w(&format!(" {}", place.repr));
                }
            }
            Ir::ReadPtr { ptr, result } => {
                if ptr.kind == IrValueKind::Opaque && result.kind == IrValueKind::Opaque && !ptr.name.is_empty() && !result.name.is_empty() {
                    if let Some(place) = self.addr_place.get(&ptr.name).cloned() {
                        let shown = if is_simple_ident(&place) { format!("%{place}") } else { place };
                        self.display_map.insert(result.name.clone(), shown);
                    }
                }
                self.w("load_ptr ");
                self.dump_value(ptr);
            }
            Ir::WritePtr { ptr, value } => {
                if ptr.kind == IrValueKind::Opaque && !ptr.name.is_empty() {
                    if let Some(place) = self.addr_place.get(&ptr.name).cloned() {
                        self.w("rec_update");
                        if !place.is_empty() {
                            self.w(&format!(" {place}"));
                        }
                        return;
                    }
                }
                self.w("store_ptr ");
                self.dump_value(ptr);
                self.w(" = ");
                self.dump_value(value);
            }
            Ir::UnaryOp { op, operand, .. } => {
                self.w(&format!("unop {op} "));
                self.dump_value(operand);
            }
            Ir::Fence { order, .. } => self.w(match order {
                IrFenceOrder::Acquire => "fence acquire",
                IrFenceOrder::Release => "fence release",
                IrFenceOrder::SeqCst => "fence seqcst",
            }),
            Ir::BinaryOp { op, lhs, rhs, .. } => {
                self.w(&format!("binop {op} "));
                self.dump_value(lhs);
                self.w(", ");
                self.dump_value(rhs);
            }
            Ir::Cast { target, value, .. } => {
                self.w("cast ");
                self.dump_value(value);
                self.w(&format!(" to {}", type_text(target)));
            }
            Ir::Transmute { to, value, .. } => {
                self.w("transmute ");
                self.dump_value(value);
                self.w(&format!(" to {}", type_text(to)));
            }
            Ir::CheckIndex { base, index } => {
                self.w("check_index ");
                self.dump_value(base);
                self.w(", ");
                self.dump_value(index);
            }
            Ir::CheckRange { base, range, .. } => {
                self.w("check_range ");
                self.dump_value(base);
                self.w(", ");
                self.dump_range(range);
            }
            Ir::CheckSliceLen { base, range, value, .. } => {
                self.w("check_slice_len ");
                self.dump_value(base);
                self.w(", ");
                self.dump_range(range);
                self.w(", ");
                self.dump_value(value);
            }
            Ir::CheckOp { op, reason, lhs, rhs } => {
                self.w(&format!("check_op {reason} {op} "));
                self.dump_value(lhs);
                if let Some(rhs) = rhs {
                    self.w(", ");
                    self.dump_value(rhs);
                }
            }
            Ir::CheckCast { target, value } => {
                self.w("check_cast ");
                self.dump_value(value);
                self.w(&format!(" to {}", type_text(target)));
            }
            Ir::Alloc { region, value, .. } => {
                self.w("alloc");
                if let Some(region) = region {
                    self.w(" in ");
                    self.dump_value(region);
                }
                self.w(" ");
                self.dump_value(value);
            }
            Ir::ContextBundleBuild { target_type, .. } => self.w(&format!("context_bundle_build {}", type_text(target_type))),
            Ir::Result { value } => {
                self.w("result ");
                self.dump_value(value);
            }
            Ir::Break { value } => {
                self.w("break");
                if let Some(value) = value {
                    self.w(" ");
                    self.dump_value(value);
                }
            }
            Ir::Continue => self.w("continue"),
            Ir::Defer => self.w("defer"),
            Ir::MoveState { place } => self.w(&format!("move_state {}", place.repr)),
            Ir::If { cond, then_ir, else_ir, .. } => {
                self.w("if ");
                self.dump_value(cond);
                self.w(" then ");
                self.dump(then_ir);
                self.w(" else ");
                self.dump(else_ir);
            }
            Ir::Block { setup, body, .. } => {
                self.w("block ");
                self.dump(setup);
                self.w(" ");
                self.dump(body);
            }
            Ir::Loop { kind, iter_ir, iter_value, cond_ir, cond_value, body_ir, invariant_entry_ir, invariant_backedge_ir, body_value, .. } => {
                self.w("loop");
                let (header, value_label) = match kind {
                    IrLoopKind::Infinite => (" infinite {\n", "value "),
                    IrLoopKind::Conditional => (" while {\n", "body_value "),
                    IrLoopKind::Iter => (" iter {\n", "body_value "),
                };
                self.w(header);
                self.indent_level += 1;
                match kind {
                    IrLoopKind::Infinite => {}
                    IrLoopKind::Conditional => {
                        self.field_ir("cond_ir ", cond_ir);
                        if let Some(value) = cond_value {
                            self.field("cond_value ", value);
                        }
                    }
                    IrLoopKind::Iter => {
                        if iter_ir.is_some() {
                            self.field_ir("iter_ir ", iter_ir);
                        }
                        if let Some(value) = iter_value {
                            self.field("iter_value ", value);
                        }
                    }
                }
                self.field_ir("body ", body_ir);
                if invariant_entry_ir.is_some() {
                    self.field_ir("invariant_entry ", invariant_entry_ir);
                }
                if invariant_backedge_ir.is_some() {
                    self.field_ir("invariant_backedge ", invariant_backedge_ir);
                }
                self.field(value_label, body_value);
                self.indent_level -= 1;
                self.indent();
                self.w("}");
            }
            Ir::IfCase { scrutinee, arms, .. } => {
                self.w("if_case ");
                self.dump_value(scrutinee);
                self.w(" {\n");
                self.indent_level += 1;
                for arm in arms {
                    self.indent();
                    self.w("case_clause {\n");
                    self.indent_level += 1;
                    if let Some(pattern) = &arm.pattern {
                        self.indent();
                        self.w("pat ");
                        self.dump_pattern(pattern);
                        self.w("\n");
                    }
                    self.indent();
                    self.w("body {\n");
                    self.indent_level += 1;
                    self.dump(&arm.body);
                    self.indent_level -= 1;
                    self.indent();
                    self.w("}\n");
                    self.field("value ", &arm.value);
                    if arm.cleanup_ir.as_deref().is_some_and(|cleanup| !matches!(cleanup, Ir::Opaque)) {
                        self.indent();
                        self.w("cleanup {\n");
                        self.indent_level += 1;
                        self.dump(&arm.cleanup_ir);
                        self.indent_level -= 1;
                        self.indent();
                        self.w("}\n");
                    }
                    self.indent_level -= 1;
                    self.indent();
                    self.w("}\n");
                }
                self.indent_level -= 1;
                self.indent();
                self.w("}");
            }
            Ir::Region { owner, .. } => {
                self.w("region ");
                self.dump_value(owner);
            }
            Ir::Frame { region, .. } => {
                self.w("frame");
                if let Some(region) = region {
                    self.w(" in ");
                    self.dump_value(region);
                }
            }
            Ir::Branch { cond, true_label, false_label } => match cond {
                Some(cond) => {
                    self.w("br_if ");
                    self.dump_value(cond);
                    self.w(&format!(" {true_label} {false_label}"));
                }
                None => self.w(&format!("br {true_label}")),
            },
            Ir::Phi { ty, .. } => self.w(&format!("phi {}", type_text(ty))),
            Ir::ClearPanic => self.w("clear_panic"),
            Ir::PanicCheck => self.w("panic_check"),
            Ir::CleanupPanicCheck { cleanup_ir } => {
                self.w("cleanup_panic_check");
                if cleanup_ir.is_some() {
                    self.w(" {\n");
                    self.indent_level += 1;
                    self.dump(cleanup_ir);
                    self.indent_level -= 1;
                    self.indent();
                    self.w("}");
                }
            }
            Ir::InitPanicHandle { module, poison_modules, cleanup_ir } => self.init_panic("init_panic_handle", module, poison_modules, cleanup_ir),
            Ir::InitPanicRaise { module, poison_modules, cleanup_ir } => self.init_panic("init_panic_raise", module, poison_modules, cleanup_ir),
            Ir::CheckPoison { module } => self.w(&format!("check_poison {module}")),
            Ir::LowerPanic { reason, .. } => self.w(&format!("panic {reason}")),
            Ir::Parallel { domain, body, .. } => {
                self.w("parallel {\n");
                self.indent_level += 1;
                self.field("domain: ", domain);
                self.indent();
                self.w("body:\n");
                self.indent_level += 1;
                self.indent();
                self.dump(body);
                self.w("\n");
                self.indent_level -= 2;
                self.indent();
                self.w("}");
            }
            Ir::Spawn { captured_env, body, body_result, result, runtime_symbol, runtime_receiver, affinity_mask, priority, name, .. } => {
                self.w("spawn {\n");
                self.indent_level += 1;
                if captured_env.is_some() {
                    self.indent();
                    self.w("captured_env:\n");
                    self.indent_level += 1;
                    self.indent();
                    self.dump(captured_env);
                    self.w("\n");
                    self.indent_level -= 1;
                }
                self.indent();
                self.w("body:\n");
                self.indent_level += 1;
                self.indent();
                self.dump(body);
                self.w("\n");
                self.indent_level -= 1;
                self.field("body_result: ", body_result);
                self.field("result: ", result);
                if let Some(symbol) = runtime_symbol {
                    self.indent();
                    self.w(&format!("runtime_symbol: {symbol}\n"));
                }
                if let Some(receiver) = runtime_receiver {
                    self.field("runtime_receiver: ", receiver);
                }
                if let Some(mask) = affinity_mask {
                    self.field("affinity: ", mask);
                }
                if let Some(priority) = priority {
                    self.field("priority: ", priority);
                }
                if !name.is_empty() {
                    self.indent();
                    self.w(&format!("name: {name}\n"));
                }
                self.indent_level -= 1;
                self.indent();
                self.w("}");
            }
            Ir::Wait { handle, result, kind } => {
                self.w("wait ");
                self.dump_value(handle);
                self.w(" -> ");
                self.dump_value(result);
                match kind {
                    IrWaitKind::Spawned => self.w(" [spawned]"),
                    IrWaitKind::Tracked => self.w(" [tracked]"),
                    IrWaitKind::Unknown => {}
                }
            }
            Ir::CancelCreate { result } => {
                self.w("cancel_create -> ");
                self.dump_value(result);
            }
            Ir::CancelRequest { token, result } => self.token_op("cancel_request ", token, result),
            Ir::CancelWait { token, result } => self.token_op("cancel_wait ", token, result),
            Ir::CancelCheck { token, result } => self.token_op("cancel_check ", token, result),
            Ir::CancelSuppress => self.w("cancel_suppress"),
            Ir::GpuBarrier { kind, result } => {
                self.w(match kind {
                    IrGpuBarrierKind::Memory => "gpu_barrier memory",
                    IrGpuBarrierKind::Workgroup => "gpu_barrier workgroup",
                    IrGpuBarrierKind::Full => "gpu_barrier full",
                });
                self.w(" -> ");
                self.dump_value(result);
            }
            Ir::Dispatch { range, body, body_result, result, reduce_op, ordered, chunk_size, workgroup_size, .. } => {
                self.w("dispatch {\n");
                self.indent_level += 1;
                self.field("range: ", range);
                if *ordered {
                    self.indent();
                    self.w("ordered: true\n");
                }
                if let Some(chunk) = chunk_size {
                    self.field("chunk_size: ", chunk);
                }
                self.field("workgroup_size: ", workgroup_size);
                if let Some(op) = reduce_op {
                    self.indent();
                    self.w(&format!("reduce_op: {op}\n"));
                }
                self.indent();
                self.w("body:\n");
                self.indent_level += 1;
                self.indent();
                self.dump(body);
                self.w("\n");
                self.indent_level -= 1;
                self.field("body_result: ", body_result);
                self.field("result: ", result);
                self.indent_level -= 1;
                self.indent();
                self.w("}");
            }
            Ir::Yield { release, value, result, state_index, .. } => {
                self.w("yield");
                if *release {
                    self.w(" release");
                }
                self.w(" ");
                self.dump_value(value);
                self.w(&format!(" [state={state_index}] -> "));
                self.dump_value(result);
            }
            Ir::YieldFrom { release, source, result, state_index, .. } => {
                self.w("yield");
                if *release {
                    self.w(" release");
                }
                self.w(" from ");
                self.dump_value(source);
                self.w(&format!(" [state={state_index}] -> "));
                self.dump_value(result);
            }
            Ir::SpecSnapshot { paths, result } => {
                self.w("spec_snapshot paths=[");
                self.paths(paths);
                self.w("] -> ");
                self.dump_value(result);
            }
            Ir::SpecValidate { paths, result } => {
                self.w("spec_validate paths=[");
                self.paths(paths);
                self.w("] -> ");
                self.dump_value(result);
            }
            Ir::SpecCommit { paths, value, result } => {
                self.w("spec_commit paths=[");
                self.paths(paths);
                self.w("] value=");
                self.dump_value(value);
                self.w(" -> ");
                self.dump_value(result);
            }
            Ir::SpecRetry { result } => {
                self.w("spec_retry -> ");
                self.dump_value(result);
            }
            Ir::SpecFallback { body, result } => {
                self.w("spec_fallback ");
                self.dump(body);
                self.w(" -> ");
                self.dump_value(result);
            }
            Ir::SpecLoop { snapshot_ir, body_ir, validate_ir, commit_ir, retry_ir, fallback_ir, result } => {
                self.w("spec_loop {\n");
                self.indent_level += 1;
                self.field_ir("snapshot: ", snapshot_ir);
                self.field_ir("body: ", body_ir);
                self.field_ir("validate: ", validate_ir);
                self.field_ir("commit: ", commit_ir);
                self.field_ir("retry: ", retry_ir);
                self.field_ir("fallback: ", fallback_ir);
                self.field("result: ", result);
                self.indent_level -= 1;
                self.indent();
                self.w("}");
            }
            Ir::Sync { async_value, result, runtime_symbol, runtime_receiver, .. } => {
                self.w("sync ");
                self.dump_value(async_value);
                if let Some(symbol) = runtime_symbol {
                    self.w(&format!(" runtime_symbol={symbol}"));
                }
                if let Some(receiver) = runtime_receiver {
                    self.w(" runtime_receiver=");
                    self.dump_value(receiver);
                }
                self.w(" -> ");
                self.dump_value(result);
            }
            Ir::RaceReturn { arms, result, .. } => self.race("race {\n", arms, result, true),
            Ir::RaceYield { arms, result, .. } => self.race("race_yield {\n", arms, result, false),
            Ir::All { async_irs, async_values, result, .. } => {
                self.w("all {\n");
                self.indent_level += 1;
                for (i, async_ir) in async_irs.iter().enumerate() {
                    self.indent();
                    self.w(&format!("async[{i}]:\n"));
                    self.indent_level += 1;
                    self.indent();
                    self.w("ir: ");
                    self.dump_ir(async_ir);
                    self.w("\n");
                    if let Some(value) = async_values.get(i) {
                        self.field("value: ", value);
                    }
                    self.indent_level -= 1;
                }
                self.field("result: ", result);
                self.indent_level -= 1;
                self.indent();
                self.w("}");
            }
            Ir::AsyncComplete { value, result, .. } => {
                self.w("async_complete value: ");
                self.dump_value(value);
                self.w(" -> ");
                self.dump_value(result);
            }
            Ir::AsyncFail { value, result, .. } => {
                self.w("async_fail value: ");
                self.dump_value(value);
                self.w(" -> ");
                self.dump_value(result);
            }
        }
    }

    fn init_panic(&mut self, label: &str, module: &str, poison_modules: &[String], cleanup_ir: &Option<IrPtr>) {
        self.w(&format!("{label} {module}"));
        if !poison_modules.is_empty() {
            self.w(&format!(" [{}]", poison_modules.join(", ")));
        }
        if cleanup_ir.is_some() {
            self.w(" cleanup");
        }
    }

    fn token_op(&mut self, label: &str, token: &IrValue, result: &IrValue) {
        self.w(label);
        self.dump_value(token);
        self.w(" -> ");
        self.dump_value(result);
    }

    fn race(&mut self, header: &str, arms: &[IrRaceArm], result: &IrValue, with_match_value: bool) {
        self.w(header);
        self.indent_level += 1;
        for (i, arm) in arms.iter().enumerate() {
            self.indent();
            self.w(&format!("arm[{i}]:\n"));
            self.indent_level += 1;
            self.field_ir("async_ir: ", &arm.async_ir);
            self.field("async_value: ", &arm.async_value);
            if with_match_value {
                self.field("match_value: ", &arm.match_value);
            }
            self.field_ir("handler_ir: ", &arm.handler_ir);
            self.field("handler_result: ", &arm.handler_result);
            self.indent_level -= 1;
        }
        self.field("result: ", result);
        self.indent_level -= 1;
        self.indent();
        self.w("}");
    }
}

/// The text of a list of declarations, as `--emit-ir` prints it.
pub fn dump_ir(decls: &IrDecls) -> String {
    let mut d = Dumper { out: String::new(), indent_level: 0, display_map: HashMap::new(), addr_place: HashMap::new() };
    for decl in decls {
        match decl {
            IrDecl::Proc(item) => {
                d.w(&format!("proc @{} {{\n", item.symbol));
                d.indent_level += 1;
                d.indent();
                d.dump(&item.body);
                d.w("\n");
                d.indent_level -= 1;
                d.w("}\n\n");
            }
            IrDecl::GlobalConst(item) => d.w(&format!("global_const @{} bytes={}\n\n", item.symbol, item.bytes.len())),
            IrDecl::GlobalZero(item) => d.w(&format!("global_zero @{} size={}\n\n", item.symbol, item.size)),
            IrDecl::GlobalVTable(item) => {
                d.w(&format!("vtable @{} size={} align={} drop=@{}", item.symbol, item.header.size, item.header.align, item.header.drop_sym));
                if !item.slots.is_empty() {
                    let slots: Vec<String> = item.slots.iter().map(|slot| format!("@{slot}")).collect();
                    d.w(&format!(" slots=[{}]", slots.join(", ")));
                }
                d.w("\n\n");
            }
            IrDecl::ExternProc(item) => {
                d.w(&format!("extern_proc @{}", item.symbol));
                if let Some(abi) = &item.abi {
                    d.w(&format!(" \"{abi}\""));
                }
                if let (Some(library), Some(foreign)) = (&item.raw_dylib_library_name, &item.raw_dylib_foreign_symbol) {
                    d.w(&format!(" raw_dylib=\"{library}\" foreign=\"{foreign}\""));
                    if item.raw_dylib_catch_unwind {
                        d.w(" unwind=catch");
                    }
                }
                d.w("\n\n");
            }
        }
    }
    d.out
}

/// The text of one node.
pub fn dump_node(ir: &Option<IrPtr>) -> String {
    let mut d = Dumper { out: String::new(), indent_level: 0, display_map: HashMap::new(), addr_place: HashMap::new() };
    d.dump(ir);
    d.out
}
