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
    let original = original_interval(original, prepared, piece.span)?;
    Ok(BoundaryInput {
        prepared,
        piece,
        fit,
        original,
    }
    .query(curves))
}
pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn prepared_query<'a>(
    original: &'a DevelopedRegionRequest,
    prepared: &'a Prepared,
    piece: &'a AuthoredPiece,
    poles: &'a [angular::RationalPoles; 2],
) -> ConeChecked<ConicalBoundaryQuery<'a>> {
    let fit = prepared
        .fits
        .get(&piece.span)
        .ok_or_else(|| missing(original.region, piece.span, "prepared boundary membership"))?;
    let input = BoundaryInput {
        prepared,
        piece,
        fit,
        original: original_interval(original, prepared, piece.span)?,
    };
    let view = fit.fit.view();
    let edges =
        [0, 1].map(|depth| input.edge(&view, depth, &poles[depth].points, &poles[depth].weights));
    Ok(input.with_edges(edges))
}
fn original_interval<'a>(
    original: &'a DevelopedRegionRequest,
    prepared: &Prepared,
    span: CurveSpan,
) -> ConeChecked<ConicalOriginalSpan<'a>> {
    let authored = prepared
        .authored_piece(original, span.curve)
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
        self.with_edges(self.edges(curves))
    }
    fn with_edges(&self, edges: [ConicalStorageQuery<'a>; 2]) -> ConicalBoundaryQuery<'a> {
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
            edges,
        }
    }
    fn edges(&self, curves: &'a [retained::OwnedCurve; 2]) -> [ConicalStorageQuery<'a>; 2] {
        let fit: &'a cone_mapping::fit::OwnedFit = &self.fit.fit;
        let view = fit.view();
        [0, 1].map(|depth| self.edge(&view, depth, &curves[depth].poles, &curves[depth].weights))
    }
    fn edge(
        &self,
        view: &cone_mapping::ConeFit<'a>,
        depth: usize,
        poles: &'a [[f64; 3]],
        weights: &'a [f64],
    ) -> ConicalStorageQuery<'a> {
        ConicalStorageQuery {
            tangent: view.tangent,
            numerator: view.slant_numerator,
            weights: view.slant_weights,
            semi_angle: view.semi_angle,
            center: view.chart_center,
            depth: self.prepared.source.depths[depth],
            outward: view.outward,
            stored_poles: poles,
            stored_weights: weights,
        }
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
    if domain::directed(prepared) {
        return directed_reuse(prepared, piece, fit, &query, work.requested);
    }
    if reusable(prepared, &fit.boundary, &query, work.requested) {
        return Ok(std::sync::Arc::clone(&fit.boundary));
    }
    issue_boundary(prepared, piece.span, query, work, cancelled)
}
fn issue_boundary(
    prepared: &Prepared,
    span: CurveSpan,
    query: ConicalBoundaryQuery<'_>,
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<std::sync::Arc<ConicalBoundaryEvidence>> {
    let region = prepared.source.region;
    let proof = domain::certify_boundary(prepared, query, work.remaining(), cancelled)
        .map_err(|cause| failure(region, Some(span), ConeCause::Boundary(Box::new(cause))))?;
    work.charge(region, Some(span), proof.operations())?;
    Ok(std::sync::Arc::new(proof))
}
fn directed_reuse(
    prepared: &Prepared,
    piece: &AuthoredPiece,
    fit: &SpanFit,
    query: &ConicalBoundaryQuery<'_>,
    requested: ConeClearanceOptions,
) -> ConeChecked<std::sync::Arc<ConicalBoundaryEvidence>> {
    let region = prepared.source.region;
    let proof = fit
        .transition
        .as_ref()
        .ok_or_else(|| missing(region, piece.span, "issued directed transition"))?;
    let matches = matches_directed(prepared, proof, query, requested);
    if !matches {
        return Err(mismatch(
            region,
            Some(piece.span),
            "complete decoded directed transition binding",
            vec![u64::from(matches)],
            vec![1],
        ));
    }
    Ok(proof.boundary_shared())
}

fn matches_directed(
    prepared: &Prepared,
    proof: &ConicalBoundaryZRowEvidence,
    query: &ConicalBoundaryQuery<'_>,
    requested: ConeClearanceOptions,
) -> bool {
    prepared.chart_domain.as_deref().is_some_and(|domain| {
        proof.matches_current(
            query,
            domain,
            ConicalPrimitiveRefinement::HalfSplitV1,
            requested.fractional_bits,
        ) && reusable(prepared, proof.boundary(), query, requested)
    })
}
fn reusable(
    prepared: &Prepared,
    proof: &ConicalBoundaryEvidence,
    query: &ConicalBoundaryQuery<'_>,
    requested: ConeClearanceOptions,
) -> bool {
    let generated = proof.options();
    domain::matches_boundary(prepared, proof, query)
        && generated.fractional_bits == requested.fractional_bits
        && generated.max_operations <= requested.max_operations
        && proof.operations() <= generated.max_operations
}
mod joins;
pub(super) use joins::certify as joins;

#[cfg(test)]
mod reuse_tests {
    use super::*;
    #[test]
    fn generated_receipt_reuse_requires_full_actual_query_and_original_policy() {
        let request = super::super::super::tests::request();
        let inputs = input::capture(&request, &mut || false).unwrap();
        let mut work = Work::new(request.proof.arithmetic);
        let selection = prepare::select(
            &ConeRun::from(&request),
            &inputs.originals,
            &mut work,
            &mut || false,
        )
        .unwrap();
        let (original, prepared) = (&inputs.originals[0], &selection.prepared[0]);
        let piece = &prepared.source.loops[0][0];
        let fit = &prepared.fits[&piece.span];
        let curves = actual_curves(fit);
        let query = query(original, prepared, piece, &curves).unwrap();
        check_mutations(prepared, &fit.boundary, query, work.requested);
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
        prepared: &Prepared,
        proof: &ConicalBoundaryEvidence,
        query: ConicalBoundaryQuery<'_>,
        policy: ConeClearanceOptions,
    ) {
        assert!(reusable(prepared, proof, &query, policy));
        for changed in changed_policies(proof, policy) {
            assert!(!reusable(prepared, proof, &query, changed));
        }
        check_query_mutations(prepared, proof, query, policy);
    }
    fn changed_policies(
        proof: &ConicalBoundaryEvidence,
        policy: ConeClearanceOptions,
    ) -> [ConeClearanceOptions; 2] {
        [
            ConeClearanceOptions {
                fractional_bits: policy.fractional_bits + 1,
                ..policy
            },
            ConeClearanceOptions {
                max_operations: proof.options().max_operations - 1,
                ..policy
            },
        ]
    }
    fn check_query_mutations(
        prepared: &Prepared,
        proof: &ConicalBoundaryEvidence,
        query: ConicalBoundaryQuery<'_>,
        policy: ConeClearanceOptions,
    ) {
        let mut changed = query;
        changed.spatial_limit *= 2.;
        assert!(!reusable(prepared, proof, &changed, policy));
        changed = query;
        changed.depths[1] = changed.depths[1].next_up();
        assert!(!reusable(prepared, proof, &changed, policy));
        changed = query;
        let mut original = changed.original.unwrap();
        original.index += 1;
        changed.original = Some(original);
        assert!(!reusable(prepared, proof, &changed, policy));
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
