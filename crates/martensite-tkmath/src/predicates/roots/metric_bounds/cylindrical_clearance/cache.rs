// SPDX-License-Identifier: GPL-2.0-only
//! Source-local dyadic nodes are computed once, independent of opposing pair order.
use super::*;
use crate::predicates::roots::DyadicBound;
use context::{Context, CurveNode};
use num_bigint::BigInt;

pub(super) fn split(
    context: &mut Context<'_, '_>,
    source: usize,
    node: usize,
) -> Checked<[usize; 2]> {
    if let Some(children) = context.curves[source].nodes[node].children {
        return Ok(children);
    }
    context.reserve_nodes(2)?;
    let parent = &context.curves[source].nodes[node];
    let (index, level) = (parent.index * 2, parent.level + 1);
    let children = split_poles(&parent.poles, &mut context.arithmetic)?;
    let first = context.curves[source].nodes.len();
    let indices = [first, first + 1];
    for (side, poles) in children.into_iter().enumerate() {
        context.curves[source]
            .nodes
            .push(CurveNode::new(poles, index + side as u64, level));
    }
    context.curves[source].nodes[node].children = Some(indices);
    context.statistics.curve_nodes += 2;
    context.statistics.curve_splits += 1;
    Ok(indices)
}

pub(super) fn samples(
    context: &mut Context<'_, '_>,
    source: usize,
    node: usize,
) -> Checked<[Point; 3]> {
    let curve = &mut context.curves[source].nodes[node];
    let midpoint = cached_midpoint(curve, &mut context.arithmetic)?;
    Ok([
        curve.poles[0].clone(),
        midpoint,
        curve.poles[curve.poles.len() - 1].clone(),
    ])
}

fn cached_midpoint(node: &mut CurveNode, arithmetic: &mut Arithmetic<'_>) -> Checked<Point> {
    if let Some(point) = &node.midpoint {
        return Ok(point.clone());
    }
    let [left, _] = split_poles(&node.poles, arithmetic)?;
    let point = left[left.len() - 1].clone();
    node.midpoint = Some(point.clone());
    Ok(point)
}

fn split_poles(poles: &[Point], arithmetic: &mut Arithmetic<'_>) -> Checked<[Vec<Point>; 2]> {
    let x = split_axis(poles, 0, arithmetic)?;
    let z = split_axis(poles, 1, arithmetic)?;
    Ok([
        curve::align_planar(&x[0], &z[0]),
        curve::align_planar(&x[1], &z[1]),
    ])
}

fn split_axis(
    poles: &[Point],
    axis: usize,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[Vec<Interval>; 2]> {
    let values: Vec<_> = poles.iter().map(|p| p[axis].clone()).collect();
    Ok(curve::bisect(&values, arithmetic)?)
}

pub(super) fn parameter(
    context: &Context<'_, '_>,
    source: usize,
    node: usize,
    sample: usize,
) -> DyadicBound {
    let parent = context.query.sources[source].interval;
    let local = &context.curves[source].nodes[node];
    let numerator = (BigInt::from(parent.index()) << usize::from(local.level + 1))
        + BigInt::from(local.index) * 2
        + BigInt::from(sample);
    DyadicBound::new(
        numerator,
        -(i32::from(parent.level()) + i32::from(local.level) + 1),
    )
}
