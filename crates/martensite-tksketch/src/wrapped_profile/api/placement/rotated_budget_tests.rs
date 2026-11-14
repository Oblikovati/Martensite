//! A rotated reference uses the same full-graph publication and retained-tolerance gates.
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::{
    CertifiedRotatedFrameSpatialBudget, certify_frame_transport_at_angle,
};
type RotatedPlan = Plan<CertifiedRotatedFrameSpatialBudget>;
fn fixture() -> (WrappedNormalRequest, input::Inputs, RotatedPlan) {
    let request = super::super::operation_tests::request();
    let mut inputs = input::read(&request, &mut || false).unwrap();
    let plan = quarter_turn(&request, &inputs);
    for source in &mut inputs.requests {
        source.tolerance = plan.budget.canonical_allowance();
    }
    (request, inputs, plan)
}
fn quarter_turn(request: &WrappedNormalRequest, inputs: &input::Inputs) -> RotatedPlan {
    let rows = [[0., -1., 0., 0.], [1., 0., 0., 0.], [0., 0., 1., 0.]];
    let transform = Transform3::from_stored_rigid_coefficients(rows).unwrap();
    let input = FrameTransportInput {
        axis: request.support.axis,
        radial: request.support.radial,
        stored: *transform.matrix().coefficients(),
    };
    let proof = certify_frame_transport_at_angle(input, std::f64::consts::FRAC_PI_2).unwrap();
    let spatial = canonical_spatial_domain(request, inputs);
    RotatedPlan {
        transform,
        budget: proof.spatial_budget(spatial).unwrap(),
    }
}
fn canonical_spatial_domain(
    request: &WrappedNormalRequest,
    inputs: &input::Inputs,
) -> FrameSpatialInput {
    let point_norm = admission::point_norm(request, &inputs.requests).unwrap();
    FrameSpatialInput {
        origin: [0.; 3],
        translation: [0.; 3],
        point_norm,
        spatial_limit: request.options.spatial_error,
    }
}
#[test]
fn rotated_budget_publishes_hole_and_disconnected_member_through_the_shared_pipeline() {
    let (request, inputs, plan) = fixture();
    let mut result = super::super::construct::from_inputs(&request, inputs, &mut || false).unwrap();
    let physical = publication::apply(&mut result, plan, &mut || false).unwrap();
    assert_eq!(physical.members.len(), 2);
    assert_eq!(
        physical.budget.frame().rotation().angle,
        std::f64::consts::FRAC_PI_2
    );
    assert!(physical.budget.world_error_bound().unwrap() <= request.options.spatial_error);
    assert!(
        result
            .reports
            .values()
            .all(|report| report.validation.is_valid())
    );
    assert_world_bounds(&physical.members, request.options.spatial_error);
    check_first_vertex(&physical.bytes, &result, request.options.spatial_error);
}
fn assert_world_bounds(
    members: &BTreeMap<SurfaceRegionId, WrappedWorldSpatialReport>,
    epsilon: f64,
) {
    let bounds = members.values().flat_map(|report| {
        [
            report.profile_error,
            report.normal_storage_error,
            report.cap_trace_error,
            report.cap_realization_error,
        ]
    });
    for bound in bounds {
        assert!(bound.upper <= epsilon);
    }
}
fn check_first_vertex(bytes: &[u8], result: &WrappedNormalResult, epsilon: f64) {
    use crate::surface_sketch::HistoryRole;
    let key = BindingKey {
        source: Source::Corner(CornerSource::Authored(SurfaceVertexId(10))),
        role: HistoryRole::FirstDepthVertex,
    };
    let (_, bindings) = pipeline::decode(bytes, &result.family.family.keys).unwrap();
    let shape = &bindings
        .iter()
        .find(|(actual, _)| *actual == key)
        .unwrap()
        .1;
    let vertex = martensite_tkbrep::topods::vertex(shape).unwrap();
    let point = martensite_tkbrep::brep::BRepTool::pnt(&vertex).unwrap();
    for (actual, expected) in point.coordinates().iter().zip([0., 9., 0.]) {
        assert!((*actual - expected).abs() <= epsilon);
    }
}
