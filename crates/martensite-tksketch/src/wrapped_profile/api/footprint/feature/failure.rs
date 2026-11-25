//! Report the actual stage and source for every refused attached feature.
use super::*;
use crate::surface_sketch::{HistorySource, SurfaceFailureCause, WrappedFailureClass};
use martensite_tkbo::body_check::ResolutionRejected;
use std::error::Error;

/// Boundary at which an attached feature stopped.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CylinderFeatureStage {
    /// Attachment, direction, depths or source solid.
    Admission,
    /// Normal tool construction and trim proofs.
    Tool,
    /// Native material operation.
    Boolean,
    /// Source-to-output correspondence.
    History,
    /// Frozen topology and semantic references.
    Encoding,
    /// Ordered validation of the frozen result.
    Validation,
    /// Final cancellation boundary before returning the result.
    Publication,
}

/// Typed reason a boss or pocket could not publish a body.
#[derive(Debug)]
#[non_exhaustive]
pub enum CylinderFeatureCause {
    /// A cancellation was observed at this stage.
    Cancelled,
    /// The feature requires depths `[0, positive finite depth]`.
    Depths([f64; 2]),
    /// A solid support face requires forward or reversed orientation.
    FaceOrientation(Orientation),
    /// The radial sense did not agree with the selected face and operation.
    Normal {
        /// Requested sense.
        actual: NormalSense,
        /// Sense required by the face and operation.
        expected: NormalSense,
    },
    /// The feature requires one solid with no detached geometry.
    SingleSolid {
        /// Kind of the native result or source root.
        root: ShapeEnum,
        /// Root members inspected when the root was a compound.
        members: usize,
    },
    /// The attachment, chart or trim-checked tool refused the request.
    Footprint(Box<CylinderFootprintFailure>),
    /// The conical attachment, chart or trim-checked tool refused the request.
    ConeFootprint(Box<ConeFootprintFailure>),
    /// The qualified normal tool could not be exported.
    ToolExport(Box<WrappedNormalFailure>),
    /// The explicit attached proof cannot admit the source or qualify its result.
    DirectAttachment {
        /// The source shape or proof invariant requiring evidence.
        expected: &'static str,
    },
    /// The directed original contours could not prove the selected planar material.
    DirectedPlanarMaterial(Box<martensite_tkbo::body_check::DirectedCapFailure>),
    /// The original or decoded stock failed its frustum certificate.
    ConicalStock(Box<ConicalStockFailure>),
    /// The decoded stock trim proof must be nonempty and cover every retained side.
    ConicalCoverage {
        /// Checked stock trim domains, required to be positive.
        stock_trim_domains: usize,
        /// Retained sides established by the geometry proof.
        retained_sides: usize,
        /// Retained sides established by the contact proof.
        qualified_sides: usize,
    },
    /// A native query failed during the stated stage.
    Native {
        /// Native query name.
        query: &'static str,
        /// Original failure.
        cause: Box<Failure>,
    },
    /// A source-bound proof failed or could not finish within its arithmetic budget.
    NumericalProof {
        /// Name of the required proof.
        query: &'static str,
        /// Classification derived from the retained proof, including a polled cancellation.
        class: WrappedFailureClass,
        /// Affected authored regions, curves or vertices, in deterministic order.
        entities: Vec<HistorySource>,
        /// Original proof error and its owned source chain.
        cause: Box<dyn Error + Send + Sync>,
    },
    /// Required topology or source proof did not have the expected number of members.
    Cardinality {
        /// Source or topology relationship that failed.
        query: &'static str,
        /// Number of actual members.
        actual: usize,
        /// Number required by the certificate.
        expected: usize,
        /// Authored entities identified by the caller boundary.
        entities: Vec<HistorySource>,
    },
    /// Native execution did not yield a completed successful Boolean.
    Boolean {
        /// Execution status returned by the native operation.
        completion: ProgressCompletion,
        /// Serialized native error report.
        errors: String,
    },
    /// Reading native diagnostics failed.
    Diagnostic(std::io::Error),
    /// The history of one input could not be established.
    History {
        /// Exact authored or revision-local source.
        source: CylinderFeatureSource,
        /// Required history condition.
        expected: &'static str,
    },
    /// An output entity had no recorded parents.
    UnownedImage(CylinderBodyRef),
    /// The selected region's exposed cap has no surviving face image.
    MissingCap(SurfaceRegionId),
    /// Output encoding or decoding failed.
    Encoding(Box<SurfaceFailureCause>),
    /// The retained graph or references did not survive a byte-identical round trip.
    ArchiveMismatch,
    /// Validation settings could not be admitted.
    Resolution(ResolutionRejected),
    /// One or more required validation stages failed or were inconclusive.
    Validation(Box<ValidationReport>),
}

/// Refusal with the revision, operation and actual failed stage.
#[derive(Debug)]
pub struct CylinderFeatureFailure {
    /// Selected face identity.
    pub attachment: FaceAttachmentId,
    /// Revision supplied for the support.
    pub revision: SupportRevision,
    /// Requested material operation.
    pub kind: CylinderFeatureKind,
    /// Stage that refused the request.
    pub stage: CylinderFeatureStage,
    /// Reason the stage could not complete.
    pub cause: Box<CylinderFeatureCause>,
}

impl CylinderFeatureFailure {
    /// Actual typed refusal category. Example: `let class = failure.class();`.
    pub fn class(&self) -> WrappedFailureClass {
        use CylinderFeatureCause as Cause;
        use WrappedFailureClass as Class;
        match self.cause.as_ref() {
            Cause::NumericalProof { class, .. } => *class,
            Cause::Cancelled => Class::Cancelled,
            Cause::DirectedPlanarMaterial(cause) => directed_class(cause),
            Cause::ToolExport(cause) => cause.class(),
            Cause::Footprint(cause) => footprint_class(cause),
            Cause::Native { .. } | Cause::Diagnostic(_) | Cause::Encoding(_) => Class::Native,
            Cause::Depths(_)
            | Cause::FaceOrientation(_)
            | Cause::Normal { .. }
            | Cause::Resolution(_) => Class::InvalidInput,
            Cause::DirectAttachment { .. } if self.stage == CylinderFeatureStage::Admission => {
                Class::Unsupported
            }
            _ => Class::Incomplete,
        }
    }

    /// Known affected authored members. Example: `let sources = failure.entities();`.
    pub fn entities(&self) -> Vec<HistorySource> {
        use CylinderFeatureCause as Cause;
        match self.cause.as_ref() {
            Cause::NumericalProof { entities, .. } | Cause::Cardinality { entities, .. } => {
                entities.clone()
            }
            Cause::DirectedPlanarMaterial(cause) => directed_entities(cause),
            Cause::ToolExport(cause) => cause.entities().to_vec(),
            Cause::Footprint(cause) => footprint_entities(cause),
            Cause::History {
                source: CylinderFeatureSource::Tool(key),
                ..
            } => vec![history_entity(*key)],
            Cause::MissingCap(region) => vec![HistorySource::Region(*region)],
            _ => Vec::new(),
        }
    }
}

fn footprint_class(cause: &CylinderFootprintFailure) -> WrappedFailureClass {
    use crate::surface_sketch::attachment::AttachmentFailureClass as Attachment;
    use CylinderFootprintFailure as Cause;
    use WrappedFailureClass as Class;
    match cause {
        Cause::Attachment(cause) => match cause.class() {
            Attachment::Native | Attachment::Persistence => Class::Native,
            Attachment::InvalidInput | Attachment::StaleRevision => Class::InvalidInput,
            _ => Class::Incomplete,
        },
        Cause::SupportMismatch { .. } | Cause::EmptySelection => Class::InvalidInput,
        Cause::UnsupportedTrim { .. } => Class::Unsupported,
        Cause::Native(_) => Class::Native,
        Cause::Source(cause) | Cause::Extrusion(cause) => cause.class(),
        _ => Class::Incomplete,
    }
}

fn footprint_entities(cause: &CylinderFootprintFailure) -> Vec<HistorySource> {
    use CylinderFootprintFailure as Cause;
    match cause {
        Cause::Source(cause) | Cause::Extrusion(cause) => cause.entities().to_vec(),
        Cause::ControlOutside { region, curve, .. }
        | Cause::FittedControlOutside { region, curve, .. } => {
            vec![HistorySource::Region(*region), HistorySource::Curve(*curve)]
        }
        Cause::InnerTrimOverlap { region, .. } | Cause::OuterTrim { region, .. } => {
            vec![HistorySource::Region(*region)]
        }
        _ => Vec::new(),
    }
}

pub(super) fn history_entity(key: WrappedHistoryKey) -> HistorySource {
    crate::surface_sketch::wrapped_profile::api::error::public_source(key.source)
}

fn directed_class(cause: &martensite_tkbo::body_check::DirectedCapFailure) -> WrappedFailureClass {
    use WrappedFailureClass as Class;
    use martensite_tkbo::body_check::DirectedCapCause as Cause;
    if cause.cancelled {
        return Class::Cancelled;
    }
    match cause.cause.as_ref() {
        Cause::Cancelled => Class::Cancelled,
        Cause::Policy { .. }
        | Cause::Components { .. }
        | Cause::Shape { .. }
        | Cause::Profile(_) => Class::InvalidInput,
        Cause::Topology(_) => Class::Native,
        _ => Class::Incomplete,
    }
}

fn directed_entities(
    cause: &martensite_tkbo::body_check::DirectedCapFailure,
) -> Vec<HistorySource> {
    use martensite_tkbo::body_check::DirectedCapCause as Cause;
    let mut entities = Vec::new();
    match cause.cause.as_ref() {
        Cause::Profile(cause) => {
            crate::surface_sketch::wrapped_profile::api::error::decline_entities(
                cause,
                &mut entities,
            )
        }
        Cause::Pair { curves, .. } => entities.extend(
            curves
                .iter()
                .map(|curve| HistorySource::Curve(SurfaceCurveId(curve.0))),
        ),
        _ => (),
    }
    entities.sort();
    entities.dedup();
    entities
}

impl std::fmt::Display for CylinderFeatureFailure {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            out,
            "{:?} on {:?} revision {:?} at {:?}",
            self.kind, self.attachment, self.revision, self.stage
        )?;
        match self.cause.as_ref() {
            CylinderFeatureCause::NumericalProof {
                query,
                class,
                entities,
                cause,
            } => {
                write!(out, " {query} {class:?} entities={entities:?}: {cause}")
            }
            cause => write!(out, ": {cause:?}"),
        }
    }
}

impl Error for CylinderFeatureFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use CylinderFeatureCause as Cause;
        match self.cause.as_ref() {
            Cause::Footprint(cause) => Some(cause.as_ref()),
            Cause::ConeFootprint(cause) => Some(cause.as_ref()),
            Cause::ToolExport(cause) => Some(cause.as_ref()),
            Cause::DirectedPlanarMaterial(cause) => Some(cause.as_ref()),
            Cause::ConicalStock(cause) => Some(cause.as_ref()),
            Cause::Native { cause, .. } => Some(cause.as_ref()),
            Cause::NumericalProof { cause, .. } => Some(cause.as_ref()),
            Cause::Diagnostic(cause) => Some(cause),
            Cause::Encoding(cause) => Some(cause.as_ref()),
            _ => None,
        }
    }
}

pub(super) type FeatureChecked<T> = Result<T, CylinderFeatureFailure>;

#[derive(Clone, Copy)]
pub(super) struct Attempt {
    pub attachment: FaceAttachmentId,
    pub revision: SupportRevision,
    pub kind: CylinderFeatureKind,
}

impl Attempt {
    pub(super) fn fail(
        self,
        stage: CylinderFeatureStage,
        cause: CylinderFeatureCause,
    ) -> CylinderFeatureFailure {
        CylinderFeatureFailure {
            attachment: self.attachment,
            revision: self.revision,
            kind: self.kind,
            stage,
            cause: Box::new(cause),
        }
    }
    pub(super) fn footprint(self, cause: CylinderFootprintFailure) -> CylinderFeatureFailure {
        let stage = match &cause {
            CylinderFootprintFailure::Attachment(_)
            | CylinderFootprintFailure::SupportMismatch { .. }
            | CylinderFootprintFailure::UnsupportedTrim { .. }
            | CylinderFootprintFailure::Native(_) => CylinderFeatureStage::Admission,
            _ => CylinderFeatureStage::Tool,
        };
        self.fail(stage, CylinderFeatureCause::Footprint(Box::new(cause)))
    }
    pub(super) fn cone_footprint(self, cause: ConeFootprintFailure) -> CylinderFeatureFailure {
        let stage = match &cause {
            ConeFootprintFailure::Attachment(_)
            | ConeFootprintFailure::SupportMismatch { .. }
            | ConeFootprintFailure::UnsupportedTrim { .. }
            | ConeFootprintFailure::Native(_) => CylinderFeatureStage::Admission,
            _ => CylinderFeatureStage::Tool,
        };
        self.fail(stage, CylinderFeatureCause::ConeFootprint(Box::new(cause)))
    }
    pub(super) fn tool(self, cause: WrappedNormalFailure) -> CylinderFeatureFailure {
        self.fail(
            CylinderFeatureStage::Tool,
            CylinderFeatureCause::ToolExport(Box::new(cause)),
        )
    }
    pub(super) fn poll(
        self,
        cancelled: &mut CancelPoll<'_>,
        stage: CylinderFeatureStage,
    ) -> FeatureChecked<()> {
        if cancelled() {
            return Err(self.fail(stage, CylinderFeatureCause::Cancelled));
        }
        Ok(())
    }
    pub(super) fn native<T>(
        self,
        stage: CylinderFeatureStage,
        query: &'static str,
        result: Result<T, Failure>,
    ) -> FeatureChecked<T> {
        result.map_err(|cause| {
            self.fail(
                stage,
                CylinderFeatureCause::Native {
                    query,
                    cause: Box::new(cause),
                },
            )
        })
    }
    pub(super) fn encoding(self, cause: SurfaceFailureCause) -> CylinderFeatureFailure {
        self.fail(
            CylinderFeatureStage::Encoding,
            CylinderFeatureCause::Encoding(Box::new(cause)),
        )
    }
    pub(super) fn history(
        self,
        source: CylinderFeatureSource,
        expected: &'static str,
    ) -> CylinderFeatureFailure {
        self.fail(
            CylinderFeatureStage::History,
            CylinderFeatureCause::History { source, expected },
        )
    }
}
