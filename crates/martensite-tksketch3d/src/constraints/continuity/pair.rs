// SPDX-License-Identifier: GPL-2.0-only
//! Two cubic stencils share the join column and keep authored traversal.
use super::{ContinuityFailure, ContinuityJoin};
use crate::constraints::{ConstraintFailure, RelationId};
use crate::{ControlSplineId, EntityId, PointId, Snapshot};
use martensite_tkmath::predicates::roots::{CubicEndJet, CubicEndStencil};
use std::cmp::Ordering;

/// Two distinct control cubics joined at the selected authored point.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CubicCubicJoin {
    /// Reference curve traversed toward the endpoint and held during insertion.
    pub reference: ControlSplineId,
    /// Adjusted curve traversed away from the endpoint.
    pub adjusted: ControlSplineId,
    /// Common point identity at one endpoint of each curve.
    pub endpoint: PointId,
}

pub(in crate::constraints) fn bind(
    geometry: &Snapshot,
    id: RelationId,
    join: CubicCubicJoin,
    g2: bool,
) -> Result<([PointId; 5], usize), ConstraintFailure> {
    if join.reference == join.adjusted {
        return Err(ConstraintFailure::RepeatedOperand(
            id,
            EntityId::ControlSpline(join.reference),
        ));
    }
    let [reference, adjusted] = [join.reference, join.adjusted].map(|curve| {
        let curve_id = EntityId::ControlSpline(curve);
        geometry
            .control_spline(curve)
            .map(|curve| curve.recipe().poles)
            .ok_or(ConstraintFailure::MissingOperand(id, curve_id))
    });
    let [reference, adjusted] = [
        at_end(reference?, join.endpoint),
        at_end(adjusted?, join.endpoint),
    ];
    let (Some(a), Some(b)) = (reference, adjusted) else {
        return Err(ConstraintFailure::Continuity(
            id,
            ContinuityFailure::UnsharedEndpoint(join.into()),
        ));
    };
    let points = [a[1], join.endpoint, b[1], b[2], a[2]];
    let count = if g2 { 5 } else { 3 };
    super::distinct(&points[..count]).map_err(|cause| ConstraintFailure::Continuity(id, cause))?;
    Ok((points, count))
}

pub(in crate::constraints) fn at_end(
    mut controls: [PointId; 4],
    endpoint: PointId,
) -> Option<[PointId; 4]> {
    if controls[3] == endpoint {
        controls.reverse();
    }
    (controls[0] == endpoint).then_some(controls)
}

pub(super) fn limits(
    [adjacent, join, next, second, reference_second]: [[f64; 3]; 5],
    sine: f64,
    curvature: Option<f64>,
) -> Result<bool, ContinuityFailure> {
    let first = jet(join, adjacent, reference_second, curvature.is_some())?;
    let next = jet(join, next, second, curvature.is_some())?;
    let direction = first
        .tangent_into(&next, sine)
        .map_err(ContinuityFailure::Endpoint)?;
    if direction.dot != Ordering::Greater || direction.sine == Ordering::Greater {
        return Ok(false);
    }
    let Some(cap) = curvature else {
        return Ok(true);
    };
    let difference = first
        .curvature_difference(&next, cap)
        .map_err(ContinuityFailure::Endpoint)?;
    Ok(difference != Ordering::Greater)
}

pub(super) fn jet(
    join: [f64; 3],
    adjacent: [f64; 3],
    next: [f64; 3],
    g2: bool,
) -> Result<CubicEndJet, ContinuityFailure> {
    let stencil = if g2 {
        CubicEndStencil::Curvature([join, adjacent, next])
    } else {
        CubicEndStencil::Tangent([join, adjacent])
    };
    CubicEndJet::new(stencil).map_err(ContinuityFailure::Endpoint)
}

impl From<CubicCubicJoin> for ContinuityJoin {
    fn from(join: CubicCubicJoin) -> Self {
        Self::CubicCubic(join)
    }
}
