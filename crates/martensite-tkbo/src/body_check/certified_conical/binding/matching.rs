// SPDX-License-Identifier: GPL-2.0-only
//! Every Math source is matched against an independently decoded native entity.
use super::*;
pub(super) fn validate(
    actual: &ConicalNativeGeometry,
    query: &ConicalActualPatchQuery<'_>,
) -> Result<(), ConicalBindingError> {
    let sides: Vec<_> = actual
        .members
        .iter()
        .flat_map(|member| member.sides.iter().map(move |side| (member, side)))
        .collect();
    let caps = actual
        .members
        .iter()
        .map(|member| member.caps.len())
        .sum::<usize>();
    inventory(actual, query, sides.len(), caps)?;
    for (member, side) in sides {
        match_side(member, side, query)?;
    }
    actual.with_cap_queries(|caps, _| match_caps(caps, query.caps))
}
fn match_side(
    member: &MemberGeometry,
    side: &native::Side,
    query: &ConicalActualPatchQuery<'_>,
) -> Result<(), ConicalBindingError> {
    let found: Vec<_> = query
        .sides
        .iter()
        .filter(|candidate| candidate.id.0 == side.id)
        .collect();
    if found.len() != 1 {
        return Err(side_mismatch(member, side));
    }
    let expected = found[0];
    if expected.member != member.id || expected.orientation != side.orientation {
        return Err(side_mismatch(member, side));
    }
    match_rows(member, side, expected)?;
    Ok(())
}
fn match_caps(
    actual: &[ConicalActualCapQuery<'_>],
    expected: &[ConicalActualCapQuery<'_>],
) -> Result<(), ConicalBindingError> {
    for cap in actual {
        let found: Vec<_> = expected
            .iter()
            .filter(|candidate| candidate.carrier.id == cap.carrier.id)
            .collect();
        if found.len() != 1 || cap_bits(cap) != cap_bits(found[0]) {
            return Err(error(
                Some(cap.carrier.member),
                None,
                Some(cap.carrier.id),
                native::rejected("complete actual cap snapshot mismatch"),
            ));
        }
    }
    Ok(())
}
fn cap_bits(cap: &ConicalActualCapQuery<'_>) -> Vec<u64> {
    let c = cap.carrier;
    let mut bits = vec![
        c.id.0,
        c.member.0,
        cap.face_orientation as u64,
        cap.contours.len() as u64,
    ];
    bits.extend(c.frame.iter().flatten().map(|x| x.to_bits()));
    bits.extend([c.radius, c.axial_origin, c.semi_angle].map(f64::to_bits));
    for contour in cap.contours {
        bits.extend([
            u64::from(contour.role == ConicalActualContourRole::Hole),
            contour.wire_orientation as u64,
            contour.edges.len() as u64,
        ]);
        for edge in contour.edges {
            edge_bits(edge, &mut bits);
        }
    }
    bits
}
fn edge_bits(edge: &ConicalActualUvEdge<'_>, bits: &mut Vec<u64>) {
    bits.extend([
        edge.side.0,
        edge.row as u64,
        u64::from(edge.reversed),
        edge.poles.len() as u64,
        edge.weights.len() as u64,
    ]);
    bits.extend(edge.range.map(f64::to_bits));
    bits.extend(edge.poles.iter().flatten().map(|x| x.to_bits()));
    bits.extend(edge.weights.iter().map(|x| x.to_bits()));
}

fn inventory(
    actual: &ConicalNativeGeometry,
    query: &ConicalActualPatchQuery<'_>,
    side_count: usize,
    caps: usize,
) -> Result<(), ConicalBindingError> {
    if side_count != query.sides.len()
        || caps != query.caps.len()
        || actual.adjacent != query.adjacent
    {
        return Err(error(
            None,
            None,
            None,
            native::rejected("numeric/native complete inventory differs"),
        ));
    }
    Ok(())
}

fn match_rows(
    member: &MemberGeometry,
    side: &native::Side,
    expected: &ConicalActualSideQuery<'_>,
) -> Result<(), ConicalBindingError> {
    for row in 0..2 {
        match_row_geometry(member, side, expected, row)?;
        let caps: Vec<_> = member
            .caps
            .iter()
            .filter(|cap| {
                cap.contours
                    .iter()
                    .flat_map(|contour| &contour.edges)
                    .any(|edge| edge.side == side.id && edge.row == row)
            })
            .map(|cap| ConicalActualCapId(cap.id))
            .collect();
        if caps != [expected.caps[row]] {
            return Err(side_mismatch(member, side));
        }
    }
    Ok(())
}

fn match_row_geometry(
    member: &MemberGeometry,
    side: &native::Side,
    expected: &ConicalActualSideQuery<'_>,
    row: usize,
) -> Result<(), ConicalBindingError> {
    let edge = expected.current.edges[row];
    let poles: Vec<_> = edge
        .stored_poles
        .iter()
        .map(|p| p.map(f64::to_bits))
        .collect();
    let actual: Vec<_> = side.rows[row].iter().map(|p| p.map(f64::to_bits)).collect();
    if poles != actual
        || edge
            .stored_weights
            .iter()
            .map(|x| x.to_bits())
            .collect::<Vec<_>>()
            != side.weights.iter().map(|x| x.to_bits()).collect::<Vec<_>>()
    {
        return Err(side_mismatch(member, side));
    }
    Ok(())
}

fn side_mismatch(member: &MemberGeometry, side: &native::Side) -> ConicalBindingError {
    error(
        Some(member.id),
        Some(ConicalActualSideId(side.id)),
        None,
        native::rejected("complete actual side snapshot mismatch"),
    )
}
