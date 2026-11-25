// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::{
    IntegralProgramFailure, IntegralProgramVersion, MetricBezier, MetricCurve,
    RationalIntegralOptions,
};

fn options() -> IntegralProgramOptions {
    IntegralProgramOptions {
        version: IntegralProgramVersion::V1,
        integration: RationalIntegralOptions::default(),
        max_components: 8,
        max_total_subintervals: 4096,
    }
}
fn curves(poles: &[[[f64; 2]; 2]]) -> Vec<OrientedMetricCurve<'_>> {
    poles
        .iter()
        .map(|poles| OrientedMetricCurve {
            curve: MetricCurve::Bezier(MetricBezier {
                poles,
                weights: None,
            }),
            reversed: false,
        })
        .collect()
}
fn rectangle() -> Vec<[[f64; 2]; 2]> {
    let mut poles: Vec<_> = (0..83)
        .map(|index| [[index as f64 / 83., 0.], [(index + 1) as f64 / 83., 0.]])
        .collect();
    poles.extend([
        [[1., 0.], [1., 1.]],
        [[1., 1.], [0., 1.]],
        [[0., 1.], [0., 0.]],
    ]);
    poles
}
fn rational(poles: &[[f64; 2]]) -> OrientedMetricCurve<'_> {
    OrientedMetricCurve {
        curve: MetricCurve::Bezier(MetricBezier {
            poles,
            weights: Some(&[1., 1., 2.]),
        }),
        reversed: false,
    }
}
#[test]
fn consecutive_chunks_bind_actual_large_boundary_and_spend_no_rational_cells() {
    let poles = rectangle();
    let source = curves(&poles);
    let mut ledger = IntegralLedger::new(options());
    let proof = ledger.boundary(&source, &mut || false).unwrap();
    assert_eq!(proof.enclosure(), [1., 1.]);
    assert_eq!(proof.completed_occurrences(), 86);
    assert_eq!(ledger.consumed(), 0);
    assert_eq!(ledger.options(), options());
    assert!(matches_boundary(&proof, &source));
    assert!(!matches_face(&proof, &[&source]));
}
#[test]
fn face_chunks_keep_whole_outer_and_hole_roles_with_owned_source_binding() {
    let outer = rectangle();
    let inner: Vec<_> = outer
        .iter()
        .map(|pair| pair.map(|p| p.map(|x| x * 0.5)))
        .collect();
    let outer = curves(&outer);
    let mut inner = curves(&inner);
    let mut ledger = IntegralLedger::new(options());
    let proof = ledger.face(&[&outer, &inner], &mut || false).unwrap();
    assert_eq!(proof.enclosure(), [0.75, 0.75]);
    assert!(matches_face(&proof, &[&outer, &inner]));
    assert!(!matches_face(&proof, &[&inner, &outer]));
    inner[0].reversed = true;
    assert!(!matches_face(&proof, &[&outer, &inner]));
}
#[test]
fn repeated_real_rational_programs_receive_remaining_graph_cells() {
    let poles = [[1., 0.], [1., 1.], [0., 1.]];
    let source = [rational(&poles)];
    let mut ledger = IntegralLedger::new(options());
    let first = ledger.boundary(&source, &mut || false).unwrap();
    let spent = first.integration_subintervals();
    assert!(spent > 0);
    let second = ledger.boundary(&source, &mut || false).unwrap();
    assert_eq!(
        second.requested_options().max_total_subintervals,
        options().max_total_subintervals - spent
    );
    assert_eq!(ledger.consumed(), spent + second.integration_subintervals());
    assert!(matches_boundary(&first, &source) && matches_boundary(&second, &source));
}
#[test]
fn failed_program_cells_are_charged_once_and_later_loan_can_be_zero() {
    let poles = [[1., 0.], [1., 1.], [0., 1.]];
    let source = [rational(&poles)];
    let mut policy = options();
    policy.integration.absolute_error_limit = 0.;
    policy.max_total_subintervals = 1;
    let mut ledger = IntegralLedger::new(policy);
    let first = ledger.boundary(&source, &mut || false).unwrap_err();
    assert_eq!(first.reported_subintervals(), 1);
    assert_eq!(ledger.consumed(), 1);
    let second = ledger.boundary(&source, &mut || false).unwrap_err();
    assert_eq!(second.reported_subintervals(), 0);
    assert_eq!(ledger.consumed(), 1);
    assert_zero_grant_line_completes(&mut ledger);
}
#[test]
fn cancellation_preserves_original_typed_source_and_graph_work() {
    use std::error::Error;
    let poles = [[[1., 0.], [1., 2.]]];
    let mut ledger = IntegralLedger::new(options());
    let failure = ledger.boundary(&curves(&poles), &mut || true).unwrap_err();
    assert!(failure.is_cancelled());
    assert!(failure.source().is_some());
    assert_eq!(failure.reported_subintervals(), 0);
    assert_eq!(ledger.consumed(), 0);
}
#[test]
fn invalid_zero_chunk_width_is_a_math_policy_error_without_a_panic() {
    let poles = [[[1., 0.], [1., 2.]]];
    let mut policy = options();
    policy.integration.max_curves = 0;
    let mut ledger = IntegralLedger::new(policy);
    let error = ledger.boundary(&curves(&poles), &mut || false).unwrap_err();
    assert!(!error.is_cancelled());
    assert_eq!(error.reported_subintervals(), 0);
    assert!(
        matches!(error, IntegralLedgerError::Math(cause) if matches!(cause.cause, IntegralProgramFailure::Numerical(_)))
    );
}
#[test]
fn inconsistent_charge_poison_prevents_any_later_success_or_budget_reset() {
    let mut policy = options();
    policy.max_total_subintervals = 1;
    let mut ledger = IntegralLedger::new(policy);
    let failure = ledger.record(2).unwrap_err();
    assert!(matches!(
        failure,
        IntegralLedgerError::Accounting {
            grant: 1,
            consumed: 0,
            reported: 2
        }
    ));
    assert_eq!(ledger.consumed(), 0);
    let poles = [[[1., 0.], [1., 2.]]];
    assert!(matches!(
        ledger.boundary(&curves(&poles), &mut || false),
        Err(IntegralLedgerError::Accounting { .. })
    ));
}
#[test]
fn original_math_failure_is_primary_when_its_charge_cannot_reconcile() {
    let poles = [[1., 0.], [1., 1.], [0., 1.]];
    let source = [rational(&poles)];
    let mut policy = options();
    policy.max_total_subintervals = 1;
    policy.integration.absolute_error_limit = 0.;
    let mut original = IntegralLedger::new(policy);
    let IntegralLedgerError::Math(cause) = original.boundary(&source, &mut || false).unwrap_err()
    else {
        panic!("expected actual Math refusal");
    };
    policy.max_total_subintervals = 0;
    let mut inconsistent = IntegralLedger::new(policy);
    let result = inconsistent.settle(Err(*cause)).unwrap_err();
    assert!(matches!(result, IntegralLedgerError::Math(_)));
    assert!(inconsistent.loan().is_err());
}
#[test]
fn component_limit_applies_to_the_whole_face_including_holes() {
    let poles = rectangle();
    let source = curves(&poles);
    let mut policy = options();
    policy.max_components = 3;
    let mut ledger = IntegralLedger::new(policy);
    let failure = ledger.face(&[&source, &source], &mut || false).unwrap_err();
    assert_eq!(ledger.consumed(), 0);
    assert!(
        matches!(failure, IntegralLedgerError::Math(cause) if matches!(cause.cause, IntegralProgramFailure::ComponentCount { actual: 4, limit: 3 }))
    );
}

fn assert_zero_grant_line_completes(ledger: &mut IntegralLedger) {
    let lines = [[[1., 0.], [1., 2.]]];
    assert_eq!(
        ledger
            .boundary(&curves(&lines), &mut || false)
            .unwrap()
            .completed_occurrences(),
        1
    );
}
