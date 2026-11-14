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
    /// Requested world-space origin.
    Origin,
    /// Actual stored translation.
    Translation,
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
    /// Point-domain placement error or its canonical allocation.
    SpatialBudget,
    /// Rounded matrix/translation evaluation on the supplied point domain.
    Evaluation,
}
/// Typed refusal for reference construction or stored-matrix qualification.
#[derive(Clone, Debug, PartialEq)]
pub enum FrameTransportError {
    /// A finite scalar did not satisfy the positive-domain contract.
    SpatialInput {
        /// Caller-supplied bound on the norm of ideal and retained points.
        point_norm: f64,
        /// Caller-requested strictly positive spatial limit.
        spatial_limit: f64,
    },
    /// Placement alone consumed the available spatial limit.
    SpatialLimit {
        /// Requested original limit.
        requested: f64,
        /// Certified placement-only upper bound.
        placement_upper: f64,
    },
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
        let expected = match self {
            Self::SpatialInput { .. } => "finite point_norm >= 0 and finite spatial_limit > 0",
            Self::SpatialLimit { .. } => {
                "placement_upper < requested and a positive representable canonical allowance"
            }
            Self::Unrepresentable(FrameTransportQuantity::Evaluation) => {
                "finite outward bounds for every affine evaluation intermediate"
            }
            Self::Unrepresentable(FrameTransportQuantity::SpatialBudget) => {
                "finite outward placement and canonical allocation bounds"
            }
            _ => {
                "finite nonparallel reference vectors, positive determinant and certified positive stretch"
            }
        };
        write!(f, "frame transport {self:?}; expected {expected}")
    }
}
impl std::error::Error for FrameTransportError {}
