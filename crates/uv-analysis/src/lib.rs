//! Semantic analysis. Ported so far: the FFI surface collection phase 1 reports on and the
//! scope layer of name resolution; the resolver, typing and the remaining analyses follow.

pub mod caps;
pub mod context;
pub mod ffi;
pub mod modal;
pub mod resolve;
