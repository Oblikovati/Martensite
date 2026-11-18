// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use context::Context;

pub(super) fn certify(context: &mut Context<'_, '_>, sources: [usize; 2]) -> Checked<()> {
    context.statistics.root_pairs += 1;
    let mut pending = vec![[0, 0]];
    while let Some(nodes) = pending.pop() {
        context.tick()?;
        if boxes_separate(context, sources, nodes) {
            continue;
        }
        witness::refuse_if_close(context, sources, nodes)?;
        if projection::separated(context, sources, nodes)? {
            continue;
        }
        subdivide(context, sources, nodes, &mut pending)?;
    }
    Ok(())
}

fn boxes_separate(context: &Context<'_, '_>, sources: [usize; 2], nodes: [usize; 2]) -> bool {
    context.curves[sources[0]].nodes[nodes[0]].bounds.separated(
        &context.curves[sources[1]].nodes[nodes[1]].bounds,
        &context.shift,
        &context.delta,
    )
}

fn subdivide(
    context: &mut Context<'_, '_>,
    sources: [usize; 2],
    nodes: [usize; 2],
    pending: &mut Vec<[usize; 2]>,
) -> Checked<()> {
    let axis = split_axis(context, sources, nodes)?;
    let children = cache::split(context, sources[axis], nodes[axis])?;
    for child in children.into_iter().rev() {
        let mut next = nodes;
        next[axis] = child;
        pending.push(next);
    }
    Ok(())
}

fn split_axis(context: &Context<'_, '_>, sources: [usize; 2], nodes: [usize; 2]) -> Checked<usize> {
    let a = &context.curves[sources[0]].nodes[nodes[0]];
    let b = &context.curves[sources[1]].nodes[nodes[1]];
    let maximum = context.query.options.max_depth;
    if a.level >= maximum && b.level >= maximum {
        return Err(CylindricalClearanceFailure::RefinementLimit {
            sources,
            indices: [a.index, b.index],
            levels: [a.level, b.level],
            maximum,
        });
    }
    if a.level >= maximum {
        return Ok(1);
    }
    if b.level >= maximum {
        return Ok(0);
    }
    Ok(usize::from(b.bounds.width() > a.bounds.width()))
}
