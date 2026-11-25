//! One shared rigid datum transports the frozen graph; canonical evidence stays owned.
use super::*;
#[cfg(test)]
use martensite_tkmath::{gp::Point3, toploc::Location};
use martensite_tkmath::{
    gp::Transform3,
    predicates::roots::metric_bounds::{
        CertifiedFrameSpatialBudget, FrameSpatialInput, FrameTransportError, FrameTransportInput,
        certify_frame_transport,
    },
};
use std::collections::BTreeMap;
mod admission;
mod chart;
mod chart_admission;
mod cone;
pub(super) use chart::from_canonical_inputs;
pub use chart::*;
pub use cone::*;
pub(super) use cone::{attachment_tool, cone_on_chart};
mod budget;
use budget::{PlacementBudget, PlacementFrame};
mod error;
mod freeze;
mod measures;
mod publication;
mod trace;
pub(super) use admission::prepare;
pub(super) use error::PlacementFailure;
use error::{at, fail};
pub(in crate::surface_sketch::wrapped_profile) use publication::publish;
pub(in crate::surface_sketch::wrapped_profile) struct Plan<
    B: PlacementBudget = CertifiedFrameSpatialBudget,
> {
    transform: Transform3,
    budget: B,
}
pub(in crate::surface_sketch::wrapped_profile) struct Placed {
    pub bytes: Vec<u8>,
    pub report: WrappedPlacementReport,
}
/// Physical-frame evidence bound to the actual decoded location coefficients.
/// Bounds cover the located graph and direct location evaluation; normalized analytic copies
/// are separate representations and require their own correspondence evidence.
#[derive(Clone, Debug, PartialEq)]
pub struct WrappedPlacementReport {
    /// Original epsilon, canonical allocation, actual matrix/translation and placement errors.
    pub spatial_budget: CertifiedFrameSpatialBudget,
    /// Achieved world-space bounds; canonical chart bounds remain in `result.reports()`.
    pub members: BTreeMap<SurfaceRegionId, WrappedWorldSpatialReport>,
}
/// Spatial bounds after transport to the physical frame, in model units.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WrappedWorldSpatialReport {
    /// Ideal profile displacement bound after placement.
    pub profile_error: WrappedCertifiedBound,
    /// Normal-sweep discrepancy including stored placement.
    pub normal_storage_error: WrappedCertifiedBound,
    /// World-space cap/edge trace discrepancy bound.
    pub cap_trace_error: WrappedCertifiedBound,
    /// World-space cap realization bound.
    pub cap_realization_error: WrappedCertifiedBound,
}
#[cfg(test)]
mod replay_tests;
fn retained(
    family: &family::UnqualifiedFamily,
    region: SurfaceRegionId,
) -> Checked<&pipeline::RetainedMember> {
    family
        .members
        .get(&region)
        .ok_or_else(|| fail(PlacementFailure::Member(region)))
}
#[cfg(test)]
mod tests;

#[cfg(test)]
mod rotated_budget_tests;
