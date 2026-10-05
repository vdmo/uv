//! Unions: a discriminant and the largest member, or a niche.

use super::*;
use crate::context::TypeDecl;

/// How many invalid values a type has that could encode other members: a valid pointer
/// has one (null). Permissions and type aliases are looked through; an alias defined
/// through itself has none.
fn niche_count(ctx: &ScopeContext<'_>, ty: &TypeRef, aliases: &mut Vec<Vec<String>>) -> u64 {
    match ty.as_deref().map(|ty| &ty.node) {
        Some(TypeNode::Perm { base, .. }) => niche_count(ctx, base, aliases),
        Some(TypeNode::Ptr { state: Some(PtrState::Valid), .. }) => 1,
        Some(TypeNode::Path { path, .. }) => match ctx.sigma.types.get(path) {
            Some(TypeDecl::TypeAlias(alias)) if !aliases.contains(path) => {
                aliases.push(path.clone());
                lower_type_for_layout(ctx, &alias.r#type).map_or(0, |lowered| niche_count(ctx, &lowered, aliases))
            }
            _ => 0,
        },
        _ => 0,
    }
}

fn is_unit_prim(ty: &TypeRef) -> bool {
    matches!(ty.as_deref().map(|ty| &ty.node), Some(TypeNode::Prim(name)) if name == "()")
}

/// A union whose only non-unit member has enough niches is laid out as that member.
pub fn union_layout_of(ctx: &ScopeContext<'_>, members: &[TypeRef]) -> Option<UnionLayout> {
    if members.is_empty() {
        return None;
    }
    let member_list = sort_union_members(members);
    let required = member_list.len().saturating_sub(1) as u64;
    let mut with_niche = member_list.iter().enumerate().filter_map(|(index, member)| {
        let count = if is_unit_prim(member) { 0 } else { niche_count(ctx, member, &mut Vec::new()) };
        (count != 0).then_some((index, count))
    });
    let payload = match (with_niche.next(), with_niche.next()) {
        (Some((index, count)), None) => {
            let others_unit = member_list.iter().enumerate().all(|(i, member)| i == index || is_unit_prim(member));
            (others_unit && count >= required).then_some(index)
        }
        _ => None,
    };
    if let Some(index) = payload {
        let layout = layout_of(ctx, &member_list[index])?;
        return Some(UnionLayout {
            layout,
            niche: true,
            niche_payload_layout: Some(layout),
            disc_type: None,
            payload_size: layout.size,
            payload_align: layout.align,
            member_list,
        });
    }
    let mut payload_size = 0;
    let mut payload_align = 1;
    for member in &member_list {
        let member_layout = layout_of(ctx, member)?;
        payload_size = payload_size.max(member_layout.size);
        payload_align = payload_align.max(member_layout.align);
    }
    let max_disc = member_list.len().wrapping_sub(1) as u64;
    let disc = disc_type_layout(max_disc);
    let align = disc.align.max(payload_align);
    Some(UnionLayout {
        layout: Layout { size: align_up(disc.size + payload_size, align), align },
        niche: false,
        niche_payload_layout: None,
        disc_type: Some(disc_type_name(max_disc).to_string()),
        payload_size,
        payload_align,
        member_list,
    })
}
