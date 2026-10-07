//! Whether control can leave a piece of IR by falling off its end. See `ir_control_flow.h`.

use crate::ir::*;

#[derive(Debug, Clone, Copy)]
pub struct IrControlFlowSummary {
    pub may_fallthrough: bool,
    pub may_return: bool,
    pub may_break: bool,
    pub may_continue: bool,
    pub may_panic: bool,
}

impl Default for IrControlFlowSummary {
    fn default() -> Self {
        IrControlFlowSummary { may_fallthrough: true, may_return: false, may_break: false, may_continue: false, may_panic: false }
    }
}

fn merge(dst: &mut IrControlFlowSummary, src: &IrControlFlowSummary) {
    dst.may_return |= src.may_return;
    dst.may_break |= src.may_break;
    dst.may_continue |= src.may_continue;
    dst.may_panic |= src.may_panic;
}

fn sequence(first: &Option<IrPtr>, second: &Option<IrPtr>) -> IrControlFlowSummary {
    let mut out = summarize(first);
    if !out.may_fallthrough {
        return out;
    }
    let next = summarize(second);
    merge(&mut out, &next);
    out.may_fallthrough = next.may_fallthrough;
    out
}

fn sequence_items(items: &[IrPtr]) -> IrControlFlowSummary {
    let mut out = IrControlFlowSummary::default();
    for item in items {
        if !out.may_fallthrough {
            break;
        }
        let flow = summarize(&Some(item.clone()));
        merge(&mut out, &flow);
        out.may_fallthrough = flow.may_fallthrough;
    }
    out
}

fn terminating(configure: impl FnOnce(&mut IrControlFlowSummary)) -> IrControlFlowSummary {
    let mut out = IrControlFlowSummary { may_fallthrough: false, ..Default::default() };
    configure(&mut out);
    out
}

pub fn summarize(ir: &Option<IrPtr>) -> IrControlFlowSummary {
    let Some(ir) = ir else {
        return IrControlFlowSummary::default();
    };
    match ir.as_ref() {
        Ir::Seq { items } => sequence_items(items),
        Ir::Return { .. } => terminating(|out| out.may_return = true),
        Ir::Break { .. } => terminating(|out| out.may_break = true),
        Ir::Continue => terminating(|out| out.may_continue = true),
        Ir::LowerPanic { cleanup_ir, .. } | Ir::InitPanicRaise { cleanup_ir, .. } => {
            let mut out = summarize(cleanup_ir);
            out.may_fallthrough = false;
            out.may_panic = true;
            out.may_return = true;
            out
        }
        Ir::PanicCheck | Ir::CheckPoison { .. } => IrControlFlowSummary { may_fallthrough: true, may_return: true, may_panic: true, ..Default::default() },
        Ir::CleanupPanicCheck { cleanup_ir } | Ir::InitPanicHandle { cleanup_ir, .. } => {
            let mut out = summarize(cleanup_ir);
            out.may_fallthrough = true;
            out.may_return = true;
            out.may_panic = true;
            out
        }
        Ir::Block { setup, body, .. } => sequence(setup, body),
        Ir::If { then_ir, else_ir, .. } => {
            let (then_flow, else_flow) = (summarize(then_ir), summarize(else_ir));
            let mut out = IrControlFlowSummary { may_fallthrough: then_flow.may_fallthrough || else_flow.may_fallthrough, ..Default::default() };
            merge(&mut out, &then_flow);
            merge(&mut out, &else_flow);
            out
        }
        Ir::IfCase { arms, .. } => {
            if arms.is_empty() {
                return IrControlFlowSummary::default();
            }
            let mut out = IrControlFlowSummary { may_fallthrough: false, ..Default::default() };
            for arm in arms {
                let flow = sequence(&arm.body, &arm.cleanup_ir);
                out.may_fallthrough |= flow.may_fallthrough;
                merge(&mut out, &flow);
            }
            out
        }
        Ir::Region { body, .. } | Ir::Frame { body, .. } | Ir::Parallel { body, .. } | Ir::SpecFallback { body, .. } => summarize(body),
        Ir::Spawn { captured_env, body, .. } | Ir::Dispatch { captured_env, body, .. } => sequence(captured_env, body),
        Ir::SpecLoop { snapshot_ir, body_ir, validate_ir, commit_ir, .. } => {
            let mut out = sequence(snapshot_ir, body_ir);
            if out.may_fallthrough {
                let rest = sequence(validate_ir, commit_ir);
                merge(&mut out, &rest);
                out.may_fallthrough = rest.may_fallthrough;
            }
            out
        }
        Ir::RaceReturn { arms, .. } | Ir::RaceYield { arms, .. } => {
            let mut out = IrControlFlowSummary { may_fallthrough: false, ..Default::default() };
            for arm in arms {
                let flow = sequence(&arm.async_ir, &arm.handler_ir);
                out.may_fallthrough |= flow.may_fallthrough;
                merge(&mut out, &flow);
            }
            out
        }
        Ir::All { async_irs, .. } => sequence_items(async_irs),
        _ => IrControlFlowSummary::default(),
    }
}

pub fn ir_flow_may_fall_through(ir: &Option<IrPtr>) -> bool {
    summarize(ir).may_fallthrough
}

pub fn ir_flow_definitely_terminates(ir: &Option<IrPtr>) -> bool {
    !ir_flow_may_fall_through(ir)
}
