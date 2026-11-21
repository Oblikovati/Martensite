// SPDX-License-Identifier: GPL-2.0-only
use super::super::{
    CylindricalProfileInterval, CylindricalProfileQuery, MetricBezier, certify_cylindrical_profile,
};
use super::*;

struct Fixture {
    authored: Vec<Vec<[f64; 2]>>,
    profiles: Vec<super::super::CertifiedCylindricalProfile>,
}
impl Fixture {
    fn rectangle(width: f64, height: f64, height_shift: f64) -> Self {
        let points = [[0., 0.], [width, 0.], [width, height], [0., height]];
        let authored: Vec<_> = (0..4)
            .map(|i| vec![points[i], points[(i + 1) % 4]])
            .collect();
        let profiles = authored
            .iter()
            .map(|p| profile(p, height_shift, 1.))
            .collect();
        Self { authored, profiles }
    }
    fn sources(&self) -> Vec<CylindricalProfileSource<'_>> {
        self.authored
            .iter()
            .zip(&self.profiles)
            .map(|(p, certificate)| CylindricalProfileSource {
                authored: p,
                interval: CylindricalProfileInterval::new(0, 0).unwrap(),
                support_radius: 1.,
                angular_origin: 0.,
                certificate,
            })
            .collect()
    }
}
fn profile(
    authored: &[[f64; 2]],
    height_shift: f64,
    height_scale: f64,
) -> super::super::CertifiedCylindricalProfile {
    let poles: Vec<_> = authored
        .iter()
        .map(|p| [1., 0., height_scale * p[1] + height_shift])
        .collect();
    certify_cylindrical_profile(profile_query(authored, &poles), || false).unwrap()
}
fn profile_query<'a>(
    authored: &'a [[f64; 2]],
    poles: &'a [[f64; 3]],
) -> CylindricalProfileQuery<'a> {
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
    }
}

fn query<'a>(sources: &'a [CylindricalProfileSource<'a>]) -> CylindricalMeasureQuery<'a> {
    CylindricalMeasureQuery {
        sources,
        depths: [0., 1.],
        limit: CylindricalMeasureLimit::Report,
        options: Default::default(),
    }
}
fn encloses(bounds: [f64; 2], value: f64) {
    assert!(
        bounds[0] <= value && value <= bounds[1],
        "{bounds:?} must enclose {value}"
    );
}

#[test]
fn rectangle_integral_and_exact_normal_factor_have_independent_values() {
    let f = Fixture::rectangle(0.125, 1., 0.);
    let sources = f.sources();
    let q = query(&sources);
    let proof = certify_cylindrical_normal_measure(q, || false).unwrap();
    assert_eq!(proof.authored_area_enclosure(), [0.125; 2]);
    assert_eq!(proof.authored_volume_enclosure(), [0.1875; 2]);
    encloses(proof.profile_volume_enclosure(), 0.);
    assert!(proof.area_discrepancy_bound() >= 0.125);
    assert!(proof.volume_discrepancy_bound() >= 0.1875);
    assert!(proof.matches_sources(q));
    assert_eq!(proof.evidence(), CylindricalMeasureEvidence::CertifiedBound);
    assert_eq!(proof.requested_options(), q.options);
    assert_eq!(proof.requested_limit(), q.limit);
}
#[test]
fn exact_generator_retains_zero_area_and_zero_error() {
    let f = Fixture::rectangle(0., 1., 0.);
    let sources = f.sources();
    let proof = certify_cylindrical_normal_measure(query(&sources), || false).unwrap();
    assert_eq!(proof.authored_area_enclosure(), [0.; 2]);
    assert_eq!(proof.profile_volume_enclosure(), [0.; 2]);
    assert_eq!(proof.area_discrepancy_bound(), 0.);
    assert_eq!(proof.volume_discrepancy_bound(), 0.);
}
#[test]
fn height_storage_error_is_included_in_the_closed_area_bound() {
    let f = Fixture::rectangle(0.125, 1., 0.125);
    let sources = f.sources();
    let proof = certify_cylindrical_normal_measure(query(&sources), || false).unwrap();
    assert!(proof.area_discrepancy_bound() >= 0.15625);
    encloses(proof.profile_volume_enclosure(), 0.);
}
#[test]
fn explicit_volume_cap_refuses_without_widening() {
    let f = Fixture::rectangle(0.125, 1., 0.);
    let sources = f.sources();
    let mut q = query(&sources);
    q.limit = CylindricalMeasureLimit::Refuse(0.01);
    let error = certify_cylindrical_normal_measure(q, || false).unwrap_err();
    assert_eq!(error.requested_limit, q.limit);
    assert!(matches!(
        error.cause,
        CylindricalMeasureFailure::Uncertified {
            requested: 0.01,
            ..
        }
    ));
}
#[test]
fn unchanged_source_bits_bind_the_report_independently_of_policy() {
    let f = Fixture::rectangle(0.125, 1., 0.);
    let mut sources = f.sources();
    let proof = certify_cylindrical_normal_measure(query(&sources), || false).unwrap();
    let mut q = query(&sources);
    q.options.max_operations = 0;
    assert!(proof.matches_sources(q));
    sources.reverse();
    assert!(!proof.matches_sources(query(&sources)));
}
#[test]
fn stale_source_or_interval_is_refused() {
    let f = Fixture::rectangle(0.125, 1., 0.);
    let mut sources = f.sources();
    sources[0].interval = CylindricalProfileInterval::new(0, 1).unwrap();
    let error = certify_cylindrical_normal_measure(query(&sources), || false).unwrap_err();
    assert_eq!(error.cause, CylindricalMeasureFailure::Source { index: 0 });
}
#[test]
fn cancellation_and_one_aggregate_budget_refuse_without_partial_measure() {
    let f = Fixture::rectangle(0.125, 1., 0.);
    let sources = f.sources();
    let mut q = query(&sources);
    let error = certify_cylindrical_normal_measure(q, || true).unwrap_err();
    assert!(matches!(
        error.cause,
        CylindricalMeasureFailure::Arithmetic(_)
    ));
    let proof = certify_cylindrical_normal_measure(q, || false).unwrap();
    q.options.max_operations = proof.operations() - 1;
    let error = certify_cylindrical_normal_measure(q, || false).unwrap_err();
    assert!(matches!(
        error.cause,
        CylindricalMeasureFailure::Arithmetic(_)
    ));
}
#[test]
fn collapsed_radius_and_unordered_depths_are_refused() {
    let f = Fixture::rectangle(0.125, 1., 0.);
    let sources = f.sources();
    let mut q = query(&sources);
    q.depths = [-1., 0.];
    assert!(matches!(
        certify_cylindrical_normal_measure(q, || false)
            .unwrap_err()
            .cause,
        CylindricalMeasureFailure::Radius { depth: 0, .. }
    ));
    q.depths = [1., 0.];
    assert_eq!(
        certify_cylindrical_normal_measure(q, || false)
            .unwrap_err()
            .cause,
        CylindricalMeasureFailure::Depths
    );
}
#[test]
fn exact_offsets_survive_binary64_radius_addition_collapse() {
    let f = Fixture::rectangle(0.125, 1., 0.);
    let sources = f.sources();
    let mut q = query(&sources);
    q.depths = [
        martensite_cmath::powi(2., -54),
        martensite_cmath::powi(2., -53),
    ];
    assert_eq!(1. + q.depths[0], 1. + q.depths[1]);
    let proof = certify_cylindrical_normal_measure(q, || false).unwrap();
    let bounds = proof.authored_volume_enclosure();
    assert!(
        bounds[0] >= martensite_cmath::powi(2., -57)
            && bounds[1] <= martensite_cmath::powi(2., -56)
    );
}
#[test]
fn generated_rectangles_include_collapsed_and_reversed_boundaries() {
    for width in [-0.125, 0., 0.0625, 0.125] {
        for height in [-1., 0., 0.5, 1.] {
            let f = Fixture::rectangle(width, height, 0.);
            let sources = f.sources();
            let proof = certify_cylindrical_normal_measure(query(&sources), || false).unwrap();
            encloses(proof.authored_area_enclosure(), width * height);
            encloses(proof.authored_volume_enclosure(), 1.5 * width * height);
            encloses(proof.profile_volume_enclosure(), 0.);
        }
    }
}
#[test]
fn measure_public_types_are_send_and_sync() {
    fn check<T: Send + Sync>() {}
    check::<CylindricalMeasureEvidence>();
    check::<CylindricalMeasureOptions>();
    check::<CylindricalMeasureLimit>();
    check::<CylindricalMeasureQuery<'static>>();
    check::<CertifiedCylindricalNormalMeasure>();
    check::<CylindricalMeasureError>();
    check::<CylindricalMeasureFailure>();
}

#[test]
fn varying_height_error_keeps_the_angular_height_cross_term() {
    let mut f = Fixture::rectangle(0.125, 1., 0.);
    f.profiles = f.authored.iter().map(|p| profile(p, 0., 1.125)).collect();
    let sources = f.sources();
    let proof = certify_cylindrical_normal_measure(query(&sources), || false).unwrap();
    assert!(proof.area_discrepancy_bound() >= 0.15625);
    encloses(proof.profile_volume_enclosure(), 0.);
}
#[test]
fn quadratic_closed_profile_uses_a_non_binary_exact_area_oracle() {
    let authored = vec![
        vec![[0., 0.], [0.125, 0.], [0.125, 1.]],
        vec![[0.125, 1.], [0., 0.]],
    ];
    let profiles = authored.iter().map(|p| profile(p, 0., 1.)).collect();
    let f = Fixture { authored, profiles };
    let sources = f.sources();
    let proof = certify_cylindrical_normal_measure(query(&sources), || false).unwrap();
    encloses_fraction(proof.authored_area_enclosure(), 1, 24);
    encloses_fraction(proof.authored_volume_enclosure(), 1, 16);
}
fn encloses_fraction(bounds: [f64; 2], numerator: i64, denominator: i64) {
    use crate::predicates::roots::DyadicBound;
    let (n, d) = DyadicBound::from_f64(bounds[0]).unwrap().ratio();
    assert!(n * denominator <= d * numerator);
    let (n, d) = DyadicBound::from_f64(bounds[1]).unwrap().ratio();
    assert!(n * denominator >= d * numerator);
}

#[test]
fn invalid_limits_and_precision_are_not_clamped() {
    let f = Fixture::rectangle(0.125, 1., 0.);
    let sources = f.sources();
    let mut q = query(&sources);
    q.limit = CylindricalMeasureLimit::Refuse(f64::INFINITY);
    assert_eq!(
        certify_cylindrical_normal_measure(q, || false)
            .unwrap_err()
            .cause,
        CylindricalMeasureFailure::Policy
    );
    q = query(&sources);
    q.options.fractional_bits = 7;
    assert_eq!(
        certify_cylindrical_normal_measure(q, || false)
            .unwrap_err()
            .cause,
        CylindricalMeasureFailure::Policy
    );
}
#[test]
fn source_count_limits_include_every_required_piece() {
    let f = Fixture::rectangle(0.125, 1., 0.);
    let sources = f.sources();
    let mut q = query(&sources);
    q.options.max_sources = 3;
    assert_eq!(
        certify_cylindrical_normal_measure(q, || false)
            .unwrap_err()
            .cause,
        CylindricalMeasureFailure::SourceCount
    );
    assert_eq!(
        certify_cylindrical_normal_measure(query(&[]), || false)
            .unwrap_err()
            .cause,
        CylindricalMeasureFailure::SourceCount
    );
}
