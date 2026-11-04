//! Exact reference-plane emptiness over the declared sixteenth-pixel lattice, with a finite work budget.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ReferenceEmptyFailure {
    Bounds { point: [i64; 2] },
    Budget { requested: u64, remaining: u32 },
    SampleOffset { bits: [u32; 2] },
    Domain { vertices: usize, areas: [i64; 2] },
}
fn edge(a: [i64; 2], b: [i64; 2], p: [i64; 2]) -> i64 {
    let first = [b[0] - a[0], b[1] - a[1]];
    let second = [p[0] - a[0], p[1] - a[1]];
    first[0] * second[1] - first[1] * second[0]
}
fn require_bounds(triangle: [[i64; 2]; 3]) -> Result<(), ReferenceEmptyFailure> {
    for point in triangle {
        if point.into_iter().any(|v| v.unsigned_abs() > 16777215) {
            return Err(ReferenceEmptyFailure::Bounds { point });
        }
    }
    Ok(())
}
fn lattice_bounds(triangle: [[i64; 2]; 3]) -> [[i64; 2]; 2] {
    let low =
        std::array::from_fn(|i| -(-triangle.iter().map(|p| p[i]).min().unwrap()).div_euclid(256));
    let high =
        std::array::from_fn(|i| triangle.iter().map(|p| p[i]).max().unwrap().div_euclid(256));
    [low, high]
}
fn reserve_work(bounds: [[i64; 2]; 2], remaining: &mut u32) -> Result<(), ReferenceEmptyFailure> {
    let width = (bounds[1][0] - bounds[0][0] + 1).max(0) as u64;
    let height = (bounds[1][1] - bounds[0][1] + 1).max(0) as u64;
    let requested = width * height;
    if requested > u64::from(*remaining) {
        return Err(ReferenceEmptyFailure::Budget {
            requested,
            remaining: *remaining,
        });
    }
    *remaining -= requested as u32;
    Ok(())
}
fn closed_hit(triangle: [[i64; 2]; 3], point: [i64; 2]) -> bool {
    let signs: [i64; 3] = std::array::from_fn(|i| edge(triangle[i], triangle[(i + 1) % 3], point));
    signs.into_iter().all(|v| v >= 0) || signs.into_iter().all(|v| v <= 0)
}
fn row_hit(triangle: [[i64; 2]; 3], x: i64, low: i64, high: i64) -> bool {
    (low..=high).any(|y| closed_hit(triangle, [x * 256, y * 256]))
}
pub(crate) fn reference_empty(
    triangle: [[i64; 2]; 3],
    remaining: &mut u32,
) -> Result<bool, ReferenceEmptyFailure> {
    require_bounds(triangle)?;
    if edge(triangle[0], triangle[1], triangle[2]) == 0 {
        return Ok(true);
    }
    let bounds = lattice_bounds(triangle);
    reserve_work(bounds, remaining)?;
    Ok(!(bounds[0][0]..=bounds[1][0]).any(|x| row_hit(triangle, x, bounds[0][1], bounds[1][1])))
}
pub(crate) fn sample_offset(value: [f32; 2]) -> Result<[u32; 2], ReferenceEmptyFailure> {
    let scaled = value.map(|v| v * 16.);
    if value
        .into_iter()
        .any(|v| !v.is_finite() || !(0. ..=1.).contains(&v))
        || scaled.into_iter().any(|v| v != v.round_ties_even())
    {
        return Err(ReferenceEmptyFailure::SampleOffset {
            bits: value.map(f32::to_bits),
        });
    }
    Ok(scaled.map(|v| v as u32))
}
fn triangle(polygon: &[[i64; 2]], fan: usize) -> [[i64; 2]; 3] {
    [polygon[0], polygon[fan + 1], polygon[fan + 2]]
}
fn mixed_empty_mask(polygon: &[[i64; 2]], areas: [i64; 2]) -> Result<u32, ReferenceEmptyFailure> {
    let mut budget = 65536;
    let mut incomplete = None;
    for fan in 0..2 {
        match reference_empty(triangle(polygon, fan), &mut budget) {
            Ok(true) => return Ok(1 << fan),
            Ok(false) => {}
            Err(error) => incomplete = Some(error),
        }
    }
    Err(incomplete.unwrap_or(ReferenceEmptyFailure::Domain {
        vertices: polygon.len(),
        areas,
    }))
}
fn area_mask(polygon: &[[i64; 2]]) -> Result<([i64; 2], u32), ReferenceEmptyFailure> {
    if !(3..=4).contains(&polygon.len()) {
        return Err(ReferenceEmptyFailure::Domain {
            vertices: polygon.len(),
            areas: [0; 2],
        });
    }
    let mut areas = [0; 2];
    let mut mask = 0;
    for (fan, area) in areas.iter_mut().enumerate().take(polygon.len() - 2) {
        let tri = triangle(polygon, fan);
        require_bounds(tri)?;
        *area = edge(tri[0], tri[1], tri[2]);
        if *area == 0 {
            mask |= 1 << fan;
        }
    }
    Ok((areas, mask))
}
pub(crate) fn certified_empty_mask(polygon: &[[i64; 2]]) -> Result<u32, ReferenceEmptyFailure> {
    let (areas, mask) = area_mask(polygon)?;
    if polygon.len() == 3 || mask != 0 || areas[0].signum() == areas[1].signum() {
        return Ok(mask);
    }
    mixed_empty_mask(polygon, areas)
}

#[cfg(test)]
#[path = "clipped_depth_reference_empty_tests.rs"]
mod tests;
