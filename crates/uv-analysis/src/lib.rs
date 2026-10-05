//! Semantic analysis. Ported so far: the FFI surface collection phase 1 reports on, name
//! resolution, and the core of the type system (semantic types, lowering, equivalence,
//! variance, substitution). Expression typing and the remaining analyses follow.

pub mod caps;
pub mod context;
pub mod composite;
pub mod contracts;
pub mod ffi;
pub mod generics;
pub mod layout;
pub mod memory;
pub mod modal;
pub mod resolve;
pub mod typing;
