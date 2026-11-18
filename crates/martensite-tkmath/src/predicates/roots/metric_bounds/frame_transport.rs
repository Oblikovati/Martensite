// SPDX-License-Identifier: GPL-2.0-only
//! Exact stored-matrix transport relative to a mathematically normalized reference frame.
mod arithmetic;
mod error;
mod reference;
mod rotation;
pub use rotation::*;
mod spatial;
mod stretch;
mod types;
use super::super::certificate_arithmetic::{
    ExactRatio,
    rounding::{outward_binary64, outward_sqrt_bounds},
};
pub use error::*;
pub use spatial::{CertifiedFrameSpatialBudget, FrameSpatialInput};
pub use types::*;
type Checked<T> = Result<T, FrameTransportError>;

/// Bound stored-frame error and stretch against exact normalization/projection of the reference.
/// This does not certify a native representation, translation or point-domain extent.
/// Example: `let proof = certify_frame_transport(input)?; assert!(proof.stretch_bounds()[0] > 0.);`.
pub fn certify_frame_transport(input: FrameTransportInput) -> Checked<CertifiedFrameTransport> {
    admit(input)?;
    let ideal = reference::ideal(input)?;
    qualify(input, &ideal)
}
fn qualify(
    input: FrameTransportInput,
    ideal: &reference::EnclosedMatrix,
) -> Checked<CertifiedFrameTransport> {
    let stored = input.stored.map(arithmetic::vector);
    let (stretch, determinant, gram_residual) = stretch::qualify(&stored)?;
    let matrix_error = reference::matrix_error(&stored, ideal)?;
    Ok(CertifiedFrameTransport {
        input,
        matrix_error,
        stretch,
        determinant,
        gram_residual,
    })
}
fn admit(input: FrameTransportInput) -> Checked<()> {
    admit_vector(input.axis, FrameTransportField::Axis, 0)?;
    admit_vector(input.radial, FrameTransportField::Radial, 0)?;
    for (row, coefficients) in input.stored.into_iter().enumerate() {
        admit_vector(coefficients, FrameTransportField::StoredMatrix, row)?;
    }
    Ok(())
}
fn admit_vector(values: [f64; 3], field: FrameTransportField, row: usize) -> Checked<()> {
    for (column, value) in values.into_iter().enumerate() {
        if !value.is_finite() {
            return Err(FrameTransportError::NonFinite {
                field,
                index: [row, column],
                value,
            });
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests;
