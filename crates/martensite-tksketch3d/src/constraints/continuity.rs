// SPDX-License-Identifier: GPL-2.0-only
//! Bind one incoming line and one outgoing control cubic by the authored join identity.
use super::{ConstraintFailure, ConstraintPolicy, RelationId};
use crate::{ControlSplineId, EntityId, LineGeometry, LineId, PointId, Snapshot};
use martensite_tkmath::predicates::roots::{CubicEndJet, CubicEndJetFailure, CubicEndStencil};
use std::{cmp::Ordering, fmt};

/// An incoming reference line and an outgoing control cubic with one shared endpoint.
///
/// Both traversal directions follow the selected point. During relation insertion only,
/// the line endpoints are held at their accepted coordinates while the cubic can adjust.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LineCubicJoin {
    /// Reference line, traversed toward the endpoint.
    pub line: LineId,
    /// Adjusted curve, traversed from the endpoint.
    pub cubic: ControlSplineId,
    /// Shared authored identity, which must be an endpoint of both curves.
    pub endpoint: PointId,
}

/// Inclusive spatial curvature difference in inverse model length units.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CurvatureTolerance(f64);

impl CurvatureTolerance {
    /// Set a nonnegative finite inverse-length limit. Example: `CurvatureTolerance::new(1e-7)?`.
    pub fn new(limit: f64) -> Result<Self, ContinuityFailure> {
        if !limit.is_finite() || limit < 0. {
            return Err(ContinuityFailure::InvalidCurvatureTolerance(limit));
        }
        Ok(Self(limit))
    }

    /// Return the inclusive inverse-length limit. Example: `limit.value()`.
    pub const fn value(self) -> f64 {
        self.0
    }
}

/// A declined line/cubic relation input.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ContinuityFailure {
    /// The selected point is absent from either set of authored endpoint identities.
    UnsharedEndpoint(LineCubicJoin),
    /// This stencil requires distinct authored controls apart from the common endpoint.
    RepeatedControl(PointId),
    /// The incoming line and outgoing cubic must start in the forward tangent branch.
    BackwardTangent,
    /// The exact endpoint stencil could not be evaluated.
    Endpoint(CubicEndJetFailure),
    /// Curvature tolerance must be a finite nonnegative inverse length.
    InvalidCurvatureTolerance(f64),
}

impl fmt::Display for ContinuityFailure {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Endpoint(source) => source.fmt(output),
            Self::InvalidCurvatureTolerance(value) => write!(
                output,
                "curvature tolerance {value} must be finite and nonnegative in inverse model units",
            ),
            cause => write!(output, "line/cubic continuity declined for {cause:?}"),
        }
    }
}

impl std::error::Error for ContinuityFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Endpoint(source) => Some(source),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct LineCubicStencil {
    pub(super) line: LineGeometry,
    points: [PointId; 4],
    count: usize,
}

impl LineCubicStencil {
    pub(super) fn points(&self) -> &[PointId] {
        &self.points[..self.count]
    }

    fn coordinates(self, geometry: &Snapshot) -> Option<[[f64; 3]; 4]> {
        let mut coordinates = [[0.; 3]; 4];
        for (xyz, &point) in coordinates.iter_mut().zip(self.points()) {
            *xyz = *geometry.point(point)?.position.coordinates();
        }
        Some(coordinates)
    }
}

pub(super) fn bind(
    geometry: &Snapshot,
    id: RelationId,
    join: LineCubicJoin,
    g2: bool,
) -> Result<LineCubicStencil, ConstraintFailure> {
    let line = geometry
        .line(join.line)
        .ok_or(ConstraintFailure::MissingOperand(
            id,
            EntityId::Line(join.line),
        ))?;
    let cubic = geometry
        .control_spline(join.cubic)
        .ok_or(ConstraintFailure::MissingOperand(
            id,
            EntityId::ControlSpline(join.cubic),
        ))?;
    let (points, count) = controls(line.line.endpoints, cubic.recipe().poles, join, g2)
        .map_err(|cause| ConstraintFailure::Continuity(id, cause))?;
    Ok(LineCubicStencil {
        line,
        points,
        count,
    })
}

fn controls(
    line: [PointId; 2],
    mut cubic: [PointId; 4],
    join: LineCubicJoin,
    g2: bool,
) -> Result<([PointId; 4], usize), ContinuityFailure> {
    let [tip, other] = if line[0] == join.endpoint {
        line
    } else {
        [line[1], line[0]]
    };
    if cubic[3] == join.endpoint {
        cubic.reverse();
    }
    if tip != join.endpoint || cubic[0] != tip {
        return Err(ContinuityFailure::UnsharedEndpoint(join));
    }
    let points = [other, tip, cubic[1], cubic[2]];
    let count = if g2 { 4 } else { 3 };
    for (index, &point) in points[..count].iter().enumerate() {
        if points[..index].contains(&point) {
            return Err(ContinuityFailure::RepeatedControl(point));
        }
    }
    Ok((points, count))
}

pub(super) fn check_seed(
    geometry: &Snapshot,
    id: RelationId,
    join: LineCubicJoin,
    g2: bool,
) -> Result<(), ConstraintFailure> {
    let stencil = bind(geometry, id, join, g2)?;
    let points = stencil
        .coordinates(geometry)
        .ok_or(ConstraintFailure::MissingOperand(
            id,
            EntityId::Point(join.endpoint),
        ))?;
    let (_, direction, _) =
        measure(points, false, 0.).map_err(|cause| ConstraintFailure::Continuity(id, cause))?;
    if direction != Ordering::Greater {
        return Err(ConstraintFailure::Continuity(
            id,
            ContinuityFailure::BackwardTangent,
        ));
    }
    Ok(())
}

pub(super) fn within(
    geometry: &Snapshot,
    join: LineCubicJoin,
    curvature: Option<CurvatureTolerance>,
    policy: ConstraintPolicy,
) -> bool {
    let Some(points) = source_points(geometry, join, curvature.is_some()) else {
        return false;
    };
    let Ok((jet, direction, sine)) =
        measure(points, curvature.is_some(), policy.parallel_sine_tolerance)
    else {
        return false;
    };
    direction == Ordering::Greater
        && sine != Ordering::Greater
        && curvature.is_none_or(|cap| {
            jet.curvature(cap.value())
                .is_ok_and(|limit| limit != Ordering::Greater)
        })
}

fn source_points(geometry: &Snapshot, join: LineCubicJoin, g2: bool) -> Option<[[f64; 3]; 4]> {
    let line = geometry.line(join.line)?;
    let cubic = geometry.control_spline(join.cubic)?;
    let (points, count) = controls(line.line.endpoints, cubic.recipe().poles, join, g2).ok()?;
    LineCubicStencil {
        line,
        points,
        count,
    }
    .coordinates(geometry)
}

fn measure(
    [incoming, join, control, second]: [[f64; 3]; 4],
    g2: bool,
    limit: f64,
) -> Result<(CubicEndJet, Ordering, Ordering), ContinuityFailure> {
    let stencil = if g2 {
        CubicEndStencil::Curvature([join, control, second])
    } else {
        CubicEndStencil::Tangent([join, control])
    };
    let jet = CubicEndJet::new(stencil).map_err(ContinuityFailure::Endpoint)?;
    let band = jet
        .tangent([incoming, join], limit)
        .map_err(ContinuityFailure::Endpoint)?;
    Ok((jet, band.dot, band.sine))
}
