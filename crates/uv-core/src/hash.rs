pub const FNV_OFFSET_64: u64 = 14695981039346656037;
pub const FNV_PRIME_64: u64 = 1099511628211;

pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = FNV_OFFSET_64;
    for &byte in bytes {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME_64);
    }
    hash
}

pub fn hex2(byte: u8) -> String {
    format!("{byte:02X}")
}

pub fn hex64(hash: u64) -> String {
    format!("{hash:016X}")
}
