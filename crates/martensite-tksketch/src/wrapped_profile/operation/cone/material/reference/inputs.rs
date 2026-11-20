//! Original planar receipts and every fitted interval are rebound before composing reference authority.
use super::*;
use crate::surface_sketch::wrapped_profile::operation::cone::domain::DomainEvidence;
pub(super) fn capture(family: &OwnedFamily<Selection, Geometry>) -> ConeChecked<Inputs<'_>> {
    let mut inputs = Inputs {
        spans: Vec::new(),
        lengths: Vec::new(),
        sector: Vec::new(),
        joins: Vec::new(),
        atlas: Vec::new(),
    };
    for prepared in &family.selection.prepared {
        member(family, prepared, &mut inputs)?;
    }
    Ok(inputs)
}
fn member<'a>(
    family: &'a OwnedFamily<Selection, Geometry>,
    prepared: &'a Prepared,
    inputs: &mut Inputs<'a>,
) -> ConeChecked<()> {
    let region = prepared.source.region;
    let geometry = &family
        .members
        .get(&region)
        .ok_or_else(|| mismatch(region, None, "reference member owner"))?
        .geometry;
    validate_original(family, prepared, geometry)?;
    inputs.spans.push(spans(
        &geometry.material.original,
        prepared,
        &geometry.spans,
    )?);
    let atlas = matches!(&family.selection.domain, DomainEvidence::Atlas(_));
    metadata(prepared, geometry, inputs, atlas)
}
fn validate_original(
    family: &OwnedFamily<Selection, Geometry>,
    prepared: &Prepared,
    geometry: &Geometry,
) -> ConeChecked<()> {
    let region = prepared.source.region;
    let original = &geometry.material.original;
    let planar = family
        .selection
        .planar
        .get(&region)
        .ok_or_else(|| mismatch(region, None, "reference original planar owner"))?;
    if !planar.matches_request(original) {
        return Err(mismatch(
            region,
            None,
            "certified original material for exact q/L reference",
        ));
    }
    binding::validate(prepared, &geometry.spans, &geometry.material)
}
fn metadata<'a>(
    prepared: &'a Prepared,
    geometry: &'a Geometry,
    inputs: &mut Inputs<'a>,
    atlas: bool,
) -> ConeChecked<()> {
    inputs
        .lengths
        .push(prepared.source.loops.iter().map(Vec::len).collect());
    let start = inputs.sector.len();
    inputs.sector.extend(
        geometry
            .material
            .original
            .loops
            .iter()
            .flatten()
            .map(|piece| piece.poles.as_slice()),
    );
    inputs.joins.push(&geometry.joins);
    if atlas {
        let proof = prepared.chart_domain.as_deref().ok_or_else(|| {
            mismatch(prepared.source.region, None, "reference atlas member chart")
        })?;
        inputs.atlas.push((proof, start..inputs.sector.len()));
    }
    Ok(())
}
fn spans<'a>(
    original: &'a DevelopedRegionRequest,
    prepared: &'a Prepared,
    proofs: &'a [storage::SpanProof],
) -> ConeChecked<Vec<ConicalJoinSpan<'a>>> {
    let index = index(original.region, proofs)?;
    prepared
        .source
        .loops
        .iter()
        .flatten()
        .map(|piece| span(original, prepared, piece, &index))
        .collect()
}
fn index(
    region: SurfaceRegionId,
    proofs: &[storage::SpanProof],
) -> ConeChecked<BTreeMap<CurveSpan, &storage::SpanProof>> {
    let index: BTreeMap<_, _> = proofs.iter().map(|proof| (proof.span, proof)).collect();
    if index.len() != proofs.len() {
        return Err(mismatch(
            region,
            None,
            "unique reference span source owners",
        ));
    }
    Ok(index)
}
fn span<'a>(
    original: &'a DevelopedRegionRequest,
    prepared: &'a Prepared,
    piece: &'a AuthoredPiece,
    index: &BTreeMap<CurveSpan, &'a storage::SpanProof>,
) -> ConeChecked<ConicalJoinSpan<'a>> {
    let proof = index.get(&piece.span).ok_or_else(|| {
        mismatch(
            original.region,
            Some(piece.span),
            "reference exact fitted interval owner",
        )
    })?;
    Ok(ConicalJoinSpan {
        query: storage::boundary_query(original, prepared, piece, &proof.curves)?,
        boundary: &proof.boundary,
    })
}
