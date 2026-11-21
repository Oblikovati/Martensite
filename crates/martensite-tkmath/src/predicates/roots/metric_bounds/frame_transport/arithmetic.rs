// SPDX-License-Identifier: GPL-2.0-only
//! Fixed-size exact arithmetic postpones rounding until a certified endpoint.
use super::*;
use std::cmp::Ordering;
pub(super) type Vector = [ExactRatio; 3];
pub(super) type Matrix = [Vector; 3];
pub(super) fn vector(values: [f64; 3]) -> Vector {
    values.map(ExactRatio::from_finite)
}
pub(super) fn dot(a: &Vector, b: &Vector) -> ExactRatio {
    a.iter()
        .zip(b)
        .fold(ExactRatio::from_finite(0.), |sum, (a, b)| {
            sum.add(&a.multiply(b))
        })
}
pub(super) fn cross(a: &Vector, b: &Vector) -> Vector {
    std::array::from_fn(|i| {
        let (j, k) = ((i + 1) % 3, (i + 2) % 3);
        a[j].multiply(&b[k]).subtract(&a[k].multiply(&b[j]))
    })
}
pub(super) fn absolute(value: &ExactRatio) -> ExactRatio {
    if value.sign() == Ordering::Less {
        return value.negated();
    }
    value.clone()
}
pub(super) fn bounds(value: &ExactRatio, quantity: FrameTransportQuantity) -> Checked<[f64; 2]> {
    outward_binary64(value).ok_or(FrameTransportError::Unrepresentable(quantity))
}
pub(super) fn square_root(
    value: &ExactRatio,
    quantity: FrameTransportQuantity,
) -> Checked<[f64; 2]> {
    outward_sqrt_bounds(value).ok_or(FrameTransportError::Unrepresentable(quantity))
}
pub(super) fn determinant(matrix: &Matrix) -> ExactRatio {
    dot(&matrix[0], &cross(&matrix[1], &matrix[2]))
}
