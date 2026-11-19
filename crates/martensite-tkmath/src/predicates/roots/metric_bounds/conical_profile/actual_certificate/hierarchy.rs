// SPDX-License-Identifier: GPL-2.0-only
//! Finite disjoint node products cover every actual unordered side pair once.
use super::*;
use capture::SideCapture;
mod tree;
mod visit;
#[derive(Clone)]
pub(super) struct Hull {
    lower: [num_bigint::BigInt; 3],
    upper: [num_bigint::BigInt; 3],
}
struct Node {
    bounds: Hull,
    count: u64,
    kind: Kind,
}
#[derive(Clone, Copy)]
enum Kind {
    Leaf(usize),
    Branch([usize; 2]),
}
pub(super) struct HierarchyStats {
    pub(super) nodes: usize,
    pub(super) visits: u64,
    pub(super) excluded_pairs: u64,
    pub(super) adjacent_pairs: u64,
    pub(super) excluded_products: u64,
}
pub(super) fn certify(
    sides: &[SideCapture],
    adjacent: &[ConicalActualAdjacent],
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<HierarchyStats> {
    let bounds = sides
        .iter()
        .map(tree::side_bounds)
        .collect::<Outcome<Vec<_>>>()?;
    let mut nodes = Vec::new();
    let mut source = (0..sides.len()).collect::<Vec<_>>();
    let root = tree::build(&bounds, &mut source, 0, &mut nodes, arithmetic)?;
    let allowed = allowed(sides, adjacent)?;
    visit::certify(
        &nodes,
        root,
        &allowed,
        &sides.iter().map(|s| s.source.id).collect::<Vec<_>>(),
        arithmetic,
    )
}
fn allowed(
    sides: &[SideCapture],
    adjacent: &[ConicalActualAdjacent],
) -> Outcome<BTreeSet<[usize; 2]>> {
    let indices = sides
        .iter()
        .enumerate()
        .map(|(i, s)| (s.source.id, i))
        .collect::<BTreeMap<_, _>>();
    let mut result = BTreeSet::new();
    for pair in adjacent {
        let a = side_index(&indices, pair.sides[0])?;
        let b = side_index(&indices, pair.sides[1])?;
        result.insert(if a < b { [a, b] } else { [b, a] });
    }
    Ok(result)
}

fn side_index(
    indices: &BTreeMap<ConicalActualSideId, usize>,
    id: ConicalActualSideId,
) -> Outcome<usize> {
    indices
        .get(&id)
        .copied()
        .ok_or_else(|| error::shape("missing adjacency side", id.0 as usize, "declared side"))
}
