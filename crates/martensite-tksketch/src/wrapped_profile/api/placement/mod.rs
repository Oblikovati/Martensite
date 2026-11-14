//! One shared rigid datum transports the frozen graph; canonical evidence stays owned.
use super::*;
use martensite_tkmath::{
    gp::{Axis3, Direction3, Point3, Transform3},
    predicates::roots::metric_bounds::{
        CertifiedFrameSpatialBudget, FrameSpatialInput, FrameTransportError, FrameTransportInput,
        certify_frame_transport,
    },
    toploc::Location,
};
use std::collections::BTreeMap;
mod admission;
mod error;
mod freeze;
mod measures;
mod publication;
mod trace;
pub(super) use admission::prepare;
pub(super) use error::PlacementFailure;
use error::{at, fail};
pub(super) use publication::publish;
pub(super) struct Plan {
    transform: Transform3,
    budget: CertifiedFrameSpatialBudget,
}
pub(super) struct Placed {
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
