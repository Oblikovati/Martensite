// SPDX-License-Identifier: GPL-2.0-only
use super::*;
#[test]
fn parent_two_contains_independent_flux_and_rebinds_all_effective_settings() {
    let spans = [span(&A, &B, None)];
    let mut input = query(&spans);
    let old = certify_analytic_conical_boundary_flux(input, || false).unwrap();
    input.options.behavior_version = 2;
    let proof = certify_analytic_conical_boundary_flux(input, || false).unwrap();
    contains(&proof, 1. / 3.);
    assert_eq!(
        proof.integration_subintervals(),
        old.integration_subintervals()
    );
    assert_eq!(
        proof.arithmetic_operations(),
        old.arithmetic_operations() + 21
    );
    assert_eq!(proof.requested_options(), input.options);
    assert_eq!(proof.effective_options(), input.options);
    changed_flux_policy(&proof, input);
}
#[test]
fn typed_accumulation_refusal_keeps_span_actual_work_and_original_cause() {
    let spans = [span(&A, &B, None)];
    let mut input = query(&spans);
    let before = certify_analytic_conical_boundary_flux(input, || false)
        .unwrap()
        .arithmetic_operations();
    input.options.behavior_version = 2;
    input.arithmetic.max_operations = before + 6;
    let failure = certify_analytic_conical_boundary_flux(input, || false).unwrap_err();
    assert_eq!(failure.operations, before + 6);
    assert_eq!(failure.integration_subintervals, 0);
    assert!(
        matches!(failure.cause,AnalyticConicalFluxFailure::Arithmetic{span:Some(0),depth:None,cause}
        if matches!(*cause,crate::predicates::roots::certificate_arithmetic::bounded_failure::BoundedArithmeticFailure::WorkLimit{completed:6,limit:6}))
    );
}
#[test]
fn legacy_source_match_still_allows_resource_changes_but_never_parent_promotion() {
    let spans = [span(&A, &B, None)];
    let mut input = query(&spans);
    let proof = certify_analytic_conical_boundary_flux(input, || false).unwrap();
    input.arithmetic.max_operations -= 1;
    assert!(proof.matches_source(input));
    input.options.behavior_version = 2;
    assert!(!proof.matches_source(input));
}
#[test]
fn unsupported_parent_keeps_original_policy_and_first_invalid_carrier_wins() {
    let spans = [span(&A, &B, None)];
    let mut input = query(&spans);
    input.options.behavior_version = 3;
    let failure = certify_analytic_conical_boundary_flux(input, || false).unwrap_err();
    assert!(
        matches!(failure.cause,AnalyticConicalFluxFailure::Input{cause,..} if matches!(*cause,CylindricalFluxFailure::Policy(options) if options.behavior_version==3))
    );
    first_invalid_carrier(input);
}
#[test]
fn ordinary_cylinder_still_refuses_conical_parent_two() {
    let spans = [cylinder_span()];
    let input = CylindricalFluxQuery {
        spans: &spans,
        radii: [1., 2.],
        options: CylindricalFluxOptions {
            behavior_version: 2,
            ..Default::default()
        },
    };
    let failure = certify_cylindrical_boundary_flux(input, || false).unwrap_err();
    assert!(
        matches!(failure.cause,CylindricalFluxFailure::Policy(options) if options.behavior_version==2)
    );
}

fn changed_flux_policy(proof: &CertifiedAnalyticConicalFlux, input: AnalyticConicalFluxQuery<'_>) {
    assert!(proof.matches_source(input));
    let mut changed = input;
    changed.options.behavior_version = 1;
    assert!(!proof.matches_source(changed));
    changed = input;
    changed.options.max_total_subintervals -= 1;
    assert!(!proof.matches_source(changed));
    changed = input;
    changed.arithmetic.max_operations -= 1;
    assert!(!proof.matches_source(changed));
}

fn first_invalid_carrier(mut input: AnalyticConicalFluxQuery<'_>) {
    input.options.behavior_version = 2;
    input.carriers[0].radius = f64::NAN;
    let mut polls = 0;
    let failure = certify_analytic_conical_boundary_flux(input, || {
        polls += 1;
        polls > 1
    })
    .unwrap_err();
    assert_eq!(polls, 1);
    assert_eq!(failure.operations, 0);
    assert_eq!(failure.integration_subintervals, 0);
    assert!(matches!(
        failure.cause,
        AnalyticConicalFluxFailure::Carrier { depth: 0, .. }
    ));
}

fn cylinder_span() -> CylindricalFluxSpan<'static> {
    CylindricalFluxSpan {
        boundaries: [
            MetricBezier {
                poles: &A,
                weights: None,
            },
            MetricBezier {
                poles: &B,
                weights: None,
            },
        ],
        cap_pcurves: [&UV, &UV],
        side_reversed: false,
    }
}
#[test]
fn parent_two_policy_binding_retains_signed_zero_cap_bits() {
    let spans = [span(&ZERO, &ZERO, None)];
    let mut input = query(&spans);
    input.options.behavior_version = 2;
    input.options.absolute_error_limit = -0.;
    let proof = certify_analytic_conical_boundary_flux(input, || false).unwrap();
    assert_eq!(proof.enclosure(), [0., 0.]);
    assert!(proof.matches_source(input));
    input.options.absolute_error_limit = 0.;
    assert!(!proof.matches_source(input));
}
