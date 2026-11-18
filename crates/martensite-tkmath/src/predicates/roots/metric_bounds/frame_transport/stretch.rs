// SPDX-License-Identifier: GPL-2.0-only
//! Symmetric Gram row sums bound every singular value without sampled directions.
use super::*;
use arithmetic::*;
use std::cmp::Ordering;
pub(super) fn qualify(matrix: &Matrix) -> Checked<([f64; 2], [f64; 2], f64)> {
    let determinant = positive_determinant(matrix)?;
    let residual = gram_residual(matrix);
    let upper = bounds(&residual, FrameTransportQuantity::GramResidual)?[1];
    let one = ExactRatio::from_finite(1.);
    if residual >= one {
        return Err(FrameTransportError::UncertifiedStretch {
            gram_residual_upper: upper,
        });
    }
    let lower = square_root(&one.subtract(&residual), FrameTransportQuantity::Stretch)?[0];
    let high = square_root(&one.add(&residual), FrameTransportQuantity::Stretch)?[1];
    if lower <= 0. {
        return Err(FrameTransportError::Unrepresentable(
            FrameTransportQuantity::Stretch,
        ));
    }
    Ok(([lower, high], determinant, upper))
}
fn positive_determinant(matrix: &Matrix) -> Checked<[f64; 2]> {
    let determinant = determinant(matrix);
    let enclosed = bounds(&determinant, FrameTransportQuantity::Determinant)?;
    if determinant.sign() != Ordering::Greater {
        return Err(FrameTransportError::NonPositiveDeterminant { bounds: enclosed });
    }
    Ok(enclosed)
}
fn gram_residual(matrix: &Matrix) -> ExactRatio {
    let columns: Matrix = std::array::from_fn(|i| std::array::from_fn(|j| matrix[j][i].clone()));
    (0..3)
        .map(|i| gram_row(&columns, i))
        .max()
        .unwrap_or_else(|| ExactRatio::from_finite(0.))
}
fn gram_row(columns: &Matrix, i: usize) -> ExactRatio {
    (0..3).fold(ExactRatio::from_finite(0.), |sum, j| {
        let target = ExactRatio::from_finite(if i == j { 1. } else { 0. });
        sum.add(&absolute(&dot(&columns[i], &columns[j]).subtract(&target)))
    })
}
