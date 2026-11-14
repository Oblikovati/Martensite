// SPDX-License-Identifier: GPL-2.0-only
//! Explicit work choices preserve geometric policy and legacy decline projection.
use super::fixtures::*;
use martensite_tkbo::profile_extrusion::*;
use std::error::Error;

#[test]
fn explicit_current_default_matches_the_existing_public_builder() {
    let profile = rectangle();
    let legacy = build_planar_regions(&profile, resolution()).unwrap();
    let explicit = build_planar_regions_with_work(
        &profile,
        resolution(),
        PlanarProofWorkLimit::CurrentDefault,
    )
    .unwrap();
    assert_eq!(legacy.certificate(), explicit.certificate());
    assert_eq!(legacy.proof_work(), explicit.proof_work());
    assert_eq!(legacy.regions()[0].area(), explicit.regions()[0].area());
    assert_eq!(explicit.resolution(), resolution());
}
#[test]
fn zero_work_completes_an_analytic_rectangle_without_changing_accuracy() {
    let requested = PlanarProofWorkLimit::PairBisections(0);
    let result = build_planar_regions_with_work(&rectangle(), resolution(), requested).unwrap();
    assert!(result.regions()[0].area().encloses(6.));
    assert_eq!(result.resolution(), resolution());
    assert_eq!(result.proof_work().requested_pair_bisections, 0);
    assert_eq!(result.proof_work().effective_pair_bisections, 0);
    assert_eq!(result.proof_work().consumed_pair_bisections, 0);
}
#[test]
fn admission_failure_keeps_requested_work_without_claiming_observation() {
    let profile = SketchFixture::new().build();
    let legacy = build_planar_regions(&profile, resolution()).unwrap_err();
    let requested = PlanarProofWorkLimit::PairBisections(7);
    let failure = build_planar_regions_with_work(&profile, resolution(), requested).unwrap_err();
    assert_eq!(failure.decline(), &legacy);
    assert_eq!(failure.requested_work(), requested);
    assert!(failure.proof_work().is_none());
    assert!(failure.pair_root_failure().is_none());
    assert!(failure.source().is_some());
    assert!(failure.to_string().contains("observed=None"));
}
#[test]
fn crossing_failure_retains_actual_work_and_the_existing_typed_cause() {
    let profile = bow_tie();
    let legacy = build_planar_regions(&profile, resolution()).unwrap_err();
    let requested = PlanarProofWorkLimit::PairBisections(8192);
    let failure = build_planar_regions_with_work(&profile, resolution(), requested).unwrap_err();
    assert_eq!(failure.decline(), &legacy);
    assert_eq!(failure.requested_work(), requested);
    assert!(failure.pair_root_failure().is_none());
    let work = failure.proof_work().unwrap();
    assert_eq!(work.requested_pair_bisections, 8192);
    assert_eq!(work.effective_pair_bisections, 8192);
    assert!(work.consumed_pair_bisections <= 8192);
}
#[test]
fn work_request_and_failure_are_owned_send_and_sync() {
    static_assertions::assert_impl_all!(PlanarProofWorkLimit: Send, Sync, Copy);
    static_assertions::assert_impl_all!(PlanarRegionWorkFailure: Send, Sync, Clone);
}

#[test]
fn insufficient_pair_work_refuses_without_relaxing_the_profile() {
    let profile = ellipse();
    let requested = PlanarProofWorkLimit::PairBisections(1);
    let failure = build_planar_regions_with_work(&profile, resolution(), requested).unwrap_err();
    assert_eq!(failure.requested_work(), requested);
    let observed = failure.proof_work().unwrap();
    assert_eq!(observed.effective_pair_bisections, 1);
    assert_eq!(observed.consumed_pair_bisections, 1);
    let root = failure.pair_root_failure().unwrap();
    assert_eq!(root.curves, [ProfileCurveId(3); 2]);
    assert!(matches!(
        root.cause,
        martensite_tkmath::predicates::roots::RootIsolationError::SubdivisionLimit { .. }
    ));
    let completed = build_planar_regions_with_work(
        &profile,
        resolution(),
        PlanarProofWorkLimit::PairBisections(8192),
    )
    .unwrap();
    assert!(completed.proof_work().consumed_pair_bisections > 0);
    assert_eq!(completed.resolution(), resolution());
    assert!(completed.regions()[0].area().encloses(std::f64::consts::PI));
}
