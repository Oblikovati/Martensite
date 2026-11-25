// SPDX-License-Identifier: GPL-2.0-only
//! Literal affine geometry in the original source charts.
use super::{
    Checked, PlanarSectionAnchor, PlanarSectionError as Failure, SectionChartSource,
    SectionLineSource, SectionUvLine, SectionVertexSource,
};
use crate::gp::Axis3;
use crate::predicates::roots::{
    certificate_arithmetic::{
        ExactRatio,
        rounding::{nearest_binary64, outward_binary64, outward_sqrt},
    },
    metric_bounds::placed_plane::{
        ExactPlacement, ExactVector, cross, dot, exact_point, exact_vector,
    },
};
use std::cmp::Ordering;

pub(super) type Scalar = ExactRatio;
pub(super) type Vector = ExactVector;
pub(super) type Uv = [Scalar; 2];

#[derive(Clone, Debug)]
pub(super) struct Chart {
    origin: Vector,
    pub normal: Vector,
    axes: [Vector; 2],
    pivot: [usize; 2],
}

#[derive(Clone, Debug)]
pub(super) struct Hit {
    pub point: Vector,
    pub anchor: PlanarSectionAnchor,
}

impl Chart {
    pub fn capture(source: SectionChartSource, index: usize) -> Checked<Self> {
        let placement = ExactPlacement::capture(source.placement)?;
        let frame = admit_frame(source.plane.position(), index)?;
        let origin = placement.point(exact_point(frame.location(), 0)?);
        let axes = [frame.x_direction(), frame.y_direction()];
        let [u, v] = axes.map(|direction| exact_vector(*direction.coordinates(), 1));
        let axes = [placement.vector(u?), placement.vector(v?)];
        let normal = cross(&axes[0], &axes[1]);
        let pivot = [[0, 1], [0, 2], [1, 2]]
            .into_iter()
            .find(|&[i, j]| !zero(&minor(&axes, i, j)))
            .ok_or(Failure::DegenerateChart(index))?;
        Ok(Self {
            origin,
            normal,
            axes,
            pivot,
        })
    }

    pub fn dist(&self, point: &Vector) -> Scalar {
        dot(&sub(point, &self.origin), &self.normal)
    }

    pub fn pullback(&self, point: &Vector) -> Uv {
        let [i, j] = self.pivot;
        let [u, v] = &self.axes;
        let d = sub(point, &self.origin);
        let det = minor(&self.axes, i, j);
        [
            d[i].multiply(&v[j])
                .subtract(&d[j].multiply(&v[i]))
                .divide(&det),
            u[i].multiply(&d[j])
                .subtract(&u[j].multiply(&d[i]))
                .divide(&det),
        ]
    }

    pub fn lift(&self, [u, v]: &Uv) -> Vector {
        std::array::from_fn(|i| {
            self.origin[i]
                .add(&self.axes[0][i].multiply(u))
                .add(&self.axes[1][i].multiply(v))
        })
    }

    pub fn separation(&self, other: &Self) -> Scalar {
        self.dist(&other.origin)
    }
}

fn admit_frame(frame: Axis3, chart: usize) -> Checked<Axis3> {
    // The native plane equation and the stored UV chart must agree on the frame.
    // The established validator preserves all coefficients and accepts either handedness.
    Axis3::from_stored_coefficients(
        frame.location(),
        *frame.direction().coordinates(),
        *frame.x_direction().coordinates(),
        *frame.y_direction().coordinates(),
    )
    .map_err(|cause| Failure::Frame {
        chart,
        cause: Box::new(cause),
    })
}

fn minor([u, v]: &[Vector; 2], i: usize, j: usize) -> Scalar {
    u[i].multiply(&v[j]).subtract(&u[j].multiply(&v[i]))
}

pub(super) fn space_endpoints(source: SectionLineSource) -> Checked<[Vector; 2]> {
    let parameters = ranges(source.range)?;
    let placement = ExactPlacement::capture(source.placement)?;
    let origin = placement.point(exact_point(source.line.location(), 0)?);
    let direction = placement.vector(exact_vector(*source.line.direction().coordinates(), 1)?);
    if direction.iter().all(zero) {
        return Err(Failure::CandidateDomain);
    }
    Ok(parameters.map(|parameter| linear_sample(&origin, &direction, &parameter)))
}

pub(super) fn vertex(source: SectionVertexSource) -> Checked<Vector> {
    let placement = ExactPlacement::capture(source.placement)?;
    Ok(placement.point(exact_point(source.point, 0)?))
}

pub(super) fn pcurve_endpoints(source: SectionUvLine) -> Checked<[Uv; 2]> {
    let parameters = ranges(source.range)?;
    let origin = source.line.location();
    let direction = source.line.direction();
    let origin = [scalar(origin.x(), 0, 0)?, scalar(origin.y(), 0, 1)?];
    let direction = [scalar(direction.x(), 1, 0)?, scalar(direction.y(), 1, 1)?];
    if direction.iter().all(zero) {
        return Err(Failure::CandidateDomain);
    }
    Ok(parameters.map(|parameter| linear_sample(&origin, &direction, &parameter)))
}

fn ranges(range: [f64; 2]) -> Checked<[Scalar; 2]> {
    if !range.into_iter().all(f64::is_finite) || range[0] >= range[1] {
        return Err(Failure::CandidateDomain);
    }
    Ok(range.map(Scalar::from_finite))
}

fn scalar(value: f64, point: usize, axis: usize) -> Checked<Scalar> {
    if value.is_finite() {
        return Ok(Scalar::from_finite(value));
    }
    Err(crate::predicates::NonFiniteCoordinate { point, axis, value }.into())
}

fn linear_sample<const N: usize>(
    origin: &[Scalar; N],
    direction: &[Scalar; N],
    parameter: &Scalar,
) -> [Scalar; N] {
    std::array::from_fn(|i| origin[i].add(&direction[i].multiply(parameter)))
}

pub(super) fn interpolate<const N: usize>(
    a: &[Scalar; N],
    b: &[Scalar; N],
    parameter: &Scalar,
) -> [Scalar; N] {
    linear_sample(a, &sub(b, a), parameter)
}

pub(super) fn orientation(a: &Uv, b: &Uv, c: &Uv) -> Ordering {
    let [x, y] = sub(b, a);
    let [sx, sy] = sub(c, a);
    x.multiply(&sy).subtract(&y.multiply(&sx)).sign()
}

pub(super) fn sub<const N: usize>(a: &[Scalar; N], b: &[Scalar; N]) -> [Scalar; N] {
    std::array::from_fn(|i| a[i].subtract(&b[i]))
}

pub(super) fn zero(value: &Scalar) -> bool {
    value.sign() == Ordering::Equal
}

pub(super) fn bounds(value: &Scalar) -> Checked<[f64; 2]> {
    outward_binary64(value).ok_or(Failure::Unrepresentable)
}

pub(super) fn rounded(values: &Vector) -> Checked<[f64; 3]> {
    let [x, y, z] = values.each_ref().map(nearest_binary64);
    Ok([
        x.ok_or(Failure::Unrepresentable)?,
        y.ok_or(Failure::Unrepresentable)?,
        z.ok_or(Failure::Unrepresentable)?,
    ])
}

pub(super) fn distance_bound(left: &[Vector; 2], right: &[Vector; 2]) -> Checked<f64> {
    let squared = |index| {
        let d = sub(&left[index], &right[index]);
        dot(&d, &d)
    };
    let upper = squared(0).max(squared(1));
    if zero(&upper) {
        return Ok(0.);
    }
    outward_sqrt(&upper).ok_or(Failure::Unrepresentable)
}
