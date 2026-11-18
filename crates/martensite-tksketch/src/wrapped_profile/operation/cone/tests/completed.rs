//! Whole-operation controls combine independent inequalities, ownership and replay.
use super::*;
use std::fmt::Write;
mod concurrency;
mod native_flux;
pub(super) mod normals;
mod occurrences;
mod policies;
const WORKER: &str =
    "surface_sketch::wrapped_profile::operation::cone::tests::completed::process_worker";
const PREFIX: &str = "CONE_COMPLETE_REPLAY=";
#[test]
fn curved_holed_cone_completes_with_independent_measure_bounds_and_owned_exports() {
    let result = check_completed(request());
    policies::assert_mode(
        &result,
        martensite_tkbo::profile_extrusion::PlanarProofVersion::FilteredBernsteinV4,
    );
}
#[test]
fn explicit_v3_curved_holed_cone_completes_with_independent_bounds_and_owned_exports() {
    policies::accepted(martensite_tkbo::profile_extrusion::PlanarProofVersion::BernsteinV3);
}
#[test]
fn inward_curved_holed_cone_completes_with_independent_bounds_and_caller_history() {
    let mut request = request();
    request.support.normal = NormalSense::Inward;
    assert_eq!(request.depths, [-0.1, 0.2]);
    assert_eq!(request.support.semi_angle, 0.7);
    check_completed(request);
}
fn check_completed(mut request: WrappedConeNormalRequest) -> WrappedConeNormalResult {
    let before = request.clone();
    let started = std::time::Instant::now();
    let result = crate::surface_sketch::normal_extrude_cone_profile(&request)
        .unwrap_or_else(|cause| panic!("{cause:?}"));
    eprintln!(
        r#"{{"cone_execute_seconds":{:.6}}}"#,
        started.elapsed().as_secs_f64()
    );
    publication::assert_publication(&result, &request);
    assert_eq!(request, before);
    assert_eq!(result.proof_options(), before.proof);
    assert_eq!(result.support(), before.support);
    check_measure(&result);
    check_coverage(&result);
    check_export_isolation(&result, &mut request, &before);
    result
}
fn check_export_isolation(
    result: &WrappedConeNormalResult,
    request: &mut WrappedConeNormalRequest,
    original: &WrappedConeNormalRequest,
) {
    let bytes = result.encoded_brep().to_vec();
    request.profile.vertices.clear();
    request.profile.curves.clear();
    let first = result.export_native().unwrap();
    normals::check(result, &first);
    occurrences::check(result, &first);
    check_depth_history(original, &first);
    let second = result.export_native().unwrap();
    assert!(!first.bodies.is_same(&second.bodies));
    assert_eq!(result.encoded_brep(), bytes);
    assert!(!result.history().is_empty());
}

fn check_measure(result: &WrappedConeNormalResult) {
    let report = &result.reports()[&SurfaceRegionId(1)];
    // Exact Green integration of the authored cubic adds 21/400 to the unit square.
    let area = 321. / 400.;
    assert!(report.planar_area.encloses(area));
    let [lower, upper] = independent_volume_bounds(area, result.support().normal);
    assert!(report.authored_volume.lower >= lower && report.authored_volume.upper <= upper);
    assert!(
        report.boundary_flux.lower > 0. && report.boundary_flux.upper >= report.boundary_flux.lower,
        "{}",
        measure_diagnostic(result)
    );
}
fn independent_volume_bounds(area: f64, normal: NormalSense) -> [f64; 2] {
    let correction = 0.015 * martensite_cmath::cos(0.7) / martensite_cmath::sin(0.7);
    let correction = match normal {
        NormalSense::Outward => correction,
        NormalSense::Inward => -correction,
    };
    // The sign reverses which slant extremum supplies the lower volume bound.
    let endpoints = [correction * area / 26.21_f64.sqrt(), correction * area / 4.];
    [
        area * 0.3 + endpoints[0].min(endpoints[1]),
        area * 0.3 + endpoints[0].max(endpoints[1]),
    ]
}
fn check_depth_history(original: &WrappedConeNormalRequest, detached: &DetachedWrappedBodies) {
    if original.support.normal != NormalSense::Inward {
        return;
    }
    let vertices: usize = detached
        .entities
        .iter()
        .map(|(key, shape)| super::inward::check_vertex(original, shape, *key))
        .sum();
    assert_eq!(vertices, original.profile.vertices.len() * 2);
}
fn measure_diagnostic(result: &WrappedConeNormalResult) -> String {
    let report = &result.reports()[&SurfaceRegionId(1)];
    format!(
        "boundary_flux={:?}; reference_volume={:?}; native_volume_estimate={:?}; requested_proof={:?}; {}",
        report.boundary_flux,
        report.reference_volume,
        report.native_volume,
        result.proof_options(),
        receipt_diagnostic(result)
    )
}
fn receipt_diagnostic(result: &WrappedConeNormalResult) -> String {
    let flux = result.flux_receipt(SurfaceRegionId(1)).unwrap();
    let reference = result.reference_volume_receipt(SurfaceRegionId(1)).unwrap();
    format!(
        "flux_evidence={:?},flux_absolute_error={},flux_options={:?}; reference_evidence={:?},reference_absolute_error={},reference_options={:?}",
        flux.evidence(),
        flux.absolute_error_bound(),
        flux.effective_options(),
        reference.evidence(),
        reference.absolute_error_bound(),
        reference.effective_options()
    )
}
fn check_coverage(result: &WrappedConeNormalResult) {
    let report = &result.reports()[&SurfaceRegionId(1)];
    let spans = report.coverage.faces - 2;
    assert_eq!(report.coverage.cap_pcurves, 2 * spans);
    assert_eq!(report.coverage.side_pcurves, 4 * spans);
    assert_eq!(report.coverage.euler_characteristic, 0);
    assert!(result.budget().achieved_spatial_error.upper <= result.options().spatial_error);
}
fn signature() -> String {
    let result = execute(&request(), &mut || false).unwrap_or_else(|cause| panic!("{cause:?}"));
    let mut signature = format!("{:?}|{:?}|", result.history(), result.budget());
    for byte in result.encoded_brep() {
        write!(signature, "{byte:02x}").unwrap();
    }
    signature
}
#[test]
fn complete_cone_thread_replay_preserves_bytes_and_owned_evidence() {
    let expected = signature();
    let actual = std::thread::spawn(signature).join().unwrap();
    assert_eq!(actual, expected);
}
#[test]
#[ignore = "subprocess worker invoked by complete cone process replay"]
fn process_worker() {
    println!("{PREFIX}{}", signature());
}
#[test]
fn complete_cone_two_processes_replay_bytes_and_evidence() {
    assert_eq!(process_signature(), process_signature());
}
fn process_signature() -> String {
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", WORKER, "--ignored", "--nocapture"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "child stderr={} stdout={}",
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );
    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .find_map(|line| line.strip_prefix(PREFIX))
        .unwrap()
        .to_owned()
}
