//! A string-keyed map that iterates in the order libstdc++'s `std::unordered_map` does.
//!
//! The reference compiler iterates hash maps in a few places where the order shows in
//! its output: the spelling suggestion for an unresolved name takes the first of several
//! equally close names, and a wildcard `using` binds names in the order of the map it
//! reads. That order is a property of the standard library the reference is built with,
//! so the port reproduces the library's table: the hash function, the bucket counts it
//! grows through, and where a new element is linked into the element list.
//!
//! `tools/oracle/unordered_probe.cpp` prints the orders this is tested against.

use std::collections::HashMap;

/// The bucket counts libstdc++ grows through, from `__prime_list`.
const PRIMES: [usize; 120] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 103, 109, 113,
    127, 137, 139, 149, 157, 167, 179, 193, 199, 211, 227, 241, 257, 277, 293, 313, 337, 359, 383, 409, 439, 467,
    503, 541, 577, 619, 661, 709, 761, 823, 887, 953, 1031, 1109, 1193, 1289, 1381, 1493, 1613, 1741, 1879, 2029,
    2179, 2357, 2549, 2753, 2971, 3209, 3469, 3739, 4027, 4349, 4703, 5087, 5503, 5953, 6427, 6949, 7517, 8123,
    8783, 9497, 10273, 11113, 12011, 12983, 14033, 15173, 16411, 17749, 19183, 20753, 22447, 24281, 26267, 28411,
    30727, 33223, 35933, 38873, 42043, 45481, 49201, 53201, 57557, 62233, 67307, 72817, 78779, 85229, 92203, 99733,
];

/// Small requests are answered from a table instead of a search.
const FAST_BUCKETS: [usize; 14] = [2, 2, 2, 3, 5, 5, 7, 7, 11, 11, 11, 11, 13, 13];

/// `std::hash<std::string>`: the 64-bit Murmur variant in libstdc++'s `_Hash_bytes`.
pub fn hash_bytes(bytes: &[u8]) -> u64 {
    const MUL: u64 = (0xc6a4_a793 << 32) + 0x5bd1_e995;
    const SEED: u64 = 0xc70f_6907;
    let shift_mix = |v: u64| v ^ (v >> 47);
    let mut hash = SEED ^ (bytes.len() as u64).wrapping_mul(MUL);
    let mut chunks = bytes.chunks_exact(8);
    for chunk in &mut chunks {
        let word = u64::from_le_bytes(chunk.try_into().expect("chunk of eight bytes"));
        hash ^= shift_mix(word.wrapping_mul(MUL)).wrapping_mul(MUL);
        hash = hash.wrapping_mul(MUL);
    }
    let tail = chunks.remainder();
    if !tail.is_empty() {
        let word = tail.iter().rev().fold(0u64, |acc, byte| (acc << 8) + u64::from(*byte));
        hash ^= word;
        hash = hash.wrapping_mul(MUL);
    }
    hash = shift_mix(hash).wrapping_mul(MUL);
    shift_mix(hash)
}

#[derive(Debug, Clone)]
pub struct UnorderedMap<V> {
    values: HashMap<String, V>,
    /// The element list: hash and key, in iteration order. Elements of one bucket are
    /// adjacent.
    order: Vec<(u64, String)>,
    bucket_count: usize,
    /// The element count above which the table considers growing.
    next_resize: usize,
}

impl<V> Default for UnorderedMap<V> {
    fn default() -> Self {
        UnorderedMap { values: HashMap::new(), order: Vec::new(), bucket_count: 1, next_resize: 0 }
    }
}

/// Links an element in front of the others in its bucket, or in front of the whole list
/// when its bucket is empty.
fn link(order: &mut Vec<(u64, String)>, bucket_count: usize, hash: u64, key: String) {
    let bucket = hash % bucket_count as u64;
    let at = order.iter().position(|(other, _)| other % bucket_count as u64 == bucket).unwrap_or(0);
    order.insert(at, (hash, key));
}

impl<V> UnorderedMap<V> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.order.len()
    }

    pub fn is_empty(&self) -> bool {
        self.order.is_empty()
    }

    pub fn bucket_count(&self) -> usize {
        self.bucket_count
    }

    pub fn get(&self, key: &str) -> Option<&V> {
        self.values.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut V> {
        self.values.get_mut(key)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }

    /// The smallest bucket count the library uses that is at least `wanted`.
    fn next_bucket_count(&mut self, wanted: usize) -> usize {
        if wanted < FAST_BUCKETS.len() {
            if wanted == 0 {
                return 1;
            }
            self.next_resize = FAST_BUCKETS[wanted];
            return FAST_BUCKETS[wanted];
        }
        match PRIMES[6..].iter().find(|prime| **prime >= wanted) {
            Some(prime) => {
                self.next_resize = *prime;
                *prime
            }
            None => panic!("scope of {wanted} names exceeds the bucket counts the port carries"),
        }
    }

    /// `reserve(n)`: room for `n` elements, with the bucket count the library picks for it.
    pub fn reserve(&mut self, n: usize) {
        let buckets = self.next_bucket_count(n.max(self.len() + 1));
        if buckets != self.bucket_count {
            self.rehash(buckets);
        }
    }

    fn rehash(&mut self, bucket_count: usize) {
        self.bucket_count = bucket_count;
        for (hash, key) in std::mem::take(&mut self.order) {
            link(&mut self.order, bucket_count, hash, key);
        }
    }

    fn insert_new(&mut self, key: String, value: V) {
        let wanted = self.len() + 1;
        if wanted > self.next_resize {
            // A table that has never held anything starts at eleven buckets or more.
            let min_buckets = wanted.max(if self.next_resize == 0 { 11 } else { 0 });
            if min_buckets >= self.bucket_count {
                let grown = self.next_bucket_count((min_buckets + 1).max(self.bucket_count * 2));
                self.rehash(grown);
            } else {
                self.next_resize = self.bucket_count;
            }
        }
        link(&mut self.order, self.bucket_count, hash_bytes(key.as_bytes()), key.clone());
        self.values.insert(key, value);
    }

    /// Inserts unless the key is present (`emplace`). Returns whether it inserted.
    pub fn emplace(&mut self, key: String, value: V) -> bool {
        if self.values.contains_key(&key) {
            return false;
        }
        self.insert_new(key, value);
        true
    }

    /// Inserts or replaces the value (`map[key] = value`). A replaced key keeps its place.
    pub fn insert(&mut self, key: String, value: V) {
        match self.values.get_mut(&key) {
            Some(slot) => *slot = value,
            None => self.insert_new(key, value),
        }
    }

    pub fn entry(&mut self, key: String) -> Entry<'_, V> {
        Entry { map: self, key }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &V)> {
        self.order.iter().map(|(_, key)| (key, &self.values[key]))
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.order.iter().map(|(_, key)| key)
    }

    /// What `target = source` leaves in `target`: the source's elements, order and
    /// bucket count.
    pub fn assign_from(&mut self, source: &Self)
    where
        V: Clone,
    {
        *self = source.clone();
    }
}

/// The `entry(..).or_insert(..)` idiom, with `emplace` semantics.
pub struct Entry<'m, V> {
    map: &'m mut UnorderedMap<V>,
    key: String,
}

impl<V> Entry<'_, V> {
    pub fn or_insert(self, value: V) {
        self.map.emplace(self.key, value);
    }

    pub fn or_insert_with(self, make: impl FnOnce() -> V) {
        if !self.map.contains_key(&self.key) {
            self.map.insert_new(self.key, make());
        }
    }
}

impl<V> FromIterator<(String, V)> for UnorderedMap<V> {
    fn from_iter<I: IntoIterator<Item = (String, V)>>(iter: I) -> Self {
        let mut map = UnorderedMap::new();
        for (key, value) in iter {
            map.emplace(key, value);
        }
        map
    }
}

impl<'m, V> IntoIterator for &'m UnorderedMap<V> {
    type Item = (&'m String, &'m V);
    type IntoIter = Box<dyn Iterator<Item = (&'m String, &'m V)> + 'm>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROBE: &str = include_str!("../../../tests/golden/unordered_order.txt");

    /// The key generator of the probe.
    fn key(i: usize) -> String {
        let alphabet = "abcdefghijklmnopqrstuvwxyz0123456789_ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mixed = (i as u64 * 2_654_435_761) % 1_000_003;
        let len = if (i % 24) + 1 > 3 { (i % 24) - 2 } else { 0 };
        let start = i % 7;
        let end = (start + len).min(alphabet.len());
        format!("{}{mixed}", &alphabet[start..end])
    }

    fn filled(range: std::ops::Range<usize>) -> UnorderedMap<i32> {
        let mut map = UnorderedMap::new();
        for i in range {
            map.emplace(key(i), 0);
        }
        map
    }

    fn line(tag: &str, id: usize, map: &UnorderedMap<i32>) -> String {
        let mut out = format!("{tag} {id} {}", map.bucket_count());
        for name in map.keys() {
            out.push(' ');
            out.push_str(name);
        }
        out
    }

    fn probe_line(tag: &str, id: usize) -> &'static str {
        let prefix = format!("{tag} {id} ");
        PROBE.lines().find(|line| line.starts_with(&prefix)).unwrap_or_else(|| panic!("no probe line {prefix}"))
    }

    #[test]
    fn primes_match_the_library() {
        let listed: Vec<usize> =
            PROBE.lines().next().unwrap().split(' ').skip(1).map(|prime| prime.parse().unwrap()).collect();
        assert_eq!(listed, PRIMES);
    }

    #[test]
    fn hashes_match_the_library() {
        let mut checked = 0;
        for probe in PROBE.lines().filter(|line| line.starts_with("H ")) {
            let fields: Vec<&str> = probe.split(' ').collect();
            let text = if fields[1] == "-" { "" } else { fields[1] };
            assert_eq!(hash_bytes(text.as_bytes()).to_string(), fields[2], "hash of {text:?}");
            checked += 1;
        }
        assert_eq!(checked, 41);
    }

    #[test]
    fn insertion_order_matches_the_library() {
        for probe in PROBE.lines().filter(|line| line.starts_with("N ")) {
            let n: usize = probe.split(' ').nth(1).unwrap().parse().unwrap();
            assert_eq!(line("N", n, &filled(0..n)), probe, "{n} elements");
        }
    }

    #[test]
    fn existing_keys_keep_their_place() {
        let mut map = filled(0..20);
        for i in (0..20).step_by(3) {
            map.emplace(key(i), 1);
        }
        for i in 15..25 {
            map.insert(key(i), 2);
        }
        assert_eq!(line("R", 0, &map), probe_line("R", 0));
    }

    #[test]
    fn copies_grow_like_the_original() {
        for n in [3, 10, 13, 14, 29, 30, 59, 100] {
            let base = filled(0..n);
            let mut copy = base.clone();
            assert_eq!(line("C", n, &copy), probe_line("C", n));
            for i in 0..9 {
                copy.emplace(key(1000 + i), 0);
            }
            assert_eq!(line("D", n, &copy), probe_line("D", n));
            for (tag_assigned, tag_grown, start, count, extra) in [("A", "B", 2000, 5, 3000), ("G", "I", 4000, 200, 5000)] {
                let mut target = filled(start..start + count);
                target.assign_from(&base);
                assert_eq!(line(tag_assigned, n, &target), probe_line(tag_assigned, n));
                for i in 0..9 {
                    target.emplace(key(extra + i), 0);
                }
                assert_eq!(line(tag_grown, n, &target), probe_line(tag_grown, n));
            }
        }
    }
}
