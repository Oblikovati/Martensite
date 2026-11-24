// SPDX-License-Identifier: GPL-2.0-only
//! Normalize each ideal component through its squared ratio, avoiding denominator overflow.
use super::*;
use arithmetic::*;
use std::cmp::Ordering;
pub(super) type EnclosedMatrix = [[[f64; 2]; 3]; 3];
pub(super) fn ideal(input: FrameTransportInput) -> Checked<EnclosedMatrix> {
    let axis = vector(input.axis);
    let tangent = cross(&axis, &vector(input.radial));
    let a2 = dot(&axis, &axis);
    let k2 = dot(&tangent, &tangent);
    require_frame(input, &a2, &k2)?;
    let columns = [
        unit(&cross(&tangent, &axis), &a2.multiply(&k2))?,
        unit(&tangent, &k2)?,
        unit(&axis, &a2)?,
    ];
    Ok(std::array::from_fn(|row| {
        std::array::from_fn(|column| columns[column][row])
    }))
}
fn require_frame(input: FrameTransportInput, a2: &ExactRatio, k2: &ExactRatio) -> Checked<()> {
    if a2.sign() == Ordering::Equal {
        return Err(FrameTransportError::ZeroAxis { axis: input.axis });
    }
    if k2.sign() == Ordering::Equal {
        return Err(FrameTransportError::ParallelRadial {
            axis: input.axis,
            radial: input.radial,
        });
    }
    Ok(())
}
fn unit(numerator: &Vector, squared_denominator: &ExactRatio) -> Checked<[[f64; 2]; 3]> {
    Ok([
        component(&numerator[0], squared_denominator)?,
        component(&numerator[1], squared_denominator)?,
        component(&numerator[2], squared_denominator)?,
    ])
}
fn component(numerator: &ExactRatio, squared_denominator: &ExactRatio) -> Checked<[f64; 2]> {
    let squared = numerator.multiply(numerator).divide(squared_denominator);
    let [lower, upper] = square_root(&squared, FrameTransportQuantity::MatrixError)?;
    if numerator.sign() == Ordering::Less {
        return Ok([-upper, -lower]);
    }
    Ok([lower, upper])
}
pub(super) fn matrix_error(stored: &Matrix, ideal: &EnclosedMatrix) -> Checked<f64> {
    let mut squared = ExactRatio::from_finite(0.);
    for (row, intervals) in stored.iter().zip(ideal) {
        for (value, interval) in row.iter().zip(intervals) {
            let error = endpoint_error(value, *interval);
            squared = squared.add(&error.multiply(&error));
        }
    }
    Ok(square_root(&squared, FrameTransportQuantity::MatrixError)?[1])
}
fn endpoint_error(value: &ExactRatio, interval: [f64; 2]) -> ExactRatio {
    let lower = absolute(&value.subtract(&ExactRatio::from_finite(interval[0])));
    let upper = absolute(&value.subtract(&ExactRatio::from_finite(interval[1])));
    lower.max(upper)
}
