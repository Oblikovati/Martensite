// SPDX-License-Identifier: GPL-2.0-only
//! Bind exact sources, selected roles and work to the issued cap authority.
use super::*;

pub(super) fn source_bits(profile: &SketchProfile) -> Vec<u64> {
    let mut bits = Vec::new();
    bits.extend(
        profile
            .plane
            .origin
            .into_iter()
            .chain(profile.plane.x_direction)
            .chain(profile.plane.normal)
            .map(f64::to_bits),
    );
    bits.push(profile.vertices.len() as u64);
    for vertex in &profile.vertices {
        bits.extend([
            vertex.id.0,
            vertex.point[0].to_bits(),
            vertex.point[1].to_bits(),
        ]);
    }
    bits.push(profile.curves.len() as u64);
    for curve in &profile.curves {
        if let ProfileCurve::Spline {
            id,
            start,
            end,
            degree,
            poles,
            knots,
            ..
        } = curve
        {
            bits.extend([
                id.0,
                start.0,
                end.0,
                u64::from(*degree),
                poles.len() as u64,
                knots.len() as u64,
            ]);
            bits.extend(poles.iter().flatten().chain(knots).map(|p| p.to_bits()));
        }
    }
    bits
}
pub(super) fn snapshot(
    profile: &SketchProfile,
    outer: &ProfileLoop,
    holes: &[ProfileLoop],
    resolution: Resolution,
    work: DirectedCapWork,
) -> Vec<u64> {
    let mut bits = source_bits(profile);
    bits.extend([
        1,
        resolution.weld.to_bits(),
        u64::from(work.requested.pair_bisections),
        u64::from(work.requested.max_components),
        work.requested.requested_integration_cells,
        work.requested.remaining_integration_cells,
        work.effective_integration_cells,
        work.pair.consumed_pair_bisections,
        work.integration_cells,
        work.winding_bisections,
        work.pair_domains as u64,
        work.carrier_pairs as u64,
        work.loops as u64,
        holes.len() as u64,
    ]);
    let cap = match resolution.cap {
        crate::profile_extrusion::ToleranceCap::NoLimit => [0, 0],
        crate::profile_extrusion::ToleranceCap::Report(limit) => [1, limit.to_bits()],
        crate::profile_extrusion::ToleranceCap::Refuse(limit) => [2, limit.to_bits()],
    };
    bits.extend(
        [
            resolution.sew,
            resolution.angular,
            resolution.display.chordal,
            resolution.display.angular,
        ]
        .map(f64::to_bits),
    );
    bits.extend(cap);
    for ring in std::iter::once(outer).chain(holes) {
        bits.push(ring.uses.len() as u64);
        for part in &ring.uses {
            bits.extend([part.curve.0, u64::from(part.reversed)]);
        }
    }
    bits
}
