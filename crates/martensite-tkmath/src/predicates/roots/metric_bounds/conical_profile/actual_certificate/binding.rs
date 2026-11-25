// SPDX-License-Identifier: GPL-2.0-only
//! Complete ordered source bits, including selected roles and all producing grants.
use super::*;
pub(super) fn bits(query: &ConicalActualPatchQuery<'_>) -> Vec<u64> {
    let mut out = header(query);
    for side in query.sides {
        side_bits(side, &mut out);
    }
    for cap in query.caps {
        cap_bits(cap, &mut out);
    }
    for adjacent in query.adjacent {
        out.extend([
            adjacent.sides[0].0,
            adjacent.sides[1].0,
            u64::from(adjacent.u_slots[0]),
            u64::from(adjacent.u_slots[1]),
        ]);
    }
    out
}
fn side_bits(side: &ConicalActualSideQuery<'_>, out: &mut Vec<u64>) {
    out.extend([
        side.id.0,
        side.member.0,
        side.caps[0].0,
        side.caps[1].0,
        side.orientation as i64 as u64,
    ]);
    super::super::boundary_snapshot::visit(&side.current, |v| out.push(v));
    let options = side.transition.remaining_options();
    out.extend([
        u64::from(options.fractional_bits),
        u64::from(options.max_operations),
        u64::from(side.transition.incremental_operations()),
        side.transition.cumulative_operations(),
    ]);
}
fn cap_bits(cap: &ConicalActualCapQuery<'_>, out: &mut Vec<u64>) {
    out.extend([
        cap.carrier.id.0,
        cap.carrier.member.0,
        cap.face_orientation as i64 as u64,
    ]);
    out.extend(gamma::admission::carrier_bits(&cap.carrier));
    out.push(cap.contours.len() as u64);
    for wire in cap.contours {
        out.extend([
            match wire.role {
                ConicalActualContourRole::Outer => 1,
                ConicalActualContourRole::Hole => 2,
            },
            wire.wire_orientation as i64 as u64,
            wire.edges.len() as u64,
        ]);
        for edge in wire.edges {
            edge_bits(edge, out);
        }
    }
}
fn edge_bits(edge: &ConicalActualUvEdge<'_>, out: &mut Vec<u64>) {
    out.extend([
        edge.side.0,
        edge.row as u64,
        u64::from(edge.reversed),
        edge.poles.len() as u64,
        edge.weights.len() as u64,
    ]);
    out.extend(edge.range.map(f64::to_bits));
    for p in edge.poles {
        out.extend(p.map(f64::to_bits));
    }
    out.extend(edge.weights.iter().map(|v| v.to_bits()));
}

fn header(query: &ConicalActualPatchQuery<'_>) -> Vec<u64> {
    vec![
        query.sides.len() as u64,
        query.caps.len() as u64,
        query.adjacent.len() as u64,
        admission::mode(query.refinement),
        u64::from(query.options.fractional_bits),
        u64::from(query.options.max_operations),
    ]
}
