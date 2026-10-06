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

/// Starts a body: nothing pending.
pub fn reset_scaffolding() {
    PENDING.with(|cell| cell.set(None));
}
