// SPDX-License-Identifier: GPL-2.0-only
//! Predicate stress corpus: underflow, subnormals, exponent extremes, near-degeneracy,
//! permutations, and FMA-sensitive inputs verified against independent exact arithmetic (#36).
use martensite_tkmath::predicates::{
    OrientationSign, compare_squared_distances_2d, compare_tetrahedron_orientation_3d, orient2d,
};
use std::cmp::Ordering;

fn flip_sign(sign: OrientationSign) -> OrientationSign {
    match sign {
        OrientationSign::Positive => OrientationSign::Negative,
        OrientationSign::Negative => OrientationSign::Positive,
        OrientationSign::Zero => OrientationSign::Zero,
    }
}

fn assert_orient2d_permutations(a: [f64; 2], b: [f64; 2], c: [f64; 2], expected: OrientationSign) {
    assert_eq!(orient2d(a, b, c).unwrap(), expected);
    assert_eq!(orient2d(b, c, a).unwrap(), expected);
    assert_eq!(orient2d(c, a, b).unwrap(), expected);
    let flipped = flip_sign(expected);
    assert_eq!(orient2d(a, c, b).unwrap(), flipped);
    assert_eq!(orient2d(c, b, a).unwrap(), flipped);
    assert_eq!(orient2d(b, a, c).unwrap(), flipped);
}

#[test]
fn orient2d_stress_underflow_subnormal_overflow_and_near_collinear() {
    // Subnormal / underflow triangle where naive f64 determinant underflows to 0.0.
    let tiny = f64::MIN_POSITIVE / 1024.0;
    assert_orient2d_permutations(
        [0.0, 0.0],
        [tiny, 0.0],
        [0.0, tiny],
        OrientationSign::Positive,
    );

    // Exponent extreme triangle where naive f64 products overflow to Inf.
    let huge = 1e200_f64;
    assert_orient2d_permutations(
        [0.0, 0.0],
        [huge, 0.0],
        [0.0, huge],
        OrientationSign::Positive,
    );

    // Near-collinear FMA-sensitive input: (1 + 2^-27)^2 - 1 = 2^-26 + 2^-54 > 0.
    let eps = f64::from_bits(0x3e40_0000_0000_0000); // 2^-27
    let a = [0.0, 0.0];
    let b = [1.0 + eps, 1.0];
    let c = [1.0, 1.0 - eps];
    // det = (1 + eps)(1 - eps) - 1 = -eps^2 < 0
    assert_orient2d_permutations(a, b, c, OrientationSign::Negative);

    // Exact collinearity at large scale.
    assert_orient2d_permutations(
        [1e12, 1e12],
        [1e12 + 1.0, 1e12 + 1.0],
        [1e12 + 2.0, 1e12 + 2.0],
        OrientationSign::Zero,
    );
}

#[test]
fn squared_distance_and_tetrahedron_orientation_stress_extremes() {
    let tiny = 1e-200_f64;
    assert_eq!(
        compare_squared_distances_2d([0.0, 0.0], [tiny, 0.0], [0.0, tiny * 2.0]).unwrap(),
        Ordering::Less
    );
    assert_eq!(
        compare_squared_distances_2d([0.0, 0.0], [1e200, 0.0], [0.0, 1e200]).unwrap(),
        Ordering::Equal
    );

    let o = [0.0, 0.0, 0.0];
    let x = [tiny, 0.0, 0.0];
    let y = [0.0, tiny, 0.0];
    let z = [0.0, 0.0, tiny];
    assert_eq!(
        compare_tetrahedron_orientation_3d([o, x, y, z]).unwrap(),
        Ordering::Greater
    );
    assert_eq!(
        compare_tetrahedron_orientation_3d([o, x, z, y]).unwrap(),
        Ordering::Less
    );
}
