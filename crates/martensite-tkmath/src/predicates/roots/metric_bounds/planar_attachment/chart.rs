// SPDX-License-Identifier: GPL-2.0-only
//! Exact local and world charts. Inversion uses the placed Gram matrix.
use super::{ExactRatio as R, PlanarAttachmentChart, PlanarAttachmentError as Error};
use crate::gp::Axis3;
use crate::predicates::roots::certificate_arithmetic::rounding::{
    nearest_binary64, outward_binary64,
};
use crate::predicates::roots::metric_bounds::placed_plane::{
    ExactPlacement, ExactVector, cross, dot, exact_point, exact_vector,
};
use std::cmp::Ordering;

pub(super) type Uv = [R; 2];
pub(super) type Vector = ExactVector;

#[derive(Clone, Debug)]
pub(super) struct Chart {
    origin: Vector,
    axes: [Vector; 2],
    inverse: [Vector; 2],
}

pub(super) struct Projection {
    pub uv: Uv,
    pub world: Vector,
    pub distance_squared: R,
}

impl Chart {
    pub(super) fn local(frame: Axis3) -> Result<Self, Error> {
        Self::placed(
            frame,
            [[1., 0., 0., 0.], [0., 1., 0., 0.], [0., 0., 1., 0.]],
        )
    }

    pub(super) fn placed(frame: Axis3, matrix: [[f64; 4]; 3]) -> Result<Self, Error> {
        let placement = ExactPlacement::capture(matrix)?;
        let origin = placement.point(exact_point(frame.location(), 0)?);
        let u = placement.vector(exact_vector(*frame.x_direction().coordinates(), 1)?);
        let v = placement.vector(exact_vector(*frame.y_direction().coordinates(), 2)?);
        let inverse = inverse(&u, &v)?;
        Ok(Self {
            origin,
            axes: [u, v],
            inverse,
        })
    }

    pub(super) fn signed_support(&self, endpoints: [&Vector; 2]) -> [R; 2] {
        let normal = cross(&self.axes[0], &self.axes[1]);
        endpoints.map(|point| {
            let relative = std::array::from_fn(|i| point[i].subtract(&self.origin[i]));
            dot(&normal, &relative)
        })
    }

    pub(super) fn lift(&self, [u, v]: &Uv) -> Vector {
        std::array::from_fn(|i| {
            self.origin[i]
                .add(&self.axes[0][i].multiply(u))
                .add(&self.axes[1][i].multiply(v))
        })
    }

    pub(super) fn pullback(&self, point: &Vector) -> Uv {
        let relative = std::array::from_fn(|i| point[i].subtract(&self.origin[i]));
        self.inverse.each_ref().map(|row| dot(row, &relative))
    }

    pub(super) fn project(&self, point: [f64; 3]) -> Result<Projection, Error> {
        let point = exact_vector(point, 3)?;
        let uv = self.pullback(&point);
        Ok(self.projection(uv, point))
    }

    pub(super) fn proposed(&self, uv: [f64; 2], point: [f64; 3]) -> Result<Projection, Error> {
        Ok(self.projection(exact_uv(uv)?, exact_vector(point, 3)?))
    }

    pub(super) fn solver_chart(&self, uv: [f64; 2]) -> Result<PlanarAttachmentChart, Error> {
        let world = rounded(&self.lift(&exact_uv(uv)?))?;
        let [du, dv] = &self.axes;
        Ok(PlanarAttachmentChart {
            uv,
            world,
            tangents: [rounded(du)?, rounded(dv)?],
        })
    }

    fn projection(&self, uv: Uv, point: Vector) -> Projection {
        let world = self.lift(&uv);
        let delta = std::array::from_fn(|i| world[i].subtract(&point[i]));
        Projection {
            uv,
            world,
            distance_squared: dot(&delta, &delta),
        }
    }
}

fn exact_uv([u, v]: [f64; 2]) -> Result<Uv, Error> {
    let [u, v, _] = exact_vector([u, v, 0.], 4)?;
    Ok([u, v])
}

fn rounded(value: &Vector) -> Result<[f64; 3], Error> {
    let mut nominal = [0.; 3];
    for (result, exact) in nominal.iter_mut().zip(value) {
        *result = nearest_binary64(exact).ok_or(Error::Unrepresentable)?;
    }
    Ok(nominal)
}

pub(super) fn retained_frame(frame: Axis3) -> Result<Axis3, Error> {
    Axis3::from_stored_coefficients(
        frame.location(),
        *frame.direction().coordinates(),
        *frame.x_direction().coordinates(),
        *frame.y_direction().coordinates(),
    )
    .map_err(|_| Error::Frame)
}

fn inverse(u: &Vector, v: &Vector) -> Result<[Vector; 2], Error> {
    let (uu, uv, vv) = (dot(u, u), dot(u, v), dot(v, v));
    let det = uu.multiply(&vv).subtract(&uv.multiply(&uv));
    if det.sign() != Ordering::Greater {
        return Err(Error::DegenerateChart);
    }
    let row = |x: &Vector, y: &Vector, norm: &R| {
        std::array::from_fn(|i| {
            x[i].multiply(norm)
                .subtract(&y[i].multiply(&uv))
                .divide(&det)
        })
    };
    Ok([row(u, v, &vv), row(v, u, &uu)])
}

pub(super) fn enclose<const N: usize>(values: &[R; N]) -> Result<[[f64; 2]; N], Error> {
    let mut bounds = [[0.; 2]; N];
    for (bound, value) in bounds.iter_mut().zip(values) {
        *bound = outward_binary64(value).ok_or(Error::Unrepresentable)?;
    }
    Ok(bounds)
}
