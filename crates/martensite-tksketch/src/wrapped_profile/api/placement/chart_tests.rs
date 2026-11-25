//! Public chart publication controls use independently evaluated frame points.
use super::*;
#[test]
fn public_quarter_turn_exports_a_qualified_physical_graph() {
    let mut request = super::super::super::operation_tests::request();
    request.support.chart_origin = std::f64::consts::FRAC_PI_2;
    assert!(normal_extrude_profile(&request).is_err());
    let result = normal_extrude_profile_on_chart(&request).unwrap();
    assert_eq!(result.behavior_version(), 1);
    assert_eq!(result.support(), request.support);
    assert_eq!(result.placement().members.len(), 2);
    assert!(
        result
            .reports()
            .values()
            .all(|report| report.validation.is_valid())
    );
    let native = result.export_native().unwrap();
    assert!(!native.entities.is_empty());
    check_vertex(&result);
    assert_send_sync::<WrappedChartNormalResult>();
    assert_send_sync::<WrappedChartPlacementReport>();
}
fn assert_send_sync<T: Send + Sync>() {}
fn check_vertex(result: &WrappedChartNormalResult) {
    let (_, bindings) = pipeline::decode(&result.bytes, &result.base.family.family.keys).unwrap();
    let key = BindingKey {
        source: Source::Corner(CornerSource::Authored(SurfaceVertexId(10))),
        role: crate::surface_sketch::HistoryRole::FirstDepthVertex,
    };
    let shape = &bindings
        .iter()
        .find(|(candidate, _)| *candidate == key)
        .unwrap()
        .1;
    let vertex = martensite_tkbrep::topods::vertex(shape).unwrap();
    let point = martensite_tkbrep::brep::BRepTool::pnt(&vertex).unwrap();
    for (actual, expected) in point.coordinates().iter().zip([0., 9., 0.]) {
        assert!((*actual - expected).abs() <= result.options().spatial_error);
    }
}
#[test]
fn cancellation_publishes_no_chart_result() {
    let request = super::super::super::operation_tests::request();
    let error = normal_extrude_profile_on_chart_with_cancel(&request, || true)
        .err()
        .unwrap();
    assert_eq!(error.class(), WrappedFailureClass::Cancelled);
}
#[test]
fn zero_angle_uses_the_verified_identity_publication_path() {
    let request = super::super::super::operation_tests::request();
    let result = normal_extrude_profile_on_chart(&request).unwrap();
    assert_eq!(result.bytes, result.base.family.family.bytes);
    assert_eq!(
        result
            .placement
            .spatial_budget
            .frame()
            .rotation()
            .operations,
        0
    );
    assert_identity_chart_validation(&result, &request);
}
#[test]
fn signed_and_multiturn_oblique_charts_follow_the_requested_reference() {
    for angle in [-0.3, 2. * std::f64::consts::PI + 0.3] {
        let mut request = super::super::super::operation_tests::request();
        request.support.origin = [1.25, -2.5, 3.75];
        request.support.axis = [1., 2., 3.];
        request.support.radial = [3., -1., 2.];
        request.support.chart_origin = angle;
        let result = normal_extrude_profile_on_chart(&request).unwrap();
        let expected = oblique_vertex(angle);
        let vertex = first_vertex(&result);
        let point = martensite_tkbrep::brep::BRepTool::pnt(&vertex).unwrap();
        for (actual, expected) in point.coordinates().iter().zip(expected) {
            assert!((*actual - expected).abs() <= request.options.spatial_error);
        }
        assert_eq!(
            result.placement.spatial_budget.frame().rotation().angle,
            angle
        );
    }
}
fn oblique_vertex(angle: f64) -> [f64; 3] {
    // Projection of (3,-1,2) off (1,2,3) is (2.5,-2,0.5).
    let x = [
        2.5 / 10.5_f64.sqrt(),
        -2. / 10.5_f64.sqrt(),
        0.5 / 10.5_f64.sqrt(),
    ];
    let z = [1. / 14_f64.sqrt(), 2. / 14_f64.sqrt(), 3. / 14_f64.sqrt()];
    let y = [
        z[1] * x[2] - z[2] * x[1],
        z[2] * x[0] - z[0] * x[2],
        z[0] * x[1] - z[1] * x[0],
    ];
    let origin = [1.25, -2.5, 3.75];
    std::array::from_fn(|i| {
        origin[i] + 9. * (martensite_cmath::cos(angle) * x[i] + martensite_cmath::sin(angle) * y[i])
    })
}
fn first_vertex(result: &WrappedChartNormalResult) -> martensite_tkbrep::topods::Vertex {
    let native = result.export_native().unwrap();
    let key = WrappedHistoryKey {
        source: WrappedEntitySource::Vertex(SurfaceVertexId(10)),
        role: crate::surface_sketch::HistoryRole::FirstDepthVertex,
    };
    martensite_tkbrep::topods::vertex(native.entities.get(&key).unwrap()).unwrap()
}
#[test]
fn source_and_native_export_mutation_cannot_change_published_chart_geometry() {
    let mut request = super::super::super::operation_tests::request();
    request.support.chart_origin = 0.3;
    let result = normal_extrude_profile_on_chart(&request).unwrap();
    let original = result.bytes.clone();
    let vertex = first_vertex(&result);
    martensite_tkbrep::brep::BRepBuilder::default()
        .update_vertex_point(&vertex, Point3::new(100., 100., 100.), 0.)
        .unwrap();
    request.support.radius = 1000.;
    assert_eq!(result.support().radius, 10.);
    assert_eq!(result.bytes, original);
    let fresh = first_vertex(&result);
    let point = martensite_tkbrep::brep::BRepTool::pnt(&fresh).unwrap();
    assert!((point.x() - 9. * martensite_cmath::cos(0.3)).abs() <= result.options().spatial_error);
    assert!((point.y() - 9. * martensite_cmath::sin(0.3)).abs() <= result.options().spatial_error);
}
#[test]
fn cancellation_at_final_publication_returns_no_result() {
    let mut request = super::super::super::operation_tests::request();
    request.support.chart_origin = 0.3;
    let (result, total) = count_chart_polls(&request);
    assert_chart_material_work(&result, total);
    let mut current = 0;
    let failure = normal_extrude_profile_on_chart_with_cancel(&request, || {
        current += 1;
        current == total
    })
    .err()
    .unwrap();
    assert_eq!(failure.class(), WrappedFailureClass::Cancelled);
    assert_eq!(failure.stage(), WrappedStage::Publication);
}

const CHART_WORKER: &str =
    "surface_sketch::wrapped_profile::api::placement::chart::tests::chart_replay_worker";
const CHART_PREFIX: &str = "CHART_REPLAY=";
fn replay_signature() -> String {
    let mut request = super::super::super::operation_tests::request();
    request.support.origin = [1.25, -2.5, 3.75];
    request.support.axis = [1., 2., 3.];
    request.support.radial = [3., -1., 2.];
    request.support.chart_origin = -0.3;
    let result = normal_extrude_profile_on_chart(&request).unwrap();
    format!(
        "{:?}|{:?}|{:?}|{:?}|{:?}|{:02x?}",
        result.history(),
        result.budget(),
        result.placement(),
        result.reports(),
        result.material_relations().collect::<Vec<_>>(),
        result.encoded_brep()
    )
}
#[test]
#[ignore = "subprocess worker for chart replay"]
fn chart_replay_worker() {
    println!("{CHART_PREFIX}{}", replay_signature());
}
#[test]
fn chart_replay_matches_across_processes_and_threads() {
    let expected = replay_signature();
    let threads: Vec<_> = (0..2)
        .map(|_| std::thread::spawn(replay_signature))
        .collect();
    for thread in threads {
        assert_eq!(thread.join().unwrap(), expected);
    }
    for _ in 0..2 {
        assert_eq!(chart_child_signature(), expected);
    }
}
fn chart_child_signature() -> String {
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", CHART_WORKER, "--ignored", "--nocapture"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .find_map(|line| line.strip_prefix(CHART_PREFIX))
        .unwrap()
        .to_owned()
}
#[test]
fn rotated_inward_depths_match_the_opposite_outward_interval() {
    let mut request = super::super::super::operation_tests::request();
    request.support.chart_origin = 0.3;
    request.support.normal = crate::surface_sketch::NormalSense::Inward;
    request.depths = [-2., 1.];
    let inward = normal_extrude_profile_on_chart(&request).unwrap();
    request.support.normal = crate::surface_sketch::NormalSense::Outward;
    request.depths = [-1., 2.];
    let outward = normal_extrude_profile_on_chart(&request).unwrap();
    assert_eq!(inward.encoded_brep(), outward.encoded_brep());
    assert_eq!(inward.depths(), [-2., 1.]);
    assert!(
        inward
            .reports()
            .values()
            .all(|member| member.validation.is_valid())
    );
}

fn assert_identity_chart_validation(
    result: &WrappedChartNormalResult,
    request: &WrappedNormalRequest,
) {
    assert!(
        result
            .reports()
            .values()
            .all(|report| report.validation.is_valid())
    );
    assert!(
        result.placement.spatial_budget.world_error_bound().unwrap()
            <= request.options.spatial_error
    );
}

fn count_chart_polls(request: &WrappedNormalRequest) -> (WrappedChartNormalResult, usize) {
    let mut total = 0;
    let result = normal_extrude_profile_on_chart_with_cancel(request, || {
        total += 1;
        false
    })
    .unwrap();
    (result, total)
}

fn assert_chart_material_work(result: &WrappedChartNormalResult, total: usize) {
    assert!(total > 1);
    assert_eq!(result.material_work(), result.base.material_work());
    assert_eq!(
        result.material_relations().collect::<Vec<_>>(),
        result.base.material_relations().collect::<Vec<_>>()
    );
}

#[test]
fn attachment_handoff_keeps_the_frozen_chart_and_semantic_keys_under_input_permutation() {
    let mut request = super::super::super::operation_tests::request();
    request.support.origin = [1.25, -2.5, 3.75];
    request.support.chart_origin = -0.3;
    let reference = normal_extrude_profile_on_chart(&request).unwrap();
    request.profile.vertices.reverse();
    request.profile.curves.reverse();
    request.selections.reverse();
    let (result, graph) = qualify_attachment_chart(&request);
    let live_root = graph.root.clone();
    let operand = result.attachment_operand(graph).unwrap();

    assert_eq!(result.encoded_brep(), reference.encoded_brep());
    assert_eq!(result.history(), reference.history());
    assert_eq!(result.placement(), reference.placement());
    assert!(operand.bodies.is_same(&live_root));
    assert_attachment_snapshot(&result, &operand);
}

#[test]
fn attachment_handoff_restores_the_frozen_graph_if_native_bits_changed() {
    let request = super::super::super::operation_tests::request();
    let (result, graph) = qualify_attachment_chart(&request);
    let live_root = graph.root.clone();
    let vertex = graph
        .bindings
        .iter()
        .find_map(|(_, shape)| martensite_tkbrep::topods::vertex(shape).ok())
        .unwrap();
    martensite_tkbrep::brep::BRepBuilder::default()
        .update_vertex_point(&vertex, Point3::new(100., 100., 100.), 0.)
        .unwrap();
    let operand = result.attachment_operand(graph).unwrap();
    assert!(!operand.bodies.is_same(&live_root));
    assert_attachment_snapshot(&result, &operand);
}

fn qualify_attachment_chart(
    request: &WrappedNormalRequest,
) -> (WrappedChartNormalResult, ChartGraph) {
    let mut never = || false;
    let inputs = input::read_canonical_chart(request, &mut never).unwrap();
    qualify_chart(request, inputs, &mut never).unwrap()
}

fn assert_attachment_snapshot(result: &WrappedChartNormalResult, operand: &DetachedWrappedBodies) {
    let keys = &result.base.family.family.keys;
    let bindings = &result.base.history.bindings;
    let typed: Vec<_> = keys
        .iter()
        .zip(bindings)
        .map(|(key, name)| (*key, operand.entities[name].clone()))
        .collect();
    assert_eq!(operand.entities.len(), typed.len());
    assert_eq!(
        encoding::encode(&operand.bodies, &typed).unwrap(),
        result.encoded_brep()
    );
}
