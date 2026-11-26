// SPDX-License-Identifier: GPL-2.0-only
use super::LoftSectionId;
use crate::spatial_prism::SpatialPrismFailure;
use martensite_tkbo::body_check::{ResolutionRejected, ValidationReport};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::predicates::{Homothety3dFailure, NonFiniteCoordinate};
use martensite_tkoffset::brep_offset_api::CorrespondingRuledPairFailure;
use martensite_tksketch3d::{PointId, wire::WireFailure};
use std::{error::Error, fmt};

/// A phase of the bounded native loft.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoftStage {
    /// Exact authored shape and correspondence.
    Admission,
    /// Private native wire creation.
    Materialize,
    /// The ordinary ruled builder.
    Construction,
    /// Four ordered body stages.
    Validation,
    /// Retained output trace and exact incidence.
    Binding,
    /// Immutable result publication.
    Publication,
    /// Fresh native export and history copy.
    Export,
}

/// Failure of the bounded ruled loft operation.
#[derive(Debug)]
#[non_exhaustive]
pub enum LoftFailure {
    /// The two selections must have distinct identities.
    RepeatedSection(LoftSectionId),
    /// The selected input must be a closed line recipe.
    OpenSection(LoftSectionId),
    /// The caller trace limit must be finite and positive.
    InvalidTraceLimit(f64),
    /// Invalid operation resolution.
    Resolution(ResolutionRejected),
    /// The section could not be proved exactly planar.
    SectionPlane {
        /// Affected input.
        section: LoftSectionId,
        /// Source plane failure.
        source: SpatialPrismFailure,
    },
    /// The supplied order has no exact positive homothety.
    Incompatible {
        /// The two source sections in requested order.
        sections: [LoftSectionId; 2],
        /// Exact mathematical correspondence failure.
        cause: Homothety3dFailure,
    },
    /// The two sections occupy the same plane.
    CoincidentPlanes,
    /// Exact station separation could not be evaluated.
    Predicate(NonFiniteCoordinate),
    /// A private native wire could not be built.
    Wire {
        /// Affected input.
        section: LoftSectionId,
        /// Original detached wire failure.
        source: WireFailure,
    },
    /// A native wire exceeds the source trace limit.
    InputTrace {
        /// Affected input.
        section: LoftSectionId,
        /// Its certified error bound.
        bound: f64,
        /// Original acceptance limit.
        limit: f64,
    },
    /// The ordinary pair builder could not complete or capture its section images.
    NativeBuild {
        /// The two source sections in requested order.
        sections: [LoftSectionId; 2],
        /// The native typed construction or capture failure.
        source: CorrespondingRuledPairFailure,
    },
    /// Body validation stopped or failed.
    Validation(Box<ValidationReport>),
    /// Actual topological correspondence is missing or ambiguous.
    Binding(&'static str),
    /// A retained vertex no longer matches a selected section point.
    SectionVertexTrace {
        /// Affected input.
        section: LoftSectionId,
        /// Original point identity.
        point: PointId,
        /// Computed spatial enclosure.
        bound: f64,
        /// Original acceptance limit.
        limit: f64,
    },
    /// Actual affine trim or volume proof did not complete.
    OutputProof(SpatialPrismFailure),
    /// Native failure outside the pair builder.
    Native {
        /// Stage that failed.
        stage: LoftStage,
        /// Original native error.
        source: Box<Failure>,
    },
    /// Cooperative cancellation was observed.
    Cancelled(LoftStage),
}

impl fmt::Display for LoftFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RepeatedSection(id) => {
                write!(f, "loft section {id:?} must have a distinct identity")
            }
            Self::OpenSection(id) => write!(f, "loft section {id:?} must be closed"),
            Self::InvalidTraceLimit(limit) => {
                write!(f, "loft trace limit {limit} must be finite and positive")
            }
            Self::Resolution(error) => write!(
                f,
                "loft resolution {}={} must be finite and positive",
                error.field, error.value
            ),
            Self::SectionPlane { section, source } => write!(
                f,
                "loft section {section:?} requires an exact plane ({source})"
            ),
            Self::Incompatible { sections, cause } => write!(
                f,
                "loft sections {sections:?} require a positive homothety ({cause:?})"
            ),
            Self::CoincidentPlanes => write!(f, "loft section planes must be distinct"),
            Self::Predicate(source) => write!(f, "loft station predicate failed ({source})"),
            Self::Wire { section, source } => write!(
                f,
                "loft section {section:?} failed native materialization ({source})"
            ),
            Self::InputTrace {
                section,
                bound,
                limit,
            } => write!(
                f,
                "loft section {section:?} has trace bound {bound}, above {limit}"
            ),
            Self::NativeBuild { sections, source } => {
                write!(f, "native ruled pair {sections:?} declined ({source})")
            }
            Self::Validation(report) => write!(
                f,
                "loft requires all ordered body checks ({:?})",
                report.first_non_pass()
            ),
            Self::Binding(expected) => write!(f, "loft history requires {expected}"),
            Self::SectionVertexTrace {
                section,
                point,
                bound,
                limit,
            } => write!(
                f,
                "loft point {section:?} {point:?} has trace bound {bound}, above {limit}"
            ),
            Self::OutputProof(source) => write!(f, "retained loft proof failed ({source})"),
            Self::Native { stage, source } => write!(f, "loft {stage:?} failed ({source})"),
            Self::Cancelled(stage) => write!(f, "loft cancelled during {stage:?}"),
        }
    }
}

impl Error for LoftFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::SectionPlane { source, .. } | Self::OutputProof(source) => Some(source),
            Self::NativeBuild { source, .. } => Some(source),
            Self::Predicate(source) => Some(source),
            Self::Wire { source, .. } => Some(source),
            Self::Native { source, .. } => Some(source.as_ref()),
            _ => None,
        }
    }
}

pub(super) fn native<T>(result: Result<T, Failure>, stage: LoftStage) -> Result<T, LoftFailure> {
    result.map_err(|source| LoftFailure::Native {
        stage,
        source: Box::new(source),
    })
}
