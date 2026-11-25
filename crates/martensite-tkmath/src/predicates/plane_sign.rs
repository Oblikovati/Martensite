// SPDX-License-Identifier: GPL-2.0-only
//! Exact affine plane signs of finite binary64 model coordinates.
use super::{NonFiniteCoordinate, exact_vector3, triple_product::exact_triple_product};
use num_bigint::{BigInt, Sign};
use std::cmp::Ordering;

/// Compare the signed tetrahedron determinant of `a,b,c,d` with zero.
/// Anchor differences are exact. A zero result includes degenerate first triangles.
/// ```
/// use martensite_tkmath::predicates::compare_tetrahedron_orientation_3d;
/// use std::cmp::Ordering;
/// let xy = [[0.,0.,0.], [1.,0.,0.], [0.,1.,0.], [0.,0.,1.]];
/// assert_eq!(compare_tetrahedron_orientation_3d(xy), Ok(Ordering::Greater));
/// ```
pub fn compare_tetrahedron_orientation_3d(
    [a, b, c, d]: [[f64; 3]; 4],
) -> Result<Ordering, NonFiniteCoordinate> {
    let a = exact_vector3(a, 0)?;
    let b = difference(exact_vector3(b, 1)?, &a);
    let c = difference(exact_vector3(c, 2)?, &a);
    let d = difference(exact_vector3(d, 3)?, &a);
    Ok(determinant_sign(&b, &c, &d))
}

/// Compare the oriented plane normal of `a,b,c` dotted with a free vector.
/// The anchors and the direction are evaluated as supplied, without rounded differences.
/// ```
/// use martensite_tkmath::predicates::compare_plane_direction_3d;
/// use std::cmp::Ordering;
/// let xy = [[0.,0.,1.], [1.,0.,1.], [0.,1.,1.]];
/// assert_eq!(compare_plane_direction_3d(xy, [0.,0.,-2.]), Ok(Ordering::Less));
/// ```
pub fn compare_plane_direction_3d(
    [a, b, c]: [[f64; 3]; 3],
    direction: [f64; 3],
) -> Result<Ordering, NonFiniteCoordinate> {
    let a = exact_vector3(a, 0)?;
    let b = difference(exact_vector3(b, 1)?, &a);
    let c = difference(exact_vector3(c, 2)?, &a);
    let d = exact_vector3(direction, 3)?;
    Ok(determinant_sign(&b, &c, &d))
}

fn determinant_sign(a: &[BigInt; 3], b: &[BigInt; 3], c: &[BigInt; 3]) -> Ordering {
    match exact_triple_product(a, b, c).sign() {
        Sign::Minus => Ordering::Less,
        Sign::NoSign => Ordering::Equal,
        Sign::Plus => Ordering::Greater,
    }
}

fn difference(mut p: [BigInt; 3], origin: &[BigInt; 3]) -> [BigInt; 3] {
    for (coordinate, base) in p.iter_mut().zip(origin) {
        *coordinate -= base;
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn untranslated_and_oblique_planes_have_exact_signed_witnesses() {
        let a = [2., -3., 5.];
        let b = [5., -3., 5.];
        let c = [5., 1., 8.];
        let d = [2., 1., 8.];
        assert_eq!(
            compare_tetrahedron_orientation_3d([a, b, c, d]),
            Ok(Ordering::Equal)
        );
        let above = [d[0], d[1], d[2].next_up()];
        assert_eq!(
            compare_tetrahedron_orientation_3d([a, b, c, above]),
            Ok(Ordering::Greater)
        );
        assert_eq!(
            compare_tetrahedron_orientation_3d([a, c, b, above]),
            Ok(Ordering::Less)
        );
        assert_eq!(
            compare_plane_direction_3d([a, b, c], [0., 4., 3.]),
            Ok(Ordering::Equal)
        );
        assert_eq!(
            compare_plane_direction_3d([a, b, c], [0., -3., 4.]),
            Ok(Ordering::Greater)
        );
    }

    #[test]
    fn signed_plane_differences_do_not_round_at_large_translations() {
        let p = 4_503_599_627_370_496.0;
        let a = [p, p, p];
        let b = [p + 2., p, p + 2.];
        let c = [p, p + 2., p + 2.];
        assert_eq!(
            compare_tetrahedron_orientation_3d([a, b, c, [p + 1., p + 1., p + 2.]]),
            Ok(Ordering::Equal)
        );
        assert_eq!(
            compare_tetrahedron_orientation_3d([a, b, c, [p + 1., p + 1., p + 3.]]),
            Ok(Ordering::Greater)
        );
    }
}
