// SPDX-License-Identifier: GPL-2.0-only
//! Immutable checked mixed prism, source history and independent native exports.
use super::{Failure, Stage, images::Image, poll};
use crate::{
    native_shape_copy::ShapeCopy,
    spatial_prism::result::{copy_failure, original_shape},
};
use martensite_tkbo::profile_extrusion::{
    AchievedTolerance, Finding, HistoryRelation, HistoryRole, Measured, OperationResult,
    PlanarProofWorkReceipt, Resolution, SelfIntersectionEvidence, ValidationReport,
};
use martensite_tkbrep::topods::Shape;
use martensite_tksketch3d::{ArcId, PointId, wire::mixed::DirectedCurveUse};

/// Authored source of a retained mixed-extrusion entity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MixedSpatialPrismSource {
    /// Selected closed region boundary, which supplies the two caps.
    Boundary,
    /// Original curve and its selected wire traversal.
    Curve(DirectedCurveUse),
    /// Authored boundary vertex. The center control is recorded by the arc receipt.
    Point(PointId),
}

/// Native image role and relation to one original source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MixedSpatialPrismHistory {
    /// Original authored source.
    pub source: MixedSpatialPrismSource,
    /// Retained native extrusion role.
    pub role: HistoryRole,
    /// Retained native extrusion relation.
    pub relation: HistoryRelation,
}

/// Exact native circular trims and original source identities for the mixed prism.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MixedPrismArcTrim {
    /// Original authored arc.
    pub arc: ArcId,
    /// Original center-control identity, with no synthetic topological vertex.
    pub center_control: PointId,
    /// Authored endpoints in selected wire order.
    pub endpoints: [PointId; 2],
    /// Actual native parameter endpoints in the selected wire direction.
    pub parameters: [f64; 2],
    /// Certified position error between the retained trim and the authored quarter at shared normalized parameters.
    pub terminal_error_upper: f64,
}

/// A stable semantic image within a detached mixed prism.
pub struct MixedSpatialPrismImage {
    /// Original identity and history role.
    pub history: MixedSpatialPrismHistory,
    /// The exact retained member in the same detached solid.
    pub shape: Shape,
}

/// Fresh detached owners of a mixed prism and all semantic images.
pub struct DetachedMixedSpatialPrism {
    /// Independent native body.
    pub solid: Shape,
    /// Its actual result members in the same order as the immutable history.
    pub images: Vec<MixedSpatialPrismImage>,
}

#[derive(Debug)]
pub(super) struct MixedEvidence {
    pub area: Measured,
    pub volume: Measured,
    pub trace_limit: f64,
    pub arc: MixedPrismArcTrim,
    pub height: f64,
    pub work: PlanarProofWorkReceipt,
}

/// Validated mixed planar prism with source-bound analytic evidence and private native owners.
#[derive(Debug)]
pub struct MixedSpatialPrismResult {
    swept: OperationResult,
    history: Vec<Image>,
    evidence: MixedEvidence,
}

impl MixedSpatialPrismResult {
    pub(super) fn new(
        swept: OperationResult,
        history: Vec<Image>,
        evidence: MixedEvidence,
    ) -> Self {
        Self {
            swept,
            history,
            evidence,
        }
    }

    /// Ordered required validation of the body. Example: `result.validation().is_valid()`.
    pub fn validation(&self) -> &ValidationReport {
        self.swept.validation()
    }

    /// Geometric invariants behind the self-intersection pass.
    /// Example: `result.self_intersection_certificate()`.
    pub fn self_intersection_certificate(&self) -> SelfIntersectionEvidence {
        self.swept.self_intersection_certificate()
    }

    /// Largest retained native tolerances. Example: `result.achieved_tolerance().max()`.
    pub fn achieved_tolerance(&self) -> AchievedTolerance {
        self.swept.achieved_tolerance()
    }

    /// Applied ordinary prism resolution. Example: `result.resolution().weld`.
    pub fn resolution(&self) -> Resolution {
        self.swept.resolution()
    }

    /// Ordered validation findings, including report-mode caps. Example: `result.findings()`.
    pub fn findings(&self) -> &[Finding] {
        self.swept.findings()
    }

    /// Outward enclosure of the translated cap area. Example: `result.area().error_bound`.
    pub fn area(&self) -> Measured {
        self.evidence.area
    }

    /// Outward volume enclosure of the retained normal sweep. Example: `result.volume().error_bound`.
    pub fn volume(&self) -> Measured {
        self.evidence.volume
    }

    /// Requested source trace in model units. Example: `result.source_trace_limit()`.
    pub fn source_trace_limit(&self) -> f64 {
        self.evidence.trace_limit
    }

    /// Certified trace bound including the binary64 quarter endpoint.
    /// Example: `result.source_trace_bound()`.
    pub fn source_trace_bound(&self) -> f64 {
        self.evidence.arc.terminal_error_upper
    }

    /// Signed physical separation of the exact translated caps. Example: `result.signed_height()`.
    pub fn signed_height(&self) -> f64 {
        self.evidence.height
    }

    /// Authored arc and retained native trim provenance. Example: `result.arc_trim().center_control`.
    pub fn arc_trim(&self) -> MixedPrismArcTrim {
        self.evidence.arc
    }

    /// Work observed in the ordinary planar region proof. Example: `result.region_proof_work()`.
    pub fn region_proof_work(&self) -> PlanarProofWorkReceipt {
        self.evidence.work
    }

    /// Every accepted native image in stable source and role order. Example: `result.history().len()`.
    pub fn history(&self) -> impl ExactSizeIterator<Item = &MixedSpatialPrismHistory> {
        self.history.iter().map(|image| &image.history)
    }

    /// Copy every retained owner and resolve its exact semantic images inside that copy.
    /// Example: `result.export_brep(|| false)?`.
    pub fn export_brep(
        &self,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<DetachedMixedSpatialPrism, Failure> {
        poll(&mut cancelled, Stage::Export)?;
        let mut copy = ShapeCopy::new(self.swept.solid()).map_err(copy_failure)?;
        let mut images = Vec::with_capacity(self.history.len());
        for entry in &self.history {
            poll(&mut cancelled, Stage::Export)?;
            let original = original_shape(&self.swept, entry.result)?;
            let shape = copy.bind(original).map_err(copy_failure)?;
            images.push(MixedSpatialPrismImage {
                history: entry.history,
                shape,
            });
        }
        poll(&mut cancelled, Stage::Export)?;
        Ok(DetachedMixedSpatialPrism {
            solid: copy.into_root(),
            images,
        })
    }
}
