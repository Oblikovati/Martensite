// SPDX-License-Identifier: GPL-2.0-only
use super::*;

#[test]
fn exact_quarter_restriction_matches_independent_quadratic_values() {
    let mut cancelled = || false;
    let mut arithmetic = Arithmetic::new(Default::default(), &mut cancelled);
    let source = capture_planar(&[[0., 1.], [2., 3.], [4., 1.]], &mut arithmetic).unwrap();
    let x = restrict(&source[0], 1, 2, &mut arithmetic).unwrap();
    let z = restrict(&source[1], 1, 2, &mut arithmetic).unwrap();
    let expected = capture_planar(&[[1., 1.75], [1.5, 2.], [2., 2.]], &mut arithmetic).unwrap();
    assert_eq!([x, z], expected);
}

#[test]
fn derivative_and_constant_coordinate_compression_preserve_the_polynomial() {
    let mut cancelled = || false;
    let mut arithmetic = Arithmetic::new(Default::default(), &mut cancelled);
    let source = capture_planar(&[[0., 3.], [2., 3.], [4., 3.]], &mut arithmetic).unwrap();
    assert_eq!(source[1].len(), 1);
    assert_eq!(
        derivative(&source[0], &mut arithmetic).unwrap(),
        vec![arithmetic.scalar(4.).unwrap(); 2]
    );
    assert_eq!(
        derivative(&source[1], &mut arithmetic).unwrap(),
        vec![arithmetic.zero()]
    );
}

#[test]
fn both_midpoint_children_share_exact_endpoints() {
    let mut cancelled = || false;
    let mut arithmetic = Arithmetic::new(Default::default(), &mut cancelled);
    let source =
        capture_planar(&[[0., 0.], [1., 2.], [3., 1.], [4., 0.]], &mut arithmetic).unwrap();
    for axis in &source {
        let [left, right] = bisect(axis, &mut arithmetic).unwrap();
        assert_eq!(left.last(), right.first());
        assert_eq!(left.first(), axis.first());
        assert_eq!(right.last(), axis.last());
    }
}
