//! Follow typed profiles failures without consulting a later cancellation signal.
use super::*;
impl MathCancellation for CylindricalClearanceFailure {
    fn is_cancelled(&self) -> bool {
        match self {
            Self::PeriodicImage { cause, .. } => cause.is_cancelled(),
            Self::Arithmetic(cause) => cause.is_cancelled(),
            Self::Policy(..)
            | Self::InputCount { .. }
            | Self::Value { .. }
            | Self::PeriodicImagePolicy { .. }
            | Self::PeriodicImageLimit { .. }
            | Self::PeriodicSlabUnresolved { .. }
            | Self::SourceShape { .. }
            | Self::SourceValue { .. }
            | Self::ExcludedPair { .. }
            | Self::DomainLimit { .. }
            | Self::NodeLimit { .. }
            | Self::RefinementLimit { .. }
            | Self::WitnessWithinMargin { .. } => false,
        }
    }
}

impl MathCancellation for CylindricalProbeFailure {
    fn is_cancelled(&self) -> bool {
        match self {
            Self::Arithmetic(cause) => cause.is_cancelled(),
            Self::Policy
            | Self::Count
            | Self::Image { .. }
            | Self::Point { .. }
            | Self::Uncertified { .. } => false,
        }
    }
}

impl MathCancellation for CylindricalConeFailure {
    fn is_cancelled(&self) -> bool {
        match self {
            Self::Arithmetic(cause) => cause.is_cancelled(),
            Self::Policy(..)
            | Self::SourceCount(..)
            | Self::SourceMismatch { .. }
            | Self::ChartMismatch { .. }
            | Self::Unresolved { .. } => false,
        }
    }
}

impl MathCancellation for CylindricalProfileFailure {
    fn is_cancelled(&self) -> bool {
        match self {
            Self::Arithmetic(cause) => cause.is_cancelled(),
            Self::Policy(..)
            | Self::Interval { .. }
            | Self::Count { .. }
            | Self::Value { .. }
            | Self::Weights { .. }
            | Self::TrigonometricRange { .. }
            | Self::DirectionUnresolved { .. }
            | Self::FitChartLevel { .. }
            | Self::FitLevel { .. }
            | Self::FitPolicy { .. }
            | Self::Uncertified { .. } => false,
        }
    }
}




impl MathCancellation for CylindricalStorageFailure {
    fn is_cancelled(&self) -> bool {
        match self {
            Self::Cancelled => true,
            Self::Policy(..)
            | Self::Value { .. }
            | Self::PoleCount { .. }
            | Self::Degree { .. }
            | Self::WorkLimit { .. }
            | Self::ArithmeticBits { .. }
            | Self::CoefficientCount { .. }
            | Self::DenominatorUnresolved { .. }
            | Self::ReferenceDirectionUnresolved { .. }
            | Self::UnrepresentableBound { .. }
            | Self::Uncertified { .. } => false,
        }
    }
}

impl MathCancellation for CylindricalDepthFailure {
    fn is_cancelled(&self) -> bool {
        match self {
            Self::Value { .. }
            | Self::DepthOrder { .. }
            | Self::NonpositiveRadius { .. }
            | Self::UnrepresentableBound { .. }
            | Self::Uncertified { .. } => false,
        }
    }
}

impl MathCancellation for CylindricalMeasureComparisonFailure {
    fn is_cancelled(&self) -> bool {
        match self {
            Self::Arithmetic(cause) => cause.is_cancelled(),
            Self::Policy
            | Self::Count
            | Self::FluxBinding
            | Self::NormalBinding
            | Self::SpanBinding { .. }
            | Self::Uncertified { .. } => false,
        }
    }
}

impl MathCancellation for FrameTransportError {
    fn is_cancelled(&self) -> bool {
        match self {
            Self::SpatialInput { .. }
            | Self::SpatialLimit { .. }
            | Self::NonFinite { .. }
            | Self::ZeroAxis { .. }
            | Self::ParallelRadial { .. }
            | Self::Unrepresentable(..)
            | Self::NonPositiveDeterminant { .. }
            | Self::UncertifiedStretch { .. } => false,
        }
    }
}

impl MathCancellation for CylindricalClearanceError {
    fn is_cancelled(&self) -> bool {
        self.cause.is_cancelled()
    }
}

impl MathCancellation for CylindricalProbeError {
    fn is_cancelled(&self) -> bool {
        self.cause.is_cancelled()
    }
}

impl MathCancellation for CylindricalConeError {
    fn is_cancelled(&self) -> bool {
        self.cause.is_cancelled()
    }
}

impl MathCancellation for CylindricalProfileError {
    fn is_cancelled(&self) -> bool {
        self.cause.is_cancelled()
    }
}

impl MathCancellation for CylindricalStorageError {
    fn is_cancelled(&self) -> bool {
        self.cause.is_cancelled()
    }
}

impl MathCancellation for CylindricalLiftError {
    fn is_cancelled(&self) -> bool {
        self.cause.is_cancelled()
    }
}

impl MathCancellation for CylindricalTrimError {
    fn is_cancelled(&self) -> bool {
        self.cause.is_cancelled()
    }
}

impl MathCancellation for CylindricalMeasureComparisonError {
    fn is_cancelled(&self) -> bool {
        self.cause.is_cancelled()
    }
}



