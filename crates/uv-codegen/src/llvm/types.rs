//! The LLVM type of a type of the language (`LLVMEmitter::GetLLVMType` and its helpers).

use super::*;

/// `GetPrimType`.
pub(super) fn prim_type(name: &str) -> Ty {
    match name {
        "bool" | "i8" | "u8" => Ty::i8(),
        "char" | "i32" | "u32" => Ty::i32(),
        "i16" | "u16" => Ty::i16(),
        "i64" | "u64" | "usize" | "isize" => Ty::i64(),
        "i128" | "u128" => Ty::Int(128),
        "f16" => Ty::Half,
        "f32" => Ty::Float,
        "f64" => Ty::Double,
        "unit" | "()" | "never" | "!" => Ty::strukt(Vec::new()),
        _ => Ty::i8(),
    }
}

/// `GetIntTypeForSize`.
fn int_type_for_size(size: u64) -> Option<Ty> {
    match size {
        1 => Some(Ty::i8()),
        2 => Some(Ty::i16()),
        4 => Some(Ty::i32()),
        8 => Some(Ty::i64()),
        _ => None,
    }
}

/// `GetAlignmentMarkerType`.
fn alignment_marker(align: u64) -> Option<Ty> {
    match align {
        1 => Some(Ty::i8()),
        2 => Some(Ty::i16()),
        4 => Some(Ty::i32()),
        8 => Some(Ty::i64()),
        16 => Some(Ty::Int(128)),
        _ => None,
    }
}

fn pad(elements: &mut Vec<Ty>, bytes: u64) {
    if bytes > 0 {
        elements.push(Ty::array(bytes, Ty::i8()));
    }
}

fn align_up(value: u64, align: u64) -> u64 {
    if align == 0 || value.is_multiple_of(align) {
        value
    } else {
        value + (align - value % align)
    }
}

pub(super) fn slice_type() -> Ty {
    Ty::strukt(vec![Ty::Ptr, Ty::i64()])
}

pub(super) fn string_managed_type() -> Ty {
    Ty::strukt(vec![Ty::Ptr, Ty::i64(), Ty::i64()])
}

pub(super) fn dynamic_type() -> Ty {
    Ty::strukt(vec![Ty::Ptr, Ty::Ptr])
}

/// `CreateTaggedBlobType`.
pub(super) fn tagged_blob_type(size: u64, align: u64) -> Ty {
    if size == 0 {
        return Ty::strukt(Vec::new());
    }
    let mut fields = vec![Ty::array(size, Ty::i8())];
    if align > 1 {
        if let Some(marker) = alignment_marker(align) {
            fields.push(Ty::array(0, marker));
        }
    }
    Ty::strukt(fields)
}

/// `CreateTaggedABIType`.
pub(super) fn tagged_abi_type(size: u64, align: u64) -> Ty {
    int_type_for_size(size).unwrap_or_else(|| tagged_blob_type(size, align))
}

/// One field of a record in LLVM terms.
pub(super) struct LayoutField {
    pub llvm: Ty,
    pub offset: u64,
    pub size: u64,
}

/// `LayoutLLVMRecord`.
pub(super) struct LayoutRecord {
    pub fields: Vec<LayoutField>,
    pub size: u64,
    pub align: u64,
}

fn same_nominal_type(lhs: &TypeRef, rhs: &TypeRef) -> bool {
    let (Some(a), Some(b)) = (lhs, rhs) else {
        return false;
    };
    if Arc::ptr_eq(a, b) {
        return true;
    }
    if let (TypeNode::ModalState(l), TypeNode::ModalState(r)) = (&a.node, &b.node) {
        return path_eq(&l.path, &r.path) && id_eq(&l.state, &r.state) && args_eq(&l.generic_args, &r.generic_args);
    }
    match (applied_type_path(a), applied_type_path(b)) {
        (Some(lp), Some(rp)) if path_eq(lp, rp) => args_eq(applied_type_args(a).unwrap_or(&[]), applied_type_args(b).unwrap_or(&[])),
        _ => false,
    }
}

fn path_eq(lhs: &[String], rhs: &[String]) -> bool {
    lhs.len() == rhs.len() && lhs.iter().zip(rhs).all(|(a, b)| id_eq(a, b))
}

fn args_eq(lhs: &[TypeRef], rhs: &[TypeRef]) -> bool {
    lhs.len() == rhs.len() && lhs.iter().zip(rhs).all(|(a, b)| type_equiv(a, b))
}

fn build_subst(generic_params: &Option<ast::GenericParams>, args: &[TypeRef]) -> Option<TypeSubst> {
    match generic_params {
        Some(params) if !params.params.is_empty() => {
            if args.len() > params.params.len() {
                None
            } else {
                Some(build_substitution(&params.params, args))
            }
        }
        _ => Some(TypeSubst::new()),
    }
}

fn lower_with_subst(scope: &ScopeContext<'_>, written: &Option<Arc<ast::Type>>, subst: &TypeSubst) -> Option<TypeRef> {
    let lowered = lower_type_for_layout(scope, written)?;
    Some(if subst.is_empty() { lowered } else { instantiate_type(&lowered, subst) })
}

/// `TypeContainsByValue`: whether `candidate` holds `needle` by value, directly or through
/// the fields of what it is made of.
fn contains_by_value(scope: &ScopeContext<'_>, candidate: &TypeRef, needle: &TypeRef, active: &mut Vec<String>) -> bool {
    let (Some(cand), Some(_)) = (candidate, needle) else {
        return false;
    };
    if same_nominal_type(candidate, needle) {
        return true;
    }
    let key = type_to_string(candidate);
    if active.contains(&key) {
        return false;
    }
    active.push(key);
    let result = match &cand.node {
        TypeNode::Perm { base, .. } | TypeNode::Refine { base, .. } => contains_by_value(scope, base, needle, active),
        TypeNode::Union(members) => members.iter().any(|member| contains_by_value(scope, member, needle, active)),
        TypeNode::Tuple(elements) => elements.iter().any(|element| contains_by_value(scope, element, needle, active)),
        TypeNode::Array { element, .. } => contains_by_value(scope, element, needle, active),
        TypeNode::ModalState(modal) => 'modal: {
            let Some(decl) = lookup_modal_decl(scope, &modal.path) else {
                break 'modal false;
            };
            let Some(subst) = build_subst(&decl.generic_params, &modal.generic_args) else {
                break 'modal false;
            };
            for state in &decl.states {
                if !id_eq(&state.name, &modal.state) {
                    continue;
                }
                for member in &state.members {
                    if let ast::StateMember::StateFieldDecl(field) = member {
                        if let Some(lowered) = lower_with_subst(scope, &field.r#type, &subst) {
                            if contains_by_value(scope, &lowered, needle, active) {
                                break 'modal true;
                            }
                        }
                    }
                }
                break 'modal false;
            }
            false
        }
        TypeNode::Path { path, .. } | TypeNode::Apply { path, .. } => 'nominal: {
            let Some(decl) = lookup_type_decl(scope, path) else {
                break 'nominal false;
            };
            let args = applied_type_args(cand).unwrap_or(&[]).to_vec();
            match decl {
                TypeDecl::TypeAlias(alias) => {
                    let Some(subst) = build_subst(&alias.generic_params, &args) else { break 'nominal false };
                    lower_with_subst(scope, &alias.r#type, &subst).is_some_and(|lowered| contains_by_value(scope, &lowered, needle, active))
                }
                TypeDecl::Record(record) => {
                    let Some(subst) = build_subst(&record.generic_params, &args) else { break 'nominal false };
                    record.members.iter().any(|member| match member {
                        ast::RecordMember::FieldDecl(field) => lower_with_subst(scope, &field.r#type, &subst).is_some_and(|lowered| contains_by_value(scope, &lowered, needle, active)),
                        _ => false,
                    })
                }
                TypeDecl::Modal(modal) => {
                    let Some(subst) = build_subst(&modal.generic_params, &args) else { break 'nominal false };
                    modal.states.iter().any(|state| {
                        state.members.iter().any(|member| match member {
                            ast::StateMember::StateFieldDecl(field) => lower_with_subst(scope, &field.r#type, &subst).is_some_and(|lowered| contains_by_value(scope, &lowered, needle, active)),
                            _ => false,
                        })
                    })
                }
                TypeDecl::Enum(_) => false,
            }
        }
        _ => false,
    };
    active.pop();
    result
}

impl<'e, 'a, 'b> Emitter<'e, 'a, 'b> {
    /// `GetOpaquePtr`.
    pub(super) fn opaque_ptr(&self) -> Ty {
        Ty::Ptr
    }

    /// `ComputeStructElements(fields, offsets, total_size, required_align)`.
    fn struct_elements_by_offsets(&mut self, fields: &[TypeRef], offsets: &[u64], total_size: u64, required_align: u64) -> Vec<Ty> {
        let mut elements = Vec::new();
        if fields.len() != offsets.len() {
            return elements;
        }
        let mut prev_end = 0;
        let mut natural_align = 1;
        for (field, offset) in fields.iter().zip(offsets) {
            if *offset > prev_end {
                pad(&mut elements, offset - prev_end);
            }
            elements.push(self.llvm_type(field));
            let scope = &self.ctx.scope;
            let size = size_of(scope, field);
            let align = align_of(scope, field);
            prev_end = match size {
                Some(size) => offset + size,
                None => *offset,
            };
            if let Some(align) = align {
                natural_align = natural_align.max(align);
            }
        }
        if total_size > prev_end {
            pad(&mut elements, total_size - prev_end);
        }
        if required_align > natural_align {
            if let Some(marker) = alignment_marker(required_align) {
                elements.push(Ty::array(0, marker));
            }
        }
        elements
    }

    /// `ComputeLayoutLLVMRecord`.
    pub(super) fn layout_record(&mut self, aggregate: &TypeRef, fields: &[TypeRef], options: &RecordLayoutOptions) -> Option<LayoutRecord> {
        let mut out = Vec::new();
        let mut offset = 0;
        let mut max_align = 1;
        for (index, field) in fields.iter().enumerate() {
            let recursive_indirect = aggregate.is_some() && {
                let mut active = Vec::new();
                contains_by_value(&self.ctx.scope, field, aggregate, &mut active)
            };
            let (layout, llvm) = if recursive_indirect {
                let ptr_size = self.ctx.scope.target_profile.map(ptr_size_bytes).unwrap_or(8) as u64;
                (Layout { size: ptr_size, align: ptr_size }, Ty::Ptr)
            } else {
                (layout_of(&self.ctx.scope, field)?, self.llvm_type(field))
            };
            let align = if options.packed { 1 } else { layout.align };
            if index != 0 {
                offset = align_up(offset, align);
            }
            out.push(LayoutField { llvm, offset, size: layout.size });
            max_align = max_align.max(align);
            offset += layout.size;
        }
        if options.packed {
            max_align = 1;
        } else if let Some(min) = options.min_align {
            max_align = max_align.max(min);
        }
        let mut size = align_up(offset, max_align);
        let mut align = max_align;
        if aggregate.is_some() {
            if let Some(whole) = layout_of(&self.ctx.scope, aggregate) {
                if whole.size >= size {
                    size = whole.size;
                    align = align.max(whole.align);
                }
            }
        }
        Some(LayoutRecord { fields: out, size, align })
    }

    /// `ComputeStructElements(layout)`.
    fn record_elements(layout: &LayoutRecord) -> Vec<Ty> {
        let mut elements = Vec::new();
        let mut prev_end = 0;
        for field in &layout.fields {
            if field.offset > prev_end {
                pad(&mut elements, field.offset - prev_end);
            }
            elements.push(field.llvm.clone());
            prev_end = field.offset + field.size;
        }
        if layout.size > prev_end {
            pad(&mut elements, layout.size - prev_end);
        }
        if layout.align > 1 {
            if let Some(marker) = alignment_marker(layout.align) {
                elements.push(Ty::array(0, marker));
            }
        }
        elements
    }

    /// `CreateTaggedStructType`: the discriminant, the payload as bytes, and padding.
    fn tagged_struct(&mut self, disc_type: &TypeRef, payload_size: u64, payload_align: u64, total_size: u64) -> Ty {
        let mut elements = vec![self.llvm_type(disc_type)];
        let disc_size = size_of(&self.ctx.scope, disc_type).unwrap_or(1);
        let payload_off = align_up(disc_size, payload_align);
        pad(&mut elements, payload_off - disc_size);
        elements.push(Ty::array(payload_size, Ty::i8()));
        let payload_end = payload_off + payload_size;
        if total_size > payload_end {
            pad(&mut elements, total_size - payload_end);
        }
        if payload_align > 1 {
            if let Some(marker) = alignment_marker(payload_align) {
                elements.push(Ty::array(0, marker));
            }
        }
        Ty::strukt(elements)
    }

    /// `GetLLVMType`.
    pub(super) fn llvm_type(&mut self, ty: &TypeRef) -> Ty {
        let Some(node) = ty.as_deref() else {
            self.fail("a type that is not known");
            return Ty::Void;
        };
        let key = type_to_string(ty);
        if let Some(cached) = self.type_cache.get(&key) {
            return cached.clone();
        }
        if self.active_types.contains(&key) {
            return Ty::Ptr;
        }
        self.active_types.push(key.clone());
        let mapped = self.map_type(ty, &node.node);
        self.active_types.pop();
        if let Some(mapped) = &mapped {
            self.type_cache.insert(key, mapped.clone());
        }
        mapped.unwrap_or_else(|| {
            self.fail(&format!("the LLVM type of {}", type_to_string(ty)));
            Ty::i8()
        })
    }

    fn map_type(&mut self, ty: &TypeRef, node: &TypeNode) -> Option<Ty> {
        if async_sig_of(&self.ctx.scope, ty).is_some() {
            self.fail("async types");
            return Some(Ty::strukt(Vec::new()));
        }
        match node {
            TypeNode::Prim(name) => Some(prim_type(name)),
            TypeNode::Perm { base, .. } | TypeNode::Refine { base, .. } => Some(self.llvm_type(base)),
            TypeNode::Opaque { class_path, .. } => {
                let underlying = self.ctx.scope.sigma.opaque_underlying_by_class_path.get(&path_key_of(class_path)).and_then(|found| found.clone());
                match underlying {
                    Some(under) if !Arc::ptr_eq(&under, ty.as_ref()?) => Some(self.llvm_type(&Some(under))),
                    _ => None,
                }
            }
            TypeNode::Ptr { .. } | TypeNode::RawPtr { .. } | TypeNode::Func { .. } => Some(Ty::Ptr),
            TypeNode::Closure { .. } => Some(Ty::strukt(vec![Ty::Ptr, Ty::Ptr])),
            TypeNode::Tuple(elements) => {
                let layout = tuple_layout_of(&self.ctx.scope, elements);
                let elems = match layout {
                    Some(layout) => self.struct_elements_by_offsets(elements, &layout.offsets, layout.layout.size, 1),
                    None => Vec::new(),
                };
                Some(Ty::strukt(elems))
            }
            TypeNode::Union(members) => self.map_union(members),
            TypeNode::Path { path, generic_args } => self.map_nominal(ty, path, generic_args),
            TypeNode::Apply { path, args } => self.map_nominal(ty, path, args),
            TypeNode::ModalState(modal) => self.map_modal_state(ty, modal),
            TypeNode::Array { element, length, .. } => {
                let element = self.llvm_type(element);
                Some(Ty::array(*length, element))
            }
            TypeNode::Slice(_) => Some(slice_type()),
            TypeNode::String(state) => Some(self.map_string_or_bytes(matches!(state, Some(StringState::View)), matches!(state, Some(StringState::Managed)))),
            TypeNode::Bytes(state) => Some(self.map_string_or_bytes(matches!(state, Some(BytesState::View)), matches!(state, Some(BytesState::Managed)))),
            TypeNode::Dynamic(_) => Some(dynamic_type()),
            TypeNode::Range(_) | TypeNode::RangeInclusive(_) | TypeNode::RangeFrom(_) | TypeNode::RangeTo(_) | TypeNode::RangeToInclusive(_) | TypeNode::RangeFull => self.map_range(node),
            TypeNode::Var(_) => None,
        }
    }

    fn map_string_or_bytes(&mut self, view: bool, managed: bool) -> Ty {
        if view {
            return slice_type();
        }
        if managed {
            return string_managed_type();
        }
        let payload_size = 3 * 8;
        let payload_align = 8;
        let payload_off = align_up(1, payload_align);
        let total = align_up(payload_off + payload_size, payload_align);
        self.tagged_struct(&make_type_prim("u8"), payload_size, payload_align, total)
    }

    fn map_union(&mut self, uni: &[TypeRef]) -> Option<Ty> {
        let layout = union_layout_of(&self.ctx.scope, uni)?;
        if layout.niche {
            let payload = layout.member_list.iter().find(|member| {
                let member: &TypeRef = member;
                let stripped = strip_perm(member).or_else(|| member.clone());
                !matches!(stripped.as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(name)) if name == "()")
            });
            return Some(match payload {
                Some(member) => self.llvm_type(&member.clone()),
                None => Ty::i8(),
            });
        }
        let disc = make_type_prim(layout.disc_type.as_deref().unwrap_or("u8"));
        Some(self.tagged_struct(&disc, layout.payload_size, layout.payload_align, layout.layout.size))
    }

    fn map_nominal(&mut self, ty: &TypeRef, path: &[String], generic_args: &[TypeRef]) -> Option<Ty> {
        if is_builtin_runtime_handle_modal_type_path(path) {
            return Some(Ty::Ptr);
        }
        if is_async_type(ty) {
            self.fail("async types");
            return Some(Ty::strukt(Vec::new()));
        }
        // Types written as `GpuPtr<T, Global>` are pointers.
        if path.len() == 1 && id_eq(&path[0], "GpuPtr") && generic_args.len() == 2 {
            return Some(Ty::Ptr);
        }
        let scope_decl = lookup_type_decl(&self.ctx.scope, path).cloned();
        match scope_decl {
            Some(TypeDecl::Record(record)) => {
                let subst = build_subst(&record.generic_params, generic_args)?;
                let mut fields = Vec::new();
                for member in &record.members {
                    if let ast::RecordMember::FieldDecl(field) = member {
                        let lowered = lower_type_for_layout(&self.ctx.scope, &field.r#type);
                        fields.push(match lowered {
                            Some(lowered) if subst.is_empty() => lowered,
                            Some(lowered) => instantiate_type(&lowered, &subst),
                            None => make_type_prim("u8"),
                        });
                    }
                }
                let options = resolve_record_layout_options(&record.attrs);
                match self.layout_record(ty, &fields, &options) {
                    Some(layout) => Some(Ty::Struct { fields: Self::record_elements(&layout), packed: options.packed }),
                    None => Some(Ty::strukt(Vec::new())),
                }
            }
            Some(TypeDecl::Enum(decl)) => {
                let options = resolve_enum_layout_options(&decl.attrs);
                let layout = enum_layout_of(&self.ctx.scope, &decl, generic_args, &options)?;
                let disc = make_type_prim(&layout.disc_type);
                if layout.payload_size == 0 {
                    Some(self.llvm_type(&disc))
                } else {
                    Some(self.tagged_struct(&disc, layout.payload_size, layout.payload_align, layout.layout.size))
                }
            }
            Some(TypeDecl::TypeAlias(alias)) => {
                let lowered = lower_type_for_layout(&self.ctx.scope, &alias.r#type)?;
                let inst = match alias.generic_params.as_ref().filter(|params| !params.params.is_empty()) {
                    Some(params) => {
                        if generic_args.len() > params.params.len() {
                            return None;
                        }
                        instantiate_type(&lowered, &build_substitution(&params.params, generic_args))
                    }
                    None => lowered,
                };
                Some(self.llvm_type(&inst))
            }
            Some(TypeDecl::Modal(modal)) => {
                let layout = modal_layout_of(&self.ctx.scope, &modal, generic_args)?;
                if let Some(disc) = &layout.disc_type {
                    return Some(self.tagged_struct(&make_type_prim(disc), layout.payload_size, layout.payload_align, layout.layout.size));
                }
                // A modal without a discriminant is its one payload field's type.
                let subst = build_subst(&modal.generic_params, generic_args)?;
                let payload_state = payload_state(&self.ctx.scope, &modal).map(str::to_string);
                let mut payload: TypeRef = None;
                for state in &modal.states {
                    if payload_state.as_ref().is_some_and(|name| !id_eq(&state.name, name)) {
                        continue;
                    }
                    let mut candidate = None;
                    let mut count = 0;
                    for member in &state.members {
                        if let ast::StateMember::StateFieldDecl(field) = member {
                            candidate = Some(field);
                            count += 1;
                        }
                    }
                    let (Some(field), 1) = (candidate, count) else { continue };
                    let Some(lowered) = lower_type_for_layout(&self.ctx.scope, &field.r#type) else { continue };
                    payload = if subst.is_empty() { lowered } else { instantiate_type(&lowered, &subst) };
                    break;
                }
                Some(match &payload {
                    Some(_) => self.llvm_type(&payload),
                    None => Ty::i8(),
                })
            }
            None => {
                if let Some(builtin) = lookup_builtin_modal_layout(path) {
                    let disc = make_type_prim(builtin.disc_prim);
                    return Some(self.tagged_struct(&disc, builtin.payload_size, builtin.payload_align, builtin.size));
                }
                None
            }
        }
    }

    fn map_modal_state(&mut self, ty: &TypeRef, modal_state: &TypeModalState) -> Option<Ty> {
        if is_builtin_runtime_handle_modal_type_path(&modal_state.path) {
            return Some(Ty::Ptr);
        }
        if is_async_type(ty) {
            self.fail("async types");
            return Some(Ty::strukt(Vec::new()));
        }
        if let Some(builtin) = lookup_builtin_modal_layout(&modal_state.path) {
            let disc = make_type_prim(builtin.disc_prim);
            return Some(self.tagged_struct(&disc, builtin.payload_size, builtin.payload_align, builtin.size));
        }
        let decl = lookup_modal_decl(&self.ctx.scope, &modal_state.path).cloned();
        let mut mapped = None;
        if let Some(decl) = &decl {
            let subst = build_subst(&decl.generic_params, &modal_state.generic_args)?;
            if let Some(state) = decl.states.iter().find(|state| id_eq(&state.name, &modal_state.state)) {
                let mut fields = Vec::new();
                for member in &state.members {
                    if let ast::StateMember::StateFieldDecl(field) = member {
                        fields.push(match lower_type_for_layout(&self.ctx.scope, &field.r#type) {
                            Some(lowered) if subst.is_empty() => lowered,
                            Some(lowered) => instantiate_type(&lowered, &subst),
                            None => make_type_prim("u8"),
                        });
                    }
                }
                mapped = Some(match self.layout_record(ty, &fields, &RecordLayoutOptions::default()) {
                    Some(layout) => Ty::strukt(Self::record_elements(&layout)),
                    None => Ty::strukt(Vec::new()),
                });
            }
            if mapped.is_none() {
                if let Some(layout) = modal_layout_of(&self.ctx.scope, decl, &modal_state.generic_args) {
                    mapped = Some(match &layout.disc_type {
                        Some(disc) => self.tagged_struct(&make_type_prim(disc), layout.payload_size, layout.payload_align, layout.layout.size),
                        None => Ty::array(layout.layout.size, Ty::i8()),
                    });
                }
            }
        }
        mapped
    }

    fn map_range(&mut self, node: &TypeNode) -> Option<Ty> {
        let fields: Vec<TypeRef> = match node {
            TypeNode::Range(base) | TypeNode::RangeInclusive(base) => vec![base.clone(), base.clone()],
            TypeNode::RangeFrom(base) | TypeNode::RangeTo(base) | TypeNode::RangeToInclusive(base) => vec![base.clone()],
            _ => Vec::new(),
        };
        if fields.is_empty() {
            return Some(Ty::strukt(Vec::new()));
        }
        Some(match record_layout_of(&self.ctx.scope, &fields, &RecordLayoutOptions::default()) {
            Some(layout) => {
                let elems = self.struct_elements_by_offsets(&fields, &layout.offsets, layout.layout.size, 1);
                Ty::strukt(elems)
            }
            None => {
                let elems: Vec<Ty> = fields.iter().map(|field| self.llvm_type(field)).collect();
                Ty::strukt(elems)
            }
        })
    }
}
