//! The stored rotated map is qualified against independently bounded trigonometry.
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::{
    CertifiedRotatedFrameSpatialBudget, certify_frame_transport_at_angle,
};
type ChartPlan = Plan<CertifiedRotatedFrameSpatialBudget>;
pub(super) fn prepare(request: &WrappedNormalRequest, sources: &[Request]) -> Checked<ChartPlan> {
    admission::admit_scalars(request)?;
    let support = request.support;
    let transform = rotated_transform(support)
        .map_err(|cause| fail(PlacementFailure::InputFrame { support, cause }))?;
    let frame = rotation_proof(
        support.axis,
        support.radial,
        support.chart_origin,
        transform,
    )?;
    let budget = frame
        .spatial_budget(admission::spatial_input(request, sources, transform)?)
        .map_err(|cause| fail(PlacementFailure::Frame(cause)))?;
    Ok(Plan { transform, budget })
}
pub(super) fn rotation_proof(
    axis: [f64; 3],
    radial: [f64; 3],
    angle: f64,
    transform: Transform3,
) -> Checked<martensite_tkmath::predicates::roots::metric_bounds::CertifiedRotatedFrameTransport> {
    certify_frame_transport_at_angle(
        FrameTransportInput {
            axis,
            radial,
            stored: *transform.matrix().coefficients(),
        },
        angle,
    )
    .map_err(|cause| fail(PlacementFailure::Rotation(cause)))
}
fn rotated_transform(
    support: crate::surface_sketch::CylinderSupport,
) -> Result<Transform3, Failure> {
    rotate(admission::transform(support)?, support.chart_origin)
}
pub(super) fn rotate(base: Transform3, angle: f64) -> Result<Transform3, Failure> {
    if angle == 0. || !angle.is_finite() {
        return Ok(base);
    }
    let (sine, cosine) = (martensite_cmath::sin(angle), martensite_cmath::cos(angle));
    stored_rotation(base, sine, cosine)
}
fn stored_rotation(base: Transform3, sine: f64, cosine: f64) -> Result<Transform3, Failure> {
    let rows = std::array::from_fn(|i| {
        let [x, y, z] = base.matrix().coefficients()[i];
        [
            cosine * x + sine * y,
            cosine * y - sine * x,
            z,
            base.translation_part().coordinates()[i],
        ]
    });
    Transform3::from_stored_rigid_coefficients(rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn inputs() -> (WrappedNormalRequest, input::Inputs) {
        let request = super::super::super::operation_tests::request();
        let inputs = input::read(&request, &mut || false).unwrap();
        (request, inputs)
    }
    #[test]
    fn chart_angle_is_retained_and_the_original_spatial_limit_is_enforced() {
        let (mut request, inputs) = inputs();
        request.support.chart_origin = std::f64::consts::FRAC_PI_2;
        let plan = prepare(&request, &inputs.requests).unwrap();
        assert_eq!(
            plan.budget.frame().rotation().angle,
            request.support.chart_origin
        );
        let rows = *plan.transform.matrix().coefficients();
        assert!((rows[1][0] - 1.).abs() < 1e-15);
        assert!((rows[0][1] + 1.).abs() < 1e-15);
        assert!(plan.budget.world_error_bound().unwrap() <= request.options.spatial_error);
    }
    #[test]
    fn nonfinite_chart_angles_are_typed_invalid_inputs() {
        let (mut request, inputs) = inputs();
        for angle in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            request.support.chart_origin = angle;
            let failure = prepare(&request, &inputs.requests).err().unwrap();
            assert_eq!(failure.class(), WrappedFailureClass::InvalidInput);
            assert!(failure.to_string().contains("finite angle"));
        }
    }
    #[test]
    fn unresolved_extreme_finite_chart_angle_is_incomplete() {
        let (mut request, inputs) = inputs();
        request.support.chart_origin = f64::MAX;
        let failure = prepare(&request, &inputs.requests).err().unwrap();
        assert_eq!(failure.class(), WrappedFailureClass::Incomplete);
        assert!(failure.to_string().contains("frame rotation"));
    }
}
