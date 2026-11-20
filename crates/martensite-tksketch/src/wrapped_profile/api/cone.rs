//! Polynomial developed selections on an analytic cone share the wrapped operation executor.
use super::*;
use crate::surface_sketch::ConeSupport;
use std::collections::BTreeMap;
mod reports;
pub use reports::*;
mod arithmetic_grant;
pub use arithmetic_grant::*;
mod proof;
pub use proof::*;
mod domain_policy;
pub use domain_policy::*;
mod execution_selection;
pub use execution_selection::*;
/// Owned cone-normal extrusion input; `normal_extrude_cone_profile(&request)?`.
#[derive(Clone, Debug, PartialEq)]
pub struct WrappedConeNormalRequest {
    /// Solved world-XY developed Cartesian coordinates, measured from the cone apex.
    pub profile: martensite_tkbo::profile_extrusion::SketchProfile,
    /// Explicit selected outer boundaries and holes.
    pub selections: Vec<WrappedRegionSelection>,
    /// Original cone and physical frame.
    pub support: ConeSupport,
    /// Increasing caller-relative signed normal depths.
    pub depths: [f64; 2],
    /// Original spatial, finite work, fit-level and measure policies.
    pub options: WrappedNormalOptions,
    /// Original finite arithmetic, integration and authored-volume policies.
    pub proof: WrappedConeProofOptions,
}
/// Complete immutable cone result; `result.export_native()?` returns independent native owners.
pub struct WrappedConeNormalResult {
    pub(in crate::surface_sketch::wrapped_profile) family: operation::cone::ConeBudgeted,
    pub(in crate::surface_sketch::wrapped_profile) history: history_projection::HistoryPublication,
    pub(in crate::surface_sketch::wrapped_profile) reports:
        BTreeMap<SurfaceRegionId, WrappedConeMemberReport>,
    pub(in crate::surface_sketch::wrapped_profile) budget: WrappedConeBudgetReport,
    pub(in crate::surface_sketch::wrapped_profile) support: ConeSupport,
    pub(in crate::surface_sketch::wrapped_profile) depths: [f64; 2],
    pub(in crate::surface_sketch::wrapped_profile) options: WrappedNormalOptions,
    pub(in crate::surface_sketch::wrapped_profile) proof: WrappedConeProofOptions,
    pub(in crate::surface_sketch::wrapped_profile) arithmetic_grant:
        Option<operation::cone::input::grant::CapturedGrant>,
}
impl WrappedConeNormalResult {
    /// Caller-relative derived/split/created lineage; `result.history().len()`.
    pub fn history(&self) -> &[WrappedHistoryRecord] {
        &self.history.records
    }
    /// Counted incidence, native modes and certified bounds; `result.reports().len()`.
    pub fn reports(&self) -> &BTreeMap<SurfaceRegionId, WrappedConeMemberReport> {
        &self.reports
    }
    /// Original policies and achieved outward bounds; `result.budget().achieved_spatial_error`.
    pub fn budget(&self) -> &WrappedConeBudgetReport {
        &self.budget
    }
    /// Original support; `result.support().semi_angle`.
    pub fn support(&self) -> ConeSupport {
        self.support
    }
    /// Original caller-relative depth order; `result.depths()`.
    pub fn depths(&self) -> [f64; 2] {
        self.depths
    }
    /// Requested limits, without replacement by achieved bounds; `result.options().spatial_error`.
    pub fn options(&self) -> WrappedNormalOptions {
        self.options
    }
    /// Original proof limits; leaf receipts retain their effective settings and consumed work.
    /// Example: `assert!(result.proof_options().arithmetic.max_operations > 0);`.
    pub fn proof_options(&self) -> WrappedConeProofOptions {
        self.proof
    }
    /// Frozen keyed body archive; `result.encoded_brep().len()`.
    pub fn encoded_brep(&self) -> &[u8] {
        &self.family.family.bytes
    }
    /// Origin of the centered field used for every reported conditional boundary flux.
    /// Canonical Phase A supports use the apex; `let origin = result.boundary_flux_origin();`.
    pub fn boundary_flux_origin(&self) -> [f64; 3] {
        self.support.apex
    }
    /// Executed whole-orbit and face integral programs, present only for explicit V5.
    /// Example: `result.planar_integral_receipt(region).map(|proof| proof.entry_allowance())`.
    pub fn planar_integral_receipt(
        &self,
        region: SurfaceRegionId,
    ) -> Option<&martensite_tkbo::profile_extrusion::PlanarIntegralEvidence> {
        self.family
            .family
            .members
            .get(&region)?
            .geometry
            .material
            .integral_program_evidence()
    }
    /// Original, effective and consumed integration work; `result.flux_receipt(region)`.
    pub fn flux_receipt(
        &self,
        region: SurfaceRegionId,
    ) -> Option<&martensite_tkmath::predicates::roots::metric_bounds::CertifiedAnalyticConicalFlux>
    {
        self.family
            .family
            .members
            .get(&region)
            .map(|member| &member.geometry.flux)
    }
    /// Source-bound authored normal-volume arithmetic; `result.authored_receipt(region)`.
    pub fn authored_receipt(
        &self,
        region: SurfaceRegionId,
    ) -> Option<&martensite_tkmath::predicates::roots::metric_bounds::ConicalAuthoredFlux> {
        self.family
            .family
            .members
            .get(&region)
            .map(|member| &member.geometry.authored)
    }
    /// Executed reference topology, filled-body and storage composition; `result.reference_body_receipt()`.
    pub fn reference_body_receipt(
        &self,
    ) -> &martensite_tkmath::predicates::roots::metric_bounds::ConicalReferenceEvidence {
        &self.family.budget.reference
    }
    /// Source-bound reference-volume integration settings and actual work; `result.reference_volume_receipt(region)`.
    pub fn reference_volume_receipt(
        &self,
        region: SurfaceRegionId,
    ) -> Option<&martensite_tkmath::predicates::roots::metric_bounds::CertifiedConicalReferenceVolume>
    {
        self.family
            .family
            .members
            .get(&region)
            .map(|member| &member.geometry.reference_volume)
    }
    /// Independent keyed native owners; `let detached = result.export_native()?;`.
    pub fn export_native(&self) -> Checked<DetachedWrappedBodies> {
        self.export_bytes(self.encoded_brep())
    }
    pub(super) fn export_bytes(&self, bytes: &[u8]) -> Checked<DetachedWrappedBodies> {
        let (bodies, bindings) =
            pipeline::decode(bytes, &self.family.family.keys).map_err(|cause| {
                export_failure(WrappedFailureClass::Native, error::Cause::Export(cause))
            })?;
        let entities = self.export_entities(bindings)?;
        Ok(DetachedWrappedBodies { bodies, entities })
    }
    fn export_entities(
        &self,
        bindings: Vec<(BindingKey, Shape)>,
    ) -> Checked<BTreeMap<WrappedHistoryKey, Shape>> {
        let mut entities = BTreeMap::new();
        export_bindings::append(
            &self.family.family.keys,
            &self.history.bindings,
            bindings,
            &mut entities,
        )
        .map_err(export_bindings_failure)?;
        Ok(entities)
    }
}
fn export_bindings_failure(cause: export_bindings::ExportBindingFailure) -> WrappedNormalFailure {
    export_failure(
        WrappedFailureClass::Incomplete,
        error::Cause::ExportBindings(cause),
    )
}
fn export_failure(class: WrappedFailureClass, cause: error::Cause) -> WrappedNormalFailure {
    let mut failure = error::failure(WrappedStage::Export, class, None, cause);
    failure.operation = WrappedOperation::DetachedExport;
    failure
}
/// Extrude selected developed profiles along cone normals within one admitted common angular chart.
/// The current scope requires apex `[0., 0., 0.]`, axis `[0., 0., 1.]` and radial `[1., 0., 0.]`.
/// Other frames return `Unsupported` before source capture; chart admission and exhausted proof
/// limits return typed refusals. Example: `let result = normal_extrude_cone_profile(&request)?;`.
pub fn normal_extrude_cone_profile(
    request: &WrappedConeNormalRequest,
) -> Checked<WrappedConeNormalResult> {
    normal_extrude_cone_profile_with_cancel(request, || false)
}
/// Cooperatively cancel the canonical-frame, single-chart operation without returning partial geometry.
/// Applies the same admission and typed refusals as `normal_extrude_cone_profile`.
/// Example: `normal_extrude_cone_profile_with_cancel(&request, || stop())`.
pub fn normal_extrude_cone_profile_with_cancel(
    request: &WrappedConeNormalRequest,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Checked<WrappedConeNormalResult> {
    operation::cone::execute(request, &mut cancelled)
}
#[cfg(test)]
mod tests;
