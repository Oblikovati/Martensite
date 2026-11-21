// SPDX-License-Identifier: GPL-2.0-only
use super::*;

#[test]
fn coefficient_work_is_shared_between_both_actual_carriers() {
    let spans = [span(&A, &B, None)];
    let proof = certify_analytic_conical_boundary_flux(query(&spans), || false).unwrap();
    let mut input = query(&spans);
    input.arithmetic.max_operations = proof.arithmetic_operations() - 1;
    let failure = certify_analytic_conical_boundary_flux(input, || false).unwrap_err();
    assert!(matches!(
        failure.cause,
        AnalyticConicalFluxFailure::Coefficients { depth: 1, .. }
    ));
    input.arithmetic.max_operations = 0;
    let failure = certify_analytic_conical_boundary_flux(input, || false).unwrap_err();
    assert!(matches!(
        failure.cause,
        AnalyticConicalFluxFailure::Coefficients { depth: 0, .. }
    ));
}

#[test]
fn original_zero_error_cap_refuses_nonzero_coefficient_uncertainty() {
    let mut spans = [span(&ZERO, &ZERO, None)];
    spans[0].cap_pcurves[0] = &[[1., 0.], [0., 1.]];
    let mut input = query(&spans);
    input.options.absolute_error_limit = 0.;
    let failure = certify_analytic_conical_boundary_flux(input, || false).unwrap_err();
    assert_eq!(failure.requested_options, input.options);
    assert!(matches!(
        failure.cause,
        AnalyticConicalFluxFailure::Integration { span: None, .. }
    ));
}

#[test]
fn invalid_actual_carrier_retains_exact_depth_and_offending_values() {
    let spans = [span(&A, &B, None)];
    for value in [0., -1., f64::INFINITY] {
        let mut input = query(&spans);
        input.carriers[1].radius = value;
        let failure = certify_analytic_conical_boundary_flux(input, || false).unwrap_err();
        assert!(
            matches!(failure.cause, AnalyticConicalFluxFailure::Carrier { depth:1, actual } if actual.radius == value)
        );
    }
}

#[test]
fn unsupported_arithmetic_and_shared_integration_policies_refuse_without_clamps() {
    let spans = [span(&A, &B, None)];
    let mut input = query(&spans);
    input.arithmetic.fractional_bits = 7;
    assert!(
        matches!(certify_analytic_conical_boundary_flux(input, || false).unwrap_err().cause,
        AnalyticConicalFluxFailure::ArithmeticPolicy(actual) if actual == input.arithmetic)
    );
    input = query(&spans);
    input.options.max_degree = 17;
    assert!(matches!(
        certify_analytic_conical_boundary_flux(input, || false)
            .unwrap_err()
            .cause,
        AnalyticConicalFluxFailure::Input {
            span: None,
            depth: None,
            ..
        }
    ));
}

#[test]
fn malformed_actual_cap_pcurve_names_the_offending_span_and_depth() {
    let mut spans = [span(&A, &B, None)];
    spans[0].cap_pcurves[1] = &[[0., f64::NAN], [0., 0.]];
    let failure = certify_analytic_conical_boundary_flux(query(&spans), || false).unwrap_err();
    assert!(matches!(
        failure.cause,
        AnalyticConicalFluxFailure::Input {
            span: Some(0),
            depth: Some(1),
            ..
        }
    ));
}

#[test]
fn cancellation_during_coefficient_work_preserves_actual_depth() {
    let spans = [span(&A, &B, None)];
    let mut polls = 0;
    let failure = certify_analytic_conical_boundary_flux(query(&spans), || {
        polls += 1;
        polls == 3
    })
    .unwrap_err();
    assert!(matches!(
        failure.cause,
        AnalyticConicalFluxFailure::Coefficients { depth: 0, .. }
    ));
}

#[test]
fn exact_moment_work_polls_and_preserves_its_original_cause() {
    let failure =
        super::super::caps::moments::integrate(&[[1., 0.], [0., 1.]], &mut || true).unwrap_err();
    assert!(matches!(failure, PolynomialIntegralFailure::Metric(_)));
}

#[test]
fn public_types_are_send_and_sync() {
    fn assert_traits<T: Send + Sync>() {}
    assert_traits::<AnalyticConicalFluxSpan<'static>>();
    assert_traits::<AnalyticConicalFluxQuery<'static>>();
    assert_traits::<CertifiedAnalyticConicalFlux>();
    assert_traits::<AnalyticConicalFluxError>();
    assert_traits::<AnalyticConicalFluxFailure>();
}
