// SPDX-License-Identifier: GPL-2.0-only
use super::{
    AxialProjectionLine, Checked, CylinderProjectionError as Failure, ProjectionCylinder,
    ProjectionHole,
};
use crate::gp::{Axis3, Direction3};
use crate::predicates::roots::{
    certificate_arithmetic::{ExactRatio, rounding::outward_binary64},
    metric_bounds::placed_plane::{ExactPlacement, ExactVector, dot, exact_point, exact_vector},
};
use std::cmp::Ordering;

pub(super) type Ratio = ExactRatio;
pub(super) type Vector = ExactVector;

#[derive(Clone, Debug)]
pub(super) struct Chart {
    pub center: Vector,
    pub axes: [Vector; 3],
    pub radius: Ratio,
    pub axial: [Ratio; 2],
    pub hole: Option<ProjectionHole>,
}

#[derive(Clone, Debug)]
pub(super) struct AxialSource {
    pub origin: Vector,
    pub slope: Vector,
    pub height: [Ratio; 2],
    pub radial: [Ratio; 2],
    pub range: [Ratio; 2],
}

impl Chart {
    pub fn capture(input: ProjectionCylinder) -> Checked<Self> {
        let frame = admitted_frame(input.cylinder.position())?;
        let placement = ExactPlacement::capture(input.placement)?;
        let center = placement.point(exact_point(frame.location(), 0)?);
        let axes = placed_axes(frame, &placement)?;
        if !signed_permutation(&axes) {
            return Err(Failure::Frame);
        }
        let radius = input.cylinder.radius();
        if !radius.is_finite() || radius <= 0. {
            return Err(Failure::Radius(radius));
        }
        let axial = range(input.axial, Failure::Trim)?;
        if let Some(hole) = input.hole {
            validate_hole(hole, &axial)?;
        }
        Ok(Self {
            center,
            axes,
            radius: Ratio::from_finite(radius),
            axial,
            hole: input.hole,
        })
    }

    pub fn vector_coordinates(&self, vector: &Vector) -> Vector {
        self.axes.each_ref().map(|axis| dot(axis, vector))
    }

    pub fn direction_coordinates(&self, direction: Direction3) -> Checked<Vector> {
        Ok(self.vector_coordinates(&exact_vector(*direction.coordinates(), 3)?))
    }
}

impl AxialSource {
    pub fn capture(input: AxialProjectionLine, chart: &Chart) -> Checked<Self> {
        let placement = ExactPlacement::capture(input.placement)?;
        let origin = placement.point(exact_point(input.line.location(), 4)?);
        let slope = placement.vector(exact_vector(*input.line.direction().coordinates(), 5)?);
        let [p, q, h] = chart.vector_coordinates(&sub(&origin, &chart.center));
        let [dp, dq, dh] = chart.vector_coordinates(&slope);
        if !zero(&dp) || !zero(&dq) || zero(&dh) {
            return Err(Failure::Source);
        }
        let range = range(input.range, Failure::Source)?;
        let source = Self {
            origin,
            slope,
            height: [h, dh],
            radial: [p, q],
            range,
        };
        if source.range.iter().any(|t| {
            let axial = source.height_at(t);
            axial <= chart.axial[0] || axial >= chart.axial[1]
        }) {
            return Err(Failure::Trim);
        }
        Ok(source)
    }

    pub fn height_at(&self, t: &Ratio) -> Ratio {
        self.height[0].add(&self.height[1].multiply(t))
    }

    pub fn parameter_at(&self, height: &Ratio) -> Ratio {
        height.subtract(&self.height[0]).divide(&self.height[1])
    }
}

fn admitted_frame(frame: Axis3) -> Checked<Axis3> {
    Axis3::from_stored_coefficients(
        frame.location(),
        *frame.direction().coordinates(),
        *frame.x_direction().coordinates(),
        *frame.y_direction().coordinates(),
    )
    .map_err(|_| Failure::Frame)
}

fn placed_axes(frame: Axis3, placement: &ExactPlacement) -> Checked<[Vector; 3]> {
    let [u, v, axial] =
        [frame.x_direction(), frame.y_direction(), frame.direction()].map(|direction| {
            exact_vector(*direction.coordinates(), 1).map(|vector| placement.vector(vector))
        });
    Ok([u?, v?, axial?])
}

fn signed_permutation(axes: &[Vector; 3]) -> bool {
    let one = Ratio::from_finite(1.);
    axes.iter().enumerate().all(|(i, axis)| {
        dot(axis, axis) == one && axes[..i].iter().all(|prior| zero(&dot(axis, prior)))
    })
}

fn validate_hole(hole: ProjectionHole, outer_axial: &[Ratio; 2]) -> Checked<()> {
    let angle = range(hole.angle, Failure::Trim)?;
    let axial = range(hole.axial, Failure::Trim)?;
    if angle[0].sign() == Ordering::Greater
        && angle[1] < Ratio::from_finite(std::f64::consts::TAU)
        && outer_axial[0] < axial[0]
        && axial[1] < outer_axial[1]
    {
        return Ok(());
    }
    Err(Failure::Trim)
}

fn range(bounds: [f64; 2], failure: Failure) -> Checked<[Ratio; 2]> {
    if !bounds.into_iter().all(f64::is_finite) || bounds[0] >= bounds[1] {
        return Err(failure);
    }
    Ok(bounds.map(Ratio::from_finite))
}

pub(super) fn sub(a: &Vector, b: &Vector) -> Vector {
    std::array::from_fn(|i| a[i].subtract(&b[i]))
}

pub(super) fn zero(number: &Ratio) -> bool {
    number.sign() == Ordering::Equal
}

pub(super) fn bounds(number: &Ratio) -> Checked<[f64; 2]> {
    outward_binary64(number).ok_or(Failure::Unrepresentable)
}
