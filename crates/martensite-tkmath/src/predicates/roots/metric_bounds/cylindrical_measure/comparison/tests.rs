// SPDX-License-Identifier: GPL-2.0-only
use super::super::super::{
    CylindricalProfileInterval, MetricBezier, certify_cylindrical_boundary_flux,
    certify_cylindrical_profile,
};
use super::*;
const AUTHORED: [[f64; 2]; 2] = [[0., 0.], [0., 1.]];
const FIRST: [[f64; 3]; 2] = [[1., 0., 0.], [1., 0., 1.]];
const LAST: [[f64; 3]; 2] = [[2., 0., 0.], [2., 0., 1.]];
const VERTICAL: [[f64; 2]; 2] = [[0., 0.], [0., 1.]];
const HORIZONTAL: [[f64; 2]; 2] = [[0., 1.], [1., 1.]];
fn span(cap: &'static [[f64; 2]]) -> CylindricalFluxSpan<'static> {
    CylindricalFluxSpan {
        boundaries: [&FIRST[..], &LAST[..]].map(|poles| MetricBezier {
            poles,
            weights: None,
        }),
        cap_pcurves: [cap; 2],
        side_reversed: false,
    }
}
fn profile(height_shift: f64) -> super::super::super::CertifiedCylindricalProfile {
    let poles = FIRST.map(|[x, y, z]| [x, y, z + height_shift]);
    certificate(&AUTHORED, &poles)
}
fn certificate(
    authored: &[[f64; 2]],
    poles: &[[f64; 3]],
) -> super::super::super::CertifiedCylindricalProfile {
    certify_cylindrical_profile(
        CylindricalProfileQuery {
            authored,
            stored: MetricBezier {
                poles,
                weights: None,
            },
            interval: CylindricalProfileInterval::new(0, 0).unwrap(),
            support_radius: 1.,
            stored_radius: 1.,
            angular_origin: 0.,
            spatial_limit: 0.5,
            options: Default::default(),
        },
        || false,
    )
    .unwrap()
}
fn source(
    profile: &super::super::super::CertifiedCylindricalProfile,
) -> CylindricalProfileSource<'_> {
    source_for(&AUTHORED, profile)
}
fn source_for<'a>(
    authored: &'a [[f64; 2]],
    profile: &'a super::super::super::CertifiedCylindricalProfile,
) -> CylindricalProfileSource<'a> {
    CylindricalProfileSource {
        authored,
        interval: CylindricalProfileInterval::new(0, 0).unwrap(),
        support_radius: 1.,
        angular_origin: 0.,
        certificate: profile,
    }
}
fn query<'a>(
    sources: &'a [CylindricalProfileSource<'a>],
    spans: &'a [CylindricalFluxSpan<'a>],
) -> CylindricalMeasureComparisonQuery<'a> {
    CylindricalMeasureComparisonQuery {
        flux_source: CylindricalFluxQuery {
            spans,
            radii: [1., 2.],
            options: Default::default(),
        },
        normal_source: CylindricalMeasureQuery {
            sources,
            depths: [0., 1.],
            limit: CylindricalMeasureLimit::Report,
            options: Default::default(),
        },
        limit: CylindricalMeasureLimit::Report,
        options: Default::default(),
    }
}
fn completed(
    query: CylindricalMeasureComparisonQuery<'_>,
) -> (CertifiedCylindricalFlux, CertifiedCylindricalNormalMeasure) {
    (
        certify_cylindrical_boundary_flux(query.flux_source, || false).unwrap(),
        certify_cylindrical_normal_measure(query.normal_source, || false).unwrap(),
    )
}
#[test]
fn zero_flux_and_zero_ideal_have_an_exact_zero_comparison() {
    let p = profile(0.);
    let sources = [source(&p)];
    let spans = [span(&VERTICAL)];
    let q = query(&sources, &spans);
    let (flux, normal) = completed(q);
    let proof = compare_cylindrical_measures(flux, normal, q, || false).unwrap();
    assert_eq!(proof.difference_enclosure(), [0.; 2]);
    assert_eq!(proof.discrepancy_bound(), 0.);
    assert_eq!(proof.representative_error_bound(), 0.);
    assert!(proof.matches_sources(q));
    assert_eq!(proof.evidence(), CylindricalMeasureEvidence::CertifiedBound);
    assert_eq!(proof.requested_options(), q.options);
    assert_eq!(proof.requested_limit(), q.limit);
}
#[test]
fn independent_cap_flux_gives_a_half_unit_difference_without_validating_a_body() {
    let p = profile(0.);
    let sources = [source(&p)];
    let spans = [span(&HORIZONTAL)];
    let q = query(&sources, &spans);
    let (flux, normal) = completed(q);
    let proof = compare_cylindrical_measures(flux, normal, q, || false).unwrap();
    assert_eq!(proof.difference_enclosure(), [-0.5; 2]);
    assert_eq!(proof.discrepancy_bound(), 0.5);
    assert_eq!(proof.representative_error_bound(), 0.5);
    assert_eq!(proof.flux_certificate().absolute_error_bound(), 0.);
    assert_eq!(proof.normal_certificate().volume_discrepancy_bound(), 0.);
}
#[test]
fn unchanged_allowance_is_applied_to_both_comparison_bounds() {
    let p = profile(0.);
    let sources = [source(&p)];
    let spans = [span(&HORIZONTAL)];
    let mut q = query(&sources, &spans);
    let (flux, normal) = completed(q);
    q.limit = CylindricalMeasureLimit::Refuse(0.25);
    let error = compare_cylindrical_measures(flux, normal, q, || false).unwrap_err();
    assert_eq!(error.requested_limit, q.limit);
    assert_eq!(
        error.cause,
        CylindricalMeasureComparisonFailure::Uncertified {
            bound: 0.5,
            requested: 0.25
        }
    );
}
#[test]
fn changed_cap_source_refuses_the_old_flux_binding() {
    let p = profile(0.);
    let sources = [source(&p)];
    let spans = [span(&VERTICAL)];
    let changed = [span(&HORIZONTAL)];
    let q = query(&sources, &spans);
    let (flux, normal) = completed(q);
    let error = compare_cylindrical_measures(flux, normal, query(&sources, &changed), || false)
        .unwrap_err();
    assert_eq!(
        error.cause,
        CylindricalMeasureComparisonFailure::FluxBinding
    );
}
#[test]
fn changed_depth_refuses_the_old_normal_binding() {
    let p = profile(0.);
    let sources = [source(&p)];
    let spans = [span(&VERTICAL)];
    let mut q = query(&sources, &spans);
    let (flux, normal) = completed(q);
    q.normal_source.depths[1] = 2.;
    let error = compare_cylindrical_measures(flux, normal, q, || false).unwrap_err();
    assert_eq!(
        error.cause,
        CylindricalMeasureComparisonFailure::NormalBinding
    );
}
#[test]
fn individually_valid_measures_cannot_bind_different_retained_first_curves() {
    let p = profile(0.125);
    let sources = [source(&p)];
    let spans = [span(&VERTICAL)];
    let q = query(&sources, &spans);
    let (flux, normal) = completed(q);
    let error = compare_cylindrical_measures(flux, normal, q, || false).unwrap_err();
    assert_eq!(
        error.cause,
        CylindricalMeasureComparisonFailure::SpanBinding { index: 0 }
    );
}
#[test]
fn unequal_complete_occurrence_counts_are_refused() {
    let p = profile(0.);
    let sources = [source(&p); 2];
    let spans = [span(&VERTICAL)];
    let q = query(&sources, &spans);
    let (flux, normal) = completed(q);
    let error = compare_cylindrical_measures(flux, normal, q, || false).unwrap_err();
    assert_eq!(error.cause, CylindricalMeasureComparisonFailure::Count);
}
#[test]
fn cancellation_and_exhausted_comparison_work_do_not_publish_partial_bounds() {
    let p = profile(0.);
    let sources = [source(&p)];
    let spans = [span(&VERTICAL)];
    let mut q = query(&sources, &spans);
    let (flux, normal) = completed(q);
    let proof = compare_cylindrical_measures(flux.clone(), normal.clone(), q, || false).unwrap();
    let error = compare_cylindrical_measures(flux.clone(), normal.clone(), q, || true).unwrap_err();
    assert!(matches!(
        error.cause,
        CylindricalMeasureComparisonFailure::Arithmetic(_)
    ));
    q.options.max_operations = proof.operations() - 1;
    let error = compare_cylindrical_measures(flux, normal, q, || false).unwrap_err();
    assert!(matches!(
        error.cause,
        CylindricalMeasureComparisonFailure::Arithmetic(_)
    ));
}
#[test]
fn comparison_public_types_are_send_and_sync() {
    fn check<T: Send + Sync>() {}
    check::<CylindricalMeasureComparisonQuery<'static>>();
    check::<CertifiedCylindricalMeasureComparison>();
    check::<CylindricalMeasureComparisonError>();
    check::<CylindricalMeasureComparisonFailure>();
}

#[test]
fn ideal_profile_uncertainty_is_preserved_in_both_comparison_bounds() {
    let authored = [[0., 0.], [0.125, 1.]];
    let p = certificate(&authored, &FIRST);
    let sources = [source_for(&authored, &p)];
    let spans = [span(&VERTICAL)];
    let q = query(&sources, &spans);
    let (flux, normal) = completed(q);
    let proof = compare_cylindrical_measures(flux, normal, q, || false).unwrap();
    let [lo, hi] = proof.difference_enclosure();
    assert!(lo <= -0.28125 && hi >= 0.09375);
    assert!(proof.discrepancy_bound() >= 0.28125);
    assert!(proof.representative_error_bound() >= 0.28125);
}
#[test]
fn invalid_comparison_policy_and_limit_are_refused() {
    let p = profile(0.);
    let sources = [source(&p)];
    let spans = [span(&VERTICAL)];
    for (bits, limit) in [
        (7, CylindricalMeasureLimit::Report),
        (128, CylindricalMeasureLimit::Refuse(f64::INFINITY)),
    ] {
        let mut q = query(&sources, &spans);
        q.options.fractional_bits = bits;
        q.limit = limit;
        let (flux, normal) = completed(q);
        let error = compare_cylindrical_measures(flux, normal, q, || false).unwrap_err();
        assert_eq!(error.cause, CylindricalMeasureComparisonFailure::Policy);
    }
}
