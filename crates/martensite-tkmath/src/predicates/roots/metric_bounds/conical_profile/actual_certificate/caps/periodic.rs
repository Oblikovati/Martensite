// SPDX-License-Identifier: GPL-2.0-only
//! Bind the periodic material's exact decoded sources before reusing its separation.
use super::*;

pub(in super::super) fn certify(
    query: &ConicalActualPatchQuery<'_>,
    authority: ConicalActualPeriodicCaps<'_>,
    support: &GammaSpanCapture,
    arithmetic: &mut Arithmetic<'_>,
    position: &mut error::FailurePosition,
) -> Outcome<Vec<CapCapture>> {
    let sides = bind_sides(query, authority, position)?;
    let mut members = BTreeMap::<_, Vec<_>>::new();
    let mut caps = Vec::with_capacity(query.caps.len());
    for cap in query.caps {
        position.clear();
        position.cap(cap.carrier.id);
        bind_cap(cap, &sides)?;
        let captured = capture::cap(
            cap,
            &support.sine,
            &support.cosine,
            None,
            false,
            false,
            arithmetic,
        )?;
        members
            .entry(cap.carrier.member)
            .or_default()
            .push(caps.len());
        caps.push(captured);
    }
    position.clear();
    bind_levels(&caps, members, arithmetic)?;
    Ok(caps)
}

type SideBindings<'a> =
    BTreeMap<ConicalActualSideId, (ConicalActualPeriodicMember<'a>, usize, usize)>;
fn bind_sides<'a>(
    query: &ConicalActualPatchQuery<'_>,
    authority: ConicalActualPeriodicCaps<'a>,
    position: &mut error::FailurePosition,
) -> Outcome<SideBindings<'a>> {
    if authority.members.is_empty()
        || !authority.reference.has_separated_decoded_fill()
        || !authority
            .reference
            .shares_decoded_join_issuance(authority.members.iter().map(|member| member.joins))
        || (authority.members.len() > 1 && authority.reference.decoded_solid_separation().is_none())
    {
        return Err(mismatch("complete periodic cap authority"));
    }
    let actual: BTreeMap<_, _> = query.sides.iter().map(|side| (side.id, side)).collect();
    let mut binding = SideBindings::new();
    let mut ids = BTreeSet::new();
    for &member in authority.members {
        if !ids.insert(member.member) {
            return Err(mismatch("unique periodic member owner"));
        }
        bind_member(member, &actual, &mut binding, position)?;
    }
    if binding.len() != actual.len() {
        return Err(mismatch("complete periodic side inventory"));
    }
    Ok(binding)
}
fn bind_member<'a>(
    member: ConicalActualPeriodicMember<'a>,
    actual: &BTreeMap<ConicalActualSideId, &ConicalActualSideQuery<'_>>,
    binding: &mut SideBindings<'a>,
    position: &mut error::FailurePosition,
) -> Outcome<()> {
    if member.sides.len() != member.joins.input.len() {
        return Err(mismatch("periodic join side count"));
    }
    let mut index = 0;
    for (contour, &len) in member.joins.contour_lengths.iter().enumerate() {
        for _ in 0..len {
            let id = member.sides[index];
            position.side(id);
            let side = actual
                .get(&id)
                .ok_or_else(|| mismatch("periodic join side owner"))?;
            if side.member != member.member
                || !member.joins.input[index].matches(&side.current)
                || binding.insert(id, (member, index, contour)).is_some()
            {
                return Err(mismatch("actual side equals decoded periodic join source"));
            }
            index += 1;
        }
    }
    Ok(())
}
fn bind_cap(cap: &ConicalActualCapQuery<'_>, sides: &SideBindings<'_>) -> Outcome<()> {
    let mut seen = BTreeSet::new();
    let mut contours = BTreeSet::new();
    for wire in cap.contours {
        let contour = bind_wire(cap.carrier, wire, sides, &mut seen)?;
        if !contours.insert(contour) {
            return Err(mismatch("one native wire per periodic contour"));
        }
    }
    let first = seen
        .first()
        .and_then(|side| sides.get(side))
        .ok_or_else(|| mismatch("nonempty periodic cap"))?;
    if seen.len() != first.0.sides.len()
        || cap.contours.len() != first.0.joins.contour_lengths.len()
    {
        return Err(mismatch("complete periodic cap boundary"));
    }
    Ok(())
}
fn bind_wire(
    carrier: ConicalActualCapCarrier,
    wire: &ConicalActualUvContour<'_>,
    sides: &SideBindings<'_>,
    seen: &mut BTreeSet<ConicalActualSideId>,
) -> Outcome<usize> {
    let first = wire
        .edges
        .first()
        .and_then(|edge| sides.get(&edge.side))
        .ok_or_else(|| mismatch("nonempty periodic cap contour"))?;
    let contour = first.2;
    if (wire.role == ConicalActualContourRole::Outer) != (contour == 0) {
        return Err(mismatch("periodic native outer and hole roles"));
    }
    for edge in wire.edges {
        if !seen.insert(edge.side) {
            return Err(mismatch("one periodic cap use per native side"));
        }
        bind_edge(carrier, sides, contour, edge)?;
    }
    Ok(contour)
}
fn bind_edge(
    carrier: ConicalActualCapCarrier,
    sides: &SideBindings<'_>,
    contour: usize,
    edge: &ConicalActualUvEdge<'_>,
) -> Outcome<()> {
    let (member, index, expected_contour) = sides
        .get(&edge.side)
        .ok_or_else(|| mismatch("periodic cap side"))?;
    let joins = member
        .joins
        .cap_charts
        .as_deref()
        .ok_or_else(|| mismatch("decoded cap join issuance"))?;
    if member.member != carrier.member
        || *expected_contour != contour
        || !joins.matches_carrier(
            edge.row,
            carrier.radius,
            carrier.axial_origin,
            carrier.semi_angle,
        )
        || !joins.matches_uv(*index, edge.poles, edge.weights)
    {
        return Err(mismatch(
            "actual cap carrier and polynomial equal periodic source",
        ));
    }
    Ok(())
}
fn bind_levels(
    caps: &[CapCapture],
    members: BTreeMap<ConicalActualMemberId, Vec<usize>>,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<()> {
    for rows in members.values() {
        let [first, last] = rows.as_slice() else {
            return Err(mismatch("two actual cap levels per periodic member"));
        };
        pairs::pair(&caps[*first], &caps[*last], arithmetic).map_err(|cause| {
            ConicalActualPatchFailure::CapPair {
                caps: [
                    caps[*first].source.carrier().id,
                    caps[*last].source.carrier().id,
                ],
                cause: Box::new(cause),
            }
        })?;
    }
    Ok(())
}
fn mismatch(field: &'static str) -> ConicalActualPatchFailure {
    error::binding(field, vec![0], vec![1])
}
