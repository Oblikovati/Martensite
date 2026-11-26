// SPDX-License-Identifier: GPL-2.0-only
//! Model-space trace of the actual arcs, generators and vertices of a rectangular UV hole.
use super::{
    Circle3, CylinderWallTraceInput, Error, ExactVector, Line3, Ratio, Trace, add, distance,
    exact_point, exact_vector, finite, is_zero, outward_binary64, poll, scale,
};
use crate::predicates::roots::metric_bounds::segment_cylinder::hole::trig_bounds;

/// Actual native curve of an oriented hole coedge, stored in increasing curve parameter.
#[derive(Clone, Copy, Debug)]
pub enum CylinderHoleCarrier {
    /// A circular arc whose raw parameter is the chart's U.
    Arc(Circle3),
    /// A generator line whose raw parameter is the chart's V.
    Generator(Line3),
}

/// Native spatial and chart evidence for one side of a rectangular hole.
#[derive(Clone, Copy, Debug)]
pub struct CylinderHoleTraceEdge {
    /// Actual native spatial curve.
    pub carrier: CylinderHoleCarrier,
    /// Original increasing parameter range of the two bound representations.
    pub range: [f64; 2],
    /// Fixed V for an arc or fixed U for a generator.
    pub fixed: f64,
    /// Both actual shared vertices in increasing curve parameter.
    pub vertices: [[f64; 3]; 2],
}

/// Bound the four native traces and their vertex joins to the certified chart.
/// The topology adapter must first prove the same raw parameterization and exact UV loop.
/// Example: `certify_cylinder_hole_trace(wall, hole_edges, || false)?`.
pub fn certify_cylinder_hole_trace(
    wall: CylinderWallTraceInput,
    edges: [CylinderHoleTraceEdge; 4],
    mut cancelled: impl FnMut() -> bool,
) -> Result<f64, Error> {
    let context = Trace::capture(wall)?;
    let mut maximum = Ratio::from_finite(0.);
    for edge in edges {
        poll(&mut cancelled)?;
        let error = match edge.carrier {
            CylinderHoleCarrier::Arc(circle) => context.hole_arc(circle, edge, &mut cancelled)?,
            CylinderHoleCarrier::Generator(line) => {
                context.hole_generator(line, edge, &mut cancelled)?
            }
        };
        maximum = maximum.max(error);
    }
    poll(&mut cancelled)?;
    Ok(outward_binary64(&maximum).ok_or(Error::PositionEnclosure)?[1])
}

impl Trace {
    fn hole_arc(
        &self,
        circle: Circle3,
        edge: CylinderHoleTraceEdge,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Ratio, Error> {
        let center = self.placement.point(exact_point(circle.location(), 0)?);
        let radial = self.rim_radial(circle, 0)?;
        let expected = self.center(edge.fixed)?;
        let mut error = distance(&center, &expected)
            .add(&distance(&radial[0], &self.radial[0]))
            .add(&distance(&radial[1], &self.radial[1]));
        for (i, u) in edge.range.into_iter().enumerate() {
            let bounds = trig_bounds(u, cancelled)?;
            let vertex = self.placement.point(exact_vector(edge.vertices[i], i)?);
            error = error.max(point_trace(&center, &radial, &bounds, &vertex));
        }
        Ok(error)
    }

    fn hole_generator(
        &self,
        line: Line3,
        edge: CylinderHoleTraceEdge,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Ratio, Error> {
        let bounds = trig_bounds(edge.fixed, cancelled)?;
        let origin = self.placement.point(exact_point(line.location(), 0)?);
        let direction = self
            .placement
            .vector(exact_vector(*line.direction().coordinates(), 0)?);
        if is_zero(&direction) {
            return Err(Error::Frame);
        }
        let mut error = Ratio::from_finite(0.);
        for (i, v) in edge.range.into_iter().enumerate() {
            let expected = self.center(v)?;
            let actual = add(&origin, &scale(&finite(v, i)?, &direction));
            let vertex = self.placement.point(exact_vector(edge.vertices[i], i)?);
            error = error.max(point_trace(&expected, &self.radial, &bounds, &actual));
            error = error.max(distance(&actual, &vertex));
        }
        Ok(error)
    }
}

fn point_trace(
    origin: &ExactVector,
    radial: &[ExactVector; 2],
    [cosines, sines]: &[[Ratio; 2]; 2],
    point: &ExactVector,
) -> Ratio {
    let mut maximum = Ratio::from_finite(0.);
    for cosine in cosines {
        for sine in sines {
            let offset = add(&scale(cosine, &radial[0]), &scale(sine, &radial[1]));
            maximum = maximum.max(distance(&add(origin, &offset), point));
        }
    }
    maximum
}
