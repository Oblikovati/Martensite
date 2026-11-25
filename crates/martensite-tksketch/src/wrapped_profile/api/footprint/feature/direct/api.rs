//! Admission for the explicit conical-frustum attachment behavior.
use super::*;
use crate::surface_sketch::attachment::ConeFaceAttachment;

impl ConeFaceAttachment {
    /// Apply an exact analytic boss or pocket to an admitted full conical frustum.
    ///
    /// Uses authored planar cycles and the family chart. The result exposes the
    /// committed solid, geometry bounds and history. The standalone tool and
    /// its volume comparison are available through the placed-tool operation.
    ///
    /// Example: `attachment.apply_certified_wrapped_profile(revision, seam, kind, &request, selection)?`.
    pub fn apply_certified_wrapped_profile(
        &self,
        revision: SupportRevision,
        native_seam: f64,
        kind: CylinderFeatureKind,
        request: &WrappedConeNormalRequest,
        selection: WrappedConeExecutionSelection,
    ) -> FeatureChecked<CertifiedConeFeatureResult> {
        self.apply_certified_wrapped_profile_with_cancel(
            revision,
            native_seam,
            kind,
            request,
            selection,
            || false,
        )
    }

    /// Poll the caller while building and qualifying the attached result.
    ///
    /// Example: `attachment.apply_certified_wrapped_profile_with_cancel(revision, seam, kind, &request, selection, || stop())?`.
    pub fn apply_certified_wrapped_profile_with_cancel(
        &self,
        revision: SupportRevision,
        native_seam: f64,
        kind: CylinderFeatureKind,
        request: &WrappedConeNormalRequest,
        selection: WrappedConeExecutionSelection,
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> FeatureChecked<CertifiedConeFeatureResult> {
        let attempt = Attempt {
            attachment: self.record().id,
            revision,
            kind,
        };
        attempt.poll(&mut cancelled, CylinderFeatureStage::Admission)?;
        direct_policy(attempt, request, selection)?;
        let (chart, support) = self
            .chart_and_body(revision, native_seam)
            .map_err(|cause| attempt.cone_footprint(cause.into()))?;
        let stock = admit(
            attempt,
            &support,
            chart.radial_orientation(),
            request.depths,
            request.support.normal,
        )?;
        let tool = chart
            .materialize_attachment_tool(revision, request, selection, &mut cancelled)
            .map_err(|cause| attempt.cone_footprint(cause))?;
        let retained = retained::Retained::capture_attachment(&tool)
            .map_err(|cause| cause.feature(attempt))?;
        let (bytes, history, mut report) = publication::build(
            Inputs {
                attempt,
                stock: &stock,
                cone: chart.trim_face().0,
                tool: tool.native(),
                retained: &retained,
                records: tool.history(),
                regions: tool.regions(),
                resolution: &self.record().resolution,
            },
            &mut cancelled,
        )?;
        let work = tool.work();
        report.authored_pairs = work.authored.pair_domains;
        report.cap_pairs = work.caps.values().map(|member| member.pair_domains).sum();
        report.pair_bisections = work.planar.consumed_pair_bisections;
        report.integration_cells = work.integration_cells;
        report.arithmetic_operations = work.arithmetic_operations;
        report.family_operations = work.actual_operations;
        Ok(CertifiedConeFeatureResult {
            attempt,
            bytes,
            history,
            report,
            authored: tool.history().to_vec(),
            spatial_budget: tool.spatial_budget().clone(),
        })
    }
}

fn direct_policy(
    attempt: Attempt,
    request: &WrappedConeNormalRequest,
    selection: WrappedConeExecutionSelection,
) -> FeatureChecked<()> {
    if !matches!(
        selection,
        WrappedConeExecutionSelection::DirectedTransportedAuthoredPlanarAndCapCyclesV9 { .. }
    ) || request.options.volume != WrappedVolumePolicy::Report
    {
        return Err(attempt.fail(
            CylinderFeatureStage::Admission,
            CylinderFeatureCause::DirectAttachment {
                expected: "the V9 family chart selector and Report volume policy",
            },
        ));
    }
    Ok(())
}
