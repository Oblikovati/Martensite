// SPDX-License-Identifier: GPL-2.0-only
//! Exact interpolation of actual affine carriers and retained degree-one coefficients.
use super::{
    Checked, CylinderProjectionError as Failure,
    affine::{Ratio, Vector},
    verify::{MAX_PROJECTION_CURVE_POINTS, NativeProjectionCurve},
};
use crate::predicates::roots::metric_bounds::placed_plane::ExactPlacement;

pub(super) struct Net<const N: usize> {
    knots: Vec<Ratio>,
    vertices: Vec<[Ratio; N]>,
}

impl<const N: usize> Net<N> {
    pub fn capture(curve: &NativeProjectionCurve<N>) -> Checked<Self> {
        match curve {
            NativeProjectionCurve::Line { origin, direction } => Ok(Self {
                knots: Vec::new(),
                vertices: vec![finite(origin)?, finite(direction)?],
            }),
            NativeProjectionCurve::DegreeOne { knots, poles } => {
                if knots.len() > MAX_PROJECTION_CURVE_POINTS
                    || poles.len() > MAX_PROJECTION_CURVE_POINTS
                {
                    return Err(Failure::WorkLimit);
                }
                if knots.len() < 2
                    || knots.len() != poles.len()
                    || knots.iter().any(|knot| !knot.is_finite())
                    || knots.windows(2).any(|pair| pair[0] >= pair[1])
                {
                    return Err(Failure::Candidate);
                }
                let knots = knots.iter().copied().map(Ratio::from_finite).collect();
                let vertices = poles.iter().map(finite).collect::<Checked<_>>()?;
                Ok(Self { knots, vertices })
            }
        }
    }

    pub fn add_cuts(&self, cuts: &mut Vec<Ratio>, [lo, hi]: &[Ratio; 2]) -> Checked<()> {
        let Some(first) = self.knots.first() else {
            return Ok(());
        };
        let last = self.knots.last().ok_or(Failure::Candidate)?;
        if lo < first || hi > last {
            return Err(Failure::Candidate);
        }
        cuts.extend(
            self.knots
                .iter()
                .filter(|knot| *knot > lo && *knot < hi)
                .cloned(),
        );
        Ok(())
    }

    pub fn at(&self, parameter: &Ratio) -> Checked<[Ratio; N]> {
        if self.knots.is_empty() {
            let [origin, direction] = self.vertices.as_slice() else {
                return Err(Failure::Candidate);
            };
            return Ok(std::array::from_fn(|axis| {
                origin[axis].add(&direction[axis].multiply(parameter))
            }));
        }
        let index = self.knots.partition_point(|knot| knot < parameter);
        if self.knots.get(index) == Some(parameter) {
            return Ok(self.vertices[index].clone());
        }
        if index == 0 || index >= self.knots.len() {
            return Err(Failure::Candidate);
        }
        let lo = index - 1;
        let u = parameter
            .subtract(&self.knots[lo])
            .divide(&self.knots[index].subtract(&self.knots[lo]));
        let [a, b] = [&self.vertices[lo], &self.vertices[index]];
        Ok(std::array::from_fn(|axis| {
            a[axis].add(&b[axis].subtract(&a[axis]).multiply(&u))
        }))
    }
}

impl Net<3> {
    pub fn place(&mut self, placement: &ExactPlacement) {
        if self.knots.is_empty() {
            self.vertices[0] = placement.point(self.vertices[0].clone());
            self.vertices[1] = placement.vector(self.vertices[1].clone());
        } else {
            for position in &mut self.vertices {
                *position = placement.point(position.clone());
            }
        }
    }
}

pub(super) fn finite<const N: usize>(values: &[f64; N]) -> Checked<[Ratio; N]> {
    if values.iter().any(|value| !value.is_finite()) {
        return Err(Failure::Candidate);
    }
    Ok(values.map(Ratio::from_finite))
}

pub(super) struct NativeBreakpoint {
    pub parameter: Ratio,
    pub position: Vector,
    /// Original source parameter and signed travel after exact native chart transport.
    pub prism: [Ratio; 2],
    pub target: [Ratio; 2],
    pub second_target: Option<[Ratio; 2]>,
}
