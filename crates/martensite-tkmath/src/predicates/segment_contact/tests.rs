// SPDX-License-Identifier: GPL-2.0-only
use super::{SegmentContact3d, exact_segment_contact_3d};

fn check(first: [[f64; 3]; 2], second: [[f64; 3]; 2], expected: SegmentContact3d) {
    for a in [first, [first[1], first[0]]] {
        for b in [second, [second[1], second[0]]] {
            assert_eq!(exact_segment_contact_3d(a, b), Ok(expected));
            assert_eq!(exact_segment_contact_3d(b, a), Ok(expected));
        }
    }
}

#[test]
fn coplanar_crossing_and_skew_with_overlapping_projections() {
    let diagonal = [[0., 0., 0.], [4., 4., 4.]];
    check(
        diagonal,
        [[0., 4., 0.], [4., 0., 4.]],
        SegmentContact3d::Point,
    );
    check(
        diagonal,
        [[0., 3., 1.], [4., 1., 4.]],
        SegmentContact3d::Disjoint,
    );
    check(
        diagonal,
        [[2., 3., 2.], [4., 5., 4.]],
        SegmentContact3d::Disjoint,
    );
}

#[test]
fn touch_overlap_and_disjoint_collinear_intervals() {
    let first = [[0., 0., 0.], [3., 6., 9.]];
    check(
        first,
        [[3., 6., 9.], [4., 8., 12.]],
        SegmentContact3d::Point,
    );
    check(
        first,
        [[2., 4., 6.], [5., 10., 15.]],
        SegmentContact3d::Overlap,
    );
    check(
        first,
        [[4., 8., 12.], [5., 10., 15.]],
        SegmentContact3d::Disjoint,
    );
}

#[test]
fn point_segments_coincide_only_on_the_exact_support_interval() {
    let line = [[0., 0., 0.], [2., 2., 2.]];
    check(line, [[1., 1., 1.]; 2], SegmentContact3d::Point);
    check(line, [[1., 1., 1.5]; 2], SegmentContact3d::Disjoint);
    check(
        [[1., 1., 1.]; 2],
        [[1., 1., 1.]; 2],
        SegmentContact3d::Point,
    );
    check([[0.; 3]; 2], [[0., 0., 1.]; 2], SegmentContact3d::Disjoint);
}

#[test]
fn exact_input_survives_underflow_and_overflow_of_float_determinants() {
    for scale in [f64::from_bits(1), martensite_cmath::powi(2., 900)] {
        let first = [[0.; 3], [2. * scale; 3]];
        let second = [[0., 2. * scale, 0.], [2. * scale, 0., 2. * scale]];
        check(first, second, SegmentContact3d::Point);
    }
}

#[test]
fn disjoint_boxes_do_not_hide_nonfinite_input() {
    let first = [[0.; 3], [1.; 3]];
    let second = [[20.; 3], [21., 21., f64::INFINITY]];
    let failed = exact_segment_contact_3d(first, second).unwrap_err();
    assert_eq!(
        (failed.point, failed.axis, failed.value),
        (3, 2, f64::INFINITY)
    );
}
