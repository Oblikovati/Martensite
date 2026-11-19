// SPDX-License-Identifier: GPL-2.0-only
//! Signed zero forces a genuinely different capture with identical exact geometry.
use super::*;
#[test]
fn pair_local_reuse_matches_cold_bounds_and_charges_only_executed_work() {
    with_fixture(|pair, source, boundary, domain| {
        let warm =
            certify_conical_trim_pair_in_chart(pair, source, boundary, domain, || false).unwrap();
        let mut cold_pair = pair;
        cold_pair.pcurves[1] = &[[-0., 0.], [0., 0.]];
        let cold =
            certify_conical_trim_pair_in_chart(cold_pair, source, boundary, domain, || false)
                .unwrap();
        assert!(warm.matches_source(pair) && !warm.matches_source(cold_pair));
        assert!(cold.matches_source(cold_pair) && !cold.matches_source(pair));
        compare_receipts(warm, cold);
    });
}
fn compare_receipts(warm: CertifiedConicalTrimPair, cold: CertifiedConicalTrimPair) {
    let warm_work = warm.operations();
    let cold_work = cold.operations();
    assert!(warm_work < cold_work);
    let warm = warm.into_traces();
    let cold = cold.into_traces();
    assert_eq!(warm_work, warm.iter().map(|trace| trace.operations()).sum());
    assert_eq!(cold_work, cold.iter().map(|trace| trace.operations()).sum());
    assert_eq!(warm[0].operations(), cold[0].operations());
    assert!(warm[1].operations() < cold[1].operations());
    assert_eq!(
        warm.map(|trace| trace.residual_bounds()),
        cold.map(|trace| trace.residual_bounds())
    );
}
#[test]
fn independently_changed_u_and_v_do_not_reuse_the_first_trace_coefficients() {
    with_fixture(|pair, source, boundary, domain| {
        changed_coordinate(pair, source, boundary, domain, &[[1e-7, 0.], [1e-7, 0.]], 0);
        changed_coordinate(pair, source, boundary, domain, &[[0., 1e-6], [0., 1e-6]], 1);
    });
}
fn changed_coordinate<'a>(
    mut pair: ConicalTrimPairQuery<'a>,
    source: ConicalBoundaryQuery<'a>,
    boundary: &ConicalBoundaryEvidence,
    domain: &ConicalChartDomainEvidence,
    uv: &'a [[f64; 2]],
    channel: usize,
) {
    pair.pcurves[1] = uv;
    let proof =
        certify_conical_trim_pair_in_chart(pair, source, boundary, domain, || false).unwrap();
    assert!(proof.matches_source(pair));
    let traces = proof.into_traces();
    assert!(traces[1].residual_bounds()[channel] > traces[0].residual_bounds()[channel]);
    assert!(traces[1].operations() >= traces[0].operations());
}
#[test]
fn cancellation_at_the_final_actual_reused_operation_returns_no_pair() {
    with_fixture(|pair, source, boundary, domain| {
        let mut polls = 0;
        certify_conical_trim_pair_in_chart(pair, source, boundary, domain, || {
            polls += 1;
            false
        })
        .unwrap();
        let mut current = 0;
        let failure = certify_conical_trim_pair_in_chart(pair, source, boundary, domain, || {
            current += 1;
            current == polls
        })
        .unwrap_err();
        assert!(failure.is_cancelled());
        assert_eq!(failure.depth, Some(1));
        assert!(failure.operations > 0);
    });
}
#[test]
fn actual_reused_work_limit_refuses_at_the_original_second_depth_budget() {
    with_fixture(|mut pair, source, boundary, domain| {
        let proof =
            certify_conical_trim_pair_in_chart(pair, source, boundary, domain, || false).unwrap();
        pair.options.max_operations = proof.operations() - 1;
        let failure = certify_conical_trim_pair_in_chart(pair, source, boundary, domain, || false)
            .unwrap_err();
        assert_eq!(failure.depth, Some(1));
        assert_eq!(failure.operations, pair.options.max_operations);
        assert!(!failure.is_cancelled());
    });
}
