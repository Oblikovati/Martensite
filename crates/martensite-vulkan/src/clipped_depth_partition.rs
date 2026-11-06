//! Independent integer fan-partition control; never supplies production geometry or depth.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PartitionFailure {
    Bounds,
    Overlap,
    PlaneDomain,
}
type Point = [i64; 2];
fn edge(a: Point, b: Point, p: Point) -> i64 {
    let first = [b[0] - a[0], b[1] - a[1]];
    let second = [p[0] - a[0], p[1] - a[1]];
    first[0] * second[1] - first[1] * second[0]
}
fn included(a: Point, b: Point, p: Point, outer: bool) -> bool {
    let value = edge(a, b, p);
    let half_open = b[1] < a[1] || (b[1] == a[1] && b[0] < a[0]);
    value > 0 || (value == 0 && (outer || half_open))
}
fn nonempty(polygon: &[Point]) -> Vec<usize> {
    (0..polygon.len() - 2)
        .filter(|&i| edge(polygon[0], polygon[i + 1], polygon[i + 2]) != 0)
        .collect()
}
fn fan_contains(
    polygon: &[Point],
    fan: usize,
    range: [usize; 2],
    point: Point,
    planes: bool,
) -> bool {
    let mut triangle = [polygon[0], polygon[fan + 1], polygon[fan + 2]];
    let mut outer = [fan == range[0], true, fan == range[1]];
    if edge(triangle[0], triangle[1], triangle[2]) < 0 {
        triangle.swap(1, 2);
        outer.swap(0, 2);
    }
    (0..3).all(|i| {
        (planes && outer[i]) || included(triangle[i], triangle[(i + 1) % 3], point, outer[i])
    })
}
fn require_domain(polygon: &[Point], point: Point) -> Result<(), PartitionFailure> {
    let bounded = polygon
        .iter()
        .flatten()
        .chain(point.iter())
        .all(|&v| v.unsigned_abs() <= 16777215);
    if !(3..=17).contains(&polygon.len()) || !bounded {
        return Err(PartitionFailure::Bounds);
    }
    Ok(())
}
fn unique_owner(
    polygon: &[Point],
    point: Point,
    active: &[usize],
    planes: bool,
) -> Result<Option<usize>, PartitionFailure> {
    let mut owner = None;
    let range = [active[0], active[active.len() - 1]];
    for &fan in active {
        if !fan_contains(polygon, fan, range, point, planes) {
            continue;
        }
        if owner.replace(fan).is_some() {
            return Err(PartitionFailure::Overlap);
        }
    }
    Ok(owner)
}
pub(crate) fn selected(polygon: &[Point], point: Point) -> Result<Option<usize>, PartitionFailure> {
    require_domain(polygon, point)?;
    let active = nonempty(polygon);
    let Some(&first) = active.first() else {
        return Ok(None);
    };
    if point == polygon[0] {
        return Ok(Some(first));
    }
    unique_owner(polygon, point, &active, false)
}

pub(crate) fn selected_plane(polygon: &[Point], point: Point) -> Result<usize, PartitionFailure> {
    require_domain(polygon, point)?;
    require_plane_domain(polygon)?;
    if point == polygon[0] {
        return Ok(0);
    }
    let active = nonempty(polygon);
    unique_owner(polygon, point, &active, true)?.ok_or(PartitionFailure::PlaneDomain)
}
fn require_plane_domain(polygon: &[Point]) -> Result<(), PartitionFailure> {
    if !(3..=4).contains(&polygon.len()) {
        return Err(PartitionFailure::PlaneDomain);
    }
    let first = edge(polygon[0], polygon[1], polygon[2]);
    if first == 0 {
        return Err(PartitionFailure::PlaneDomain);
    }
    if polygon.len() == 4 {
        let second = edge(polygon[0], polygon[2], polygon[3]);
        if first.signum() != second.signum() {
            return Err(PartitionFailure::PlaneDomain);
        }
    }
    Ok(())
}

pub(crate) fn selected_certified_plane(
    polygon: &[Point],
    point: Point,
) -> Result<Option<usize>, PartitionFailure> {
    require_domain(polygon, point)?;
    let empty = crate::clipped_depth_reference_empty::certified_empty_mask(polygon)
        .map_err(|_| PartitionFailure::PlaneDomain)?;
    let active: Vec<_> = (0..polygon.len() - 2)
        .filter(|i| empty & (1 << i) == 0)
        .collect();
    let Some(&first) = active.first() else {
        return Ok(None);
    };
    if point == polygon[0] {
        return Ok(Some(first));
    }
    unique_owner(polygon, point, &active, true)
}

#[cfg(test)]
#[path = "clipped_depth_partition_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "clipped_depth_plane_domain_tests.rs"]
mod plane_domain_tests;
