// SPDX-License-Identifier: GPL-2.0-only
//! Complete owned numeric and incidence snapshots, never hashes or native aliases.
use super::*;
pub(super) struct MemberSnapshot {
    geometry: Vec<u64>,
    materials: Vec<(u64, material::MaterialSource)>,
}

pub(super) fn member(
    member: &MemberGeometry,
    adjacent: &[ConicalActualAdjacent],
) -> MemberSnapshot {
    let mut geometry = Vec::new();
    geometry_bits(member, adjacent, &mut geometry);
    MemberSnapshot {
        geometry,
        materials: member.materials.clone(),
    }
}

impl MemberSnapshot {
    pub(super) fn matches(
        &self,
        member: &MemberGeometry,
        adjacent: &[ConicalActualAdjacent],
    ) -> bool {
        let mut comparison = BitComparison {
            expected: self.geometry.iter().copied(),
            matches: true,
        };
        geometry_bits(member, adjacent, &mut comparison);
        comparison.matches
            && comparison.expected.next().is_none()
            && self.materials == member.materials
    }
}

struct BitComparison<'a> {
    expected: std::iter::Copied<std::slice::Iter<'a, u64>>,
    matches: bool,
}
impl Extend<u64> for BitComparison<'_> {
    fn extend<T: IntoIterator<Item = u64>>(&mut self, bits: T) {
        for bit in bits {
            self.matches &= self.expected.next() == Some(bit);
        }
    }
}

fn geometry_bits(
    member: &MemberGeometry,
    adjacent: &[ConicalActualAdjacent],
    bits: &mut impl Extend<u64>,
) {
    bits.extend([
        member.id.0,
        member.sides.len() as u64,
        member.caps.len() as u64,
    ]);
    for side in &member.sides {
        side_bits(side, bits);
    }
    for cap in &member.caps {
        cap_bits(cap, bits);
    }
    for incidence in &adjacent[member.adjacency.clone()] {
        let [first, second] = incidence.sides;
        let [first_slot, second_slot] = incidence.u_slots.map(u64::from);
        bits.extend([first.0, first_slot + 2, second.0, second_slot + 2]);
    }
}
fn side_bits(side: &native::Side, bits: &mut impl Extend<u64>) {
    bits.extend([side.id, side.orientation as u64, side.weights.len() as u64]);
    for row in &side.rows {
        bits.extend(row.iter().flatten().map(|x| x.to_bits()));
    }
    bits.extend(side.weights.iter().map(|x| x.to_bits()));
    side.source.append(bits);
}
fn cap_bits(cap: &native::Cap, bits: &mut impl Extend<u64>) {
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
fn edge_bits(edge: &native::UvEdge, bits: &mut impl Extend<u64>) {
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
