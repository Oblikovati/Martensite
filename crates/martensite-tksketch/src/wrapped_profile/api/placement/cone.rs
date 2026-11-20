//! Place the owned canonical cone once, with a separately certified world-space budget.
use super::*;
use crate::surface_sketch::ConeSupport;
use crate::surface_sketch::wrapped_profile::operation::cone::{self as operation_cone, ConeRun};
use martensite_tkmath::predicates::roots::metric_bounds::{
    CertifiedRotatedFrameSpatialBudget, CertifiedRotatedFrameTransport,
};
mod members;

type ConePlan = Plan<CertifiedRotatedFrameSpatialBudget>;

/// An independently owned world-space cone result and its canonical proof receipts.
pub struct WrappedPlacedConeResult {
    base: WrappedConeNormalResult,
    bytes: Vec<u8>,
    support: ConeSupport,
    options: WrappedNormalOptions,
    placement: WrappedConePlacementReport,
}

/// World-space cone bounds under the exact requested frame and chart-angle convention.
#[derive(Clone, Debug)]
pub struct WrappedConePlacementReport {
    /// Original world limit, actual stored frame, and effective canonical allocation.
    pub spatial_budget: CertifiedRotatedFrameSpatialBudget,
    /// Outward family error, including direct evaluation of the stored transform.
    pub achieved_spatial_error: WrappedCertifiedBound,
    /// Physical native observations and transported bounds by authored region.
    pub members: BTreeMap<SurfaceRegionId, WrappedPlacedConeMemberReport>,
}

/// Physical cone member measurements and spatial checks.
#[derive(Clone, Debug)]
pub struct WrappedPlacedConeMemberReport {
    /// Ordered native observations in the world-space frame.
    pub validation: martensite_tkbo::body_check::ValidationReport,
    /// Authored-to-stored boundary discrepancy under placement.
    pub boundary_error: WrappedCertifiedBound,
    /// Full-depth stored ruling discrepancy under placement.
    pub ruled_storage_error: WrappedCertifiedBound,
    /// Largest placed cap trace residual, checked against every retained limit.
    pub cap_trace_error: WrappedCertifiedBound,
    /// Authored-to-stored filled-cap discrepancy under placement.
    pub cap_realization_error: WrappedCertifiedBound,
    /// Conditional occurrence flux, transported by the certified positive determinant.
    pub boundary_flux: WrappedCertifiedRange,
    /// Physical proxy flux compared to the ideal rigid-frame reference volume.
    pub proxy_reference_discrepancy: WrappedCertifiedBound,
    /// Native quadrature observation of the placed solid.
    pub native_volume: WrappedNativeVolumeEstimate,
}
impl WrappedPlacedConeResult {
    /// Canonical proof receipts and the same authored history; `result.canonical().history()`.
    pub fn canonical(&self) -> &WrappedConeNormalResult {
        &self.base
    }
    /// Original requested frame; `result.support().apex`.
    pub fn support(&self) -> ConeSupport {
        self.support
    }
    /// Original world-space limits; `result.options().spatial_error`.
    pub fn options(&self) -> WrappedNormalOptions {
        self.options
    }
    /// Transport and physical observations; `result.placement().members.len()`.
    pub fn placement(&self) -> &WrappedConePlacementReport {
        &self.placement
    }
    /// Decoded origin of the centered world-space field; `result.boundary_flux_origin()`.
    pub fn boundary_flux_origin(&self) -> [f64; 3] {
        self.placement.spatial_budget.input().translation
    }
    /// Frozen native graph in the requested chart frame; `result.encoded_brep().len()`.
    pub fn encoded_brep(&self) -> &[u8] {
        &self.bytes
    }
    /// Detached world-space graph with the canonical authored identity keys.
    /// Example: `let graph = result.export_native()?`.
    pub fn export_native(&self) -> Checked<DetachedWrappedBodies> {
        self.base.export_bytes(&self.bytes)
    }
}

/// Extrude in an explicitly placed cone chart using the selected canonical behavior.
/// Example: `let result = normal_extrude_cone_profile_on_chart(&request, selection)?;`.
pub fn normal_extrude_cone_profile_on_chart(
    request: &WrappedConeNormalRequest,
    selection: WrappedConeExecutionSelection,
) -> Checked<WrappedPlacedConeResult> {
    normal_extrude_cone_profile_on_chart_with_cancel(request, selection, || false)
}

/// Cooperatively cancel the selected placed cone operation before publication.
/// Example: `normal_extrude_cone_profile_on_chart_with_cancel(&request, selection, || false)?`.
pub fn normal_extrude_cone_profile_on_chart_with_cancel(
    request: &WrappedConeNormalRequest,
    selection: WrappedConeExecutionSelection,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Checked<WrappedPlacedConeResult> {
    cone_on_chart(request, selection, &mut cancelled, |_, _| Ok(())).map(|(body, ())| body)
}

pub(in crate::surface_sketch::wrapped_profile::api) fn cone_on_chart<T, E>(
    request: &WrappedConeNormalRequest,
    selection: WrappedConeExecutionSelection,
    cancelled: &mut CancelPoll<'_>,
    inspect: impl FnOnce(&[DevelopedRegionRequest], &mut CancelPoll<'_>) -> Result<T, E>,
) -> Result<(WrappedPlacedConeResult, T), E>
where
    E: From<WrappedNormalFailure>,
{
    input::poll(cancelled, WrappedStage::Admission)?;
    let (transform, frame) = frame(request.support)?;
    let mut run = canonical_run(request);
    let mut inputs = operation_cone::input::capture_with_selection(&run, selection, cancelled)?;
    let inspected = inspect(&inputs.originals, cancelled)?;
    let plan = allocate(request, transform, &frame, &inputs.originals)?;
    run.options.spatial_error = plan.budget.canonical_allowance();
    for source in &mut inputs.originals {
        source.tolerance = run.options.spatial_error;
    }
    let base = operation_cone::execute_captured(&run, selection, inputs, cancelled)?;
    Ok((publish(request, base, plan, cancelled)?, inspected))
}

fn canonical_run(request: &WrappedConeNormalRequest) -> ConeRun<'_> {
    let mut run = ConeRun::from(request);
    run.support.apex = [0.; 3];
    run.support.axis = [0., 0., 1.];
    run.support.radial = [1., 0., 0.];
    run.support.chart_origin = 0.;
    run
}

fn frame(support: ConeSupport) -> Checked<(Transform3, CertifiedRotatedFrameTransport)> {
    let transform = crate::surface_sketch::wrapped_profile::physical_frame::stored_rigid_transform(
        support.apex,
        support.axis,
        support.radial,
    )
    .and_then(|base| chart_admission::rotate(base, support.chart_origin))
    .map_err(|cause| fail(PlacementFailure::ConeFrame { support, cause }))?;
    let proof = chart_admission::rotation_proof(
        support.axis,
        support.radial,
        support.chart_origin,
        transform,
    )?;
    Ok((transform, proof))
}
fn allocate(
    request: &WrappedConeNormalRequest,
    transform: Transform3,
    frame: &CertifiedRotatedFrameTransport,
    sources: &[DevelopedRegionRequest],
) -> Checked<ConePlan> {
    let spatial = FrameSpatialInput {
        origin: request.support.apex,
        translation: *transform.translation_part().coordinates(),
        point_norm: point_norm(request, sources)?,
        spatial_limit: request.options.spatial_error,
    };
    let budget = frame
        .spatial_budget(spatial)
        .map_err(|cause| fail(PlacementFailure::Frame(cause)))?;
    Ok(Plan { transform, budget })
}
fn point_norm(
    request: &WrappedConeNormalRequest,
    sources: &[DevelopedRegionRequest],
) -> Checked<f64> {
    let maxima = sources
        .iter()
        .flat_map(|source| source.loops.iter().flatten())
        .flat_map(|piece| &piece.poles)
        .fold([0_f64; 2], |[x, y], point| {
            [x.max(point[0].abs()), y.max(point[1].abs())]
        });
    let max_depth = request.depths[0].abs().max(request.depths[1].abs());
    // A normal offset has canonical norm at most the developed slant plus the depth magnitude.
    // The extra allowance covers the certified stored-boundary displacement.
    [
        maxima[0],
        maxima[1],
        max_depth,
        request.options.spatial_error,
    ]
    .into_iter()
    .try_fold(0., |sum, value| {
        error::budget(publication_budget::add_upper(
            sum,
            value,
            publication_budget::Quantity::CapDomain,
        ))
    })
}

fn publish(
    request: &WrappedConeNormalRequest,
    base: WrappedConeNormalResult,
    plan: ConePlan,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<WrappedPlacedConeResult> {
    input::poll(cancelled, WrappedStage::Placement)?;
    let family = &base.family.family;
    let physical = if freeze::identity(plan.transform) {
        freeze::unchanged_bytes(&family.bytes, &family.keys)?
    } else {
        freeze::place_bytes(&family.bytes, &family.keys, plan.transform)?
    };
    let member_reports = members::certify(&base, &plan, &physical, cancelled)?;
    let achieved = publication::world_bound(base.budget().achieved_spatial_error, &plan)?;
    input::poll(cancelled, WrappedStage::Publication)?;
    let placement = WrappedConePlacementReport {
        spatial_budget: plan.budget,
        achieved_spatial_error: achieved,
        members: member_reports,
    };
    Ok(WrappedPlacedConeResult {
        base,
        bytes: physical.bytes,
        support: request.support,
        options: request.options,
        placement,
    })
}

#[cfg(test)]
mod tests;
