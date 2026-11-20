//! Repeated complete contours test conditional joins, never family separation or topology.
use super::*;
#[test]
fn chart_joins_compose_true_closed_contours_beyond_the_legacy_total_cap() {
    let (inputs, selected, options) = chart_fixture();
    let prepared = &selected.prepared[0];
    let queries = queries(&inputs.originals[0], prepared);
    let (spans, lengths) = repeated_contours(prepared, &queries);
    let query = ConicalJoinsQuery {
        spans: &spans,
        contour_lengths: &lengths,
        options,
    };
    let domain = prepared.chart_domain.as_deref().unwrap();
    let proof = certify_conical_joins_in_chart(query, domain, || false).unwrap();
    assert!(proof.operations() <= options.max_operations);
    assert!(proof.matches_chart(&join_query(&spans, &lengths, options), domain));
    assert!(!proof.matches(&join_query(&spans, &lengths, options)));
    check_legacy_cap(&spans, &lengths, options);
    check_chart_binding(&spans, &lengths, options, domain, &proof);
    check_chart_failures(&spans, &lengths, options, domain);
    check_domain_binding(&inputs.originals, &spans, &lengths, options, &proof);
}
fn chart_fixture() -> (Inputs, Selection, ConeClearanceOptions) {
    let request = super::super::super::super::tests::request();
    let inputs = input::capture(&request, &mut || false).unwrap();
    let mut work = Work::new(request.proof.arithmetic);
    let selected = super::super::super::select_with_policy(
        &ConeRun::from(&request),
        &inputs.originals,
        WrappedConeDomainPolicy::PositiveXCommonChartV2,
        &mut work,
        &mut || false,
    )
    .unwrap();
    (inputs, selected, work.remaining())
}
fn repeated_contours<'a>(
    prepared: &'a Prepared,
    queries: &[ConicalBoundaryQuery<'a>],
) -> (Vec<ConicalJoinSpan<'a>>, Vec<usize>) {
    let base = source_spans(prepared, queries);
    let repeats = 128 / base.len() + 1;
    let spans: Vec<_> = (0..repeats).flat_map(|_| base.iter().copied()).collect();
    let lengths: Vec<_> = (0..repeats)
        .flat_map(|_| prepared.source.loops.iter().map(Vec::len))
        .collect();
    assert!(spans.len() > 128);
    assert!(lengths.len() <= 128 && lengths.iter().all(|count| *count <= 128));
    (spans, lengths)
}
fn source_spans<'a>(
    prepared: &'a Prepared,
    queries: &[ConicalBoundaryQuery<'a>],
) -> Vec<ConicalJoinSpan<'a>> {
    prepared
        .source
        .loops
        .iter()
        .flatten()
        .zip(queries)
        .map(|(piece, query)| ConicalJoinSpan {
            query: *query,
            boundary: &prepared.fits[&piece.span].boundary,
        })
        .collect()
}
fn join_query<'a>(
    spans: &'a [ConicalJoinSpan<'a>],
    lengths: &'a [usize],
    options: ConeClearanceOptions,
) -> ConicalJoinsQuery<'a> {
    ConicalJoinsQuery {
        spans,
        contour_lengths: lengths,
        options,
    }
}
fn check_legacy_cap(
    spans: &[ConicalJoinSpan<'_>],
    lengths: &[usize],
    options: ConeClearanceOptions,
) {
    assert!(
        matches!(certify_conical_joins(join_query(spans, lengths, options), || false),
        Err(ConicalJoinsFailure::SpanCount { actual }) if actual == spans.len())
    );
}
fn check_chart_binding(
    spans: &[ConicalJoinSpan<'_>],
    lengths: &[usize],
    options: ConeClearanceOptions,
    domain: &ConicalChartDomainEvidence,
    proof: &ConicalJoinsEvidence,
) {
    let mut changed = spans.to_vec();
    changed[0].query.chart_origin = changed[0].query.chart_origin.next_up();
    assert!(!proof.matches_chart(&join_query(&changed, lengths, options), domain));
    let changed = ConeClearanceOptions {
        fractional_bits: options.fractional_bits - 1,
        ..options
    };
    assert!(!proof.matches_chart(&join_query(spans, lengths, changed), domain));
    assert!(!proof.matches_chart(&join_query(spans, &lengths[1..], options), domain));
}
fn check_chart_failures(
    spans: &[ConicalJoinSpan<'_>],
    lengths: &[usize],
    options: ConeClearanceOptions,
    domain: &ConicalChartDomainEvidence,
) {
    check_error_offset(spans, lengths, options, domain);
    let error =
        certify_conical_joins_in_chart(join_query(spans, lengths, options), domain, || true)
            .unwrap_err();
    assert!(error.is_cancelled());
    let exhausted = ConeClearanceOptions {
        max_operations: 0,
        ..options
    };
    let error =
        certify_conical_joins_in_chart(join_query(spans, lengths, exhausted), domain, || false)
            .unwrap_err();
    assert_eq!(error.operations, 0);
    assert!(!error.is_cancelled());
}

fn check_error_offset(
    spans: &[ConicalJoinSpan<'_>],
    lengths: &[usize],
    options: ConeClearanceOptions,
    domain: &ConicalChartDomainEvidence,
) {
    let mut changed = spans.to_vec();
    let offset = lengths[0];
    changed[offset].query.depths[0] = changed[offset].query.depths[0].next_up();
    let error =
        certify_conical_joins_in_chart(join_query(&changed, lengths, options), domain, || true)
            .unwrap_err();
    assert_eq!(error.index, Some(offset));
    assert!(!error.is_cancelled());
}

fn check_domain_binding(
    originals: &[DevelopedRegionRequest],
    spans: &[ConicalJoinSpan<'_>],
    lengths: &[usize],
    options: ConeClearanceOptions,
    proof: &ConicalJoinsEvidence,
) {
    let mut request = super::super::super::super::tests::request();
    request.support.chart_origin += 0.125;
    let mut work = Work::new(request.proof.arithmetic);
    let changed = domain::certify(
        &ConeRun::from(&request),
        originals,
        WrappedConeDomainPolicy::PositiveXCommonChartV2,
        &mut work,
        &mut || false,
    )
    .unwrap();
    assert!(!proof.matches_chart(
        &join_query(spans, lengths, options),
        changed.chart().unwrap()
    ));
}
