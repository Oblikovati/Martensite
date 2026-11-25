//! Actual typed numerical causes retain the same policy/resource distinction at publication.
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::BoundedArithmeticFailure;
pub(super) fn actual(cause: &ConicalActualPatchFailure) -> Class {
    match cause {
        ConicalActualPatchFailure::Policy { .. } | ConicalActualPatchFailure::Value { .. } => {
            Class::InvalidInput
        }
        ConicalActualPatchFailure::Shape { .. } => Class::Unsupported,
        ConicalActualPatchFailure::Binding { .. }
        | ConicalActualPatchFailure::Unresolved { .. }
        | ConicalActualPatchFailure::UnresolvedSidePair { .. } => Class::Incomplete,
        ConicalActualPatchFailure::SidePair { cause, .. }
        | ConicalActualPatchFailure::CapPair { cause, .. } => actual(cause),
        ConicalActualPatchFailure::Arithmetic(cause) => arithmetic(cause),
    }
}
pub(super) fn rows(cause: &ConicalBoundaryZRowFailure) -> Class {
    match cause {
        ConicalBoundaryZRowFailure::Boundary(cause) => numerical::boundary(cause),
        ConicalBoundaryZRowFailure::Gamma(cause) => actual(&cause.cause),
        ConicalBoundaryZRowFailure::Arithmetic(cause) => arithmetic(cause),
        ConicalBoundaryZRowFailure::Binding { .. }
        | ConicalBoundaryZRowFailure::Row { .. }
        | ConicalBoundaryZRowFailure::Ancestry { .. } => Class::Incomplete,
    }
}
fn arithmetic(cause: &BoundedArithmeticFailure) -> Class {
    match cause {
        BoundedArithmeticFailure::NonFiniteScalar { .. } => Class::InvalidInput,
        BoundedArithmeticFailure::Cancelled => Class::Cancelled,
        BoundedArithmeticFailure::CoefficientCount { .. } => Class::Unsupported,
        BoundedArithmeticFailure::WorkLimit { .. }
        | BoundedArithmeticFailure::ArithmeticBits { .. }
        | BoundedArithmeticFailure::DenominatorUnresolved { .. }
        | BoundedArithmeticFailure::UnrepresentableBound { .. } => Class::Incomplete,
    }
}
