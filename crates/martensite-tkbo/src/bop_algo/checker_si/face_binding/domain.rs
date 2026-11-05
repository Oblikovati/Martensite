// SPDX-License-Identifier: GPL-2.0-only
//! Exact rectangle incidence is separate from support injectivity and metric conformance.
use super::{capture_curves::unsupported, exact, types::*};
use martensite_tkg3d::top_abs::Orientation;

pub(super) fn check(
    packet: &OwnedFacePacket,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<[BoundaryRole; 4], BindingFailure> {
    exact::poll(cancelled)?;
    support_domain(packet)?;
    count("face wires", packet.wire_count, 1)?;
    count("edge occurrences", packet.edges.len(), 4)?;
    let roles = boundary_cycle(packet, cancelled)?;
    for expected in [
        BoundaryRole::UFirst,
        BoundaryRole::ULast,
        BoundaryRole::VFirst,
        BoundaryRole::VLast,
    ] {
        count(
            "unique rectangle side",
            roles.iter().filter(|r| **r == expected).count(),
            1,
        )?;
    }
    seam(packet, &roles)?;
    Ok(roles)
}
fn boundary_cycle(
    packet: &OwnedFacePacket,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<[BoundaryRole; 4], BindingFailure> {
    let mut roles = [BoundaryRole::UFirst; 4];
    let mut incoming = [0usize; 4];
    let mut outgoing = [0usize; 4];
    for (index, edge) in packet.edges.iter().enumerate() {
        let [start, end] = directed_corners(edge, packet.uv_bounds, cancelled)?;
        roles[index] = role(start, end)?;
        outgoing[start] += 1;
        incoming[end] += 1;
        exact::require_invertible(&edge.placement)?;
    }
    if incoming != [1; 4] || outgoing != [1; 4] {
        return Err(unsupported(
            "oriented boundary cycle",
            format!("incoming={incoming:?}, outgoing={outgoing:?}; expected one each"),
        ));
    }
    Ok(roles)
}

fn support_domain(packet: &OwnedFacePacket) -> Result<(), BindingFailure> {
    let b = &packet.basis;
    let [u0, u1, v0, v1] = packet.uv_bounds;
    if ![u0, u1, v0, v1].iter().all(|v| v.is_finite())
        || u0 >= u1
        || v0 >= v1
        || [u0, u1] != b.domain
    {
        return Err(BindingFailure::Domain {
            role: "full support strip",
            actual: packet.uv_bounds.to_vec(),
        });
    }
    super::layout::check(b)?;
    exact::require_invertible(&packet.support_placement)?;
    if packet.direction.iter().any(|v| !v.is_finite()) || packet.direction == [0.; 3] {
        return Err(unsupported(
            "extrusion direction",
            format!("{:?}; expected finite nonzero", packet.direction),
        ));
    }
    orientation(packet.face_orientation)?;
    Ok(())
}
fn directed_corners(
    edge: &OwnedEdgeUse,
    bounds: [f64; 4],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<[usize; 2], BindingFailure> {
    if edge.range.iter().any(|v| !v.is_finite())
        || edge.range[0] >= edge.range[1]
        || edge.range != edge.curve_range
    {
        return Err(BindingFailure::Domain {
            role: "same-parameter finite edge range",
            actual: edge.range.into_iter().chain(edge.curve_range).collect(),
        });
    }
    let [u0, u1, v0, v1] = bounds;
    let corners = [[u0, v0], [u1, v0], [u1, v1], [u0, v1]];
    let mut endpoints = [
        endpoint_corner(edge, edge.range[0], &corners, cancelled)?,
        endpoint_corner(edge, edge.range[1], &corners, cancelled)?,
    ];
    if orientation(edge.canonical_orientation)? {
        endpoints.swap(0, 1);
    }
    Ok(endpoints)
}
fn endpoint_corner(
    edge: &OwnedEdgeUse,
    parameter: f64,
    corners: &[[f64; 2]; 4],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<usize, BindingFailure> {
    let mut matched = Vec::new();
    for (index, corner) in corners.iter().enumerate() {
        if exact::endpoint_matches(&edge.pcurve, parameter, *corner, cancelled)? {
            matched.push(index);
        }
    }
    count("exact trim endpoint corner", matched.len(), 1)?;
    Ok(matched[0])
}

fn role(a: usize, b: usize) -> Result<BoundaryRole, BindingFailure> {
    match (a.min(b), a.max(b)) {
        (0, 3) => Ok(BoundaryRole::UFirst),
        (1, 2) => Ok(BoundaryRole::ULast),
        (0, 1) => Ok(BoundaryRole::VFirst),
        (2, 3) => Ok(BoundaryRole::VLast),
        _ => Err(unsupported(
            "rectangle side",
            format!("corners=[{a},{b}]; expected distinct adjacent corners"),
        )),
    }
}
fn seam(packet: &OwnedFacePacket, roles: &[BoundaryRole; 4]) -> Result<(), BindingFailure> {
    let first = roles
        .iter()
        .position(|r| *r == BoundaryRole::UFirst)
        .ok_or_else(|| unsupported("u first side", "absent"))?;
    let last = roles
        .iter()
        .position(|r| *r == BoundaryRole::ULast)
        .ok_or_else(|| unsupported("u last side", "absent"))?;
    relations(packet, first, last)?;
    let (a, b) = (&packet.edges[first], &packet.edges[last]);
    if !a.geometric_seam
        || !b.geometric_seam
        || a.canonical_orientation == b.canonical_orientation
        || !matches!((a.stored_branch, b.stored_branch), (1, 2) | (2, 1))
    {
        return Err(unsupported(
            "true stored seam",
            format!(
                "branches=[{},{}], orientations=[{:?},{:?}]; expected opposite uses of both stored branches",
                a.stored_branch, b.stored_branch, a.canonical_orientation, b.canonical_orientation
            ),
        ));
    }
    Ok(())
}
fn relations(packet: &OwnedFacePacket, first: usize, last: usize) -> Result<(), BindingFailure> {
    count("pairwise occurrence relations", packet.relations.len(), 6)?;
    let mut visited = Vec::new();
    let mut same = 0;
    for relation in &packet.relations {
        let pair = (relation.left, relation.right);
        require_pair(pair, &visited)?;
        visited.push(pair);
        if !relation.same {
            continue;
        }
        same += 1;
        if pair != (first.min(last), first.max(last)) || !relation.location_equal {
            return Err(unsupported(
                "seam identity",
                format!(
                    "pair={pair:?}, same={}, location_equal={}; expected exact seam pair",
                    relation.same, relation.location_equal
                ),
            ));
        }
    }
    count("same-identity seam pairs", same, 1)
}
fn orientation(value: Orientation) -> Result<bool, BindingFailure> {
    match value {
        Orientation::Forward => Ok(false),
        Orientation::Reversed => Ok(true),
        _ => Err(unsupported(
            "boundary orientation",
            format!("{value:?}; expected Forward or Reversed"),
        )),
    }
}
fn count(role: &'static str, actual: usize, expected: usize) -> Result<(), BindingFailure> {
    if actual != expected {
        return Err(BindingFailure::Count {
            role,
            actual,
            expected,
        });
    }
    Ok(())
}

fn require_pair(pair: (usize, usize), visited: &[(usize, usize)]) -> Result<(), BindingFailure> {
    if pair.0 >= pair.1 || pair.1 >= 4 || visited.contains(&pair) {
        return Err(unsupported(
            "occurrence pair",
            format!("{pair:?}; expected unique in-range unordered pair"),
        ));
    }
    Ok(())
}
