// SPDX-License-Identifier: GPL-2.0-only
use super::*;

fn plane(origin: [f64; 3], normal: [f64; 3]) -> SegmentWorkPlane {
    SegmentWorkPlane::new(origin, normal).unwrap()
}

fn cross(endpoints: [[f64; 3]; 2], plane: SegmentWorkPlane) -> SegmentPlaneCut {
    let SegmentPlaneIntersection::Crossing(cut) =
        intersect_segment_plane(endpoints, plane).unwrap()
    else {
        panic!("expected an interior crossing");
    };
    cut
}

#[test]
fn an_oblique_cut_retains_the_exact_ratio_and_binary64_enclosure() {
    // x+y+z=1, on [0,0,0] + t[1,2,0] gives t=1/3 and [1/3,2/3,0].
    let points = [[0.; 3], [1., 2., 0.]];
    let cut = cross(points, plane([1., 0., 0.], [1.; 3]));
    let third: f64 = 1. / 3.;
    let two_thirds: f64 = 2. / 3.;
    assert_eq!(cut.parameter, [third, third.next_up()]);
    assert_eq!(
        cut.coordinates,
        [cut.parameter, [two_thirds, two_thirds.next_up()], [0.; 2]]
    );
    assert_eq!(cut.position, [third, two_thirds, 0.]);
    let one_gap = 2_f64.powi(-54) / 3.;
    assert!(cut.error_upper >= one_gap * 5_f64.sqrt());
    assert!(cut.error_upper < 5e-17);
    assert_eq!(
        cut,
        cross(points, plane([1., 0., 0.], [-f64::from_bits(1); 3]))
    );
}

#[test]
fn reversing_the_segment_complements_the_parameter_and_preserves_cut_evidence() {
    let work_plane = plane([1., 0., 0.], [1.; 3]);
    let start = [0.; 3];
    let end = [1., 2., 0.];
    let forward = cross([start, end], work_plane);
    let backward = cross([end, start], work_plane);
    let two_thirds: f64 = 2. / 3.;
    assert_eq!(backward.parameter, [two_thirds, two_thirds.next_up()]);
    assert_eq!(forward.coordinates, backward.coordinates);
    assert_eq!(forward.position, backward.position);
    assert_eq!(forward.error_upper, backward.error_upper);
}

#[test]
fn original_endpoint_signs_survive_overflow_and_exact_midpoint_rounding() {
    let end = f64::MAX;
    let spatial = [[-end, -end, 0.], [end, end, 0.]];
    let cut = cross(spatial, plane([0.; 3], [end, end, 0.]));
    assert_eq!(cut.position, [0.; 3]);
    assert_eq!(cut.parameter, [0.5; 2]);
    assert_eq!(cut.error_upper, 0.);

    for [start, end, expected] in [
        [1., 1_f64.next_up(), 1.],
        [
            1_f64.next_up(),
            1_f64.next_up().next_up(),
            1_f64.next_up().next_up(),
        ],
    ] {
        let cut = cross(
            [[-1., start, 0.], [1., end, 0.]],
            plane([0.; 3], [1., 0., 0.]),
        );
        assert_eq!(cut.position[1], expected);
        assert_eq!(cut.coordinates[1], [start, end]);
        assert_eq!(cut.error_upper, 2_f64.powi(-53));
    }
}

#[test]
fn endpoints_and_overlap_are_distinct_from_a_completed_miss() {
    use SegmentPlaneIntersection::{Disjoint, End, Overlap, Start};
    let yz = plane([0.; 3], [1., 0., 0.]);
    for (points, expected) in [
        ([[1., 0., 0.], [2., 0., 0.]], Disjoint),
        ([[0., 0., 0.], [1., 0., 0.]], Start),
        ([[1., 0., 0.], [0., 0., 0.]], End),
        ([[0., 0., 0.], [0., 1., 0.]], Overlap),
    ] {
        assert_eq!(intersect_segment_plane(points, yz), Ok(expected));
    }
    assert_eq!(
        intersect_segment_plane([[0.; 3]; 2], yz),
        Err(SegmentPlaneFailure::CoincidentEndpoints)
    );
}

#[test]
fn a_proved_interior_parameter_can_round_to_a_binary64_endpoint() {
    let smallest = f64::from_bits(1);
    let cut = cross(
        [[0.; 3], [2., 0., 0.]],
        plane([smallest, 0., 0.], [1., 0., 0.]),
    );
    assert_eq!(cut.parameter, [0., smallest]);
    assert_eq!(cut.position, [smallest, 0., 0.]);
    assert_eq!(cut.error_upper, 0.);
}

#[test]
fn malformed_plane_and_segment_coefficients_report_the_actual_operand() {
    assert_eq!(
        SegmentWorkPlane::new([0.; 3], [-0.; 3]),
        Err(SegmentPlaneFailure::ZeroNormal)
    );
    for (origin, normal, point, axis) in [
        ([f64::INFINITY, 0., 0.], [1., 0., 0.], 2, 0),
        ([0.; 3], [1., 0., f64::INFINITY], 3, 2),
    ] {
        assert!(matches!(
            SegmentWorkPlane::new(origin, normal),
            Err(SegmentPlaneFailure::Coordinate(cause))
                if cause.point == point && cause.axis == axis && cause.value == f64::INFINITY
        ));
    }
    assert!(matches!(
        intersect_segment_plane([[0.; 3], [0., f64::NEG_INFINITY, 0.]], plane([0.; 3], [1., 0., 0.])),
        Err(SegmentPlaneFailure::Coordinate(cause))
            if cause.point == 1 && cause.axis == 1 && cause.value == f64::NEG_INFINITY
    ));
}
