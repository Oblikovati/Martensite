// SPDX-License-Identifier: GPL-2.0-only
//! A missing bound is a refusal, never a zero-error certificate.
/// Input component that failed finite-value admission.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrameTransportField {
    /// Requested axis vector.
    Axis,
    /// Requested radial vector.
    Radial,
    /// Actual stored matrix.
    StoredMatrix,
}
/// Quantity whose finite outward enclosure was unavailable.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrameTransportQuantity {
    /// Stored-to-reference operator error.
    MatrixError,
    /// Positive singular-value enclosure.
    Stretch,
    /// Stored determinant enclosure.
    Determinant,
    /// Absolute Gram-residual row sum.
    GramResidual,
}
/// Typed refusal for reference construction or stored-matrix qualification.
#[derive(Clone, Debug, PartialEq)]
pub enum FrameTransportError {
    /// Exact offending coordinate and its row/column location.
    NonFinite {
        /// Rejected input field.
        field: FrameTransportField,
        /// Zero-based row and column; vectors use row zero.
        index: [usize; 2],
        /// Original nonfinite value.
        value: f64,
    },
    /// The reference axis cannot define a unit direction.
    ZeroAxis {
        /// Original zero vector.
        axis: [f64; 3],
    },
    /// The radial vector cannot define a perpendicular direction.
    ParallelRadial {
        /// Original axis.
        axis: [f64; 3],
        /// Original parallel or zero radial vector.
        radial: [f64; 3],
    },
    /// No finite binary64 enclosure was obtained for the named quantity.
    Unrepresentable(FrameTransportQuantity),
    /// A right-handed nonsingular stored map was not established.
    NonPositiveDeterminant {
        /// Outward enclosure of the rejected determinant.
        bounds: [f64; 2],
    },
    /// The Gram-residual bound cannot prove positive lower stretch.
    UncertifiedStretch {
        /// Computed upper bound, which did not establish positive stretch.
        gram_residual_upper: f64,
    },
}
impl std::fmt::Display for FrameTransportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "frame transport {self:?}; expected finite nonparallel reference vectors, positive determinant and certified positive stretch"
        )
    }
}
impl std::error::Error for FrameTransportError {}
