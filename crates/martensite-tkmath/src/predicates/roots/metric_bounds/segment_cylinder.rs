// SPDX-License-Identifier: GPL-2.0-only
//! Exact directed segment contacts with a located, full-period cylindrical chart.
mod hole;
mod point;
mod root;
pub use point::{CylinderPointChart, CylinderPointError, CylinderPointQuery, require_nearby_lift};
mod trace;
pub use trace::{
    CylinderHoleCarrier, CylinderHoleTraceEdge, CylinderWallTraceInput,
    certify_cylinder_hole_trace, certify_cylinder_wall_trace,
};

use super::super::certificate_arithmetic::ExactRatio as Ratio;
use super::CylindricalProfileFailure;
use super::placed_plane::{ExactPlacement, ExactVector, cross, dot, exact_point, exact_vector};
pub use super::segment_plane::SegmentPlaneCut as SegmentCylinderCut;
use crate::{gp::Cylinder3, predicates::NonFiniteCoordinate};
use hole::Hole;
use root::Root;
use std::{cmp::Ordering, error::Error, fmt};

/// Exact source parameter position of an isolated native wall contact.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CylinderLinePlace {
    /// The original authored first point.
    Start,
    /// Strictly inside the directed source segment.
    Interior,
    /// The original authored last point.
    End,
}

/// Exact axial classification in the selected closed trim.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CylinderWallPlace {
    /// Below the first retained rim.
    Below,
    /// On the first retained rim.
    FirstRim,
    /// Inside the full-period wall. The topological seam is included.
    Interior,
    /// On the last retained rim.
    LastRim,
    /// Beyond the last retained rim.
    Above,
    /// On the closed boundary of the selected rectangular hole.
    HoleBoundary,
    /// Strictly inside the removed rectangular hole.
    HoleInterior,
}

impl CylinderWallPlace {
    fn on_wall(self) -> bool {
        !matches!(self, Self::Below | Self::Above | Self::HoleInterior)
    }
}

/// Multiplicity of an isolated intersection with the infinite support.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CylinderContactSense {
    /// The exact quadratic changes sign.
    Transverse,
    /// An exact double root where the source only touches the support.
    Tangent,
}

/// The bound classification of an isolated source contact, before line editing.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CylinderSegmentContact {
    /// Stable ordinal in increasing source parameter on the infinite support.
    pub root_ordinal: u8,
    /// Location within the closed authored line.
    pub line: CylinderLinePlace,
    /// Exact axial location in the selected wall.
    pub wall: CylinderWallPlace,
    /// Exact crossing or tangency.
    pub sense: CylinderContactSense,
    /// Positional evidence for interior transverse hits admitted by the closed trim.
    pub cut: Option<SegmentCylinderCut>,
}

/// The complete contact outcome of an exact segment query.
#[derive(Clone, Debug, PartialEq)]
pub enum CylinderSegmentIntersection {
    /// No isolated support root belongs to the authored segment and no material is shared.
    Disjoint,
    /// All isolated source roots in exact directed order, retaining axial trim misses.
    Isolated(Vec<CylinderSegmentContact>),
    /// A coincident generator reaches a single retained rim at an existing endpoint.
    GeneratorEndpoint {
        /// Endpoint of the original directed source.
        line: CylinderLinePlace,
        /// Retained rim reached by the endpoint.
        rim: CylinderWallPlace,
    },
    /// A generator was wholly within the open hole or reached its closed rim only at existing endpoints.
    GeneratorHole {
        /// The retained original endpoints on the hole rim, in source order.
        endpoints: [bool; 2],
    },
    /// A coincident generator shares a positive-length range with the actual axial trim.
    Overlap,
}

/// A cylindrical coefficient, query, or positional certificate was incomplete.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum CylinderSegmentError {
    /// An input point or placement coefficient is nonfinite.
    Coordinate(NonFiniteCoordinate),
    /// The stored actual cylinder radius is outside the positive finite domain.
    Radius(f64),
    /// The observed axial trim is not finite with positive length.
    Axial([f64; 2]),
    /// The stored placed chart does not have an exact nonsingular inverse.
    Frame,
    /// The two original source endpoints are equal.
    CoincidentEndpoints,
    /// Algebraic rounding did not finish within the fixed precision schedule.
    PositionEnclosure,
    /// A rectangular hole is outside the positive strictly interior native chart domain.
    HoleBounds {
        /// Literal angular pcurve bounds.
        angular: [f64; 2],
        /// Literal axial pcurve bounds.
        axial: [f64; 2],
    },
    /// Bounded exact comparisons could not separate an algebraic root and the literal angular ray.
    AngularSign(f64),
    /// Certification of a stored angular ray could not complete.
    Trigonometry(Box<CylindricalProfileFailure>),
    /// The operation observed cooperative cancellation.
    Cancelled,
}

impl fmt::Display for CylinderSegmentError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Coordinate(cause) => cause.fmt(out),
            Self::Radius(radius) => write!(
                out,
                "cylindrical radius {radius} must be finite and positive"
            ),
            Self::Axial(domain) => write!(
                out,
                "cylindrical axial interval {domain:?} must be finite and increasing"
            ),
            Self::Frame => {
                out.write_str("cylindrical intersection needs an invertible placed native chart")
            }
            Self::CoincidentEndpoints => {
                out.write_str("cylindrical intersection requires distinct source endpoints")
            }
            Self::PositionEnclosure => {
                out.write_str("cylindrical intersection position enclosure did not complete")
            }
            Self::HoleBounds { angular, axial } => write!(
                out,
                "cylindrical hole {angular:?} by {axial:?} needs positive bounds strictly within the outer wall"
            ),
            Self::AngularSign(angle) => write!(
                out,
                "cylindrical hole ray {angle} did not have a certified strict angular sign"
            ),
            Self::Trigonometry(cause) => {
                write!(out, "cylindrical angular certificate failed ({cause})")
            }
            Self::Cancelled => out.write_str("cylindrical intersection cancelled"),
        }
    }
}

impl Error for CylinderSegmentError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Coordinate(cause) => Some(cause),
            Self::Trigonometry(cause) => Some(cause.as_ref()),
            _ => None,
        }
    }
}

impl From<NonFiniteCoordinate> for CylinderSegmentError {
    fn from(cause: NonFiniteCoordinate) -> Self {
        Self::Coordinate(cause)
    }
}

impl CylinderSegmentError {
    fn from_trig(cause: impl Into<CylindricalProfileFailure>) -> Self {
        Self::Trigonometry(Box::new(cause.into()))
    }
}

/// Exact placed chart coefficients and a caller-certified full-period axial trim.
#[derive(Clone, Debug)]
pub struct SegmentCylinderWall {
    origin: ExactVector,
    inverse: [ExactVector; 3],
    axes: [ExactVector; 3],
    radius: Ratio,
    axial: [Ratio; 2],
    hole: Option<Hole>,
}

impl SegmentCylinderWall {
    /// Freeze the supplied stored chart and finite axial interval as exact values.
    /// The native adapter separately proves that the selected face covers the full period.
    /// `SegmentCylinderWall::capture(cylinder, placement_rows, [0., 2.])?` captures the domain.
    pub fn capture(
        cylinder: Cylinder3,
        placement: [[f64; 4]; 3],
        axial: [f64; 2],
    ) -> Result<Self, CylinderSegmentError> {
        let radius = cylinder.radius();
        if !radius.is_finite() || radius <= 0. {
            return Err(CylinderSegmentError::Radius(radius));
        }
        if !axial.iter().all(|n| n.is_finite()) || axial[0] >= axial[1] {
            return Err(CylinderSegmentError::Axial(axial));
        }
        let place = ExactPlacement::capture(placement)?;
        let origin = place.point(exact_point(cylinder.location(), 0)?);
        let frame = cylinder.position();
        let u = place.vector(exact_vector(*frame.x_direction().coordinates(), 1)?);
        let v = place.vector(exact_vector(*frame.y_direction().coordinates(), 2)?);
        let axis = place.vector(exact_vector(*frame.direction().coordinates(), 3)?);
        let axes = [u, v, axis];
        let inverse = invert(axes.clone())?;
        Ok(Self {
            origin,
            inverse,
            axes,
            radius: Ratio::from_finite(radius),
            axial: axial.map(Ratio::from_finite),
            hole: None,
        })
    }

    /// Bind one certified rectangular hole lying strictly inside the full-period chart.
    /// Example: `wall.with_rectangular_hole([0.5, 2.5], [1., 2.], || false)?`.
    pub fn with_rectangular_hole(
        mut self,
        angular: [f64; 2],
        axial: [f64; 2],
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<Self, CylinderSegmentError> {
        self.hole = Some(Hole::capture(angular, axial, &self.axial, &mut cancelled)?);
        Ok(self)
    }

    /// Classify exact roots and only then round the admitted interior crossings.
    /// Use `wall.intersect(endpoints, || false)?` for original directed binary64 endpoints.
    pub fn intersect(
        &self,
        source: [[f64; 3]; 2],
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<CylinderSegmentIntersection, CylinderSegmentError> {
        poll(&mut cancelled)?;
        let source = [exact_vector(source[0], 0)?, exact_vector(source[1], 1)?];
        if source[0] == source[1] {
            return Err(CylinderSegmentError::CoincidentEndpoints);
        }
        let [start, end] = source.each_ref().map(|p| self.pullback(p));
        let local = LocalSegment::new(start, &end);
        let result = self.contacts(source.each_ref(), &local, &mut cancelled)?;
        poll(&mut cancelled)?;
        Ok(result)
    }

    fn pullback(&self, world: &ExactVector) -> ExactVector {
        let relative = std::array::from_fn(|i| world[i].subtract(&self.origin[i]));
        self.inverse.each_ref().map(|row| dot(row, &relative))
    }

    fn contacts(
        &self,
        source: [&ExactVector; 2],
        local: &LocalSegment,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<CylinderSegmentIntersection, CylinderSegmentError> {
        use CylinderSegmentIntersection as Hit;
        let [a, b, c] = local.polynomial(&self.radius);
        if a.sign() == Ordering::Equal {
            if c.sign() != Ordering::Equal {
                return Ok(Hit::Disjoint);
            }
            return self.generator(local);
        }
        let determinant = b
            .multiply(&b)
            .subtract(&Ratio::from_finite(4.).multiply(&a).multiply(&c));
        let sense = match determinant.sign() {
            Ordering::Less => return Ok(Hit::Disjoint),
            Ordering::Equal => CylinderContactSense::Tangent,
            Ordering::Greater => CylinderContactSense::Transverse,
        };
        let mut contacts = Vec::new();
        for (ordinal, root) in [0, 1].into_iter().zip(Root::ordered(&a, &b, &determinant)) {
            if sense == CylinderContactSense::Tangent && ordinal == 1 {
                break;
            }
            poll(cancelled)?;
            if let Some(contact) = self.classify(local, &root, source, ordinal, sense, cancelled)? {
                contacts.push(contact);
            }
        }
        Ok(if contacts.is_empty() {
            Hit::Disjoint
        } else {
            Hit::Isolated(contacts)
        })
    }

    fn classify(
        &self,
        local: &LocalSegment,
        root: &Root,
        source: [&ExactVector; 2],
        root_ordinal: u8,
        sense: CylinderContactSense,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Option<CylinderSegmentContact>, CylinderSegmentError> {
        let Some(line) = classify_line(root) else {
            return Ok(None);
        };
        let [first, last] = self
            .axial
            .each_ref()
            .map(|rim| root.affine_sign(&local.start[2].subtract(rim), &local.rate[2]));
        let wall = match (&self.hole, classify_wall(first, last)) {
            (Some(hole), CylinderWallPlace::Interior) => hole.classify(local, root)?,
            (_, place) => place,
        };
        let cut = if line == CylinderLinePlace::Interior
            && wall.on_wall()
            && sense == CylinderContactSense::Transverse
        {
            Some(root.materialize(source, cancelled)?)
        } else {
            None
        };
        Ok(Some(CylinderSegmentContact {
            root_ordinal,
            line,
            wall,
            sense,
            cut,
        }))
    }

    fn generator(
        &self,
        local: &LocalSegment,
    ) -> Result<CylinderSegmentIntersection, CylinderSegmentError> {
        use CylinderSegmentIntersection as Hit;
        let [zero, one] = [0., 1.].map(Ratio::from_finite);
        let slope = &local.rate[2];
        if slope.sign() == Ordering::Equal {
            return Err(CylinderSegmentError::Frame);
        }
        if let Some(result) = self.generator_hole(local)? {
            return Ok(result);
        }
        let [a, b] = self
            .axial
            .each_ref()
            .map(|rim| rim.subtract(&local.start[2]).divide(slope));
        let low = a.clone().min(b.clone()).max(zero.clone());
        let high = a.max(b).min(one.clone());
        match low.cmp(&high) {
            Ordering::Greater => Ok(Hit::Disjoint),
            Ordering::Less => Ok(Hit::Overlap),
            Ordering::Equal => {
                let line = if low == zero {
                    CylinderLinePlace::Start
                } else if low == one {
                    CylinderLinePlace::End
                } else {
                    return Err(CylinderSegmentError::Frame);
                };
                let v = local.start[2].add(&slope.multiply(&low));
                let rim = classify_wall(v.cmp(&self.axial[0]), v.cmp(&self.axial[1]));
                Ok(Hit::GeneratorEndpoint { line, rim })
            }
        }
    }

    fn generator_hole(
        &self,
        local: &LocalSegment,
    ) -> Result<Option<CylinderSegmentIntersection>, CylinderSegmentError> {
        let Some(hole) = &self.hole else {
            return Ok(None);
        };
        let ends = [&local.start[2], &local.start[2].add(&local.rate[2])];
        let [first, last] = &hole.axial;
        if ends.iter().any(|end| *end < first || *end > last) {
            return Ok(None);
        }
        let [low, high] = hole.angular(local, &Root::zero())?;
        if low != Ordering::Greater || high != Ordering::Less {
            return Ok(None);
        }
        let endpoints = ends.map(|end| end == first || end == last);
        Ok(Some(CylinderSegmentIntersection::GeneratorHole {
            endpoints,
        }))
    }
}

struct LocalSegment {
    start: ExactVector,
    rate: ExactVector,
}

impl LocalSegment {
    fn new(start: ExactVector, end: &ExactVector) -> Self {
        let rate = std::array::from_fn(|i| end[i].subtract(&start[i]));
        Self { start, rate }
    }

    fn polynomial(&self, radius: &Ratio) -> [Ratio; 3] {
        let [x, y, _] = &self.start;
        let [dx, dy, _] = &self.rate;
        let a = dx.multiply(dx).add(&dy.multiply(dy));
        let b = Ratio::from_finite(2.).multiply(&x.multiply(dx).add(&y.multiply(dy)));
        let c = x
            .multiply(x)
            .add(&y.multiply(y))
            .subtract(&radius.multiply(radius));
        [a, b, c]
    }
}

fn invert([u, v, w]: [ExactVector; 3]) -> Result<[ExactVector; 3], CylinderSegmentError> {
    let cofactor = [cross(&v, &w), cross(&w, &u), cross(&u, &v)];
    let determinant = dot(&u, &cofactor[0]);
    if determinant.sign() == Ordering::Equal {
        return Err(CylinderSegmentError::Frame);
    }
    Ok(cofactor.map(|row| row.map(|entry| entry.divide(&determinant))))
}

fn classify_line(root: &Root) -> Option<CylinderLinePlace> {
    let [zero, one] = [0., 1.].map(Ratio::from_finite);
    match (
        root.affine_sign(&zero, &one),
        root.affine_sign(&one.negated(), &one),
    ) {
        (Ordering::Less, _) | (_, Ordering::Greater) => None,
        (Ordering::Equal, _) => Some(CylinderLinePlace::Start),
        (_, Ordering::Equal) => Some(CylinderLinePlace::End),
        _ => Some(CylinderLinePlace::Interior),
    }
}

fn classify_wall(first: Ordering, last: Ordering) -> CylinderWallPlace {
    match (first, last) {
        (Ordering::Less, _) => CylinderWallPlace::Below,
        (_, Ordering::Greater) => CylinderWallPlace::Above,
        (Ordering::Equal, _) => CylinderWallPlace::FirstRim,
        (_, Ordering::Equal) => CylinderWallPlace::LastRim,
        _ => CylinderWallPlace::Interior,
    }
}

fn poll(cancelled: &mut impl FnMut() -> bool) -> Result<(), CylinderSegmentError> {
    if cancelled() {
        return Err(CylinderSegmentError::Cancelled);
    }
    Ok(())
}

#[cfg(test)]
mod tests;
