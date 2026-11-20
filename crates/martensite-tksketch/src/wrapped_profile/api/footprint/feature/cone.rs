//! Publish a conical boss or pocket through the shared material and history stages.
use super::*;
use crate::surface_sketch::attachment::ConeFaceAttachment;

/// Material operation on an attached wrapped profile.
pub type WrappedFeatureKind = CylinderFeatureKind;
/// Shared Boolean, history, publication and support-specific tool diagnostics.
pub type WrappedFeatureFailure = CylinderFeatureFailure;
/// Support and authored parents of each published Boolean image.
pub type WrappedFeatureHistory = CylinderFeatureHistory;
/// Independently exported solid and its archive-local references.
pub type DetachedWrappedFeature = DetachedCylinderFeature;

/// A validated conical boss or pocket and its revision-bound extrusion tool.
pub struct ConeFeatureResult {
    kind: WrappedFeatureKind,
    tool: ConeTrimmedTool,
    archive: FeatureArchive,
}

impl ConeFeatureResult {
    /// Applied operation. Example: `feature.kind()`.
    pub fn kind(&self) -> WrappedFeatureKind {
        self.kind
    }

    /// Selected trim and extrusion proof. Example: `feature.tool().footprint()`.
    pub fn tool(&self) -> &ConeTrimmedTool {
        &self.tool
    }

    /// Composed native and authored history. Example: `feature.history().images.len()`.
    pub fn history(&self) -> &WrappedFeatureHistory {
        &self.archive.history
    }

    /// Ordered validation with its actual check modes. Example: `feature.validation().is_valid()`.
    pub fn validation(&self) -> &ValidationReport {
        &self.archive.validation
    }

    /// Frozen Boolean solid and image references. Example: `feature.encoded_brep().len()`.
    pub fn encoded_brep(&self) -> &[u8] {
        &self.archive.bytes
    }

    /// Decode an independent native solid. Example: `let solid = feature.export_native()?.body;`.
    pub fn export_native(&self) -> Result<DetachedWrappedFeature, WrappedFeatureFailure> {
        let footprint = self.tool.footprint();
        self.archive.export(Attempt {
            attachment: footprint.attachment,
            revision: footprint.revision,
            kind: self.kind,
        })
    }
}

impl ConeFaceAttachment {
    /// Apply a boss or pocket to the selected conical trim using the given native seam.
    /// The request starts at depth zero and uses the corresponding radial normal sense.
    /// Example: `attachment.apply_wrapped_profile(revision, seam, WrappedFeatureKind::Boss, &request, selection)?`.
    pub fn apply_wrapped_profile(
        &self,
        revision: SupportRevision,
        native_seam: f64,
        kind: WrappedFeatureKind,
        request: &WrappedConeNormalRequest,
        selection: WrappedConeExecutionSelection,
    ) -> Result<ConeFeatureResult, WrappedFeatureFailure> {
        self.apply_wrapped_profile_with_cancel(
            revision,
            native_seam,
            kind,
            request,
            selection,
            || false,
        )
    }

    /// Poll during proof work and between native material stages before returning an owned solid.
    /// Example: `attachment.apply_wrapped_profile_with_cancel(revision, seam, kind, &request, selection, || stop())?`.
    pub fn apply_wrapped_profile_with_cancel(
        &self,
        revision: SupportRevision,
        native_seam: f64,
        kind: WrappedFeatureKind,
        request: &WrappedConeNormalRequest,
        selection: WrappedConeExecutionSelection,
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> Result<ConeFeatureResult, WrappedFeatureFailure> {
        let attempt = Attempt {
            attachment: self.record().id,
            revision,
            kind,
        };
        attempt.poll(&mut cancelled, CylinderFeatureStage::Admission)?;
        let (chart, support) = self
            .chart_and_body(revision, native_seam)
            .map_err(|cause| attempt.cone_footprint(cause.into()))?;
        let solid = admit(
            attempt,
            &support,
            chart.radial_orientation(),
            request.depths,
            request.support.normal,
        )?;
        let tool = chart
            .extrude_within_trim_band_with_cancel(revision, request, selection, &mut cancelled)
            .map_err(|cause| attempt.cone_footprint(cause))?;
        let native = tool
            .extrusion()
            .export_native()
            .map_err(|cause| attempt.tool(cause))?;
        let archive = apply_material(
            attempt,
            chart.trim_face().0,
            &solid,
            native,
            &self.record().resolution,
            &mut cancelled,
        )?;
        Ok(ConeFeatureResult {
            kind,
            tool,
            archive,
        })
    }
}
