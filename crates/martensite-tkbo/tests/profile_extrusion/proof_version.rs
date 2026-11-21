// SPDX-License-Identifier: GPL-2.0-only
//! Issue #539: explicit new exact evidence retains old public behavior and real graph holes.
use super::fixtures::*;
use martensite_tkbo::profile_extrusion::*;

pub(super) fn monotone_triangle() -> SketchProfile {
    let x = [
        0,
        4599676419421066576,
        4604180019048437080,
        4607182418800017408,
    ]
    .map(f64::from_bits);
    let poles = x.map(|value| [value, 0.]);
    let knots = [0., 0., 0., 0., 1., 1., 1., 1.];
    SketchFixture::new()
        .vertex(1, [0., 0.])
        .vertex(2, [1., 0.])
        .vertex(3, [0., 1.])
        .spline(11, [1, 2], 3, &poles, None, &knots)
        .line(12, 2, 3)
        .line(13, 3, 1)
        .build()
}
#[test]
fn exact_graph_route_completes_the_monotone_cubic_triangle() {
    let profile = monotone_triangle();
    let result = build_planar_regions_with_proof_and_cancel(
        &profile,
        resolution(),
        PlanarProofWorkLimit::PairBisections(4096),
        PlanarProofVersion::BernsteinV3,
        || false,
    )
    .unwrap();
    assert_eq!(result.proof_version(), PlanarProofVersion::BernsteinV3);
    assert_eq!(result.regions().len(), 1);
    assert!(result.regions()[0].area().encloses(0.5));
    assert_eq!(result.certificate().pair_domains, 6);
    assert_eq!(result.curve_edges().len(), 3);
}
#[test]
fn explicit_carrier_route_retains_existing_polls_and_work() {
    let profile = rectangle();
    let (old, old_polls) = tracked_carrier(&profile, false);
    let (explicit, explicit_polls) = tracked_carrier(&profile, true);
    assert_eq!(old.proof_version(), PlanarProofVersion::CarrierV2);
    assert_eq!(old_polls, explicit_polls);
    assert_eq!(old.proof_work(), explicit.proof_work());
    assert_eq!(old.certificate(), explicit.certificate());
    assert_eq!(old.regions()[0].area(), explicit.regions()[0].area());
}
fn tracked_carrier(profile: &SketchProfile, explicit: bool) -> (PlanarRegionSet, usize) {
    let requested = PlanarProofWorkLimit::PairBisections(4096);
    let mut polls = 0;
    let cancelled = || {
        polls += 1;
        false
    };
    let result = if explicit {
        build_planar_regions_with_proof_and_cancel(
            profile,
            resolution(),
            requested,
            PlanarProofVersion::CarrierV2,
            cancelled,
        )
    } else {
        build_planar_regions_with_work_and_cancel(profile, resolution(), requested, cancelled)
    };
    (result.unwrap(), polls)
}

#[test]
fn exact_graph_route_consumes_real_outer_and_hole_domains() {
    let result = build_planar_regions_with_proof_and_cancel(
        &annulus(),
        resolution(),
        PlanarProofWorkLimit::PairBisections(8192),
        PlanarProofVersion::BernsteinV3,
        || false,
    )
    .unwrap();
    assert_eq!(result.proof_version(), PlanarProofVersion::BernsteinV3);
    assert_eq!(result.regions().len(), 2);
    assert_eq!(
        result
            .regions()
            .iter()
            .map(|region| region.holes().len())
            .sum::<usize>(),
        1
    );
    assert_eq!(result.certificate().components, 2);
    assert!(result.proof_work().consumed_pair_bisections <= 8192);
}
#[test]
fn new_proof_selector_remains_owned_send_and_sync() {
    static_assertions::assert_impl_all!(PlanarProofVersion: Send, Sync, Copy);
}
