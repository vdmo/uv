//! Contracts and invariants: predicates that see extra names.

use uv_source::ast::*;

use super::resolve_expr::resolve_expr;
use super::resolver::*;
use super::scopes_intro::intro;
use crate::context::Scope;

/// A loop invariant sees what the loop sees.
pub fn resolve_loop_invariant_opt(
    ctx: &mut ResolveContext<'_, '_>,
    invariant_opt: &Option<LoopInvariant>,
) -> Res<Option<LoopInvariant>> {
    let Some(invariant) = invariant_opt else {
        return Ok(None);
    };
    let predicate = resolve_expr(ctx, &invariant.predicate)?;
    Ok(Some(LoopInvariant { predicate, span: invariant.span.clone() }))
}

/// A type invariant sees the value as `self`.
pub fn resolve_type_invariant_opt(
    ctx: &mut ResolveContext<'_, '_>,
    invariant_opt: &Option<TypeInvariant>,
) -> Res<Option<TypeInvariant>> {
    let Some(invariant) = invariant_opt else {
        return Ok(None);
    };
    with_scope(ctx, Scope::new(), |ctx| {
        let introduced = intro(ctx.ctx, "self", &value_entity());
        if !introduced.ok {
            return Err(ResError::from_id(introduced.diag_id, None));
        }
        let predicate = resolve_expr(ctx, &invariant.predicate)?;
        Ok(Some(TypeInvariant { predicate, span: invariant.span.clone() }))
    })
}

/// A postcondition sees the returned value as `@result`.
pub fn resolve_contract_opt(
    ctx: &mut ResolveContext<'_, '_>,
    contract_opt: &Option<ContractClause>,
) -> Res<Option<ContractClause>> {
    let Some(contract) = contract_opt else {
        return Ok(None);
    };
    let mut out = contract.clone();
    if contract.precondition.is_some() {
        out.precondition = resolve_expr(ctx, &contract.precondition).map_err(ResError::id_span)?;
    }
    if contract.postcondition.is_some() {
        out.postcondition = with_scope(ctx, Scope::new(), |ctx| {
            let introduced = intro(ctx.ctx, "@result", &value_entity());
            if !introduced.ok {
                return Err(ResError::from_id(introduced.diag_id, None));
            }
            resolve_expr(ctx, &contract.postcondition).map_err(ResError::id_span)
        })?;
    }
    Ok(Some(out))
}

/// A name a foreign contract cannot resolve is a contract error, not a name error.
fn resolve_foreign_contract(ctx: &mut ResolveContext<'_, '_>, contract: &ForeignContractClause) -> Res<ForeignContractClause> {
    let mut predicates = Vec::with_capacity(contract.predicates.len());
    for pred in &contract.predicates {
        predicates.push(resolve_expr(ctx, pred).map_err(|err| {
            let mut err = err.id_span();
            if matches!(err.diag_id, Some("ResolveExpr-Ident-Err" | "Expr-Unresolved-Err" | "E-MOD-1301")) {
                err.diag_id = Some("E-SEM-2852");
            }
            err
        })?);
    }
    Ok(ForeignContractClause { kind: contract.kind, predicates, span: contract.span.clone() })
}

pub fn resolve_foreign_contracts_opt(
    ctx: &mut ResolveContext<'_, '_>,
    contracts_opt: &Option<Vec<ForeignContractClause>>,
) -> Res<Option<Vec<ForeignContractClause>>> {
    let Some(contracts) = contracts_opt else {
        return Ok(None);
    };
    Ok(Some(contracts.iter().map(|contract| resolve_foreign_contract(ctx, contract)).collect::<Res<Vec<_>>>()?))
}
