//! Budget-driven dyadic fits precede native construction; only spatial refusals subdivide.
use super::*;
#[cfg(test)]
mod tests;
use martensite_cmath::atan;
pub(super) fn plan(
    prepared: &mut Prepared,
    original: &AuthoredPiece,
    endpoints: &mut endpoints::EndpointPool,
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<Vec<AuthoredPiece>> {
    let mut pending = vec![seed_piece(prepared, original)?];
    let mut accepted = Vec::new();
    while let Some(mut piece) = pending.pop() {
        let candidate = candidate(prepared, original, &mut piece, endpoints, work, cancelled);
        match candidate {
            Ok(fit) => {
                prepared.fits.insert(piece.span, fit);
                accepted.push(piece);
            }
            Err(cause) => split_or_refuse(prepared, piece, &mut pending, accepted.len(), cause)?,
        }
    }
    Ok(accepted)
}
fn seed_piece(prepared: &Prepared, original: &AuthoredPiece) -> ConeChecked<AuthoredPiece> {
    let scope = prepared
        .chart_domain
        .as_ref()
        .and_then(|chart| chart.source_restriction());
    let Some((index, level)) = scope else {
        return Ok(original.clone());
    };
    let within = level
        .checked_sub(original.span.level)
        .is_some_and(|extra| index >> extra == u64::from(original.span.index));
    if level > prepared.source.max_fit_level || !within {
        return Err(restriction_failure(prepared, original.span, index, level));
    }
    Ok(subdivision::restrict_piece(original, index, level))
}

fn restriction_failure(
    prepared: &Prepared,
    source: CurveSpan,
    index: u64,
    level: u8,
) -> ConeFailure {
    failure(
        Some(prepared.source.region),
        Some(source),
        ConeCause::Binding {
            field: "local chart restriction within authored span and allowed fit level",
            actual: vec![index, u64::from(level)],
            expected: vec![
                u64::from(source.index),
                u64::from(source.level),
                u64::from(prepared.source.max_fit_level),
            ],
        },
    )
}

fn candidate(
    prepared: &mut Prepared,
    original: &AuthoredPiece,
    piece: &mut AuthoredPiece,
    endpoints: &mut endpoints::EndpointPool,
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<SpanFit> {
    let turns = endpoint_turns(piece, prepared.wedge);
    piece.chart_center = prepared.chart_center;
    let fit = candidate_fit(prepared, piece, turns, endpoints)?;
    let poles = candidate_edges(prepared, piece, &fit)?;
    let uv = cap_uv(&fit, prepared.reference_slant);
    let view = fit.view();
    let query = boundary_query(prepared, original, piece, &view, &poles, turns[0]);
    let boundary = certify(prepared, piece.span, query, work, cancelled)?;
    Ok(SpanFit {
        transition: None,
        fit,
        poles,
        uv,
        developed_turn: turns[0],
        boundary,
    })
}

fn candidate_fit(
    prepared: &Prepared,
    piece: &AuthoredPiece,
    turns: [i32; 2],
    endpoints: &mut endpoints::EndpointPool,
) -> ConeChecked<cone_mapping::fit::OwnedFit> {
    let query = cone_mapping::fit::FitInput {
        source: &piece.poles,
        semi_angle: prepared.semi_angle,
        origin: prepared.chart_origin,
        center: piece.chart_center,
        developed_turns: turns,
        physical_turns: [0; 2],
        outward: true,
    };
    let shared = endpoints.pair(prepared.source.region, piece, &query)?;
    let fit = cone_mapping::fit::fit_span(&query, &shared).map_err(|cause| {
        failure(
            Some(prepared.source.region),
            Some(piece.span),
            ConeCause::Candidate(cause),
        )
    })?;
    Ok(fit)
}
fn candidate_edges(
    prepared: &Prepared,
    piece: &AuthoredPiece,
    fit: &cone_mapping::fit::OwnedFit,
) -> ConeChecked<[angular::RationalPoles; 2]> {
    let mut poles =
        cone_mapping::cone_edges(&fit.view(), prepared.source.depths).map_err(|cause| {
            failure(
                Some(prepared.source.region),
                Some(piece.span),
                ConeCause::Coefficients(cause),
            )
        })?;
    if domain::directed(prepared) {
        native_weight_gauge(&mut poles);
    }
    bind_native_endpoints(prepared, piece, &mut poles);
    Ok(poles)
}

fn native_weight_gauge(rows: &mut [angular::RationalPoles; 2]) {
    // #536: the two rows share the same positive weights. Native Bézier storage
    // encodes a constant weight as unit weights, with no change in the curve.
    let weights = &rows[0].weights;
    if weights
        .first()
        .is_some_and(|first| *first != 1. && weights.iter().all(|w| w == first))
    {
        for row in rows {
            row.weights.fill(1.);
        }
    }
}

fn certify(
    prepared: &Prepared,
    span: CurveSpan,
    query: ConicalBoundaryQuery<'_>,
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<std::sync::Arc<ConicalBoundaryEvidence>> {
    let region = prepared.source.region;
    let result = domain::certify_boundary(prepared, query, work.remaining(), cancelled);
    let operations = match &result {
        Ok(proof) => proof.operations(),
        Err(cause) => cause.operations(),
    };
    work.charge(region, Some(span), operations)?;
    let proof = result.map_err(|cause| {
        failure(
            Some(region),
            Some(span),
            ConeCause::Boundary(Box::new(cause)),
        )
    })?;
    admit_anchor(region, span, &proof)?;
    Ok(std::sync::Arc::new(proof))
}
fn bind_native_endpoints(
    prepared: &Prepared,
    piece: &AuthoredPiece,
    poles: &mut [angular::RationalPoles; 2],
) {
    let ends = [piece.poles[0], piece.poles[piece.poles.len() - 1]];
    for (depth, edge) in poles.iter_mut().enumerate() {
        let last = edge.points.len() - 1;
        for (index, point) in [(0, ends[0]), (last, ends[1])] {
            edge.points[index] = native::endpoint(
                point,
                prepared.semi_angle,
                prepared.chart_origin,
                prepared.wedge,
                prepared.source.depths[depth],
            );
        }
    }
}

fn admit_anchor(
    region: SurfaceRegionId,
    span: CurveSpan,
    proof: &ConicalBoundaryEvidence,
) -> ConeChecked<()> {
    if proof.source_anchor_turn_adjustment() != Some(0) {
        return Err(failure(
            Some(region),
            Some(span),
            ConeCause::SourceAnchor {
                adjustment: proof.source_anchor_turn_adjustment(),
            },
        ));
    }
    Ok(())
}

fn boundary_query<'a>(
    prepared: &Prepared,
    original: &'a AuthoredPiece,
    piece: &'a AuthoredPiece,
    fit: &cone_mapping::ConeFit<'a>,
    poles: &'a [angular::RationalPoles; 2],
    developed_turn: i32,
) -> ConicalBoundaryQuery<'a> {
    ConicalBoundaryQuery {
        source: &piece.poles,
        original: original_span(original, piece.span),
        angular_fit: fit.tangent,
        slant_numerator: fit.slant_numerator,
        slant_weights: fit.slant_weights,
        semi_angle: fit.semi_angle,
        chart_origin: prepared.chart_origin,
        fit_center: fit.chart_center,
        developed_turn,
        physical_turn: 0,
        depths: prepared.source.depths,
        outward: true,
        spatial_limit: prepared.source.tolerance / 4.,
        edges: [
            edge_query(fit, prepared.source.depths[0], &poles[0]),
            edge_query(fit, prepared.source.depths[1], &poles[1]),
        ],
    }
}
fn original_span(original: &AuthoredPiece, span: CurveSpan) -> Option<ConicalOriginalSpan<'_>> {
    Some(ConicalOriginalSpan {
        poles: &original.poles,
        index: u64::from(span.index),
        level: span.level,
    })
}

fn edge_query<'a>(
    fit: &cone_mapping::ConeFit<'a>,
    depth: f64,
    poles: &'a angular::RationalPoles,
) -> ConicalStorageQuery<'a> {
    ConicalStorageQuery {
        tangent: fit.tangent,
        numerator: fit.slant_numerator,
        weights: fit.slant_weights,
        semi_angle: fit.semi_angle,
        center: fit.chart_center,
        depth,
        outward: true,
        stored_poles: &poles.points,
        stored_weights: &poles.weights,
    }
}
fn endpoint_turns(piece: &AuthoredPiece, wedge: ConicalSectorWedge) -> [i32; 2] {
    [piece.poles[0], piece.poles[piece.poles.len() - 1]]
        .map(|[_, y]| i32::from(wedge == ConicalSectorWedge::NegativeX && y < 0.))
}
fn cap_uv(owned: &cone_mapping::fit::OwnedFit, reference: f64) -> [[f64; 2]; 4] {
    let fit = owned.view();
    let t = fit.tangent;
    let u0 = fit.chart_center + 2. * atan(t[0]);
    let u1 = fit.chart_center + 2. * atan(t[3]);
    let du0 = 2. * (t[1] - t[0]) / (1. + t[0] * t[0]);
    let du1 = 2. * (t[3] - t[2]) / (1. + t[3] * t[3]);
    let u = [u0, u0 + du0, u1 - du1, u1];
    std::array::from_fn(|i| [u[i], fit.slant_numerator[i] - reference])
}
fn split_or_refuse(
    prepared: &Prepared,
    piece: AuthoredPiece,
    pending: &mut Vec<AuthoredPiece>,
    completed: usize,
    cause: ConeFailure,
) -> ConeChecked<()> {
    let retry = matches!(&cause.cause, ConeCause::Boundary(error) if matches!(error.cause(), ConicalBoundaryFailure::Proof(proof) if proof.kind() == ConicalBoundaryFailureKind::Budget));
    if !retry
        || piece.span.level >= prepared.source.max_fit_level
        || completed + pending.len() + 2
            > crate::surface_sketch::wrapped_profile::storage::MAX_CERTIFIED_SPANS as usize
    {
        return Err(cause);
    }
    let [left, right] = subdivision::split_piece(&piece);
    pending.extend([right, left]);
    Ok(())
}
