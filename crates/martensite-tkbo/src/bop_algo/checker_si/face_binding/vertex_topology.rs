// SPDX-License-Identifier: GPL-2.0-only
//! Verify the owned endpoint quotient against observed IsSame, never coordinate proximity.
use super::{capture_curves::unsupported, exact, types::*};
use martensite_tkg3d::top_abs::{self, Orientation};

pub(super) fn check<'a>(
    packet: &'a OwnedFacePacket,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Vec<EndpointUse<'a>>, BindingFailure> {
    let mut endpoints = Vec::new();
    for (edge_address, edge) in packet.edges.iter().enumerate() {
        require_two_ends(edge)?;
        for (vertex_address, vertex) in edge.vertices.iter().enumerate() {
            exact::poll(cancelled)?;
            endpoints.push(endpoint(
                packet,
                edge_address,
                vertex_address,
                vertex,
                cancelled,
            )?);
        }
    }
    require_corner_counts(&endpoints)?;
    graph(packet, &endpoints, cancelled)?;
    Ok(endpoints)
}
fn require_two_ends(edge: &OwnedEdgeUse) -> Result<(), BindingFailure> {
    if edge.vertices.len() != 2 {
        return Err(BindingFailure::Count {
            role: "direct endpoint occurrences",
            actual: edge.vertices.len(),
            expected: 2,
        });
    }
    let forward = edge
        .vertices
        .iter()
        .filter(|v| v.intrinsic.orientation == Orientation::Forward)
        .count();
    let reversed = edge
        .vertices
        .iter()
        .filter(|v| v.intrinsic.orientation == Orientation::Reversed)
        .count();
    if forward != 1 || reversed != 1 {
        return Err(unsupported(
            "endpoint orientations",
            format!("forward={forward}, reversed={reversed}; expected one each"),
        ));
    }
    source_positions(edge)
}
fn source_positions(edge: &OwnedEdgeUse) -> Result<(), BindingFailure> {
    let positions = edge.vertices.iter().map(|v| v.position).collect::<Vec<_>>();
    if positions[0] == positions[1] || positions.iter().any(|p| *p >= 2) {
        return Err(unsupported(
            "endpoint source positions",
            format!("{positions:?}; expected unique local positions"),
        ));
    }
    Ok(())
}

fn endpoint<'a>(
    packet: &OwnedFacePacket,
    edge_address: usize,
    vertex_address: usize,
    vertex: &'a OwnedVertexUse,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<EndpointUse<'a>, BindingFailure> {
    let edge = &packet.edges[edge_address];
    provenance(edge, vertex)?;
    let (parameter, side) = parameter(edge, vertex)?;
    let corner = corner(edge, parameter, packet.uv_bounds, cancelled)?;
    Ok(EndpointUse {
        edge_address,
        vertex_address,
        vertex,
        parameter,
        side,
        corner,
    })
}
fn corner(
    edge: &OwnedEdgeUse,
    parameter: f64,
    bounds: [f64; 4],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<usize, BindingFailure> {
    let [u0, u1, v0, v1] = bounds;
    let corners = [[u0, v0], [u1, v0], [u1, v1], [u0, v1]];
    let mut matched = Vec::new();
    for (index, corner) in corners.iter().enumerate() {
        if exact::endpoint_matches(&edge.pcurve, parameter, *corner, cancelled)? {
            matched.push(index);
        }
    }
    if matched.len() != 1 {
        return Err(BindingFailure::Count {
            role: "endpoint exact strip corner",
            actual: matched.len(),
            expected: 1,
        });
    }
    Ok(matched[0])
}

fn provenance(edge: &OwnedEdgeUse, vertex: &OwnedVertexUse) -> Result<(), BindingFailure> {
    let same_tokens = vertex.raw.tshape_token == vertex.intrinsic.tshape_token
        && vertex.raw.tshape_token == vertex.boundary.tshape_token;
    let boundary = top_abs::compose(edge.canonical_orientation, vertex.intrinsic.orientation);
    if !same_tokens
        || vertex.raw.orientation != vertex.intrinsic.orientation
        || boundary != vertex.boundary.orientation
        || vertex.intrinsic.location_datums != vertex.boundary.location_datums
        || vertex.point_representation_count != 0
    {
        return Err(unsupported(
            "endpoint provenance",
            format!(
                "same_tokens={same_tokens}, raw={:?}, intrinsic={:?}, boundary={:?}, point_representations={}; expected direct endpoint with coherent wrappers and no uninspected point records",
                vertex.raw.orientation,
                vertex.intrinsic.orientation,
                vertex.boundary.orientation,
                vertex.point_representation_count
            ),
        ));
    }
    exact::require_invertible(&vertex.placement)?;
    exact::require_invertible(&vertex.raw_placement)?;
    Ok(())
}
fn parameter(
    edge: &OwnedEdgeUse,
    vertex: &OwnedVertexUse,
) -> Result<(f64, EndpointSide), BindingFailure> {
    let EndpointParameterObservation::TopologicalRangeEnd {
        side,
        recorded_range,
        returned,
        modified_before,
        modified_after,
    } = &vertex.parameter
    else {
        return Err(unsupported(
            "endpoint parameter source",
            format!("{:?}; expected topological range end", vertex.parameter),
        ));
    };
    let expected = parameter_context(
        edge,
        vertex,
        *side,
        *recorded_range,
        *modified_before,
        *modified_after,
    )?;
    match returned {
        Ok(Some(value)) if value.is_finite() && *value == expected => Ok((*value, *side)),
        Err(cause) => Err(BindingFailure::Native(cause.clone())),
        actual => Err(unsupported(
            "endpoint parameter",
            format!("{actual:?}; expected Some({expected})"),
        )),
    }
}
fn parameter_context(
    edge: &OwnedEdgeUse,
    vertex: &OwnedVertexUse,
    side: EndpointSide,
    recorded_range: [f64; 2],
    modified_before: bool,
    modified_after: bool,
) -> Result<f64, BindingFailure> {
    let expected_side = if vertex.intrinsic.orientation == Orientation::Forward {
        EndpointSide::First
    } else {
        EndpointSide::Last
    };
    let expected = recorded_range[if side == EndpointSide::First { 0 } else { 1 }];
    if side != expected_side
        || recorded_range != edge.range
        || recorded_range != edge.curve_range
        || modified_before != modified_after
    {
        return Err(unsupported(
            "endpoint parameter context",
            format!(
                "side={side:?}, range={recorded_range:?}, modified=[{modified_before},{modified_after}]; expected coherent unchanged range context"
            ),
        ));
    }
    Ok(expected)
}

fn require_corner_counts(endpoints: &[EndpointUse<'_>]) -> Result<(), BindingFailure> {
    let mut counts = [0usize; 4];
    for endpoint in endpoints {
        counts[endpoint.corner] += 1;
    }
    if counts != [2; 4] {
        return Err(unsupported(
            "endpoint corner census",
            format!("{counts:?}; expected two direct uses per corner"),
        ));
    }
    Ok(())
}
fn graph(
    packet: &OwnedFacePacket,
    endpoints: &[EndpointUse<'_>],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(), BindingFailure> {
    let expected = endpoints.len() * (endpoints.len() - 1) / 2;
    if packet.vertex_relations.len() != expected {
        return Err(BindingFailure::Count {
            role: "vertex pair census",
            actual: packet.vertex_relations.len(),
            expected,
        });
    }
    let mut seen = Vec::new();
    for relation in &packet.vertex_relations {
        exact::poll(cancelled)?;
        let pair = (relation.left, relation.right);
        if pair.0 >= pair.1 || pair.1 >= endpoints.len() || seen.contains(&pair) {
            return Err(unsupported(
                "vertex pair address",
                format!("{pair:?}; expected unique in-range pair"),
            ));
        }
        seen.push(pair);
        pair_identity(relation, &endpoints[pair.0], &endpoints[pair.1])?;
    }
    Ok(())
}
fn pair_identity(
    relation: &OccurrenceRelation,
    a: &EndpointUse<'_>,
    b: &EndpointUse<'_>,
) -> Result<(), BindingFailure> {
    let same_token = a.vertex.intrinsic.tshape_token == b.vertex.intrinsic.tshape_token;
    let same_location = a.vertex.intrinsic.location_datums == b.vertex.intrinsic.location_datums;
    let quotient_same = (a.corner < 2) == (b.corner < 2);
    if relation.same != (same_token && same_location)
        || relation.location_equal != same_location
        || relation.same != quotient_same
    {
        return Err(unsupported(
            "actual vertex quotient",
            format!(
                "pair=[{},{}], same={}, location_equal={}, expected_quotient_same={quotient_same}; expected observed identity and location to realize the endpoint quotient",
                relation.left, relation.right, relation.same, relation.location_equal
            ),
        ));
    }
    shared_coefficients(relation.same, same_token, a.vertex, b.vertex)
}
fn shared_coefficients(
    same: bool,
    same_token: bool,
    a: &OwnedVertexUse,
    b: &OwnedVertexUse,
) -> Result<(), BindingFailure> {
    if same && !same_placement(&a.placement, &b.placement) {
        return Err(unsupported(
            "shared vertex placement",
            "same located identity contains inconsistent owned affine coefficients",
        ));
    }
    if same_token && (a.local_point != b.local_point || a.raw_tolerance != b.raw_tolerance) {
        return Err(unsupported(
            "shared vertex coefficients",
            "same TShape contains inconsistent owned point/tolerance values",
        ));
    }
    Ok(())
}

fn same_placement(a: &AffinePlacement, b: &AffinePlacement) -> bool {
    a.linear == b.linear && a.origin == b.origin && a.scale == b.scale
}
