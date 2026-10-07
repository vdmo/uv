//! A textual LLVM IR builder.
//!
//! The reference compiler emits code through the LLVM C++ API. This crate offers the part
//! of that API the emitter uses, as a builder that writes textual IR (`.ll`), which
//! `llvm-as` assembles and `ld.lld` links (see PLAN.md, "Backend decision").

mod builder;
mod module;
mod types;

pub use builder::{BlockId, Builder, FuncId};
pub use module::{CallConv, FuncAttr, GlobalId, Linkage, Module, ParamAttr};
pub use types::{format_fp, DataLayout, FnTy, Ty, Value};

#[cfg(test)]
mod tests {
    use super::*;

    /// A function that adds, branches and merges with a phi.
    fn sample() -> String {
        let module = Module::new("sample", "x86_64-unknown-linux-gnu", "e-m:e-p270:32:32-p271:32:32-p272:64:64-i128:128-n8:16:32:64-S128");
        let mut b = Builder::new(module);
        let f = b.function("pick", Ty::func(Ty::i32(), vec![Ty::i32(), Ty::i1()], false), Linkage::External);
        let entry = b.block(f, "entry");
        let yes = b.block(f, "yes");
        let no = b.block(f, "no");
        let join = b.block(f, "join");
        b.set_insert_point(entry);
        let slot = b.alloca(&Ty::i32(), "slot");
        let x = b.param(f, 0);
        b.store(&x, &slot);
        let flag = b.param(f, 1);
        b.cond_br(&flag, yes, no);
        b.set_insert_point(yes);
        let loaded = b.load(&Ty::i32(), &slot, "loaded");
        let sum = b.binary("add", &loaded, &Value::int(Ty::i32(), 1), "sum");
        b.br(join);
        b.set_insert_point(no);
        b.br(join);
        b.set_insert_point(join);
        let (phi, merged) = b.phi(&Ty::i32(), "merged");
        b.add_incoming(phi, &sum, yes);
        b.add_incoming(phi, &Value::int(Ty::i32(), 0), no);
        b.ret(&merged);
        b.finish()
    }

    #[test]
    fn prints_a_function() {
        let text = sample();
        if let Ok(path) = std::env::var("UV_LLVM_SAMPLE_OUT") {
            std::fs::write(path, &text).unwrap();
        }
        assert!(text.contains("define i32 @pick(i32 %0, i1 %1) {"));
        assert!(text.contains("%slot.0 = alloca i32, align 4"));
        assert!(text.contains("= phi i32 [ %sum.2, %yes ], [ 0, %no ]"), "{text}");
        assert!(text.contains("ret i32 %merged.3"));
    }
}
