// SPDX-License-Identifier: GPL-2.0-only
//! Absolute error propagation includes rounded intermediate magnitudes and gradual underflow.
use super::*;
struct Evaluated {
    magnitude: ExactRatio,
    error: ExactRatio,
}
pub(super) fn bound(matrix: [[f64; 3]; 3], input: FrameSpatialInput) -> Checked<f64> {
    let mut error = ExactRatio::from_finite(0.);
    for (row, translation) in matrix.into_iter().zip(input.translation) {
        let [a, b, c] = row.map(|coefficient| product(coefficient, input.point_norm));
        let sum = add(add(a?, b?)?, c?)?;
        let value = add(sum, exact(translation.abs()))?;
        error = error.add(&value.error);
    }
    // L1 bounds Euclidean error without assuming independent rounding directions.
    upper(&error)
}
fn exact(magnitude: f64) -> Evaluated {
    Evaluated {
        magnitude: ExactRatio::from_finite(magnitude),
        error: ExactRatio::from_finite(0.),
    }
}
fn product(coefficient: f64, point_norm: f64) -> Checked<Evaluated> {
    let magnitude =
        ExactRatio::from_finite(coefficient.abs()).multiply(&ExactRatio::from_finite(point_norm));
    if coefficient == 0. || coefficient.abs() == 1. || point_norm == 0. {
        return Ok(Evaluated {
            magnitude,
            error: ExactRatio::from_finite(0.),
        });
    }
    rounded(magnitude, ExactRatio::from_finite(0.))
}
fn add(a: Evaluated, b: Evaluated) -> Checked<Evaluated> {
    if a.magnitude == ExactRatio::from_finite(0.) {
        return Ok(b);
    }
    if b.magnitude == ExactRatio::from_finite(0.) {
        return Ok(a);
    }
    rounded(a.magnitude.add(&b.magnitude), a.error.add(&b.error))
}
fn rounded(magnitude: ExactRatio, prior_error: ExactRatio) -> Checked<Evaluated> {
    let upper = arithmetic::bounds(&magnitude, FrameTransportQuantity::Evaluation)?[1];
    let successor = upper.next_up();
    if !successor.is_finite() {
        return Err(FrameTransportError::Unrepresentable(
            FrameTransportQuantity::Evaluation,
        ));
    }
    // A full upward ulp bounds nearest rounding at every magnitude <= upper, including subnormals.
    let rounding = ExactRatio::from_finite(successor).subtract(&ExactRatio::from_finite(upper));
    Ok(Evaluated {
        magnitude: magnitude.add(&rounding),
        error: prior_error.add(&rounding),
    })
}
