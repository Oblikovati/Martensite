//! The admitted Phase A reference closes in one ideal chart before native construction.
use super::*;
fn fixture() -> (Inputs, Selection, ConeClearanceOptions) {
    let request = super::super::super::tests::request();
    let inputs = input::capture(&request, &mut || false).unwrap();
    let mut work = Work::new(request.proof.arithmetic);
    let selected =
        super::super::select(&request, &inputs.originals, &mut work, &mut || false).unwrap();
    (inputs, selected, work.remaining())
}
#[test]
fn common_chart_curved_corner_closes_and_the_existing_join_theorem_completes() {
    let (inputs, mut selected, options) = fixture();
    let prepared = &selected.prepared[0];
    check_endpoints(prepared);
    let queries = queries(&inputs.originals[0], prepared);
    assert!(joins(prepared, &queries, options).is_ok());
    check_snapshot_mutations(prepared, &queries, options);
    selected.prepared[0].source.loops[0][0].span.curve = SurfaceCurveId(999);
    assert!(matches!(
        history::certify_developed(&inputs.originals, &selected.prepared, &[]),
        Err(history::HistoryFailure::SourceBinding)
    ));
}
fn check_endpoints(prepared: &Prepared) {
    for contour in &prepared.source.loops {
        for (i, piece) in contour.iter().enumerate() {
            let first = prepared.fits[&piece.span].fit.view();
            let next = prepared.fits[&contour[(i + 1) % contour.len()].span]
                .fit
                .view();
            assert_eq!(first.chart_center, prepared.chart_center);
            assert_eq!(first.chart_center, next.chart_center);
            assert_eq!(first.tangent[3], next.tangent[0]);
            assert_eq!(
                first.slant_numerator[3] / first.slant_weights[3],
                next.slant_numerator[0] / next.slant_weights[0]
            );
        }
    }
}
fn queries<'a>(
    original: &'a DevelopedRegionRequest,
    prepared: &'a Prepared,
) -> Vec<ConicalBoundaryQuery<'a>> {
    prepared
        .source
        .loops
        .iter()
        .flatten()
        .map(|piece| query_for_piece(original, prepared, piece))
        .collect()
}
fn query_for_piece<'a>(
    original: &'a DevelopedRegionRequest,
    prepared: &'a Prepared,
    piece: &'a AuthoredPiece,
) -> ConicalBoundaryQuery<'a> {
    let authored = original
        .loops
        .iter()
        .flatten()
        .find(|p| p.span.curve == piece.span.curve)
        .unwrap();
    let fit = &prepared.fits[&piece.span];
    boundary_query(
        prepared,
        authored,
        piece,
        &fit.fit.view(),
        &fit.poles,
        fit.developed_turn,
    )
}

fn joins(
    prepared: &Prepared,
    queries: &[ConicalBoundaryQuery<'_>],
    options: ConeClearanceOptions,
) -> Result<ConicalJoinsEvidence, ConicalJoinsFailure> {
    let pieces = prepared.source.loops.iter().flatten();
    let spans: Vec<_> = pieces
        .zip(queries)
        .map(|(piece, query)| ConicalJoinSpan {
            query: *query,
            boundary: &prepared.fits[&piece.span].boundary,
        })
        .collect();
    let lengths: Vec<_> = prepared.source.loops.iter().map(Vec::len).collect();
    certify_conical_joins(
        ConicalJoinsQuery {
            spans: &spans,
            contour_lengths: &lengths,
            options,
        },
        || false,
    )
}
fn check_snapshot_mutations(
    prepared: &Prepared,
    queries: &[ConicalBoundaryQuery<'_>],
    options: ConeClearanceOptions,
) {
    let mut tangent = queries[0].angular_fit.to_vec();
    tangent[0] = tangent[0].next_up();
    let mut slant = queries[0].slant_numerator.to_vec();
    slant[0] = slant[0].next_up();
    for mutation in 0..4 {
        let mut changed = queries.to_vec();
        match mutation {
            0 => changed[0].angular_fit = &tangent,
            1 => changed[0].slant_numerator = &slant,
            2 => changed[0].fit_center = changed[0].fit_center.next_up(),
            3 => changed[0].original.as_mut().unwrap().index += 1,
            _ => panic!("mutation={mutation}; expected 0..4"),
        }
        assert!(matches!(
            joins(prepared, &changed, options),
            Err(ConicalJoinsFailure::Boundary { index: 0 })
        ));
    }
}
