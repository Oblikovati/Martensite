// SPDX-License-Identifier: GPL-2.0-only
use super::*;

fn rational<'a>(poles: &'a [[f64; 2]], weights: &'a [f64]) -> OrientedMetricCurve<'a> {
    OrientedMetricCurve {
        curve: MetricCurve::Bezier(MetricBezier {
            poles,
            weights: Some(weights),
        }),
        reversed: false,
    }
}
fn rational_failure(policy: IntegralProgramOptions) -> IntegralProgramError {
    let poles = [[1., 0.], [1., 1.], [0., 1.]];
    let curves = [rational(&poles, &[1., 1., 2.])];
    boundary(&[curves.as_slice()], policy).unwrap_err()
}
#[test]
fn global_cells_are_enforced_during_first_curve_and_failure_receipts_are_exact() {
    let mut policy = options();
    policy.integration.absolute_error_limit = 0.;
    policy.max_total_subintervals = 1;
    let error = rational_failure(policy);
    assert_eq!(error.integration_subintervals, 1);
    assert_eq!(error.completed_occurrences, 0);
    assert_eq!(
        (
            error.loop_index,
            error.component_index,
            error.occurrence_index
        ),
        (Some(0), Some(0), Some(0))
    );
    assert!(matches!(
        error.cause,
        IntegralProgramFailure::AggregateWork { limit: 1, .. }
    ));
}
#[test]
fn per_curve_limit_remains_distinct_from_remaining_program_cells() {
    let mut policy = options();
    policy.integration.max_subintervals = 1;
    policy.integration.absolute_error_limit = 0.;
    policy.max_total_subintervals = 64;
    let error = rational_failure(policy);
    assert_eq!(error.integration_subintervals, 1);
    assert!(
        matches!(error.cause, IntegralProgramFailure::Numerical(cause) if matches!(*cause, RationalIntegralFailure::WorkLimit { limit: 1, .. }))
    );
}
#[test]
fn zero_cell_program_still_admits_and_completes_actual_polynomial_occurrences() {
    let poles = rectangle();
    let curves = occurrences(&poles);
    let parts = [&curves[..43], &curves[43..]];
    let mut policy = options();
    policy.max_total_subintervals = 0;
    let proof = boundary(&parts, policy).unwrap();
    assert_eq!(proof.completed_occurrences(), 86);
    assert_eq!(proof.integration_subintervals(), 0);
}
#[test]
fn cancellation_at_every_executed_checkpoint_never_publishes_partial_result() {
    let poles = [[[0., 0.], [1., 0.]], [[1., 0.], [0., 1.]]];
    let curves = occurrences(&poles);
    let parts = [curves.as_slice()];
    let query = BoundaryIntegralProgramQuery {
        components: &parts,
        options: options(),
    };
    let mut calls = 0;
    certify_boundary_integral_program(query, || {
        calls += 1;
        false
    })
    .unwrap();
    assert_cancellation_checkpoints(query, calls);
}
fn assert_cancellation_checkpoints(query: BoundaryIntegralProgramQuery<'_>, calls: usize) {
    for stop in 1..=calls {
        let mut current = 0;
        let result = certify_boundary_integral_program(query, || {
            current += 1;
            current == stop
        });
        assert_eq!(current, stop);
        assert_cancelled(result.unwrap_err());
    }
}
#[test]
fn invalid_first_source_precedes_later_cancel_and_keeps_numerical_cause() {
    let poles = [[1., 0.], [0., 1.]];
    let curves = [rational(&poles, &[-1., 1.])];
    let parts = [curves.as_slice()];
    let query = BoundaryIntegralProgramQuery {
        components: &parts,
        options: options(),
    };
    let mut baseline = 0;
    let failure = certify_boundary_integral_program(query, || {
        baseline += 1;
        false
    })
    .unwrap_err();
    let repeated = failure_after_checkpoint(query, baseline);
    assert_eq!(
        format!("{:?}", failure.cause),
        format!("{:?}", repeated.cause)
    );
    assert_eq!(repeated.integration_subintervals, 0);
}
fn failure_after_checkpoint(
    query: BoundaryIntegralProgramQuery<'_>,
    baseline: usize,
) -> IntegralProgramError {
    let mut calls = 0;
    certify_boundary_integral_program(query, || {
        calls += 1;
        calls > baseline
    })
    .unwrap_err()
}
#[test]
fn component_count_refusal_retains_actual_requested_count() {
    let poles = [[[1., 0.], [0., 1.]]];
    let curves = occurrences(&poles);
    let parts = [curves.as_slice(), curves.as_slice()];
    let mut policy = options();
    policy.max_components = 1;
    let error = boundary(&parts, policy).unwrap_err();
    assert!(matches!(
        error.cause,
        IntegralProgramFailure::ComponentCount {
            actual: 2,
            limit: 1
        }
    ));
}
#[test]
fn aggregate_policy_and_empty_refusals_keep_their_actual_request() {
    let poles = [[[1., 0.], [0., 1.]]];
    let curves = occurrences(&poles);
    let mut policy = options();
    policy.max_total_subintervals = 64 * 4096 + 1;
    let error = boundary(&[curves.as_slice()], policy).unwrap_err();
    assert_eq!(*error.requested_options, policy);
    assert!(matches!(
        error.cause,
        IntegralProgramFailure::Policy {
            field: "max_total_subintervals",
            actual: 262145,
            minimum: 0,
            maximum: 262144,
        }
    ));
    assert!(matches!(
        boundary(&[], options()).unwrap_err().cause,
        IntegralProgramFailure::Empty
    ));
}

#[test]
fn prior_exact_occurrences_remain_in_the_failed_global_work_receipt() {
    let line_poles = [[[1., 0.], [1., 2.]]];
    let exact = occurrences(&line_poles);
    let poles = [[1., 0.], [1., 1.], [0., 1.]];
    let curved = [rational(&poles, &[1., 1., 2.])];
    let mut policy = options();
    policy.integration.absolute_error_limit = 0.;
    policy.max_total_subintervals = 1;
    let error = boundary(&[exact.as_slice(), curved.as_slice()], policy).unwrap_err();
    assert_eq!(error.completed_occurrences, 1);
    assert_eq!(error.integration_subintervals, 1);
    assert_eq!(
        (error.component_index, error.occurrence_index),
        (Some(1), Some(1))
    );
}
#[test]
fn signed_zero_error_policy_bits_are_not_rebound() {
    let poles = [[[1., 0.], [1., 2.]]];
    let curves = occurrences(&poles);
    let parts = [curves.as_slice()];
    let mut policy = options();
    policy.integration.absolute_error_limit = 0.;
    let proof = boundary(&parts, policy).unwrap();
    policy.integration.absolute_error_limit = -0.;
    assert!(!proof.matches_boundary(BoundaryIntegralProgramQuery {
        components: &parts,
        options: policy
    }));
}

fn assert_cancelled(error: IntegralProgramError) {
    use crate::predicates::roots::{RootIsolationError, metric_bounds::ResidualError};
    assert!(
        matches!(&error.cause, IntegralProgramFailure::Numerical(cause)
        if matches!(cause.as_ref(), RationalIntegralFailure::Metric(metric)
            if matches!(metric.as_ref(), ResidualError::Roots(RootIsolationError::Cancelled))))
    );
}
#[test]
fn invalid_component_limits_report_actual_fields_and_expected_ranges() {
    let poles = [[[1., 0.], [1., 2.]]];
    let curves = occurrences(&poles);
    for actual in [0, 65] {
        let mut policy = options();
        policy.max_components = actual;
        let error = boundary(&[curves.as_slice()], policy).unwrap_err();
        assert!(error.to_string().contains("expected 1..=64"));
        assert!(matches!(error.cause, IntegralProgramFailure::Policy {
            field: "max_components", actual: value, minimum: 1, maximum: 64,
        } if value == u64::from(actual)));
    }
}

#[test]
fn adaptive_rational_components_match_independent_two_quarter_circle_integrals() {
    let poles = [[1., 0.], [1., 1.], [0., 1.]];
    let curves = [rational(&poles, &[1., 1., 2.])];
    let parts = [curves.as_slice(), curves.as_slice()];
    let proof = boundary(&parts, options()).unwrap();
    let [lower, upper] = proof.enclosure();
    assert!(lower <= std::f64::consts::FRAC_PI_2 && upper >= std::f64::consts::FRAC_PI_2);
    assert_eq!(proof.completed_occurrences(), 2);
    assert!(proof.integration_subintervals() > 0);
    assert!(proof.integration_subintervals() <= options().max_total_subintervals);
    assert_eq!(
        proof.evidence(),
        RationalIntegralEvidence::ErrorBoundedIntegration
    );
}
#[test]
fn rational_final_checkpoint_cancellation_retains_all_actual_cells_and_occurrences() {
    let poles = [[1., 0.], [1., 1.], [0., 1.]];
    let curves = [rational(&poles, &[1., 1., 2.])];
    let parts = [curves.as_slice(), curves.as_slice()];
    let query = BoundaryIntegralProgramQuery {
        components: &parts,
        options: options(),
    };
    let mut calls = 0;
    let proof = certify_boundary_integral_program(query, || {
        calls += 1;
        false
    })
    .unwrap();
    assert_rational_final_cancel(query, calls, &proof);
}
fn assert_rational_final_cancel(
    query: BoundaryIntegralProgramQuery<'_>,
    stop: usize,
    proof: &CertifiedIntegralProgram,
) {
    let mut current = 0;
    let error = certify_boundary_integral_program(query, || {
        current += 1;
        current == stop
    })
    .unwrap_err();
    assert_eq!(current, stop);
    assert_eq!(error.completed_occurrences, proof.completed_occurrences());
    assert_eq!(
        error.integration_subintervals,
        proof.integration_subintervals()
    );
    assert!(error.integration_subintervals > 0);
    assert_cancelled(error);
}
#[test]
fn cancellation_inside_the_failing_rational_cell_preserves_examined_work() {
    let poles = [[1., 0.], [1., 1.], [0., 1.]];
    let curves = [rational(&poles, &[1., 1., 2.])];
    let parts = [curves.as_slice()];
    let mut policy = options();
    policy.integration.absolute_error_limit = 0.;
    policy.max_total_subintervals = 1;
    let query = BoundaryIntegralProgramQuery {
        components: &parts,
        options: policy,
    };
    assert_failing_cell_cancel(query);
}
fn assert_failing_cell_cancel(query: BoundaryIntegralProgramQuery<'_>) {
    let mut checkpoints = 0;
    let work_failure = certify_boundary_integral_program(query, || {
        checkpoints += 1;
        false
    })
    .unwrap_err();
    assert_eq!(work_failure.integration_subintervals, 1);
    assert!(matches!(
        work_failure.cause,
        IntegralProgramFailure::AggregateWork { .. }
    ));
    let mut current = 0;
    let error = certify_boundary_integral_program(query, || {
        current += 1;
        current == checkpoints
    })
    .unwrap_err();
    assert_cell_cancel(error, current, checkpoints);
}
fn assert_cell_cancel(error: IntegralProgramError, actual: usize, expected: usize) {
    assert_eq!(actual, expected);
    assert_eq!(error.integration_subintervals, 1);
    assert_eq!(error.completed_occurrences, 0);
    assert_cancelled(error);
}
