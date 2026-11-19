// SPDX-License-Identifier: GPL-2.0-only
use super::super::super::{ConicalTrimCarrier, CylindricalProfileOptions, MetricBezier};
use super::super::boundary_chart::tests as fixture;
use super::*;
mod mutations;
mod numerical;
mod reuse;
fn with_fixture(
    body: impl FnOnce(
        ConicalTrimPairQuery<'_>,
        ConicalBoundaryQuery<'_>,
        &ConicalBoundaryEvidence,
        &ConicalChartDomainEvidence,
    ),
) {
    let poles = fixture::poles();
    let source = fixture::query(&poles);
    let domain = fixture::domain(source.semi_angle);
    let boundary =
        certify_conical_boundary_in_chart(source, &domain, fixture::options(), || false).unwrap();
    let pair = pair(source);
    body(pair, source, &boundary, &domain);
}
fn pair(source: ConicalBoundaryQuery<'_>) -> ConicalTrimPairQuery<'_> {
    ConicalTrimPairQuery {
        curves: source.edges.map(|edge| MetricBezier {
            poles: edge.stored_poles,
            weights: Some(edge.stored_weights),
        }),
        pcurves: [&[[0., 0.], [0., 0.]]; 2],
        carriers: source.edges.map(|edge| ConicalTrimCarrier {
            radius: edge.stored_poles[0][0],
            axial_origin: edge.stored_poles[0][2],
            semi_angle: edge.semi_angle,
        }),
        spatial_limits: [1e-4; 2],
        options: CylindricalProfileOptions {
            fractional_bits: 128,
            max_operations: 100_000,
            ..CylindricalProfileOptions::default()
        },
    }
}
#[test]
fn actual_decoded_chart_pair_has_owned_bound_receipts_and_partitioned_work() {
    with_fixture(|pair, source, boundary, domain| {
        let proof =
            certify_conical_trim_pair_in_chart(pair, source, boundary, domain, || false).unwrap();
        assert!(proof.matches_source(pair));
        let work = proof.operations();
        let traces = proof.into_traces();
        assert_eq!(work, traces.iter().map(|trace| trace.operations()).sum());
        assert!(work > 0);
        assert!(traces.iter().all(|trace| trace.residual_bounds()[2] < 1e-4));
    });
}
#[test]
fn zero_work_and_cancellation_keep_the_original_arithmetic_cause() {
    with_fixture(|mut pair, source, boundary, domain| {
        pair.options.max_operations = 0;
        let failure = certify_conical_trim_pair_in_chart(pair, source, boundary, domain, || false)
            .unwrap_err();
        assert_eq!((failure.depth, failure.operations), (Some(0), 0));
        assert!(!failure.is_cancelled());
        let failure = certify_conical_trim_pair_in_chart(pair, source, boundary, domain, || true)
            .unwrap_err();
        assert!(failure.is_cancelled());
    });
}
#[test]
fn second_trace_admission_follows_completed_first_work() {
    with_fixture(|mut pair, source, boundary, domain| {
        pair.pcurves[1] = &[[f64::NAN, 0.], [0., 0.]];
        let failure = certify_conical_trim_pair_in_chart(pair, source, boundary, domain, || false)
            .unwrap_err();
        assert_eq!(failure.depth, Some(1));
        assert!(failure.operations > 0);
        assert!(!failure.is_cancelled());
    });
}
#[test]
fn real_first_failure_precedes_invalid_second_input_and_later_cancel() {
    with_fixture(|mut pair, source, boundary, domain| {
        pair.spatial_limits[0] = 0.;
        pair.pcurves[1] = &[[f64::NAN, 0.], [0., 0.]];
        let failure = certify_conical_trim_pair_in_chart(pair, source, boundary, domain, || false)
            .unwrap_err();
        assert_eq!(failure.depth, Some(0));
        assert!(matches!(
            failure.cause.as_ref(),
            ConicalChartTrimPairFailure::Trace(ConicalTrimError {
                cause: CylindricalProfileFailure::Uncertified { .. },
                ..
            })
        ));
    });
}
#[test]
fn tight_budget_reports_exact_completed_work_without_partial_pair() {
    with_fixture(|mut pair, source, boundary, domain| {
        pair.options.max_operations = 3;
        let failure = certify_conical_trim_pair_in_chart(pair, source, boundary, domain, || false)
            .unwrap_err();
        assert_eq!(failure.operations, 3);
        assert!(matches!(
            failure.cause.as_ref(),
            ConicalChartTrimPairFailure::Trace(ConicalTrimError {
                cause: CylindricalProfileFailure::Arithmetic(_),
                ..
            })
        ));
    });
}
#[test]
fn chart_receipts_and_failures_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<CertifiedConicalTrimPair>();
    assert_send_sync::<ConicalChartTrimPairError>();
    assert_send_sync::<ConicalChartTrimPairFailure>();
}

mod developed_material;
