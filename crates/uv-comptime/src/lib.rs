//! Phase 2: compile-time execution. Evaluates `comptime` code, expands derives and
//! quoted syntax, and returns the modules that the semantic phases see.

mod derive;
pub mod eval;
mod files;
mod hygiene;
mod pass;
mod quote;
mod reflect;
mod rewrite;
mod util;
pub mod value;

pub use pass::{comptime_pass, execute_comptime, ComptimePassOptions, ComptimeResult};
