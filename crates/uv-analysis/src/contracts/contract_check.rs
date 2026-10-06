//! Well-formedness of contracts and type invariants, and behavioural subtyping of an
//! implementation's contract against its class's. See the reference's `contract_check`.

use uv_source::ast::{self, ExprPtr};

use super::intrinsics::{contains_entry, contains_result};
use super::purity::{check_purity, ContractCheckResult, ContractContext};
use super::verification::{add_fact, static_proof_at, StaticProofContext};

fn fail(diag_id: &'static str, span: Option<uv_core::span::Span>) -> ContractCheckResult {
    ContractCheckResult { ok: false, diag_id: Some(diag_id), span }
}

fn ok() -> ContractCheckResult {
    ContractCheckResult { ok: true, diag_id: None, span: None }
}

fn check_precondition(ctx: &ContractContext<'_, '_>, expr: &ExprPtr) -> ContractCheckResult {
    let span = expr.as_deref().map(|expr| expr.span.clone());
    if contains_result(expr) {
        return fail("E-SEM-2806", span);
    }
    if contains_entry(expr) {
        return fail("E-SEM-2852", span);
    }
    check_purity(ctx, expr)
}

/// See `CheckContractWellFormed`.
pub fn check_contract_well_formed(ctx: &ContractContext<'_, '_>, contract: &ast::ContractClause) -> ContractCheckResult {
    if contract.precondition.is_some() {
        let pre = check_precondition(ctx, &contract.precondition);
        if !pre.ok {
            return pre;
        }
    }
    if contract.postcondition.is_some() {
        let post_ctx = ContractContext { is_postcondition: true, ..ctx.clone() };
        let post = check_purity(&post_ctx, &contract.postcondition);
        if !post.ok {
            return post;
        }
    }
    ok()
}

/// See `CheckTypeInvariant`: no `@result`, and pure.
pub fn check_type_invariant(ctx: &ContractContext<'_, '_>, invariant: &ast::TypeInvariant) -> ContractCheckResult {
    if contains_result(&invariant.predicate) {
        return fail("E-SEM-2854", None);
    }
    if !check_purity(ctx, &invariant.predicate).ok {
        return fail("E-SEM-3004", None);
    }
    ok()
}

/// The conjuncts of a predicate as facts.
fn add_implication_facts(proof_ctx: &mut StaticProofContext, predicate: &ExprPtr, target_span: &uv_core::span::Span) {
    let Some(e) = predicate.as_deref() else {
        return;
    };
    if let ast::ExprNode::BinaryExpr(binary) = &e.node {
        if binary.op == "&&" {
            add_implication_facts(proof_ctx, &binary.lhs, target_span);
            add_implication_facts(proof_ctx, &binary.rhs, target_span);
            return;
        }
    }
    add_fact(proof_ctx, predicate, target_span);
}

fn predicate_implies(antecedent: &ExprPtr, consequent: &ExprPtr) -> bool {
    let Some(consequent_expr) = consequent.as_deref() else {
        return true;
    };
    let mut proof_ctx = StaticProofContext::default();
    if antecedent.is_some() {
        add_implication_facts(&mut proof_ctx, antecedent, &consequent_expr.span);
    }
    static_proof_at(&proof_ctx, &consequent_expr.span, consequent).provable
}

/// An implementation may weaken the precondition and strengthen the postcondition.
pub fn check_behavioral_subtyping(class_contract: &ast::ContractClause, impl_contract: &ast::ContractClause) -> ContractCheckResult {
    if !predicate_implies(&class_contract.precondition, &impl_contract.precondition) {
        return fail("E-SEM-2803", None);
    }
    if !predicate_implies(&impl_contract.postcondition, &class_contract.postcondition) {
        return fail("E-SEM-2804", None);
    }
    ok()
}
