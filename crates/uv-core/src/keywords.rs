/// Reserved keywords. The reference table has 55 slots with 52 named entries; the
/// remaining slots are empty strings, so the empty string is a member of the set.
pub const ULTRAVIOLET_KEYWORDS: [&str; 53] = [
    "all", "as", "break", "class", "comptime", "continue", "copy", "derive", "dispatch", "else",
    "enum", "extern", "false", "defer", "frame", "from", "if", "is", "imm", "import", "internal",
    "let", "loop", "modal", "move", "mut", "null", "parallel", "private", "procedure", "public",
    "quote", "race", "record", "region", "return", "shared", "spawn", "sync", "transition",
    "transmute", "true", "type", "unique", "unsafe", "var", "where", "widen", "using", "yield",
    "const", "override", "",
];

pub const ULTRAVIOLET_FIXED_IDENTIFIERS: [&str; 22] = [
    "read", "write", "dynamic", "speculative", "release", "cancel", "name", "workgroup",
    "workgroups", "affinity", "priority", "reduce", "ordered", "chunk", "min", "max", "and", "or",
    "pattern", "target", "requires", "emits",
];

pub const ULTRAVIOLET_OPERATORS: [&str; 47] = [
    "+", "-", "*", "/", "%", "**", "==", "!=", "<", "<=", ">", ">=", "&&", "||", "!", "&", "|",
    "^", "<<", ">>", "=", "+=", "-=", "*=", "/=", "%=", "&=", "|:", "|=", "^=", "<<=", ">>=", ":=",
    "<:", "..", "..=", "=>", "->", "::", "~", "~>", "~!", "~%", "?", "#", "@", "$",
];

pub const ULTRAVIOLET_PUNCTUATORS: [&str; 10] = ["(", ")", "[", "]", "{", "}", ",", ":", ";", "."];

/// The reference table has 23 slots with 22 named entries (empty string included).
pub const UNSUPPORTED_LEXEMES: [&str; 23] = [
    "attribute", "opaque_type", "refinement_type", "closure", "pipeline", "async",
    "metaprogramming", "Network", "Reactor", "GPUFactory", "CPUFactory", "AsyncRuntime",
    "key_system", "extern_block", "foreign_decl", "class_generics", "associated_type",
    "modal_class", "class_contract", "type_item", "abstract_state", "override_in_class", "",
];

pub fn is_keyword(s: &str) -> bool {
    ULTRAVIOLET_KEYWORDS.contains(&s)
}

pub fn is_fixed_identifier(s: &str) -> bool {
    ULTRAVIOLET_FIXED_IDENTIFIERS.contains(&s)
}

pub fn is_unsupported_lexeme(s: &str) -> bool {
    UNSUPPORTED_LEXEMES.contains(&s)
}
