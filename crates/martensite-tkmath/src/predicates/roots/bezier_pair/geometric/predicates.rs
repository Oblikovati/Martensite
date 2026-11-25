// SPDX-License-Identifier: GPL-2.0-only
//! Both differences are formed in the existing exact integer coordinate representation.
use super::*;
use crate::predicates::{exact_determinant, exact_point, orient2d};
pub(super) fn cross(a: &Vector, b: &Vector) -> Checked<OrientationSign> {
    if a.from == b.from {
        return orient2d(a.from, a.to, b.to).map_err(coordinate);
    }
    let x = difference(a)?;
    let y = difference(b)?;
    Ok(match exact_determinant(&x, &y).sign() {
        num_bigint::Sign::Minus => OrientationSign::Negative,
        num_bigint::Sign::NoSign => OrientationSign::Zero,
        num_bigint::Sign::Plus => OrientationSign::Positive,
    })
}
fn difference(v: &Vector) -> Checked<[num_bigint::BigInt; 2]> {
    let a = exact_point(v.from, 0).map_err(coordinate)?;
    let b = exact_point(v.to, 1).map_err(coordinate)?;
    Ok([&b[0] - &a[0], &b[1] - &a[1]])
}
pub(super) fn same_direction(a: &Vector, b: &Vector) -> bool {
    std::array::from_fn::<_, 2, _>(|i| a.to[i].partial_cmp(&a.from[i]))
        == std::array::from_fn::<_, 2, _>(|i| b.to[i].partial_cmp(&b.from[i]))
}
fn coordinate(cause: NonFiniteCoordinate) -> BezierPairGeometricFailure {
    BezierPairGeometricFailure::Source {
        source_index: 0,
        cause: BezierPairCaptureFailure::Coordinate(cause),
    }
}
