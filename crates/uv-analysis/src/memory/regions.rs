//! Where a value's storage comes from.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProvenanceKind {
    Global,
    Stack,
    Heap,
    Region,
    Bottom,
    Param,
}
