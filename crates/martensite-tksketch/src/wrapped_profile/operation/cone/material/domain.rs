//! Source-keyed exact UV loops reuse the owned planar regularity, separation and nesting engine.
use super::*;
type SpanIndex<'a> = BTreeMap<CurveSpan, &'a storage::SpanProof>;
pub(super) fn request(
    prepared: &Prepared,
    spans: &[storage::SpanProof],
) -> ConeChecked<DevelopedRegionRequest> {
    // Swapping coordinates preserves the authored winding under cone development.
    projected_request(prepared, spans, 1, |[u, v]| [v, u])
}

type DecodedMember<'a> = (&'a Prepared, &'a [storage::SpanProof]);

pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn periodic_requests(
    members: &[DecodedMember<'_>],
) -> ConeChecked<Vec<Request>> {
    let bases = periodic_bases(members)?;
    members
        .iter()
        .map(|&(prepared, spans)| {
            // Reflect slant while keeping angle on the periodic axis.
            let source =
                projected_request(prepared, spans, bases[&prepared.source.region], |[u, v]| {
                    [u, -v]
                })?;
            Ok(Request { source, radius: 1. })
        })
        .collect()
}

fn periodic_bases(members: &[DecodedMember<'_>]) -> ConeChecked<BTreeMap<SurfaceRegionId, u64>> {
    let mut first = 1_u64;
    let mut bases = BTreeMap::new();
    for (region, count) in periodic_counts(members)? {
        bases.insert(region, first);
        // A span contributes one curve and at most two distinct corner keys.
        first = first
            .checked_add(count as u64)
            .and_then(|n| n.checked_add(count as u64))
            .ok_or_else(|| mismatch(region, None, "bounded synthetic cap source keys"))?;
    }
    Ok(bases)
}

fn periodic_counts(members: &[DecodedMember<'_>]) -> ConeChecked<BTreeMap<SurfaceRegionId, usize>> {
    let mut counts = BTreeMap::new();
    let datum = members.first().map(|(prepared, _)| cap_datum(prepared));
    for &(prepared, spans) in members {
        let region = prepared.source.region;
        if datum != Some(cap_datum(prepared)) {
            return Err(mismatch(region, None, "shared periodic cap datum"));
        }
        if counts.insert(region, spans.len()).is_some() {
            return Err(mismatch(region, None, "unique periodic cap region"));
        }
    }
    Ok(counts)
}

fn cap_datum(prepared: &Prepared) -> [u64; 5] {
    [
        prepared.reference_slant,
        prepared.chart_origin,
        prepared.semi_angle,
        prepared.source.depths[0],
        prepared.source.depths[1],
    ]
    .map(f64::to_bits)
}

fn projected_request(
    prepared: &Prepared,
    spans: &[storage::SpanProof],
    first: u64,
    project: impl Fn([f64; 2]) -> [f64; 2] + Copy,
) -> ConeChecked<DevelopedRegionRequest> {
    let index = index(prepared.source.region, spans)?;
    let ids = curve_ids_from(prepared, spans.len(), first)?;
    let vertices = vertices(&prepared.source, first);
    let loops = prepared
        .source
        .loops
        .iter()
        .map(|contour| {
            contour_request(
                prepared.source.region,
                contour,
                &index,
                &ids,
                &vertices,
                project,
            )
        })
        .collect::<ConeChecked<_>>()?;
    Ok(DevelopedRegionRequest {
        loops,
        region: prepared.source.region,
        depths: prepared.source.depths,
        tolerance: prepared.source.tolerance,
        max_fit_level: prepared.source.max_fit_level,
    })
}
pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn curve_ids(
    prepared: &Prepared,
    count: usize,
) -> ConeChecked<BTreeMap<CurveSpan, SurfaceCurveId>> {
    curve_ids_from(prepared, count, 1)
}

fn curve_ids_from(
    prepared: &Prepared,
    count: usize,
    first: u64,
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
        .map(|(i, span)| (*span, SurfaceCurveId(first + i as u64)))
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
fn vertices(
    source: &DevelopedRegionRequest,
    first: u64,
) -> BTreeMap<CornerSource, SurfaceVertexId> {
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
        .map(|(i, corner)| (*corner, SurfaceVertexId(first + i as u64)))
        .collect()
}
fn contour_request(
    region: SurfaceRegionId,
    contour: &[AuthoredPiece],
    index: &SpanIndex<'_>,
    ids: &BTreeMap<CurveSpan, SurfaceCurveId>,
    vertices: &BTreeMap<CornerSource, SurfaceVertexId>,
    project: impl Fn([f64; 2]) -> [f64; 2] + Copy,
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
        pieces.push(transformed(piece, proof, ids, vertices, project));
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
    project: impl Fn([f64; 2]) -> [f64; 2],
) -> AuthoredPiece {
    AuthoredPiece {
        span: CurveSpan {
            curve: ids[&piece.span],
            level: 0,
            index: 0,
        },
        start: CornerSource::Authored(vertices[&piece.start]),
        end: CornerSource::Authored(vertices[&piece.end]),
        poles: proof.pcurves[0].iter().copied().map(project).collect(),
        weighted_source: None,
        chart_center: piece.chart_center,
    }
}
