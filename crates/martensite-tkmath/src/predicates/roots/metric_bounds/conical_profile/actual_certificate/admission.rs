// SPDX-License-Identifier: GPL-2.0-only
//! Complete source, physical-role and incidence admission before numerical setup.
use super::*;
type SideIndex<'a, 'b> = BTreeMap<ConicalActualSideId, &'a ConicalActualSideQuery<'b>>;
pub(super) fn admit(
    query: &ConicalActualPatchQuery<'_>,
    position: &mut error::FailurePosition,
) -> Outcome<()> {
    numeric::policy(query.options)?;
    if query.sides.is_empty() {
        return Err(error::shape("actual sides", 0, "nonempty complete family"));
    }
    precision(query)?;
    let sides = complete_sides(query, position)?;
    position.clear();
    caps(query, &sides, position)?;
    position.clear();
    adjacencies(query, &sides, position)?;
    position.clear();
    Ok(())
}
pub(super) fn mode(value: ConicalPrimitiveRefinement) -> u64 {
    match value {
        ConicalPrimitiveRefinement::UnsplitV1 => 1,
        ConicalPrimitiveRefinement::HalfSplitV1 => 2,
    }
}
fn precision(query: &ConicalActualPatchQuery<'_>) -> Outcome<()> {
    if query.gamma.options().fractional_bits != query.options.fractional_bits
        || query.gamma.primitive_refinement() != query.refinement
    {
        let actual = vec![
            u64::from(query.options.fractional_bits),
            mode(query.refinement),
        ];
        let expected = vec![
            u64::from(query.gamma.options().fractional_bits),
            mode(query.gamma.primitive_refinement()),
        ];
        return Err(error::binding(
            "family precision/refinement",
            actual,
            expected,
        ));
    }
    Ok(())
}
fn complete_sides<'a>(
    query: &ConicalActualPatchQuery<'a>,
    position: &mut error::FailurePosition,
) -> Outcome<SideIndex<'a, 'a>> {
    let mut sides = SideIndex::new();
    for side in query.sides {
        position.side(side.id);
        if sides.insert(side.id, side).is_some() {
            return Err(error::shape(
                "duplicate actual side",
                side.id.0 as usize,
                "unique side ID",
            ));
        }
        side_binding(side, query)?;
    }
    side_inventory(query, &sides)?;
    Ok(sides)
}
fn side_binding(
    side: &ConicalActualSideQuery<'_>,
    query: &ConicalActualPatchQuery<'_>,
) -> Outcome<()> {
    numeric::orientation(side.orientation, "side face orientation")?;
    let capture = side
        .transition
        .checked_actual_capture(
            &side.current,
            side.domain,
            query.refinement,
            query.options.fractional_bits,
        )
        .ok_or_else(|| transition_source_error(side))?;
    if capture.span != side.id || !capture.matches_gamma(query.gamma) {
        return Err(error::binding(
            "issued transition span/family",
            vec![side.id.0],
            vec![capture.span.0],
        ));
    }
    let issued = issued_side(query, side.id)?;
    physical_roles(side, issued)
}
fn transition_source_error(side: &ConicalActualSideQuery<'_>) -> ConicalActualPatchFailure {
    error::binding(
        "actual decoded boundary",
        BoundarySnapshot::capture(&side.current).bits,
        side.transition.boundary().input.bits.clone(),
    )
}
fn physical_roles(side: &ConicalActualSideQuery<'_>, issued: &GammaSpanCapture) -> Outcome<()> {
    if issued.member != side.member || issued.cap_ids != side.caps {
        let actual = vec![side.member.0, side.caps[0].0, side.caps[1].0];
        let expected = vec![issued.member.0, issued.cap_ids[0].0, issued.cap_ids[1].0];
        return Err(error::binding("physical cap roles", actual, expected));
    }
    Ok(())
}
fn caps(
    query: &ConicalActualPatchQuery<'_>,
    sides: &SideIndex<'_, '_>,
    position: &mut error::FailurePosition,
) -> Outcome<()> {
    let mut ids = BTreeSet::new();
    let mut uses = BTreeSet::new();
    for cap in query.caps {
        position.cap(cap.carrier.id);
        gamma::admission::carrier(&cap.carrier)?;
        numeric::orientation(cap.face_orientation, "cap face orientation")?;
        if !ids.insert(cap.carrier.id) {
            return Err(error::shape(
                "duplicate cap",
                cap.carrier.id.0 as usize,
                "unique",
            ));
        }
        carrier_source(cap, query)?;
        cap_edges(cap, sides, &mut uses)?;
    }
    complete_caps(query, &ids, &uses)
}
fn carrier_source(
    cap: &ConicalActualCapQuery<'_>,
    query: &ConicalActualPatchQuery<'_>,
) -> Outcome<()> {
    let expected = query
        .gamma
        .carrier(cap.carrier.id)
        .ok_or_else(|| error::shape("unknown cap", cap.carrier.id.0 as usize, "gamma carrier"))?;
    if !gamma::admission::same_carrier(&cap.carrier, expected) {
        return Err(error::binding(
            "actual cap carrier",
            gamma::admission::carrier_bits(&cap.carrier),
            gamma::admission::carrier_bits(expected),
        ));
    }
    Ok(())
}
fn complete_caps(
    query: &ConicalActualPatchQuery<'_>,
    ids: &BTreeSet<ConicalActualCapId>,
    uses: &BTreeSet<(ConicalActualSideId, usize)>,
) -> Outcome<()> {
    let expected = query
        .sides
        .iter()
        .flat_map(|s| s.caps)
        .collect::<BTreeSet<_>>();
    if ids != &expected {
        return Err(error::binding(
            "complete cap inventory",
            ids.iter().map(|v| v.0).collect(),
            expected.iter().map(|v| v.0).collect(),
        ));
    }
    if uses.len() != 2 * query.sides.len() {
        return Err(error::shape(
            "cap edge uses",
            uses.len(),
            "exactly one use of each physical side row",
        ));
    }
    Ok(())
}
fn cap_edges(
    cap: &ConicalActualCapQuery<'_>,
    sides: &SideIndex<'_, '_>,
    uses: &mut BTreeSet<(ConicalActualSideId, usize)>,
) -> Outcome<()> {
    outer_contour(cap)?;
    for contour in cap.contours {
        numeric::orientation(contour.wire_orientation, "intrinsic wire orientation")?;
        if contour.edges.is_empty() {
            return Err(error::shape("cap contour edges", 0, "nonempty"));
        }
        for edge in contour.edges {
            cap_edge(edge, cap, sides, uses)?;
        }
    }
    Ok(())
}
fn cap_edge(
    edge: &ConicalActualUvEdge<'_>,
    cap: &ConicalActualCapQuery<'_>,
    sides: &SideIndex<'_, '_>,
    uses: &mut BTreeSet<(ConicalActualSideId, usize)>,
) -> Outcome<()> {
    edge_range(edge)?;
    let side = sides.get(&edge.side).ok_or_else(|| {
        error::shape(
            "cap incident side",
            edge.side.0 as usize,
            "declared actual side",
        )
    })?;
    if side.member != cap.carrier.member || side.caps[edge.row] != cap.carrier.id {
        let actual = vec![cap.carrier.member.0, cap.carrier.id.0];
        let expected = vec![side.member.0, side.caps[edge.row].0];
        return Err(error::binding("native cap-row ownership", actual, expected));
    }
    if !uses.insert((edge.side, edge.row)) {
        return Err(error::shape(
            "duplicate cap side-row use",
            edge.row,
            "one occurrence per physical row",
        ));
    }
    Ok(())
}
fn edge_range(edge: &ConicalActualUvEdge<'_>) -> Outcome<()> {
    if edge.row > 1 {
        return Err(error::shape("physical cap row", edge.row, "zero or one"));
    }
    if edge.range.map(f64::to_bits) != [0, 1f64.to_bits()] {
        return Err(error::binding(
            "actual cap curve range",
            edge.range.map(f64::to_bits).to_vec(),
            vec![0, 1f64.to_bits()],
        ));
    }
    Ok(())
}
fn adjacencies(
    query: &ConicalActualPatchQuery<'_>,
    sides: &SideIndex<'_, '_>,
    position: &mut error::FailurePosition,
) -> Outcome<()> {
    let mut slots = BTreeSet::new();
    for pair in query.adjacent {
        position.side(pair.sides[0]);
        if pair.sides[0] == pair.sides[1] {
            return Err(error::shape(
                "self adjacency",
                pair.sides[0].0 as usize,
                "distinct actual sides",
            ));
        }
        for (id, slot) in pair.sides.into_iter().zip(pair.u_slots) {
            adjacent_slot(sides, &mut slots, id, slot)?;
        }
    }
    complete_slots(query, slots.len())?;
    Ok(())
}
fn adjacent_slot(
    sides: &SideIndex<'_, '_>,
    slots: &mut BTreeSet<(ConicalActualSideId, u8)>,
    id: ConicalActualSideId,
    slot: u8,
) -> Outcome<()> {
    if slot > 1 || !sides.contains_key(&id) || !slots.insert((id, slot)) {
        return Err(error::binding(
            "actual shared U-slot incidence",
            vec![id.0, u64::from(slot)],
            vec![0, 1],
        ));
    }
    Ok(())
}

fn side_inventory(query: &ConicalActualPatchQuery<'_>, ids: &SideIndex<'_, '_>) -> Outcome<()> {
    let actual = ids.keys().map(|i| i.0).collect::<Vec<_>>();
    let mut expected = query.gamma.span_ids();
    expected.sort();
    if actual != expected {
        return Err(error::binding(
            "complete family sides",
            actual,
            query.gamma.span_ids(),
        ));
    }
    Ok(())
}

fn issued_side<'a>(
    query: &ConicalActualPatchQuery<'a>,
    id: ConicalActualSideId,
) -> Outcome<&'a GammaSpanCapture> {
    query
        .gamma
        .span_capture(id)
        .ok_or_else(|| error::binding("issued span", vec![id.0], query.gamma.span_ids()))
}

fn outer_contour(cap: &ConicalActualCapQuery<'_>) -> Outcome<()> {
    let outer = cap
        .contours
        .iter()
        .filter(|c| c.role == ConicalActualContourRole::Outer)
        .count();
    if outer != 1 {
        return Err(error::shape(
            "selected outer contours",
            outer,
            "one issued bounded outer contour",
        ));
    }
    Ok(())
}

fn complete_slots(query: &ConicalActualPatchQuery<'_>, actual: usize) -> Outcome<()> {
    if actual != 2 * query.sides.len() {
        return Err(error::shape(
            "closed U-slot coverage",
            actual,
            "each side U0/U1 exactly once",
        ));
    }
    Ok(())
}
