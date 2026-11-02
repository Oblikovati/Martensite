use super::BilinearInconclusive;
use crate::predicates::NonFiniteCoordinate;
use std::fmt;

/// A rejected numeric patch input, distinct from an inconclusive certificate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BilinearInputError {
    /// A corner coordinate is not finite.
    NonFiniteCorner(NonFiniteCoordinate),
    /// An interval is not finite and strictly increasing.
    InvalidDomain {
        /// Zero-based parameter direction, U then V.
        direction: usize,
        /// Supplied lower and upper endpoints.
        interval: [f64; 2],
    },
    /// A weight is not positive and finite.
    InvalidWeight {
        /// Zero-based corner in boundary order.
        corner: usize,
        /// Supplied weight, including any nonfinite payload.
        value: f64,
    },
}
impl fmt::Display for BilinearInputError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteCorner(error) => write!(out, "{error}"),
            Self::InvalidDomain {
                direction,
                interval,
            } => write!(
                out,
                "parameter direction {direction} interval {interval:?}; expected finite increasing endpoints"
            ),
            Self::InvalidWeight { corner, value } => write!(
                out,
                "corner {corner} weight {value}; expected a positive finite weight"
            ),
        }
    }
}
impl std::error::Error for BilinearInputError {}

/// Corners in boundary order (00,10,11,01), parameter intervals and rational weights.
#[derive(Clone, Debug, PartialEq)]
pub struct BilinearPatchInput {
    corners: [[f64; 3]; 4],
    domain: [[f64; 2]; 2],
    weights: [f64; 4],
}
impl BilinearPatchInput {
    /// Validate and own a patch; `BilinearPatchInput::new(corners, [[0.,1.];2], [1.;4])?`.
    pub fn new(
        corners: [[f64; 3]; 4],
        domain: [[f64; 2]; 2],
        weights: [f64; 4],
    ) -> Result<Self, BilinearInputError> {
        validate_corners(&corners)?;
        validate_domain(&domain)?;
        validate_weights(&weights)?;
        Ok(Self {
            corners,
            domain,
            weights,
        })
    }
    /// Copy the bound corners; `input.corners()`.
    pub fn corners(&self) -> [[f64; 3]; 4] {
        self.corners
    }
    /// Copy the bound parameter intervals; `input.domain()`.
    pub fn domain(&self) -> [[f64; 2]; 2] {
        self.domain
    }
    /// Copy the bound weights; `input.weights()`.
    pub fn weights(&self) -> [f64; 4] {
        self.weights
    }
    /// Compare representation bits, not entity identity; `old.same_representation(&fresh)`.
    pub fn same_representation(&self, other: &Self) -> bool {
        self.numeric_values()
            .zip(other.numeric_values())
            .all(|(a, b)| a.to_bits() == b.to_bits())
    }
    fn numeric_values(&self) -> impl Iterator<Item = f64> + '_ {
        self.corners
            .iter()
            .flatten()
            .chain(self.domain.iter().flatten())
            .chain(self.weights.iter())
            .copied()
    }
    pub(super) fn restriction(&self) -> Option<BilinearInconclusive> {
        if self.weights.iter().any(|weight| *weight != self.weights[0]) {
            return Some(BilinearInconclusive::NonConstantWeights);
        }
        self.domain
            .iter()
            .enumerate()
            .find_map(|(direction, [first, last])| {
                let span = last - first;
                (!span.is_finite() || !span.recip().is_finite())
                    .then_some(BilinearInconclusive::UnrepresentableSpan { direction })
            })
    }
}
fn validate_corners(corners: &[[f64; 3]; 4]) -> Result<(), BilinearInputError> {
    for (point, corner) in corners.iter().enumerate() {
        validate_corner(point, corner)?;
    }
    Ok(())
}
fn validate_corner(point: usize, corner: &[f64; 3]) -> Result<(), BilinearInputError> {
    for (axis, &value) in corner.iter().enumerate() {
        if !value.is_finite() {
            return Err(BilinearInputError::NonFiniteCorner(NonFiniteCoordinate {
                point,
                axis,
                value,
            }));
        }
    }
    Ok(())
}

fn validate_domain(domain: &[[f64; 2]; 2]) -> Result<(), BilinearInputError> {
    for (direction, &interval) in domain.iter().enumerate() {
        if interval.iter().any(|v| !v.is_finite()) || interval[0] >= interval[1] {
            return Err(BilinearInputError::InvalidDomain {
                direction,
                interval,
            });
        }
    }
    Ok(())
}
fn validate_weights(weights: &[f64; 4]) -> Result<(), BilinearInputError> {
    for (corner, &value) in weights.iter().enumerate() {
        if !value.is_finite() || value <= 0. {
            return Err(BilinearInputError::InvalidWeight { corner, value });
        }
    }
    Ok(())
}
