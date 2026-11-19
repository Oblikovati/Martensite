// SPDX-License-Identifier: GPL-2.0-only
use super::*;
fn rejects(
    pair: ConicalTrimPairQuery<'_>,
    source: ConicalBoundaryQuery<'_>,
    boundary: &ConicalBoundaryEvidence,
    domain: &ConicalChartDomainEvidence,
) {
    let failure =
        certify_conical_trim_pair_in_chart(pair, source, boundary, domain, || true).unwrap_err();
    assert_eq!(failure.operations, 0);
    assert!(!failure.is_cancelled());
    assert!(matches!(
        failure.cause.as_ref(),
        ConicalChartTrimPairFailure::Binding { .. }
    ));
}
#[test]
fn actual_decoded_poles_and_weights_bind_before_arithmetic_or_cancel() {
    with_fixture(|pair, source, boundary, domain| {
        let mut poles = source.edges[0].stored_poles.to_vec();
        poles[0][0] = f64::from_bits(poles[0][0].to_bits() + 1);
        let mut changed = pair;
        changed.curves[0].poles = &poles;
        rejects(changed, source, boundary, domain);
        changed = pair;
        changed.curves[0].weights = None;
        rejects(changed, source, boundary, domain);
        changed.curves[0].weights = Some(&[1., 2.]);
        rejects(changed, source, boundary, domain);
    });
}
#[test]
fn boundary_source_fit_depth_and_sense_mutations_cannot_reuse_authority() {
    with_fixture(|pair, source, boundary, domain| {
        let mut changed = source;
        changed.source = &[[81., 0.], [80., 0.]];
        rejects(pair, changed, boundary, domain);
        changed = source;
        changed.angular_fit = &[0., 0.01];
        rejects(pair, changed, boundary, domain);
        changed = source;
        changed.slant_numerator = &[81., 80.];
        rejects(pair, changed, boundary, domain);
        changed = source;
        changed.slant_weights = &[1., 2.];
        rejects(pair, changed, boundary, domain);
        support_mutations(pair, source, boundary, domain);
    });
}
fn support_mutations(
    pair: ConicalTrimPairQuery<'_>,
    source: ConicalBoundaryQuery<'_>,
    boundary: &ConicalBoundaryEvidence,
    domain: &ConicalChartDomainEvidence,
) {
    let mut changed = source;
    changed.depths[0] = 0.;
    rejects(pair, changed, boundary, domain);
    changed = source;
    changed.outward = !source.outward;
    rejects(pair, changed, boundary, domain);
    changed = source;
    changed.physical_turn = 1;
    rejects(pair, changed, boundary, domain);
    changed = source;
    changed.fit_center = -0.;
    rejects(pair, changed, boundary, domain);
}
#[test]
fn precision_domain_and_legacy_receipts_are_strictly_distinct() {
    with_fixture(|mut pair, source, boundary, domain| {
        pair.options.fractional_bits = 127;
        rejects(pair, source, boundary, domain);
        pair.options.fractional_bits = 128;
        let other = fixture::domain(0.25);
        rejects(pair, source, boundary, &other);
        let legacy = certify_conical_boundary(source, fixture::options(), || false).unwrap();
        rejects(pair, source, &legacy, domain);
    });
}
#[test]
fn carrier_angle_mutation_has_the_actual_failed_trace() {
    with_fixture(|mut pair, source, boundary, domain| {
        pair.carriers[0].semi_angle = 0.25;
        let failure = certify_conical_trim_pair_in_chart(pair, source, boundary, domain, || true)
            .unwrap_err();
        assert_eq!((failure.depth, failure.operations), (Some(0), 0));
        assert!(matches!(
            failure.cause.as_ref(),
            ConicalChartTrimPairFailure::Binding {
                field: "actual carrier semi-angle",
                ..
            }
        ));
    });
}
