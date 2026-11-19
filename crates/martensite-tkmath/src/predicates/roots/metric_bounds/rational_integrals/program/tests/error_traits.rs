// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::{RootIsolationError, metric_bounds::ResidualError};

fn program_error(
    curve: OrientedMetricCurve<'_>,
    policy: IntegralProgramOptions,
    cancelled: impl FnMut() -> bool,
) -> IntegralProgramError {
    let curves = [curve];
    let parts = [curves.as_slice()];
    certify_boundary_integral_program(
        BoundaryIntegralProgramQuery {
            components: &parts,
            options: policy,
        },
        cancelled,
    )
    .unwrap_err()
}
fn curve(poles: &[[f64; 2]]) -> OrientedMetricCurve<'_> {
    OrientedMetricCurve {
        curve: MetricCurve::Bezier(MetricBezier {
            poles,
            weights: None,
        }),
        reversed: false,
    }
}
#[test]
fn issued_legacy_policy_failure_clone_keeps_complete_typed_operands() {
    let poles = [[1., 0.], [1., 2.]];
    let curves = [curve(&poles)];
    let policy = RationalIntegralOptions {
        behavior_version: 99,
        ..Default::default()
    };
    let error = certify_rational_boundary_integral(&curves, policy, || false).unwrap_err();
    assert_eq!(error, error.clone());
    assert_eq!(error.cause, RationalIntegralFailure::Policy(policy));
    let mut changed = error.clone();
    changed.curve_index = Some(0);
    assert_ne!(error, changed);
}
#[test]
fn issued_metric_weight_failure_has_value_equality_without_losing_its_payload() {
    let poles = [[1., 0.], [1., 2.]];
    let curves = [OrientedMetricCurve {
        curve: MetricCurve::Bezier(MetricBezier {
            poles: &poles,
            weights: Some(&[-2., 1.]),
        }),
        reversed: false,
    }];
    let error =
        certify_rational_boundary_integral(&curves, Default::default(), || false).unwrap_err();
    assert_eq!(error, error.clone());
    assert_weight_payload(&error.cause);
}
fn assert_weight_payload(cause: &RationalIntegralFailure) {
    let RationalIntegralFailure::Metric(metric) = cause else {
        panic!("expected metric value failure");
    };
    assert_eq!(
        metric.as_ref(),
        &ResidualError::Value {
            role: "weight",
            index: 0,
            value: -2.
        }
    );
    assert_eq!(metric, &metric.clone());
}
#[test]
fn issued_program_policy_failure_equality_keeps_requested_and_expected_bounds() {
    let poles = [[1., 0.], [1., 2.]];
    let mut policy = options();
    policy.max_components = 0;
    let error = program_error(curve(&poles), policy, || false);
    assert_eq!(error, error.clone());
    assert_eq!(*error.requested_options, policy);
    assert_eq!(
        error.cause,
        IntegralProgramFailure::Policy {
            field: "max_components",
            actual: 0,
            minimum: 1,
            maximum: 64
        }
    );
    let mut changed = error.clone();
    changed.integration_subintervals = 1;
    assert_ne!(error, changed);
}
#[test]
fn issued_work_failure_clone_preserves_actual_cells_and_original_work_limit() {
    let poles = [[1., 0.], [1., 1.], [0., 1.]];
    let rational = OrientedMetricCurve {
        curve: MetricCurve::Bezier(MetricBezier {
            poles: &poles,
            weights: Some(&[1., 1., 2.]),
        }),
        reversed: false,
    };
    let mut policy = options();
    policy.integration.absolute_error_limit = 0.;
    policy.max_total_subintervals = 1;
    let error = program_error(rational, policy, || false);
    assert_eq!(error, error.clone());
    assert_eq!(error.integration_subintervals, 1);
    assert!(
        matches!(&error.cause, IntegralProgramFailure::AggregateWork { limit: 1, cause } if matches!(cause.as_ref(), RationalIntegralFailure::WorkLimit { limit: 1, .. }))
    );
}
#[test]
fn issued_cancellation_equality_retains_the_original_root_failure() {
    let poles = [[1., 0.], [1., 2.]];
    let error = program_error(curve(&poles), options(), || true);
    assert_eq!(error, error.clone());
    assert_eq!(
        error.cause,
        IntegralProgramFailure::Numerical(Box::new(RationalIntegralFailure::Metric(Box::new(
            ResidualError::Roots(RootIsolationError::Cancelled)
        ))))
    );
    assert_eq!(error.completed_occurrences, 0);
    assert_eq!(error.integration_subintervals, 0);
}
#[test]
fn invalid_nan_clone_keeps_exact_bits_and_ordinary_nonreflexive_float_equality() {
    let payload = 0x7ff8_0000_0000_0123;
    let poles = [[f64::from_bits(payload), 0.], [1., 2.]];
    let error = program_error(curve(&poles), options(), || false);
    let copied = error.clone();
    assert_eq!(metric_value_bits(&error), payload);
    assert_eq!(metric_value_bits(&copied), payload);
    assert_ne!(error, copied);
}
fn metric_value_bits(error: &IntegralProgramError) -> u64 {
    let IntegralProgramFailure::Numerical(cause) = &error.cause else {
        panic!("expected numerical failure");
    };
    let RationalIntegralFailure::Metric(metric) = cause.as_ref() else {
        panic!("expected metric failure");
    };
    let ResidualError::Value { value, .. } = metric.as_ref() else {
        panic!("expected actual nonfinite value");
    };
    value.to_bits()
}
