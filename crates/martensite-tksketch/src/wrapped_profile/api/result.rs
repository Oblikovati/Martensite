//! Published metadata and proof inputs are owned; native geometry is decoded only on export.
use super::*;
use std::collections::BTreeMap;
/// Fresh native representation with its authored keys, detached from every other export.
/// Native mutation follows the underlying toolkit's native-handle synchronization rules.
pub struct DetachedWrappedBodies {
    /// Compound transport root containing every selected solid.
    pub bodies: Shape,
    /// References into these detached bodies, keyed by original authored identity.
    pub entities: BTreeMap<WrappedHistoryKey, Shape>,
}
/// Immutable normal extrusion of selected profiles, with frozen bytes and retained evidence.
pub struct WrappedNormalResult {
    pub(in crate::surface_sketch::wrapped_profile) family: family::BudgetedFamily,
    pub(in crate::surface_sketch::wrapped_profile) history: history_projection::HistoryPublication,
    pub(in crate::surface_sketch::wrapped_profile) reports:
        BTreeMap<SurfaceRegionId, WrappedMemberReport>,
    pub(in crate::surface_sketch::wrapped_profile) budget: WrappedBudgetReport,
    pub(in crate::surface_sketch::wrapped_profile) support: crate::surface_sketch::CylinderSupport,
    pub(in crate::surface_sketch::wrapped_profile) depths: [f64; 2],
    pub(in crate::surface_sketch::wrapped_profile) options: WrappedNormalOptions,
    pub(in crate::surface_sketch::wrapped_profile) placement: Option<Box<placement::Placed>>,
}
impl WrappedNormalResult {
    /// Ordered authored history; `result.history().len()`.
    pub fn history(&self) -> &[WrappedHistoryRecord] {
        &self.history.records
    }
    /// Per-region coverage and achieved errors; `result.reports().len()`.
    pub fn reports(&self) -> &BTreeMap<SurfaceRegionId, WrappedMemberReport> {
        &self.reports
    }
    /// Actual allocations and family bounds; `result.budget().storage_allowance`.
    pub fn budget(&self) -> &WrappedBudgetReport {
        &self.budget
    }
    /// Requested analytic support; `result.support().radius`.
    pub fn support(&self) -> crate::surface_sketch::CylinderSupport {
        self.support
    }
    /// Requested ordered signed normal depths; `result.depths()`.
    pub fn depths(&self) -> [f64; 2] {
        self.depths
    }
    /// Original options, with no silent clamps; `result.options().maximum_fit_level`.
    pub fn options(&self) -> WrappedNormalOptions {
        self.options
    }
    /// Read-only retained native encoding; `result.encoded_brep().len()`.
    pub fn encoded_brep(&self) -> &[u8] {
        self.placement
            .as_ref()
            .map_or(&self.family.family.bytes, |placed| &placed.bytes)
    }
    /// Physical placement evidence, absent for the exact canonical fast path; `result.placement()`.
    pub fn placement(&self) -> Option<&WrappedPlacementReport> {
        self.placement.as_ref().map(|placed| &placed.report)
    }
    /// Origin of `F(p)=(p-origin)/3` for every reported boundary flux.
    /// Uses the decoded placement translation, or zero for the canonical graph.
    /// Example: `let origin = result.boundary_flux_origin();`.
    pub fn boundary_flux_origin(&self) -> [f64; 3] {
        self.placement()
            .map_or([0.; 3], |placed| placed.spatial_budget.input().translation)
    }
    /// Fresh native owners with keyed references; `let detached = result.export_native()?;`.
    pub fn export_native(&self) -> Checked<DetachedWrappedBodies> {
        export_bytes(self, self.encoded_brep())
    }
}
pub(super) fn export_bytes(
    result: &WrappedNormalResult,
    bytes: &[u8],
) -> Checked<DetachedWrappedBodies> {
    let graph = pipeline::decode(bytes, &result.family.family.keys)
        .map_err(|cause| export_failure(error::Cause::Export(cause)))?;
    export_graph(result, graph)
}
pub(super) fn export_graph(
    result: &WrappedNormalResult,
    (bodies, bindings): super::super::frozen_placement::DecodedGraph,
) -> Checked<DetachedWrappedBodies> {
    let mut entities = BTreeMap::new();
    export_bindings::append(
        &result.family.family.keys,
        &result.history.bindings,
        bindings,
        &mut entities,
    )
    .map_err(|cause| export_failure(error::Cause::ExportBindings(cause)))?;
    Ok(DetachedWrappedBodies { bodies, entities })
}
fn export_failure(cause: error::Cause) -> WrappedNormalFailure {
    let mut failure = error::failure(
        WrappedStage::Export,
        WrappedFailureClass::Native,
        None,
        cause,
    );
    failure.operation = WrappedOperation::DetachedExport;
    failure
}
