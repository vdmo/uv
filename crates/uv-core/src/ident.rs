use crate::keywords::is_keyword;
use crate::source_load::decode;
use crate::unicode::{is_ident_continue, is_ident_start};

pub fn is_identifier(ident: &str) -> bool {
    if ident.is_empty() {
        return false;
    }
    let decoded = decode(ident.as_bytes());
    if !decoded.ok || decoded.scalars.is_empty() {
        return false;
    }
    is_ident_start(decoded.scalars[0])
        && decoded.scalars[1..].iter().all(|&c| is_ident_continue(c))
}

pub fn is_name(ident: &str) -> bool {
    is_identifier(ident) && !is_keyword(ident)
}
