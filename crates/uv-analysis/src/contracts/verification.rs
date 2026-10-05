//! Static proof of predicates from known facts.
//!
//! A predicate is provable when it is literally true, evaluates to true, matches a known
//! fact, is a conjunction or disjunction of provable parts, or is a linear integer
//! comparison that the known linear facts entail. Entailment is decided by refutation:
//! the facts together with the negated goal must have no integer solution, which a
//! simplex relaxation with branch and bound checks.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use uv_core::numeric_literals::{parse_int_core, strip_int_suffix};
use uv_core::span::Span;
use uv_source::ast::*;
use uv_source::lexer::token::TokenKind;

use super::struct_equal::expr_struct_equal;
use crate::typing::types::{TypeNode as SemNode, TypeRef};

#[derive(Debug, Clone)]
pub struct VerificationFact {
    pub predicate: ExprPtr,
    pub location: Span,
    pub scope_id: usize,
}

#[derive(Debug, Clone, Default)]
pub struct StaticProofContext {
    pub facts: Vec<VerificationFact>,
    pub current_scope: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StaticProofResult {
    pub provable: bool,
    pub diag_id: Option<&'static str>,
    /// Which rule proved the predicate; empty when it is not provable.
    pub explanation: &'static str,
}

fn bool_literal(expr: &ExprPtr) -> Option<bool> {
    match &expr.as_deref()?.node {
        ExprNode::LiteralExpr(lit) if lit.literal.kind == TokenKind::BoolLiteral => Some(lit.literal.lexeme == "true"),
        _ => None,
    }
}

pub fn is_literal_true(expr: &ExprPtr) -> bool {
    matches!(&expr.as_deref().map(|expr| &expr.node), Some(ExprNode::LiteralExpr(lit))
        if lit.literal.kind == TokenKind::BoolLiteral && lit.literal.lexeme == "true")
}

pub fn is_literal_false(expr: &ExprPtr) -> bool {
    matches!(&expr.as_deref().map(|expr| &expr.node), Some(ExprNode::LiteralExpr(lit))
        if lit.literal.kind == TokenKind::BoolLiteral && lit.literal.lexeme == "false")
}

/// A sum of integer multiples of opaque terms, plus a constant.
#[derive(Debug, Clone, Default)]
struct LinearExpr {
    terms: BTreeMap<String, i64>,
    constant: i64,
}

/// `sum(coeff * term) <= rhs`.
#[derive(Debug, Clone, Default)]
struct LinearConstraint {
    coeffs: BTreeMap<String, i64>,
    rhs: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ComparisonOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

/// `diff <op> 0`, where `diff` is the left side minus the right side.
#[derive(Debug, Clone)]
struct ParsedLinearComparison {
    op: ComparisonOp,
    diff: LinearExpr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SatStatus {
    Unsat,
    Sat,
    Unknown,
}

fn add_term(expr: &mut LinearExpr, var: &str, coeff: i64) -> Option<()> {
    if coeff == 0 {
        return Some(());
    }
    match expr.terms.get(var).copied() {
        None => {
            expr.terms.insert(var.to_string(), coeff);
        }
        Some(existing) => match existing.checked_add(coeff)? {
            0 => {
                expr.terms.remove(var);
            }
            sum => {
                expr.terms.insert(var.to_string(), sum);
            }
        },
    }
    Some(())
}

/// `dst += src * scale`; fails on overflow.
fn merge_linear(dst: &mut LinearExpr, src: &LinearExpr, scale: i64) -> Option<()> {
    dst.constant = dst.constant.checked_add(src.constant.checked_mul(scale)?)?;
    for (var, coeff) in &src.terms {
        add_term(dst, var, coeff.checked_mul(scale)?)?;
    }
    Some(())
}

/// A non-negative literal that fits `i64`.
fn parse_int_literal(lexeme: &str) -> Option<i64> {
    i64::try_from(parse_int_core(strip_int_suffix(lexeme))?).ok()
}

fn path_key(path: &[String], name: &str) -> String {
    let mut out = path.join("::");
    if !path.is_empty() {
        out.push_str("::");
    }
    out.push_str(name);
    out
}

/// Arguments are spelled by their literal text or their key; a moved argument has none.
fn paren_arg_keys(out: &mut String, args: &[Arg]) -> Option<()> {
    out.push('(');
    for (index, arg) in args.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        if arg.pass == ArgPassKind::Move {
            return None;
        }
        match &arg.value.as_deref()?.node {
            ExprNode::LiteralExpr(lit) => out.push_str(&lit.literal.lexeme),
            _ => out.push_str(&expr_key(&arg.value)?),
        }
    }
    out.push(')');
    Some(())
}

/// A canonical spelling for a place or a pure-looking call, so that two occurrences of
/// it count as the same unknown.
fn expr_key(expr: &ExprPtr) -> Option<String> {
    Some(match &expr.as_deref()?.node {
        ExprNode::IdentifierExpr(ident) => ident.name.clone(),
        ExprNode::PathExpr(path) => path_key(&path.path, &path.name),
        ExprNode::QualifiedNameExpr(qual) => path_key(&qual.path, &qual.name),
        ExprNode::FieldAccessExpr(field) => format!("{}.{}", expr_key(&field.base)?, field.name),
        ExprNode::TupleAccessExpr(tuple) => format!("{}.{}", expr_key(&tuple.base)?, tuple.index),
        ExprNode::IndexAccessExpr(index) => {
            let base = expr_key(&index.base)?;
            let ExprNode::LiteralExpr(lit) = &index.index.as_deref()?.node else {
                return None;
            };
            if lit.literal.kind != TokenKind::IntLiteral {
                return None;
            }
            format!("{base}[{}]", parse_int_literal(&lit.literal.lexeme)?)
        }
        ExprNode::EntryExpr(entry) => format!("@entry({})", expr_key(&entry.expr)?),
        ExprNode::ResultExpr(_) => "@result".to_string(),
        ExprNode::CallExpr(call) => {
            let mut out = expr_key(&call.callee)?;
            paren_arg_keys(&mut out, &call.args)?;
            out
        }
        ExprNode::MethodCallExpr(method) => {
            let mut out = format!("{}~>{}", expr_key(&method.receiver)?, method.name);
            paren_arg_keys(&mut out, &method.args)?;
            out
        }
        ExprNode::QualifiedApplyExpr(apply) => {
            let ApplyArgs::ParenArgs(paren) = &apply.args else {
                return None;
            };
            let mut out = path_key(&apply.path, &apply.name);
            paren_arg_keys(&mut out, &paren.args)?;
            out
        }
        _ => return None,
    })
}

/// The expression as a linear form, when it is one: literals, keyed terms, negation,
/// sums, and products with a constant factor.
fn linearize_expr(expr: &ExprPtr) -> Option<LinearExpr> {
    let mut out = LinearExpr::default();
    match &expr.as_deref()?.node {
        ExprNode::LiteralExpr(lit) => {
            if lit.literal.kind != TokenKind::IntLiteral {
                return None;
            }
            out.constant = parse_int_literal(&lit.literal.lexeme)?;
            return Some(out);
        }
        _ => {
            if let Some(key) = expr_key(expr) {
                add_term(&mut out, &key, 1)?;
                return Some(out);
            }
        }
    }
    match &expr.as_deref()?.node {
        ExprNode::UnaryExpr(unary) if unary.op == "-" || unary.op == "+" => {
            let inner = linearize_expr(&unary.value)?;
            merge_linear(&mut out, &inner, if unary.op == "-" { -1 } else { 1 })?;
        }
        ExprNode::BinaryExpr(binary) if binary.op == "+" || binary.op == "-" => {
            let left = linearize_expr(&binary.lhs)?;
            let right = linearize_expr(&binary.rhs)?;
            merge_linear(&mut out, &left, 1)?;
            merge_linear(&mut out, &right, if binary.op == "-" { -1 } else { 1 })?;
        }
        ExprNode::BinaryExpr(binary) if binary.op == "*" => {
            let left = linearize_expr(&binary.lhs)?;
            let right = linearize_expr(&binary.rhs)?;
            match (left.terms.is_empty(), right.terms.is_empty()) {
                (false, false) => return None,
                (true, _) => merge_linear(&mut out, &right, left.constant)?,
                (false, true) => merge_linear(&mut out, &left, right.constant)?,
            }
        }
        _ => return None,
    }
    Some(out)
}

fn negate_linear(expr: &mut LinearExpr) -> Option<()> {
    expr.constant = expr.constant.checked_mul(-1)?;
    for coeff in expr.terms.values_mut() {
        *coeff = coeff.checked_mul(-1)?;
    }
    Some(())
}

fn negated(mut expr: LinearExpr) -> Option<LinearExpr> {
    negate_linear(&mut expr)?;
    Some(expr)
}

fn evaluate_comparison(lhs: i64, rhs: i64, op: ComparisonOp) -> bool {
    match op {
        ComparisonOp::Eq => lhs == rhs,
        ComparisonOp::Ne => lhs != rhs,
        ComparisonOp::Lt => lhs < rhs,
        ComparisonOp::Le => lhs <= rhs,
        ComparisonOp::Gt => lhs > rhs,
        ComparisonOp::Ge => lhs >= rhs,
    }
}

fn parse_comparison_op(op: &str) -> Option<ComparisonOp> {
    Some(match op {
        "==" => ComparisonOp::Eq,
        "!=" => ComparisonOp::Ne,
        "<" => ComparisonOp::Lt,
        "<=" => ComparisonOp::Le,
        ">" => ComparisonOp::Gt,
        ">=" => ComparisonOp::Ge,
        _ => return None,
    })
}

/// `expr <= bound`.
fn make_inequality(expr: &LinearExpr, bound: i64) -> Option<LinearConstraint> {
    Some(LinearConstraint { coeffs: expr.terms.clone(), rhs: bound.checked_sub(expr.constant)? })
}

fn parse_linear_comparison(expr: &ExprPtr) -> Option<ParsedLinearComparison> {
    let ExprNode::BinaryExpr(binary) = &expr.as_deref()?.node else {
        return None;
    };
    let op = parse_comparison_op(&binary.op)?;
    let mut diff = linearize_expr(&binary.lhs)?;
    merge_linear(&mut diff, &linearize_expr(&binary.rhs)?, -1)?;
    Some(ParsedLinearComparison { op, diff })
}

/// The comparison as inequalities that hold together. Integers make `<` a `<=` one
/// lower; `!=` contributes nothing.
fn conjunction_constraints(pred: &ParsedLinearComparison) -> Option<Vec<LinearConstraint>> {
    let diff = &pred.diff;
    Some(match pred.op {
        ComparisonOp::Le => vec![make_inequality(diff, 0)?],
        ComparisonOp::Lt => vec![make_inequality(diff, -1)?],
        ComparisonOp::Ge => vec![make_inequality(&negated(diff.clone())?, 0)?],
        ComparisonOp::Gt => vec![make_inequality(&negated(diff.clone())?, -1)?],
        ComparisonOp::Eq => vec![make_inequality(diff, 0)?, make_inequality(&negated(diff.clone())?, 0)?],
        ComparisonOp::Ne => Vec::new(),
    })
}

/// The negation of the comparison, as alternatives of which one must hold.
fn negated_alternatives(pred: &ParsedLinearComparison) -> Option<Vec<Vec<LinearConstraint>>> {
    let diff = &pred.diff;
    let flipped = || negated(diff.clone());
    Some(match pred.op {
        ComparisonOp::Le => vec![vec![make_inequality(&flipped()?, -1)?]],
        ComparisonOp::Lt => vec![vec![make_inequality(&flipped()?, 0)?]],
        ComparisonOp::Ge => vec![vec![make_inequality(diff, -1)?]],
        ComparisonOp::Gt => vec![vec![make_inequality(diff, 0)?]],
        ComparisonOp::Eq => vec![vec![make_inequality(diff, -1)?], vec![make_inequality(&flipped()?, -1)?]],
        ComparisonOp::Ne => {
            vec![conjunction_constraints(&ParsedLinearComparison { op: ComparisonOp::Eq, diff: diff.clone() })?]
        }
    })
}

/// Adds what a fact says in linear terms. A fact that is `false`, or a comparison of
/// constants that does not hold, is a contradiction: anything follows.
fn collect_linear_facts(expr: &ExprPtr, facts: &mut Vec<LinearConstraint>, contradiction: &mut bool) {
    let Some(node) = expr.as_deref().map(|expr| &expr.node) else {
        return;
    };
    if *contradiction {
        return;
    }
    match node {
        ExprNode::LiteralExpr(_) => {
            *contradiction |= is_literal_false(expr);
            return;
        }
        ExprNode::BinaryExpr(binary) if binary.op == "&&" => {
            collect_linear_facts(&binary.lhs, facts, contradiction);
            collect_linear_facts(&binary.rhs, facts, contradiction);
            return;
        }
        _ => {}
    }
    let Some(pred) = parse_linear_comparison(expr) else {
        return;
    };
    if pred.diff.terms.is_empty() {
        *contradiction |= !evaluate_comparison(pred.diff.constant, 0, pred.op);
        return;
    }
    facts.extend(conjunction_constraints(&pred).unwrap_or_default());
}

const EPS: f64 = 1e-12;

/// Two-phase simplex on a dense tableau, for feasibility of `A x <= b`, `x >= 0`.
///
/// The reference computes in `long double`; this uses `f64`. The systems that reach it
/// are small with small integer coefficients, where both are exact enough to agree.
struct LpSolver {
    m: usize,
    n: usize,
    basis: Vec<i64>,
    non_basis: Vec<i64>,
    tableau: Vec<Vec<f64>>,
}

impl LpSolver {
    fn new(a: &[Vec<f64>], b: &[f64], n: usize) -> LpSolver {
        let m = b.len();
        let mut tableau = vec![vec![0.0; n + 2]; m + 2];
        let mut basis = vec![0; m];
        for i in 0..m {
            tableau[i][..n].copy_from_slice(&a[i]);
            basis[i] = (n + i) as i64;
            tableau[i][n] = -1.0;
            tableau[i][n + 1] = b[i];
        }
        let mut non_basis: Vec<i64> = (0..n as i64).collect();
        non_basis.push(-1);
        tableau[m + 1][n] = 1.0;
        LpSolver { m, n, basis, non_basis, tableau }
    }

    fn solution(&self) -> Vec<f64> {
        let mut x = vec![0.0; self.n];
        for i in 0..self.m {
            if (0..self.n as i64).contains(&self.basis[i]) {
                x[self.basis[i] as usize] = self.tableau[i][self.n + 1];
            }
        }
        x
    }

    /// A feasible point, or none when the system is infeasible.
    fn solve(&mut self) -> Option<Vec<f64>> {
        let (m, n) = (self.m, self.n);
        let mut r = 0;
        for i in 1..m {
            if self.tableau[i][n + 1] < self.tableau[r][n + 1] {
                r = i;
            }
        }
        if self.tableau[r][n + 1] < -EPS {
            self.pivot(r, n);
            if !self.simplex(1) || self.tableau[m + 1][n + 1].abs() > EPS {
                return None;
            }
            if let Some(row) = self.basis.iter().position(|b| *b == -1) {
                let mut s = 0;
                for j in 1..=n {
                    let (candidate, best) = (self.tableau[row][j], self.tableau[row][s]);
                    if candidate.abs() > best.abs() + EPS
                        || ((candidate - best).abs() <= EPS && self.non_basis[j] < self.non_basis[s])
                    {
                        s = j;
                    }
                }
                self.pivot(row, s);
            }
        }
        // Whether the second phase is bounded does not matter for feasibility.
        self.simplex(2);
        Some(self.solution())
    }

    fn pivot(&mut self, r: usize, s: usize) {
        let inv = 1.0 / self.tableau[r][s];
        let pivot_row = self.tableau[r].clone();
        for i in 0..self.m + 2 {
            if i == r {
                continue;
            }
            let factor = self.tableau[i][s];
            for (j, pivot) in pivot_row.iter().enumerate().take(self.n + 2) {
                if j != s {
                    self.tableau[i][j] -= pivot * factor * inv;
                }
            }
            self.tableau[i][s] *= -inv;
        }
        for j in 0..self.n + 2 {
            if j != s {
                self.tableau[r][j] *= inv;
            }
        }
        self.tableau[r][s] = inv;
        std::mem::swap(&mut self.basis[r], &mut self.non_basis[s]);
    }

    /// False when the objective is unbounded.
    fn simplex(&mut self, phase: usize) -> bool {
        let (m, n) = (self.m, self.n);
        let x = if phase == 1 { m + 1 } else { m };
        loop {
            let mut s: Option<usize> = None;
            for j in 0..=n {
                if phase == 2 && self.non_basis[j] == -1 {
                    continue;
                }
                let better = match s {
                    None => true,
                    Some(best) => {
                        self.tableau[x][j] < self.tableau[x][best] - EPS
                            || ((self.tableau[x][j] - self.tableau[x][best]).abs() <= EPS
                                && self.non_basis[j] < self.non_basis[best])
                    }
                };
                if better {
                    s = Some(j);
                }
            }
            let Some(s) = s.filter(|s| self.tableau[x][*s] < -EPS) else {
                return true;
            };
            let mut r: Option<usize> = None;
            for i in 0..m {
                if self.tableau[i][s] <= EPS {
                    continue;
                }
                let Some(best) = r else {
                    r = Some(i);
                    continue;
                };
                let lhs = self.tableau[i][n + 1] / self.tableau[i][s];
                let rhs = self.tableau[best][n + 1] / self.tableau[best][s];
                if lhs < rhs - EPS || ((lhs - rhs).abs() <= EPS && self.basis[i] < self.basis[best]) {
                    r = Some(i);
                }
            }
            let Some(r) = r else {
                return false;
            };
            self.pivot(r, s);
        }
    }
}

/// A real solution of the constraints, each unknown split into a non-negative positive
/// and negative part.
fn solve_lp_relaxation(constraints: &[LinearConstraint], vars: &[String]) -> Option<Vec<f64>> {
    if vars.is_empty() {
        return constraints.iter().all(|c| c.rhs >= 0).then(Vec::new);
    }
    let n = vars.len() * 2;
    let mut a = vec![vec![0.0; n]; constraints.len()];
    let mut b = vec![0.0; constraints.len()];
    for (row, constraint) in constraints.iter().enumerate() {
        b[row] = constraint.rhs as f64;
        for (name, coeff) in &constraint.coeffs {
            if let Ok(index) = vars.binary_search(name) {
                a[row][index * 2] += *coeff as f64;
                a[row][index * 2 + 1] -= *coeff as f64;
            }
        }
    }
    let values = LpSolver::new(&a, &b, n).solve()?;
    Some((0..vars.len()).map(|i| values[i * 2] - values[i * 2 + 1]).collect())
}

fn to_i64(v: f64) -> Option<i64> {
    (v.is_finite() && v >= i64::MIN as f64 && v <= i64::MAX as f64).then_some(v as i64)
}

/// Branch and bound on the first unknown whose relaxed value is not an integer.
fn solve_integer_feasibility(constraints: &[LinearConstraint], vars: &[String], nodes_visited: &mut usize) -> SatStatus {
    const MAX_BRANCH_NODES: usize = 20000;
    let visited = *nodes_visited;
    *nodes_visited += 1;
    if visited >= MAX_BRANCH_NODES {
        return SatStatus::Unknown;
    }
    let Some(values) = solve_lp_relaxation(constraints, vars) else {
        return SatStatus::Unsat;
    };
    let almost_integer = |v: &f64| (v - v.round()).abs() <= 1e-9;
    let Some(frac_index) = values.iter().position(|v| !almost_integer(v)) else {
        return SatStatus::Sat;
    };
    let value = values[frac_index];
    let (Some(floor), Some(ceil)) = (to_i64(value.floor()), to_i64(value.ceil())) else {
        return SatStatus::Unknown;
    };
    let split_var = &vars[frac_index];
    let branch = |coeff: i64, rhs: i64, nodes_visited: &mut usize| {
        let mut extended = constraints.to_vec();
        extended.push(LinearConstraint { coeffs: BTreeMap::from([(split_var.clone(), coeff)]), rhs });
        solve_integer_feasibility(&extended, vars, nodes_visited)
    };
    let left = branch(1, floor, nodes_visited);
    if left == SatStatus::Sat {
        return SatStatus::Sat;
    }
    let Some(lower) = ceil.checked_mul(-1) else {
        return SatStatus::Unknown;
    };
    match (left, branch(-1, lower, nodes_visited)) {
        (_, SatStatus::Sat) => SatStatus::Sat,
        (SatStatus::Unknown, _) | (_, SatStatus::Unknown) => SatStatus::Unknown,
        _ => SatStatus::Unsat,
    }
}

fn is_satisfiable(constraints: &[LinearConstraint]) -> SatStatus {
    let vars: Vec<String> =
        constraints.iter().flat_map(|c| c.coeffs.keys().cloned()).collect::<BTreeSet<_>>().into_iter().collect();
    solve_integer_feasibility(constraints, &vars, &mut 0)
}

/// The facts entail the target when every way of negating it contradicts them.
fn entails(facts: &[LinearConstraint], target: &ParsedLinearComparison) -> bool {
    if target.diff.terms.is_empty() {
        return evaluate_comparison(target.diff.constant, 0, target.op);
    }
    let Some(alternatives) = negated_alternatives(target).filter(|alts| !alts.is_empty()) else {
        return false;
    };
    alternatives.iter().all(|alt| {
        let combined: Vec<LinearConstraint> = facts.iter().chain(alt).cloned().collect();
        is_satisfiable(&combined) == SatStatus::Unsat
    })
}

/// A fact holds at a location in the same file at or after where it was established.
pub fn fact_dominates(fact: &VerificationFact, location: &Span) -> bool {
    fact.location.file == location.file && fact.location.start_offset <= location.start_offset
}

fn ent_fact_at(ctx: &StaticProofContext, location: &Span, expr: &ExprPtr) -> bool {
    ctx.facts.iter().any(|fact| fact_dominates(fact, location) && expr_struct_equal(&fact.predicate, expr))
}

fn ent_linear_at(ctx: &StaticProofContext, location: &Span, expr: &ExprPtr) -> bool {
    let Some(target) = parse_linear_comparison(expr) else {
        return false;
    };
    let mut facts = Vec::new();
    let mut contradiction = false;
    for fact in ctx.facts.iter().filter(|fact| fact_dominates(fact, location)) {
        collect_linear_facts(&fact.predicate, &mut facts, &mut contradiction);
        if contradiction {
            return true;
        }
    }
    entails(&facts, &target)
}

pub fn static_proof_at(ctx: &StaticProofContext, location: &Span, predicate: &ExprPtr) -> StaticProofResult {
    let proved = |explanation| StaticProofResult { provable: true, diag_id: None, explanation };
    let provable = |expr: &ExprPtr| static_proof_at(ctx, location, expr).provable;
    if ent_true(predicate) {
        return proved("Trivially true");
    }
    if let ConstValue::Bool(value) = evaluate_constant(predicate) {
        return if value { proved("Constant evaluation") } else { StaticProofResult::default() };
    }
    if ent_fact_at(ctx, location, predicate) {
        return proved("Matches known fact");
    }
    if let Some(ExprNode::BinaryExpr(binary)) = predicate.as_deref().map(|expr| &expr.node) {
        if binary.op == "&&" && provable(&binary.lhs) && provable(&binary.rhs) {
            return proved("Both conjuncts provable");
        }
        if binary.op == "||" && (provable(&binary.lhs) || provable(&binary.rhs)) {
            return proved("At least one disjunct provable");
        }
    }
    if ent_linear_at(ctx, location, predicate) {
        return proved("Linear integer reasoning");
    }
    StaticProofResult { provable: false, diag_id: Some("E-SEM-2850"), explanation: "" }
}

/// Proves the predicate where it is written.
pub fn static_proof(ctx: &StaticProofContext, predicate: &ExprPtr) -> StaticProofResult {
    let location = predicate.as_deref().map(|expr| expr.span.clone()).unwrap_or_default();
    static_proof_at(ctx, &location, predicate)
}

pub fn ent_true(expr: &ExprPtr) -> bool {
    is_literal_true(expr)
}

pub fn ent_fact(ctx: &StaticProofContext, expr: &ExprPtr) -> bool {
    expr.as_deref().is_some_and(|node| ent_fact_at(ctx, &node.span, expr))
}

pub fn ent_linear(ctx: &StaticProofContext, expr: &ExprPtr) -> bool {
    expr.as_deref().is_some_and(|node| ent_linear_at(ctx, &node.span, expr))
}

/// The value of an expression built from literals only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstValue {
    Unknown,
    Int(i64),
    Bool(bool),
}

/// Folds literals through arithmetic, comparison and logic. Integer arithmetic wraps;
/// division by zero is unknown.
pub fn evaluate_constant(expr: &ExprPtr) -> ConstValue {
    use ConstValue::*;
    let Some(node) = expr.as_deref().map(|expr| &expr.node) else {
        return Unknown;
    };
    match node {
        ExprNode::LiteralExpr(lit) => match lit.literal.kind {
            TokenKind::IntLiteral => parse_int_literal(&lit.literal.lexeme).map_or(Unknown, Int),
            TokenKind::BoolLiteral => bool_literal(expr).map_or(Unknown, Bool),
            _ => Unknown,
        },
        ExprNode::UnaryExpr(unary) => match (unary.op.as_str(), evaluate_constant(&unary.value)) {
            ("-", Int(value)) => Int(value.wrapping_neg()),
            ("+", Int(value)) => Int(value),
            ("!", Bool(value)) => Bool(!value),
            _ => Unknown,
        },
        ExprNode::BinaryExpr(binary) => {
            match (evaluate_constant(&binary.lhs), binary.op.as_str(), evaluate_constant(&binary.rhs)) {
                (Int(a), "+", Int(b)) => Int(a.wrapping_add(b)),
                (Int(a), "-", Int(b)) => Int(a.wrapping_sub(b)),
                (Int(a), "*", Int(b)) => Int(a.wrapping_mul(b)),
                (Int(a), "/", Int(b)) if b != 0 => Int(a.wrapping_div(b)),
                (Int(a), "%", Int(b)) if b != 0 => Int(a.wrapping_rem(b)),
                (Int(a), op, Int(b)) => parse_comparison_op(op).map_or(Unknown, |op| Bool(evaluate_comparison(a, b, op))),
                (Bool(a), "&&", Bool(b)) => Bool(a && b),
                (Bool(a), "||", Bool(b)) => Bool(a || b),
                (Bool(a), "==", Bool(b)) => Bool(a == b),
                (Bool(a), "!=", Bool(b)) => Bool(a != b),
                _ => Unknown,
            }
        }
        _ => Unknown,
    }
}

/// The range of a small integer type; wider types are treated as unbounded.
pub fn get_type_bounds(ty: &TypeRef) -> Option<(i64, i64)> {
    let Some(SemNode::Prim(name)) = ty.as_deref().map(|ty| &ty.node) else {
        return None;
    };
    Some(match name.as_str() {
        "u8" => (0, 255),
        "u16" => (0, 65535),
        "u32" => (0, 4_294_967_295),
        "i8" => (-128, 127),
        "i16" => (-32768, 32767),
        "i32" => (-2_147_483_648, 2_147_483_647),
        _ => return None,
    })
}

pub fn add_fact(ctx: &mut StaticProofContext, predicate: &ExprPtr, location: &Span) {
    ctx.facts.push(VerificationFact {
        predicate: predicate.clone(),
        location: location.clone(),
        scope_id: ctx.current_scope,
    });
}

/// Adds each conjunct of the predicate as a fact of its own.
pub fn add_predicate_facts_at(ctx: &mut StaticProofContext, predicate: &ExprPtr, location: &Span) {
    match predicate.as_deref().map(|expr| &expr.node) {
        None => {}
        Some(ExprNode::BinaryExpr(binary)) if binary.op == "&&" => {
            add_predicate_facts_at(ctx, &binary.lhs, location);
            add_predicate_facts_at(ctx, &binary.rhs, location);
        }
        Some(_) => add_fact(ctx, predicate, location),
    }
}

pub fn add_predicate_facts(ctx: &mut StaticProofContext, predicate: &ExprPtr) {
    let location = predicate.as_deref().map(|expr| expr.span.clone()).unwrap_or_default();
    add_predicate_facts_at(ctx, predicate, &location);
}

/// A copy of the context that also knows the predicate; nothing when there is neither.
pub fn extend_proof_context_with_predicate_at(
    base: Option<&StaticProofContext>,
    predicate: &ExprPtr,
    location: &Span,
) -> Option<StaticProofContext> {
    if base.is_none() && predicate.is_none() {
        return None;
    }
    let mut ctx = base.cloned().unwrap_or_default();
    add_predicate_facts_at(&mut ctx, predicate, location);
    Some(ctx)
}

/// The predicate's negation: `!p` loses its `!`, a comparison flips its operator,
/// anything else gains a `!`. A unary expression other than `!p` has none.
pub fn negated_predicate(predicate: &ExprPtr) -> Option<ExprPtr> {
    let expr = predicate.as_deref()?;
    let make = |node| Some(Arc::new(Expr { span: expr.span.clone(), node }));
    let not = || make(ExprNode::UnaryExpr(UnaryExpr { op: "!".to_string(), value: predicate.clone() }));
    match &expr.node {
        ExprNode::UnaryExpr(unary) => (unary.op == "!" && unary.value.is_some()).then(|| unary.value.clone()),
        ExprNode::BinaryExpr(binary) => {
            let flipped = match binary.op.as_str() {
                "<" => ">=",
                "<=" => ">",
                ">" => "<=",
                ">=" => "<",
                "==" => "!=",
                "!=" => "==",
                _ => return Some(not()),
            };
            Some(make(ExprNode::BinaryExpr(BinaryExpr { op: flipped.to_string(), ..binary.clone() })))
        }
        _ => Some(not()),
    }
}
