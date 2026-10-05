#!/usr/bin/env python3
"""Generate projects under target/relcases that pin the relations between types.

Each case declares types (as aliases, fields and signatures) whose subtyping,
well-formedness, intrinsic class membership, class tables or static proofs are worth
comparing with the reference.
"""
from pathlib import Path
import shutil

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "target" / "relcases"

APP = '[[assembly]]\nname = "App"\nkind = "executable"\nroot = "Source"\n'
MAIN = "public procedure main() -> i32 {\n    return 0\n}\n"

BASE = """internal record Point {
    public x: i32
    public y: i32
}

internal enum Shape {
    Empty
    Circle(i32)
}

internal enum Color {
    Red
    Green
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
    lines = "".join(f"internal type T{index} = {ty}\n" for index, ty in enumerate(types))
    return BASE + prefix + lines + MAIN


def contracts(*clauses: str, params: str = "x: i32, y: i32, z: i32") -> str:
    procs = "".join(
        f"internal procedure p{index}({params}) -> i32\n    |: {clause}\n{{\n    return 0\n}}\n\n"
        for index, clause in enumerate(clauses)
    )
    return BASE + procs + MAIN


CASES = {
    "sub_prims": aliases("i8", "i16", "i32", "i64", "u8", "u32", "f32", "f64", "bool", "char", "()", "!", "usize"),
    "sub_perms": aliases("i32", "const i32", "unique i32", "shared i32", "Point", "const Point", "unique Point",
                         "shared Point", "unique [i32]", "const [i32]", "shared (i32, bool)"),
    "sub_tuples": aliases("(i32, bool)", "(i32, bool, char)", "(!, bool)", "(i32, !)", "(i32;)", "(!;)", "()",
                          "((i32, !), bool)", "((i32, i32), bool)"),
    "sub_arrays": aliases("[i32; 4]", "[i32; 5]", "[i32]", "[!; 4]", "[!]", "[[i32; 2]; 2]", "[[i32]; 2]", "[[i32]]",
                          "[string@View; 2]", "[string; 2]", "[string]"),
    "sub_ranges": aliases("Range<i32>", "Range<!>", "RangeInclusive<i32>", "RangeFrom<i32>", "RangeTo<i32>",
                          "RangeToInclusive<i32>", "RangeFull", "Range<u8>", "RangeInclusive<!>"),
    "sub_ptrs": aliases("Ptr<i32>", "Ptr<i32>@Valid", "Ptr<i32>@Null", "Ptr<i32>@Expired", "Ptr<u8>@Valid", "Ptr<u8>",
                        "*imm i32", "*mut i32", "*imm u8", "Ptr<Point>@Valid", "Ptr<Point>"),
    "sub_text": aliases("string", "string@View", "string@Managed", "bytes", "bytes@View", "bytes@Managed",
                        "const string@View", "unique string@Managed", "const string", "(string@View, bytes@Managed)",
                        "(string, bytes)"),
    "sub_funcs": aliases("(i32) -> i32", "(i32) -> !", "(!) -> i32", "(move i32) -> i32", "(i32, bool) -> i32",
                         "() -> i32", "() -> !", "(string) -> string@View", "(string@View) -> string",
                         "((i32) -> i32) -> i32"),
    "sub_closures": aliases("|i32| -> i32", "|i32| -> !", "|move i32| -> i32", "|| -> i32", "|i32, bool| -> ()",
                            "|| -> i32 [shared: { value: shared i32 }]", "|| -> i32 [shared: { value: shared u8 }]",
                            "|| -> i32 [shared: { other: shared i32 }]", "|string| -> string@View",
                            "|string@View| -> string"),
    "sub_unions": aliases("i32 | bool", "bool | i32", "i32 | bool | char", "i32", "bool", "!", "i32 | !",
                          "string | i32", "string@View | i32", "Point | Shape", "Point",
                          "[i32; 2] | [i32]", "[i32]"),
    "sub_modal": aliases("Door", "Door@Open", "Door@Closed", "Slot", "Slot@Full", "Slot@Empty", "Big", "Big@Loaded",
                         "Big@Idle", "Gen<i32>", "Gen<i32>@Some", "Gen<bool>@Some", "Gen<i32>@None", "Gen<bool>",
                         prefix="internal modal Slot {\n    @Full {\n        public item: Ptr<i32>@Valid\n    }\n    @Empty {}\n}\n"
                                "internal modal Big {\n    @Loaded {\n        public data: [u8; 512]\n    }\n    @Idle {}\n}\n"
                                "internal modal Gen<T> {\n    @Some {\n        public value: T\n    }\n    @None {}\n}\n"),
    "sub_alias": aliases("Num", "Num2", "i32", "Pair<i32>", "(i32, i32)", "Pair<!>", "Either", "bool | i32",
                         "Nest<i32>", "((i32, i32), (i32, i32))", "unique Num", "[Num; 2]", "[i32; 2]",
                         "Wrap<Num>", "Wrap<i32>", "Pair<Num2>",
                         prefix="internal type Num = i32\ninternal type Num2 = Num\ninternal type Pair<T> = (T, T)\n"
                                "internal type Either = i32 | bool\ninternal type Nest<T> = Pair<Pair<T>>\n"
                                "internal record Wrap<T> {\n    public value: T\n}\n"),
    "sub_variance": aliases("Box<i32>", "Box<!>", "Box<string@View>", "Box<string>", "Sink<i32>", "Sink<!>",
                            "Sink<string@View>", "Sink<string>", "Both<i32>", "Both<!>", "Both<string@View>",
                            "Both<string>", "Phantom<i32>", "Phantom<bool>", "Empty<i32>", "Empty<bool>",
                            "Opt<string@View>", "Opt<string>", "Two<!, i32>", "Two<i32, !>", "Two<i32, i32>",
                            "const Box<string@View>", "const Box<string>", "const Sink<string@View>",
                            "const Sink<string>", "unique Box<string@View>", "unique Box<string>",
                            prefix="internal record Box<T> {\n    public value: T\n}\n"
                                   "internal record Sink<T> {\n    public take: (T) -> ()\n}\n"
                                   "internal record Both<T> {\n    public value: T\n    public take: (T) -> ()\n}\n"
                                   "internal record Phantom<T> {\n    public value: i32\n}\n"
                                   "internal record Empty<T> {}\n"
                                   "internal enum Opt<T> {\n    None\n    Some(T)\n}\n"
                                   "internal record Two<A; B> {\n    public first: A\n    public take: (B) -> ()\n}\n"),
    "sub_async": aliases("Async<i32>", "Async<!>", "Async<i32, bool>", "Async<i32, !>", "Async<i32, bool, char>",
                         "Async<i32, bool, char, u8>", "Async<i32, bool, !, u8>", "Future<i32>", "Future<!>",
                         "Sequence<i32>", "Stream<i32, bool>", "Async<i32>@Suspended", "Async<i32>@Failed"),
    "sub_dynamic": aliases("$Named", "$Loud", "$Other", "Point", "Speaker", "Shouter", "unique Speaker", "$IO",
                           "$Network", "$FileIo", "shared $Named", "shared $Mutating", "const $Mutating",
                           "$Mutating", "opaque Named", "opaque Loud",
                           prefix="internal class Loud <: Named {\n    procedure shout(~) -> i32\n}\n"
                                  "internal class Other {\n    procedure other(~) -> i32\n}\n"
                                  "internal class FileIo <: IO {\n    procedure extra(~) -> i32\n}\n"
                                  "internal class Mutating {\n    procedure change(~!) -> i32\n    procedure read(~) -> i32\n}\n"
                                  "internal record Speaker <: Named {\n    public id: i32\n    procedure name(~) -> i32 {\n        return 1\n    }\n}\n"
                                  "internal record Shouter <: Loud {\n    public id: i32\n    procedure name(~) -> i32 {\n        return 1\n    }\n    procedure shout(~) -> i32 {\n        return 2\n    }\n}\n"),
    "refine_basic": aliases("i32", "i32 |: { self > 0 }", "i32 |: { self > 5 }", "i32 |: { self >= 1 }",
                            "i32 |: { self > 0 && self < 10 }", "i32 |: { true }", "i32 |: { 1 + 1 == 2 }",
                            "i32 |: { self * 2 > 10 }", "u8 |: { self < 200 }", "i32 |: { self > 0 || self < 0 }",
                            "i32 |: { !(self <= 0) }", "i32 |: { self != 0 }", "i32 |: { self == 7 }",
                            "i32 |: { 0 < self }", "u8 |: { self > 0 }", "i32 |: { false }",
                            "i32 |: { self + 1 > 1 }", "i32 |: { self - 1 >= 0 }", "i32 |: { 2 * self >= 2 }",
                            "i32 |: { self > 0 && self > 5 }", "i32 |: { self % 2 == 0 }", "i32 |: { self / 2 > 1 }"),
    "refine_nested": aliases("(i32 |: { self > 0 }, bool)", "(i32, bool)", "[i32 |: { self > 0 }; 2]", "[i32; 2]",
                             "unique i32 |: { self > 0 }", "const i32 |: { self > 0 }", "Point |: { self.x > 0 }",
                             "Point |: { self.x > 0 && self.y > 0 }", "Point |: { self.y > 0 }", "Point",
                             "Ptr<i32 |: { self > 0 }>"),
    "wf_generic": aliases("Box", "Box<i32>", "Box<i32, bool>", "Two", "Two<i32>", "Two<i32, bool>",
                          "Two<i32, bool, char>", "Def", "Def<i32>", "Def<i32, bool>", "(Box, i32)", "[Box; 2]",
                          "Box<Box>", "Box<Box<i32>>", "App::Box<i32>",
                          prefix="internal record Box<T> {\n    public value: T\n}\n"
                                 "internal record Two<A; B> {\n    public a: A\n    public b: B\n}\n"
                                 "internal record Def<A; B = bool> {\n    public a: A\n    public b: B\n}\n"),
    "wf_misc": aliases("Door@Open", "Async<i32, i32, i32, i32, i32>", "$Named", 
                       "shared $Named", "i32 | i32", "i32 | bool", "TestAuthority", "Context", "$HeapAllocator",
                       "$Reactor", "$System", "$ExecutionDomain", "Gen<i32>@Some", "Gen@Some", "Gen<i32, i32>@Some",
                       prefix="internal modal Gen<T> {\n    @Some {\n        public value: T\n    }\n    @None {}\n}\n"),
    "pred_bitcopy": aliases("Point", "Owner", "Text", "View", "Chain", "Loop", "Tree", "Shape", "Heavy", "Door",
                            "Door@Open", "Door@Closed", "Vault", "Vault@Locked", "Gen<i32>", "Gen<string@Managed>",
                            "Gen<unique i32>", "Bound<i32>", "PointAlias", "OwnerAlias", "(Point, Owner)",
                            "[Owner; 2]", "Point | Owner", "FileKind", "IoError", "Duration", "Context",
                            prefix="internal record Owner {\n    public value: unique i32\n}\n"
                                   "internal record Text {\n    public value: string@Managed\n}\n"
                                   "internal record View {\n    public value: string@View\n}\n"
                                   "internal record Chain {\n    public next: Ptr<Chain>\n}\n"
                                   "internal record Loop {\n    public inner: (Loop;)\n}\n"
                                   "internal record Tree {\n    public left: Point\n    public right: View\n}\n"
                                   "internal enum Heavy {\n    None\n    Text(string@Managed)\n    Pair { a: i32, b: bytes@Managed }\n}\n"
                                   "internal modal Vault {\n    @Locked {\n        public key: unique i32\n    }\n    @Unlocked {\n        public code: i32\n    }\n}\n"
                                   "internal record Gen<T> {\n    public value: T\n}\n"
                                   "internal record Bound<T <: Bitcopy> {\n    public value: T\n}\n"
                                   "internal type PointAlias = Point\ninternal type OwnerAlias = Owner\n"),
    "pred_clone_drop": aliases("Cloner", "BadClone", "ArgClone", "Dropper", "BadDrop", "RetDrop", "Both",
                               "unique Cloner", "const Dropper", "string@Managed", "bytes@Managed", "string@View",
                               "string", "(Cloner, Dropper)", "MoveClone",
                               prefix="internal record Cloner {\n    public text: string@Managed\n    procedure clone(~) -> Self {\n        return self\n    }\n}\n"
                                      "internal record BadClone {\n    public text: string@Managed\n    procedure clone(~!) -> Self {\n        return self\n    }\n}\n"
                                      "internal record ArgClone {\n    public text: string@Managed\n    procedure clone(~, other: i32) -> Self {\n        return self\n    }\n}\n"
                                      "internal record MoveClone {\n    public text: string@Managed\n    procedure clone(~) -> i32 {\n        return 1\n    }\n}\n"
                                      "internal record Dropper {\n    public text: string@Managed\n    procedure drop(~!) {\n        return\n    }\n}\n"
                                      "internal record BadDrop {\n    public text: string@Managed\n    procedure drop(~) {\n        return\n    }\n}\n"
                                      "internal record RetDrop {\n    public text: string@Managed\n    procedure drop(~!) -> i32 {\n        return 1\n    }\n}\n"
                                      "internal record Both {\n    public text: string@Managed\n    procedure clone(~) -> Both {\n        return self\n    }\n    procedure drop(~!) -> () {\n        return\n    }\n}\n"),
    "pred_ffi": aliases("CPoint", "Point", "CEnum", "Shape", "CNested", "CBad", "CGen<i32>", "CGen<bool>", "CGen",
                        "CBound<i32>", "CBound", "CAlias", "*mut CPoint", "[CPoint; 2]", "(CPoint) -> i32",
                        "(bool) -> i32", "bool", "char", "i128", "f16", "string", "Ptr<i32>", "(i32, i32)",
                        "CSelf", "CPayload", "CBadPayload", "Door", "Context",
                        prefix="#layout(C)\ninternal record CPoint {\n    public x: i32\n    public y: f64\n}\n"
                               "#layout(C)\ninternal enum CEnum {\n    A\n    B\n}\n"
                               "#layout(C)\ninternal record CNested {\n    public p: CPoint\n    public e: CEnum\n    public raw: *imm u8\n}\n"
                               "#layout(C)\ninternal record CBad {\n    public p: Point\n}\n"
                               "#layout(C)\ninternal record CGen<T> {\n    public value: T\n}\n"
                               "#layout(C)\ninternal record CBound<T <: FfiSafe> {\n    public value: T\n}\n"
                               "#layout(C)\ninternal record CSelf {\n    public inner: (CSelf) -> i32\n}\n"
                               "#layout(C)\ninternal enum CPayload {\n    None\n    Some(i32)\n    Pair { a: i32, b: CPoint }\n}\n"
                               "#layout(C)\ninternal enum CBadPayload {\n    None\n    Some(bool)\n}\n"
                               "internal type CAlias = CPoint\n"),
    "pred_gpu_zero": aliases("Point", "GGen<i32>", "GGen<bool>", "GBound<i32>", "GGen<i128>", "Shape", "Color",
                             "ZeroFirst", "NoZero", "ZeroPayload", "Ptr<i32>", "Ptr<i32>@Valid", "Ptr<i32>@Null",
                             "*mut i32", "[i32; 4]", "[i32]", "(i32, bool)", "(i32, char)", "string@View", "string",
                             "$Named", "$IO", "Door", "Door@Open", "i128", "u128", "char", "bool", "()", "!",
                             "Range<i32>", "RangeFull", "(i32) -> i32", "PointAlias", "i32 | bool", "WithPtr",
                             prefix="internal record GGen<T> {\n    public value: T\n}\n"
                                    "internal record GBound<T <: GpuSafe> {\n    public value: T\n}\n"
                                    "internal enum ZeroFirst {\n    A = 0\n    B = 1\n}\n"
                                    "internal enum NoZero {\n    A = 1\n    B = 2\n}\n"
                                    "internal enum ZeroPayload {\n    A(Ptr<i32>@Valid) = 0\n    B = 1\n}\n"
                                    "internal record WithPtr {\n    public p: Ptr<i32>@Valid\n}\n"
                                    "internal type PointAlias = Point\n"),
    "pred_eq": aliases("i32", "f32", "bool", "char", "Color", "Shape", "Point", "string", "bytes@View", "Ptr<i32>",
                       "*imm i32", "unique i32", "i32 |: { self > 0 }", "(i32, i32)", "[i32; 2]", "Color | i32",
                       "u8", "usize", "isize", "const char"),
    "class_linear": BASE + (
        "internal class A {\n    procedure a(~) -> i32\n}\n"
        "internal class B <: A {\n    procedure b(~) -> i32\n}\n"
        "internal class C <: A {\n    procedure c(~) -> i32\n}\n"
        "internal class D <: B + C {\n    procedure d(~) -> i32\n}\n"
        "internal class E <: C + B {\n    procedure e(~) -> i32\n}\n"
        "internal class F <: D + E {\n    procedure f(~) -> i32\n}\n"
        "internal class G <: A + B {\n    procedure g(~) -> i32\n}\n"
        "internal class H <: Missing {\n    procedure h(~) -> i32\n}\n"
        "internal class I <: J {\n    procedure i(~) -> i32\n}\n"
        "internal class J <: I {\n    procedure j(~) -> i32\n}\n"
        "internal class K <: K {\n    procedure k(~) -> i32\n}\n"
        "internal record R <: D {\n    public v: i32\n    procedure a(~) -> i32 {\n        return 1\n    }\n    procedure d(~) -> i32 {\n        return 1\n    }\n}\n"
        "internal record S <: B, Named {\n    public v: i32\n}\n"
        "internal type T0 = R\ninternal type T1 = S\ninternal type T2 = $D\ninternal type T3 = $A\ninternal type T4 = $F\n"
        + MAIN),
    "class_methods": BASE + (
        "internal class Base {\n    procedure same(~) -> i32\n    procedure differ(~) -> i32\n    procedure body(~) -> i32 {\n        return 1\n    }\n}\n"
        "internal class Same <: Base {\n    procedure same(~) -> i32\n}\n"
        "internal class Clash <: Base {\n    procedure differ(~) -> bool\n}\n"
        "internal class Recv <: Base {\n    procedure differ(~!) -> i32\n}\n"
        "internal class Params <: Base {\n    procedure differ(~, extra: i32) -> i32\n}\n"
        "internal class Generic {\n    procedure pick<T>(~, value: T) -> i32\n    procedure plain(~) -> i32\n}\n"
        "internal class BySelf {\n    procedure merge(~, other: Self) -> i32\n    procedure plain(~) -> i32\n}\n"
        "internal class ByRef {\n    procedure merge(~, other: const Self) -> Self\n}\n"
        "internal class Inherit <: Generic {\n    procedure more(~) -> i32\n}\n"
        "internal class Shared {\n    procedure touch(~%) -> i32\n}\n"
        "internal class Explicit {\n    procedure touch(self: const Self) -> i32\n    procedure take(self: unique Self) -> i32\n}\n"
        "internal record Impl <: Base {\n    public v: i32\n    procedure same(~) -> i32 {\n        return 1\n    }\n}\n"
        "internal record Full <: Base {\n    public v: i32\n    procedure same(~) -> i32 {\n        return 1\n    }\n    procedure differ(~) -> i32 {\n        return 1\n    }\n}\n"
        "internal type T0 = $Base\ninternal type T1 = $Generic\ninternal type T2 = $BySelf\ninternal type T3 = shared $Recv\n"
        "internal type T4 = shared $Explicit\ninternal type T5 = shared $Shared\ninternal type T6 = Impl\ninternal type T7 = Full\n"
        + MAIN),
    "class_caps": BASE + (
        "internal class FileIo <: IO {\n    procedure extra(~) -> i32\n}\n"
        "internal class Deep <: FileIo {\n    procedure deeper(~) -> i32\n}\n"
        "internal class Plain {\n    procedure plain(~) -> i32\n}\n"
        "internal class Net <: Network + Plain {\n    procedure both(~) -> i32\n}\n"
        "internal class Domain <: ExecutionDomain {\n    procedure run(~) -> i32\n}\n"
        "internal type T0 = $FileIo\ninternal type T1 = $Deep\ninternal type T2 = $Plain\ninternal type T3 = $Net\n"
        "internal type T4 = $IO\ninternal type T5 = $Network\ninternal type T6 = $Domain\ninternal type T7 = $ExecutionDomain\n"
        "internal type T8 = $Time\ninternal type T9 = $MonotonicTime\ninternal type T10 = Context\n"
        + MAIN),
    "class_bounds": BASE + (
        "internal class Loud <: Named {\n    procedure shout(~) -> i32\n}\n"
        "internal procedure a<T <: Named>(value: T) -> T {\n    return value\n}\n"
        "internal procedure b<T <: Loud>(value: T, other: i32) -> T {\n    return value\n}\n"
        "internal procedure c<T <: Bitcopy; U <: Eq>(value: T, other: U) -> U {\n    return other\n}\n"
        "internal record Holder<T <: Named> {\n    public value: T\n}\n"
        "internal record Speaker <: Loud {\n    public id: i32\n    procedure name(~) -> i32 {\n        return 1\n    }\n    procedure shout(~) -> i32 {\n        return 1\n    }\n}\n"
        "internal enum Tone <: Named {\n    Low\n    High\n}\n"
        "internal modal Gate <: Named {\n    @Up {}\n    @Down {}\n}\n"
        "internal type T0 = Speaker\ninternal type T1 = Tone\ninternal type T2 = Gate\ninternal type T3 = Holder<Speaker>\n"
        "internal type T4 = unique Speaker\ninternal type T5 = Gate@Up\n"
        + MAIN),
    "sig_methods": BASE + (
        "internal record Counter {\n    public n: i32\n"
        "    procedure get(~) -> i32 {\n        return self.n\n    }\n"
        "    procedure set(~!, value: i32) {\n        return\n    }\n"
        "    procedure share(~%, other: const Self) -> Self {\n        return other\n    }\n"
        "    procedure take(move ~, move extra: Point) -> (Self, Point) {\n        return (self, extra)\n    }\n"
        "    procedure explicit(self: unique Self, n: i32) -> bool {\n        return true\n    }\n"
        "    procedure generic(~, value: Box) -> i32 {\n        return 1\n    }\n"
        "    procedure nested(~, value: [Self; 2]) -> Self | i32 {\n        return 1\n    }\n"
        "}\n"
        "internal record Box<T> {\n    public value: T\n}\n"
        "internal modal Conn {\n"
        "    @Idle {\n        public host: i32\n"
        "        procedure peek(~) -> i32 {\n            return self.host\n        }\n"
        "        transition open(port: i32) -> @Live {\n            return Conn@Live { port: port }\n        }\n"
        "    }\n"
        "    @Live {\n        public port: i32\n"
        "        procedure send(~!, data: [u8]) -> Self {\n            return self\n        }\n"
        "        transition close() -> @Idle {\n            return Conn@Idle { host: 1 }\n        }\n"
        "    }\n"
        "}\n"
        + MAIN),
    "generic_params": BASE + (
        "internal record Box<T> {\n    public value: T\n}\n"
        "internal record Two<A; B> {\n    public a: A\n    public b: B\n}\n"
        "internal record Def<A; B = bool> {\n    public a: A\n    public b: B\n}\n"
        "internal record DefAll<A = i32; B = (A, bool); C = [u8; 4]> {\n    public a: A\n    public b: B\n    public c: C\n}\n"
        "internal record DefShapes<A = Point; B = Box<i32>; C = unique Point; D = string@View; E = Ptr<i32>@Valid; F = (i32) -> bool> {\n    public a: A\n}\n"
        "internal record DefMore<A = [i32]; B = *imm u8; C = $Named; D = Door@Open; E = i32 | bool; F = |i32| -> i32; G = bytes> {\n    public a: A\n}\n"
        "internal record DefRange<A = Range<i32>> {\n    public a: A\n}\n"
        # A default with an unsupported form nested in it (`(Range<i32>, i32)`) is left out:
        # the reference validates it into a type with a hole, which its printer cannot print.
        "internal record DefOrder<A = i32; B> {\n    public a: A\n    public b: B\n}\n"
        "internal record Bound<T <: Named> {\n    public value: T\n}\n"
        "internal record BoundDef<T <: Named = Speaker> {\n    public value: T\n}\n"
        "internal record BoundBad<T <: Named = i32> {\n    public value: T\n}\n"
        "internal record BoundTwo<T <: Named, Bitcopy; U <: Eq> {\n    public value: T\n    public other: U\n}\n"
        "internal record BoundCap<T <: IO; U <: Drop = string@Managed> {\n    public value: T\n}\n"
        "internal record BoundRange<T <: Named = Range<i32>> {\n    public value: T\n}\n"
        "internal record Speaker <: Named {\n    public id: i32\n    procedure name(~) -> i32 {\n        return 1\n    }\n}\n"
        "internal enum Opt<T> {\n    None\n    Some(T)\n}\n"
        "internal modal Cell<T; U = i32> {\n    @Full {\n        public value: T\n    }\n    @Empty {}\n}\n"
        "internal class Container<T> {\n    procedure get(~) -> i32\n}\n"
        "internal type Pair<T> = (T, T)\n"
        "internal type T0 = Speaker\ninternal type T1 = i32\ninternal type T2 = Point\ninternal type T3 = unique Speaker\n"
        "internal type T4 = $Named\ninternal type T5 = Door@Open\ninternal type T6 = (i32, bool)\ninternal type T7 = opaque Named\n"
        "internal type T8 = string@Managed\ninternal type T9 = Box<Speaker>\ninternal type T10 = [i32]\ninternal type T11 = $IO\n"
        + MAIN),
    "generic_infer": BASE + (
        "internal record Box<T> {\n    public value: T\n}\n"
        "internal procedure id<T>(value: T) -> T {\n    return value\n}\n"
        "internal procedure both<T>(a: T, b: T) -> T {\n    return a\n}\n"
        "internal procedure pair<T; U>(a: T, b: U) -> T {\n    return a\n}\n"
        "internal procedure tuple<T; U>(p: (T, U), q: (U, T)) -> T {\n    return p.0\n}\n"
        "internal procedure slice<T>(items: [T], first: T) -> T {\n    return first\n}\n"
        "internal procedure array<T>(items: [T; 4], other: [T; 2]) -> i32 {\n    return 1\n}\n"
        "internal procedure ptr<T>(p: Ptr<T>@Valid, q: *imm T, r: Ptr<T>) -> i32 {\n    return 1\n}\n"
        "internal procedure func<T; U>(f: (T) -> U, g: (move T, U) -> ()) -> i32 {\n    return 1\n}\n"
        "internal procedure closure<T; U>(f: |T| -> U, g: |move T| -> U) -> i32 {\n    return 1\n}\n"
        "internal procedure nominal<T>(b: Box<T>, bb: Box<Box<T>>) -> i32 {\n    return 1\n}\n"
        "internal procedure perm<T>(a: unique T, b: const T, c: shared T) -> i32 {\n    return 1\n}\n"
        "internal procedure union<T>(a: T | bool, b: i32 | T) -> i32 {\n    return 1\n}\n"
        "internal procedure fixed<T>(a: i32, b: Point, c: string@View, d: T) -> i32 {\n    return 1\n}\n"
        "internal procedure unused<T; U>(a: T) -> i32 {\n    return 1\n}\n"
        "internal procedure defaulted<T; U = bool>(a: T) -> i32 {\n    return 1\n}\n"
        "internal procedure baddefault<T; U = Range<i32>>(a: T) -> i32 {\n    return 1\n}\n"
        "internal procedure modalp<T>(a: Door@Open, b: T, c: $Named, d: Range<T>) -> i32 {\n    return 1\n}\n"
        "internal procedure refine<T>(a: T |: { true }, b: [T]) -> i32 {\n    return 1\n}\n"
        "internal procedure bounded<T <: Named; U <: Bitcopy>(a: T, b: U) -> i32 {\n    return 1\n}\n"
        "internal type T0 = i32\ninternal type T1 = bool\ninternal type T2 = Point\ninternal type T3 = (i32, bool)\n"
        "internal type T4 = [i32]\ninternal type T5 = Box<i32>\ninternal type T6 = unique Point\ninternal type T7 = string@View\n"
        + MAIN),
    "proof_linear": contracts(
        "x > 0 |= x >= 1", "x > 0 && y > x |= y > 1", "x >= 0 |= x + 1 > 0", "x < y && y < z |= x < z",
        "2 * x > 5 |= x >= 3", "true |= 1 < 2", "x == 3 |= x * 2 == 6", "x != 0 |= x > 0", "x > 0 |= x > 1",
        "x > 0 || x < 0 |= x != 0", "x + y > 10 && x < 3 |= y > 7", "x >= y && y >= x |= x == y",
        "x > 0 |= x > 0", "x > 0 |= 0 < x", "x > 5 |= x > 0 && x > 1", "x > 5 |= x > 0 || x < 0",
        "x > 5 |= x < 0 || x > 0", "3 * x + 2 * y >= 12 && x <= 2 |= y >= 3", "x - y > 0 |= x > y",
        "x == y |= x - y == 0", "x > 0 && y > 0 |= x + y > 1", "x > 0 && y > 0 |= x * y > 0",
        "x <= 10 && x >= 10 |= x == 10", "x < 0 |= -x > 0", "!(x > 0) |= x <= 0", "x > 0 |= !(x <= 0)",
        "x % 2 == 0 |= x != 1", "x / 2 > 1 |= x > 2", "x > y |= x >= y + 1", "2 * x == 3 |= false",
        "x > 0 && x < 1 |= false", "x > 0 && x < 2 |= x == 1", "false |= x > 0", "x > 0 |= true"),
    "proof_const": contracts(
        "1 + 1 == 2", "2 * 3 > 5 |= 10 / 2 == 5", "7 % 3 == 1", "!(1 > 2)", "true && false", "true || false",
        "1 / 0 == 0", "-1 < 0", "9223372036854775807 + 1 < 0", "0x10 == 16", "1 == 1 && 2 == 2 |= 3 != 3",
        "(1 + 2) * 3 == 9", "true == true", "true != false", "1 < 2 == true", "x > 0 |= 1 + 1 == 2",
        "5 - 10 < 0", "0b101 == 5", "1_000 == 1000", "10 % 0 == 0"),
    "proof_forms": contracts(
        "@result > 0", "x > 0 |= @result > 0", "x > 0 |= @result == x", "x > 0 && @entry(y) > 0 |= y > 0",
        "x > 0", "x > 0 && y > 0 && z > 0 |= z > 0", "(x > 0) |= (x > 0)", "x > 0 |= ((x) > (0))",
        "x as i64 > 0 |= x > 0", "x > 0 |= x as i64 > 0", "x > 0 && true |= x >= 1",
        "x >= 1 && x <= 3 && y >= x |= y >= 1", "x > 0 |= y > 0", "x == 1 || x == 2 |= x >= 1"),
    "proof_wide": contracts(
        "a > 0 |= a >= 1", "a < 255 |= a <= 254", "b > 0 |= b >= 1", "c >= 0", "a >= 0 |= a + 1 > 0",
        "c > 4294967295 |= c > 0", "d > 0.0 |= d >= 0.0", "e |= e", "e |= !e", "e && a > 0 |= a > 0",
        params="a: u8, b: i64, c: u64, d: f64, e: bool"),
}


def main() -> None:
    if OUT.exists():
        shutil.rmtree(OUT)
    entries = []
    for name, text in sorted(CASES.items()):
        case_dir = OUT / name
        (case_dir / "Source").mkdir(parents=True)
        (case_dir / "Ultraviolet.toml").write_text(APP, encoding="utf-8")
        (case_dir / "Source" / "Main.uv").write_text(text, encoding="utf-8", newline="")
        entries.append(f"target/relcases/{name}/Ultraviolet.toml")
    (ROOT / "tests" / "golden" / "relations_extra.list").write_text("\n".join(entries) + "\n", encoding="utf-8")
    print(f"wrote {len(entries)} relation cases")


if __name__ == "__main__":
    main()
