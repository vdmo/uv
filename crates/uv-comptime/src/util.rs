//! Small helpers shared by the compile-time stages.

/// `std::stoull(text, nullptr, 10)`: leading whitespace and a sign are accepted, parsing
/// stops at the first non-digit, and a missing number or an overflow is an error. A
/// leading `-` negates in unsigned arithmetic.
pub fn stoull(text: &str) -> Option<u64> {
    let bytes = text.as_bytes();
    let mut pos = 0;
    while pos < bytes.len() && matches!(bytes[pos], b' ' | b'\t' | b'\n' | 0x0B | 0x0C | b'\r') {
        pos += 1;
    }
    let negative = match bytes.get(pos) {
        Some(b'-') => {
            pos += 1;
            true
        }
        Some(b'+') => {
            pos += 1;
            false
        }
        _ => false,
    };
    let start = pos;
    let mut value: u64 = 0;
    while let Some(digit) = bytes.get(pos).filter(|b| b.is_ascii_digit()) {
        value = value.checked_mul(10)?.checked_add(u64::from(digit - b'0'))?;
        pos += 1;
    }
    if pos == start {
        return None;
    }
    Some(if negative { value.wrapping_neg() } else { value })
}
