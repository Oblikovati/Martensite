// SPDX-License-Identifier: GPL-2.0-only
use super::{AxisDirection, CoordinateAxis, LineAngle, ParallelDirection, metric};
use martensite_tkmath::gp::Point3;
use martensite_tksolve::{
    ad::{DualNumber, DualVector},
    residual::{ResidualError, ResidualSource, ResidualTolerance},
};

type Vector = DualVector<3>;
// Each evaluator has at most 12 input scalars and three result rows.
// The parallel path uses fewer than 100 elementary dual operations and copies, including
// both vector normalizations, the two basis vectors and the branch dot product. Charging
// 256 per existing per-lane structural unit also covers fixed-dimensional seeding and row
// collection, and the fixed-size outward trial guard for Angle. All bounds are on abstract
// scalar work, independent of allocator or libm time.
const SCALAR_WORK_PER_UNIT: usize = 256;

pub(super) fn numeric_budget(residual_work: usize) -> usize {
    residual_work / SCALAR_WORK_PER_UNIT
}
pub(super) fn work_bound(units: usize) -> usize {
    units.saturating_mul(SCALAR_WORK_PER_UNIT)
}

#[derive(Clone, Copy)]
pub(super) enum Equation {
    Coincident {
        scale: f64,
    },
    Length {
        scale: f64,
        target: f64,
    },
    Parallel {
        direction: ParallelDirection,
        reference: usize,
    },
    Axis(CoordinateAxis, AxisDirection),
    Plane([f64; 3]),
    Perpendicular,
    EqualLength {
        scale: f64,
    },
    Angle {
        target: LineAngle,
        minimum_sine: f64,
    },
}

impl Equation {
    pub(super) fn rows(self) -> usize {
        match self {
            Self::Coincident { .. } => 3,
            Self::Length { .. }
            | Self::Plane(_)
            | Self::Perpendicular
            | Self::EqualLength { .. }
            | Self::Angle { .. } => 1,
            Self::Parallel { .. } | Self::Axis(..) => 2,
        }
    }

    fn width(self) -> usize {
        match self {
            Self::Parallel { .. }
            | Self::Perpendicular
            | Self::EqualLength { .. }
            | Self::Angle { .. } => 12,
            _ => 6,
        }
    }
}

impl ResidualSource for Equation {
    fn tolerance_kind(&self) -> ResidualTolerance {
        ResidualTolerance::Unitless
    }

    fn evaluate(&self, v: &[DualNumber]) -> Result<Vec<DualNumber>, ResidualError> {
        let expected = self.width();
        if v.len() != expected {
            return Err(ResidualError::Domain(format!(
                "relation input width {}: expected {expected}",
                v.len()
            )));
        }
        let first = displacement(v, 0)?;
        match *self {
            Self::Coincident { scale } => Ok(first.scaled(scale)?.components().to_vec()),
            Self::Length { scale, target } => Ok(vec![
                checked_length(&first)?.shifted(-target)?.scaled(scale)?,
            ]),
            Self::Parallel {
                direction,
                reference,
            } => {
                let second = displacement(v, 6)?;
                parallel(unit(&first)?, unit(&second)?, direction, reference)
            }
            Self::Axis(axis, direction) => axis_rows(unit(&first)?, axis, direction),
            Self::Plane(normal) => Ok(vec![unit(&first)?.dot(&unit(&constant_vector(normal)?)?)?]),
            Self::Perpendicular => Ok(vec![unit(&first)?.dot(&unit(&displacement(v, 6)?)?)?]),
            Self::EqualLength { scale } => Ok(vec![
                checked_length(&first)?
                    .minus(&checked_length(&displacement(v, 6)?)?)?
                    .scaled(scale)?,
            ]),
            Self::Angle {
                target,
                minimum_sine,
            } => angle_row(v, &first, target, minimum_sine),
        }
    }
}

fn angle_row(
    v: &[DualNumber],
    first: &Vector,
    target: LineAngle,
    minimum_sine: f64,
) -> Result<Vec<DualNumber>, ResidualError> {
    let point = |i: usize| Point3::new(v[i].value(), v[i + 1].value(), v[i + 2].value());
    let points = [[point(0), point(3)], [point(6), point(9)]];
    if !metric::angle::nonsingular_coordinates(points, minimum_sine) {
        return Err(ResidualError::Domain(format!(
            "angle trial needs a line direction sine above {minimum_sine}"
        )));
    }
    let (sine, cosine) = target.sin_cos();
    let projection = unit(first)?.dot(&unit(&displacement(v, 6)?)?)?;
    Ok(vec![projection.shifted(-cosine)?.scaled(sine.recip())?])
}

fn displacement(v: &[DualNumber], start: usize) -> Result<Vector, ResidualError> {
    Ok(vector(v, start + 3)?.minus(&vector(v, start)?)?)
}

fn constant_vector([x, y, z]: [f64; 3]) -> Result<Vector, ResidualError> {
    Ok(Vector::new([
        DualNumber::constant(x)?,
        DualNumber::constant(y)?,
        DualNumber::constant(z)?,
    ])?)
}

fn axis_rows(
    vector: Vector,
    axis: CoordinateAxis,
    direction: AxisDirection,
) -> Result<Vec<DualNumber>, ResidualError> {
    let index = axis.index();
    let components = vector.components();
    let component = components[index].value();
    if !axis_branch(direction, component) {
        return Err(ResidualError::Domain(format!(
            "{axis:?} component {component}: expected the {direction:?} branch"
        )));
    }
    Ok(vec![
        components[(index + 1) % 3].clone(),
        components[(index + 2) % 3].clone(),
    ])
}

pub(super) fn axis_branch(direction: AxisDirection, value: f64) -> bool {
    match direction {
        AxisDirection::Positive => value > 0.,
        AxisDirection::Negative => value < 0.,
    }
}

fn vector(v: &[DualNumber], start: usize) -> Result<Vector, ResidualError> {
    Ok(Vector::new(std::array::from_fn(|axis| {
        v[start + axis].clone()
    }))?)
}

fn checked_length(v: &Vector) -> Result<DualNumber, ResidualError> {
    let length = v.length()?;
    // Ordinary sketch admission checks the exact length floor at publication.
    if length.value() <= 0. {
        return Err(ResidualError::Domain(format!(
            "trial length {}: expected a nonzero displacement",
            length.value()
        )));
    }
    Ok(length)
}

fn unit(v: &Vector) -> Result<Vector, ResidualError> {
    Ok(v.product(&DualNumber::constant(1.)?.quotient(&checked_length(v)?)?)?)
}

fn parallel(
    a: Vector,
    b: Vector,
    direction: ParallelDirection,
    reference: usize,
) -> Result<Vec<DualNumber>, ResidualError> {
    let dot = a.dot(&b)?.value();
    if !branch(direction, dot) {
        return Err(ResidualError::Domain(format!(
            "directed dot {dot}: expected the {direction:?} branch"
        )));
    }
    let projection = &a.components()[reference];
    if projection.value().abs() > 0.9 {
        return Err(ResidualError::Domain(format!(
            "parallel frame component {}: expected magnitude at most 0.9",
            projection.value()
        )));
    }
    let axis = Vector::new([
        DualNumber::constant(f64::from(reference == 0))?,
        DualNumber::constant(f64::from(reference == 1))?,
        DualNumber::constant(f64::from(reference == 2))?,
    ])?;
    let e1 = unit(&axis.minus(&a.product(projection)?)?)?;
    let e2 = a.cross(&e1)?;
    Ok(vec![e1.dot(&b)?, e2.dot(&b)?])
}

pub(super) fn branch(direction: ParallelDirection, dot: f64) -> bool {
    match direction {
        ParallelDirection::Same => dot > 0.,
        ParallelDirection::Opposite => dot < 0.,
    }
}
