// SPDX-License-Identifier: GPL-2.0-only
//! Rational perturbations, representation fallback and actual work/cancellation controls.
use super::super::boundary_chart::tests::{domain, options, poles, query};
use super::*;
fn execute<T>(body: impl FnOnce(&mut Arithmetic<'_>) -> T) -> T {
    let mut never = || false;
    let mut arithmetic = Arithmetic::new(sector_api::policy(options()), &mut never);
    body(&mut arithmetic)
}
fn coefficients(values: [[f64; 2]; 4], arithmetic: &mut Arithmetic<'_>) -> Coefficients {
    values.map(|axis| {
        axis.into_iter()
            .map(|v| arithmetic.scalar(v).unwrap())
            .collect()
    })
}
fn context(arithmetic: &mut Arithmetic<'_>) -> PairStorage {
    let poles = poles();
    prepare::capture(&query(&poles).edges[0], &domain(0.125), arithmetic).unwrap()
}
#[test]
fn independent_nonzero_coordinate_and_weight_perturbations_are_included() {
    execute(|arithmetic| {
        let ideal = coefficients([[80., 80.], [0., 0.], [0., 0.], [1., 1.]], arithmetic);
        let stored = coefficients([[80., 82.], [0., 0.], [0., 0.], [1., 2.]], arithmetic);
        let mut context = context(arithmetic);
        context.denominators.error =
            bound::difference_bound(&ideal[3], &stored[3], arithmetic).unwrap();
        let cutoff = arithmetic.scalar(100.).unwrap();
        let result = bound::conditioned(&ideal, &stored, &context, &cutoff, arithmetic)
            .unwrap()
            .unwrap();
        let upper = arithmetic.upper_f64(result.upper).unwrap();
        let endpoint_error = (80_f64 - 82_f64 / 2.).abs();
        assert!(upper >= endpoint_error);
        assert!(upper >= 82. && upper <= 82_f64.next_up());
    });
}
#[test]
fn unequal_degrees_and_rescaled_gauges_use_general_residual() {
    execute(|arithmetic| {
        let ideal = coefficients([[80., 80.], [0., 0.], [0., 0.], [1., 1.]], arithmetic);
        let scaled = coefficients([[160., 160.], [0., 0.], [0., 0.], [2., 2.]], arithmetic);
        let mut context = context(arithmetic);
        context.denominators.error =
            bound::difference_bound(&ideal[3], &scaled[3], arithmetic).unwrap();
        let cutoff = arithmetic.scalar(1e-7).unwrap();
        assert_fallback(&ideal, &scaled, &context, &cutoff, arithmetic);
        assert!(
            storage::residual_sum(&ideal, &scaled, arithmetic)
                .unwrap()
                .upper
                <= 1.into()
        );
        let elevated = std::array::from_fn(|axis| vec![ideal[axis][0].clone(); 3]);
        assert_fallback(&ideal, &elevated, &context, &cutoff, arithmetic);
    });
}
#[test]
fn paired_receipt_has_less_actual_work_than_both_legacy_storage_calls() {
    let poles = poles();
    let query = query(&poles);
    let context = domain(0.125);
    let new = execute(|arithmetic| {
        pair(&query, &context, arithmetic).unwrap();
        arithmetic.operations()
    });
    let old = execute(|arithmetic| {
        for edge in &query.edges {
            storage::storage_discrepancy(edge, arithmetic).unwrap();
        }
        arithmetic.operations()
    });
    assert!(
        new < old,
        "actual pair work {new}; independent legacy work {old}"
    );
}
#[test]
fn negative_and_unresolved_tiny_actual_weights_never_certify() {
    for weight in [-1., f64::MIN_POSITIVE] {
        refuses_weight(weight);
    }
}
fn refuses_weight(weight: f64) {
    let poles = poles();
    let mut query = query(&poles);
    let weights = [weight; 2];
    for edge in &mut query.edges {
        edge.stored_weights = &weights;
    }
    execute(|arithmetic| {
        let error = pair(&query, &domain(0.125), arithmetic).unwrap_err();
        assert!(matches!(
            error,
            CombinedFailure::Storage {
                depth: 0,
                cause: Failure::Weight { .. }
            }
        ));
    });
}
#[test]
fn first_depth_failure_precedes_any_second_depth_cancellation() {
    let poles = poles();
    let mut query = query(&poles);
    query.edges[0].stored_poles = &[[f64::NAN, 0., 0.]; 2];
    let mut polls = 0;
    let mut stop = || {
        polls += 1;
        true
    };
    let mut arithmetic = Arithmetic::new(sector_api::policy(options()), &mut stop);
    let error = pair(&query, &domain(0.125), &mut arithmetic).unwrap_err();
    assert!(matches!(
        error,
        CombinedFailure::Storage {
            depth: 0,
            cause: Failure::Nonfinite { .. }
        }
    ));
    drop(arithmetic);
    assert_eq!(polls, 0);
}
#[test]
fn actual_remaining_work_cannot_publish_a_pair() {
    let poles = poles();
    let query = query(&poles);
    let domain = domain(0.125);
    let completed = execute(|arithmetic| {
        pair(&query, &domain, arithmetic).unwrap();
        arithmetic.operations()
    });
    for limit in [0, completed - 1] {
        refuses_work(&query, &domain, limit);
    }
}
fn refuses_work(query: &ConicalBoundaryQuery<'_>, domain: &ConicalChartDomainEvidence, limit: u32) {
    let mut never = || false;
    let mut policy = sector_api::policy(options());
    policy.max_operations = limit;
    let mut arithmetic = Arithmetic::new(policy, &mut never);
    assert!(pair(query, domain, &mut arithmetic).is_err());
    assert_eq!(arithmetic.operations(), limit);
}

#[test]
fn cancellation_at_the_final_actual_pair_checkpoint_returns_no_partial_result() {
    let (completed, polls) = poll_pair(usize::MAX);
    assert!(completed.is_ok());
    let (cancelled, observed) = poll_pair(polls);
    assert_eq!(observed, polls);
    assert!(matches!(
        cancelled,
        Err(CombinedFailure::Storage {
            depth: 1,
            cause: Failure::Arithmetic(BoundedArithmeticFailure::Cancelled)
        })
    ));
}
fn poll_pair(stop: usize) -> (Result<[Interval; 2], CombinedFailure>, usize) {
    let poles = poles();
    let query = query(&poles);
    let domain = domain(0.125);
    let mut polls = 0;
    let mut callback = || {
        polls += 1;
        polls == stop
    };
    let mut arithmetic = Arithmetic::new(sector_api::policy(options()), &mut callback);
    let result = pair(&query, &domain, &mut arithmetic);
    drop(arithmetic);
    (result, polls)
}

fn pair(
    query: &ConicalBoundaryQuery<'_>,
    domain: &ConicalChartDomainEvidence,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<[Interval; 2], CombinedFailure> {
    certify(
        &super::super::boundary::combined_request(query),
        domain,
        arithmetic,
    )
}
fn assert_fallback(
    ideal: &Coefficients,
    stored: &Coefficients,
    context: &PairStorage,
    cutoff: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) {
    assert!(
        bound::conditioned(ideal, stored, context, cutoff, arithmetic)
            .unwrap()
            .is_none()
    );
}
