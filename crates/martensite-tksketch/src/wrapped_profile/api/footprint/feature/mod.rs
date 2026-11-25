//! Publish a wrapped boss or pocket only after trim, Boolean and frozen-body checks.
use super::*;
use crate::surface_sketch::{HistoryRole, NormalSense, encoding};
use martensite_tkbo::{
    body_check::ValidationReport,
    bop_algo::{Operation, ProgressCompletion},
    brep_algo_api::BRepAlgoAPIBooleanOperation,
};
use martensite_tkbrep::brep_tools::History;
use martensite_tkernel::{message::ProgressRange, ncollection::List};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use std::collections::BTreeMap;

mod body;
mod cone;
pub use cone::*;
mod direct;
pub use direct::{CertifiedConeAttachmentReport, CertifiedConeFeatureResult};
mod failure;
mod history;
mod source_members;
use failure::{Attempt, FeatureChecked};
pub use failure::{CylinderFeatureCause, CylinderFeatureFailure, CylinderFeatureStage};
pub use history::{
    CylinderBodyRef, CylinderFeatureHistory, CylinderFeatureParent, CylinderFeatureRelation,
    CylinderFeatureSource,
};

#[cfg(test)]
mod tests;

/// Requested material change on the selected face.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CylinderFeatureKind {
    /// Join outward-extruded material to the support solid.
    Boss,
    /// Remove inward-extruded material from the support solid.
    Pocket,
}

/// Validated solid with immutable native bytes and complete supported history.
/// Example: `feature.export_native()?.body`.
pub struct CylinderFeatureResult {
    kind: CylinderFeatureKind,
    tool: CylinderTrimmedTool,
    archive: FeatureArchive,
}

struct FeatureArchive {
    bytes: Vec<u8>,
    validation: ValidationReport,
    history: CylinderFeatureHistory,
}

/// Independently decoded body and references into that body.
pub struct DetachedCylinderFeature {
    /// The accepted single solid.
    pub body: Shape,
    /// Archive-local references into this detached solid.
    pub entities: BTreeMap<CylinderBodyRef, Shape>,
}

impl CylinderFeatureResult {
    /// Completed material operation. Example: `feature.kind()`.
    pub fn kind(&self) -> CylinderFeatureKind {
        self.kind
    }

    /// Trim evidence, achieved extrusion bounds and original authored history.
    /// Example: `feature.tool().footprint().revision`.
    pub fn tool(&self) -> &CylinderTrimmedTool {
        &self.tool
    }

    /// Original and tool-parent relations into the frozen Boolean body.
    /// Example: `feature.history().images.len()`.
    pub fn history(&self) -> &CylinderFeatureHistory {
        &self.archive.history
    }

    /// Ordered native validation with its actual check modes. Example: `feature.validation().is_valid()`.
    pub fn validation(&self) -> &ValidationReport {
        &self.archive.validation
    }

    /// Frozen solid and its image references. Example: `feature.encoded_brep().len()`.
    pub fn encoded_brep(&self) -> &[u8] {
        &self.archive.bytes
    }

    /// Decode fresh native owners. Example: `let solid = feature.export_native()?.body;`.
    pub fn export_native(&self) -> Result<DetachedCylinderFeature, CylinderFeatureFailure> {
        let report = self.tool.footprint();
        let attempt = Attempt {
            attachment: report.attachment,
            revision: report.revision,
            kind: self.kind,
        };
        self.archive.export(attempt)
    }
}

impl FeatureArchive {
    fn export(&self, attempt: Attempt) -> FeatureChecked<DetachedCylinderFeature> {
        body::decode(
            &self.bytes,
            self.history.images.keys().copied().collect(),
            attempt,
        )
    }
}

impl CylinderFaceAttachment {
    /// Apply a wrapped boss or pocket on a verified rectangular cylindrical band.
    /// Depths start at zero and extend in the requested normal sense from the selected face.
    /// Example: `attachment.apply_wrapped_profile(revision, CylinderFeatureKind::Pocket, &request)?`.
    pub fn apply_wrapped_profile(
        &self,
        revision: SupportRevision,
        kind: CylinderFeatureKind,
        request: &WrappedNormalRequest,
    ) -> Result<CylinderFeatureResult, CylinderFeatureFailure> {
        self.apply_wrapped_profile_with_cancel(revision, kind, request, || false)
    }

    /// Poll during tool construction and at native operation boundaries before publication.
    /// Example: `attachment.apply_wrapped_profile_with_cancel(revision, kind, &request, || stop())?`.
    pub fn apply_wrapped_profile_with_cancel(
        &self,
        revision: SupportRevision,
        kind: CylinderFeatureKind,
        request: &WrappedNormalRequest,
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> Result<CylinderFeatureResult, CylinderFeatureFailure> {
        let attempt = Attempt {
            attachment: self.record().id,
            revision,
            kind,
        };
        attempt.poll(&mut cancelled, CylinderFeatureStage::Admission)?;
        let (support, mut band) = self
            .resolve_band(revision, request)
            .map_err(|cause| attempt.footprint(cause))?;
        let solid = admit(
            attempt,
            &support.body,
            chart_orientation(support.face.orientation(), band.reflect_axial),
            request.depths,
            request.support.normal,
        )?;
        let (tool, graph) = self
            .prepare_trimmed_tool(revision, request, &mut band, &mut cancelled)
            .map_err(|cause| attempt.footprint(cause))?;
        let native = tool
            .extrusion()
            .attachment_operand(graph)
            .map_err(|cause| attempt.tool(cause))?;
        let archive = apply_material(
            attempt,
            &support.face,
            &solid,
            native,
            &self.record().resolution,
            &mut cancelled,
        )?;
        Ok(CylinderFeatureResult {
            kind,
            tool,
            archive,
        })
    }
}

fn chart_orientation(native: Orientation, reflect_axial: bool) -> Orientation {
    match (native, reflect_axial) {
        (Orientation::Forward, true) => Orientation::Reversed,
        (Orientation::Reversed, true) => Orientation::Forward,
        _ => native,
    }
}

fn admit(
    attempt: Attempt,
    support: &Shape,
    orientation: Orientation,
    depths: [f64; 2],
    normal: NormalSense,
) -> FeatureChecked<Shape> {
    if depths[0] != 0. || !depths[1].is_finite() || depths[1] <= 0. {
        return Err(attempt.fail(
            CylinderFeatureStage::Admission,
            CylinderFeatureCause::Depths(depths),
        ));
    }
    let expected = normal_sense(attempt, orientation)?;
    if normal != expected {
        return Err(attempt.fail(
            CylinderFeatureStage::Admission,
            CylinderFeatureCause::Normal {
                actual: normal,
                expected,
            },
        ));
    }
    body::single_solid(attempt, support, CylinderFeatureStage::Admission)
}

fn apply_material(
    attempt: Attempt,
    selected_face: &Face,
    support: &Shape,
    tool: DetachedWrappedBodies,
    resolution: &martensite_tkbo::profile_extrusion::Resolution,
    cancelled: &mut CancelPoll<'_>,
) -> FeatureChecked<FeatureArchive> {
    let boolean = boolean(
        attempt,
        support,
        &tool.bodies,
        FEATURE_BOOLEAN_PARALLEL,
        cancelled,
    )?;
    let captured = history::capture(attempt, selected_face, support, &tool, &boolean, cancelled)?;
    // The captured solid and history own their bindings. Publication can reuse the workspace's memory.
    drop((boolean, tool));
    let (bytes, validation) = body::freeze(attempt, &captured, resolution, cancelled)?;
    Ok(FeatureArchive {
        bytes,
        validation,
        history: captured.history,
    })
}

fn normal_sense(attempt: Attempt, orientation: Orientation) -> FeatureChecked<NormalSense> {
    use CylinderFeatureKind::{Boss, Pocket};
    match (attempt.kind, orientation) {
        (Boss, Orientation::Forward) | (Pocket, Orientation::Reversed) => Ok(NormalSense::Outward),
        (Pocket, Orientation::Forward) | (Boss, Orientation::Reversed) => Ok(NormalSense::Inward),
        _ => Err(attempt.fail(
            CylinderFeatureStage::Admission,
            CylinderFeatureCause::FaceOrientation(orientation),
        )),
    }
}

// The Fuse or Cut tasks share one intersection context (#496). On the 239-curve conical text
// boss the parallel Boolean cut the operation's time by about 17% with unchanged peak memory,
// and the published archive was identical byte for byte to the serial one.
const FEATURE_BOOLEAN_PARALLEL: bool = true;

fn boolean(
    attempt: Attempt,
    support: &Shape,
    tool: &Shape,
    parallel: bool,
    cancelled: &mut CancelPoll<'_>,
) -> FeatureChecked<BRepAlgoAPIBooleanOperation> {
    let stage = CylinderFeatureStage::Boolean;
    attempt.poll(cancelled, stage)?;
    let mut boolean = BRepAlgoAPIBooleanOperation::new();
    boolean.set_arguments(&List::from_iter([support.clone()]));
    boolean.set_tools(&List::from_iter([tool.clone()]));
    boolean.set_operation(match attempt.kind {
        CylinderFeatureKind::Boss => Operation::Fuse,
        CylinderFeatureKind::Pocket => Operation::Cut,
    });
    boolean.set_non_destructive(true);
    boolean.set_to_fill_history(true);
    boolean.set_run_parallel(parallel);
    let completion = boolean.build_with_progress(&ProgressRange::default());
    accept_boolean(attempt, &boolean, completion)?;
    attempt.poll(cancelled, stage)?;
    Ok(boolean)
}

fn accept_boolean(
    attempt: Attempt,
    boolean: &BRepAlgoAPIBooleanOperation,
    completion: ProgressCompletion,
) -> FeatureChecked<()> {
    if completion == ProgressCompletion::Completed && boolean.is_done() && !boolean.has_errors() {
        return Ok(());
    }
    let mut errors = Vec::new();
    boolean.dump_errors(&mut errors).map_err(|cause| {
        attempt.fail(
            CylinderFeatureStage::Boolean,
            CylinderFeatureCause::Diagnostic(cause),
        )
    })?;
    Err(attempt.fail(
        CylinderFeatureStage::Boolean,
        CylinderFeatureCause::Boolean {
            completion,
            errors: String::from_utf8_lossy(&errors).into_owned(),
        },
    ))
}
