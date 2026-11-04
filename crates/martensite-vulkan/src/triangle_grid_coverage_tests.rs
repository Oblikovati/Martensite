use super::*;

fn vertex(x: i32, y: i32) -> GridVertex {
    GridVertex { x, y }
}

fn square_halves() -> [GridTriangle; 2] {
    let unit = 4 * 4096;
    [
        GridTriangle::new([vertex(0, 0), vertex(unit, 0), vertex(0, unit)]),
        GridTriangle::new([vertex(unit, 0), vertex(unit, unit), vertex(0, unit)]),
    ]
}

#[test]
fn hardware_default_preserves_replay() {
    assert_eq!(
        RenderTriangleCoverageBehavior::default(),
        RenderTriangleCoverageBehavior::HardwareV1
    );
}

#[test]
fn legacy_behavior_versions_remain_stable() {
    assert_eq!(
        RenderTriangleCoverageBehavior::HardwareV1.behavior_version(),
        1
    );
    assert_eq!(
        RenderTriangleCoverageBehavior::FixedGrid12V2 {
            origin: RenderTriangleGridOrigin::BottomUp,
        }
        .behavior_version(),
        2
    );
    assert_eq!(
        RenderTriangleCoverageBehavior::FixedGrid12V3 {
            origin: RenderTriangleGridOrigin::TopDown,
        }
        .behavior_version(),
        3
    );
}

#[test]
fn native_grid_v4_behavior_version_is_four() {
    let behavior = RenderTriangleCoverageBehavior::NativeGrid12V4 {
        origin: RenderTriangleGridOrigin::TopDown,
    };
    assert_eq!(behavior.behavior_version(), 4);
}

#[test]
fn v4_failure_messages_report_device_and_q12_requirements() {
    let property = RenderTriangleCoverageFailure::MissingDegenerateTriangleRasterization;
    assert!(
        property
            .to_string()
            .contains("degenerateTrianglesRasterized=false")
    );
}

#[test]
fn snap_uses_nearest_even_and_refuses_invalid_ranges() {
    assert_eq!(
        GridVertex::snap([0.5 / 4096.0, 1.5 / 4096.0]),
        Some(vertex(0, 2))
    );
    assert_eq!(GridVertex::snap([-1.5 / 4096.0, 0.0]), Some(vertex(-2, 0)));
    assert_eq!(GridVertex::snap([f64::NAN, 0.0]), None);
    assert_eq!(GridVertex::snap([f64::INFINITY, 0.0]), None);
    assert_eq!(GridVertex::snap([f64::MAX, 0.0]), None);
}

#[test]
fn public_coverage_selections_are_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<RenderTriangleCoverageBehavior>();
    assert_send_sync::<RenderTriangleGridOrigin>();
    assert_send_sync::<RenderTriangleCoverageFailure>();
}

#[test]
fn complete_square_operation_has_exactly_one_owner_per_shared_edge_sample() {
    for origin in [
        RenderTriangleGridOrigin::BottomUp,
        RenderTriangleGridOrigin::TopDown,
    ] {
        let halves = square_halves();
        for y in 0..4 {
            for x in 0..4 {
                let covered = halves.map(|triangle| triangle.covers([x, y], origin));
                assert_ne!(covered[0], covered[1], "sample ({x},{y}) {origin:?}");
            }
        }
    }
}

#[test]
fn half_open_edge_choice_changes_with_declared_framebuffer_origin() {
    let [first, second] = square_halves();
    assert_eq!(
        [
            covered_count(first, RenderTriangleGridOrigin::BottomUp),
            covered_count(second, RenderTriangleGridOrigin::BottomUp),
        ],
        [6, 10]
    );
    assert_eq!(
        [
            covered_count(first, RenderTriangleGridOrigin::TopDown),
            covered_count(second, RenderTriangleGridOrigin::TopDown),
        ],
        [6, 10]
    );
}

#[test]
fn right_triangle_matches_closed_form_integer_sample_membership() {
    let triangle = square_halves()[0];
    for y in 0..4 {
        for x in 0..4 {
            let bottom_up = triangle.covers([x, y], RenderTriangleGridOrigin::BottomUp);
            let top_down = triangle.covers([x, y], RenderTriangleGridOrigin::TopDown);
            assert_eq!(bottom_up, x + y < 3, "bottom-up sample ({x},{y})");
            assert_eq!(top_down, x + y < 3, "top-down sample ({x},{y})");
        }
    }
}

#[test]
fn top_down_rule_owns_top_and_left_edges_after_fractional_translation() {
    let unit = 4096;
    let triangle = GridTriangle::new([
        vertex(unit / 2, unit / 2),
        vertex(9 * unit / 2, unit / 2),
        vertex(unit / 2, 9 * unit / 2),
    ]);
    assert!(triangle.covers([2, 0], RenderTriangleGridOrigin::TopDown));
    assert!(triangle.covers([0, 2], RenderTriangleGridOrigin::TopDown));
    assert!(!triangle.covers([2, 2], RenderTriangleGridOrigin::TopDown));
}

#[test]
fn negative_height_viewport_reflection_preserves_logical_pixel_ownership() {
    let [first, second] = square_halves();
    for triangle in [first, second] {
        let reflected = GridTriangle::new(triangle.vertices.map(|point| GridVertex {
            x: point.x,
            y: 4 * 4096 - point.y,
        }));
        for y in 0..4 {
            for x in 0..4 {
                assert_eq!(
                    triangle.covers([x, 3 - y], RenderTriangleGridOrigin::BottomUp),
                    reflected.covers([x, y], RenderTriangleGridOrigin::TopDown),
                    "sample ({x},{y})"
                );
            }
        }
    }
}

fn covered_count(triangle: GridTriangle, origin: RenderTriangleGridOrigin) -> usize {
    (0..4)
        .flat_map(|y| (0..4).map(move |x| [x, y]))
        .filter(|pixel| triangle.covers(*pixel, origin))
        .count()
}

#[test]
fn reversed_winding_preserves_coverage_but_changes_facing() {
    let [first, _] = square_halves();
    let reversed = GridTriangle::new([vertex(0, 0), vertex(0, 4 * 4096), vertex(4 * 4096, 0)]);
    assert_eq!(first.signed_area(), -reversed.signed_area());
    assert!(first.covers([0, 0], RenderTriangleGridOrigin::BottomUp));
    assert!(reversed.covers([0, 0], RenderTriangleGridOrigin::BottomUp));
    assert!(!first.survives_culling(true, TriangleFaceCull::Back));
    assert!(reversed.survives_culling(true, TriangleFaceCull::Back));
    assert!(first.survives_culling(true, TriangleFaceCull::Front));
    assert!(!reversed.survives_culling(true, TriangleFaceCull::Front));
}

#[test]
fn snapped_degenerate_triangle_has_no_coverage_or_facing() {
    let triangle = GridTriangle::new([vertex(0, 0), vertex(4096, 4096), vertex(8192, 8192)]);
    assert_eq!(triangle.signed_area(), 0);
    assert!(!triangle.covers([0, 0], RenderTriangleGridOrigin::BottomUp));
    assert!(!triangle.survives_culling(true, TriangleFaceCull::None));
}

#[test]
fn device_grid_admission_rejects_triangles_that_may_collapse() {
    let large = square_halves()[0];
    let tiny = GridTriangle::new([vertex(0, 0), vertex(1, 0), vertex(0, 1)]);
    assert!(large.survives_grid(8, 0.0));
    assert!(!tiny.survives_grid(8, 0.0));
    assert!(large.survives_grid(12, 0.0));
    assert!(!tiny.survives_grid(u32::MAX, 0.0));
    assert!(!large.survives_grid(8, f64::NAN));
}

#[test]
fn q12_stable_triangle_may_collapse_on_eight_bit_grid() {
    let triangle = GridTriangle::new([vertex(-7, -7), vertex(7, -7), vertex(-7, 7)]);
    assert!(triangle.survives_grid(12, 0.0));
    assert!(!triangle.survives_grid(8, 0.0));
}

#[test]
fn near_collinear_interval_proves_nonzero_area_without_widening_error_bounds() {
    let triangle = reported_triangle_122();
    assert!(triangle.survives_grid(8, 0.001));
    assert!(!triangle.survives_grid(8, 0.01));
    assert_eq!(
        interval_area_range(triangle.vertices, 22),
        (1_240_020, 11_647_252)
    );
    assert_eq!(
        covered_pixels(triangle, [248, 260], [38, 56]),
        vec![[255, 48]]
    );
    assert_reversed_interval(triangle);
}

fn assert_reversed_interval(triangle: GridTriangle) {
    let reversed = GridTriangle::new([
        triangle.vertices[0],
        triangle.vertices[2],
        triangle.vertices[1],
    ]);
    assert!(reversed.survives_grid(8, 0.001));
    assert_eq!(
        interval_area_range(reversed.vertices, 22),
        (-11_647_252, -1_240_020)
    );
}

#[test]
fn interval_corner_extrema_match_independent_f64_enumeration() {
    let vertices = [vertex(13, -7), vertex(103, 21), vertex(37, 109)];
    let expected = independent_corner_area_range(vertices, 19);
    assert_eq!(interval_area_range(vertices, 19), expected);
}

#[test]
fn coordinate_interval_area_proves_orientation_and_rejects_zero_crossing() {
    let positive = [
        GridVertexInterval::new([1014062, 1014062], [278330, 278332]).unwrap(),
        GridVertexInterval::new([1014063, 1014064], [278312, 278314]).unwrap(),
        GridVertexInterval::new([1012647, 1012648], [308581, 308583]).unwrap(),
    ];
    assert!(interval_area_excludes_zero(positive));
    assert!(interval_area_excludes_zero([
        positive[0],
        positive[2],
        positive[1]
    ]));
    let crossing = [
        GridVertexInterval::new([-2, 2], [-2, 2]).unwrap(),
        GridVertexInterval::new([0, 4], [-2, 2]).unwrap(),
        GridVertexInterval::new([-2, 2], [0, 4]).unwrap(),
    ];
    assert!(!interval_area_excludes_zero(crossing));
}

#[test]
fn coordinate_interval_area_rejects_invalid_ranges_and_matches_corner_reference() {
    let intervals = [
        GridVertexInterval::new([13, 15], [-7, -5]).unwrap(),
        GridVertexInterval::new([103, 105], [21, 23]).unwrap(),
        GridVertexInterval::new([37, 39], [109, 111]).unwrap(),
    ];
    assert_eq!(
        interval_area_range_for_vertices(intervals),
        independent_interval_area_range(intervals)
    );
    assert!(GridVertexInterval::new([2, 1], [0, 0]).is_none());
    assert!(GridVertexInterval::new([0, i32::MAX], [0, 0]).is_none());
}

fn independent_interval_area_range(vertices: [GridVertexInterval; 3]) -> (i128, i128) {
    let mut values = Vec::new();
    for corner in 0..64 {
        let points = std::array::from_fn(|index| {
            [0, 1].map(|axis| {
                let endpoint = (corner >> (2 * index + axis)) & 1;
                i128::from(vertices[index].coordinates[axis][endpoint])
            })
        });
        values.push(interval_area(points));
    }
    (*values.iter().min().unwrap(), *values.iter().max().unwrap())
}

fn independent_corner_area_range(vertices: [GridVertex; 3], radius: i128) -> (i128, i128) {
    let mut values = Vec::new();
    for mask in 0..64 {
        let p = std::array::from_fn(|index| reference_corner(vertices[index], radius, mask, index));
        values.push(reference_area(p));
    }
    (
        values.iter().copied().min().unwrap(),
        values.iter().copied().max().unwrap(),
    )
}

fn reference_corner(vertex: GridVertex, radius: i128, mask: u32, index: usize) -> [f64; 2] {
    [0, 1].map(|axis| {
        let sign = if mask & (1 << (2 * index + axis)) == 0 {
            -1.0
        } else {
            1.0
        };
        let value = if axis == 0 { vertex.x } else { vertex.y };
        value as f64 + sign * radius as f64
    })
}

fn reference_area(points: [[f64; 2]; 3]) -> i128 {
    let [[ax, ay], [bx, by], [cx, cy]] = points;
    let determinant = (bx - ax) * (cy - ay) - (by - ay) * (cx - ax);
    determinant as i128
}

#[test]
fn interval_radius_refuses_nonfinite_and_out_of_range_inputs() {
    let triangle = reported_triangle_122();
    assert!(!triangle.survives_grid(8, f64::MAX));
    assert!(!triangle.survives_grid(8, f64::INFINITY));
    assert!(!triangle.survives_grid(8, -f64::EPSILON));
}

fn reported_triangle_122() -> GridTriangle {
    GridTriangle::new([
        vertex(1_065_752, 226_920),
        vertex(1_017_804, 156_604),
        vertex(1_040_199, 189_312),
    ])
}

fn covered_pixels(triangle: GridTriangle, x: [i32; 2], y: [i32; 2]) -> Vec<[i32; 2]> {
    (y[0]..y[1])
        .flat_map(|row| (x[0]..x[1]).map(move |column| [column, row]))
        .filter(|pixel| triangle.covers([pixel[0], pixel[1]], RenderTriangleGridOrigin::TopDown))
        .collect()
}

#[test]
fn empty_carrier_version_is_explicit_without_changing_hardware_or_v2() {
    assert_eq!(
        RenderTriangleCoverageBehavior::FixedGrid12V3 {
            origin: RenderTriangleGridOrigin::BottomUp
        }
        .behavior_version(),
        3
    );
    assert_eq!(
        RenderTriangleCoverageBehavior::FixedGrid12V2 {
            origin: RenderTriangleGridOrigin::BottomUp
        }
        .behavior_version(),
        2
    );
    assert_eq!(
        RenderTriangleCoverageBehavior::default().behavior_version(),
        1
    );
}

#[test]
fn q8_collapse_does_not_define_v4_q12_logical_winding() {
    use crate::RenderViewportGridPrecision::{Fractional8, Fractional12};
    let screen = q8_collapse_triangle();
    let q8 = screen.map(|point| point.map(|value| Fractional8.snap(value).unwrap()));
    assert!(q8.iter().all(|point| point[1] == q8[0][1]));
    assert_eq!(screen_area(q8), 0.0);
    let q12 =
        screen.map(|point| point.map(|value| (Fractional12.snap(value).unwrap() * 4096.0) as i32));
    let triangle = GridTriangle::new(q12.map(|point| vertex(point[0], point[1])));
    assert_eq!(triangle.front_facing(true), Some(false));
    assert_ne!(triangle.signed_area(), 0);
}

#[test]
fn v4_logical_facing_applies_requested_culling_after_q8_collapse() {
    let triangle = q12_collapse_triangle();
    assert!(triangle.survives_culling(true, TriangleFaceCull::None));
    assert!(!triangle.survives_culling(true, TriangleFaceCull::Back));
    assert!(triangle.survives_culling(true, TriangleFaceCull::Front));
    let reversed = reverse_triangle(triangle);
    assert_eq!(reversed.front_facing(true), Some(true));
    assert!(reversed.survives_culling(true, TriangleFaceCull::Back));
    assert!(!reversed.survives_culling(true, TriangleFaceCull::Front));
}

fn q8_collapse_triangle() -> [[f64; 2]; 3] {
    [
        [32.25, 32.4990234375],
        [32.75, 32.4990234375],
        [32.5, 32.5009765625],
    ]
}

fn q12_collapse_triangle() -> GridTriangle {
    let screen = q8_collapse_triangle();
    let points = screen.map(|point| {
        point.map(|value| {
            (crate::RenderViewportGridPrecision::Fractional12
                .snap(value)
                .unwrap()
                * 4096.0) as i32
        })
    });
    GridTriangle::new(points.map(|point| vertex(point[0], point[1])))
}

fn reverse_triangle(triangle: GridTriangle) -> GridTriangle {
    GridTriangle::new([
        triangle.vertices[0],
        triangle.vertices[2],
        triangle.vertices[1],
    ])
}

fn screen_area(points: [[f64; 2]; 3]) -> f64 {
    let [[ax, ay], [bx, by], [cx, cy]] = points;
    (bx - ax) * (cy - ay) - (by - ay) * (cx - ax)
}
