// SPDX-License-Identifier: GPL-2.0-only
use crate::{CurveRole, EditFailure, LineId, PointId, line_edit::EndpointProtection};
use martensite_tkmath::predicates::roots::metric_bounds::CardinalBendFailure;
use std::{error::Error, fmt};

/// A model quantity checked under its own Bend limit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BendQuantity {
    /// Support point placement deviation, in model units.
    Placement,
    /// Difference between actual and requested radius, in model units.
    Radius,
    /// Analytic terminal tangent error, in radians.
    TangentAngle,
    /// Distance of the nominal analytic arc end from the terminal vertex, in model units.
    NominalTerminal,
}

/// Actual stored geometry that could not be associated with its source evidence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BendCheck {
    /// The replacement straight segment must retain its authored source trace and orientation.
    LineTrace(LineId),
    /// The stored arc, authored points and center must agree with the cut points.
    ArcPlacement,
    /// The stored radial frame must have the literal signed cardinal axes.
    ArcFrame,
    /// The stored arc radius must agree with the common exact radius.
    ArcRadius,
    /// The stored positive trim must be the binary64 quarter turn.
    ArcTrim,
}

/// A refused or cancelled Bend leaves the source snapshot readable.
#[derive(Debug)]
#[non_exhaustive]
pub enum BendFailure {
    /// The same line was requested twice.
    RepeatedLine(LineId),
    /// The requested line does not exist.
    MissingLine(LineId),
    /// The requested corner does not exist.
    MissingCorner(PointId),
    /// A selected line does not reference the corner identity.
    CornerIncidence {
        /// Requested corner identity.
        corner: PointId,
        /// Selected line that does not reference it.
        line: LineId,
    },
    /// An independent frozen or shared incidence protects the original corner.
    ProtectedCorner {
        /// Protected corner identity.
        corner: PointId,
        /// Binding that prevents replacement of the corner.
        protection: EndpointProtection,
    },
    /// The selected lines have different modelling participation.
    MixedParticipation([CurveRole; 2]),
    /// A quality limit is outside its admitted domain.
    InvalidLimit {
        /// Measured quantity the limit addresses.
        quantity: BendQuantity,
        /// Refused caller limit.
        value: f64,
    },
    /// An independently measured quantity exceeds the requested limit.
    Bound {
        /// Measured quantity whose limit was exceeded.
        quantity: BendQuantity,
        /// Certified outward bound.
        achieved: f64,
        /// Original requested limit in the same units.
        limit: f64,
    },
    /// The source legs admit no representable cardinal placement under this request.
    Placement(CardinalBendFailure),
    /// The stored geometry did not match the certified construction.
    StoredGeometry(BendCheck),
    /// Ordinary atomic sketch admission refused the draft.
    Geometry(EditFailure),
    /// The caller cancelled the draft.
    Cancelled,
}

impl fmt::Display for BendFailure {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Placement(cause) => cause.fmt(output),
            Self::Geometry(cause) => cause.fmt(output),
            Self::InvalidLimit { quantity, value } => write!(
                output,
                "bend {quantity:?} limit {value} must be finite and nonnegative"
            ),
            Self::Bound {
                quantity,
                achieved,
                limit,
            } => write!(
                output,
                "bend {quantity:?} bound {achieved} exceeds limit {limit}"
            ),
            Self::Cancelled => write!(output, "bend cancelled"),
            other => write!(
                output,
                "bend refused for the selected exclusive corner: {other:?}"
            ),
        }
    }
}

impl Error for BendFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Placement(cause) => Some(cause),
            Self::Geometry(cause) => Some(cause),
            _ => None,
        }
    }
}

impl From<CardinalBendFailure> for BendFailure {
    fn from(cause: CardinalBendFailure) -> Self {
        Self::Placement(cause)
    }
}

impl From<EditFailure> for BendFailure {
    fn from(cause: EditFailure) -> Self {
        match cause {
            EditFailure::Cancelled(_) => Self::Cancelled,
            other => Self::Geometry(other),
        }
    }
}
