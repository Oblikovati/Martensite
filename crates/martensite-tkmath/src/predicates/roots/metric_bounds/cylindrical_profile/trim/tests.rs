// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::metric_bounds::MetricBezier;

fn query(curve: &[[f64; 3]]) -> CylindricalTrimQuery<'_> {
    CylindricalTrimQuery {
        curve: MetricBezier {
            poles: curve,
            weights: None,
        },
        pcurve: &[[0., 3.], [0., 4.]],
        radius: 1.,
        spatial_limit: 0.,
        options: Default::default(),
    }
}

#[test]
fn exact_generator_has_zero_full_interval_residual() {
    let input = query(&[[1., 0., 3.], [1., 0., 4.]]);
    let proof = certify_cylindrical_trim(input, || false).unwrap();
    assert_eq!(proof.xy_residual_bound(), 0.);
    assert_eq!(proof.height_residual_bound(), 0.);
    assert_eq!(proof.spatial_residual_bound(), 0.);
    assert_eq!(proof.requested_spatial_limit(), 0.);
    assert_eq!(proof.requested_options(), input.options);
    assert!(proof.operations() > 0);
    assert!(proof.matches_source(input));
    fn send_sync<T: Send + Sync>() {}
    send_sync::<CertifiedCylindricalTrim>();
}

#[test]
fn radial_and_height_errors_are_paid_in_physical_units_at_the_exact_cap() {
    let mut input = query(&[[8.125, 0., 3.25], [8.125, 0., 4.25]]);
    input.radius = 8.;
    input.spatial_limit = 0.375;
    let proof = certify_cylindrical_trim(input, || false).unwrap();
    assert_eq!(proof.xy_residual_bound(), 0.125);
    assert_eq!(proof.height_residual_bound(), 0.25);
    assert_eq!(proof.spatial_residual_bound(), 0.375);
    input.spatial_limit = 0.375f64.next_down();
    let error = certify_cylindrical_trim(input, || false).unwrap_err();
    assert!(error.to_string().contains("cylindrical cap trim"));
    assert!(matches!(
        error.cause,
        CylindricalProfileFailure::Uncertified { .. }
    ));
}

#[test]
fn a_rational_space_curve_uses_the_actual_weighted_height() {
    let mut input = query(&[[1., 0., 3.25], [1., 0., 4.25]]);
    input.curve.weights = Some(&[1., 2.]);
    input.spatial_limit = 1.;
    let proof = certify_cylindrical_trim(input, || false).unwrap();
    // At u=1/2: (3.25 + 2*4.25)/3 - 3.5 = 5/12.
    assert!(proof.height_residual_bound() >= 5. / 12.);
    assert_eq!(proof.xy_residual_bound(), 0.);
}

#[test]
fn stale_uv_space_weight_and_radius_bits_cannot_match() {
    let input = query(&[[1., 0., 3.], [1., 0., 4.]]);
    let proof = certify_cylindrical_trim(input, || false).unwrap();
    let mut changed = input;
    changed.pcurve = &[[0., 3.], [0., 4.25]];
    assert!(!proof.matches_source(changed));
    changed = input;
    changed.radius = 2.;
    assert!(!proof.matches_source(changed));
    changed = input;
    changed.curve.weights = Some(&[1., 2.]);
    assert!(!proof.matches_source(changed));
    changed = input;
    changed.curve.poles = &[[1., 0., 3.], [1., 0., 4.25]];
    assert!(!proof.matches_source(changed));
}

#[test]
fn invalid_geometry_policy_exhaustion_and_cancellation_never_publish() {
    let mut input = query(&[[1., 0., 3.], [1., 0., 4.]]);
    input.radius = 0.;
    assert!(certify_cylindrical_trim(input, || false).is_err());
    input.radius = 1.;
    input.options.max_operations = 0;
    assert!(matches!(
        certify_cylindrical_trim(input, || false).unwrap_err().cause,
        CylindricalProfileFailure::Arithmetic(_)
    ));
    input.options = Default::default();
    assert!(matches!(
        certify_cylindrical_trim(input, || true).unwrap_err().cause,
        CylindricalProfileFailure::Arithmetic(_)
    ));
}

#[test]
fn invalid_precision_policy_refuses_before_capture() {
    let mut input = query(&[[1., 0., 3.], [1., 0., 4.]]);
    input.options.fractional_bits = 7;
    assert!(matches!(
        certify_cylindrical_trim(input, || false).unwrap_err().cause,
        CylindricalProfileFailure::Policy(_)
    ));
}

#[test]
fn a_trace_bound_does_not_require_an_invertible_angular_direction() {
    let mut input = query(&[[0., 0., 3.], [0., 0., 4.]]);
    input.spatial_limit = 1.;
    let proof = certify_cylindrical_trim(input, || false).unwrap();
    assert_eq!(proof.spatial_residual_bound(), 1.);
}

#[test]
fn restricted_certificate_excludes_a_large_inactive_trace_error() {
    let points = [[2., 0., 0.], [2., 0., 1.]];
    let pcurve = [[0., 0.], [0., 2.]];
    let query = CylindricalTrimQuery {
        curve: MetricBezier {
            poles: &points,
            weights: None,
        },
        pcurve: &pcurve,
        radius: 2.,
        spatial_limit: 0.2,
        options: Default::default(),
    };
    assert!(certify_cylindrical_trim(query, || false).is_err());
    let proof = certify_cylindrical_trim_ranges(query, [[0.05, 0.125]; 2], || false).unwrap();
    assert!(0.125 <= proof.spatial_residual_bound() && proof.spatial_residual_bound() < 0.126);
    assert!(proof.matches_ranges(query, [[0.05, 0.125]; 2]));
    assert!(!proof.matches_source(query));
    assert!(certify_cylindrical_trim_ranges(query, [[0.05, 0.25]; 2], || false).is_err());
}

#[test]
fn a_degree_five_chart_can_have_its_own_normalized_active_range() {
    let points = [[2., 0., 0.], [2., 0., 1.]];
    let pcurve = [0.25, 0.35, 0.45, 0.55, 0.65, 0.75].map(|height| [0., height]);
    let query = CylindricalTrimQuery {
        curve: MetricBezier {
            poles: &points,
            weights: None,
        },
        pcurve: &pcurve,
        radius: 2.,
        spatial_limit: 1e-12,
        options: Default::default(),
    };
    let ranges = [[0.25, 0.75], [0., 1.]];
    let proof = certify_cylindrical_trim_ranges(query, ranges, || false).unwrap();
    assert!(proof.spatial_residual_bound() < 1e-12);
    assert!(proof.matches_ranges(query, ranges));
    assert!(!proof.matches_ranges(query, [ranges[1]; 2]));
    assert!(certify_cylindrical_trim(query, || false).is_err());
}
