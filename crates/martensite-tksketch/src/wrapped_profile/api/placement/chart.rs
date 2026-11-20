//! Versioned chart placement retains independent rotation evidence without changing legacy results.
use super::*;
mod material;
use martensite_tkmath::predicates::roots::metric_bounds::CertifiedRotatedFrameSpatialBudget;
/// Physical graph and rotation-aware bounds for chart behavior version one.
pub struct WrappedChartNormalResult {
    base: WrappedNormalResult,
    bytes: Vec<u8>,
    placement: WrappedChartPlacementReport,
}
/// Certified placement relative to the exact requested chart angle.
#[derive(Clone, Debug, PartialEq)]
pub struct WrappedChartPlacementReport {
    /// Original limits, bounded rotation evidence and actual stored coefficients.
    pub spatial_budget: CertifiedRotatedFrameSpatialBudget,
    /// Achieved physical bounds for every authored region.
    pub members: BTreeMap<SurfaceRegionId, WrappedWorldSpatialReport>,
}
impl WrappedChartNormalResult {
    /// Selected replay behavior; `assert_eq!(result.behavior_version(), 1)`.
    pub fn behavior_version(&self) -> u32 {
        1
    }
    /// Authored lineage; `result.history().len()`.
    pub fn history(&self) -> &[WrappedHistoryRecord] {
        self.base.history()
    }
    /// Canonical bounds and physical native validation; `result.reports().len()`.
    pub fn reports(&self) -> &BTreeMap<SurfaceRegionId, WrappedMemberReport> {
        self.base.reports()
    }
    /// Retained family allocation; `result.budget().storage_allowance`.
    pub fn budget(&self) -> &WrappedBudgetReport {
        self.base.budget()
    }
    /// Original support and chart angle; `result.support().chart_origin`.
    pub fn support(&self) -> crate::surface_sketch::CylinderSupport {
        self.base.support()
    }
    /// Requested signed depths; `result.depths()`.
    pub fn depths(&self) -> [f64; 2] {
        self.base.depths()
    }
    /// Original policies; `result.options().spatial_error`.
    pub fn options(&self) -> WrappedNormalOptions {
        self.base.options()
    }
    /// Frozen physical graph; `result.encoded_brep().len()`.
    pub fn encoded_brep(&self) -> &[u8] {
        &self.bytes
    }
    /// Rotation-aware world evidence; `result.placement().members.len()`.
    pub fn placement(&self) -> &WrappedChartPlacementReport {
        &self.placement
    }
    /// Decoded translation used by the reported field `F(p)=(p-origin)/3`.
    /// Example: `let origin = result.boundary_flux_origin();`.
    pub fn boundary_flux_origin(&self) -> [f64; 3] {
        self.placement.spatial_budget.input().translation
    }
    /// Independent mutable native export; `let native = result.export_native()?;`.
    pub fn export_native(&self) -> Checked<DetachedWrappedBodies> {
        result::export_bytes(&self.base, &self.bytes)
    }

    pub(in crate::surface_sketch::wrapped_profile::api) fn fitted_sources(&self) -> &[Request] {
        &self.base.family.family.selection.prepared.prepared
    }
}
/// Extrude with a finite chart angle using behavior version one.
/// Example: `let result = normal_extrude_profile_on_chart(&request)?;`.
pub fn normal_extrude_profile_on_chart(
    request: &WrappedNormalRequest,
) -> Checked<WrappedChartNormalResult> {
    normal_extrude_profile_on_chart_with_cancel(request, || false)
}
/// Cancel at operation boundaries without publishing partial geometry.
/// Example: `normal_extrude_profile_on_chart_with_cancel(&request, || stop())`.
pub fn normal_extrude_profile_on_chart_with_cancel(
    request: &WrappedNormalRequest,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Checked<WrappedChartNormalResult> {
    input::poll(&mut cancelled, WrappedStage::Admission)?;
    let inputs = input::read_canonical_chart(request, &mut cancelled)?;
    from_canonical_inputs(request, inputs, &mut cancelled)
}
pub(in crate::surface_sketch::wrapped_profile::api) fn from_canonical_inputs(
    request: &WrappedNormalRequest,
    mut inputs: input::Inputs,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<WrappedChartNormalResult> {
    let plan = chart_admission::prepare(request, &inputs.requests)?;
    for source in &mut inputs.requests {
        source.tolerance = plan.budget.canonical_allowance();
    }
    let mut base = operation::from_inputs(request, inputs, cancelled)?;
    let physical = publish_chart(&mut base, plan, cancelled)?;
    input::poll(cancelled, WrappedStage::Publication)?;
    Ok(assemble(base, physical))
}
fn assemble(
    base: WrappedNormalResult,
    physical: publication::PhysicalPublication<CertifiedRotatedFrameSpatialBudget>,
) -> WrappedChartNormalResult {
    WrappedChartNormalResult {
        base,
        bytes: physical.bytes,
        placement: WrappedChartPlacementReport {
            spatial_budget: physical.budget,
            members: physical.members,
        },
    }
}

fn publish_chart(
    result: &mut WrappedNormalResult,
    plan: Plan<CertifiedRotatedFrameSpatialBudget>,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<publication::PhysicalPublication<CertifiedRotatedFrameSpatialBudget>> {
    if plan.transform.matrix().coefficients() == &[[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]
        && plan.transform.translation_part().coordinates() == &[0.; 3]
    {
        input::poll(cancelled, WrappedStage::Placement)?;
        let physical = freeze::unchanged(&result.family.family)?;
        return publication::finish(result, plan, physical, cancelled);
    }
    publication::apply(result, plan, cancelled)
}
#[cfg(test)]
#[path = "chart_tests.rs"]
mod tests;
