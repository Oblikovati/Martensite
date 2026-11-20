//! Independent world-coordinate, topology and replay checks for a placed holed cone.
use super::*;
use crate::surface_sketch::{HistoryRole, NormalSense};
use martensite_cmath::{atan2, cos, hypot, sin};
use martensite_tkbo::body_check::{CheckMode, ValidationStage};
use martensite_tkbo::profile_extrusion::PlanarProofVersion;
use martensite_tkbrep::{brep::BRepTool, topods};
use martensite_tkmath::gp::Point3;
use std::fmt::Write;

const DIRECTED: WrappedConeExecutionSelection =
    WrappedConeExecutionSelection::DirectedRecenteredChartRequireCertifiedV3 {
        arithmetic: WrappedConeArithmeticGrantPolicy::OriginalV1,
    };
const WORKER: &str = "surface_sketch::wrapped_profile::api::placement::cone::tests::process_worker";
const PREFIX: &str = "PLACED_CONE=";

#[test]
fn oblique_caps_keep_the_hole_and_world_vertices_and_replay_across_processes() {
    let request = fixture(NormalSense::Outward);
    let original = request.clone();
    let result = complete(&request);
    assert_eq!(request, original);
    verify(&request, &result);
    assert_eq!(signature(&result), child_signature());
}
#[test]
fn inward_oblique_cone_preserves_caller_depth_roles() {
    let request = fixture(NormalSense::Inward);
    verify(&request, &complete(&request));
}
#[test]
fn the_placed_atlas_retains_both_separate_chart_receipts() {
    let mut request = operation_cone::tests::atlas_request();
    apply_frame(&mut request, NormalSense::Outward);
    let selection = WrappedConeExecutionSelection::DirectedRegionAtlasRequireCertifiedV4 {
        arithmetic: WrappedConeArithmeticGrantPolicy::OriginalV1,
    };
    let result = normal_extrude_cone_profile_on_chart(&request, selection).unwrap();
    assert_eq!(result.placement().members.len(), 2);
    for (region, report) in &result.placement().members {
        assert!(
            result
                .canonical()
                .region_chart_domain_receipt(*region)
                .is_some()
        );
        assert!(report.validation.is_valid());
    }
    assert_vertices(&request, &result.export_native().unwrap());
}
#[test]
fn translation_that_exhausts_spatial_budget_returns_no_world_shape() {
    let mut request = fixture(NormalSense::Outward);
    request.support.apex = [1e15, -1e15, 1e15];
    let refusal = normal_extrude_cone_profile_on_chart(&request, DIRECTED)
        .err()
        .unwrap();
    assert_eq!(refusal.stage(), WrappedStage::Placement, "{refusal:?}");
    assert_eq!(
        refusal.class(),
        WrappedFailureClass::Incomplete,
        "{refusal:?}"
    );
}
#[test]
fn canonical_entry_keeps_its_admission_and_placed_entry_cancels() {
    let request = fixture(NormalSense::Outward);
    let refusal = normal_extrude_cone_profile_with_selection(&request, DIRECTED)
        .err()
        .unwrap();
    assert_eq!(refusal.class(), WrappedFailureClass::Unsupported);
    let stopped = normal_extrude_cone_profile_on_chart_with_cancel(&request, DIRECTED, || true)
        .err()
        .unwrap();
    assert_eq!(stopped.class(), WrappedFailureClass::Cancelled);
}
#[test]
#[ignore = "subprocess control for placed cone replay"]
fn process_worker() {
    println!(
        "{PREFIX}{}",
        signature(&complete(&fixture(NormalSense::Outward)))
    );
}

fn fixture(normal: NormalSense) -> WrappedConeNormalRequest {
    let mut request = operation_cone::tests::request();
    apply_frame(&mut request, normal);
    request.proof.planar_proof = PlanarProofVersion::GeometricCompositeV6 {
        max_components: 64,
        max_total_subintervals: 100_000,
    };
    request
}
fn apply_frame(request: &mut WrappedConeNormalRequest, normal: NormalSense) {
    request.support.apex = [3., -5., 7.];
    request.support.axis = [0., 3., 4.];
    request.support.radial = [2., 0., 0.];
    request.support.chart_origin = 0.4;
    request.support.normal = normal;
}
fn complete(request: &WrappedConeNormalRequest) -> WrappedPlacedConeResult {
    normal_extrude_cone_profile_on_chart(request, DIRECTED)
        .unwrap_or_else(|cause| panic!("placed cone refused: {cause:?}"))
}
fn verify(request: &WrappedConeNormalRequest, result: &WrappedPlacedConeResult) {
    fn shared<T: Send + Sync>() {}
    shared::<WrappedPlacedConeResult>();
    shared::<WrappedConePlacementReport>();
    shared::<WrappedPlacedConeMemberReport>();
    assert_eq!(result.support(), request.support);
    assert_eq!(result.options(), request.options);
    assert_eq!(result.canonical().depths(), request.depths);
    assert_eq!(result.boundary_flux_origin(), request.support.apex);
    check_bounds(request, result);
    let first = result.export_native().unwrap();
    let second = result.export_native().unwrap();
    assert!(!first.bodies.is_partner(&second.bodies));
    assert_eq!(
        first.entities.keys().collect::<Vec<_>>(),
        second.entities.keys().collect::<Vec<_>>()
    );
    assert_vertices(request, &first);
}
fn assert_vertices(request: &WrappedConeNormalRequest, graph: &DetachedWrappedBodies) {
    let checked = graph
        .entities
        .iter()
        .map(|(key, shape)| check_vertex(request, key, shape))
        .sum::<usize>();
    assert_eq!(checked, request.profile.vertices.len() * 2);
}
fn check_bounds(request: &WrappedConeNormalRequest, result: &WrappedPlacedConeResult) {
    let id = SurfaceRegionId(1);
    let placement = result.placement();
    let canonical = &result.canonical().reports()[&id];
    let world = &placement.members[&id];
    assert_eq!(canonical.coverage.euler_characteristic, 0);
    assert_eq!(
        canonical
            .validation
            .stage(ValidationStage::SelfIntersection)
            .mode,
        CheckMode::Exact
    );
    assert!(world.validation.is_valid(), "{:?}", world.validation);
    let epsilon = request.options.spatial_error;
    assert!(placement.achieved_spatial_error.upper <= epsilon);
    assert!(world.cap_trace_error.upper <= epsilon);
    assert!(world.cap_realization_error.upper <= epsilon);
    assert!(world.boundary_flux.lower > 0.);
    assert!(world.native_volume.value > 0.);
}
fn check_vertex(
    request: &WrappedConeNormalRequest,
    key: &WrappedHistoryKey,
    shape: &Shape,
) -> usize {
    let WrappedEntitySource::Vertex(id) = key.source else {
        return 0;
    };
    let d = match key.role {
        HistoryRole::FirstDepthVertex => request.depths[0],
        HistoryRole::LastDepthVertex => request.depths[1],
        _ => return 0,
    };
    let d = if request.support.normal == NormalSense::Inward {
        -d
    } else {
        d
    };
    let vertex = request
        .profile
        .vertices
        .iter()
        .find(|vertex| vertex.id.0 == id.0)
        .unwrap();
    let target = position(request.support, vertex.point, d);
    let stored = BRepTool::pnt(&topods::vertex(shape).unwrap()).unwrap();
    assert!(stored.distance(&target) <= request.options.spatial_error);
    1
}
fn position(support: ConeSupport, [x, y]: [f64; 2], depth: f64) -> Point3 {
    let (s, c) = (sin(support.semi_angle), cos(support.semi_angle));
    let u = atan2(y, x) / s + support.chart_origin;
    let slant = hypot(x, y);
    let r = slant * s + depth * c;
    let z = slant * c - depth * s;
    let y = r * sin(u);
    let [a, b, c] = support.apex;
    Point3::new(a + r * cos(u), b + 0.8 * y + 0.6 * z, c - 0.6 * y + 0.8 * z)
}
fn signature(result: &WrappedPlacedConeResult) -> String {
    let mut signature = format!(
        "{:?}|{:?}|",
        result.canonical().history(),
        result.placement()
    );
    for byte in result.encoded_brep() {
        write!(signature, "{byte:02x}").unwrap();
    }
    signature
}
fn child_signature() -> String {
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", WORKER, "--ignored", "--nocapture"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .find_map(|line| line.strip_prefix(PREFIX))
        .unwrap()
        .to_owned()
}
