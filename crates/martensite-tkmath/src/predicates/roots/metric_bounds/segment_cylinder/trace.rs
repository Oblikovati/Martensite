// SPDX-License-Identifier: GPL-2.0-only
//! Bound actual rim, seam and common vertex representations against the cylindrical chart.
use super::{CylinderSegmentError as Error, Ratio, cross, poll};
use crate::{
    gp::{Circle3, Cylinder3, Line3},
    predicates::roots::{
        certificate_arithmetic::rounding::outward_binary64,
        metric_bounds::placed_plane::{ExactPlacement, ExactVector, exact_point, exact_vector},
    },
};
use std::{cmp::Ordering, f64::consts::TAU};

/// Stored native evidence from the selected full-period wall trim.
#[derive(Clone, Copy, Debug)]
pub struct CylinderWallTraceInput {
    /// Actual local chart and radius.
    pub cylinder: Cylinder3,
    /// Stored affine placement coefficients of this occurrence.
    pub placement: [[f64; 4]; 3],
    /// Exact chart bounds from the two original horizontal rim pcurves.
    pub axial: [f64; 2],
    /// The two actual original circular rim curves, in axial order.
    pub rims: [Circle3; 2],
    /// The actual spatial line shared by both coedges of the periodic seam.
    pub generator: Line3,
    /// Increasing range of the stored spatial generator and its two pcurves.
    pub generator_range: [f64; 2],
    /// Shared endpoint vertices, in increasing generator parameter.
    pub anchors: [[f64; 3]; 2],
}

/// Bound the largest model-space L1 error of the actual chart, rims and seam joins.
/// The native caller proves the literal trim ranges and topology before calling this predicate.
/// `certify_cylinder_wall_trace(input, || false)?` returns an outward bound in model units.
pub fn certify_cylinder_wall_trace(
    input: CylinderWallTraceInput,
    mut cancelled: impl FnMut() -> bool,
) -> Result<f64, Error> {
    poll(&mut cancelled)?;
    let context = Trace::capture(input)?;
    if input.anchors[0] == input.anchors[1] {
        return Err(Error::Frame);
    }
    let mut maximum = Ratio::from_finite(0.);
    for i in 0..2 {
        poll(&mut cancelled)?;
        maximum = maximum.max(context.rim(input, i)?);
        maximum = maximum.max(context.seam(input, i)?);
    }
    poll(&mut cancelled)?;
    Ok(outward_binary64(&maximum).ok_or(Error::PositionEnclosure)?[1])
}

struct Trace {
    placement: ExactPlacement,
    origin: ExactVector,
    radial: [ExactVector; 2],
    axis: ExactVector,
}

impl Trace {
    fn capture(input: CylinderWallTraceInput) -> Result<Self, Error> {
        let placement = ExactPlacement::capture(input.placement)?;
        let frame = input.cylinder.position();
        let origin = placement.point(exact_point(frame.location(), 0)?);
        let radius = finite(input.cylinder.radius(), 1)?;
        let x = placement.vector(exact_vector(*frame.x_direction().coordinates(), 2)?);
        let y = placement.vector(exact_vector(*frame.y_direction().coordinates(), 3)?);
        let axis = placement.vector(exact_vector(*frame.direction().coordinates(), 4)?);
        let radial = [x, y].map(|row| scale(&radius, &row));
        Ok(Self {
            placement,
            origin,
            radial,
            axis,
        })
    }

    fn rim(&self, input: CylinderWallTraceInput, i: usize) -> Result<Ratio, Error> {
        let circle = input.rims[i];
        let expected_center = self.center(input.axial[i])?;
        let center = self.placement.point(exact_point(circle.location(), i)?);
        let radial = self.rim_radial(circle, i)?;
        let trace = distance(&center, &expected_center)
            .add(&distance(&radial[0], &self.radial[0]))
            .add(&distance(&radial[1], &self.radial[1]));
        let vertex = self.placement.point(exact_vector(input.anchors[i], i)?);
        let endpoint = add(&center, &radial[0]);
        let join = distance(&endpoint, &vertex).add(&stored_period_error(&radial));
        Ok(trace.max(join))
    }

    fn rim_radial(&self, circle: Circle3, i: usize) -> Result<[ExactVector; 2], Error> {
        let radius = finite(circle.radius(), i)?;
        let frame = circle.position();
        let x = self
            .placement
            .vector(exact_vector(*frame.x_direction().coordinates(), i)?);
        let y = self
            .placement
            .vector(exact_vector(*frame.y_direction().coordinates(), i)?);
        if radius.sign() != Ordering::Greater || is_zero(&cross(&x, &y)) {
            return Err(Error::Frame);
        }
        Ok([x, y].map(|vector| scale(&radius, &vector)))
    }

    fn seam(&self, input: CylinderWallTraceInput, i: usize) -> Result<Ratio, Error> {
        let ideal = add(&self.center(input.axial[i])?, &self.radial[0]);
        let origin = self
            .placement
            .point(exact_point(input.generator.location(), i)?);
        let direction = self
            .placement
            .vector(exact_vector(*input.generator.direction().coordinates(), i)?);
        if is_zero(&direction) {
            return Err(Error::Frame);
        }
        let range = finite(input.generator_range[i], i)?;
        let edge = add(&origin, &scale(&range, &direction));
        let vertex = self.placement.point(exact_vector(input.anchors[i], i)?);
        let period_error = stored_period_error(&self.radial);
        let edge_error = distance(&ideal, &edge).add(&period_error);
        let vertex_error = distance(&ideal, &vertex).add(&period_error);
        Ok(edge_error.max(vertex_error).max(distance(&edge, &vertex)))
    }

    fn center(&self, v: f64) -> Result<ExactVector, Error> {
        Ok(add(&self.origin, &scale(&finite(v, 5)?, &self.axis)))
    }
}

fn stored_period_error(radial: &[ExactVector; 2]) -> Ratio {
    // Mathematical 2π lies between the two binary64 neighbours of TAU.
    // Both sin and cos change by at most the argument error.
    let angle = Ratio::from_finite(TAU.next_up()).subtract(&Ratio::from_finite(TAU));
    let rate = norm(&radial[0]).add(&norm(&radial[1]));
    angle.multiply(&rate)
}

fn finite(value: f64, point: usize) -> Result<Ratio, Error> {
    let [value, _, _] = exact_vector([value, 0., 0.], point)?;
    Ok(value)
}

fn add(a: &ExactVector, b: &ExactVector) -> ExactVector {
    std::array::from_fn(|i| a[i].add(&b[i]))
}

fn scale(factor: &Ratio, point: &ExactVector) -> ExactVector {
    point.each_ref().map(|value| value.multiply(factor))
}

fn distance(a: &ExactVector, b: &ExactVector) -> Ratio {
    norm(&std::array::from_fn(|i| a[i].subtract(&b[i])))
}

fn is_zero(vector: &ExactVector) -> bool {
    vector
        .iter()
        .all(|component| component.sign() == Ordering::Equal)
}

fn norm(point: &ExactVector) -> Ratio {
    point.iter().fold(Ratio::from_finite(0.), |sum, value| {
        if value.sign() == Ordering::Less {
            sum.subtract(value)
        } else {
            sum.add(value)
        }
    })
}
