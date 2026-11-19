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
    for prepared in &selection.prepared {
        let region = prepared.source.region;
        let original = originals
            .iter()
            .find(|original| original.region == region)
            .ok_or_else(|| missing(region, "actual original member"))?;
        let (geometry, _) = members
            .get(&region)
            .ok_or_else(|| missing(region, "actual retained geometry"))?;
        for proof in &geometry.spans {
            sides.push(side(original, prepared, proof, family, actual)?);
        }
    }
    sides.sort_by_key(|side| side.id);
    Ok(sides)
}
fn side<'a>(
    original: &'a DevelopedRegionRequest,
    prepared: &'a Prepared,
    proof: &'a storage::SpanProof,
    family: &prepare::directed::DirectedFamily,
    actual: &martensite_tkbo::body_check::ConicalNativeGeometry,
) -> ConeChecked<ConicalActualSideQuery<'a>> {
    let region = original.region;
    let piece = piece(prepared, proof.span)?;
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

fn piece(prepared: &Prepared, span: CurveSpan) -> ConeChecked<&AuthoredPiece> {
    prepared
        .source
        .loops
        .iter()
        .flatten()
        .find(|piece| piece.span == span)
        .ok_or_else(|| missing(prepared.source.region, "actual prepared piece"))
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
