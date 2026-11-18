//! Typed numerical admission differs from unresolved or exhausted proof work.
use super::*;
pub(super) fn profile(cause: &CylindricalProfileFailure) -> Class {
    use CylindricalProfileFailure as F;
    match cause {
        F::Policy(_)
        | F::Interval { .. }
        | F::Value { .. }
        | F::Weights { .. }
        | F::FitPolicy { .. } => Class::InvalidInput,
        F::Count { .. } | F::FitIntervalLevel { .. } => Class::Unsupported,
        F::Arithmetic(_)
        | F::TrigonometricRange { .. }
        | F::DirectionUnresolved { .. }
        | F::FitChartLevel { .. }
        | F::FitLevel { .. }
        | F::Uncertified { .. } => Class::Incomplete,
    }
}
pub(super) fn candidate(cause: &cone_mapping::fit::FitFailure) -> Class {
    use cone_mapping::fit::FitFailure as F;
    match cause {
        F::Count(_) | F::Chart { .. } => Class::Unsupported,
        F::Nonfinite { .. } | F::Support(_) | F::Apex { .. } => Class::InvalidInput,
    }
}
pub(super) fn coefficients(cause: &cone_mapping::ConeFitFailure) -> Class {
    use cone_mapping::ConeFitFailure as F;
    match cause {
        F::Shape { .. } => Class::Unsupported,
        F::Nonfinite { .. } | F::InvalidAngle(_) | F::NonpositiveWeight { .. } => {
            Class::InvalidInput
        }
        F::Overflow { .. } => Class::Incomplete,
    }
}
pub(super) fn boundary(cause: &ConicalBoundaryError) -> Class {
    match cause.cause() {
        ConicalBoundaryFailure::Policy { .. } => Class::InvalidInput,
        ConicalBoundaryFailure::Shape { .. } => Class::Unsupported,
        ConicalBoundaryFailure::Proof(proof) => boundary_proof(proof.kind()),
    }
}
fn boundary_proof(kind: ConicalBoundaryFailureKind) -> Class {
    use ConicalBoundaryFailureKind as K;
    match kind {
        K::InvalidSource | K::InvalidAngularFit | K::InvalidSlantFit => Class::InvalidInput,
        K::Arithmetic
        | K::Binding
        | K::UnresolvedOriginal
        | K::UnresolvedAngular
        | K::UnresolvedSlant
        | K::Polynomial
        | K::Storage
        | K::Budget => Class::Incomplete,
    }
}
pub(super) fn depth(cause: &ConicalDepthFailure) -> Class {
    use ConicalDepthFailure as F;
    match cause {
        F::InvalidDepthLimit { .. }
        | F::InvalidPolicy { .. }
        | F::InvalidThreshold(_)
        | F::InvalidDepths(_)
        | F::InvalidConeInputs { .. } => Class::InvalidInput,
        F::Source(cause) => source(cause),
        F::Trigonometric(cause) => profile(cause),
        F::Unresolved { .. } | F::Arithmetic(_) => Class::Incomplete,
    }
}
fn source(cause: &ConicalSourceFailure) -> Class {
    match cause {
        ConicalSourceFailure::PoleCount(_) => Class::Unsupported,
        ConicalSourceFailure::NonFinite { .. } => Class::InvalidInput,
        ConicalSourceFailure::Arithmetic(_) => Class::Incomplete,
    }
}
pub(super) fn sector(cause: &ConicalSectorFailure) -> Class {
    use ConicalSectorFailure as F;
    match cause {
        F::Policy { .. } | F::Point { .. } | F::Angle(_) => Class::InvalidInput,
        F::SpanCount(_) | F::PoleCount { .. } | F::UnresolvedWedge { .. } | F::UnresolvedSine => {
            Class::Unsupported
        }
        F::Trigonometric(cause) => profile(cause),
        F::Arithmetic(_) => Class::Incomplete,
    }
}
pub(super) fn joins(cause: &ConicalJoinsFailure) -> Class {
    use ConicalJoinsFailure as F;
    match cause {
        F::SpanCount { .. } => Class::Unsupported,
        F::Contours { .. } | F::Policy { .. } => Class::InvalidInput,
        F::Boundary { .. } | F::Proof { .. } => Class::Incomplete,
    }
}

pub(super) fn reference(cause: &ConicalReferenceFailure) -> Class {
    match cause {
        ConicalReferenceFailure::Binding { .. }
        | ConicalReferenceFailure::Bound { .. }
        | ConicalReferenceFailure::Arithmetic(_) => Class::Incomplete,
        ConicalReferenceFailure::Trigonometric(cause) => profile(cause),
    }
}
