// SPDX-License-Identifier: GPL-2.0-only
//! A certified-box hierarchy covers all requested pairs without materializing the pair matrix.
use super::*;
use context::{Context, HierarchyKind, HierarchyNode};

pub(super) fn build(context: &mut Context<'_, '_>, sources: &mut [usize]) -> Checked<usize> {
    context.arithmetic.check_cancelled()?;
    let bounds = sources.iter().skip(1).fold(
        context.curves[sources[0]].nodes[0].bounds.clone(),
        |acc, i| acc.union(&context.curves[*i].nodes[0].bounds),
    );
    let kind = children(context, sources, &bounds)?;
    let index = context.hierarchy.len();
    context.hierarchy.push(HierarchyNode {
        bounds,
        count: sources.len(),
        kind,
    });
    context.statistics.hierarchy_nodes += 1;
    Ok(index)
}

fn children(
    context: &mut Context<'_, '_>,
    sources: &mut [usize],
    bounds: &bounds::HullBox,
) -> Checked<HierarchyKind> {
    if sources.len() == 1 {
        return Ok(HierarchyKind::Leaf(sources[0]));
    }
    let axis = bounds.longest_axis();
    sources.sort_by(|a, b| {
        center(context, *a, axis)
            .cmp(&center(context, *b, axis))
            .then(a.cmp(b))
    });
    let middle = sources.len() / 2;
    let (left, right) = sources.split_at_mut(middle);
    let left = build(context, left)?;
    let right = build(context, right)?;
    Ok(HierarchyKind::Branch([left, right]))
}

fn center(context: &Context<'_, '_>, source: usize, axis: usize) -> num_bigint::BigInt {
    let bounds = &context.curves[source].nodes[0].bounds;
    &bounds.lower[axis] + &bounds.upper[axis]
}

pub(super) fn certify(context: &mut Context<'_, '_>, root: usize) -> Checked<()> {
    let mut pending = vec![[root, root]];
    while let Some(nodes) = pending.pop() {
        context.tick()?;
        visit(context, nodes, &mut pending)?;
    }
    Ok(())
}

fn visit(
    context: &mut Context<'_, '_>,
    nodes: [usize; 2],
    pending: &mut Vec<[usize; 2]>,
) -> Checked<()> {
    if context.query.domain.within() && nodes[0] == nodes[1] {
        return same_node(context, nodes[0], pending);
    }
    let a = &context.hierarchy[nodes[0]];
    let b = &context.hierarchy[nodes[1]];
    if a.bounds
        .separated(&b.bounds, &context.shift, &context.delta)
    {
        return Ok(());
    }
    match (a.kind, b.kind) {
        (HierarchyKind::Leaf(x), HierarchyKind::Leaf(y)) => leaf(context, [x, y]),
        _ => {
            expand(context, nodes, pending);
            Ok(())
        }
    }
}

fn same_node(context: &Context<'_, '_>, node: usize, pending: &mut Vec<[usize; 2]>) -> Checked<()> {
    if let HierarchyKind::Branch([a, b]) = context.hierarchy[node].kind {
        pending.extend([[b, b], [a, b], [a, a]]);
    }
    Ok(())
}

fn expand(context: &Context<'_, '_>, nodes: [usize; 2], pending: &mut Vec<[usize; 2]>) {
    let a = &context.hierarchy[nodes[0]];
    let b = &context.hierarchy[nodes[1]];
    let axis = match (a.kind, b.kind) {
        (HierarchyKind::Leaf(_), _) => 1,
        (_, HierarchyKind::Leaf(_)) => 0,
        _ => usize::from(b.count > a.count),
    };
    let kind = context.hierarchy[nodes[axis]].kind;
    if let HierarchyKind::Branch(children) = kind {
        for child in children.into_iter().rev() {
            let mut next = nodes;
            next[axis] = child;
            pending.push(next);
        }
    }
}

fn leaf(context: &mut Context<'_, '_>, mut sources: [usize; 2]) -> Checked<()> {
    if context.query.domain.within() {
        sources.sort();
        if context.excluded.contains(&sources) {
            return Ok(());
        }
    }
    pair::certify(context, sources)
}
