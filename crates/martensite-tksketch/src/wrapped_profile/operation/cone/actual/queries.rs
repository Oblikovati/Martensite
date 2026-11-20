//! Actual side queries reuse complete retained authored/chart/transition sources.
use super::*;
pub(super) fn family(selection: &Selection) -> ConeChecked<&prepare::directed::DirectedFamily> {
    selection
        .prepared
        .first()
        .and_then(|prepared| prepared.directed_family.as_deref())
        .ok_or_else(|| global_missing("directed actual family"))
}
pub(super) fn sides<'a>(
    originals: &'a [DevelopedRegionRequest],
    selection: &'a Selection,
    members: &'a BTreeMap<SurfaceRegionId, (Geometry, Shape)>,
    actual: &martensite_tkbo::body_check::ConicalNativeGeometry,
) -> ConeChecked<Vec<ConicalActualSideQuery<'a>>> {
    let mut sides = Vec::new();
    let family = family(selection)?;
    let originals = operation::source::by_region(originals);
    for prepared in &selection.prepared {
        let region = prepared.source.region;
        let original = originals
            .get(&region)
            .copied()
            .ok_or_else(|| missing(region, "actual original member"))?;
        let (geometry, _) = members
            .get(&region)
            .ok_or_else(|| missing(region, "actual retained geometry"))?;
        append_sides(original, prepared, geometry, family, actual, &mut sides)?;
    }
    sides.sort_by_key(|side| side.id);
    Ok(sides)
}
fn append_sides<'a>(
    original: &'a DevelopedRegionRequest,
    prepared: &'a Prepared,
    geometry: &'a Geometry,
    family: &prepare::directed::DirectedFamily,
    actual: &martensite_tkbo::body_check::ConicalNativeGeometry,
    sides: &mut Vec<ConicalActualSideQuery<'a>>,
) -> ConeChecked<()> {
    let mut by_span = BTreeMap::new();
    for piece in prepared.source.loops.iter().flatten() {
        by_span.entry(piece.span).or_insert(piece);
    }
    for proof in &geometry.spans {
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
    Ok(ConicalActualSideQuery {
        id,
        member: ConicalActualMemberId(region.0),
        current: storage::boundary_query(original, prepared, piece, &proof.curves)?,
        transition: transition(prepared, proof.span)?,
        domain: domain(prepared)?,
        caps: family.inventory.cap_pair(region)?,
        orientation: actual
            .side_orientation(id)
            .ok_or_else(|| missing(region, "actual native side orientation"))?,
    })
}

fn transition(prepared: &Prepared, span: CurveSpan) -> ConeChecked<&ConicalBoundaryZRowEvidence> {
    prepared
        .fits
        .get(&span)
        .and_then(|fit| fit.transition.as_deref())
        .ok_or_else(|| missing(prepared.source.region, "actual checked row transition"))
}
fn domain(prepared: &Prepared) -> ConeChecked<&ConicalChartDomainEvidence> {
    prepared
        .chart_domain
        .as_deref()
        .ok_or_else(|| missing(prepared.source.region, "actual chart domain"))
}
