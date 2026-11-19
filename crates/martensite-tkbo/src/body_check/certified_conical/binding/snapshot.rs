// SPDX-License-Identifier: GPL-2.0-only
//! Complete owned numeric and incidence snapshots, never hashes or native aliases.
use super::*;
pub(super) fn member(member: &MemberGeometry) -> Vec<u64> {
    let mut bits = vec![
        member.id.0,
        member.sides.len() as u64,
        member.caps.len() as u64,
    ];
    for side in &member.sides {
        side_bits(side, &mut bits);
    }
    for cap in &member.caps {
        cap_bits(cap, &mut bits);
    }
    for (id, source) in &member.materials {
        bits.extend([*id, source.len() as u64]);
        bits.extend(source);
    }
    incidences(&member.sides, &mut bits);
    bits
}
fn side_bits(side: &native::Side, bits: &mut Vec<u64>) {
    bits.extend([side.id, side.orientation as u64, side.weights.len() as u64]);
    for row in &side.rows {
        bits.extend(row.iter().flatten().map(|x| x.to_bits()));
    }
    bits.extend(side.weights.iter().map(|x| x.to_bits()));
    side.source.append(bits);
}
fn cap_bits(cap: &native::Cap, bits: &mut Vec<u64>) {
    bits.extend([cap.id, cap.orientation as u64, cap.contours.len() as u64]);
    bits.extend(cap.frame.iter().flatten().map(|x| x.to_bits()));
    bits.extend(cap.datums.map(f64::to_bits));
    for contour in &cap.contours {
        bits.extend([
            contour.role as u64,
            contour.orientation as u64,
            contour.edges.len() as u64,
        ]);
        for edge in &contour.edges {
            edge_bits(edge, bits);
        }
    }
}
fn edge_bits(edge: &native::UvEdge, bits: &mut Vec<u64>) {
    bits.extend([
        edge.side,
        edge.row as u64,
        u64::from(edge.reversed),
        edge.poles.len() as u64,
    ]);
    bits.extend(edge.range.map(f64::to_bits));
    bits.extend(edge.poles.iter().flatten().map(|x| x.to_bits()));
    bits.extend(edge.weights.iter().map(|x| x.to_bits()));
}

fn incidences(sides: &[native::Side], bits: &mut Vec<u64>) {
    for (i, first) in sides.iter().enumerate() {
        for second in &sides[i + 1..] {
            shared_edges(first, second, bits);
        }
    }
}

fn shared_edges(first: &native::Side, second: &native::Side, bits: &mut Vec<u64>) {
    for (a, edge) in first.edges.iter().enumerate() {
        shared_edge(first.id, a, edge, second, bits);
    }
}

fn shared_edge(first_id: u64, a: usize, edge: &Shape, second: &native::Side, bits: &mut Vec<u64>) {
    for (b, candidate) in second.edges.iter().enumerate() {
        if edge.is_same(candidate) {
            bits.extend([first_id, a as u64, second.id, b as u64]);
        }
    }
}
