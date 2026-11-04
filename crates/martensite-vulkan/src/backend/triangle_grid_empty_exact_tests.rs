use super::sample::{
    Q12EmptyCertificate as Certificate, Q12EmptyRefusal as Refusal, q12_interval_vertices_empty,
};
use crate::triangle_grid_coverage::GridVertexInterval;

fn interval(x: [i32; 2], y: [i32; 2]) -> GridVertexInterval {
    GridVertexInterval::new(x, y).unwrap()
}

#[test]
fn triangle671_full_outward_boxes_prove_all_bbox_centers_empty() {
    let vertices = [
        interval([798572, 798573], [263189, 263191]),
        interval([826329, 826330], [264228, 264230]),
        interval([812506, 812507], [263711, 263713]),
    ];
    assert_eq!(q12_interval_vertices_empty(vertices), Certificate::Empty);
    assert!(!super::sample::q12_interval_box_empty(vertices));
    assert!(!independent_reference_finds_coverage(vertices));
}

#[test]
fn triangle671_actual_draw_projects_to_the_enumerated_interval_boxes() {
    let draw = triangle671_draw();
    let projected =
        super::super::super::triangle_grid_projection_intervals::projected_q12_intervals(
            &draw,
            &[0, 1, 2],
            [0.0, 410.0, 409.0, -409.0],
        )
        .unwrap();
    assert_triangle671_bounds(projected);
}

fn triangle671_draw() -> crate::PrimitiveDraw {
    // bug29791 draw0 ordinal671 from the bound source witness (geometry SHA aae63ffeb38ba9162b00e59f9150104635cc2cd30852666fb494257ea5e828f3).
    let position_bits = [
        [3218276137, 1089084363, 1112517015],
        [3203675907, 1089437784, 1112505530],
        [3213343127, 1089312480, 1112511243],
    ];
    let transform_bits = [
        1021820257, 0, 0, 0, 0, 0, 1032357976, 0, 0, 1021820257, 0, 0, 0, 3209111231, 0, 1065353216,
    ];
    let vertices =
        position_bits.map(|bits| crate::RenderVertex::new(bits.map(f32::from_bits), [1.0; 4]));
    crate::PrimitiveDraw::new(
        vertices.into(),
        vec![0, 1, 2],
        transform_bits.map(f32::from_bits),
    )
    .unwrap()
}

fn assert_triangle671_bounds(projected: [GridVertexInterval; 3]) {
    assert_eq!(
        projected.map(GridVertexInterval::coordinates),
        [
            [[798572, 798573], [263189, 263191]],
            [[826329, 826330], [264228, 264230]],
            [[812506, 812507], [263711, 263713]],
        ]
    );
    assert_eq!(q12_interval_vertices_empty(projected), Certificate::Empty);
}

#[test]
fn exact_enumeration_matches_independent_small_integer_domain_reference() {
    for seed in 0..96_i32 {
        let vertices = std::array::from_fn(|vertex| {
            let x = 2048 + (seed * 7 + vertex as i32 * 11).rem_euclid(5) - 2;
            let y = 2048 + (seed * 13 + vertex as i32 * 17).rem_euclid(5) - 2;
            let width = (seed + vertex as i32).rem_euclid(2);
            interval([x, x + width], [y, y + width])
        });
        let expected_empty = !independent_reference_finds_coverage(vertices);
        match q12_interval_vertices_empty(vertices) {
            Certificate::Empty => assert!(expected_empty, "seed {seed}"),
            Certificate::NotProven(Refusal::SampleMayBeCovered) => {
                assert!(!expected_empty, "seed {seed}")
            }
            refusal => panic!("unexpected bounded-reference result for seed {seed}: {refusal:?}"),
        }
    }
}

#[test]
fn vertex_order_reversal_and_closed_boundaries_remain_potentially_covered() {
    assert_triangle_and_reverse_are_covered();
    assert_degenerate_boundary_is_covered();
}

fn assert_triangle_and_reverse_are_covered() {
    let triangle = [
        interval([0, 0], [0, 0]),
        interval([4096, 4096], [0, 0]),
        interval([0, 0], [4096, 4096]),
    ];
    assert_eq!(
        q12_interval_vertices_empty(triangle),
        Certificate::NotProven(Refusal::SampleMayBeCovered)
    );
    assert_eq!(
        q12_interval_vertices_empty([triangle[0], triangle[2], triangle[1]]),
        Certificate::NotProven(Refusal::SampleMayBeCovered)
    );
}

fn assert_degenerate_boundary_is_covered() {
    let degenerate_boundary = [
        interval([0, 0], [2048, 2048]),
        interval([4096, 4096], [2048, 2048]),
        interval([8192, 8192], [2048, 2048]),
    ];
    assert_eq!(
        q12_interval_vertices_empty(degenerate_boundary),
        Certificate::NotProven(Refusal::SampleMayBeCovered)
    );
}

#[test]
fn enumeration_returns_explicit_work_budget_refusal() {
    let broad = interval([2044, 2051], [2044, 2051]);
    assert_eq!(
        q12_interval_vertices_empty([broad; 3]),
        Certificate::NotProven(Refusal::WorkBudgetExceeded)
    );
}

fn independent_reference_finds_coverage(vertices: [GridVertexInterval; 3]) -> bool {
    let points = vertices.map(enumerate_points);
    let coordinates = vertices.map(GridVertexInterval::coordinates);
    let x = sample_range(coordinates.map(|vertex| vertex[0]));
    let y = sample_range(coordinates.map(|vertex| vertex[1]));
    for sample_y in y {
        if any_reference_triangle_covers(sample_y, &x, &points) {
            return true;
        }
    }
    false
}

fn any_reference_triangle_covers(
    sample_y: i64,
    x_samples: &[i64],
    points: &[Vec<[i64; 2]>; 3],
) -> bool {
    x_samples.iter().any(|sample_x| {
        reference_choices_cover([*sample_x, sample_y], points, 0, &mut [[0_i64; 2]; 3])
    })
}

fn reference_choices_cover(
    sample: [i64; 2],
    points: &[Vec<[i64; 2]>; 3],
    vertex: usize,
    selected: &mut [[i64; 2]; 3],
) -> bool {
    if vertex == 3 {
        return reference_closed_membership(*selected, sample);
    }
    for point in &points[vertex] {
        selected[vertex] = *point;
        if reference_choices_cover(sample, points, vertex + 1, selected) {
            return true;
        }
    }
    false
}

fn enumerate_points(vertex: GridVertexInterval) -> Vec<[i64; 2]> {
    let [[x0, x1], [y0, y1]] = vertex.coordinates();
    (y0..=y1)
        .flat_map(|y| (x0..=x1).map(move |x| [i64::from(x), i64::from(y)]))
        .collect()
}

fn sample_range(intervals: [[i32; 2]; 3]) -> Vec<i64> {
    let lower = intervals.map(|range| range[0]).into_iter().min().unwrap();
    let upper = intervals.map(|range| range[1]).into_iter().max().unwrap();
    let first = (i64::from(lower) - 2048 + 4095).div_euclid(4096);
    let last = (i64::from(upper) - 2048).div_euclid(4096);
    (first..=last).map(|index| index * 4096 + 2048).collect()
}

fn reference_closed_membership(triangle: [[i64; 2]; 3], point: [i64; 2]) -> bool {
    let area = reference_cross(
        subtract(triangle[1], triangle[0]),
        subtract(triangle[2], triangle[0]),
    );
    reference_membership_for_area(triangle, point, area)
}

fn reference_membership_for_area(triangle: [[i64; 2]; 3], point: [i64; 2], area: i128) -> bool {
    if area == 0 {
        return reference_collinear_contains(triangle, point);
    }
    let edges: [i128; 3] = std::array::from_fn(|edge| {
        reference_cross(
            subtract(triangle[(edge + 1) % 3], triangle[edge]),
            subtract(point, triangle[edge]),
        )
    });
    if area > 0 {
        edges.into_iter().all(|edge| edge >= 0)
    } else {
        edges.into_iter().all(|edge| edge <= 0)
    }
}

fn reference_collinear_contains(triangle: [[i64; 2]; 3], point: [i64; 2]) -> bool {
    (0..3).any(|edge| {
        let a = triangle[edge];
        let b = triangle[(edge + 1) % 3];
        reference_cross(subtract(b, a), subtract(point, a)) == 0
            && between(point[0], a[0], b[0])
            && between(point[1], a[1], b[1])
    })
}

fn between(value: i64, first: i64, second: i64) -> bool {
    value >= first.min(second) && value <= first.max(second)
}

fn subtract(a: [i64; 2], b: [i64; 2]) -> [i64; 2] {
    [a[0] - b[0], a[1] - b[1]]
}

fn reference_cross(a: [i64; 2], b: [i64; 2]) -> i128 {
    i128::from(a[0]) * i128::from(b[1]) - i128::from(a[1]) * i128::from(b[0])
}
