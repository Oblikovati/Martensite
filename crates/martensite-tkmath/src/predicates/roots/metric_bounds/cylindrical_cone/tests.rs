// SPDX-License-Identifier: GPL-2.0-only
use super::super::{
    CylindricalProfileInterval, CylindricalProfileQuery, MetricBezier, certify_cylindrical_profile,
};
use super::*;
use std::cmp::Ordering;

fn profile<'a>(authored: &'a [[f64; 2]], poles: &'a [[f64; 3]]) -> CylindricalProfileQuery<'a> {
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
        spatial_limit: 0.25,
        options: Default::default(),
    }
}

fn source<'a>(
    query: CylindricalProfileQuery<'a>,
    certificate: &'a super::super::CertifiedCylindricalProfile,
) -> CylindricalConeSource<'a> {
    CylindricalConeSource {
        authored: query.authored,
        interval: query.interval,
        support_radius: query.support_radius,
        angular_origin: query.angular_origin,
        certificate,
    }
}

fn cone_for(
    sources: &[CylindricalConeSource<'_>],
) -> Result<CertifiedCylindricalCone, CylindricalConeError> {
    certify_cylindrical_cone(
        CylindricalConeQuery {
            sources,
            options: Default::default(),
        },
        || false,
    )
}

#[test]
fn exact_generator_has_a_unit_positive_homotopy_margin() {
    let q = profile(&[[0., 0.], [0., 1.]], &[[1., 0., 0.], [1., 0., 1.]]);
    let certificate = certify_cylindrical_profile(q, || false).unwrap();
    let sources = [source(q, &certificate)];
    let query = CylindricalConeQuery {
        sources: &sources,
        options: Default::default(),
    };
    let cone = certify_cylindrical_cone(query, || false).unwrap();
    assert_eq!(cone.direction(), [0., 1.]);
    assert_eq!(
        cone.directional_margin_bounds()[0].compare_f64(1.).unwrap(),
        Ordering::Equal
    );
    assert!(cone.matches_sources(&sources));
    assert_eq!(cone.requested_options(), query.options);
    assert!(cone.operations() > 0);
}

#[test]
fn retained_derivative_error_is_subtracted_from_the_source_margin() {
    let q = profile(&[[0., 0.], [0., 1.]], &[[1., 0., 0.], [1., 0., 1.125]]);
    let certificate = certify_cylindrical_profile(q, || false).unwrap();
    assert_eq!(certificate.developed_derivative_bound(), 0.125);
    let sources = [source(q, &certificate)];
    let cone = cone_for(&sources).unwrap();
    assert_eq!(
        cone.directional_margin_bounds()[0]
            .compare_f64(0.875)
            .unwrap(),
        Ordering::Equal
    );
}

#[test]
fn hull_direction_succeeds_when_both_cartesian_projections_have_zero_margin() {
    let authored = [[0., 0.], [1., 0.], [1., 1.]];
    let poles = [[1., 0., 0.], [1., 0.001, 0.], [1., 0.001, 1.]];
    let mut q = profile(&authored, &poles);
    q.support_radius = 1000.;
    q.spatial_limit = 0.001;
    let certificate = certify_cylindrical_profile(q, || false).unwrap();
    let sources = [source(q, &certificate)];
    let cone = cone_for(&sources).unwrap();
    assert!(cone.direction_attempts() > 4);
    assert_eq!(cone.direction(), [1., 1.]);
    assert_eq!(
        cone.directional_margin_bounds()[0]
            .compare_f64(1.99)
            .unwrap(),
        Ordering::Greater
    );
}

#[test]
fn opposite_branches_and_interior_stationarity_do_not_produce_a_cone() {
    let q = profile(&[[0., 0.], [0., 1.]], &[[1., 0., 0.], [1., 0., 1.]]);
    let r = profile(&[[0., 1.], [0., 0.]], &[[1., 0., 1.], [1., 0., 0.]]);
    let proofs = [
        certify_cylindrical_profile(q, || false).unwrap(),
        certify_cylindrical_profile(r, || false).unwrap(),
    ];
    let sources = [source(q, &proofs[0]), source(r, &proofs[1])];
    assert_unresolved(&sources);
    let cusp = profile(
        &[[0., 0.], [0., 1.], [0., 0.]],
        &[[1., 0., 0.], [1., 0., 1.], [1., 0., 0.]],
    );
    let proof = certify_cylindrical_profile(cusp, || false).unwrap();
    assert_unresolved(&[source(cusp, &proof)]);
}

fn assert_unresolved(sources: &[CylindricalConeSource<'_>]) {
    let error = cone_for(sources).unwrap_err();
    assert!(matches!(
        error.cause,
        CylindricalConeFailure::Unresolved { .. }
    ));
}

#[test]
fn exact_source_interval_controls_the_local_derivative_margin() {
    let mut q = profile(&[[0., 0.], [0., 2.]], &[[1., 0., 0.], [1., 0., 1.]]);
    q.interval = CylindricalProfileInterval::new(0, 1).unwrap();
    let proof = certify_cylindrical_profile(q, || false).unwrap();
    let sources = [source(q, &proof)];
    let cone = cone_for(&sources).unwrap();
    assert_eq!(
        cone.directional_margin_bounds()[0].compare_f64(1.).unwrap(),
        Ordering::Equal
    );
    let mut wrong = sources;
    wrong[0].interval = CylindricalProfileInterval::new(0, 0).unwrap();
    assert!(!cone.matches_sources(&wrong));
    let error = cone_for(&wrong).unwrap_err();
    assert!(matches!(
        error.cause,
        CylindricalConeFailure::SourceMismatch { index: 0 }
    ));
}

#[test]
fn policy_work_and_cancellation_failures_remain_typed() {
    let q = profile(&[[0., 0.], [0., 1.]], &[[1., 0., 0.], [1., 0., 1.]]);
    let proof = certify_cylindrical_profile(q, || false).unwrap();
    let sources = [source(q, &proof)];
    let mut query = CylindricalConeQuery {
        sources: &sources,
        options: Default::default(),
    };
    query.options.max_operations = 0;
    assert!(matches!(
        certify_cylindrical_cone(query, || false).unwrap_err().cause,
        CylindricalConeFailure::Arithmetic(_)
    ));
    query.options = Default::default();
    assert!(matches!(
        certify_cylindrical_cone(query, || true).unwrap_err().cause,
        CylindricalConeFailure::Arithmetic(_)
    ));
}

#[test]
fn cone_sources_reports_and_errors_are_send_and_sync() {
    fn require<T: Send + Sync>() {}
    require::<CylindricalConeSource<'static>>();
    require::<CylindricalConeQuery<'static>>();
    require::<CertifiedCylindricalCone>();
    require::<CylindricalConeError>();
}

#[test]
fn invalid_policy_and_source_counts_refuse_before_arithmetic() {
    let mut query = CylindricalConeQuery {
        sources: &[],
        options: Default::default(),
    };
    query.options.fractional_bits = 7;
    assert!(matches!(
        certify_cylindrical_cone(query, || false).unwrap_err().cause,
        CylindricalConeFailure::Policy(_)
    ));
    query.options = Default::default();
    assert!(matches!(
        certify_cylindrical_cone(query, || false).unwrap_err().cause,
        CylindricalConeFailure::SourceCount(0)
    ));
}

#[test]
fn consecutive_generators_share_a_strict_positive_projection() {
    let a = profile(&[[0., 0.], [0., 1.]], &[[1., 0., 0.], [1., 0., 1.]]);
    let b = profile(&[[0., 1.], [0., 2.]], &[[1., 0., 1.], [1., 0., 2.]]);
    let proofs = [
        certify_cylindrical_profile(a, || false).unwrap(),
        certify_cylindrical_profile(b, || false).unwrap(),
    ];
    let sources = [source(a, &proofs[0]), source(b, &proofs[1])];
    let cone = cone_for(&sources).unwrap();
    assert_eq!(cone.directional_margin_bounds().len(), 2);
    assert!(
        cone.directional_margin_bounds()
            .iter()
            .all(|m| m.compare_f64(1.).unwrap() == Ordering::Equal)
    );
}

#[test]
fn source_binding_retains_chart_and_signed_zero_identity() {
    let q = profile(&[[0., 0.], [0., 1.]], &[[1., 0., 0.], [1., 0., 1.]]);
    let certificate = certify_cylindrical_profile(q, || false).unwrap();
    let input = source(q, &certificate);
    let cone = cone_for(&[input]).unwrap();
    for changed in changed_sources(input) {
        assert!(!cone.matches_sources(&[changed]));
        assert!(matches!(
            cone_for(&[changed]).unwrap_err().cause,
            CylindricalConeFailure::SourceMismatch { index: 0 }
        ));
    }
}

#[test]
fn individually_certified_sources_must_share_the_same_developed_chart() {
    let a = profile(&[[0., 0.], [0., 1.]], &[[1., 0., 0.], [1., 0., 1.]]);
    let mut b = a;
    b.support_radius = 2.;
    let proofs = [
        certify_cylindrical_profile(a, || false).unwrap(),
        certify_cylindrical_profile(b, || false).unwrap(),
    ];
    let sources = [source(a, &proofs[0]), source(b, &proofs[1])];
    assert!(matches!(
        cone_for(&sources).unwrap_err().cause,
        CylindricalConeFailure::ChartMismatch { index: 1, .. }
    ));
}

fn changed_sources(input: CylindricalConeSource<'_>) -> [CylindricalConeSource<'_>; 3] {
    [
        CylindricalConeSource {
            support_radius: 2.,
            ..input
        },
        CylindricalConeSource {
            angular_origin: -0.,
            ..input
        },
        CylindricalConeSource {
            authored: &[[-0., 0.], [0., 1.]],
            ..input
        },
    ]
}

#[test]
fn candidate_generation_cannot_admit_more_than_two_sources() {
    let q = profile(&[[0., 0.], [0., 1.]], &[[1., 0., 0.], [1., 0., 1.]]);
    let certificate = certify_cylindrical_profile(q, || false).unwrap();
    let sources = [source(q, &certificate); 3];
    assert!(matches!(
        cone_for(&sources).unwrap_err().cause,
        CylindricalConeFailure::SourceCount(3)
    ));
}
