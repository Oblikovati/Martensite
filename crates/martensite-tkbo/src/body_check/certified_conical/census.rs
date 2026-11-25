// SPDX-License-Identifier: GPL-2.0-only
//! Native binding counts have separate units from bounded arithmetic operations.
use super::*;
/// Exact inventory of independently captured actual geometry, without sampling.
/// Example: `let counts = body.native_census();`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ConicalNativeCensus {
    /// Source-bound native members.
    pub members: usize,
    /// Actual rational ruled sides.
    pub sides: usize,
    /// Actual analytic caps.
    pub caps: usize,
    /// Selected actual cap wires, including holes.
    pub cap_wires: usize,
    /// Actual cap edge occurrences.
    pub cap_edge_uses: usize,
    /// Captured Euclidean side controls across both rows.
    pub side_controls: usize,
    /// Captured actual polynomial cap UV controls.
    pub cap_uv_controls: usize,
    /// Actual shared-ruling incidences.
    pub shared_rulings: usize,
}
impl ConicalNativeGeometry {
    /// Counts for this complete native capture; these are not arithmetic work units.
    /// Example: `assert!(actual.census().caps > 0);`.
    pub fn census(&self) -> ConicalNativeCensus {
        let mut count = ConicalNativeCensus {
            members: self.members.len(),
            shared_rulings: self.adjacent.len(),
            ..Default::default()
        };
        for member in &self.members {
            accumulate(&mut count, member);
        }
        count
    }
}
fn accumulate(count: &mut ConicalNativeCensus, member: &MemberGeometry) {
    count.sides += member.sides.len();
    count.caps += member.caps.len();
    count.side_controls += member
        .sides
        .iter()
        .map(|side| side.rows.iter().map(Vec::len).sum::<usize>())
        .sum::<usize>();
    for cap in &member.caps {
        count.cap_wires += cap.contours.len();
        for contour in &cap.contours {
            count.cap_edge_uses += contour.edges.len();
            count.cap_uv_controls += contour
                .edges
                .iter()
                .map(|edge| edge.poles.len())
                .sum::<usize>();
        }
    }
}
