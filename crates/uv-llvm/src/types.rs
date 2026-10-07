//! Types and values.

use std::fmt;

/// An LLVM type. Pointers are opaque, as in LLVM 17 and later.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ty {
    Void,
    Int(u32),
    Half,
    Float,
    Double,
    Ptr,
    Array(u64, Box<Ty>),
    Struct { fields: Vec<Ty>, packed: bool },
    /// A named struct: `%name`; its body is declared in the module.
    Named(String),
    Func(Box<FnTy>),
    Vector(u64, Box<Ty>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnTy {
    pub ret: Ty,
    pub params: Vec<Ty>,
    pub vararg: bool,
}

impl Ty {
    pub fn i1() -> Ty {
        Ty::Int(1)
    }
    pub fn i8() -> Ty {
        Ty::Int(8)
    }
    pub fn i16() -> Ty {
        Ty::Int(16)
    }
    pub fn i32() -> Ty {
        Ty::Int(32)
    }
    pub fn i64() -> Ty {
        Ty::Int(64)
    }
    pub fn array(count: u64, element: Ty) -> Ty {
        Ty::Array(count, Box::new(element))
    }
    pub fn strukt(fields: Vec<Ty>) -> Ty {
        Ty::Struct { fields, packed: false }
    }
    pub fn func(ret: Ty, params: Vec<Ty>, vararg: bool) -> FnTy {
        FnTy { ret, params, vararg }
    }
    pub fn is_void(&self) -> bool {
        matches!(self, Ty::Void)
    }
    pub fn is_int(&self) -> bool {
        matches!(self, Ty::Int(_))
    }
    pub fn is_float(&self) -> bool {
        matches!(self, Ty::Half | Ty::Float | Ty::Double)
    }
    pub fn is_ptr(&self) -> bool {
        matches!(self, Ty::Ptr)
    }
    pub fn int_bits(&self) -> Option<u32> {
        match self {
            Ty::Int(bits) => Some(*bits),
            _ => None,
        }
    }
    /// `i1`... as a first-class value type: anything but `void` and a function.
    pub fn is_first_class(&self) -> bool {
        !matches!(self, Ty::Void | Ty::Func(_))
    }
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ty::Void => f.write_str("void"),
            Ty::Int(bits) => write!(f, "i{bits}"),
            Ty::Half => f.write_str("half"),
            Ty::Float => f.write_str("float"),
            Ty::Double => f.write_str("double"),
            Ty::Ptr => f.write_str("ptr"),
            Ty::Array(count, element) => write!(f, "[{count} x {element}]"),
            Ty::Vector(count, element) => write!(f, "<{count} x {element}>"),
            Ty::Struct { fields, packed } => {
                f.write_str(if *packed { "<{ " } else { "{ " })?;
                for (index, field) in fields.iter().enumerate() {
                    if index > 0 {
                        f.write_str(", ")?;
                    }
                    write!(f, "{field}")?;
                }
                f.write_str(if *packed { " }>" } else { " }" })
            }
            Ty::Named(name) => write!(f, "%{}", quote(name)),
            Ty::Func(ty) => {
                write!(f, "{} (", ty.ret)?;
                for (index, param) in ty.params.iter().enumerate() {
                    if index > 0 {
                        f.write_str(", ")?;
                    }
                    write!(f, "{param}")?;
                }
                if ty.vararg {
                    f.write_str(if ty.params.is_empty() { "..." } else { ", ..." })?;
                }
                f.write_str(")")
            }
        }
    }
}

/// An identifier as LLVM writes it: bare when it is plain, quoted otherwise.
pub(crate) fn quote(name: &str) -> String {
    let plain = !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '$' | '-')) && !name.starts_with(|c: char| c.is_ascii_digit());
    if plain {
        return name.to_string();
    }
    let mut out = String::from("\"");
    for byte in name.bytes() {
        match byte {
            b'"' | b'\\' => out.push_str(&format!("\\{byte:02X}")),
            0x20..=0x7e => out.push(byte as char),
            _ => out.push_str(&format!("\\{byte:02X}")),
        }
    }
    out.push('"');
    out
}

/// A value: its text in the IR (a register, a constant, a global) and its type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Value {
    pub text: String,
    pub ty: Ty,
}

impl Value {
    pub fn new(text: impl Into<String>, ty: Ty) -> Value {
        Value { text: text.into(), ty }
    }

    pub fn int(ty: Ty, value: i128) -> Value {
        Value { text: value.to_string(), ty }
    }

    /// An unsigned constant of `bits` bits written in its signed form, as LLVM prints it.
    pub fn uint(bits: u32, value: u128) -> Value {
        let signed = if bits >= 128 {
            value as i128
        } else {
            let mask = (1u128 << bits) - 1;
            let value = value & mask;
            if value >> (bits - 1) == 1 {
                (value as i128) - (1i128 << bits)
            } else {
                value as i128
            }
        };
        Value { text: signed.to_string(), ty: Ty::Int(bits) }
    }

    /// An integer constant of `bits` bits holding the low bits of `value`, written as LLVM does
    /// (`true` and `false` for `i1`, the signed decimal otherwise).
    pub fn const_int(bits: u32, value: u128) -> Value {
        if bits == 1 {
            return Value::bool(value & 1 == 1);
        }
        Value::uint(bits, value)
    }

    /// The bits of an integer constant, masked to its width.
    pub fn const_bits(&self) -> Option<u128> {
        let Ty::Int(bits) = self.ty else {
            return None;
        };
        let raw: i128 = match self.text.as_str() {
            "true" => 1,
            "false" => 0,
            text => text.parse().ok()?,
        };
        let mask = if bits >= 128 { u128::MAX } else { (1u128 << bits) - 1 };
        Some((raw as u128) & mask)
    }

    pub fn bool(value: bool) -> Value {
        Value { text: if value { "true" } else { "false" }.to_string(), ty: Ty::i1() }
    }

    pub fn null() -> Value {
        Value { text: "null".to_string(), ty: Ty::Ptr }
    }

    pub fn zero(ty: Ty) -> Value {
        let text = match &ty {
            Ty::Int(_) => "0".to_string(),
            Ty::Half | Ty::Float | Ty::Double => "0.0".to_string(),
            Ty::Ptr => "null".to_string(),
            _ => "zeroinitializer".to_string(),
        };
        Value { text, ty }
    }

    pub fn undef(ty: Ty) -> Value {
        Value { text: "undef".to_string(), ty }
    }

    pub fn poison(ty: Ty) -> Value {
        Value { text: "poison".to_string(), ty }
    }

    pub fn global(name: &str) -> Value {
        Value { text: format!("@{}", quote(name)), ty: Ty::Ptr }
    }

    /// A double constant written in the hexadecimal form LLVM requires for exact bits.
    pub fn double(value: f64) -> Value {
        Value { text: format!("0x{:016X}", value.to_bits()), ty: Ty::Double }
    }

    /// A float constant: LLVM writes the double that equals it.
    pub fn float(value: f32) -> Value {
        Value { text: format!("0x{:016X}", f64::from(value).to_bits()), ty: Ty::Float }
    }

    /// `ty text`, as an operand is written.
    pub fn typed(&self) -> String {
        format!("{} {}", self.ty, self.text)
    }
}

/// The part of an LLVM data layout string that decides sizes and alignments: the ABI and
/// preferred alignments of integers, floats and pointers, and the aggregate defaults. The
/// defaults are LLVM's own (`DataLayout::DefaultAlignments`), which a layout string overrides
/// only for what it names.
#[derive(Debug, Clone)]
pub struct DataLayout {
    /// `(bits, abi, preferred)` in bytes.
    ints: Vec<(u32, u64, u64)>,
    floats: Vec<(u32, u64, u64)>,
    pub ptr_bytes: u64,
    ptr_abi: u64,
    ptr_pref: u64,
    /// The preferred alignment of an aggregate: at least this.
    aggregate_pref: u64,
}

impl Default for DataLayout {
    fn default() -> Self {
        DataLayout {
            ints: vec![(1, 1, 1), (8, 1, 1), (16, 2, 2), (32, 4, 4), (64, 4, 8)],
            floats: vec![(16, 2, 2), (32, 4, 4), (64, 8, 8), (128, 16, 16)],
            ptr_bytes: 8,
            ptr_abi: 8,
            ptr_pref: 8,
            aggregate_pref: 8,
        }
    }
}

impl DataLayout {
    pub fn parse(spec: &str) -> DataLayout {
        let mut layout = DataLayout::default();
        for item in spec.split('-') {
            let mut parts = item.split(':');
            let head = parts.next().unwrap_or("");
            let numbers: Vec<u64> = parts.filter_map(|part| part.parse().ok()).collect();
            let (kind, bits) = match head.split_at(head.len().min(1)) {
                ("i", rest) => ('i', rest.parse::<u32>().ok()),
                ("f", rest) => ('f', rest.parse::<u32>().ok()),
                ("p", rest) if rest.is_empty() || rest == "0" => ('p', Some(0)),
                _ => continue,
            };
            let bytes = |bits: u64| bits / 8;
            match (kind, bits) {
                ('i', Some(bits)) => {
                    let abi = bytes(numbers.first().copied().unwrap_or(0));
                    let pref = numbers.get(1).map_or(abi, |pref| bytes(*pref));
                    layout.ints.retain(|(existing, ..)| *existing != bits);
                    layout.ints.push((bits, abi, pref));
                }
                ('f', Some(bits)) => {
                    let abi = bytes(numbers.first().copied().unwrap_or(0));
                    let pref = numbers.get(1).map_or(abi, |pref| bytes(*pref));
                    layout.floats.retain(|(existing, ..)| *existing != bits);
                    layout.floats.push((bits, abi, pref));
                }
                ('p', Some(_)) => {
                    // p:size:abi:pref
                    if let [size, abi, rest @ ..] = &numbers[..] {
                        layout.ptr_bytes = bytes(*size);
                        layout.ptr_abi = bytes(*abi);
                        layout.ptr_pref = rest.first().map_or(layout.ptr_abi, |pref| bytes(*pref));
                    }
                }
                _ => {}
            }
        }
        layout
    }

    /// The entry for an integer of `bits` bits: the exact size when the layout names it,
    /// else the next larger named one, else the largest.
    fn int_entry(&self, bits: u32) -> (u64, u64) {
        let mut best: Option<&(u32, u64, u64)> = None;
        for entry in &self.ints {
            if entry.0 >= bits && best.is_none_or(|current| entry.0 < current.0) {
                best = Some(entry);
            }
        }
        let entry = best.or_else(|| self.ints.iter().max_by_key(|entry| entry.0)).copied().unwrap_or((8, 1, 1));
        (entry.1, entry.2)
    }

    fn float_entry(&self, bits: u32) -> (u64, u64) {
        self.floats.iter().find(|entry| entry.0 == bits).map_or((bits as u64 / 8, bits as u64 / 8), |entry| (entry.1, entry.2))
    }

    /// The ABI alignment of a type, in bytes.
    pub fn abi_align(&self, ty: &Ty) -> u64 {
        self.align(ty, true)
    }

    /// The preferred alignment of a type, in bytes (what `alloca` uses).
    pub fn pref_align(&self, ty: &Ty) -> u64 {
        self.align(ty, false)
    }

    fn align(&self, ty: &Ty, abi: bool) -> u64 {
        match ty {
            Ty::Void | Ty::Func(_) => 1,
            Ty::Int(bits) => {
                let (a, p) = self.int_entry(*bits);
                if abi { a } else { p }
            }
            Ty::Half => {
                let (a, p) = self.float_entry(16);
                if abi { a } else { p }
            }
            Ty::Float => {
                let (a, p) = self.float_entry(32);
                if abi { a } else { p }
            }
            Ty::Double => {
                let (a, p) = self.float_entry(64);
                if abi { a } else { p }
            }
            Ty::Ptr => {
                if abi { self.ptr_abi } else { self.ptr_pref }
            }
            Ty::Array(_, element) => self.align(element, abi),
            Ty::Vector(count, element) => {
                let size = self.size_of(element) * count;
                size.next_power_of_two().max(1)
            }
            Ty::Struct { fields, packed } => {
                let natural = if *packed { 1 } else { fields.iter().map(|field| self.abi_align(field)).max().unwrap_or(1) };
                if abi { natural } else { natural.max(self.aggregate_pref) }
            }
            Ty::Named(_) => 1,
        }
    }

    /// `RequiredAllocaAlignment`: what the reference raises the alignment of every `alloca` to
    /// when it finishes a module.
    pub fn required_alloca_align(&self, ty: &Ty) -> u64 {
        let mut required = self.abi_align(ty);
        match ty {
            Ty::Ptr => required = required.max(8),
            Ty::Int(bits) => {
                let floor = match *bits {
                    128.. => 16,
                    64.. => 8,
                    32.. => 4,
                    16.. => 2,
                    _ => 1,
                };
                required = required.max(floor);
            }
            Ty::Double => required = required.max(8),
            Ty::Float => required = required.max(4),
            Ty::Half => required = required.max(2),
            _ => {}
        }
        match ty {
            Ty::Array(_, element) => required = required.max(self.required_alloca_align(element)),
            Ty::Struct { fields, .. } => {
                for field in fields {
                    required = required.max(self.required_alloca_align(field));
                }
            }
            _ => {}
        }
        required
    }

    /// The allocation size of a type: its store size rounded up to its ABI alignment.
    pub fn size_of(&self, ty: &Ty) -> u64 {
        match ty {
            Ty::Void | Ty::Func(_) => 0,
            Ty::Int(bits) => {
                let store = u64::from(*bits).div_ceil(8);
                round_up(store, self.abi_align(ty))
            }
            Ty::Half => 2,
            Ty::Float => 4,
            Ty::Double => 8,
            Ty::Ptr => self.ptr_bytes,
            Ty::Array(count, element) => count * self.size_of(element),
            Ty::Vector(count, element) => round_up(count * self.size_of(element), self.abi_align(ty)),
            Ty::Struct { fields, packed } => {
                let mut offset = 0;
                for field in fields {
                    if !*packed {
                        offset = round_up(offset, self.abi_align(field));
                    }
                    offset += self.size_of(field);
                }
                round_up(offset, self.abi_align(ty))
            }
            Ty::Named(_) => 0,
        }
    }

    /// The byte offsets of the fields of a struct type.
    pub fn struct_offsets(&self, fields: &[Ty], packed: bool) -> Vec<u64> {
        let mut offset = 0;
        let mut out = Vec::new();
        for field in fields {
            if !packed {
                offset = round_up(offset, self.abi_align(field));
            }
            out.push(offset);
            offset += self.size_of(field);
        }
        out
    }
}

fn round_up(value: u64, align: u64) -> u64 {
    if align <= 1 {
        value
    } else {
        value.div_ceil(align) * align
    }
}
