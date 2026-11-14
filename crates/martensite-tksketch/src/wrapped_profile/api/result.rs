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
    pub(super) family: family::BudgetedFamily,
    pub(super) history: history_projection::HistoryPublication,
    pub(super) reports: BTreeMap<SurfaceRegionId, WrappedMemberReport>,
    pub(super) budget: WrappedBudgetReport,
    pub(super) support: crate::surface_sketch::CylinderSupport,
    pub(super) depths: [f64; 2],
    pub(super) options: WrappedNormalOptions,
    pub(super) placement: Option<Box<placement::Placed>>,
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
    /// Fresh native owners with keyed references; `let detached = result.export_native()?;`.
    pub fn export_native(&self) -> Checked<DetachedWrappedBodies> {
        let (bodies, bindings) = pipeline::decode(self.encoded_brep(), &self.family.family.keys)
            .map_err(|cause| export_failure(error::Cause::Export(cause)))?;
        if bindings.len() != self.history.bindings.len() {
            return Err(export_failure(error::Cause::ExportCount {
                actual: bindings.len(),
                expected: self.history.bindings.len(),
            }));
        }
        let entities = self
            .history
            .bindings
            .iter()
            .copied()
            .zip(bindings.into_iter().map(|(_, shape)| shape))
            .collect();
        Ok(DetachedWrappedBodies { bodies, entities })
    }
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
