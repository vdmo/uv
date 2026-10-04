pub fn strip_int_suffix(lexeme: &str) -> &str {
    const INT_SUFFIXES: [&str; 12] = [
        "i128", "u128", "isize", "usize", "i64", "u64", "i32", "u32", "i16", "u16", "i8", "u8",
    ];
    for suffix in INT_SUFFIXES {
        if lexeme.len() <= suffix.len() {
            continue;
        }
        if let Some(core) = lexeme.strip_suffix(suffix) {
            return core;
        }
    }
    lexeme
}

pub fn parse_int_core(core: &str) -> Option<u128> {
    if core.is_empty() {
        return None;
    }
    let (base, digits) = if let Some(rest) = core.strip_prefix("0x").or(core.strip_prefix("0X")) {
        (16u32, rest)
    } else if let Some(rest) = core.strip_prefix("0o").or(core.strip_prefix("0O")) {
        (8, rest)
    } else if let Some(rest) = core.strip_prefix("0b").or(core.strip_prefix("0B")) {
        (2, rest)
    } else {
        (10, core)
    };
    if digits.is_empty() {
        return None;
    }
    let mut value: u128 = 0;
    let mut saw_digit = false;
    for c in digits.chars() {
        if c == '_' {
            continue;
        }
        let digit = c.to_digit(16).filter(|_| base > 10 || c.is_ascii_digit())?;
        if digit >= base {
            return None;
        }
        saw_digit = true;
        value = value.checked_mul(base as u128)?.checked_add(digit as u128)?;
    }
    saw_digit.then_some(value)
}

pub fn parse_unsigned_int_literal(lexeme: &str) -> Option<u64> {
    if lexeme.is_empty() || lexeme.starts_with('-') {
        return None;
    }
    u64::try_from(parse_int_core(strip_int_suffix(lexeme))?).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_cores() {
        assert_eq!(parse_int_core("0x_ff"), Some(255));
        assert_eq!(parse_int_core("1_000"), Some(1000));
        assert_eq!(parse_int_core("0b102"), None);
        assert_eq!(parse_int_core("12a"), None);
        assert_eq!(parse_int_core("340282366920938463463374607431768211455"), Some(u128::MAX));
        assert_eq!(parse_int_core("340282366920938463463374607431768211456"), None);
        assert_eq!(parse_unsigned_int_literal("42u8"), Some(42));
        assert_eq!(parse_unsigned_int_literal("18446744073709551616"), None);
    }
}
