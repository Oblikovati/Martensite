//! Report the actual stage and source for every refused attached feature.
use super::*;
use crate::surface_sketch::SurfaceFailureCause;
use martensite_tkbo::body_check::ResolutionRejected;

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
        /// Diagnostics from the underlying proof.
        detail: String,
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

impl std::fmt::Display for CylinderFeatureFailure {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            out,
            "{:?} on {:?} revision {:?} at {:?}: {:?}",
            self.kind, self.attachment, self.revision, self.stage, self.cause
        )
    }
}
impl std::error::Error for CylinderFeatureFailure {}

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
