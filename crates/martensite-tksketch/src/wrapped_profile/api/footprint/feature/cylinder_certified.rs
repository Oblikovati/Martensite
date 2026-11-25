//! Explicit source-bound publication of one selected cylindrical material operation.
use super::*;
use crate::surface_sketch::WrappedHistoryRecord;
use martensite_tkbo::profile_extrusion::PlanarProofVersion;

mod assembly;
mod material;
mod proof;
#[cfg(test)]
mod tests;

type Checked<T> = FeatureChecked<T>;

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

fn certify<T, E: std::fmt::Debug>(
    attempt: Attempt,
    stage: CylinderFeatureStage,
    query: &'static str,
    result: Result<T, E>,
) -> Checked<T> {
    result.map_err(|cause| {
        attempt.fail(
            stage,
            CylinderFeatureCause::Native {
                query,
                cause: Box::new(Failure::new(&format!("{cause:?}"))),
            },
        )
    })
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
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> Checked<CertifiedCylinderFeatureResult> {
        let attempt = Attempt {
            attachment: self.record().id,
            revision,
            kind,
        };
        execute(self, attempt, request, planar, &mut cancelled, |_| {})
    }
}

fn execute(
    attachment: &CylinderFaceAttachment,
    attempt: Attempt,
    request: &WrappedNormalRequest,
    planar: PlanarProofVersion,
    cancelled: &mut CancelPoll<'_>,
    mut complete: impl FnMut(CylinderFeatureStage),
) -> Checked<CertifiedCylinderFeatureResult> {
    let input = material::admit(attachment, attempt, request, planar, cancelled)?;
    complete(CylinderFeatureStage::Admission);
    let tool = material::prepare(attempt, request, &input, planar, cancelled)?;
    complete(CylinderFeatureStage::Tool);
    let candidate = assembly::build(attempt, &input, &tool, cancelled)?;
    complete(CylinderFeatureStage::Boolean);
    let (validation, spatial) = proof::validate(attempt, &input, &tool, &candidate, cancelled)?;
    complete(CylinderFeatureStage::Validation);
    let coverage = CertifiedCylinderAttachmentReport {
        requested_spatial_error: request.options.spatial_error,
        requested_sew_limit: input.resolution.sew,
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

const _: fn() = || {
    fn shared<T: Send + Sync>() {}
    shared::<CertifiedCylinderFeatureResult>();
    shared::<CertifiedCylinderAttachmentReport>();
};
