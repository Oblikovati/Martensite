//! Canonical input stays allocation-free; other frames reserve error before construction.
use super::*;
use crate::surface_sketch::CylinderSupport;
pub(in super::super) fn prepare(
    request: &WrappedNormalRequest,
    requests: &mut [Request],
) -> Checked<Option<Box<Plan>>> {
    if canonical(request.support) {
        return Ok(None);
    }
    let plan = compile(request, requests)?;
    for source in requests {
        source.tolerance = plan.budget.canonical_allowance();
    }
    Ok(Some(Box::new(plan)))
}
fn compile(request: &WrappedNormalRequest, sources: &[Request]) -> Checked<Plan> {
    admit_scalars(request)?;
    let support = request.support;
    let transform = transform(support)
        .map_err(|cause| fail(PlacementFailure::InputFrame { support, cause }))?;
    let frame = certify_frame_transport(FrameTransportInput {
        axis: support.axis,
        radial: support.radial,
        stored: *transform.matrix().coefficients(),
    })
    .map_err(|cause| fail(PlacementFailure::Frame(cause)))?;
    let budget = frame
        .spatial_budget(spatial_input(request, sources, transform)?)
        .map_err(|cause| fail(PlacementFailure::Frame(cause)))?;
    Ok(Plan { transform, budget })
}
pub(super) fn admit_scalars(request: &WrappedNormalRequest) -> Checked<()> {
    let radius = request.support.radius;
    let spatial_error = request.options.spatial_error;
    if !radius.is_finite()
        || radius <= 0.
        || !spatial_error.is_finite()
        || spatial_error <= 0.
        || !(4. * spatial_error).is_finite()
    {
        return Err(fail(PlacementFailure::SpatialInput {
            radius,
            spatial_error,
        }));
    }
    Ok(())
}
fn canonical(support: CylinderSupport) -> bool {
    support.origin == [0.; 3] && support.axis == [0., 0., 1.] && support.radial == [1., 0., 0.]
}
pub(super) fn transform(support: CylinderSupport) -> Result<Transform3, Failure> {
    crate::surface_sketch::wrapped_profile::physical_frame::stored_rigid_transform(
        support.origin,
        support.axis,
        support.radial,
    )
}
pub(super) fn spatial_input(
    request: &WrappedNormalRequest,
    sources: &[Request],
    transform: Transform3,
) -> Checked<FrameSpatialInput> {
    Ok(FrameSpatialInput {
        origin: request.support.origin,
        translation: *transform.translation_part().coordinates(),
        point_norm: point_norm(request, sources)?,
        spatial_limit: request.options.spatial_error,
    })
}
pub(super) fn point_norm(request: &WrappedNormalRequest, sources: &[Request]) -> Checked<f64> {
    let mut norm = request.support.radius;
    let depth = request.depths.into_iter().map(f64::abs).fold(0., f64::max);
    for value in [depth, height_bound(sources), request.options.spatial_error] {
        norm = error::budget(publication_budget::add_upper(
            norm,
            value,
            publication_budget::Quantity::CapDomain,
        ))?;
    }
    Ok(norm)
}
fn height_bound(sources: &[Request]) -> f64 {
    sources
        .iter()
        .flat_map(|r| &r.loops)
        .flatten()
        .flat_map(|piece| &piece.poles)
        .map(|p| p[1].abs())
        .fold(0., f64::max)
}
