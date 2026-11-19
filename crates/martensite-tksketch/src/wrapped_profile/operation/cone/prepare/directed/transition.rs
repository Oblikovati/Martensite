//! Final row movements are checked while the nearest parent remains immutable.
use super::*;
pub(super) fn member(
    original: &DevelopedRegionRequest,
    prepared: &mut Prepared,
    plan: &DirectedPlan<'_>,
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<()> {
    for piece in prepared.source.loops.iter().flatten() {
        let (poles, proof) = issue(original, prepared, piece, plan, work, cancelled)?;
        let fit = prepared.fits.get_mut(&piece.span).ok_or_else(|| {
            missing(
                original.region,
                Some(piece.span),
                "directed final fit owner",
            )
        })?;
        fit.poles = poles;
        fit.boundary = proof.boundary_shared();
        fit.transition = Some(proof);
    }
    Ok(())
}
fn issue(
    original: &DevelopedRegionRequest,
    prepared: &Prepared,
    piece: &AuthoredPiece,
    plan: &DirectedPlan<'_>,
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<(
    [angular::RationalPoles; 2],
    std::sync::Arc<ConicalBoundaryZRowEvidence>,
)> {
    let fit = fit(prepared, piece)?;
    let poles = moved(prepared, piece, &fit.poles, plan)?;
    let query = ConicalBoundaryZRowTransition {
        span: plan.inventory.side(original.region, piece.span)?,
        parent: &fit.boundary,
        previous: storage::prepared_query(original, prepared, piece, &fit.poles)?,
        current: storage::prepared_query(original, prepared, piece, &poles)?,
        domain: chart(prepared)?,
        refinement: ConicalPrimitiveRefinement::HalfSplitV1,
        gamma: plan.gamma,
    };
    let options = work.remaining_checked(original.region, Some(piece.span))?;
    let result = certify_conical_boundary_z_rows(query, options, cancelled);
    let operations = result.as_ref().map_or_else(
        |cause| cause.operations,
        ConicalBoundaryZRowEvidence::incremental_operations,
    );
    let charged = work.charge(original.region, Some(piece.span), operations);
    finish(result, charged, original.region, piece.span).map(|proof| (poles, proof))
}
fn moved(
    prepared: &Prepared,
    piece: &AuthoredPiece,
    previous: &[angular::RationalPoles; 2],
    plan: &DirectedPlan<'_>,
) -> ConeChecked<[angular::RationalPoles; 2]> {
    let mut result = previous.each_ref().map(|row| angular::RationalPoles {
        points: row.points.clone(),
        weights: row.weights.clone(),
    });
    let [first, last] = endpoint_pair(prepared, piece, plan)?;
    for (row, poles) in result.iter_mut().enumerate() {
        move_interior(
            prepared.source.region,
            piece.span,
            row,
            &mut poles.points,
            plan.gamma,
        )?;
        let end = poles.points.len() - 1;
        poles.points[0] = first[row];
        poles.points[end] = last[row];
    }
    Ok(result)
}
fn endpoint_pair(
    prepared: &Prepared,
    piece: &AuthoredPiece,
    plan: &DirectedPlan<'_>,
) -> ConeChecked<[[[f64; 3]; 2]; 2]> {
    let ends = [piece.start, piece.end];
    let source = [piece.poles[0], piece.poles[piece.poles.len() - 1]];
    let first = plan
        .endpoints
        .lookup(prepared.source.region, piece.span, ends[0], source[0])?;
    let last = plan
        .endpoints
        .lookup(prepared.source.region, piece.span, ends[1], source[1])?;
    Ok([first, last])
}
fn move_interior(
    region: SurfaceRegionId,
    span: CurveSpan,
    row: usize,
    points: &mut [[f64; 3]],
    gamma: &ConicalDirectedGammaEvidence,
) -> ConeChecked<()> {
    let last = points.len() - 1;
    for point in &mut points[1..last] {
        point[2] = gamma.transform_z(row, point[2]).map_err(|cause| {
            failure(
                Some(region),
                Some(span),
                ConeCause::ActualPatch(Box::new(cause)),
            )
        })?;
    }
    Ok(())
}

fn finish(
    result: Result<ConicalBoundaryZRowEvidence, ConicalBoundaryZRowError>,
    charged: ConeChecked<()>,
    region: SurfaceRegionId,
    span: CurveSpan,
) -> ConeChecked<std::sync::Arc<ConicalBoundaryZRowEvidence>> {
    match (result, charged) {
        (Err(cause), _) => Err(failure(
            Some(region),
            Some(span),
            ConeCause::DirectedRows(Box::new(cause)),
        )),
        (Ok(_), Err(cause)) => Err(cause),
        (Ok(proof), Ok(())) => Ok(std::sync::Arc::new(proof)),
    }
}
