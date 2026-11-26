// SPDX-License-Identifier: GPL-2.0-only
//! Bind the reference and adjusted curves by their shared authored endpoint.
use super::{ConstraintFailure, ConstraintPolicy, RelationId};
use crate::{ControlSplineId, EntityId, FitSplineId, LineId, PointId, Snapshot};
use martensite_tkmath::predicates::roots::{
    CubicEndJetFailure, CubicFitNumericError, CubicFitTangent,
};
use std::{cmp::Ordering, fmt};

mod fit;
mod pair;
pub use fit::LineFitCubicJoin;
mod reference;
pub use pair::CubicCubicJoin;
pub(super) use reference::JoinReference;

/// Directed continuity between supported curves at a shared point.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContinuityJoin {
    /// An incoming line and an outgoing control cubic.
    LineCubic(LineCubicJoin),
    /// An incoming line and an outgoing fixed four-station fit cubic. Tangent only.
    LineFitCubic(LineFitCubicJoin),
    /// An incoming and an outgoing control cubic.
    CubicCubic(CubicCubicJoin),
}

impl ContinuityJoin {
    pub(super) fn reference(self) -> JoinReference {
        match self {
            Self::LineCubic(join) => JoinReference::Line(join.line),
            Self::LineFitCubic(join) => JoinReference::Line(join.line),
            Self::CubicCubic(join) => JoinReference::Cubic(join.reference, join.endpoint),
        }
    }
}

impl From<LineCubicJoin> for ContinuityJoin {
    fn from(join: LineCubicJoin) -> Self {
        Self::LineCubic(join)
    }
}

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

/// A declined curve continuity relation input.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ContinuityFailure {
    /// The selected point is absent from either set of authored endpoint identities.
    UnsharedEndpoint(ContinuityJoin),
    /// This stencil requires distinct authored controls apart from the common endpoint.
    RepeatedControl(PointId),
    /// The incoming and outgoing curves must start in the forward tangent branch.
    BackwardTangent,
    /// The reference cubic's finite endpoint direction could not be represented.
    ReferenceDirection(ControlSplineId, PointId),
    /// The exact endpoint stencil could not be evaluated.
    Endpoint(CubicEndJetFailure),
    /// Curvature constraints on a fit cubic have no checked solver stencil.
    FitCurvatureUnsupported(FitSplineId),
    /// The retained station-to-pole derivative could not supply finite sensitivities.
    FitSensitivity(FitSplineId, CubicFitNumericError),
    /// The rebuilt stored endpoint does not retain the authored point's coordinate bits.
    FitEndpointIdentity(FitSplineId, PointId),
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
            cause => write!(output, "curve continuity declined for {cause:?}"),
        }
    }
}

impl std::error::Error for ContinuityFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Endpoint(source) => Some(source),
            Self::FitSensitivity(_, source) => Some(source),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct JoinStencil {
    points: [PointId; 5],
    count: usize,
    fit: Option<(FitSplineId, CubicFitTangent)>,
}

impl JoinStencil {
    pub(super) fn points(&self) -> &[PointId] {
        &self.points[..self.count]
    }

    pub(super) const fn fit_tangent(self) -> Option<(FitSplineId, CubicFitTangent)> {
        self.fit
    }

    fn coordinates(self, geometry: &Snapshot) -> Option<[[f64; 3]; 5]> {
        let mut coordinates = [[0.; 3]; 5];
        for (xyz, &point) in coordinates.iter_mut().zip(self.points()) {
            *xyz = *geometry.point(point)?.position.coordinates();
        }
        Some(coordinates)
    }
}

pub(super) fn bind(
    geometry: &Snapshot,
    id: RelationId,
    join: ContinuityJoin,
    g2: bool,
) -> Result<JoinStencil, ConstraintFailure> {
    let (points, count) = match join {
        ContinuityJoin::LineCubic(join) => line_stencil(geometry, id, join, g2)?,
        ContinuityJoin::LineFitCubic(join) => return fit::bind(geometry, id, join, g2),
        ContinuityJoin::CubicCubic(join) => pair::bind(geometry, id, join, g2)?,
    };
    Ok(JoinStencil {
        points,
        count,
        fit: None,
    })
}

fn line_stencil(
    geometry: &Snapshot,
    id: RelationId,
    join: LineCubicJoin,
    g2: bool,
) -> Result<([PointId; 5], usize), ConstraintFailure> {
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
    line_controls(line.line.endpoints, cubic.recipe().poles, join, g2)
        .map_err(|cause| ConstraintFailure::Continuity(id, cause))
}

fn line_controls(
    line: [PointId; 2],
    cubic: [PointId; 4],
    join: LineCubicJoin,
    g2: bool,
) -> Result<([PointId; 5], usize), ContinuityFailure> {
    let [tip, other] = if line[0] == join.endpoint {
        line
    } else {
        [line[1], line[0]]
    };
    let Some(cubic) = pair::at_end(cubic, join.endpoint).filter(|_| tip == join.endpoint) else {
        return Err(ContinuityFailure::UnsharedEndpoint(join.into()));
    };
    let points = [other, tip, cubic[1], cubic[2], tip];
    let count = if g2 { 4 } else { 3 };
    distinct(&points[..count])?;
    Ok((points, count))
}

fn distinct(points: &[PointId]) -> Result<(), ContinuityFailure> {
    for (index, &point) in points.iter().enumerate() {
        if points[..index].contains(&point) {
            return Err(ContinuityFailure::RepeatedControl(point));
        }
    }
    Ok(())
}

pub(super) fn check_seed(
    geometry: &Snapshot,
    id: RelationId,
    join: ContinuityJoin,
    g2: bool,
) -> Result<(), ConstraintFailure> {
    let stencil = bind(geometry, id, join, g2)?;
    let points = stencil
        .coordinates(geometry)
        .ok_or(ConstraintFailure::MissingOperand(
            id,
            EntityId::Point(stencil.points[1]),
        ))?;
    let forward = measure(geometry, id, join, points, 1., None)?;
    if !forward {
        return Err(ConstraintFailure::Continuity(
            id,
            ContinuityFailure::BackwardTangent,
        ));
    }
    Ok(())
}

pub(super) fn within(
    geometry: &Snapshot,
    id: RelationId,
    join: ContinuityJoin,
    curvature: Option<CurvatureTolerance>,
    policy: ConstraintPolicy,
) -> bool {
    let Some(points) = bind(geometry, id, join, curvature.is_some())
        .ok()
        .and_then(|stencil| stencil.coordinates(geometry))
    else {
        return false;
    };
    measure(
        geometry,
        id,
        join,
        points,
        policy.parallel_sine_tolerance,
        curvature.map(CurvatureTolerance::value),
    )
    .unwrap_or(false)
}

fn measure(
    geometry: &Snapshot,
    id: RelationId,
    join: ContinuityJoin,
    points: [[f64; 3]; 5],
    limit: f64,
    curvature: Option<f64>,
) -> Result<bool, ConstraintFailure> {
    let checked = match join {
        ContinuityJoin::LineCubic(_) => line_limits(points, limit, curvature),
        ContinuityJoin::LineFitCubic(join) => {
            return fit::limits(geometry, id, join, [points[0], points[1]], limit);
        }
        ContinuityJoin::CubicCubic(_) => pair::limits(points, limit, curvature),
    };
    checked.map_err(|cause| ConstraintFailure::Continuity(id, cause))
}

fn line_limits(
    [incoming, join, control, second, _]: [[f64; 3]; 5],
    limit: f64,
    curvature: Option<f64>,
) -> Result<bool, ContinuityFailure> {
    let jet = pair::jet(join, control, second, curvature.is_some())?;
    let band = jet
        .tangent([incoming, join], limit)
        .map_err(ContinuityFailure::Endpoint)?;
    if band.dot != Ordering::Greater || band.sine == Ordering::Greater {
        return Ok(false);
    }
    let Some(cap) = curvature else {
        return Ok(true);
    };
    Ok(jet.curvature(cap).map_err(ContinuityFailure::Endpoint)? != Ordering::Greater)
}
