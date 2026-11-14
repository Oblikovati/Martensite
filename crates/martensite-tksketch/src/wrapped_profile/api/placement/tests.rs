//! Exercise physical publication, decoded graph correspondence and an independent rigid oracle.
use super::*;
pub(super) fn request() -> WrappedNormalRequest {
    let mut request = super::super::operation_tests::request();
    request.support.origin = [1.25, -2.5, 3.75];
    request.support.axis = [1., 2., 3.];
    request.support.radial = [3., -1., 2.];
    request
}
#[test]
fn oblique_profile_with_hole_and_disconnected_member_publishes_physical_evidence() {
    let request = request();
    let result = normal_extrude_profile(&request).unwrap_or_else(|cause| panic!("{cause}"));
    let placement = result.placement().unwrap();
    assert_eq!(placement.members.len(), 2);
    assert!(placement.spatial_budget.canonical_allowance() < request.options.spatial_error);
    assert!(placement.spatial_budget.world_error_bound().unwrap() <= request.options.spatial_error);
    assert!(
        result
            .reports()
            .values()
            .all(|report| report.validation.is_valid())
    );
    assert_eq!(result.budget().spatial_error, request.options.spatial_error);
    assert!(result.budget().volume.discrepancy.upper <= 1e-4);
    let exported = result.export_native().unwrap();
    assert!(!exported.bodies.location().is_identity());
    assert_eq!(exported.entities.len(), result.history().len());
}
#[test]
fn physical_publication_reports_are_send_sync() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<WrappedPlacementReport>();
    send_sync::<WrappedWorldSpatialReport>();
}
#[test]
fn canonical_request_keeps_the_existing_fast_path() {
    let result = normal_extrude_profile(&super::super::operation_tests::request()).unwrap();
    assert!(result.placement().is_none());
    assert_eq!(result.encoded_brep(), result.family.family.bytes);
}
#[test]
fn large_origin_refuses_before_returning_any_geometry() {
    let mut request = request();
    request.support.origin = [1e16; 3];
    let failure = normal_extrude_profile(&request).err().unwrap();
    assert_eq!(failure.stage(), WrappedStage::Placement);
    assert_eq!(failure.class(), WrappedFailureClass::Incomplete);
    assert!(failure.to_string().contains("SpatialLimit"));
}

#[test]
fn oblique_outer_and_hole_vertices_follow_independent_cylindrical_normals() {
    let input = request();
    let result = normal_extrude_profile(&input).unwrap();
    let native = result.export_native().unwrap();
    for (vertex, developed) in [(10, [0., 0.]), (20, [0.5, 0.5])] {
        for (role, depth) in depth_roles() {
            let actual = point(&native, vertex, role);
            let expected = ideal_point(developed, depth);
            assert_point_error(actual, expected, input.options.spatial_error);
        }
        assert_normal_direction(&native, vertex, developed, input.options.spatial_error);
    }
}
fn assert_point_error(actual: [f64; 3], expected: [f64; 3], epsilon: f64) {
    for i in 0..3 {
        assert!((actual[i] - expected[i]).abs() <= epsilon);
    }
}
fn depth_roles() -> [(crate::surface_sketch::HistoryRole, f64); 2] {
    use crate::surface_sketch::HistoryRole as H;
    [(H::FirstDepthVertex, -1.), (H::LastDepthVertex, 2.)]
}
fn point(
    native: &DetachedWrappedBodies,
    vertex: u64,
    role: crate::surface_sketch::HistoryRole,
) -> [f64; 3] {
    let key = WrappedHistoryKey {
        source: WrappedEntitySource::Vertex(SurfaceVertexId(vertex)),
        role,
    };
    let vertex = martensite_tkbrep::topods::vertex(&native.entities[&key]).unwrap();
    let point = martensite_tkbrep::brep::BRepTool::pnt(&vertex).unwrap();
    [point.x(), point.y(), point.z()]
}
fn ideal_point([s, height]: [f64; 2], depth: f64) -> [f64; 3] {
    let x = [5., -4., 1.].map(|v| v / 42_f64.sqrt());
    let y = [1., 1., -1.].map(|v| v / 3_f64.sqrt());
    let z = [1., 2., 3.].map(|v| v / 14_f64.sqrt());
    let origin = [1.25, -2.5, 3.75];
    let (cos, sin) = (
        martensite_cmath::cos(s / 10.),
        martensite_cmath::sin(s / 10.),
    );
    std::array::from_fn(|i| origin[i] + (10. + depth) * (cos * x[i] + sin * y[i]) + height * z[i])
}
fn assert_normal_direction(
    native: &DetachedWrappedBodies,
    vertex: u64,
    developed: [f64; 2],
    epsilon: f64,
) {
    let [(first_role, _), (last_role, _)] = depth_roles();
    let (first, last) = (
        point(native, vertex, first_role),
        point(native, vertex, last_role),
    );
    let (ideal_first, ideal_last) = (ideal_point(developed, -1.), ideal_point(developed, 2.));
    for i in 0..3 {
        assert!(((last[i] - first[i]) - (ideal_last[i] - ideal_first[i])).abs() <= epsilon);
    }
}
#[test]
fn inward_physical_placement_keeps_geometry_and_reverses_requested_depth_roles() {
    let input = request();
    let outward = normal_extrude_profile(&input).unwrap();
    let mut inward_input = input;
    inward_input.support.normal = crate::surface_sketch::NormalSense::Inward;
    inward_input.depths = [-2., 1.];
    let inward = normal_extrude_profile(&inward_input).unwrap();
    assert_eq!(inward.encoded_brep(), outward.encoded_brep());
    let a = inward.export_native().unwrap();
    let b = outward.export_native().unwrap();
    let [(first, _), (last, _)] = depth_roles();
    assert_eq!(point(&a, 10, first), point(&b, 10, last));
    assert_eq!(point(&a, 10, last), point(&b, 10, first));
}
#[test]
fn oblique_replay_and_concurrent_operations_have_identical_bytes_history_and_bounds() {
    let input = request();
    let expected = normal_extrude_profile(&input).unwrap();
    let workers: Vec<_> = (0..2)
        .map(|_| {
            let input = input.clone();
            std::thread::spawn(move || normal_extrude_profile(&input).unwrap())
        })
        .collect();
    for worker in workers {
        let result = worker.join().unwrap();
        assert_eq!(expected.encoded_brep(), result.encoded_brep());
        assert_eq!(expected.history(), result.history());
        assert_eq!(expected.budget(), result.budget());
        assert_eq!(expected.placement(), result.placement());
    }
}
#[test]
fn degenerate_and_nonfinite_physical_frames_refuse_as_invalid_input() {
    let mut input = request();
    for axis in [[0.; 3], input.support.radial, [f64::NAN, 0., 1.]] {
        input.support.axis = axis;
        let failure = normal_extrude_profile(&input).err().unwrap();
        assert_eq!(failure.class(), WrappedFailureClass::InvalidInput);
        assert!(failure.to_string().contains("expected"));
    }
}
#[test]
fn invalid_spatial_limits_refuse_as_input_errors_before_geometry_construction() {
    let mut input = request();
    for spatial_error in [0., -1., f64::NAN, f64::INFINITY, f64::MAX] {
        input.options.spatial_error = spatial_error;
        let failure = normal_extrude_profile(&input).err().unwrap();
        assert_eq!(failure.class(), WrappedFailureClass::InvalidInput);
        assert!(failure.to_string().contains("spatial_error"));
    }
}
#[test]
fn unresolvable_nearly_parallel_frame_refuses_instead_of_choosing_a_fallback() {
    let mut input = request();
    input.support.axis = [1., 0., 0.];
    input.support.radial = [1., f64::MIN_POSITIVE, 0.];
    let failure = normal_extrude_profile(&input).err().unwrap();
    assert_eq!(failure.class(), WrappedFailureClass::InvalidInput);
    assert!(failure.to_string().contains("support:"));
}

#[test]
fn source_and_detached_export_mutation_do_not_change_the_published_physical_result() {
    let mut input = request();
    let result = normal_extrude_profile(&input).unwrap();
    let bytes = result.encoded_brep().to_vec();
    let first = result.export_native().unwrap();
    let second = result.export_native().unwrap();
    let role = crate::surface_sketch::HistoryRole::FirstDepthVertex;
    let before = point(&second, 10, role);
    mutate_vertex(&first, role);
    assert_ne!(point(&first, 10, role), before);
    input.profile.vertices[0].point[0] += 100.;
    input.support.origin[0] += 100.;
    assert_eq!(point(&second, 10, role), before);
    assert_eq!(point(&result.export_native().unwrap(), 10, role), before);
    assert_eq!(result.encoded_brep(), bytes);
}
fn mutate_vertex(native: &DetachedWrappedBodies, role: crate::surface_sketch::HistoryRole) {
    let key = WrappedHistoryKey {
        source: WrappedEntitySource::Vertex(SurfaceVertexId(10)),
        role,
    };
    let vertex = martensite_tkbrep::topods::vertex(&native.entities[&key]).unwrap();
    martensite_tkbrep::brep::BRepBuilder::default()
        .update_vertex_point(&vertex, Point3::new(100., 100., 100.), 0.)
        .unwrap();
}
#[test]
fn cancellation_during_placement_keeps_inputs_and_prior_results_unchanged() {
    let input = request();
    let snapshot = input.clone();
    let (result, count) = count_polls(&input);
    let before = result.encoded_brep().to_vec();
    let mut current = 0;
    let failure = normal_extrude_profile_with_cancel(&input, || {
        current += 1;
        current == count - 1
    })
    .err()
    .unwrap();
    assert_eq!(failure.stage(), WrappedStage::Placement);
    assert_eq!(failure.class(), WrappedFailureClass::Cancelled);
    assert_eq!(input, snapshot);
    assert_eq!(result.encoded_brep(), before);
}

fn count_polls(input: &WrappedNormalRequest) -> (WrappedNormalResult, usize) {
    let mut count = 0;
    let result = normal_extrude_profile_with_cancel(input, || {
        count += 1;
        false
    })
    .unwrap();
    (result, count)
}
