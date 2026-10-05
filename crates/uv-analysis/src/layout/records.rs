//! Records and tuples: fields in declaration order, each at the next offset its
//! alignment allows.

use super::*;

pub fn record_layout_of(ctx: &ScopeContext<'_>, fields: &[TypeRef], options: &RecordLayoutOptions) -> Option<RecordLayout> {
    let mut offsets = Vec::with_capacity(fields.len());
    let mut offset = 0;
    let mut max_align = 1;
    for field in fields {
        let field_layout = layout_of(ctx, field)?;
        let align = if options.packed { 1 } else { field_layout.align };
        offset = align_up(offset, align);
        offsets.push(offset);
        max_align = max_align.max(align);
        offset += field_layout.size;
    }
    if let (false, Some(min_align)) = (options.packed, options.min_align) {
        max_align = max_align.max(min_align);
    }
    Some(RecordLayout { layout: Layout { size: align_up(offset, max_align), align: max_align }, offsets })
}

pub fn tuple_layout_of(ctx: &ScopeContext<'_>, elems: &[TypeRef]) -> Option<RecordLayout> {
    record_layout_of(ctx, elems, &RecordLayoutOptions::default())
}
