// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::{RootIsolationError, metric_bounds::ResidualError};

#[test]
fn equal_cap_moments_preserve_second_occurrence_polls_and_typed_depth() {
    let lower = [[0., 0.], [0.5, 0.25], [1., 0.75], [1.125, 1.]];
    let mut upper = lower;
    upper[0][0] = -0.;
    let paired = occurrence([&lower, &lower]);
    let independent = occurrence([&lower, &upper]);
    let mut paired = query(&paired);
    paired.carriers[1].radius = 2.5;
    let independent = AnalyticConicalFluxQuery {
        spans: &independent,
        ..paired
    };

    let (reference, total_polls) = complete(independent);
    let (measured, measured_polls) = complete(paired);
    assert_eq!(
        measured.enclosure().map(f64::to_bits),
        reference.enclosure().map(f64::to_bits)
    );
    assert_eq!(
        measured.arithmetic_operations(),
        reference.arithmetic_operations()
    );
    assert_eq!(measured.span_work(), reference.span_work());
    assert_eq!(measured_polls, total_polls);
    assert!(measured.matches_source(paired));
    assert!(!measured.matches_source(independent));

    let cap_polls = moment_polls(&lower);
    // In V1 the successful cap work is followed only by the last publication poll.
    for stop in [total_polls - cap_polls, total_polls - 1] {
        assert_eq!(
            cancel_at_second_cap(independent, stop),
            cancel_at_second_cap(paired, stop)
        );
    }
}

fn occurrence(cap_pcurves: [&[[f64; 2]]; 2]) -> [AnalyticConicalFluxSpan<'_>; 1] {
    [AnalyticConicalFluxSpan {
        cap_pcurves,
        cap_reversed: [true, false],
        ..span(&A, &B, None)
    }]
}

fn complete(query: AnalyticConicalFluxQuery<'_>) -> (CertifiedAnalyticConicalFlux, usize) {
    let mut polls = 0;
    let complete = certify_analytic_conical_boundary_flux(query, || {
        polls += 1;
        false
    })
    .unwrap();
    (complete, polls)
}

fn moment_polls(poles: &[[f64; 2]]) -> usize {
    let mut polls = 0;
    caps::moments::integrate(poles, &mut || {
        polls += 1;
        false
    })
    .unwrap();
    polls
}

fn cancel_at_second_cap(query: AnalyticConicalFluxQuery<'_>, stop: usize) -> (u32, u64) {
    let mut polls = 0;
    let error = certify_analytic_conical_boundary_flux(query, || {
        polls += 1;
        polls == stop
    })
    .unwrap_err();
    assert_eq!(polls, stop);
    let AnalyticConicalFluxFailure::Cap {
        span: 0,
        depth: 1,
        cause,
    } = error.cause
    else {
        panic!("expected second-cap cancellation at poll {stop}: {error:?}");
    };
    let PolynomialIntegralFailure::Metric(cause) = *cause else {
        panic!("expected original metric cancellation at poll {stop}: {cause:?}");
    };
    assert!(matches!(
        *cause,
        ResidualError::Roots(RootIsolationError::Cancelled)
    ));
    (error.operations, error.integration_subintervals)
}
