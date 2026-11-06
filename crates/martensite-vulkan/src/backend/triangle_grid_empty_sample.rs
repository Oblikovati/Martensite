//! A Q12 triangle's convex hull cannot cover a pixel center outside its widened closed bbox.
const HALF_GRID_STEP: f64 = 1.0 / 8192.0;
const Q12_SCALE: i64 = 4096;
const Q12_PIXEL_CENTER: i64 = Q12_SCALE / 2;
const MAX_ENUMERATED_VERTEX_POINTS: u64 = 512;
const MAX_ENUMERATED_BBOX_SAMPLES: u64 = 4096;
const MAX_EMPTY_PROOF_EDGE_PREDICATES: u64 = 65_536;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Q12EmptyCertificate {
    Empty,
    NotProven(Q12EmptyRefusal),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Q12EmptyRefusal {
    SampleMayBeCovered,
    WorkBudgetExceeded,
    InvalidBounds,
}

pub(super) fn sample_box_empty(points: [[f64; 2]; 3], errors: [[f64; 2]; 3]) -> bool {
    if !points.into_iter().flatten().all(f64::is_finite)
        || !errors
            .into_iter()
            .flatten()
            .all(|v| v.is_finite() && v >= 0.0)
    {
        return false;
    }
    (0..2).any(|axis| empty_axis(points, errors, axis))
}

pub(super) fn q12_interval_box_empty(
    vertices: [crate::triangle_grid_coverage::GridVertexInterval; 3],
) -> bool {
    let bounds: [[i32; 2]; 2] = std::array::from_fn(|axis| {
        vertices
            .map(|vertex| vertex.coordinates()[axis])
            .into_iter()
            .fold([i32::MAX, i32::MIN], |range, interval| {
                [range[0].min(interval[0]), range[1].max(interval[1])]
            })
    });
    bounds.into_iter().any(axis_has_no_q12_center)
}

pub(super) fn q12_interval_vertices_empty(
    vertices: [crate::triangle_grid_coverage::GridVertexInterval; 3],
) -> Q12EmptyCertificate {
    if q12_interval_box_empty(vertices) {
        return Q12EmptyCertificate::Empty;
    }
    let Some((samples, points)) = bounded_interval_search(vertices) else {
        return Q12EmptyCertificate::NotProven(Q12EmptyRefusal::WorkBudgetExceeded);
    };
    match any_possible_sample_is_covered(&samples, &points) {
        Some(true) => return Q12EmptyCertificate::NotProven(Q12EmptyRefusal::SampleMayBeCovered),
        Some(false) => {}
        None => return Q12EmptyCertificate::NotProven(Q12EmptyRefusal::InvalidBounds),
    }
    Q12EmptyCertificate::Empty
}

/// Sample centers and the candidate points of each triangle vertex.
type IntervalSearch = (Vec<[i64; 2]>, [Vec<[i64; 2]>; 3]);

fn bounded_interval_search(
    vertices: [crate::triangle_grid_coverage::GridVertexInterval; 3],
) -> Option<IntervalSearch> {
    let samples = bbox_sample_centers(interval_bounds(vertices))?;
    let points = interval_points(vertices)?;
    let counts = points.each_ref().map(Vec::len);
    if exceeds_predicate_budget(samples.len(), counts) {
        return None;
    }
    Some((samples, points))
}

fn interval_bounds(
    vertices: [crate::triangle_grid_coverage::GridVertexInterval; 3],
) -> [[i32; 2]; 2] {
    let coordinates = vertices.map(|vertex| vertex.coordinates());
    std::array::from_fn(|axis| {
        coordinates
            .map(|vertex| vertex[axis])
            .into_iter()
            .fold([i32::MAX, i32::MIN], |range, interval| {
                [range[0].min(interval[0]), range[1].max(interval[1])]
            })
    })
}

fn bbox_sample_centers(bounds: [[i32; 2]; 2]) -> Option<Vec<[i64; 2]>> {
    let x = sample_indices(bounds[0])?;
    let y = sample_indices(bounds[1])?;
    let count = u64::try_from(x.len())
        .ok()?
        .checked_mul(u64::try_from(y.len()).ok()?)?;
    if count > MAX_ENUMERATED_BBOX_SAMPLES {
        return None;
    }
    let mut samples = Vec::with_capacity(usize::try_from(count).ok()?);
    for y_index in y {
        for x_index in x.iter().copied() {
            samples.push([sample_center(x_index)?, sample_center(y_index)?]);
        }
    }
    Some(samples)
}

fn sample_indices([lower, upper]: [i32; 2]) -> Option<Vec<i64>> {
    if lower > upper {
        return None;
    }
    let low_delta = i64::from(lower).checked_sub(Q12_PIXEL_CENTER)?;
    let high_delta = i64::from(upper).checked_sub(Q12_PIXEL_CENTER)?;
    let first = ceil_div_euclidean(low_delta, Q12_SCALE)?;
    let last = high_delta.div_euclid(Q12_SCALE);
    if first > last {
        return Some(Vec::new());
    }
    let count = last.checked_sub(first)?.checked_add(1)?;
    if u64::try_from(count).ok()? > MAX_ENUMERATED_BBOX_SAMPLES {
        return None;
    }
    Some((first..=last).collect())
}

fn ceil_div_euclidean(value: i64, divisor: i64) -> Option<i64> {
    let quotient = value.div_euclid(divisor);
    quotient.checked_add(i64::from(value.rem_euclid(divisor) != 0))
}

fn sample_center(index: i64) -> Option<i64> {
    index.checked_mul(Q12_SCALE)?.checked_add(Q12_PIXEL_CENTER)
}

fn interval_points(
    vertices: [crate::triangle_grid_coverage::GridVertexInterval; 3],
) -> Option<[Vec<[i64; 2]>; 3]> {
    let points = vertices.map(interval_vertex_points);
    Some([points[0].clone()?, points[1].clone()?, points[2].clone()?])
}

fn interval_vertex_points(
    vertex: crate::triangle_grid_coverage::GridVertexInterval,
) -> Option<Vec<[i64; 2]>> {
    let [[x0, x1], [y0, y1]] = vertex.coordinates();
    let width = u64::try_from(i64::from(x1).checked_sub(i64::from(x0))?.checked_add(1)?).ok()?;
    let height = u64::try_from(i64::from(y1).checked_sub(i64::from(y0))?.checked_add(1)?).ok()?;
    let count = width.checked_mul(height)?;
    if count > MAX_ENUMERATED_VERTEX_POINTS {
        return None;
    }
    let mut points = Vec::with_capacity(usize::try_from(count).ok()?);
    for y in y0..=y1 {
        for x in x0..=x1 {
            points.push([i64::from(x), i64::from(y)]);
        }
    }
    Some(points)
}

fn exceeds_predicate_budget(sample_count: usize, point_counts: [usize; 3]) -> bool {
    let combinations = point_counts.into_iter().try_fold(1_u64, |total, count| {
        total.checked_mul(u64::try_from(count).ok()?)
    });
    combinations
        .and_then(|count| count.checked_mul(u64::try_from(sample_count).ok()?))
        .and_then(|count| count.checked_mul(4))
        .is_none_or(|count| count > MAX_EMPTY_PROOF_EDGE_PREDICATES)
}

fn any_possible_sample_is_covered(
    samples: &[[i64; 2]],
    points: &[Vec<[i64; 2]>; 3],
) -> Option<bool> {
    for sample in samples {
        if any_vertex_choice_covers(*sample, points, 0, &mut [[0_i64; 2]; 3])? {
            return Some(true);
        }
    }
    Some(false)
}

fn any_vertex_choice_covers(
    sample: [i64; 2],
    points: &[Vec<[i64; 2]>; 3],
    vertex: usize,
    selected: &mut [[i64; 2]; 3],
) -> Option<bool> {
    if vertex == 3 {
        return closed_triangle_contains(*selected, sample);
    }
    for point in &points[vertex] {
        selected[vertex] = *point;
        if any_vertex_choice_covers(sample, points, vertex + 1, selected)? {
            return Some(true);
        }
    }
    Some(false)
}

fn closed_triangle_contains(triangle: [[i64; 2]; 3], sample: [i64; 2]) -> Option<bool> {
    let area = exact_edge(triangle[0], triangle[1], triangle[2])?;
    if area == 0 {
        return degenerate_triangle_contains(triangle, sample);
    }
    let edges = [
        exact_edge(triangle[0], triangle[1], sample)?,
        exact_edge(triangle[1], triangle[2], sample)?,
        exact_edge(triangle[2], triangle[0], sample)?,
    ];
    Some(edges.iter().all(|edge| *edge >= 0) || edges.iter().all(|edge| *edge <= 0))
}

fn degenerate_triangle_contains(triangle: [[i64; 2]; 3], sample: [i64; 2]) -> Option<bool> {
    for edge in 0..3 {
        let a = triangle[edge];
        let b = triangle[(edge + 1) % 3];
        if exact_edge(a, b, sample)? == 0 && within_segment(sample, a, b) {
            return Some(true);
        }
    }
    Some(false)
}

fn within_segment(point: [i64; 2], a: [i64; 2], b: [i64; 2]) -> bool {
    (0..2).all(|axis| point[axis] >= a[axis].min(b[axis]) && point[axis] <= a[axis].max(b[axis]))
}

fn exact_edge(a: [i64; 2], b: [i64; 2], point: [i64; 2]) -> Option<i128> {
    let dx = i128::from(b[0]) - i128::from(a[0]);
    let dy = i128::from(b[1]) - i128::from(a[1]);
    let px = i128::from(point[0]) - i128::from(a[0]);
    let py = i128::from(point[1]) - i128::from(a[1]);
    dx.checked_mul(py)?.checked_sub(dy.checked_mul(px)?)
}

fn axis_has_no_q12_center([lower, upper]: [i32; 2]) -> bool {
    if lower > upper {
        return false;
    }
    let lower_delta = i64::from(lower) - Q12_PIXEL_CENTER;
    let first_index =
        lower_delta.div_euclid(Q12_SCALE) + i64::from(lower_delta.rem_euclid(Q12_SCALE) != 0);
    first_index * Q12_SCALE + Q12_PIXEL_CENTER > i64::from(upper)
}

fn empty_axis(points: [[f64; 2]; 3], errors: [[f64; 2]; 3], axis: usize) -> bool {
    let mut bounds = [f64::INFINITY, f64::NEG_INFINITY];
    for (point, error) in points.into_iter().zip(errors) {
        let interval = coordinate_interval(point[axis], error[axis]);
        bounds[0] = bounds[0].min(interval[0]);
        bounds[1] = bounds[1].max(interval[1]);
    }
    half_integer_absent(bounds)
}

fn coordinate_interval(point: f64, radius: f64) -> [f64; 2] {
    // Cover rounding of each endpoint operation independently; exact half-grid ties stay included.
    [
        ((point - radius).next_down() - HALF_GRID_STEP).next_down(),
        ((point + radius).next_up() + HALF_GRID_STEP).next_up(),
    ]
}

fn half_integer_absent([lower, upper]: [f64; 2]) -> bool {
    lower.is_finite()
        && upper.is_finite()
        && lower <= upper
        && (lower - 0.5).ceil() > (upper - 0.5).floor()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn q12_interval(x: [i32; 2], y: [i32; 2]) -> crate::triangle_grid_coverage::GridVertexInterval {
        crate::triangle_grid_coverage::GridVertexInterval::new(x, y).unwrap()
    }

    #[test]
    fn actual_refused_microtriangles_have_a_proven_empty_axis() {
        for bounds in [
            [101.001892, 101.121297],
            [198.160859, 198.413177],
            [197.956329, 198.195145],
        ] {
            let points = [[bounds[0], 0.0], [bounds[1], 1.0], [bounds[0], 2.0]];
            assert!(sample_box_empty(points, [[0.0007; 2]; 3]));
            assert!(sample_box_empty(
                points.map(|p| [-p[0], 409.0 - p[1]]),
                [[0.0007; 2]; 3]
            ));
        }
    }

    #[test]
    fn half_integer_boundary_and_snap_uncertainty_refuse_empty_claim() {
        for x in [0.5, 0.5 - HALF_GRID_STEP, 0.5 + HALF_GRID_STEP, -0.5, 101.5] {
            assert!(!sample_box_empty(
                [[x, 0.0], [x, 1.0], [x, 2.0]],
                [[0.0; 2]; 3]
            ));
        }
        assert!(!sample_box_empty(
            [[0.49, 0.0], [0.49, 1.0], [0.49, 2.0]],
            [[0.02; 2]; 3]
        ));
    }

    #[test]
    fn covered_bbox_and_invalid_inputs_never_certify_empty() {
        let points = [[0.1, 0.1], [0.9, 0.1], [0.1, 0.9]];
        assert!(!sample_box_empty(points, [[0.0; 2]; 3]));
        for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -f64::EPSILON] {
            assert!(!sample_box_empty(points, [[invalid; 2]; 3]));
        }
        let mut invalid = points;
        invalid[0][0] = f64::NAN;
        assert!(!sample_box_empty(invalid, [[0.0; 2]; 3]));
    }

    #[test]
    fn outward_interval_and_closed_half_integer_controls() {
        let [lo, hi] = coordinate_interval(0.5, 0.0);
        assert!(lo < 0.5 - HALF_GRID_STEP && hi > 0.5 + HALF_GRID_STEP);
        assert!(half_integer_absent([1.0, 1.25]));
        assert!(!half_integer_absent([1.0, 1.5]));
        assert!(!half_integer_absent([2.0, 1.0]));
        assert!(!half_integer_absent([f64::NEG_INFINITY, 1.0]));
    }

    #[test]
    fn q12_rectangle_excludes_pixel_centers_only_when_a_closed_axis_misses_them() {
        let absent_x = [q12_interval([1000, 1500], [2048, 2048]); 3];
        let center_on_x_bound = [q12_interval([1024, 2048], [2048, 2048]); 3];
        let spans_center = [q12_interval([2047, 2049], [2048, 2048]); 3];
        assert!(q12_interval_box_empty(absent_x));
        assert!(!q12_interval_box_empty(center_on_x_bound));
        assert!(!q12_interval_box_empty(spans_center));
    }

    #[test]
    fn generated_q12_rectangles_match_independent_integer_lattice_search() {
        for seed in 0..1024_i32 {
            let vertices = std::array::from_fn(|index| {
                let base = (seed * 7919 + index as i32 * 104729).rem_euclid(1_000_000) - 500_000;
                let x = [base, base + (seed % 2048)];
                let y = [base / 3, base / 3 + (seed % 4096)];
                q12_interval(x, y)
            });
            assert_eq!(
                q12_interval_box_empty(vertices),
                rectangle_has_no_sample(vertices)
            );
        }
    }

    fn rectangle_has_no_sample(
        vertices: [crate::triangle_grid_coverage::GridVertexInterval; 3],
    ) -> bool {
        let bounds = std::array::from_fn::<_, 2, _>(|axis| {
            vertices
                .map(|vertex| vertex.coordinates()[axis])
                .into_iter()
                .fold([i32::MAX, i32::MIN], |range, interval| {
                    [range[0].min(interval[0]), range[1].max(interval[1])]
                })
        });
        [0, 1].into_iter().any(|axis_index| {
            (-128..=128).all(|pixel| {
                let center = pixel * 4096 + 2048;
                center < bounds[axis_index][0] || center > bounds[axis_index][1]
            })
        })
    }
    #[test]
    fn generated_certificates_exclude_every_reference_sample_after_corner_perturbations() {
        for seed in 0..512 {
            let points = generated_points(seed);
            let radius = [0.0, 0.0007, 0.01, 0.125][seed as usize % 4];
            if sample_box_empty(points, [[radius; 2]; 3]) {
                assert_corner_samples_absent(points, radius);
            }
        }
    }

    fn assert_corner_samples_absent(points: [[f64; 2]; 3], radius: f64) {
        for corner in 0..64 {
            assert_integer_samples_absent(perturbed_grid(points, radius, corner));
        }
    }

    fn assert_integer_samples_absent(triangle: [[i64; 2]; 3]) {
        for x in -3..=3 {
            for y in -3..=3 {
                assert!(!reference_inside(
                    triangle,
                    [x * 4096 + 2048, y * 4096 + 2048]
                ));
            }
        }
    }

    fn generated_points(seed: u64) -> [[f64; 2]; 3] {
        std::array::from_fn(|i| {
            std::array::from_fn(|axis| {
                let mixed = (seed + 1)
                    .wrapping_mul(6364136223846793005)
                    .rotate_left((i * 13 + axis * 7) as u32);
                ((mixed % 32769) as f64 - 16384.0) / 8192.0
            })
        })
    }

    fn perturbed_grid(points: [[f64; 2]; 3], radius: f64, corner: u32) -> [[i64; 2]; 3] {
        std::array::from_fn(|i| {
            std::array::from_fn(|axis| {
                let sign = if corner & (1 << (i * 2 + axis)) == 0 {
                    -1.0
                } else {
                    1.0
                };
                ((points[i][axis] + sign * radius) * 4096.0).round_ties_even() as i64
            })
        })
    }

    fn reference_inside(triangle: [[i64; 2]; 3], point: [i64; 2]) -> bool {
        let edge = |a: [i64; 2], b: [i64; 2], p: [i64; 2]| {
            i128::from(b[0] - a[0]) * i128::from(p[1] - a[1])
                - i128::from(b[1] - a[1]) * i128::from(p[0] - a[0])
        };
        if edge(triangle[0], triangle[1], triangle[2]) == 0 {
            return false;
        }
        let signs =
            std::array::from_fn::<_, 3, _>(|i| edge(triangle[i], triangle[(i + 1) % 3], point));
        signs.into_iter().all(|v| v >= 0) || signs.into_iter().all(|v| v <= 0)
    }
}
