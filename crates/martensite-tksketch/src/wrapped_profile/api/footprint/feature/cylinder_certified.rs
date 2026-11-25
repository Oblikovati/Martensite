//! Explicit source-bound publication of one selected cylindrical material operation.
use super::*;
use crate::surface_sketch::{HistorySource, WrappedHistoryRecord};
use martensite_tkbo::profile_extrusion::PlanarProofVersion;
use std::num::NonZeroUsize;

mod assembly;
mod material;
mod proof;
mod refusal;
#[cfg(test)]
mod tests;
mod work;

type Checked<T> = FeatureChecked<T>;

const DEFAULT_WORKER_LIMIT: NonZeroUsize = NonZeroUsize::MIN.saturating_add(11);

#[derive(Clone, Copy)]
struct Execution {
    attempt: Attempt,
    worker_limit: NonZeroUsize,
}

impl Execution {
    fn new(attempt: Attempt, worker_limit: NonZeroUsize) -> Self {
        Self {
            attempt,
            worker_limit: worker_limit.min(DEFAULT_WORKER_LIMIT),
        }
    }
}

fn require(
    attempt: Attempt,
    stage: CylinderFeatureStage,
    condition: bool,
    expected: &'static str,
) -> Checked<()> {
    if condition {
        return Ok(());
    }
    Err(attempt.fail(stage, CylinderFeatureCause::DirectAttachment { expected }))
}

fn missing(
    attempt: Attempt,
    stage: CylinderFeatureStage,
    expected: &'static str,
) -> CylinderFeatureFailure {
    attempt.fail(stage, CylinderFeatureCause::DirectAttachment { expected })
}

fn certify<T, E: refusal::ProofFailure>(
    attempt: Attempt,
    stage: CylinderFeatureStage,
    query: &'static str,
    entities: impl IntoIterator<Item = HistorySource>,
    result: Result<T, E>,
) -> Checked<T> {
    result.map_err(|cause| refusal::project(attempt, stage, query, entities, cause, false))
}

fn certify_cancellable<T, E: refusal::ProofFailure>(
    attempt: Attempt,
    stage: CylinderFeatureStage,
    query: &'static str,
    entities: impl IntoIterator<Item = HistorySource>,
    cancelled: &mut CancelPoll<'_>,
    operation: impl FnOnce(&mut CancelPoll<'_>) -> Result<T, E>,
) -> Checked<T> {
    let (result, observed) = observe_cancellation(cancelled, operation);
    match result {
        Err(cause) => Err(refusal::project(
            attempt, stage, query, entities, cause, observed,
        )),
        Ok(_) if observed => Err(attempt.fail(stage, CylinderFeatureCause::Cancelled)),
        Ok(value) => Ok(value),
    }
}

fn observe_cancellation<T>(
    cancelled: &mut CancelPoll<'_>,
    operation: impl FnOnce(&mut CancelPoll<'_>) -> T,
) -> (T, bool) {
    let mut observed = false;
    let result = operation(&mut || {
        observed = observed || cancelled();
        observed
    });
    (result, observed)
}

#[derive(Clone, Copy, Debug)]
enum Phase {
    Admission,
    AuthoredPlan,
    ToolStorage,
    ToolFamily,
    Archive,
    OrderedGeometry,
    ExactContact,
    LateTolerance,
}

/// Final solid, directed authored lineage, and ordered validation for a certified cylinder feature.
/// Example: `let solid = feature.export_native()?.body;`.
pub struct CertifiedCylinderFeatureResult {
    attempt: Attempt,
    archive: FeatureArchive,
    authored: Vec<WrappedHistoryRecord>,
    coverage: CertifiedCylinderAttachmentReport,
}

/// Certified spatial bounds on the actual decoded attachment geometry.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct CertifiedCylinderAttachmentReport {
    /// Requested authored tool spatial error in model units.
    pub requested_spatial_error: f64,
    /// Unchanged sewing limit for attaching the actual stock traces.
    pub requested_sew_limit: f64,
    /// Effective planning ceiling, after accounting for the canonical fit allocation and sew.
    pub effective_fit_ceiling: f64,
    /// Maximum certified developed discrepancy between authored and stored reference profiles.
    pub authored_fit_upper: f64,
    /// Maximum certified spatial discrepancy of the retained normal sweep under its depth budgets.
    pub normal_storage_upper: f64,
    /// Full-domain upper error on the ruled walls and both physical cap boundaries.
    pub authored_to_stored_upper: f64,
    /// Full-interval upper residual of the attached stock pcurves.
    pub seam_residual_upper: f64,
    /// Full-interval upper residual of the exposed cap pcurves.
    pub exposed_cap_residual_upper: f64,
    /// Bound on the original or re-cut stock boundary carrier discrepancy.
    pub stock_boundary_residual_upper: f64,
    /// Number of actual ruled faces covered on the committed body.
    pub side_faces: usize,
}

impl CertifiedCylinderFeatureResult {
    /// Completed material operation. Example: `feature.kind()`.
    pub fn kind(&self) -> CylinderFeatureKind {
        self.attempt.kind
    }

    /// Parents of the final body members. Example: `feature.history().images.len()`.
    pub fn history(&self) -> &CylinderFeatureHistory {
        &self.archive.history
    }

    /// Original sketch lineage of the tool images. Example: `feature.authored_history().len()`.
    pub fn authored_history(&self) -> &[WrappedHistoryRecord] {
        &self.authored
    }

    /// Coverage of the final decoded solid. Example: `feature.validation().is_valid()`.
    pub fn validation(&self) -> &ValidationReport {
        &self.archive.validation
    }

    /// Qualified spatial bounds. Example: `feature.attachment_report().seam_residual_upper`.
    pub fn attachment_report(&self) -> &CertifiedCylinderAttachmentReport {
        &self.coverage
    }

    /// Immutable encoded final solid and semantic references. Example: `feature.encoded_brep().len()`.
    pub fn encoded_brep(&self) -> &[u8] {
        &self.archive.bytes
    }

    /// Decode independently owned native geometry. Example: `let native = feature.export_native()?;`.
    pub fn export_native(&self) -> Checked<DetachedCylinderFeature> {
        self.archive.export(self.attempt)
    }
}

impl CylinderFaceAttachment {
    /// Apply a source-bound boss or pocket to an admitted circular cylinder.
    /// The explicit V6 choice selects the authored planar proof and its work limits.
    /// Example: `attachment.apply_certified_wrapped_profile(revision, kind, &request, proof)?`.
    pub fn apply_certified_wrapped_profile(
        &self,
        revision: SupportRevision,
        kind: CylinderFeatureKind,
        request: &WrappedNormalRequest,
        planar: PlanarProofVersion,
    ) -> Checked<CertifiedCylinderFeatureResult> {
        self.apply_certified_wrapped_profile_with_cancel(revision, kind, request, planar, || false)
    }

    /// Poll at proof, materialization, and final publication boundaries.
    /// Example: `attachment.apply_certified_wrapped_profile_with_cancel(revision, kind, &request, proof, || stop())?`.
    pub fn apply_certified_wrapped_profile_with_cancel(
        &self,
        revision: SupportRevision,
        kind: CylinderFeatureKind,
        request: &WrappedNormalRequest,
        planar: PlanarProofVersion,
        cancelled: impl FnMut() -> bool + Send,
    ) -> Checked<CertifiedCylinderFeatureResult> {
        self.apply_certified_wrapped_profile_with_worker_limit(
            revision,
            kind,
            request,
            planar,
            DEFAULT_WORKER_LIMIT,
            cancelled,
        )
    }

    /// Apply the certified feature with an upper bound on operation-local certificate workers.
    /// A limit of one runs the certificates on the calling thread. Output and
    /// diagnostic order follow the same canonical source keys for every limit.
    /// Example: `attachment.apply_certified_wrapped_profile_with_worker_limit(revision, kind, &request, proof, std::num::NonZeroUsize::MIN, || false)?`.
    pub fn apply_certified_wrapped_profile_with_worker_limit(
        &self,
        revision: SupportRevision,
        kind: CylinderFeatureKind,
        request: &WrappedNormalRequest,
        planar: PlanarProofVersion,
        worker_limit: NonZeroUsize,
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> Checked<CertifiedCylinderFeatureResult> {
        let attempt = Attempt {
            attachment: self.record().id,
            revision,
            kind,
        };
        let execution = Execution::new(attempt, worker_limit);
        execute(self, execution, request, planar, &mut cancelled, |_| {})
    }
}

fn execute(
    attachment: &CylinderFaceAttachment,
    execution: Execution,
    request: &WrappedNormalRequest,
    planar: PlanarProofVersion,
    cancelled: &mut CancelPoll<'_>,
    mut complete: impl FnMut(Phase),
) -> Checked<CertifiedCylinderFeatureResult> {
    let attempt = execution.attempt;
    let input = material::admit(attachment, attempt, request, planar, cancelled)?;
    complete(Phase::Admission);
    let tool = material::prepare(execution, request, &input, planar, cancelled, &mut complete)?;
    let candidate = assembly::build(attempt, &input, &tool, cancelled)?;
    complete(Phase::Archive);
    let (validation, spatial) = proof::validate(
        execution,
        &input,
        &tool,
        &candidate,
        cancelled,
        &mut complete,
    )?;
    let authored_to_stored_upper = realized_boundary_bound(&tool, input.support.radius);
    require(
        attempt,
        CylinderFeatureStage::Validation,
        authored_to_stored_upper.is_finite()
            && authored_to_stored_upper <= request.options.spatial_error,
        "same-parameter authored-to-stored wall and cap boundary error within the requested spatial budget",
    )?;
    let coverage = CertifiedCylinderAttachmentReport {
        requested_spatial_error: request.options.spatial_error,
        requested_sew_limit: input.resolution.sew,
        effective_fit_ceiling: input.fit_ceiling(),
        authored_fit_upper: tool.authored_fit_upper(),
        normal_storage_upper: tool.normal_storage_upper(),
        authored_to_stored_upper,
        seam_residual_upper: spatial[0],
        exposed_cap_residual_upper: spatial[1],
        stock_boundary_residual_upper: candidate.native_stock_error,
        side_faces: tool.spans.len(),
    };
    attempt.poll(cancelled, CylinderFeatureStage::Publication)?;
    let archive = FeatureArchive {
        bytes: candidate.bytes,
        validation,
        history: candidate.history,
    };
    Ok(CertifiedCylinderFeatureResult {
        attempt,
        archive,
        authored: tool.authored,
        coverage,
    })
}

fn realized_boundary_bound(tool: &material::Tool, support_radius: f64) -> f64 {
    tool.spans
        .values()
        .map(|span| {
            let stored = &span.storage;
            let nominal = stored.certificate.stored_radii()[1];
            let outer = (nominal + stored.depths.radius_rounding_bounds()[1]).next_up();
            let developed = stored.profile.developed_position_bound();
            // The cylindrical lift scales the chart angle by r/R. The depth
            // receipt binds the additional discrepancy of the stored ruled wall.
            let scale = (outer / support_radius).next_up().max(1.);
            let boundary = (developed * scale).next_up();
            let wall = (boundary + stored.depths.normal_sweep_error_bound()).next_up();
            let cap = stored
                .trims
                .cap_residuals
                .iter()
                .map(|proof| proof.spatial_residual_bound())
                .fold(0_f64, f64::max);
            let combined = (wall + cap).next_up();
            if combined.is_finite() {
                combined
            } else {
                f64::INFINITY
            }
        })
        .fold(0_f64, f64::max)
}

const _: fn() = || {
    fn shared<T: Send + Sync>() {}
    shared::<CertifiedCylinderFeatureResult>();
    shared::<CertifiedCylinderAttachmentReport>();
};
