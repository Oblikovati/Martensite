// SPDX-License-Identifier: GPL-2.0-only
//! A line and a fixed-station fit cubic share one authored endpoint slot.
use super::{ContinuityFailure, ContinuityJoin, JoinStencil, distinct, pair};
use crate::{
    EntityId, FitSplineGeometry, FitSplineId, LineId, PointId, Snapshot,
    constraints::{ConstraintFailure as Failure, RelationId},
};
use martensite_tkmath::predicates::roots::{CubicEndJet, CubicEndStencil, CubicFitEnd};
use std::cmp::Ordering;

/// An incoming line and an outgoing four-station fit cubic with one shared endpoint.
///
/// The line endpoints are held during insertion. Later solves use the authored hard relations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LineFitCubicJoin {
    /// Reference line, traversed toward the endpoint.
    pub line: LineId,
    /// Adjusted fit curve, traversed from the selected endpoint into the cubic.
    pub fit: FitSplineId,
    /// Common authored endpoint identity.
    pub endpoint: PointId,
}

impl From<LineFitCubicJoin> for ContinuityJoin {
    fn from(join: LineFitCubicJoin) -> Self {
        Self::LineFitCubic(join)
    }
}

pub(super) fn bind(
    geometry: &Snapshot,
    id: RelationId,
    join: LineFitCubicJoin,
    g2: bool,
) -> Result<JoinStencil, Failure> {
    let refuse = |cause| Failure::Continuity(id, cause);
    if g2 {
        return Err(refuse(ContinuityFailure::FitCurvatureUnsupported(join.fit)));
    }
    let line = geometry
        .line(join.line)
        .ok_or(Failure::MissingOperand(id, EntityId::Line(join.line)))?;
    let fit = fit_source(geometry, id, join.fit)?;
    let (points, end) = slots(line.line.endpoints, fit.recipe().stations, join).map_err(refuse)?;
    Ok(JoinStencil {
        points,
        count: points.len(),
        fit: Some((join.fit, fit.solver_tangent(end))),
    })
}

fn slots(
    line: [PointId; 2],
    stations: [PointId; 4],
    join: LineFitCubicJoin,
) -> Result<([PointId; 5], CubicFitEnd), ContinuityFailure> {
    let other = line
        .iter()
        .position(|point| *point == join.endpoint)
        .map(|end| line[1 - end]);
    let (Some(other), Some([shared, a, b, c])) = (other, pair::at_end(stations, join.endpoint))
    else {
        return Err(ContinuityFailure::UnsharedEndpoint(join.into()));
    };
    let points = [other, shared, a, b, c];
    distinct(&points)?;
    let end = if stations[0] == shared {
        CubicFitEnd::Start
    } else {
        CubicFitEnd::End
    };
    Ok((points, end))
}

pub(super) fn limits(
    geometry: &Snapshot,
    id: RelationId,
    join: LineFitCubicJoin,
    line: [[f64; 3]; 2],
    sine: f64,
) -> Result<bool, Failure> {
    let fit = fit_source(geometry, id, join.fit)?;
    let jet = stored_jet(fit, join, line[1]).map_err(|cause| Failure::Continuity(id, cause))?;
    let band = jet
        .tangent(line, sine)
        .map_err(|cause| Failure::Continuity(id, ContinuityFailure::Endpoint(cause)))?;
    Ok(band.dot == Ordering::Greater && band.sine != Ordering::Greater)
}

fn stored_jet(
    fit: &FitSplineGeometry,
    join: LineFitCubicJoin,
    shared: [f64; 3],
) -> Result<CubicEndJet, ContinuityFailure> {
    let mut stored = fit.poles().map(|point| *point.coordinates());
    if fit.recipe().stations[0] != join.endpoint {
        stored.reverse();
    }
    if stored[0].map(f64::to_bits) != shared.map(f64::to_bits) {
        return Err(ContinuityFailure::FitEndpointIdentity(
            join.fit,
            join.endpoint,
        ));
    }
    let jet = CubicEndJet::new(CubicEndStencil::Tangent([stored[0], stored[1]]))
        .map_err(ContinuityFailure::Endpoint)?;
    jet.rounded().map_err(ContinuityFailure::Endpoint)?;
    Ok(jet)
}

fn fit_source(
    geometry: &Snapshot,
    id: RelationId,
    fit: FitSplineId,
) -> Result<&FitSplineGeometry, Failure> {
    geometry
        .fit_spline(fit)
        .ok_or(Failure::MissingOperand(id, EntityId::FitSpline(fit)))
}
