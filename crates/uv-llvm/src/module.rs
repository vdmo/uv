//! The module: types, globals and functions, and their text.

use std::collections::HashMap;
use std::fmt::Write as _;

use crate::types::{quote, FnTy, Ty};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Linkage {
    #[default]
    External,
    Internal,
    Private,
    LinkOnceOdr,
    WeakOdr,
    Weak,
    Common,
    ExternWeak,
}

impl Linkage {
    fn text(self) -> &'static str {
        match self {
            Linkage::External => "",
            Linkage::Internal => "internal ",
            Linkage::Private => "private ",
            Linkage::LinkOnceOdr => "linkonce_odr ",
            Linkage::WeakOdr => "weak_odr ",
            Linkage::Weak => "weak ",
            Linkage::Common => "common ",
            Linkage::ExternWeak => "extern_weak ",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CallConv {
    #[default]
    C,
    Fast,
    Win64,
    X86_64SysV,
}

impl CallConv {
    pub(crate) fn text(self) -> &'static str {
        match self {
            CallConv::C => "",
            CallConv::Fast => "fastcc ",
            CallConv::Win64 => "win64cc ",
            CallConv::X86_64SysV => "x86_64_sysvcc ",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuncAttr {
    AlwaysInline,
    NoInline,
    Cold,
    NoReturn,
    NoUnwind,
    OptNone,
    Other(String),
}

impl Function {
    /// `ValueSymbolTable::makeUniqueName`: the name, or the name with the next number of the
    /// function's counter appended until nothing has it.
    pub fn unique_name(&mut self, name: &str) -> String {
        if self.names.insert(name.to_string()) {
            return name.to_string();
        }
        loop {
            self.last_unique += 1;
            let candidate = format!("{name}{}", self.last_unique);
            if self.names.insert(candidate.clone()) {
                return candidate;
            }
        }
    }
}

impl FuncAttr {
    fn text(&self) -> String {
        match self {
            FuncAttr::AlwaysInline => "alwaysinline".to_string(),
            FuncAttr::NoInline => "noinline".to_string(),
            FuncAttr::Cold => "cold".to_string(),
            FuncAttr::NoReturn => "noreturn".to_string(),
            FuncAttr::NoUnwind => "nounwind".to_string(),
            FuncAttr::OptNone => "optnone".to_string(),
            FuncAttr::Other(text) => text.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParamAttr {
    SRet(Ty),
    ByVal(Ty),
    NoAlias,
    NonNull,
    NoCapture,
    ReadOnly,
    ZExt,
    SExt,
    NoUndef,
    Align(u32),
    Dereferenceable(u64),
    WriteOnly,
    CapturesNone,
}

impl ParamAttr {
    /// The place LLVM's printer gives an attribute: attributes are printed in the order of
    /// their kinds, whatever order they were added in.
    pub fn rank(&self) -> u8 {
        match self {
            ParamAttr::NoAlias => 0,
            ParamAttr::NoUndef => 1,
            ParamAttr::NonNull => 2,
            ParamAttr::SRet(_) => 3,
            ParamAttr::ByVal(_) => 4,
            ParamAttr::ReadOnly => 5,
            ParamAttr::WriteOnly => 6,
            ParamAttr::CapturesNone => 7,
            ParamAttr::ZExt => 8,
            ParamAttr::SExt => 9,
            ParamAttr::NoCapture => 10,
            ParamAttr::Align(_) => 11,
            ParamAttr::Dereferenceable(_) => 12,
        }
    }

    pub(crate) fn text(&self) -> String {
        match self {
            ParamAttr::SRet(ty) => format!("sret({ty})"),
            ParamAttr::ByVal(ty) => format!("byval({ty})"),
            ParamAttr::NoAlias => "noalias".to_string(),
            ParamAttr::NonNull => "nonnull".to_string(),
            ParamAttr::NoCapture => "nocapture".to_string(),
            ParamAttr::ReadOnly => "readonly".to_string(),
            ParamAttr::ZExt => "zeroext".to_string(),
            ParamAttr::SExt => "signext".to_string(),
            ParamAttr::NoUndef => "noundef".to_string(),
            ParamAttr::Align(align) => format!("align {align}"),
            ParamAttr::Dereferenceable(bytes) => format!("dereferenceable({bytes})"),
            ParamAttr::WriteOnly => "writeonly".to_string(),
            ParamAttr::CapturesNone => "captures(none)".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlobalId(pub(crate) usize);

#[derive(Debug, Clone)]
pub(crate) struct Global {
    pub name: String,
    pub ty: Ty,
    pub init: Option<String>,
    pub constant: bool,
    pub linkage: Linkage,
    pub align: Option<u64>,
    pub section: Option<String>,
    pub unnamed_addr: bool,
    pub hidden: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct Block {
    pub name: String,
    pub insts: Vec<String>,
    pub terminated: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct Function {
    pub name: String,
    pub ty: FnTy,
    pub linkage: Linkage,
    pub cc: CallConv,
    pub attrs: Vec<FuncAttr>,
    pub param_attrs: Vec<Vec<ParamAttr>>,
    pub ret_attrs: Vec<ParamAttr>,
    pub blocks: Vec<Block>,
    pub hidden: bool,
    pub next_reg: usize,
    /// The names taken in the function and the counter that makes colliding ones unique, as
    /// LLVM's symbol table does (one counter for blocks, arguments and values).
    pub names: std::collections::HashSet<String>,
    pub last_unique: usize,
    pub param_names: Vec<Option<String>>,
    pub removed: bool,
}

/// A module of LLVM IR.
#[derive(Debug, Clone, Default)]
pub struct Module {
    pub name: String,
    pub triple: String,
    pub data_layout: String,
    pub(crate) named_types: Vec<(String, Ty)>,
    pub(crate) globals: Vec<Global>,
    pub(crate) global_index: HashMap<String, GlobalId>,
    pub(crate) funcs: Vec<Function>,
    pub(crate) func_index: HashMap<String, usize>,
    /// `@llvm.global_ctors` entries: priority and function.
    pub(crate) ctors: Vec<(u32, String)>,
    pub(crate) dtors: Vec<(u32, String)>,
    /// `@llvm.used` entries (globals or functions kept).
    pub(crate) used: Vec<String>,
    /// Whether the constructor and destructor lists are hidden.
    pub(crate) ctor_lists_hidden: bool,
}

impl Module {
    pub fn new(name: &str, triple: &str, data_layout: &str) -> Module {
        Module { name: name.to_string(), triple: triple.to_string(), data_layout: data_layout.to_string(), ..Default::default() }
    }

    /// Declares the body of a named struct.
    pub fn define_named_type(&mut self, name: &str, body: Ty) -> Ty {
        if !self.named_types.iter().any(|(existing, _)| existing == name) {
            self.named_types.push((name.to_string(), body));
        }
        Ty::Named(name.to_string())
    }

    pub fn named_type_body(&self, name: &str) -> Option<&Ty> {
        self.named_types.iter().find(|(existing, _)| existing == name).map(|(_, body)| body)
    }

    pub fn has_function(&self, name: &str) -> bool {
        self.func_index.contains_key(name)
    }

    pub fn function_type(&self, name: &str) -> Option<&FnTy> {
        self.func_index.get(name).map(|index| &self.funcs[*index].ty)
    }

    pub fn function_is_declaration(&self, name: &str) -> bool {
        self.func_index.get(name).is_none_or(|index| self.funcs[*index].blocks.is_empty())
    }

    pub fn global(&self, name: &str) -> Option<GlobalId> {
        self.global_index.get(name).copied()
    }

    pub fn global_type(&self, id: GlobalId) -> &Ty {
        &self.globals[id.0].ty
    }

    /// Adds a global variable, or returns the one of that name.
    pub fn add_global(&mut self, name: &str, ty: Ty, init: Option<String>, constant: bool, linkage: Linkage) -> GlobalId {
        if let Some(existing) = self.global_index.get(name) {
            return *existing;
        }
        let id = GlobalId(self.globals.len());
        self.globals.push(Global { name: name.to_string(), ty, init, constant, linkage, align: None, section: None, unnamed_addr: false, hidden: false });
        self.global_index.insert(name.to_string(), id);
        id
    }

    pub fn set_global_init(&mut self, id: GlobalId, init: Option<String>) {
        self.globals[id.0].init = init;
    }

    pub fn set_global_align(&mut self, id: GlobalId, align: u64) {
        self.globals[id.0].align = Some(align);
    }

    pub fn set_global_section(&mut self, id: GlobalId, section: &str) {
        self.globals[id.0].section = Some(section.to_string());
    }

    pub fn set_global_hidden(&mut self, id: GlobalId) {
        self.globals[id.0].hidden = true;
    }

    pub fn set_ctor_lists_hidden(&mut self) {
        self.ctor_lists_hidden = true;
    }

    /// The names of the globals and of the defined functions that are not local.
    pub fn defined_non_local_symbols(&self) -> (Vec<String>, Vec<String>) {
        let globals = self.globals.iter().filter(|global| global.init.is_some() && !matches!(global.linkage, Linkage::Internal | Linkage::Private)).map(|global| global.name.clone()).collect();
        let funcs = self.funcs.iter().filter(|func| !func.blocks.is_empty() && !matches!(func.linkage, Linkage::Internal | Linkage::Private)).map(|func| func.name.clone()).collect();
        (globals, funcs)
    }

    pub fn set_function_hidden_by_name(&mut self, name: &str) {
        if let Some(index) = self.func_index.get(name) {
            self.funcs[*index].hidden = true;
        }
    }

    pub fn set_global_hidden_by_name(&mut self, name: &str) {
        if let Some(id) = self.global_index.get(name) {
            self.globals[id.0].hidden = true;
        }
    }

    pub fn set_global_unnamed_addr(&mut self, id: GlobalId) {
        self.globals[id.0].unnamed_addr = true;
    }

    pub fn set_global_linkage(&mut self, id: GlobalId, linkage: Linkage) {
        self.globals[id.0].linkage = linkage;
    }

    pub fn add_ctor(&mut self, priority: u32, function: &str) {
        self.ctors.push((priority, function.to_string()));
    }

    pub fn add_dtor(&mut self, priority: u32, function: &str) {
        self.dtors.push((priority, function.to_string()));
    }

    pub fn add_used(&mut self, symbol: &str) {
        self.used.push(symbol.to_string());
    }

    /// The text of the module.
    pub fn print(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "; ModuleID = '{}'", self.name);
        let _ = writeln!(out, "source_filename = \"{}\"", self.name);
        if !self.data_layout.is_empty() {
            let _ = writeln!(out, "target datalayout = \"{}\"", self.data_layout);
        }
        if !self.triple.is_empty() {
            let _ = writeln!(out, "target triple = \"{}\"", self.triple);
        }
        out.push('\n');
        for (name, body) in &self.named_types {
            let _ = writeln!(out, "%{} = type {}", quote(name), body);
        }
        if !self.named_types.is_empty() {
            out.push('\n');
        }
        for global in &self.globals {
            let _ = write!(out, "@{} = ", quote(&global.name));
            if global.init.is_none() {
                out.push_str("external ");
            } else {
                out.push_str(global.linkage.text());
            }
            if global.hidden {
                out.push_str("hidden ");
            }
            if global.unnamed_addr {
                out.push_str("unnamed_addr ");
            }
            out.push_str(if global.constant { "constant" } else { "global" });
            match &global.init {
                Some(init) => {
                    let _ = write!(out, " {} {init}", global.ty);
                }
                None => {
                    let _ = write!(out, " {}", global.ty);
                }
            }
            if let Some(section) = &global.section {
                let _ = write!(out, ", section \"{section}\"");
            }
            if let Some(align) = global.align {
                let _ = write!(out, ", align {align}");
            }
            out.push('\n');
        }
        if !self.globals.is_empty() {
            out.push('\n');
        }
        let mut groups: Vec<String> = Vec::new();
        for func in self.funcs.iter().filter(|func| !func.removed) {
            let group = if func.attrs.is_empty() {
                None
            } else {
                let mut texts: Vec<String> = func.attrs.iter().map(FuncAttr::text).collect();
                texts.sort();
                let text = texts.join(" ");
                Some(match groups.iter().position(|known| *known == text) {
                    Some(index) => index,
                    None => {
                        groups.push(text);
                        groups.len() - 1
                    }
                })
            };
            self.print_function(&mut out, func, group);
        }
        self.print_list(&mut out, "llvm.global_ctors", &self.ctors);
        self.print_list(&mut out, "llvm.global_dtors", &self.dtors);
        if !self.used.is_empty() {
            let entries: Vec<String> = self.used.iter().map(|symbol| format!("ptr @{}", quote(symbol))).collect();
            let _ = writeln!(out, "@llvm.used = appending global [{} x ptr] [{}], section \"llvm.metadata\"", entries.len(), entries.join(", "));
        }
        if !groups.is_empty() {
            out.push('\n');
        }
        for (index, text) in groups.iter().enumerate() {
            let _ = writeln!(out, "attributes #{index} = {{ {text} }}");
        }
        out
    }

    fn print_list(&self, out: &mut String, name: &str, entries: &[(u32, String)]) {
        if entries.is_empty() {
            return;
        }
        let items: Vec<String> = entries.iter().map(|(priority, function)| format!("{{ i32, ptr, ptr }} {{ i32 {priority}, ptr @{}, ptr null }}", quote(function))).collect();
        let hidden = if self.ctor_lists_hidden { "hidden " } else { "" };
        let _ = writeln!(out, "@{name} = appending {hidden}global [{} x {{ i32, ptr, ptr }}] [{}]", items.len(), items.join(", "));
    }

    fn print_function(&self, out: &mut String, func: &Function, group: Option<usize>) {
        let defined = !func.blocks.is_empty();
        out.push_str(if defined { "define " } else { "declare " });
        out.push_str(func.linkage.text());
        if func.hidden {
            out.push_str("hidden ");
        }
        out.push_str(func.cc.text());
        let mut ret_attrs: Vec<&ParamAttr> = func.ret_attrs.iter().collect();
        ret_attrs.sort_by_key(|attr| attr.rank());
        for attr in ret_attrs {
            let _ = write!(out, "{} ", attr.text());
        }
        let _ = write!(out, "{} @{}(", func.ty.ret, quote(&func.name));
        for (index, param) in func.ty.params.iter().enumerate() {
            if index > 0 {
                out.push_str(", ");
            }
            let _ = write!(out, "{param}");
            let mut attrs: Vec<&ParamAttr> = func.param_attrs.get(index).into_iter().flatten().collect();
            attrs.sort_by_key(|attr| attr.rank());
            for attr in attrs {
                let _ = write!(out, " {}", attr.text());
            }
            if defined {
                match func.param_names.get(index).and_then(Option::as_ref) {
                    Some(name) => {
                        let _ = write!(out, " %{}", quote(name));
                    }
                    None => {
                        // Unnamed arguments are numbered among themselves.
                        let number = func.param_names[..index].iter().filter(|name| name.is_none()).count();
                        let _ = write!(out, " %{number}");
                    }
                }
            }
        }
        if func.ty.vararg {
            out.push_str(if func.ty.params.is_empty() { "..." } else { ", ..." });
        }
        out.push(')');
        if let Some(group) = group {
            let _ = write!(out, " #{group}");
        }
        if !defined {
            out.push_str("\n\n");
            return;
        }
        out.push_str(" {\n");
        for block in &func.blocks {
            let _ = writeln!(out, "{}:", quote(&block.name));
            for inst in &block.insts {
                let _ = writeln!(out, "  {inst}");
            }
        }
        out.push_str("}\n\n");
    }
}
