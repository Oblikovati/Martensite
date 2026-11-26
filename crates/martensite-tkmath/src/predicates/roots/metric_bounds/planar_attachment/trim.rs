// SPDX-License-Identifier: GPL-2.0-only
//! Validate complete directed trim cycles before accepting any winding query.

use super::{
    ExactRatio as R, PLANAR_ATTACHMENT_MAX_LOOP_EDGES, PlanarAttachmentEdge,
    PlanarAttachmentError as Error, PlanarAttachmentLocation as Location,
    chart::{Chart, Uv, Vector},
    poll,
};
use crate::gp::{Axis2d, Line3};
use crate::predicates::roots::metric_bounds::placed_plane::{exact_point, exact_vector};
use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub(super) struct Loop {
    points: Vec<Uv>,
    pub sense: Ordering,
}

impl Loop {
    pub(super) fn capture(
        chart: &Chart,
        edges: &[PlanarAttachmentEdge],
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Self, Error> {
        if !(3..=PLANAR_ATTACHMENT_MAX_LOOP_EDGES).contains(&edges.len()) {
            return Err(Error::LoopSize(edges.len()));
        }
        let mut spans = Vec::with_capacity(edges.len());
        for (i, edge) in edges.iter().enumerate() {
            poll(cancelled)?;
            spans.push(edge_uv(chart, *edge, i)?);
        }
        let points = close(spans)?;
        let [a, b, c, ..] = points.as_slice() else {
            return Err(Error::Loop);
        };
        let sense = turn(a, b, c);
        if sense == Ordering::Equal {
            return Err(Error::Loop);
        }
        let ring = Self { points, sense };
        ring.require_convex(cancelled)?;
        Ok(ring)
    }

    fn require_convex(&self, cancelled: &mut impl FnMut() -> bool) -> Result<(), Error> {
        let length = self.points.len();
        for (i, (a, b)) in segments(&self.points).enumerate() {
            for (j, point) in self.points.iter().enumerate() {
                poll(cancelled)?;
                if j != i && j != (i + 1) % length && turn(a, b, point) != self.sense {
                    return Err(Error::Loop);
                }
            }
        }
        Ok(())
    }

    pub(super) fn require_inside(
        &self,
        outer: &Self,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<(), Error> {
        for point in &self.points {
            if outer.position(point, cancelled)? != Side::Inside {
                return Err(Error::Hole);
            }
        }
        poll(cancelled)
    }

    pub(super) fn clip(
        &self,
        source: [&Uv; 2],
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Option<[R; 2]>, Error> {
        self.clip_span(source, Boundary::Closed, cancelled)
    }

    pub(super) fn intersects_open_interior(
        &self,
        source: [&Uv; 2],
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<bool, Error> {
        Ok(self.clip_span(source, Boundary::Open, cancelled)?.is_some())
    }

    fn clip_span(
        &self,
        source: [&Uv; 2],
        boundary: Boundary,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Option<[R; 2]>, Error> {
        let mut domain = [R::from_finite(0.), R::from_finite(1.)];
        for (a, b) in segments(&self.points) {
            poll(cancelled)?;
            let sides = source.map(|point| self.inside_measure(a, b, point));
            if !clip_halfspace(&mut domain, sides, boundary) {
                return Ok(None);
            }
        }
        Ok(Some(domain))
    }

    fn inside_measure(&self, a: &Uv, b: &Uv, point: &Uv) -> R {
        let determinant = measure(a, b, point);
        if self.sense == Ordering::Greater {
            determinant
        } else {
            determinant.negated()
        }
    }

    pub(super) fn bernstein_contains<const N: usize>(
        &self,
        net: &[Uv; N],
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<bool, Error> {
        for (a, b) in segments(&self.points) {
            poll(cancelled)?;
            if net
                .iter()
                .any(|pole| self.inside_measure(a, b, pole).sign() == Ordering::Less)
            {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub(super) fn bernstein_excludes_open<const N: usize>(
        &self,
        net: &[Uv; N],
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<bool, Error> {
        for (a, b) in segments(&self.points) {
            poll(cancelled)?;
            if net
                .iter()
                .all(|pole| self.inside_measure(a, b, pole).sign() != Ordering::Greater)
            {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub(super) fn position(
        &self,
        point: &Uv,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Side, Error> {
        let mut side = Side::Inside;
        for (a, b) in segments(&self.points) {
            poll(cancelled)?;
            match turn(a, b, point) {
                Ordering::Equal => side = Side::Boundary,
                sign if sign != self.sense => return Ok(Side::Outside),
                _ => {}
            }
        }
        Ok(side)
    }
}

pub(super) fn locate(
    outer: &Loop,
    hole: Option<&Loop>,
    uv: &Uv,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Location, Error> {
    match outer.position(uv, cancelled)? {
        Side::Outside => return Ok(Location::OutsideOuter),
        Side::Boundary => return Ok(Location::OuterBoundary),
        Side::Inside => {}
    }
    let Some(hole) = hole else {
        return Ok(Location::Material);
    };
    Ok(match hole.position(uv, cancelled)? {
        Side::Outside => Location::Material,
        Side::Inside => Location::HoleInterior,
        Side::Boundary => Location::HoleBoundary,
    })
}

fn edge_uv(chart: &Chart, edge: PlanarAttachmentEdge, i: usize) -> Result<[Uv; 2], Error> {
    let mut world = line_ends(edge.line, edge.range, i)?;
    let mut uv = world.each_ref().map(|point| chart.pullback(point));
    if world[0] == world[1] || uv.each_ref().map(|value| chart.lift(value)) != world {
        return Err(Error::Edge(i));
    }
    if let Some((pcurve, range)) = edge.pcurve
        && (range != edge.range || pcurve_ends(pcurve, range, i)? != uv)
    {
        return Err(Error::Edge(i));
    }
    if edge.reversed {
        world.swap(0, 1);
        uv.swap(0, 1);
    }
    let [a, b] = edge.vertices;
    if [exact_vector(a, i)?, exact_vector(b, i)?] != world {
        return Err(Error::Edge(i));
    }
    Ok(uv)
}

fn line_ends(line: Line3, range: [f64; 2], i: usize) -> Result<[Vector; 2], Error> {
    if !range.into_iter().all(f64::is_finite) || range[0] >= range[1] {
        return Err(Error::Edge(i));
    }
    let origin = exact_point(line.location(), i)?;
    let direction = exact_vector(*line.direction().coordinates(), i)?;
    Ok(range.map(|t| {
        let parameter = R::from_finite(t);
        std::array::from_fn(|axis| origin[axis].add(&direction[axis].multiply(&parameter)))
    }))
}

fn pcurve_ends(pcurve: Axis2d, range: [f64; 2], i: usize) -> Result<[Uv; 2], Error> {
    let origin = pcurve.location();
    let direction = pcurve.direction();
    let a = exact_vector([origin.x(), origin.y(), 0.], i)?;
    let b = exact_vector([direction.x(), direction.y(), 0.], i)?;
    Ok(range.map(|t| {
        let parameter = R::from_finite(t);
        std::array::from_fn(|axis| a[axis].add(&b[axis].multiply(&parameter)))
    }))
}

fn close(spans: Vec<[Uv; 2]>) -> Result<Vec<Uv>, Error> {
    let following = spans.iter().cycle().skip(1);
    if spans.iter().zip(following).any(|(a, b)| a[1] != b[0]) {
        return Err(Error::Loop);
    }
    Ok(spans.into_iter().map(|[first, _]| first).collect())
}

fn segments(points: &[Uv]) -> impl Iterator<Item = (&Uv, &Uv)> {
    points.iter().zip(points.iter().cycle().skip(1))
}

fn turn(a: &Uv, b: &Uv, c: &Uv) -> Ordering {
    measure(a, b, c).sign()
}

fn measure(a: &Uv, b: &Uv, c: &Uv) -> R {
    let [x, y] = std::array::from_fn(|i| b[i].subtract(&a[i]));
    let [u, v] = std::array::from_fn(|i| c[i].subtract(&a[i]));
    x.multiply(&v).subtract(&y.multiply(&u))
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Boundary {
    Closed,
    Open,
}

impl Boundary {
    fn excludes(self, sign: Ordering) -> bool {
        sign == Ordering::Less || (self == Self::Open && sign == Ordering::Equal)
    }
}

fn clip_halfspace(domain: &mut [R; 2], [a, b]: [R; 2], boundary: Boundary) -> bool {
    match (boundary.excludes(a.sign()), boundary.excludes(b.sign())) {
        (true, true) => return false,
        (true, false) => {
            let root = a.divide(&a.subtract(&b));
            domain[0] = root.max(domain[0].clone());
        }
        (false, true) => {
            let root = a.divide(&a.subtract(&b));
            domain[1] = root.min(domain[1].clone());
        }
        (false, false) => {}
    }
    // A single rim point has no open interior. Closed clipping retains that touch.
    !boundary.excludes(domain[1].cmp(&domain[0]))
}

#[derive(PartialEq, Eq)]
pub(super) enum Side {
    Inside,
    Boundary,
    Outside,
}
