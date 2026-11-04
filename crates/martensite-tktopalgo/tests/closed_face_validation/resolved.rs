//! Independent represented boundaries retain the exact replay refusal and raw limits.
use super::{check, fixture};
use martensite_tkbrep::brep::{BRepBuilder, face_input_capture::*};
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::face_validation::*;
fn capture(shape: &fixture::StripFixture) -> FaceInputSnapshot {
    capture_closed_extrusion_face(&shape.face, FaceCaptureLimits::bounded(), || false).unwrap()
}
fn run(input: FaceInputSnapshot) -> ToleranceResolvedFaceReport {
    certify_tolerance_resolved_extrusion_face(input, ToleranceResolvedFacePolicy::v4(), || false)
}
fn actual_carrier(tolerance: f64, points: [[f64; 2]; 9]) -> fixture::StripFixture {
    let shape = fixture::strip_with_carrier(tolerance, points);
    for (index, z) in [-1., 2.].into_iter().enumerate() {
        BRepBuilder::default()
            .update_vertex_point(
                &shape.vertices[index],
                Point3::new(points[0][0], points[0][1], z),
                tolerance,
            )
            .unwrap();
    }
    shape
}
fn shifted() -> fixture::StripFixture {
    let delta = 1. / 1_073_741_824.;
    actual_carrier(1e-7, fixture::ROUND_NET.map(|[x, y]| [x + delta, y]))
}
#[test]
fn explicit_mode_retains_metric_gap_without_reinterpreting_exact_replay() {
    let shape = shifted();
    assert!(
        check(&shape, ClosedFacePolicy::placed_piecewise_boundary())
            .certificate()
            .is_none()
    );
    let report = run(capture(&shape));
    assert!(report.certificate().is_some(), "{:?}", report.completion());
    assert_eq!(report.certificate().unwrap().behavior_version(), 4);
    assert_eq!(report.incidences().len(), 4);
    assert!(
        report
            .error_bounds()
            .iter()
            .any(|b| b.squared_upper.compare_f64(0.).unwrap() == std::cmp::Ordering::Greater)
    );
}
#[test]
fn exact_closed_strip_also_needs_all_proper_component_witnesses() {
    let report = run(capture(&fixture::strip(1e-7, 0.)));
    assert!(report.certificate().is_some(), "{:?}", report.completion());
    assert!(
        report
            .incidences()
            .iter()
            .all(|p| p.proof.source().certificate().is_some())
    );
    assert_eq!(report.history(), ClosedFaceHistory::NoGeometryChanges);
}
#[test]
fn whole_loop_inside_a_vertex_ball_is_not_tolerance_resolved() {
    let shape = actual_carrier(100., fixture::ROUND_NET);
    let report = run(capture(&shape));
    assert!(report.certificate().is_none());
    assert!(matches!(
        report.completion(),
        ToleranceResolvedFaceCompletion::Neighborhood { .. }
    ));
}
#[test]
fn folded_and_collapsed_actual_carriers_remain_uncertified() {
    let collapsed = [[1., 0.]; 9];
    let mut folded = fixture::ROUND_NET;
    folded[2] = [0., -2.];
    folded[6] = [0., 2.];
    for net in [collapsed, folded] {
        let report = run(capture(&actual_carrier(100., net)));
        assert!(report.certificate().is_none(), "{:?}", report.completion());
    }
}
#[test]
fn beyond_raw_metric_limits_refuses_without_a_neighborhood_attempt() {
    let shape = actual_carrier(1e-7, fixture::ROUND_NET.map(|[x, y]| [x + 1e-3, y]));
    let report = run(capture(&shape));
    assert!(report.certificate().is_none());
    assert!(!report.coverage().geometry);
    assert_eq!(report.incidence_work(), 0);
}
#[test]
fn exhausted_incidence_work_and_cancellation_are_not_validity() {
    let input = capture(&shifted());
    let mut policy = ToleranceResolvedFacePolicy::v4();
    policy.max_incidence_work = 0;
    let report = certify_tolerance_resolved_extrusion_face(input.clone(), policy, || false);
    assert!(report.certificate().is_none());
    let cancelled = certify_tolerance_resolved_extrusion_face(input, policy, || true);
    assert!(cancelled.certificate().is_none());
    assert!(!cancelled.coverage().topology);
}
#[test]
fn incomplete_support_proof_cannot_publish_incidence_success() {
    let mut policy = ToleranceResolvedFacePolicy::v4();
    policy.face.support.max_obligations = 1;
    let report = certify_tolerance_resolved_extrusion_face(capture(&shifted()), policy, || false);
    assert!(report.certificate().is_none());
    assert!(!report.coverage().support);
}
#[test]
fn immutable_input_isolated_from_later_live_vertex_mutation() {
    let shape = shifted();
    let input = capture(&shape);
    let before = format!("{input:?}");
    BRepBuilder::default()
        .update_vertex_point(&shape.vertices[0], Point3::new(9., 9., 9.), 1e-7)
        .unwrap();
    let report = run(input);
    assert!(report.certificate().is_some(), "{:?}", report.completion());
    assert_eq!(format!("{:?}", report.input()), before);
}
#[test]
fn published_reports_are_send_sync_and_thread_deterministic() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<ToleranceResolvedFaceReport>();
    let input = capture(&shifted());
    let other = input.clone();
    let first = run(input);
    let second = std::thread::spawn(move || run(other)).join().unwrap();
    assert_eq!(format!("{first:?}"), format!("{second:?}"));
    assert!(first.certificate().is_some());
}
#[test]
fn print_owned_tolerance_resolved_report_for_process_comparison() {
    let report = run(capture(&shifted()));
    assert!(report.certificate().is_some(), "{:?}", report.completion());
    println!("RESOLVED_FACE_REPORT={report:?}");
}

#[test]
fn largest_declared_incidence_budget_is_not_narrowed_by_a_cast() {
    let mut policy = ToleranceResolvedFacePolicy::v4();
    policy.max_incidence_work = u32::MAX;
    let report = certify_tolerance_resolved_extrusion_face(capture(&shifted()), policy, || false);
    assert_eq!(report.policy().max_incidence_work, u32::MAX);
    assert!(report.certificate().is_some(), "{:?}", report.completion());
}
