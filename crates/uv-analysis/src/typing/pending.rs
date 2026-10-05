//! Scaffolding for porting the typer piece by piece. A construct whose typing is not
//! ported records its name here and fails; the parity dump then reports the whole body
//! as pending instead of printing a result that would not mean anything. This module
//! goes away when nothing calls `pending` any more.

use std::cell::Cell;

thread_local! {
    static PENDING: Cell<Option<&'static str>> = const { Cell::new(None) };
}

/// Records the first unported construct reached.
pub fn pending(what: &'static str) {
    PENDING.with(|cell| {
        if cell.get().is_none() {
            cell.set(Some(what));
        }
    });
}

/// What was reached since the last call, if anything.
pub fn take_pending() -> Option<&'static str> {
    PENDING.with(Cell::take)
}

thread_local! {
    static PROOF_CONTEXT_INCOMPLETE: Cell<bool> = const { Cell::new(false) };
}

/// Starts a body: nothing pending, the proof context complete.
pub fn reset_scaffolding() {
    PENDING.with(|cell| cell.set(None));
    PROOF_CONTEXT_INCOMPLETE.with(|cell| cell.set(false));
}

/// A statement whose proof facts are not tracked yet has been typed.
pub fn mark_proof_context_incomplete() {
    PROOF_CONTEXT_INCOMPLETE.with(|cell| cell.set(true));
}

/// To be called wherever typing consults the proof context: if facts may be missing
/// from it, the answer would not be the reference's, and the body becomes pending.
pub fn consult_proof_context() {
    if PROOF_CONTEXT_INCOMPLETE.with(Cell::get) {
        pending("ProofContext");
    }
}
