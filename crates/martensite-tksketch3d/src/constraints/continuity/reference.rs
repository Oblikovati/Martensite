// SPDX-License-Identifier: GPL-2.0-only
//! Bind a chart or a temporary authoring anchor to the accepted reference curve.
use super::{ContinuityFailure, pair};
use crate::constraints::{ConstraintFailure as Failure, RelationId};
use crate::{ControlSplineId, EntityId, LineGeometry, LineId, PointId, Snapshot};

#[derive(Clone, Copy)]
pub(in crate::constraints) enum JoinReference {
    Line(LineId),
    Cubic(ControlSplineId, PointId),
}

impl JoinReference {
    pub(in crate::constraints) fn chart_direction(
        self,
        geometry: &Snapshot,
        id: RelationId,
    ) -> Result<[f64; 3], Failure> {
        match self {
            Self::Line(line) => Ok(*line_source(geometry, id, line)?
                .carrier
                .direction()
                .coordinates()),
            Self::Cubic(curve, endpoint) => cubic_direction(geometry, id, curve, endpoint),
        }
    }

    pub(in crate::constraints) fn authoring_points(
        self,
        geometry: &Snapshot,
        id: RelationId,
    ) -> Result<Vec<PointId>, Failure> {
        match self {
            Self::Line(line) => Ok(line_source(geometry, id, line)?.line.endpoints.to_vec()),
            Self::Cubic(curve, _) => Ok(cubic_source(geometry, id, curve)?.to_vec()),
        }
    }
}

fn line_source(geometry: &Snapshot, id: RelationId, line: LineId) -> Result<LineGeometry, Failure> {
    geometry
        .line(line)
        .ok_or(Failure::MissingOperand(id, EntityId::Line(line)))
}

fn cubic_source(
    geometry: &Snapshot,
    id: RelationId,
    curve: ControlSplineId,
) -> Result<[PointId; 4], Failure> {
    geometry
        .control_spline(curve)
        .map(|cubic| cubic.recipe().poles)
        .ok_or(Failure::MissingOperand(id, EntityId::ControlSpline(curve)))
}

fn cubic_direction(
    geometry: &Snapshot,
    id: RelationId,
    curve: ControlSplineId,
    endpoint: PointId,
) -> Result<[f64; 3], Failure> {
    let fail = || Failure::Continuity(id, ContinuityFailure::ReferenceDirection(curve, endpoint));
    let source = cubic_source(geometry, id, curve)?;
    let [end, adjacent, _, _] = pair::at_end(source, endpoint).ok_or_else(fail)?;
    let position = |point| {
        geometry
            .point(point)
            .map(|value| *value.position.coordinates())
            .ok_or(Failure::MissingPoint(point))
    };
    let from = position(end)?;
    let to = position(adjacent)?;
    scaled_unit(std::array::from_fn(|axis| to[axis] - from[axis])).ok_or_else(fail)
}

fn scaled_unit(direction: [f64; 3]) -> Option<[f64; 3]> {
    let scale = direction.into_iter().map(f64::abs).fold(0., f64::max);
    if !scale.is_finite() || scale == 0. {
        return None;
    }
    let scaled = direction.map(|coordinate| coordinate / scale);
    let norm = scaled
        .into_iter()
        .map(|coordinate| coordinate * coordinate)
        .sum::<f64>()
        .sqrt();
    Some(scaled.map(|coordinate| coordinate / norm))
}
