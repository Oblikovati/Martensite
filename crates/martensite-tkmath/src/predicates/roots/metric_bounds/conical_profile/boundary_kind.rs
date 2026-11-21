// SPDX-License-Identifier: GPL-2.0-only
//! Classify component refusals without parsing their diagnostics.
use super::angle::*;
use super::angular::AngularSpanFailure;
use super::combined::CombinedFailure;
use super::slant::SlantFailure;
use super::spatial::SpatialFailure;

/// Stable class of a failed boundary component proof.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConicalBoundaryFailureKind {
    /// Bounded arithmetic failed or stopped before completing.
    Arithmetic,
    /// Supplied component bindings disagree.
    Binding,
    /// Input domain or source was invalid.
    InvalidSource,
    /// Original-to-child radius could not be bounded away from the apex.
    UnresolvedOriginal,
    /// Angular fit or support was invalid.
    InvalidAngularFit,
    /// A positive angular distance or argument reduction could not be proven.
    UnresolvedAngular,
    /// Slant coefficients were invalid.
    InvalidSlantFit,
    /// Positivity of a slant quantity could not be proven.
    UnresolvedSlant,
    /// A component polynomial or trigonometric certificate did not complete.
    Polynomial,
    /// A stored edge failed its coefficient check.
    Storage,
    /// The original spatial budget could not be certified.
    Budget,
}
pub(super) fn classify(failure: &CombinedFailure) -> ConicalBoundaryFailureKind {
    use ConicalBoundaryFailureKind as K;
    if super::boundary_cause::combined(failure).is_some() {
        return K::Arithmetic;
    }
    match failure {
        CombinedFailure::Binding { .. } => K::Binding,
        CombinedFailure::Spatial(cause) => spatial(cause),
        CombinedFailure::Storage { .. } => K::Storage,
        CombinedFailure::Arithmetic(_) => K::Arithmetic,
        CombinedFailure::Uncertified { .. } => K::Budget,
    }
}
pub(super) fn spatial(failure: &SpatialFailure) -> ConicalBoundaryFailureKind {
    use ConicalBoundaryFailureKind as K;
    match failure {
        SpatialFailure::Domain { .. } | SpatialFailure::Source(_) => K::InvalidSource,
        SpatialFailure::Original(cause) => original(cause),
        SpatialFailure::Angular(cause) => angular(cause),
        SpatialFailure::Slant(cause) => slant(cause),
        SpatialFailure::Arithmetic(_) => K::Arithmetic,
        SpatialFailure::Uncertified { .. } => K::Budget,
    }
}
pub(super) fn original(failure: &ConicalOriginalFailure) -> ConicalBoundaryFailureKind {
    use ConicalBoundaryFailureKind as K;
    match failure {
        ConicalOriginalFailure::Interval { .. } | ConicalOriginalFailure::Source(_) => {
            K::InvalidSource
        }
        ConicalOriginalFailure::Radius { .. } => K::UnresolvedOriginal,
        ConicalOriginalFailure::Polynomial(_) => K::Polynomial,
        ConicalOriginalFailure::Arithmetic(_) => K::Arithmetic,
    }
}
pub(super) fn angular(failure: &AngularSpanFailure) -> ConicalBoundaryFailureKind {
    use ConicalBoundaryFailureKind as K;
    match failure {
        AngularSpanFailure::InvalidFit { .. } | AngularSpanFailure::MismatchedAnchor { .. } => {
            K::InvalidAngularFit
        }
        AngularSpanFailure::UnresolvedDistance { .. } => K::UnresolvedAngular,
        AngularSpanFailure::Initial(cause) => initial(cause),
        AngularSpanFailure::Polynomial(_) => K::Polynomial,
        AngularSpanFailure::Arithmetic(_) => K::Arithmetic,
    }
}
pub(super) fn initial(failure: &InitialAngleFailure) -> ConicalBoundaryFailureKind {
    use ConicalBoundaryFailureKind as K;
    match failure {
        InitialAngleFailure::InvalidPoint(_)
        | InitialAngleFailure::InvalidSupport { .. }
        | InitialAngleFailure::InvalidFit { .. } => K::InvalidAngularFit,
        InitialAngleFailure::ReductionOutOfRange => K::UnresolvedAngular,
        InitialAngleFailure::Trigonometric(_) => K::Polynomial,
        InitialAngleFailure::Arithmetic(_) => K::Arithmetic,
    }
}
pub(super) fn slant(failure: &SlantFailure) -> ConicalBoundaryFailureKind {
    use ConicalBoundaryFailureKind as K;
    match failure {
        SlantFailure::Shape { .. } | SlantFailure::Nonfinite { .. } => K::InvalidSlantFit,
        SlantFailure::Nonpositive { .. } => K::UnresolvedSlant,
        SlantFailure::Polynomial(_) => K::Polynomial,
        SlantFailure::Arithmetic(_) => K::Arithmetic,
    }
}
