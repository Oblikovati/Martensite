//! Source-keyed exact UV loops reuse the owned planar regularity, separation and nesting engine.
use super::*;
type SpanIndex<'a> = BTreeMap<CurveSpan, &'a storage::SpanProof>;
pub(super) fn request(
    prepared: &Prepared,
    spans: &[storage::SpanProof],
) -> ConeChecked<DevelopedRegionRequest> {
    let index = index(prepared.source.region, spans)?;
    let ids = curve_ids(prepared, spans.len())?;
    let vertices = vertices(&prepared.source);
    let loops = prepared
        .source
        .loops
        .iter()
        .map(|contour| contour_request(prepared.source.region, contour, &index, &ids, &vertices))
        .collect::<ConeChecked<_>>()?;
    Ok(DevelopedRegionRequest {
        loops,
        ..prepared.source.clone()
    })
}
fn curve_ids(
    prepared: &Prepared,
    count: usize,
) -> ConeChecked<BTreeMap<CurveSpan, SurfaceCurveId>> {
    let ids: BTreeMap<_, _> = prepared
        .source
        .loops
        .iter()
        .flatten()
        .map(|piece| (piece.span, ()))
        .collect();
    if ids.len() != count {
        return Err(mismatch(
            prepared.source.region,
            None,
            "UV source span count",
        ));
    }
    Ok(ids
        .keys()
        .enumerate()
        .map(|(i, span)| (*span, SurfaceCurveId(i as u64 + 1)))
        .collect())
}

fn index(region: SurfaceRegionId, spans: &[storage::SpanProof]) -> ConeChecked<SpanIndex<'_>> {
    let mut index = BTreeMap::new();
    for span in spans {
        if index.insert(span.span, span).is_some() {
            return Err(mismatch(
                region,
                Some(span.span),
                "unique source-keyed UV span",
            ));
        }
    }
    Ok(index)
}
fn vertices(source: &DevelopedRegionRequest) -> BTreeMap<CornerSource, SurfaceVertexId> {
    let corners: BTreeMap<_, _> = source
        .loops
        .iter()
        .flatten()
        .flat_map(|piece| [piece.start, piece.end])
        .map(|corner| (corner, ()))
        .collect();
    corners
        .keys()
        .enumerate()
        .map(|(i, corner)| (*corner, SurfaceVertexId(i as u64 + 1)))
        .collect()
}
fn contour_request(
    region: SurfaceRegionId,
    contour: &[AuthoredPiece],
    index: &SpanIndex<'_>,
    ids: &BTreeMap<CurveSpan, SurfaceCurveId>,
    vertices: &BTreeMap<CornerSource, SurfaceVertexId>,
) -> ConeChecked<Vec<AuthoredPiece>> {
    let mut pieces = Vec::new();
    for (position, piece) in contour.iter().enumerate() {
        let proof = index
            .get(&piece.span)
            .ok_or_else(|| mismatch(region, Some(piece.span), "actual UV source owner"))?;
        let next = &contour[(position + 1) % contour.len()];
        let next_proof = index
            .get(&next.span)
            .ok_or_else(|| mismatch(region, Some(next.span), "actual UV successor owner"))?;
        admit(region, piece, next, proof, next_proof)?;
        pieces.push(transformed(piece, proof, ids, vertices));
    }
    Ok(pieces)
}
fn admit(
    region: SurfaceRegionId,
    piece: &AuthoredPiece,
    next: &AuthoredPiece,
    proof: &storage::SpanProof,
    next_proof: &storage::SpanProof,
) -> ConeChecked<()> {
    validate_uv(
        region,
        piece.span,
        &proof.pcurves,
        &next_proof.pcurves[0],
        piece.end == next.start,
    )
}
pub(super) fn validate_uv(
    region: SurfaceRegionId,
    span: CurveSpan,
    pcurves: &[Vec<[f64; 2]>; 2],
    next: &[[f64; 2]],
    keyed_join: bool,
) -> ConeChecked<()> {
    let first = &pcurves[0];
    let second = &pcurves[1];
    if !(2..=4).contains(&first.len()) || !first.iter().eq(second) {
        return Err(mismatch(
            region,
            Some(span),
            "both actual caps share one UV polynomial",
        ));
    }
    if next.is_empty() || first[first.len() - 1] != next[0] || !keyed_join {
        return Err(mismatch(
            region,
            Some(span),
            "exact source-keyed actual UV closure",
        ));
    }
    Ok(())
}

fn transformed(
    piece: &AuthoredPiece,
    proof: &storage::SpanProof,
    ids: &BTreeMap<CurveSpan, SurfaceCurveId>,
    vertices: &BTreeMap<CornerSource, SurfaceVertexId>,
) -> AuthoredPiece {
    let mut transformed = piece.clone();
    transformed.span = CurveSpan {
        curve: ids[&piece.span],
        level: 0,
        index: 0,
    };
    transformed.start = CornerSource::Authored(vertices[&piece.start]);
    transformed.end = CornerSource::Authored(vertices[&piece.end]);
    // Development has negative UV Jacobian. Swapping coordinates preserves authored winding.
    transformed.poles = proof.pcurves[0]
        .iter()
        .map(|point| [point[1], point[0]])
        .collect();
    transformed
}
