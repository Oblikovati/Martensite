//! Original intervals and fitted charts are re-bound to decoded depth-edge coefficients.
use super::*;
pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn query<'a>(
    original: &'a DevelopedRegionRequest,
    prepared: &'a Prepared,
    piece: &'a AuthoredPiece,
    curves: &'a [retained::OwnedCurve; 2],
) -> ConeChecked<ConicalBoundaryQuery<'a>> {
    let fit = prepared
        .fits
        .get(&piece.span)
        .ok_or_else(|| missing(original.region, piece.span, "frozen fit membership"))?;
    let original = original_interval(original, piece.span)?;
    Ok(BoundaryInput {
        prepared,
        piece,
        fit,
        original,
    }
    .query(curves))
}
fn original_interval(
    original: &DevelopedRegionRequest,
    span: CurveSpan,
) -> ConeChecked<ConicalOriginalSpan<'_>> {
    let authored = original
        .loops
        .iter()
        .flatten()
        .find(|source| source.span.curve == span.curve)
        .ok_or_else(|| missing(original.region, span, "authored span membership"))?;
    Ok(ConicalOriginalSpan {
        poles: &authored.poles,
        index: u64::from(span.index),
        level: span.level,
    })
}
fn missing(region: SurfaceRegionId, span: CurveSpan, field: &'static str) -> ConeFailure {
    mismatch(region, Some(span), field, vec![0], vec![1])
}
struct BoundaryInput<'a> {
    prepared: &'a Prepared,
    piece: &'a AuthoredPiece,
    fit: &'a SpanFit,
    original: ConicalOriginalSpan<'a>,
}
impl<'a> BoundaryInput<'a> {
    fn query(&self, curves: &'a [retained::OwnedCurve; 2]) -> ConicalBoundaryQuery<'a> {
        let fit: &'a cone_mapping::fit::OwnedFit = &self.fit.fit;
        let view = fit.view();
        ConicalBoundaryQuery {
            source: &self.piece.poles,
            original: Some(self.original),
            angular_fit: view.tangent,
            slant_numerator: view.slant_numerator,
            slant_weights: view.slant_weights,
            semi_angle: view.semi_angle,
            chart_origin: self.prepared.chart_origin,
            fit_center: view.chart_center,
            developed_turn: self.fit.developed_turn,
            physical_turn: 0,
            depths: self.prepared.source.depths,
            outward: true,
            spatial_limit: self.prepared.source.tolerance / 4.,
            edges: self.edges(curves),
        }
    }
    fn edges(&self, curves: &'a [retained::OwnedCurve; 2]) -> [ConicalStorageQuery<'a>; 2] {
        let fit: &'a cone_mapping::fit::OwnedFit = &self.fit.fit;
        let view = fit.view();
        [0, 1].map(|depth| ConicalStorageQuery {
            tangent: view.tangent,
            numerator: view.slant_numerator,
            weights: view.slant_weights,
            semi_angle: view.semi_angle,
            center: view.chart_center,
            depth: self.prepared.source.depths[depth],
            outward: view.outward,
            stored_poles: &curves[depth].poles,
            stored_weights: &curves[depth].weights,
        })
    }
}
pub(super) fn certify(
    original: &DevelopedRegionRequest,
    prepared: &Prepared,
    piece: &AuthoredPiece,
    curves: &[retained::OwnedCurve; 2],
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<std::sync::Arc<ConicalBoundaryEvidence>> {
    let query = query(original, prepared, piece, curves)?;
    let fit = prepared
        .fits
        .get(&piece.span)
        .ok_or_else(|| missing(original.region, piece.span, "retained boundary receipt"))?;
    if reusable(&fit.boundary, &query, work.requested) {
        return Ok(std::sync::Arc::clone(&fit.boundary));
    }
    let proof = certify_conical_boundary(query, work.remaining(), cancelled).map_err(|cause| {
        failure(
            original.region,
            Some(piece.span),
            ConeCause::Boundary(Box::new(cause)),
        )
    })?;
    work.charge(original.region, Some(piece.span), proof.operations())?;
    Ok(std::sync::Arc::new(proof))
}
fn reusable(
    proof: &ConicalBoundaryEvidence,
    query: &ConicalBoundaryQuery<'_>,
    requested: ConeClearanceOptions,
) -> bool {
    let generated = proof.options();
    proof.matches(query)
        && generated.fractional_bits == requested.fractional_bits
        && generated.max_operations <= requested.max_operations
        && proof.operations() <= generated.max_operations
}
pub(super) fn joins(
    original: &DevelopedRegionRequest,
    prepared: &Prepared,
    spans: &[SpanProof],
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<ConicalJoinsEvidence> {
    let mut queries = Vec::with_capacity(spans.len());
    for (piece, proof) in prepared.source.loops.iter().flatten().zip(spans) {
        queries.push(ConicalJoinSpan {
            query: query(original, prepared, piece, &proof.curves)?,
            boundary: &proof.boundary,
        });
    }
    let lengths: Vec<_> = prepared.source.loops.iter().map(Vec::len).collect();
    let proof = certify_conical_joins(
        ConicalJoinsQuery {
            spans: &queries,
            contour_lengths: &lengths,
            options: work.remaining(),
        },
        cancelled,
    )
    .map_err(|cause| joins_failure(original.region, spans, cause))?;
    work.charge(original.region, None, proof.operations())?;
    Ok(proof)
}
fn joins_failure(
    region: SurfaceRegionId,
    spans: &[SpanProof],
    cause: ConicalJoinsFailure,
) -> ConeFailure {
    let index = match &cause {
        ConicalJoinsFailure::Boundary { index } | ConicalJoinsFailure::Proof { index, .. } => {
            Some(*index)
        }
        _ => None,
    };
    let span = index
        .and_then(|index| spans.get(index))
        .map(|proof| proof.span);
    failure(region, span, ConeCause::Joins(Box::new(cause)))
}

#[cfg(test)]
mod reuse_tests {
    use super::*;
    #[test]
    fn generated_receipt_reuse_requires_full_actual_query_and_original_policy() {
        let request = super::super::super::tests::request();
        let inputs = input::capture(&request, &mut || false).unwrap();
        let mut work = Work::new(request.proof.arithmetic);
        let selection =
            prepare::select(&request, &inputs.originals, &mut work, &mut || false).unwrap();
        let (original, prepared) = (&inputs.originals[0], &selection.prepared[0]);
        let piece = &prepared.source.loops[0][0];
        let fit = &prepared.fits[&piece.span];
        let curves = actual_curves(fit);
        let query = query(original, prepared, piece, &curves).unwrap();
        check_mutations(&fit.boundary, query, work.requested);
        let consumed = work.consumed;
        let reused = certify(original, prepared, piece, &curves, &mut work, &mut || false).unwrap();
        assert!(std::sync::Arc::ptr_eq(&reused, &fit.boundary));
        assert_eq!(work.consumed, consumed);
        check_reproof(original, prepared, piece, fit, curves, &mut work);
    }
    fn actual_curves(fit: &SpanFit) -> [retained::OwnedCurve; 2] {
        fit.poles.each_ref().map(|poles| retained::OwnedCurve {
            poles: poles.points.clone(),
            weights: poles.weights.clone(),
        })
    }
    fn check_mutations(
        proof: &ConicalBoundaryEvidence,
        query: ConicalBoundaryQuery<'_>,
        policy: ConeClearanceOptions,
    ) {
        assert!(reusable(proof, &query, policy));
        assert!(!reusable(
            proof,
            &query,
            ConeClearanceOptions {
                fractional_bits: policy.fractional_bits + 1,
                ..policy
            }
        ));
        assert!(!reusable(
            proof,
            &query,
            ConeClearanceOptions {
                max_operations: proof.options().max_operations - 1,
                ..policy
            }
        ));
        check_query_mutations(proof, query, policy);
    }
    fn check_query_mutations(
        proof: &ConicalBoundaryEvidence,
        query: ConicalBoundaryQuery<'_>,
        policy: ConeClearanceOptions,
    ) {
        let mut changed = query;
        changed.spatial_limit *= 2.;
        assert!(!reusable(proof, &changed, policy));
        changed = query;
        changed.depths[1] = changed.depths[1].next_up();
        assert!(!reusable(proof, &changed, policy));
        changed = query;
        let mut original = changed.original.unwrap();
        original.index += 1;
        changed.original = Some(original);
        assert!(!reusable(proof, &changed, policy));
    }
    fn check_reproof(
        original: &DevelopedRegionRequest,
        prepared: &Prepared,
        piece: &AuthoredPiece,
        fit: &SpanFit,
        mut curves: [retained::OwnedCurve; 2],
        work: &mut Work,
    ) {
        let consumed = work.consumed;
        curves[0].poles[0][0] = curves[0].poles[0][0].next_up();
        let repeated = certify(original, prepared, piece, &curves, work, &mut || false).unwrap();
        assert!(!std::sync::Arc::ptr_eq(&repeated, &fit.boundary));
        assert!(work.consumed > consumed);
    }
}
