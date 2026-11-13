// SPDX-License-Identifier: GPL-2.0-only
use super::super::MetricBezier;
use super::*;

const AUTHORED: [[f64; 2]; 2] = [[0., 3.], [0., 4.]];
const STORED: [[f64; 3]; 2] = [[1., 0., 3.], [1., 0., 4.]];
fn query(stored: &[[f64; 3]]) -> CylindricalProfileQuery<'_> {
    CylindricalProfileQuery {
        authored: &AUTHORED,
        stored: MetricBezier {
            poles: stored,
            weights: None,
        },
        interval: CylindricalProfileInterval::new(0, 0).unwrap(),
        support_radius: 2.,
        stored_radius: 1.,
        angular_origin: 0.,
        spatial_limit: 1e-6,
        options: Default::default(),
    }
}

#[test]
fn exact_generator_has_zero_position_and_derivative_error() {
    let input = query(&STORED);
    let proof = certify_cylindrical_profile(input, || false).unwrap();
    assert_eq!(proof.angular_position_bound(), 0.);
    assert_eq!(proof.height_position_bound(), 0.);
    assert_eq!(proof.developed_position_bound(), 0.);
    assert_eq!(proof.angular_derivative_bound(), 0.);
    assert_eq!(proof.height_derivative_bound(), 0.);
    assert_eq!(proof.developed_derivative_bound(), 0.);
    assert_eq!(proof.requested_options(), input.options);
    assert_eq!(proof.requested_spatial_limit(), input.spatial_limit);
    assert!(proof.matches_source(input));
    assert!(proof.operations() > 0);
}

#[test]
fn exact_restriction_uses_local_parameter_derivatives() {
    let poles = [[1., 0., 3.5], [1., 0., 4.]];
    let mut input = query(&poles);
    input.interval = CylindricalProfileInterval::new(1, 1).unwrap();
    let proof = certify_cylindrical_profile(input, || false).unwrap();
    assert_eq!(proof.developed_position_bound(), 0.);
    assert_eq!(proof.developed_derivative_bound(), 0.);
    input.interval = CylindricalProfileInterval::new(0, 0).unwrap();
    assert!(matches!(
        certify_cylindrical_profile(input, || false)
            .unwrap_err()
            .cause,
        CylindricalProfileFailure::Uncertified { .. }
    ));
}

#[test]
fn nonzero_bounds_cover_the_closed_form_direction_and_height_errors() {
    let poles = [[1., 0.125, 3.25], [1., 0.125, 4.25]];
    let mut input = query(&poles);
    input.support_radius = 1.;
    input.spatial_limit = 0.4;
    let proof = certify_cylindrical_profile(input, || false).unwrap();
    assert!((1. / 7. ..0.143).contains(&proof.angular_position_bound()));
    assert_eq!(proof.height_position_bound(), 0.25);
    assert!((11. / 28. ..0.393).contains(&proof.developed_position_bound()));
    assert_eq!(proof.developed_derivative_bound(), 0.);
}

#[test]
fn invalid_intervals_work_limits_and_cancelled_proofs_refuse() {
    assert!(CylindricalProfileInterval::new(1, 0).is_err());
    assert!(CylindricalProfileInterval::new(0, 64).is_err());
    let mut input = query(&STORED);
    input.options.max_operations = 0;
    assert!(matches!(
        certify_cylindrical_profile(input, || false)
            .unwrap_err()
            .cause,
        CylindricalProfileFailure::Arithmetic(_)
    ));
    input.options = Default::default();
    assert!(matches!(
        certify_cylindrical_profile(input, || true)
            .unwrap_err()
            .cause,
        CylindricalProfileFailure::Arithmetic(_)
    ));
}

#[test]
fn height_derivative_error_uses_the_original_authored_polynomial() {
    let poles = [[1., 0., 3.], [1., 0., 4.125]];
    let mut input = query(&poles);
    input.spatial_limit = 0.125;
    let proof = certify_cylindrical_profile(input, || false).unwrap();
    assert_eq!(proof.height_position_bound(), 0.125);
    assert_eq!(proof.height_derivative_bound(), 0.125);
    assert_eq!(proof.developed_derivative_bound(), 0.125);
}

fn fit_query(authored: &[[f64; 2]]) -> CylindricalFitQuery<'_> {
    CylindricalFitQuery {
        authored,
        interval: CylindricalProfileInterval::new(0, 0).unwrap(),
        support_radius: 10.,
        angular_origin: 0.,
        spatial_limit: 2.5e-6,
        max_level: 8,
        max_operations: 100_000,
    }
}

#[test]
fn generator_fit_needs_no_geometric_subdivision() {
    let input = fit_query(&AUTHORED);
    let plan = plan_cylindrical_fit(input, || false).unwrap();
    assert_eq!(plan.subdivision_level(), 0);
    assert_eq!(plan.ideal_developed_error_bound(), 0.);
    assert_eq!(plan.requested_spatial_limit(), input.spatial_limit);
}

#[test]
fn fit_plan_is_stable_under_sufficient_work_limits_and_refuses_exhaustion() {
    let authored = [[-1., 0.], [1., 0.]];
    let mut input = fit_query(&authored);
    let first = plan_cylindrical_fit(input, || false).unwrap();
    assert!(first.subdivision_level() > 0);
    assert!(first.ideal_developed_error_bound() <= input.spatial_limit / 2.);
    input.max_operations = 200_000;
    let second = plan_cylindrical_fit(input, || false).unwrap();
    assert_eq!(first.subdivision_level(), second.subdivision_level());
    assert_eq!(
        first.chart_center().to_bits(),
        second.chart_center().to_bits()
    );
    assert_fit_resource_refusals(input);
}

fn assert_fit_resource_refusals(mut input: CylindricalFitQuery<'_>) {
    input.max_level = 0;
    assert!(matches!(
        plan_cylindrical_fit(input, || false).unwrap_err().cause,
        CylindricalProfileFailure::FitLevel { .. }
    ));
    input.max_operations = 0;
    assert!(matches!(
        plan_cylindrical_fit(input, || false).unwrap_err().cause,
        CylindricalProfileFailure::Arithmetic(_)
    ));
}

#[test]
fn source_binding_includes_signed_zero_weights_and_chart_parameters() {
    let proof = certify_cylindrical_profile(query(&STORED), || false).unwrap();
    let mut changed = STORED;
    changed[0][1] = -0.;
    assert!(!proof.matches_source(query(&changed)));
    let weights = [1., 1.];
    let mut input = query(&STORED);
    input.stored.weights = Some(&weights);
    assert!(!proof.matches_source(input));
    input = query(&STORED);
    input.angular_origin = -0.;
    assert!(!proof.matches_source(input));
    input = query(&STORED);
    input.support_radius = 3.;
    assert!(!proof.matches_source(input));
}

#[test]
fn profile_inputs_reports_and_errors_are_send_and_sync() {
    fn require<T: Send + Sync>() {}
    require::<CylindricalProfileQuery<'static>>();
    require::<CylindricalProfileOptions>();
    require::<CylindricalProfileInterval>();
    require::<CertifiedCylindricalProfile>();
    require::<CylindricalProfileError>();
    require::<CylindricalFitQuery<'static>>();
    require::<CylindricalFitPlan>();
}

#[test]
fn rational_circle_derivative_bound_covers_independent_endpoint_error() {
    let authored = [[0., 3.], [1., 3.]];
    let poles = [[1., 0., 3.], [1., 0.5, 3.], [0.6, 0.8, 3.]];
    let weights = [1., 1., 1.25];
    let mut input = query(&poles);
    input.authored = &authored;
    input.support_radius = 1.;
    input.stored.weights = Some(&weights);
    input.spatial_limit = 0.15;
    let proof = certify_cylindrical_profile(input, || false).unwrap();
    // t=u/2 gives theta'=1/(1+u^2/4), hence an endpoint discrepancy of 1/5.
    assert!(proof.angular_derivative_bound() >= 0.2);
    assert!(proof.angular_derivative_bound() < 0.32);
    assert!(proof.developed_position_bound() < 0.15);
}

#[test]
fn invalid_profile_values_and_weight_counts_refuse_before_arithmetic() {
    let weights = [1.];
    let mut input = query(&STORED);
    input.stored.weights = Some(&weights);
    assert!(matches!(
        certify_cylindrical_profile(input, || false)
            .unwrap_err()
            .cause,
        CylindricalProfileFailure::Weights { .. }
    ));
    input = query(&STORED);
    input.support_radius = 0.;
    assert!(matches!(
        certify_cylindrical_profile(input, || false)
            .unwrap_err()
            .cause,
        CylindricalProfileFailure::Value { .. }
    ));
}

#[test]
fn fit_planning_preserves_a_negative_chart_center_and_refuses_invalid_limits() {
    let authored = [[-3., 0.], [-1., 0.]];
    let input = fit_query(&authored);
    let plan = plan_cylindrical_fit(input, || false).unwrap();
    assert!((plan.chart_center() + 0.2).abs() < 1e-16);
}

#[test]
fn unsupported_profile_precision_refuses() {
    let mut input = query(&STORED);
    input.options.fractional_bits = 7;
    assert!(matches!(
        certify_cylindrical_profile(input, || false)
            .unwrap_err()
            .cause,
        CylindricalProfileFailure::Policy(_)
    ));
}

#[test]
fn fit_planning_refuses_invalid_limits_and_cancellation() {
    let authored = [[-3., 0.], [-1., 0.]];
    let mut input = fit_query(&authored);
    input.max_level = 64;
    assert!(matches!(
        plan_cylindrical_fit(input, || false).unwrap_err().cause,
        CylindricalProfileFailure::FitPolicy { .. }
    ));
    input.max_level = 8;
    input.spatial_limit = -1.;
    assert!(matches!(
        plan_cylindrical_fit(input, || false).unwrap_err().cause,
        CylindricalProfileFailure::Value { .. }
    ));
    input.spatial_limit = 1e-5;
    assert!(matches!(
        plan_cylindrical_fit(input, || true).unwrap_err().cause,
        CylindricalProfileFailure::Arithmetic(_)
    ));
}
