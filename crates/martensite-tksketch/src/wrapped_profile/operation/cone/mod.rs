//! Analytic cone mapping and mathematical receipts feed the common operation stages.
use super::*;
use crate::surface_sketch::{ConeSupport, HistoryRole, NormalSense};
use martensite_tkmath::predicates::roots::metric_bounds::*;
use std::collections::BTreeMap;
mod actual;
mod domain;
mod error;
pub(in crate::surface_sketch::wrapped_profile) use error::{ConeCause, ConeFailure};
mod input;
mod material;
mod measures;
mod native;
mod prepare;
mod publication;
mod stages;
mod storage;
#[cfg(test)]
pub(in crate::surface_sketch::wrapped_profile) mod tests;
type ConeChecked<T> = Result<T, ConeFailure>;
pub(in crate::surface_sketch::wrapped_profile) type ConeBudgeted =
    super::stages::Budgeted<Selection, Geometry, Budget>;
pub(in crate::surface_sketch::wrapped_profile) struct Prepared {
    pub source: DevelopedRegionRequest,
    pub executed_selection: WrappedConeExecutionSelection,
    pub fits: BTreeMap<CurveSpan, SpanFit>,
    pub directed_family: Option<std::sync::Arc<prepare::directed::DirectedFamily>>,
    pub final_endpoints: Option<std::sync::Arc<prepare::directed::FinalEndpoints>>,
    pub reference_slant: f64,
    pub semi_angle: f64,
    pub chart_origin: f64,
    pub chart_center: f64,
    pub chart_domain: Option<std::sync::Arc<ConicalChartDomainEvidence>>,
    pub arithmetic_operations: u32,
    pub radii: Vec<ConeRequestReceipt>,
    pub wedge: ConicalSectorWedge,
}
pub(in crate::surface_sketch::wrapped_profile) struct SpanFit {
    pub transition: Option<std::sync::Arc<ConicalBoundaryZRowEvidence>>,
    pub fit: cone_mapping::fit::OwnedFit,
    pub developed_turn: i32,
    pub uv: [[f64; 2]; 4],
    pub poles: [angular::RationalPoles; 2],
    // The immutable full-query snapshot remains operation-owned across preparation and freeze.
    pub boundary: std::sync::Arc<ConicalBoundaryEvidence>,
}
impl AsRef<DevelopedRegionRequest> for Prepared {
    fn as_ref(&self) -> &DevelopedRegionRequest {
        &self.source
    }
}
pub(in crate::surface_sketch::wrapped_profile) struct Inputs {
    pub originals: Vec<DevelopedRegionRequest>,
    pub directions: BTreeMap<SurfaceRegionId, profile_input::CurveDirections>,
}
pub(in crate::surface_sketch::wrapped_profile) struct Selection {
    pub executed_selection: WrappedConeExecutionSelection,
    pub prepared: Vec<Prepared>,
    pub planar: BTreeMap<SurfaceRegionId, planar::PlanarEvidence>,
    pub census: martensite_tkbo::profile_extrusion::RegionCertificate,
    pub work: martensite_tkbo::profile_extrusion::PlanarProofWorkReceipt,
    pub domain: domain::DomainEvidence,
    pub clearance: CertifiedCylindricalClearance,
    pub arithmetic_operations: u32,
}
pub(in crate::surface_sketch::wrapped_profile) struct Geometry {
    pub coverage: coverage::CoverageEvidence,
    pub spans: Vec<storage::SpanProof>,
    pub flux: CertifiedAnalyticConicalFlux,
    pub authored: ConicalAuthoredFlux,
    pub reference_volume: CertifiedConicalReferenceVolume,
    pub joins: ConicalJoinsEvidence,
    pub material: material::MaterialEvidence,
    pub arithmetic_operations: u32,
}
pub(in crate::surface_sketch::wrapped_profile) struct Budget {
    pub actual_certificate:
        Option<std::sync::Arc<martensite_tkbo::body_check::CertifiedConicalBody>>,
    pub arithmetic_operations: u32,
    pub integration_subintervals: u32,
    pub planar_work: martensite_tkbo::profile_extrusion::PlanarProofWorkReceipt,
    pub spatial_error: f64,
    pub volume: WrappedConeVolumeBounds,
    pub reference: ConicalReferenceEvidence,
    pub cap_reference_proxy_error: f64,
}
pub(in crate::surface_sketch::wrapped_profile) fn execute(
    request: &WrappedConeNormalRequest,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<WrappedConeNormalResult> {
    execute_with_domain(request, WrappedConeDomainPolicy::LegacySectorV1, cancelled)
}
pub(in crate::surface_sketch::wrapped_profile) fn execute_with_domain(
    request: &WrappedConeNormalRequest,
    domain_policy: WrappedConeDomainPolicy,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<WrappedConeNormalResult> {
    execute_with_selection(
        request,
        WrappedConeExecutionSelection::RoundedProxyNativeSearchV1 {
            domain: domain_policy,
        },
        cancelled,
    )
}
pub(in crate::surface_sketch::wrapped_profile) fn execute_with_selection(
    request: &WrappedConeNormalRequest,
    execution_selection: WrappedConeExecutionSelection,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<WrappedConeNormalResult> {
    input::poll(cancelled, WrappedStage::Admission)?;
    let inputs = input::capture_with_selection(request, execution_selection, cancelled)?;
    let mut checkpoint = |_| cancelled();
    let family = super::execute::budgeted(
        &stages::ConeStages {
            request,
            execution_selection,
        },
        &inputs.originals,
        &mut checkpoint,
    )
    .map_err(ConeFailure::public)?;
    publication::publish(request, inputs, family, cancelled)
}
