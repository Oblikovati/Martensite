//! Actual side queries reuse complete retained authored/chart/transition sources.
use super::*;
pub(super) fn family(prepared: &[Prepared]) -> ConeChecked<&prepare::directed::DirectedFamily> {
    prepared
        .first()
        .and_then(|prepared| prepared.directed_family.as_deref())
        .ok_or_else(|| global_missing("directed actual family"))
}
pub(super) fn sides<'a>(
    originals: &'a [DevelopedRegionRequest],
    prepared: &'a [Prepared],
    members: &'a ActualMembers<'_>,
    actual: &martensite_tkbo::body_check::ConicalNativeGeometry,
) -> ConeChecked<Vec<ConicalActualSideQuery<'a>>> {
    let mut sides = Vec::new();
    let family = family(prepared)?;
    let originals = operation::source::by_region(originals);
    for prepared in prepared {
        let region = prepared.source.region;
        let original = originals
            .get(&region)
            .copied()
            .ok_or_else(|| missing(region, "actual original member"))?;
        let owner = members
            .get(&region)
            .ok_or_else(|| missing(region, "actual retained geometry"))?;
        append_sides(original, prepared, owner.spans, family, actual, &mut sides)?;
    }
    sides.sort_by_key(|side| side.id);
    Ok(sides)
}
fn append_sides<'a>(
    original: &'a DevelopedRegionRequest,
    prepared: &'a Prepared,
    proofs: &'a [storage::SpanProof],
    family: &prepare::directed::DirectedFamily,
    actual: &martensite_tkbo::body_check::ConicalNativeGeometry,
    sides: &mut Vec<ConicalActualSideQuery<'a>>,
) -> ConeChecked<()> {
    let mut by_span = BTreeMap::new();
    for piece in prepared.source.loops.iter().flatten() {
        by_span.entry(piece.span).or_insert(piece);
    }
    for proof in proofs {
        let piece = by_span
            .get(&proof.span)
            .copied()
            .ok_or_else(|| missing(original.region, "actual prepared piece"))?;
        sides.push(side(original, prepared, piece, proof, family, actual)?);
    }
    Ok(())
}
fn side<'a>(
    original: &'a DevelopedRegionRequest,
    prepared: &'a Prepared,
    piece: &'a AuthoredPiece,
    proof: &'a storage::SpanProof,
    family: &prepare::directed::DirectedFamily,
    actual: &martensite_tkbo::body_check::ConicalNativeGeometry,
) -> ConeChecked<ConicalActualSideQuery<'a>> {
    let region = original.region;
    let id = family.inventory.side(region, proof.span)?;
    let fit = prepare::directed::fit(prepared, piece)?;
    Ok(ConicalActualSideQuery {
        id,
        member: ConicalActualMemberId(region.0),
        current: storage::boundary_query(original, prepared, piece, &proof.curves)?,
        transition: transition(region, fit)?,
        domain: prepare::directed::chart(prepared, piece.span, fit)?,
        caps: family.inventory.cap_pair(region)?,
        orientation: actual
            .side_orientation(id)
            .ok_or_else(|| missing(region, "actual native side orientation"))?,
    })
}

fn transition(region: SurfaceRegionId, fit: &SpanFit) -> ConeChecked<&ConicalBoundaryZRowEvidence> {
    fit.transition
        .as_deref()
        .ok_or_else(|| missing(region, "actual checked row transition"))
}
