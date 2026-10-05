//! The order in which a class and its superclasses are searched (C3 linearisation).

use std::collections::BTreeMap;

use crate::context::{PathKey, ScopeContext};
use crate::resolve::scopes::{path_eq, path_key_of};

pub type ClassPath = Vec<String>;

/// The class itself first, then its superclasses, each before its own superclasses and
/// in the order they are listed.
pub type Linearization = Result<Vec<ClassPath>, &'static str>;

/// The first list head that appears in no other list's tail, removed from every list.
fn merge(mut lists: Vec<Vec<ClassPath>>) -> Option<Vec<ClassPath>> {
    let mut order = Vec::new();
    while lists.iter().any(|list| !list.is_empty()) {
        let head = lists
            .iter()
            .filter_map(|list| list.first())
            .find(|head| lists.iter().all(|list| list.iter().skip(1).all(|entry| !path_eq(entry, head))))?
            .clone();
        for list in &mut lists {
            if list.first().is_some_and(|first| path_eq(first, &head)) {
                list.remove(0);
            }
        }
        order.push(head);
    }
    Some(order)
}

#[derive(Default)]
struct State {
    memo: BTreeMap<PathKey, Linearization>,
    active: Vec<PathKey>,
}

fn linearize(ctx: &ScopeContext<'_>, path: &[String], state: &mut State) -> Linearization {
    let key = path_key_of(path);
    if let Some(done) = state.memo.get(&key) {
        return done.clone();
    }
    // A class reached again while its own superclasses are being linearised.
    if state.active.contains(&key) {
        return Err("E-TYP-2508");
    }
    state.active.push(key.clone());
    let result = (|| {
        let decl = ctx.sigma.classes.get(&key).ok_or("Superclass-Undefined")?;
        let mut lists = Vec::with_capacity(decl.supers.len() + 1);
        for sup in &decl.supers {
            lists.push(linearize(ctx, sup, state)?);
        }
        if decl.supers.is_empty() {
            return Ok(vec![path.to_vec()]);
        }
        lists.push(decl.supers.clone());
        let mut order = vec![path.to_vec()];
        order.extend(merge(lists).ok_or("E-TYP-2508")?);
        Ok(order)
    })();
    state.active.pop();
    state.memo.insert(key, result.clone());
    result
}

pub fn linearize_class(ctx: &ScopeContext<'_>, path: &[String]) -> Linearization {
    linearize(ctx, path, &mut State::default())
}
