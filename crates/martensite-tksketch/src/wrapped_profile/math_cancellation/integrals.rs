//! Follow typed integrals failures without consulting a later cancellation signal.
use super::*;
impl MathCancellation for PolynomialIntegralFailure {
    fn is_cancelled(&self) -> bool {
        match self {
            Self::Metric(cause) => cause.is_cancelled(),
            Self::Policy(..)
            | Self::EmptyBoundary
            | Self::CurveCount { .. }
            | Self::Degree { .. }
            | Self::RationalWeights { .. }
            | Self::Unrepresentable { .. }
            | Self::ErrorLimit { .. } => false,
        }
    }
}

impl MathCancellation for RationalIntegralFailure {
    fn is_cancelled(&self) -> bool {
        match self {
            Self::Metric(cause) => cause.is_cancelled(),
            Self::Policy(..)
            | Self::EmptyBoundary
            | Self::CurveCount { .. }
            | Self::Degree { .. }
            | Self::DenominatorSign { .. }
            | Self::WorkLimit { .. }
            | Self::IndeterminateOrientation { .. }
            | Self::Unrepresentable { .. }
            | Self::ErrorLimit { .. } => false,
        }
    }
}

impl MathCancellation for CylindricalFluxFailure {
    fn is_cancelled(&self) -> bool {
        match self {
            Self::Geometry { cause, .. } => cause.is_cancelled(),
            Self::Cap { cause, .. } => cause.is_cancelled(),
            Self::Integration { cause, .. } => cause.is_cancelled(),
            Self::Policy(..)
            | Self::SpanCount { .. }
            | Self::Radii(..)
            | Self::PairCounts { .. }
            | Self::PairWeights { .. } => false,
        }
    }
}

impl MathCancellation for PolynomialIntegralError {
    fn is_cancelled(&self) -> bool {
        self.cause.is_cancelled()
    }
}

impl MathCancellation for CylindricalFluxError {
    fn is_cancelled(&self) -> bool {
        self.cause.is_cancelled()
    }
}

impl MathCancellation for CylindricalMeasureError {
    fn is_cancelled(&self) -> bool {
        self.cause.is_cancelled()
    }
}
impl MathCancellation for CylindricalMeasureFailure {
    fn is_cancelled(&self) -> bool {
        match self {
            Self::Arithmetic(cause) => cause.is_cancelled(),
            Self::Policy
            | Self::SourceCount
            | Self::Source { .. }
            | Self::Depths
            | Self::Radius { .. }
            | Self::Uncertified { .. } => false,
        }
    }
}
