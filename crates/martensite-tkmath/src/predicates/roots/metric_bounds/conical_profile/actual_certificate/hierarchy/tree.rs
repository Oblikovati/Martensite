// SPDX-License-Identifier: GPL-2.0-only
//! Actual positive-weight control hulls; metadata construction stays cancellation-aware.
use super::*;
pub(super) fn side_bounds(side: &SideCapture) -> Outcome<Hull> {
    let points = side.coordinates.iter().flatten().collect::<Vec<_>>();
    let first = points
        .first()
        .ok_or_else(|| error::shape("actual side hull", 0, "nonempty actual control net"))?;
    let mut lower = std::array::from_fn(|i| first[i].lower.clone());
    let mut upper = std::array::from_fn(|i| first[i].upper.clone());
    for point in points.iter().skip(1) {
        for axis in 0..3 {
            lower[axis] = lower[axis].clone().min(point[axis].lower.clone());
            upper[axis] = upper[axis].clone().max(point[axis].upper.clone());
        }
    }
    Ok(Hull { lower, upper })
}
pub(super) fn build(
    bounds: &[Hull],
    sources: &mut [usize],
    depth: usize,
    nodes: &mut Vec<Node>,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<usize> {
    arithmetic.check_cancelled()?;
    if sources.is_empty() {
        return Err(error::shape("hierarchy sources", 0, "nonempty"));
    }
    let kind = if sources.len() == 1 {
        Kind::Leaf(sources[0])
    } else {
        children(bounds, sources, depth, nodes, arithmetic)?
    };
    let mut hull = bounds[sources[0]].clone();
    for i in sources.iter().skip(1) {
        hull = union(&hull, &bounds[*i]);
    }
    let index = nodes.len();
    nodes.push(Node {
        bounds: hull,
        count: sources.len() as u64,
        kind,
    });
    Ok(index)
}
fn children(
    bounds: &[Hull],
    sources: &mut [usize],
    depth: usize,
    nodes: &mut Vec<Node>,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<Kind> {
    let axis = depth % 3;
    sources.sort_by(|a, b| {
        bounds[*a].lower[axis]
            .cmp(&bounds[*b].lower[axis])
            .then(a.cmp(b))
    });
    let middle = sources.len() / 2;
    let (left, right) = sources.split_at_mut(middle);
    let a = build(bounds, left, depth + 1, nodes, arithmetic)?;
    let b = build(bounds, right, depth + 1, nodes, arithmetic)?;
    Ok(Kind::Branch([a, b]))
}
fn union(a: &Hull, b: &Hull) -> Hull {
    Hull {
        lower: std::array::from_fn(|i| a.lower[i].clone().min(b.lower[i].clone())),
        upper: std::array::from_fn(|i| a.upper[i].clone().max(b.upper[i].clone())),
    }
}
