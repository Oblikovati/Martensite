// SPDX-License-Identifier: GPL-2.0-only
//! #544: public version four retains real graph faces and the earlier selector.
use super::fixtures::*;
use super::proof_version::monotone_triangle;
use martensite_tkbo::profile_extrusion::*;
fn filtered(profile: &SketchProfile) -> PlanarRegionSet {
    build_planar_regions_with_proof_and_cancel(
        profile,
        resolution(),
        PlanarProofWorkLimit::PairBisections(8192),
        PlanarProofVersion::FilteredBernsteinV4,
        || false,
    )
    .unwrap()
}
#[test]
fn filtered_selector_publishes_actual_native_triangle_and_earlier_version_stays_available() {
    let profile = monotone_triangle();
    let result = filtered(&profile);
    assert_eq!(
        result.proof_version(),
        PlanarProofVersion::FilteredBernsteinV4
    );
    assert_eq!(result.regions().len(), 1);
    assert!(result.regions()[0].area().encloses(0.5));
    assert_eq!(result.certificate().pair_domains, 6);
    assert_eq!(result.curve_edges().len(), 3);
    let old = earlier(&profile);
    assert_eq!(old.proof_version(), PlanarProofVersion::BernsteinV3);
    assert_eq!(old.certificate(), result.certificate());
    assert_eq!(old.regions()[0].area(), result.regions()[0].area());
}
fn earlier(profile: &SketchProfile) -> PlanarRegionSet {
    build_planar_regions_with_proof_and_cancel(
        profile,
        resolution(),
        PlanarProofWorkLimit::PairBisections(8192),
        PlanarProofVersion::BernsteinV3,
        || false,
    )
    .unwrap()
}
#[test]
fn filtered_selector_consumes_every_real_annular_graph_domain() {
    let result = filtered(&annulus());
    assert_eq!(
        result.proof_version(),
        PlanarProofVersion::FilteredBernsteinV4
    );
    assert_eq!(result.regions().len(), 2);
    assert_eq!(
        result
            .regions()
            .iter()
            .map(|r| r.holes().len())
            .sum::<usize>(),
        1
    );
    assert_eq!(result.certificate().components, 2);
    assert!(result.proof_work().consumed_pair_bisections <= 8192);
}
