// SPDX-License-Identifier: GPL-2.0-only
//! Node visits share the same charged grant as all numerical bounds.
use super::*;
pub(super) fn certify(
    nodes: &[Node],
    root: usize,
    allowed: &BTreeSet<[usize; 2]>,
    names: &[ConicalActualSideId],
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<HierarchyStats> {
    let domain = Inventory {
        nodes,
        allowed,
        names,
        bits: arithmetic.fractional_bits(),
    };
    let mut pending = vec![[root, root]];
    let mut seen = BTreeSet::new();
    let mut stats = initial_stats(nodes.len());
    while let Some(pair) = pending.pop() {
        arithmetic.certificate_step()?;
        stats.visits += 1;
        visit(&domain, pair, &mut pending, &mut seen, &mut stats)?;
    }
    complete(&domain, root, &seen, &stats)?;
    Ok(stats)
}
struct Inventory<'a> {
    nodes: &'a [Node],
    allowed: &'a BTreeSet<[usize; 2]>,
    names: &'a [ConicalActualSideId],
    bits: u16,
}
fn visit(
    domain: &Inventory<'_>,
    pair: [usize; 2],
    pending: &mut Vec<[usize; 2]>,
    seen: &mut BTreeSet<[usize; 2]>,
    stats: &mut HierarchyStats,
) -> Outcome<()> {
    let a = &domain.nodes[pair[0]];
    let b = &domain.nodes[pair[1]];
    if pair[0] == pair[1] {
        same_node(a, pending);
        return Ok(());
    }
    if separated(&a.bounds, &b.bounds) {
        stats.excluded_products += 1;
        stats.excluded_pairs += a.count * b.count;
        return Ok(());
    }
    if let (Kind::Leaf(x), Kind::Leaf(y)) = (a.kind, b.kind) {
        return leaf_pair(domain, [a, b], [x, y], seen, stats);
    }
    expand(domain.nodes, pair, pending);
    Ok(())
}
fn separated(a: &Hull, b: &Hull) -> bool {
    (0..3).any(|i| a.upper[i] < b.lower[i] || b.upper[i] < a.lower[i])
}
fn expand(nodes: &[Node], pair: [usize; 2], pending: &mut Vec<[usize; 2]>) {
    let a = &nodes[pair[0]];
    let b = &nodes[pair[1]];
    let axis = match (a.kind, b.kind) {
        (Kind::Leaf(_), _) => 1,
        (_, Kind::Leaf(_)) => 0,
        _ => usize::from(b.count > a.count),
    };
    if let Kind::Branch(children) = nodes[pair[axis]].kind {
        for child in children.into_iter().rev() {
            let mut next = pair;
            next[axis] = child;
            pending.push(next);
        }
    }
}

fn initial_stats(nodes: usize) -> HierarchyStats {
    HierarchyStats {
        nodes,
        visits: 0,
        excluded_pairs: 0,
        adjacent_pairs: 0,
        excluded_products: 0,
    }
}
fn complete(
    domain: &Inventory<'_>,
    root: usize,
    seen: &BTreeSet<[usize; 2]>,
    stats: &HierarchyStats,
) -> Outcome<()> {
    if seen != domain.allowed {
        return Err(error::shape(
            "visited actual adjacency pairs",
            seen.len(),
            "every declared adjacent pair",
        ));
    }
    let n = domain.nodes[root].count;
    if stats.excluded_pairs + stats.adjacent_pairs != n * (n - 1) / 2 {
        return Err(error::shape(
            "pair coverage",
            (stats.excluded_pairs + stats.adjacent_pairs) as usize,
            "complete unordered product",
        ));
    }
    Ok(())
}

fn same_node(node: &Node, pending: &mut Vec<[usize; 2]>) {
    if let Kind::Branch([l, r]) = node.kind {
        pending.extend([[r, r], [l, r], [l, l]]);
    }
}
fn leaf_pair(
    domain: &Inventory<'_>,
    nodes: [&Node; 2],
    indices: [usize; 2],
    seen: &mut BTreeSet<[usize; 2]>,
    stats: &mut HierarchyStats,
) -> Outcome<()> {
    let [x, y] = indices;
    let key = if x < y { [x, y] } else { [y, x] };
    if !domain.allowed.contains(&key) {
        return Err(residual(domain, nodes, indices));
    }
    seen.insert(key);
    stats.adjacent_pairs += 1;
    Ok(())
}
fn residual(
    domain: &Inventory<'_>,
    nodes: [&Node; 2],
    indices: [usize; 2],
) -> ConicalActualPatchFailure {
    let exponent = -i32::from(domain.bits);
    let lower = nodes.map(|n| dyadic_points(&n.bounds.lower, exponent));
    let upper = nodes.map(|n| dyadic_points(&n.bounds.upper, exponent));
    ConicalActualPatchFailure::UnresolvedSidePair {
        sides: indices.map(|i| domain.names[i]),
        lower: Box::new(lower),
        upper: Box::new(upper),
        expected: "strict complete hull separation for nonincident actual sides",
    }
}
fn dyadic_points(points: &[num_bigint::BigInt; 3], exponent: i32) -> [DyadicBound; 3] {
    std::array::from_fn(|i| DyadicBound::new(points[i].clone(), exponent))
}
