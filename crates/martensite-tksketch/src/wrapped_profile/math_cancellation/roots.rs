//! Follow typed roots failures without consulting a later cancellation signal.
use super::*;
impl MathCancellation for BoundedArithmeticFailure {
    fn is_cancelled(&self) -> bool {
        match self {
            Self::Cancelled => true,
            Self::NonFiniteScalar { .. }
            | Self::WorkLimit { .. }
            | Self::ArithmeticBits { .. }
            | Self::CoefficientCount { .. }
            | Self::DenominatorUnresolved { .. }
            | Self::UnrepresentableBound { .. } => false,
        }
    }
}

impl MathCancellation for RootIsolationError {
    fn is_cancelled(&self) -> bool {
        match self {
            Self::Cancelled => true,
            Self::OptionsVersion(..)
            | Self::Coefficient { .. }
            | Self::Interval(..)
            | Self::Width(..)
            | Self::ZeroPolynomial { .. }
            | Self::Degree { .. }
            | Self::SubdivisionLimit { .. }
            | Self::Variation { .. } => false,
        }
    }
}

impl MathCancellation for RootSignError {
    fn is_cancelled(&self) -> bool {
        match self {
            Self::Root { cause, .. } => cause.is_cancelled(),
            Self::Variation { .. } => false,
        }
    }
}

impl MathCancellation for BezierLineError {
    fn is_cancelled(&self) -> bool {
        match self {
            Self::Root { cause, .. } => cause.is_cancelled(),
            Self::EmptyControls
            | Self::WeightCount { .. }
            | Self::Coordinate { .. }
            | Self::Weight { .. }
            | Self::DegenerateLine(..)
            | Self::ZeroDenominator
            | Self::DenominatorRoots(..) => false,
        }
    }
}

impl MathCancellation for BezierRayError {
    fn is_cancelled(&self) -> bool {
        match self {
            Self::Line(cause) => cause.is_cancelled(),
            Self::Direction(cause) => cause.is_cancelled(),
            Self::Endpoint(cause) => cause.is_cancelled(),
            Self::Signs(cause) => cause.is_cancelled(),
            Self::SupportingLineCoincidence | Self::LocalNormalSign(..) => false,
        }
    }
}

impl MathCancellation for BezierWireError {
    fn is_cancelled(&self) -> bool {
        match self {
            Self::Segment { cause, .. } => cause.is_cancelled(),
            Self::Cancelled { .. } => true,
            Self::Empty
            | Self::DuplicateId { .. }
            | Self::OpenJoin { .. }
            | Self::WindingOverflow { .. } => false,
        }
    }
}

impl MathCancellation for ResidualError {
    fn is_cancelled(&self) -> bool {
        match self {
            Self::Roots(cause) => cause.is_cancelled(),
            Self::Signs(cause) => cause.is_cancelled(),
            Self::Shape { .. }
            | Self::Value { .. }
            | Self::Domain(..)
            | Self::ExactDomain { .. }
            | Self::ParameterBracket { .. }
            | Self::ParameterDomain { .. } => false,
        }
    }
}
