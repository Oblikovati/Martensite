// SPDX-License-Identifier: GPL-2.0-only
//! Borrowed Math queries are made from the independently captured native values.
use super::*;
impl ConicalNativeGeometry {
    /// Lend complete intrinsic cap sources and actual shared-ruling incidence.
    /// Example: `actual.with_cap_queries(|caps, adjacent| certify(caps, adjacent))`.
    pub fn with_cap_queries<T>(
        &self,
        apply: impl FnOnce(&[ConicalActualCapQuery<'_>], &[ConicalActualAdjacent]) -> T,
    ) -> T {
        let caps: Vec<_> = self
            .members
            .iter()
            .flat_map(|member| member.caps.iter().map(move |cap| (member.id, cap)))
            .collect();
        let edges: Vec<Vec<Vec<_>>> = caps
            .iter()
            .map(|(_, cap)| cap.contours.iter().map(edges).collect())
            .collect();
        let contours: Vec<Vec<_>> = caps
            .iter()
            .zip(&edges)
            .map(|((_, cap), edges)| contours(cap, edges))
            .collect();
        let queries: Vec<_> = caps
            .iter()
            .zip(&contours)
            .map(|((member, cap), contours)| cap_query(*member, cap, contours))
            .collect();
        apply(&queries, &self.adjacent)
    }
    /// Actual shell orientation for one exhaustively bound source side.
    /// Example: `actual.side_orientation(side_id)`.
    pub fn side_orientation(&self, id: ConicalActualSideId) -> Option<i8> {
        self.members
            .iter()
            .flat_map(|member| &member.sides)
            .find(|side| side.id == id.0)
            .map(|side| side.orientation)
    }
}
fn edges(contour: &native::Contour) -> Vec<ConicalActualUvEdge<'_>> {
    contour
        .edges
        .iter()
        .map(|edge| ConicalActualUvEdge {
            side: ConicalActualSideId(edge.side),
            row: edge.row,
            poles: &edge.poles,
            weights: &edge.weights,
            range: edge.range,
            reversed: edge.reversed,
        })
        .collect()
}
fn contours<'a>(
    cap: &native::Cap,
    edges: &'a [Vec<ConicalActualUvEdge<'a>>],
) -> Vec<ConicalActualUvContour<'a>> {
    cap.contours
        .iter()
        .zip(edges)
        .map(|(contour, edges)| ConicalActualUvContour {
            role: if contour.role == 0 {
                ConicalActualContourRole::Outer
            } else {
                ConicalActualContourRole::Hole
            },
            wire_orientation: contour.orientation,
            edges,
        })
        .collect()
}
fn cap_query<'a>(
    member: ConicalActualMemberId,
    cap: &native::Cap,
    contours: &'a [ConicalActualUvContour<'a>],
) -> ConicalActualCapQuery<'a> {
    ConicalActualCapQuery {
        carrier: ConicalActualCapCarrier {
            id: ConicalActualCapId(cap.id),
            member,
            frame: cap.frame,
            radius: cap.datums[0],
            axial_origin: cap.datums[1],
            semi_angle: cap.datums[2],
        },
        face_orientation: cap.orientation,
        contours,
    }
}
