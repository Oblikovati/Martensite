//! Integration admission and work exhaustion retain different public classes.
use super::*;
pub(super) fn flux(cause: &AnalyticConicalFluxFailure) -> Class {
    use AnalyticConicalFluxFailure as F;
    match cause {
        F::Input { cause, .. } => cylindrical(cause),
        F::ArithmeticPolicy(_) | F::Carrier { .. } => Class::InvalidInput,
        F::Coefficients { cause, .. } => numerical::profile(cause),
        F::Cap { cause, .. } => polynomial(cause),
        F::Integration { cause, .. } => rational(cause),
    }
}
fn cylindrical(cause: &CylindricalFluxFailure) -> Class {
    use CylindricalFluxFailure as F;
    match cause {
        F::Policy(_) | F::Radii(_) | F::PairCounts { .. } | F::PairWeights { .. } => {
            Class::InvalidInput
        }
        F::SpanCount { .. } => Class::Unsupported,
        F::Geometry { .. } => Class::Incomplete,
        F::Cap { cause, .. } => polynomial(cause),
        F::Integration { cause, .. } => rational(cause),
    }
}
fn polynomial(cause: &PolynomialIntegralFailure) -> Class {
    use PolynomialIntegralFailure as F;
    match cause {
        F::Policy(_) | F::EmptyBoundary => Class::InvalidInput,
        F::CurveCount { .. } | F::Degree { .. } | F::RationalWeights { .. } => Class::Unsupported,
        F::Metric(_) | F::Unrepresentable { .. } | F::ErrorLimit { .. } => Class::Incomplete,
    }
}
fn rational(cause: &RationalIntegralFailure) -> Class {
    use RationalIntegralFailure as F;
    match cause {
        F::Policy(_) | F::EmptyBoundary | F::DenominatorSign { .. } => Class::InvalidInput,
        F::CurveCount { .. } | F::Degree { .. } => Class::Unsupported,
        F::Metric(_)
        | F::Unrepresentable { .. }
        | F::ErrorLimit { .. }
        | F::WorkLimit { .. }
        | F::IndeterminateOrientation { .. } => Class::Incomplete,
    }
}
pub(super) fn clearance(cause: &CylindricalClearanceFailure) -> Class {
    use CylindricalClearanceFailure as F;
    match cause {
        F::Policy(_)
        | F::Value { .. }
        | F::PeriodicImagePolicy { .. }
        | F::SourceValue { .. }
        | F::ExcludedPair { .. } => Class::InvalidInput,
        F::InputCount { .. } | F::SourceShape { .. } => Class::Unsupported,
        F::PeriodicImageLimit { .. }
        | F::PeriodicImage { .. }
        | F::PeriodicSlabUnresolved { .. }
        | F::Arithmetic(_)
        | F::DomainLimit { .. }
        | F::NodeLimit { .. }
        | F::RefinementLimit { .. }
        | F::WitnessWithinMargin { .. } => Class::Incomplete,
    }
}
pub(super) fn authored(cause: &ConicalAuthoredFluxError) -> Class {
    if let Some((_, _, cause)) = cause.depth_cause() {
        return numerical::depth(cause);
    }
    match cause.stage() {
        ConicalAuthoredFluxStage::Shape => Class::Unsupported,
        ConicalAuthoredFluxStage::Volume | ConicalAuthoredFluxStage::Comparison => {
            Class::Incomplete
        }
    }
}

pub(super) fn reference(cause: &ConicalReferenceVolumeFailure) -> Class {
    use ConicalReferenceVolumeFailure as F;
    match cause {
        F::Input { .. } | F::Policy(_) => Class::InvalidInput,
        F::Shape { .. } => Class::Unsupported,
        F::Arithmetic(_) => Class::Incomplete,
        F::Trigonometric(cause) => numerical::profile(cause),
        F::Integration { cause, .. } => rational(cause),
    }
}
