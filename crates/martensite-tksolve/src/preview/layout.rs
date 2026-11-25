// SPDX-License-Identifier: GPL-2.0-only

use super::{Block, SolveError, local::LocalSeeds};
use std::collections::BTreeMap;

pub(super) struct Component {
    pub(super) free: Vec<usize>,
    pub(super) rows: Vec<(usize, LocalSeeds)>,
}

pub(super) fn components(
    blocks: &[Block],
    constants: &[Option<f64>],
) -> Result<Vec<Component>, SolveError> {
    let mut parent = (0..constants.len()).collect::<Vec<_>>();
    let mut seeds = Vec::with_capacity(blocks.len());
    for (index, block) in blocks.iter().enumerate() {
        if let Some(&bad) = block.variables.iter().find(|&&i| i >= constants.len()) {
            return Err(SolveError::Options(format!(
                "block {index} variable {bad}: expected <{}",
                constants.len()
            )));
        }
        let scope = LocalSeeds::new(&block.variables, |i| constants[i].is_none());
        union_scope(&mut parent, &scope.columns);
        seeds.push(scope);
    }
    let mut groups = group_variables(&mut parent, constants);
    for (index, scope) in seeds.into_iter().enumerate() {
        let root = scope
            .columns
            .first()
            .map_or(usize::MAX, |&i| find(&mut parent, i));
        groups
            .entry(root)
            .or_insert_with(empty)
            .rows
            .push((index, scope));
    }
    Ok(groups.into_values().collect())
}

fn group_variables(parent: &mut [usize], constants: &[Option<f64>]) -> BTreeMap<usize, Component> {
    let mut result = BTreeMap::new();
    for (index, fixed) in constants.iter().enumerate() {
        if fixed.is_none() {
            result
                .entry(find(parent, index))
                .or_insert_with(empty)
                .free
                .push(index);
        }
    }
    result
}

fn empty() -> Component {
    Component {
        free: Vec::new(),
        rows: Vec::new(),
    }
}

fn union_scope(parent: &mut [usize], scope: &[usize]) {
    if let Some((&first, rest)) = scope.split_first() {
        for &node in rest {
            let a = find(parent, first);
            let b = find(parent, node);
            parent[a.max(b)] = a.min(b);
        }
    }
}

fn find(parent: &mut [usize], mut i: usize) -> usize {
    while parent[i] != i {
        parent[i] = parent[parent[i]];
        i = parent[i];
    }
    i
}
