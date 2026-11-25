//! Family extrema are issued before any directed native row is constructed.
use super::*;
pub(super) fn certify(
    originals: &[DevelopedRegionRequest],
    members: &OriginalMembers<'_>,
    prepared: &[Prepared],
    inventory: &Inventory,
    refinement: ConicalPrimitiveRefinement,
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<std::sync::Arc<ConicalDirectedGammaEvidence>> {
    let spans = spans(members, prepared, inventory)?;
    let caps = caps(prepared, inventory)?;
    let region = selection::first_region(originals)?;
    let query = ConicalDirectedGammaQuery {
        spans: &spans,
        caps: &caps,
        refinement,
        options: work.remaining_checked(region, None)?,
    };
    let result = certify_conical_directed_gamma(query, cancelled);
    let operations = result.as_ref().map_or_else(
        |cause| cause.operations,
        ConicalDirectedGammaEvidence::operations,
    );
    let charged = work.charge(region, None, operations);
    finish(result, charged, query, inventory)
}
fn spans<'a>(
    originals: &OriginalMembers<'a>,
    prepared: &'a [Prepared],
    inventory: &Inventory,
) -> ConeChecked<Vec<ConicalDirectedGammaSpan<'a>>> {
    let mut result = Vec::new();
    for member in prepared {
        let original = original(originals, member.source.region)?;
        for piece in member.source.loops.iter().flatten() {
            result.push(span(original, member, piece, inventory)?);
        }
    }
    Ok(result)
}
fn span<'a>(
    original: &'a DevelopedRegionRequest,
    member: &'a Prepared,
    piece: &'a AuthoredPiece,
    inventory: &Inventory,
) -> ConeChecked<ConicalDirectedGammaSpan<'a>> {
    let fit = fit(member, piece)?;
    Ok(ConicalDirectedGammaSpan {
        id: inventory.side(original.region, piece.span)?,
        member: ConicalActualMemberId(original.region.0),
        previous: storage::prepared_query(original, member, piece, &fit.poles)?,
        parent: &fit.boundary,
        domain: chart(member, piece.span, fit)?,
        caps: inventory.cap_pair(original.region)?,
    })
}
fn caps(prepared: &[Prepared], inventory: &Inventory) -> ConeChecked<Vec<ConicalActualCapCarrier>> {
    let mut result = Vec::new();
    for member in prepared {
        for (row, id) in inventory
            .cap_pair(member.source.region)?
            .into_iter()
            .enumerate()
        {
            result.push(cap(member, row, id)?);
        }
    }
    Ok(result)
}
pub(super) fn finish(
    result: Result<ConicalDirectedGammaEvidence, ConicalActualPatchError>,
    charged: ConeChecked<()>,
    query: ConicalDirectedGammaQuery<'_>,
    inventory: &Inventory,
) -> ConeChecked<std::sync::Arc<ConicalDirectedGammaEvidence>> {
    match (result, charged) {
        (Err(cause), _) => Err(actual_failure(cause, inventory)),
        (Ok(_), Err(cause)) => Err(cause),
        (Ok(proof), Ok(())) if proof.matches(&query) => Ok(std::sync::Arc::new(proof)),
        (Ok(_), Ok(())) => Err(failure(
            None,
            None,
            ConeCause::Binding {
                field: "issued directed gamma source",
                actual: vec![],
                expected: vec![1],
            },
        )),
    }
}

fn cap(
    member: &Prepared,
    row: usize,
    id: ConicalActualCapId,
) -> ConeChecked<ConicalActualCapCarrier> {
    let carrier = native::cap_carrier(member, member.semi_angle, row).map_err(|cause| {
        failure(
            Some(member.source.region),
            None,
            ConeCause::Native(Box::new(cause)),
        )
    })?;
    let datum = carrier.datum;
    Ok(ConicalActualCapCarrier {
        id,
        member: ConicalActualMemberId(member.source.region.0),
        frame: carrier.frame(),
        radius: datum.radius,
        axial_origin: datum.axial_origin,
        semi_angle: datum.semi_angle,
    })
}
#[cfg(test)]
mod tests;
