// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::metric_bounds::{
    CertifiedCylindricalProfile, CylindricalProfileQuery, CylindricalProfileSource, MetricBezier,
    certify_cylindrical_profile,
};

fn profiles(
    sources: &[CylindricalClearanceSource<'_>],
    radius: f64,
    origin: f64,
) -> Vec<CertifiedCylindricalProfile> {
    sources
        .iter()
        .map(|source| profile(source, radius, origin))
        .collect()
}

fn profile(
    source: &CylindricalClearanceSource<'_>,
    radius: f64,
    origin: f64,
) -> CertifiedCylindricalProfile {
    let poles: Vec<_> = source
        .authored
        .iter()
        .map(|[s, z]| [1., s / radius, *z])
        .collect();
    let stored = MetricBezier {
        poles: &poles,
        weights: None,
    };
    certify_cylindrical_profile(profile_query(source, stored, radius, origin), || false).unwrap()
}

fn profile_query<'a>(
    source: &CylindricalClearanceSource<'a>,
    stored: MetricBezier<'a, 3>,
    radius: f64,
    origin: f64,
) -> CylindricalProfileQuery<'a> {
    CylindricalProfileQuery {
        authored: source.authored,
        stored,
        interval: source.interval,
        support_radius: radius,
        stored_radius: 1.,
        angular_origin: origin,
        spatial_limit: 100_000.,
        options: Default::default(),
    }
}

fn bound_sources<'a>(
    sources: &[CylindricalClearanceSource<'a>],
    proofs: &'a [CertifiedCylindricalProfile],
    radius: f64,
    origin: f64,
) -> Vec<CylindricalProfileSource<'a>> {
    sources
        .iter()
        .zip(proofs)
        .map(|(source, certificate)| CylindricalProfileSource {
            authored: source.authored,
            interval: source.interval,
            support_radius: radius,
            angular_origin: origin,
            certificate,
        })
        .collect()
}

#[test]
fn preconstruction_clearance_binds_to_later_retained_profiles() {
    let sources = [source(&[[0., 0.], [0., 1.]]), source(&[[2., 0.], [2., 1.]])];
    let clearance = certify(&sources, 1.);
    let proofs = profiles(&sources, 1000., 0.);
    let bound = bound_sources(&sources, &proofs, 1000., 0.);
    assert!(clearance.matches_profiles(&bound));
    assert_eq!(clearance.support_radius(), 1000.);
    assert_eq!(clearance.angular_origin(), 0.);
    assert_eq!(
        clearance.requested_options(),
        CylindricalClearanceOptions::default()
    );
}

#[test]
fn a_different_valid_profile_chart_cannot_reuse_the_clearance_binding() {
    let sources = [source(&[[0., 0.], [0., 1.]]), source(&[[2., 0.], [2., 1.]])];
    let clearance = certify(&sources, 1.);
    let proofs = profiles(&sources, 2000., 0.);
    let bound = bound_sources(&sources, &proofs, 2000., 0.);
    assert!(!clearance.matches_profiles(&bound));
}

#[test]
fn original_source_matching_preserves_interval_and_signed_zero() {
    let sources = [source(&[[0., 0.], [0., 1.]]), source(&[[2., 0.], [2., 1.]])];
    let clearance = certify(&sources, 1.);
    let mut changed = sources;
    changed[0].authored = &[[-0., 0.], [0., 1.]];
    assert!(!clearance.matches_sources(&changed));
    changed = sources;
    changed[0].interval = CylindricalProfileInterval::new(0, 1).unwrap();
    assert!(!clearance.matches_sources(&changed));
}

#[test]
fn excluded_pair_accounting_and_source_order_are_explicit() {
    let curves: Vec<_> = (0..8)
        .map(|i| [[2. * i as f64, 0.], [2. * i as f64, 1.]])
        .collect();
    let mut sources: Vec<_> = curves.iter().map(|p| source(p)).collect();
    let mut input = query(&sources, 1.);
    input.domain = CylindricalClearanceDomain::Within {
        excluded: &[[0, 7], [1, 5], [3, 4]],
    };
    let clearance = certify_cylindrical_clearance(input, || false).unwrap();
    assert_eq!(clearance.pair_count(), 25);
    assert_eq!(clearance.pruned_pairs(), 25);
    sources.reverse();
    assert!(!clearance.matches_sources(&sources));
    assert_eq!(certify(&sources, 1.).pair_count(), 28);
}
