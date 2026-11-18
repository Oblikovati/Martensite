// SPDX-License-Identifier: GPL-2.0-only
//! Reuse the bounded trigonometric proof for a requested rotation of the exact reference frame.
use super::super::{CylindricalProfileFailure, cylindrical_profile::trigonometric};
use super::*;
use crate::predicates::roots::certificate_arithmetic::{
    RatioInterval,
    bounded_dyadic::{Arithmetic, ArithmeticPolicy},
};
mod spatial;
mod types;
pub use spatial::CertifiedRotatedFrameSpatialBudget;
pub use types::*;
const FRACTIONAL_BITS: u16 = 128;
const OPERATION_LIMIT: u32 = 10_000;
const ARITHMETIC_POLICY: ArithmeticPolicy = ArithmeticPolicy {
    fractional_bits: FRACTIONAL_BITS,
    max_operations: OPERATION_LIMIT,
};
type RotationChecked<T> = Result<T, FrameRotationFailure>;

/// Certify the stored matrix against the ideal normalized frame rotated by `angle` radians.
/// Rotation is about the ideal axis: X' = cos(angle) X + sin(angle) Y,
/// Y' = -sin(angle) X + cos(angle) Y; the ideal Z column is unchanged.
/// The original inputs and bounded trigonometric work are retained in the certificate.
/// Example: `let proof = certify_frame_transport_at_angle(input, 0.25)?;`.
/// Zero angles take the existing unrotated path without trigonometric work.
pub fn certify_frame_transport_at_angle(
    input: FrameTransportInput,
    angle: f64,
) -> Result<CertifiedRotatedFrameTransport, FrameRotationError> {
    execute(input, angle).map_err(|cause| FrameRotationError {
        angle,
        fractional_bits: FRACTIONAL_BITS,
        operation_limit: OPERATION_LIMIT,
        cause,
    })
}
fn execute(
    input: FrameTransportInput,
    angle: f64,
) -> RotationChecked<CertifiedRotatedFrameTransport> {
    if !angle.is_finite() {
        return Err(FrameRotationFailure::InvalidAngle);
    }
    if angle == 0. {
        return unrotated(input, angle);
    }
    rotated(input, angle)
}
fn rotated(
    input: FrameTransportInput,
    angle: f64,
) -> RotationChecked<CertifiedRotatedFrameTransport> {
    admit(input).map_err(FrameRotationFailure::Frame)?;
    let ideal = reference::ideal(input).map_err(FrameRotationFailure::Frame)?;
    let frame = qualify(input, &ideal).map_err(FrameRotationFailure::Frame)?;
    let rotation = trigonometry(angle)
        .map_err(|cause| FrameRotationFailure::Trigonometric(Box::new(cause)))?;
    let rotated = rotate(ideal, &rotation).map_err(FrameRotationFailure::Frame)?;
    let stored = input.stored.map(arithmetic::vector);
    let matrix_error =
        reference::matrix_error(&stored, &rotated).map_err(FrameRotationFailure::Frame)?;
    Ok(CertifiedRotatedFrameTransport {
        frame,
        matrix_error,
        rotation,
    })
}

fn unrotated(
    input: FrameTransportInput,
    angle: f64,
) -> RotationChecked<CertifiedRotatedFrameTransport> {
    let frame = certify_frame_transport(input).map_err(FrameRotationFailure::Frame)?;
    let rotation = trigonometry(angle)
        .map_err(|cause| FrameRotationFailure::Trigonometric(Box::new(cause)))?;
    Ok(CertifiedRotatedFrameTransport {
        matrix_error: frame.matrix_error_bound(),
        frame,
        rotation,
    })
}
fn trigonometry(angle: f64) -> Result<FrameRotationReport, CylindricalProfileFailure> {
    if angle == 0. {
        return Ok(rotation_report(angle, 0, [1., 1.], [0., 0.]));
    }
    let mut cancelled = || false;
    let mut arithmetic = Arithmetic::new(ARITHMETIC_POLICY, &mut cancelled);
    let phase = arithmetic.scalar(angle)?;
    let [cosine, sine] = trigonometric::enclose(&[phase], &mut arithmetic)?;
    let cosine_bounds = endpoints(&arithmetic, &cosine[0])?;
    let sine_bounds = endpoints(&arithmetic, &sine[0])?;
    Ok(rotation_report(
        angle,
        arithmetic.operations(),
        cosine_bounds,
        sine_bounds,
    ))
}
fn rotation_report(
    angle: f64,
    operations: u32,
    cosine_bounds: [f64; 2],
    sine_bounds: [f64; 2],
) -> FrameRotationReport {
    FrameRotationReport {
        angle,
        fractional_bits: FRACTIONAL_BITS,
        operation_limit: OPERATION_LIMIT,
        operations,
        cosine_bounds,
        sine_bounds,
    }
}

fn endpoints(
    arithmetic: &Arithmetic<'_>,
    interval: &crate::predicates::roots::certificate_arithmetic::bounded_dyadic::Interval,
) -> Result<[f64; 2], CylindricalProfileFailure> {
    Ok([
        -arithmetic.upper_f64(-interval.lower.clone())?,
        arithmetic.upper_f64(interval.upper.clone())?,
    ])
}
fn rotate(
    ideal: reference::EnclosedMatrix,
    rotation: &FrameRotationReport,
) -> Checked<reference::EnclosedMatrix> {
    let mut result = ideal;
    let cosine = interval(rotation.cosine_bounds);
    let sine = interval(rotation.sine_bounds);
    let negative_sine = interval([-rotation.sine_bounds[1], -rotation.sine_bounds[0]]);
    for (row, output) in ideal.into_iter().zip(&mut result) {
        output[0] = rotated_component(row, &cosine, &sine)?;
        output[1] = rotated_component(row, &negative_sine, &cosine)?;
    }
    Ok(result)
}
fn rotated_component(
    row: [[f64; 2]; 3],
    x: &RatioInterval,
    y: &RatioInterval,
) -> Checked<[f64; 2]> {
    let result = interval(row[0])
        .multiply(x)
        .add(&interval(row[1]).multiply(y));
    Ok([
        arithmetic::bounds(&result.lower, FrameTransportQuantity::MatrixError)?[0],
        arithmetic::bounds(&result.upper, FrameTransportQuantity::MatrixError)?[1],
    ])
}
fn interval([lower, upper]: [f64; 2]) -> RatioInterval {
    RatioInterval {
        lower: ExactRatio::from_finite(lower),
        upper: ExactRatio::from_finite(upper),
    }
}
#[cfg(test)]
mod tests;
