// SPDX-License-Identifier: GPL-2.0-only
//! A directed line-to-circle tangent at one shared authored endpoint.
use super::{ContinuityFailure, ContinuityJoin, distinct, line_other};
use crate::{
    ArcDefinition, ArcGeometry, ArcId, EntityId, LineId, PointId, Snapshot,
    constraints::{ArcConstraintFailure, ConstraintFailure as Failure, RelationId, arc_layout},
};
use martensite_tkmath::{
    elclib,
    predicates::{
        ArcTripleFailure, compare_arc_endpoint_tangent_3d, compare_directed_line_sine_3d,
        compare_squared_length_3d,
    },
};
use std::cmp::Ordering;

/// An incoming reference line and an outgoing midpoint-witness circular arc.
///
/// Both curves share a point ID. Insertion holds the reference line's accepted endpoints.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LineArcJoin {
    /// Line traversed toward the selected endpoint.
    pub line: LineId,
    /// Adjusted circular arc, traversed away from the selected endpoint.
    pub arc: ArcId,
    /// Common authored endpoint identity.
    pub endpoint: PointId,
}

impl From<LineArcJoin> for ContinuityJoin {
    fn from(join: LineArcJoin) -> Self {
        Self::LineArc(join)
    }
}

pub(super) fn bind(
    geometry: &Snapshot,
    id: RelationId,
    join: LineArcJoin,
    g2: bool,
) -> Result<([PointId; 5], usize), Failure> {
    if g2 {
        return Err(Failure::Continuity(
            id,
            ContinuityFailure::ArcCurvatureMismatch(join.arc),
        ));
    }
    let line = geometry
        .line(join.line)
        .ok_or(Failure::MissingOperand(id, EntityId::Line(join.line)))?;
    let arc = arc_layout::operand(geometry, id, join.arc)?;
    if !matches!(arc.recipe().definition, ArcDefinition::Through { .. }) {
        return Err(Failure::Arc(
            join.arc,
            ArcConstraintFailure::CenterDefinition,
        ));
    }
    let points = slots(line.line.endpoints, arc.recipe().points(), join)
        .map_err(|cause| Failure::Continuity(id, cause))?;
    Ok((
        [points[0], points[1], points[2], points[3], join.endpoint],
        points.len(),
    ))
}

fn slots(
    line: [PointId; 2],
    [start, end, witness]: [PointId; 3],
    join: LineArcJoin,
) -> Result<[PointId; 4], ContinuityFailure> {
    let unshared = || ContinuityFailure::UnsharedEndpoint(join.into());
    let other_on_line = line_other(line, join.endpoint).ok_or_else(unshared)?;
    let other_on_arc = line_other([start, end], join.endpoint).ok_or_else(unshared)?;
    let points = [other_on_line, join.endpoint, other_on_arc, witness];
    distinct(&points)?;
    Ok(points)
}

pub(super) fn limits(
    geometry: &Snapshot,
    id: RelationId,
    join: LineArcJoin,
    [incoming, selected, other, witness, _]: [[f64; 3]; 5],
    sine: f64,
) -> Result<bool, Failure> {
    let arc = arc_layout::operand(geometry, id, join.arc)?;
    let authored =
        compare_arc_endpoint_tangent_3d([selected, other, witness], [incoming, selected], sine)
            .map_err(|cause| metric_failure(id, join.arc, cause))?;
    if !forward(authored) {
        return Ok(false);
    }
    let jet = stored_tangent(arc, id, join, selected)?;
    let native = compare_directed_line_sine_3d([incoming, selected], [[0.; 3], jet], sine)
        .map_err(|cause| metric_failure(id, join.arc, ArcTripleFailure::NonFinite(cause)))?;
    Ok(forward(native))
}

fn stored_tangent(
    arc: ArcGeometry,
    id: RelationId,
    join: LineArcJoin,
    authored: [f64; 3],
) -> Result<[f64; 3], Failure> {
    let at_start = arc.recipe().endpoints[0] == join.endpoint;
    let parameter = arc.range()[usize::from(!at_start)];
    let (value, derivative) = elclib::circle_d1(parameter, arc.frame(), arc.radius());
    let fit = compare_squared_length_3d(
        *value.coordinates(),
        authored,
        arc.recipe().fit_tolerance.value(),
    )
    .map_err(|cause| metric_failure(id, join.arc, ArcTripleFailure::NonFinite(cause)))?;
    if fit.is_gt() {
        return Err(Failure::Continuity(
            id,
            ContinuityFailure::ArcNativeEndpoint(join.arc, join.endpoint),
        ));
    }
    let direction = *derivative.xyz().coordinates();
    Ok(if at_start {
        direction
    } else {
        direction.map(|value| -value)
    })
}

fn forward((dot, sine): (Ordering, Ordering)) -> bool {
    dot.is_gt() && !sine.is_gt()
}

fn metric_failure(id: RelationId, arc: ArcId, cause: ArcTripleFailure) -> Failure {
    Failure::Continuity(id, ContinuityFailure::ArcTangent(arc, cause))
}
