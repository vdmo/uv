#!/usr/bin/env python3
"""Generate type-core edge-case projects under target/typecases and their list.

Each case declares types whose lowering, printing, ordering, equivalence, lookup,
variance or instantiation is worth pinning: unions in every member order, array lengths
that need evaluation, refinements, generic defaults, the asynchronous aliases.
"""
from pathlib import Path
import shutil

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "target" / "typecases"

APP = '[[assembly]]\nname = "App"\nkind = "executable"\nroot = "Source"\n'
LIB = '[[assembly]]\nname = "Lib"\nkind = "library"\nroot = "Lib"\n'
MAIN = "public procedure main() -> i32 {\n    return 0\n}\n"

BASE = """internal record Point {
    public x: i32
    public y: i32
}

internal enum Shape {
    Empty
    Circle(i32)
}

internal modal Door {
    @Open {
        public width: i32
    }
    @Closed {}
}

internal class Named {
    procedure name(~) -> i32
}

"""


def aliases(*types: str, prefix: str = "") -> str:
    """One alias per type, so that each is lowered and compared with the others."""
    lines = "".join(f"internal type T{index} = {ty}\n" for index, ty in enumerate(types))
    return BASE + prefix + lines + MAIN


CASES = {
    "prims": aliases("i8", "i16", "i32", "i64", "i128", "u8", "u16", "u32", "u64", "u128", "f16", "f32", "f64", "bool",
                     "char", "usize", "isize", "()", "!"),
    "strings": aliases("string", "string@View", "string@Managed", "bytes", "bytes@View", "bytes@Managed"),
    "perms": aliases("const i32", "unique i32", "shared i32", "unique Point", "shared Point",
                     "unique [i32]", "const string@View"),
    "tuples": aliases("()", "(i32;)", "(i32, bool)", "(i32, (bool, char))", "((i32;);)", "(bool, i32)", "(i32, bool)"),
    "arrays_slices": aliases("[i32; 4]", "[i32; 4usize]", "[i32; 0x4]", "[i32; 5]", "[bool; 4]", "[[i32; 2]; 3]", "[i32]",
                             "[[i32]]", "[(i32, bool); 2]", "[i32; 0]", "[i32; 1_000]", "[i32; 0b100]", "[i32; 0o4]"),
    "array_len_static": aliases("[i32; SIZE]", "[i32; OTHER]", "[i32; 8]", "[i32; CHAINED]", "[u8; SIZE]",
                                prefix="internal let SIZE: usize = 8\ninternal let OTHER: usize = 8usize\ninternal let CHAINED: usize = SIZE\n"),
    "array_len_expr": aliases("[i32; 2 + 2]", "[i32; SIZE * 2]", "[i32; 4]", "[i32; (1 + 1) * 2]", "[i32; twice(2usize)]",
                              "[i32; 16 / 4]", "[i32; 9 - 5]",
                              prefix="internal let SIZE: usize = 2\ncomptime internal procedure twice(value: usize) -> usize {\n    return value + value\n}\n"),
    "array_len_bad": aliases("[i32; true]", "[i32; 1.5]", "[i32; \"4\"]", "[i32; NOT_CONST]",
                             "[i32; 340282366920938463463374607431768211455]", "[i32; 18446744073709551615]",
                             "[i32; 18446744073709551616]", "[i32; -1]", "[i32; FLAG]",
                             prefix="internal var NOT_CONST: usize = 4\ninternal let FLAG: bool = true\n"),
    "array_len_self_ref": aliases("[i32; LOOP]", "[i32; PING]", prefix="internal let LOOP: usize = LOOP\ninternal let PING: usize = PONG\ninternal let PONG: usize = PING\n"),
    "unions_order": aliases("i32 | bool", "bool | i32", "i32 | bool | char", "char | bool | i32", "i32 | i32",
                            "i32 | bool | i32", "Point | i32",
                            "i32 | Point", "Point | Shape", "Shape | Point", "string | bytes", "string@View | string",
                            "string | string@Managed | string@View", "[i32] | [i32; 2]", "() | i32", "i32 | ()",
                            "(i32, bool) | (bool, i32)", "! | i32"),
    "unions_paths_case": aliases("alpha | Beta | ALPHA | beta", "Beta | alpha | beta | ALPHA", "Zed | alpha",
                                 prefix="internal record alpha {\n    public v: i32\n}\ninternal record ALPHA {\n    public v: i32\n}\ninternal record Beta {\n    public v: i32\n}\ninternal record beta {\n    public v: i32\n}\ninternal record Zed {\n    public v: i32\n}\n"),
    "unions_nested_kinds": aliases("Ptr<i32> | *mut i32 | *imm i32 | [i32] | (i32;) | () -> i32 | $Named | Door@Open | Door",
                                   "Door | Door@Open | $Named | () -> i32 | (i32;) | [i32] | *imm i32 | *mut i32 | Ptr<i32>",
                                   "Ptr<i32>@Valid | Ptr<i32>@Null | Ptr<i32>@Expired | Ptr<i32>"),
    "pointers": aliases("Ptr<i32>", "Ptr<i32>@Valid", "Ptr<i32>@Null", "Ptr<i32>@Expired", "*imm i32", "*mut i32",
                        "*mut *imm u8", "Ptr<Ptr<i32>>", "Ptr<Point>", "Ptr<[i32; 2]>"),
    "functions": aliases("() -> i32", "(i32) -> i32", "(i32, bool) -> ()", "(move i32) -> i32", "(i32) -> (bool) -> i32",
                         "((i32) -> i32) -> i32", "() -> ()", "(Point) -> Shape", "() -> !"),
    "closures": aliases("|| -> i32", "|i32| -> i32", "|i32, bool| -> ()", "|move i32| -> i32",
                        "|| -> i32 [shared: { value: shared i32 }]", "|| -> i32 [shared: { value: shared i32, other: shared bool }]",
                        "|| -> i32 [shared: { }]", "|| -> || -> i32"),
    "dynamic_opaque": aliases("$Named", "$App::Named", "$IO", "$Missing", "opaque Named", "opaque Named", "$Drop"),
    "modal_states": aliases("Door", "Door@Open", "Door@Closed", "App::Door@Open", "Door@Missing", "Cell<i32>@Full",
                            "Cell<bool>@Full", "Cell<i32>@Empty", "Cell<i32>",
                            prefix="internal modal Cell<TValue> {\n    @Full {\n        public value: TValue\n    }\n    @Empty {}\n}\n"),
    "ranges": aliases("Range<i32>", "RangeInclusive<i32>", "RangeFrom<i32>", "RangeTo<i32>", "RangeToInclusive<i32>",
                      "RangeFull", "Range<usize>", "RangeFrom<usize>", "unique Range<usize>", "Range<u8> | Range<usize>"),
    "refinements": aliases("i32 |: { self > 0 }", "i32 |: { self > 0 }", "i32 |: { self > 1 }", "i32 |: { self > 0 && self < 9 }",
                           "i32 |: { self < 9 && self > 0 }", "u8 |: { self > 0 }",
                           "i32 |: { true }", "Range<usize> |: { true }", "i32 |: { self > LIMIT }", "i32 |: { self > App::LIMIT }",
                           prefix="internal let LIMIT: i32 = 3\n"),
    "paths": aliases("Point", "App::Point", "Shape", "Count", "App::Count", "Wrapper<i32>", "Wrapper<bool>", "Wrapper<Wrapper<i32>>",
                     "App::Wrapper<i32>", "Pair<i32, bool>", "Pair<i32>",
                     prefix="internal type Count = i32\ninternal record Wrapper<TValue> {\n    public value: TValue\n}\ninternal record Pair<TFirst; TSecond = bool> {\n    public first: TFirst\n    public second: TSecond\n}\n"),
    "builtin_paths": aliases("Context", "File", "File@Read", "DirIter@Open", "IoError", "Outcome<i32, IoError>", "Duration",
                             "Region", "Region@Active", "RegionOptions", "CancelToken@Active", "Spawned<i32>@Ready",
                             "Tracked<i32, bool>", "Priority", "CpuSet", "AllocationError", "TimeError", "FileKind", "DirEntry"),
    "async_builtin": aliases("Async<i32>", "Async<i32, bool>", "Async<i32, bool, char>", "Async<i32, bool, char, u8>",
                             "Future<i32>", "Future<i32, bool>", "Sequence<i32>", "Stream<i32, bool>", "Pipe<i32, bool>",
                             "Exchange<i32>", "Async<i32>@Suspended", "Async<i32, bool, char, u8>@Completed",
                             "Async<i32>@Failed", "unique Future<i32>", "Future<Future<i32>>", "Sequence<(i32, bool)>"),
    "async_bad_arity": aliases("Sequence<i32, bool>", "Stream<i32>", "Pipe<i32>", "Exchange<i32, bool>", "Future<i32, bool, char>",
                               "Async<i32, bool, char, u8, u16>"),
    "async_user_alias": aliases("Task", "Job<i32>", "Job<bool>", "Indirect", "Twice<i32>", "NotAsync", "Mixed<i32, bool>",
                                "Defaulted", "Job<i32, bool>",
                                prefix="internal type Task = Future<i32>\ninternal type Job<TValue> = Future<TValue, bool>\ninternal type Indirect = Task\ninternal type Twice<TValue> = Job<TValue>\ninternal type NotAsync = (i32, bool)\ninternal type Mixed<TFirst; TSecond> = Stream<TSecond, TFirst>\ninternal type Defaulted = Job<u8>\n"),
    "generic_defaults": BASE + "internal record One<TValue = i32> {\n    public value: TValue\n}\ninternal record Two<TFirst; TSecond = TFirst> {\n    public first: TFirst\n    public second: TSecond\n}\ninternal record Three<TFirst; TSecond = (TFirst, bool); TThird = [TSecond]> {\n    public first: TFirst\n    public second: TSecond\n    public third: TThird\n}\ninternal record Shapes<TValue = Ptr<i32>@Valid; TOther = *mut u8; TLast = string@View> {\n    public value: TValue\n    public other: TOther\n    public last: TLast\n}\ninternal record Odd<TValue = [i32; 4]; TOther = Wrapper<bool>; TFunc = (i32) -> bool; TDyn = $Named> {\n    public value: TValue\n    public other: TOther\n    public func: TFunc\n    public dynamic: TDyn\n}\ninternal record Wrapper<TValue> {\n    public value: TValue\n}\ninternal record Ranged<TValue = Range<i32>; TUnion = i32 | bool; TModal = Door@Open; TRefined = i32 |: { self > 0 }> {\n    public value: TValue\n    public union: TUnion\n    public state_value: TModal\n    public refined: TRefined\n}\n" + MAIN,
    "generic_instantiation": BASE + "internal record Holder<TFirst; TSecond> {\n    public plain: TFirst\n    public pair: (TFirst, TSecond)\n    public array: [TFirst; 3]\n    public slice: [TSecond]\n    public pointer: Ptr<TFirst>@Valid\n    public raw: *mut TSecond\n    public union: TFirst | TSecond\n    public union_same: TFirst | i32\n    public func: (TFirst) -> TSecond\n    public closure: |TFirst| -> TSecond\n    public nested: (Holder<TSecond, TFirst>, i32)\n    public perm: unique TFirst\n    public refined: TFirst |: { true }\n    public range: (Range<TFirst>, i32)\n    public qualified: [App::Holder<TFirst, TFirst>]\n    public other: Point\n}\n" + MAIN,
    "generic_alias_instantiation": aliases("Same<i32>", "Swap<i32, bool>", "Nest<i32>", "Fixed<i32>", "Collapse<i32>",
                                           prefix="internal type Same<TValue> = TValue\ninternal type Swap<TFirst; TSecond> = (TSecond, TFirst)\ninternal type Nest<TValue> = [Same<TValue>]\ninternal type Fixed<TValue> = i32\ninternal type Collapse<TValue> = TValue | i32 | bool\n"),
    "variance": BASE + "internal record Out<TValue> {\n    public value: TValue\n}\ninternal record In<TValue> {\n    public take: (TValue) -> i32\n}\ninternal record Both<TValue> {\n    public value: TValue\n    public take: (TValue) -> i32\n}\ninternal record Unused<TValue> {\n    public value: i32\n}\ninternal record Mutable<TValue> {\n    public value: unique TValue\n    public array: [TValue; 2]\n}\ninternal record ConstOnly<TValue> {\n    public value: const TValue\n}\ninternal record Sliced<TValue> {\n    public value: [TValue]\n}\ninternal record Pointed<TValue> {\n    public value: Ptr<TValue>@Valid\n}\ninternal record Returned<TValue> {\n    public make: () -> TValue\n}\ninternal record Curried<TValue> {\n    public make: ((TValue) -> i32) -> i32\n}\ninternal record InUnion<TValue> {\n    public value: TValue | i32\n}\ninternal record InTuple<TValue; TOther> {\n    public value: (TValue, (TOther) -> i32)\n}\ninternal record InNominal<TValue> {\n    public value: (Out<TValue>, i32)\n    public other: [In<TValue>]\n}\ninternal record InClosure<TValue> {\n    public value: |TValue| -> TValue\n}\ninternal record InApply<TValue; TOther> {\n    public value: [Out<(TValue) -> TOther>]\n}\n" + MAIN,
    "fields": BASE + "internal record Vis {\n    public a: i32\n    internal b: bool\n    private c: char\n    public d: [i32; 2]\n    public f: Point\n}\ninternal record GenericVis<TValue; TOther = bool> {\n    public a: TValue\n    private b: TOther\n    public c: (TValue, TOther)\n}\n" + MAIN,
    "members": BASE + "internal record R {\n    public value: i32\n    internal procedure a(~, other: R, more: App::R) -> Self {\n        return self\n    }\n    internal procedure b(self: unique R) -> i32 {\n        return 0\n    }\n}\ninternal class C {\n    type Item = i32\n    value: i32\n    procedure get(~, other: Self) -> Self::Item\n}\ninternal modal M {\n    @S {\n        public value: i32\n        internal procedure get(~, other: M@S) -> M {\n            return widen self\n        }\n        internal transition next(step: (i32, bool)) -> @T {\n            return M@T {}\n        }\n    }\n    @T {}\n}\ninternal enum E {\n    A(i32, bool)\n    B { value: [i32; 2], other: E }\n}\nextern \"C\" {\n    public procedure imported(value: *imm u8, count: usize) -> i32\n}\ninternal let table: [i32; 3] = [1, 2, 3]\n" + MAIN,
}

CASES.update({
    # --- layout ---
    "layout_records": BASE + "internal record Empty {}\ninternal record Bytes3 {\n    public a: u8\n    public b: u8\n    public c: u8\n}\ninternal record Padded {\n    public a: u8\n    public b: u32\n    public c: u8\n    public d: u64\n    public e: u16\n}\ninternal record Wide {\n    public a: u128\n    public b: bool\n}\ninternal record Nested {\n    public inner: Padded\n    public tail: u8\n    public point: Point\n}\ninternal record Texts {\n    public a: string\n    public b: string@View\n    public c: string@Managed\n    public d: bytes\n    public e: bytes@View\n    public f: bytes@Managed\n}\ninternal record Pointers {\n    public a: (Ptr<i32>@Valid;)\n    public b: *mut u8\n    public c: (i32) -> i32\n    public d: |i32| -> i32\n    public e: [i32]\n    public f: $Named\n    public g: usize\n    public h: ()\n    public i: char\n    public j: f16\n}\ninternal record Arrays {\n    public a: [u8; 3]\n    public b: [u32; 2]\n    public c: [Padded; 2]\n    public d: [(); 9]\n    public e: [[u16; 3]; 2]\n}\ninternal record Tuples {\n    public a: (u8, u32)\n    public b: (u8;)\n    public c: ((u8, u16), u8)\n    public d: ()\n}\n" + MAIN,
    "layout_record_attrs": BASE + "#layout(packed)\ninternal record Packed {\n    public a: u8\n    public b: u32\n    public c: u16\n}\n#layout(align(16))\ninternal record Aligned {\n    public a: u8\n}\n#layout(C)\ninternal record CLayout {\n    public a: u8\n    public b: u32\n}\n#layout(C, align(32))\ninternal record CAligned {\n    public a: u8\n    public b: u32\n}\n#layout(packed, align(8))\ninternal record PackedAligned {\n    public a: u8\n    public b: u32\n}\n#layout(align(2))\ninternal record UnderAligned {\n    public a: u64\n}\n#layout(align(3))\ninternal record OddAligned {\n    public a: u8\n    public b: u8\n}\n#layout(align(1_6))\ninternal record Separated {\n    public a: u8\n}\n#layout(align(0x10))\ninternal record HexAligned {\n    public a: u8\n}\n#layout(align(16))\ninternal record EmptyAligned {}\n#layout(packed)\ninternal record EmptyPacked {}\ninternal record Holder {\n    public a: u8\n    public packed: Packed\n    public aligned: Aligned\n}\n" + MAIN,
    "layout_enums": BASE + "internal enum Units {\n    A\n    B\n    C\n}\ninternal enum Explicit {\n    A = 1\n    B = 5\n    C\n}\ninternal enum Big {\n    A = 255\n    B\n}\ninternal enum Huge {\n    A = 65536\n}\ninternal enum Giant {\n    A = 4294967296\n    B\n}\ninternal enum Payloads {\n    None\n    Small(u8)\n    Pair(u8, u32)\n    Named { a: u16, b: u64 }\n    Text(string@View)\n}\ninternal enum Aligned16 {\n    A(u128)\n    B(u8)\n}\ninternal enum One {\n    Only(i32)\n}\ninternal enum Recursive {\n    Leaf\n    Node(Ptr<Recursive>@Valid)\n}\ninternal enum Direct {\n    Leaf\n    Node(Direct)\n}\ninternal enum Hex {\n    A = 0x10\n    B = 0b11\n    C = 1_0\n    D = 7u8\n}\n" + MAIN,
    "layout_enum_attrs": BASE + "#layout(u16)\ninternal enum Wide {\n    A\n    B(u8)\n}\n#layout(u64)\ninternal enum Widest {\n    A\n}\n#layout(i8)\ninternal enum Signed {\n    A = 127\n}\n#layout(i8)\ninternal enum SignedOver {\n    A = 128\n}\n#layout(u8)\ninternal enum Over {\n    A = 256\n}\n#layout(u8, align(8))\ninternal enum WithAlign {\n    A(u8)\n}\n#layout(align(4))\ninternal enum OnlyAlign {\n    A\n    B\n}\n#layout(C)\ninternal enum CEnum {\n    A\n    B(u32)\n}\n" + MAIN,
    "layout_enum_bad_discs": BASE + "internal enum Dup {\n    A = 1\n    B = 1\n}\ninternal enum DupImplicit {\n    A = 1\n    B = 0\n    C\n}\ninternal enum Max {\n    A = 18446744073709551615\n}\ninternal enum MaxThenMore {\n    A = 18446744073709551615\n    B\n}\ninternal enum TooBig {\n    A = 18446744073709551616\n}\ninternal enum Holder {\n    A(Dup)\n    B(MaxThenMore)\n}\ninternal record UsesBad {\n    public a: Dup\n    public b: u8\n}\n" + MAIN,
    "layout_enum_generic": BASE + "internal enum Maybe<TValue> {\n    Nothing\n    Just(TValue)\n}\ninternal enum Either<TLeft; TRight = bool> {\n    Left(TLeft)\n    Right(TRight)\n    Both { left: TLeft, right: TRight }\n}\ninternal record Uses {\n    public a: (Maybe<u8>;)\n    public b: (Maybe<u64>;)\n    public c: (Either<u8>;)\n    public d: (Either<u64, u16>;)\n    public e: [Maybe<Maybe<u8>>]\n}\ninternal type A = Maybe<u32>\ninternal type B = Either<u8, u8>\ninternal type C = Maybe<i32, bool>\n" + MAIN,
    "layout_modals": BASE + "internal modal Two {\n    @A {\n        public value: u32\n    }\n    @B {\n        public value: u64\n        public flag: bool\n    }\n}\ninternal modal Niche {\n    @Some {\n        public value: Ptr<i32>@Valid\n    }\n    @None {}\n}\ninternal modal NicheThree {\n    @Some {\n        public value: Ptr<i32>@Valid\n    }\n    @None {}\n    @Other {}\n}\ninternal modal NicheTwoPayloads {\n    @A {\n        public value: Ptr<i32>@Valid\n    }\n    @B {\n        public value: Ptr<u8>@Valid\n    }\n}\ninternal modal NicheNonEmptyOther {\n    @A {\n        public value: Ptr<i32>@Valid\n    }\n    @B {\n        public value: u8\n    }\n}\ninternal modal NotValidPtr {\n    @A {\n        public value: Ptr<i32>\n    }\n    @B {}\n}\ninternal modal NicheWithMethod {\n    @A {\n        public value: Ptr<i32>@Valid\n        internal procedure get(~) -> i32 {\n            return 0\n        }\n    }\n    @B {}\n}\ninternal modal Single {\n    @Only {\n        public value: u16\n    }\n}\ninternal modal AllEmpty {\n    @A {}\n    @B {}\n    @C {}\n}\ninternal modal Generic<TValue> {\n    @Full {\n        public value: TValue\n    }\n    @Empty {}\n}\ninternal record Uses {\n    public a: Two\n    public b: Two@A\n    public c: Two@B\n    public d: Niche\n    public e: Niche@Some\n    public f: Niche@None\n    public g: (Generic<u64>;)\n    public h: (Generic<u8>@Full;)\n    public i: Two@Missing\n    public j: AllEmpty\n}\n" + MAIN,
    "layout_unions": aliases("u8 | u32", "u8 | ()", "() | bool", "Ptr<i32>@Valid | ()", "Ptr<i32> | ()", "Ptr<i32>@Valid | () | bool",
                             "Ptr<i32>@Valid | Ptr<u8>@Valid", "unique Ptr<i32>@Valid | ()", "Handle | ()", "Chain | ()",
                             "u8 | u16 | u32 | u64 | u128", "(u8, u8) | [u8; 3]", "string | bytes", "Point | Shape | Door",
                             "! | u8", "Ptr<i32>@Null | ()", "Wrapped | ()",
                             prefix="internal type Handle = Ptr<i32>@Valid\ninternal type Chain = Handle\ninternal type Wrapped = (Ptr<i32>@Valid;)\n"),
    "layout_recursive": BASE + "internal record ListNode {\n    public value: i32\n    public next: Ptr<ListNode>\n}\ninternal record Direct {\n    public value: i32\n    public next: Direct\n}\ninternal record MutualA {\n    public b: MutualB\n    public x: u8\n}\ninternal record MutualB {\n    public a: MutualA\n    public y: u64\n}\ninternal record ViaArray {\n    public items: [ViaArray; 2]\n}\ninternal record ViaTuple {\n    public pair: (ViaTuple, u8)\n}\ninternal record ViaUnion {\n    public either: ViaUnion | u8\n}\ninternal record ViaSlice {\n    public items: [ViaSlice]\n}\n" + MAIN,
    "layout_recursive_alias_tuple": BASE + "internal type A1 = (A1, u8)\n" + MAIN,
    "layout_recursive_alias_array": BASE + "internal type B1 = [B1; 2]\n" + MAIN,
    "layout_recursive_alias_direct": BASE + "internal type C1 = C1\ninternal record Holds {\n    public value: C1\n}\n" + MAIN,
    "layout_recursive_direct_record": BASE + "internal record Direct {\n    public value: i32\n    public next: Direct\n}\n" + MAIN,
    "layout_recursive_mutual_records": BASE + "internal record MutualA {\n    public b: MutualB\n    public x: u8\n}\ninternal record MutualB {\n    public a: MutualA\n    public y: u64\n}\n" + MAIN,
    "layout_recursive_via_union": BASE + "internal record ViaUnion {\n    public either: ViaUnion | u8\n}\n" + MAIN,
    "layout_generic_records": BASE + "internal record Box<TValue> {\n    public value: TValue\n    public tag: u8\n}\ninternal record Pair<TFirst; TSecond = u64> {\n    public first: TFirst\n    public second: TSecond\n}\ninternal record Uses {\n    public a: (Box<u8>;)\n    public b: (Box<u64>;)\n    public c: (Pair<u8>;)\n    public d: (Pair<u8, u8>;)\n    public e: (Box<Box<u16>>;)\n    public f: (Box<Pair<u8, u16>>;)\n    public g: (Pair<u8, u8, u8>;)\n    public h: Box\n}\ninternal type A = Box<[u32; 3]>\ninternal type B = Pair<(), ()>\ninternal type Alias<TValue> = Box<TValue>\ninternal type C = Alias<u64>\ninternal type D = Alias<u8, u8>\n" + MAIN,
    "layout_async": aliases("Async<u8>", "Async<u64>", "Async<u128>", "Async<(), (), u128>", "Async<(), (), (), u128>",
                            "Async<u8, u8, [u8; 40], [u8; 60]>", "Future<u8>", "Future<[u64; 4], u8>", "Sequence<u128>",
                            "Stream<u8, [u8; 33]>", "Pipe<u8, u128>", "Exchange<u128>", "Async<u8>@Suspended",
                            "Async<u8, u8, u8, u8>@Failed", "Job", "Job2<u128>", "Spawned<u8>", "Spawned<u8>@Ready",
                            "Tracked<u8, u8>", "CancelToken", "CancelToken@Active", "Region", "Region@Active", "File",
                            "File@Read", "DirIter", "Context", "Outcome<u8, u64>", "Duration", "RegionOptions",
                            prefix="internal type Job = Future<u128>\ninternal type Job2<TValue> = Stream<TValue, u8>\n"),
    "layout_misc": aliases("GpuPtr<i32, Global>", "GpuPtr<i32, Shared>", "GpuPtr<i32, Private>", "Range<u8>", "RangeInclusive<u64>",
                           "RangeFrom<u16>", "RangeTo<u32>", "RangeToInclusive<u128>", "RangeFull", "unique Range<u8>",
                           "i32 |: { self > 0 }", "opaque Named", "Count", "Deep", "!", "()", "(!, u8)", "[!; 3]", "[u8; 0]",
                           "[u64; 1_000_000]", "$Named", "[$Named; 2]",
                           prefix="internal type Count = usize\ninternal type Deep = Count\n"),
})

OTHER = """public let WIDTH: usize = 6
internal let INNER: usize = 7
private let HIDDEN: usize = 8
public let NOT_INT: bool = true
public record Remote {
    public value: i32
}
public type Alias = Remote
public type Later = Future<Remote>
public enum Kind {
    One
}
"""

MULTI = {
    "array_len_qualified": (APP, {
        "Source/Main.uv": "import App::Other\ninternal type A = [i32; App::Other::WIDTH]\ninternal type B = [i32; Other::WIDTH]\ninternal type C = [i32; App::Other::INNER]\ninternal type E = [i32; App::Other::NOT_INT]\ninternal type H = [i32; 6]\n" + MAIN,
        "Source/Other/o.uv": OTHER,
    }),
    "array_len_using": (APP, {
        "Source/Main.uv": "using App::Other::WIDTH\nusing App::Other::{ INNER as RENAMED }\ninternal type A = [i32; WIDTH]\ninternal type B = [i32; RENAMED]\ninternal type C = [i32; 6]\ninternal type D = [i32; 7]\n" + MAIN,
        "Source/Other/o.uv": OTHER,
    }),
    "array_len_shadowed": (APP, {
        "Source/Main.uv": "internal let WIDTH: usize = 2\ninternal type A = [i32; WIDTH]\ninternal type B = [i32; App::Other::WIDTH]\n" + MAIN,
        "Source/Other/o.uv": OTHER,
    }),
    "lookup_other_module": (APP, {
        "Source/Main.uv": "import App::Other\nusing App::Other::Remote\nusing App::Other::{ Kind as Sort }\ninternal type A = Remote\ninternal type B = App::Other::Remote\ninternal type C = Other::Remote\ninternal type D = Sort\ninternal type E = App::Other::Alias\ninternal type F = Remote | App::Other::Remote\ninternal type G = (Remote, Sort, App::Other::Kind)\ninternal type H = App::Other::Later\ninternal type I = Other::Later\n" + MAIN,
        "Source/Other/o.uv": OTHER,
    }),
    "lookup_nested_module": (APP, {
        "Source/Main.uv": "internal type A = App::Other::Deep::Far\ninternal type B = Other::Deep::Far\ninternal type C = App::Other::Deep::Far | App::Other::Remote\n" + MAIN,
        "Source/Other/o.uv": OTHER,
        "Source/Other/Deep/d.uv": "public record Far {\n    public value: i32\n}\n",
    }),
    "lookup_other_assembly": (APP + LIB, {
        "Source/Main.uv": "import Lib\ninternal type A = Lib::Thing\ninternal type B = [i32; Lib::COUNT]\ninternal type C = Lib::Thing | i32\ninternal record Holder {\n    public thing: Lib::Thing\n    private secret: Lib::Thing\n}\n" + MAIN,
        "Lib/l.uv": "public record Thing {\n    public value: i32\n}\npublic let COUNT: usize = 5\n",
    }),
}


def main() -> None:
    if OUT.exists():
        shutil.rmtree(OUT)
    cases = {name: (APP, {"Source/Main.uv": text}) for name, text in CASES.items()}
    cases.update(MULTI)
    entries = []
    for name, (manifest, files) in sorted(cases.items()):
        case_dir = OUT / name
        case_dir.mkdir(parents=True)
        (case_dir / "Ultraviolet.toml").write_text(manifest, encoding="utf-8")
        for rel, content in files.items():
            path = case_dir / rel
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(content, encoding="utf-8", newline="")
        entries.append(f"target/typecases/{name}/Ultraviolet.toml")
    (ROOT / "tests" / "golden" / "types_extra.list").write_text("\n".join(entries) + "\n", encoding="utf-8")
    print(f"wrote {len(entries)} type cases")


if __name__ == "__main__":
    main()
