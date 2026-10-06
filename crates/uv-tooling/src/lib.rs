//! What the language server needs besides the compiler's phases: paths and URIs, line
//! and column conversion, the open documents, and one analysis of the workspace.

pub mod analysis;
pub mod document_store;
pub mod line_index;
pub mod uri;
