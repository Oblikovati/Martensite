// SPDX-License-Identifier: GPL-2.0-only
//! Every Math source is matched against an independently decoded native entity.
use super::*;
use std::collections::BTreeMap;
pub(super) fn validate(
    actual: &ConicalNativeGeometry,
    query: &ConicalActualPatchQuery<'_>,
) -> Result<(), ConicalBindingError> {
    let sides = actual.members.iter().map(|member| member.sides.len()).sum();
    let caps = actual
        .members
        .iter()
        .map(|member| member.caps.len())
        .sum::<usize>();
    inventory(actual, query, sides, caps)?;
    let expected = unique_by_id(query.sides, |side| side.id.0);
    for member in &actual.members {
        let caps = cap_rows(member);
        for side in &member.sides {
            match_side(member, side, &expected, &caps)?;
        }
    }
    actual.with_cap_queries(|caps, _| match_caps(caps, query.caps))
}
fn match_side(
    member: &MemberGeometry,
    side: &native::Side,
    expected: &BTreeMap<u64, Option<&ConicalActualSideQuery<'_>>>,
    caps: &CapRows,
) -> Result<(), ConicalBindingError> {
    let expected = expected
        .get(&side.id)
        .copied()
        .flatten()
        .ok_or_else(|| side_mismatch(member, side))?;
    if expected.member != member.id || expected.orientation != side.orientation {
        return Err(side_mismatch(member, side));
    }
    match_rows(member, side, expected, caps)?;
    Ok(())
}
fn match_caps(
    actual: &[ConicalActualCapQuery<'_>],
    expected: &[ConicalActualCapQuery<'_>],
) -> Result<(), ConicalBindingError> {
    let expected = unique_by_id(expected, |cap| cap.carrier.id.0);
    for cap in actual {
        let comparison = expected.get(&cap.carrier.id.0).copied().flatten();
        if comparison.is_none_or(|expected| cap_bits(cap) != cap_bits(expected)) {
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
fn unique_by_id<T>(values: &[T], id: impl Fn(&T) -> u64) -> BTreeMap<u64, Option<&T>> {
    let mut result = BTreeMap::new();
    for value in values {
        result
            .entry(id(value))
            .and_modify(|old| *old = None)
            .or_insert(Some(value));
    }
    result
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
    caps: &CapRows,
) -> Result<(), ConicalBindingError> {
    for row in 0..2 {
        match_row_geometry(member, side, expected, row)?;
        if caps.get(&(side.id, row)).copied().flatten() != Some(expected.caps[row]) {
            return Err(side_mismatch(member, side));
        }
    }
    Ok(())
}

type CapRows = BTreeMap<(u64, usize), Option<ConicalActualCapId>>;

fn cap_rows(member: &MemberGeometry) -> CapRows {
    let mut rows = CapRows::new();
    for cap in &member.caps {
        let id = Some(ConicalActualCapId(cap.id));
        for edge in cap.contours.iter().flat_map(|contour| &contour.edges) {
            rows.entry((edge.side, edge.row))
                .and_modify(|owner| {
                    if *owner != id {
                        *owner = None;
                    }
                })
                .or_insert(id);
        }
    }
    rows
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
