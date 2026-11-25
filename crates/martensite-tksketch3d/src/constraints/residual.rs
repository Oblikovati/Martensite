// SPDX-License-Identifier: GPL-2.0-only
use super::ParallelDirection;
use martensite_tksolve::{
    ad::{DualNumber, DualVector},
    residual::{ResidualError, ResidualSource, ResidualTolerance},
};

type Vector = DualVector<3>;
// Each evaluator has at most 12 input scalars and two projected direction equations.
// The parallel path uses fewer than 100 elementary dual operations and copies, including
// both vector normalizations, the two basis vectors and the branch dot product. Charging
// 256 per existing per-lane structural unit also covers fixed-dimensional seeding and row
// collection. All bounds are on abstract scalar work, independent of allocator or libm time.
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
}

impl Equation {
    pub(super) fn rows(self) -> usize {
        match self {
            Self::Coincident { .. } => 3,
            Self::Length { .. } => 1,
            Self::Parallel { .. } => 2,
        }
    }
}

impl ResidualSource for Equation {
    fn tolerance_kind(&self) -> ResidualTolerance {
        ResidualTolerance::Unitless
    }

    fn evaluate(&self, v: &[DualNumber]) -> Result<Vec<DualNumber>, ResidualError> {
        let expected = if matches!(self, Self::Parallel { .. }) {
            12
        } else {
            6
        };
        if v.len() != expected {
            return Err(ResidualError::Domain(format!(
                "relation input width {}: expected {expected}",
                v.len()
            )));
        }
        let first = vector(v, 3)?.minus(&vector(v, 0)?)?;
        match *self {
            Self::Coincident { scale } => Ok(first.scaled(scale)?.components().to_vec()),
            Self::Length { scale, target } => Ok(vec![
                checked_length(&first)?.shifted(-target)?.scaled(scale)?,
            ]),
            Self::Parallel {
                direction,
                reference,
            } => {
                let second = vector(v, 9)?.minus(&vector(v, 6)?)?;
                parallel(unit(&first)?, unit(&second)?, direction, reference)
            }
        }
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
