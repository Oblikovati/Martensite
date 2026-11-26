// SPDX-License-Identifier: GPL-2.0-only
use super::{
    ArcBranch, AxisDirection, CoordinateAxis, LineAngle, ParallelDirection, metric,
    on_face::AttachmentEquation,
};
use martensite_tkmath::{gp::Point3, predicates::roots::CubicFitTangent};
use martensite_tksolve::{
    ad::{DualNumber, DualVector},
    residual::{ResidualError, ResidualSource, ResidualTolerance},
};

mod arc;
mod continuity;
mod frame;
pub(super) use frame::FrameReference;

type Vector = DualVector<3>;
// Each evaluator has at most 18 input scalars and four result rows.
// The parallel path uses fewer than 100 elementary dual operations and copies, including
// both vector normalizations, the two basis vectors and the branch dot product. Colinear adds
// two length projections. G2 adds fixed-size exact endpoint stencils and two curvature
// projections. Equal Radius evaluates two sagitta stencils. The fit tangent stencil uses
// three fixed affine differences per axis and two normalized tangent rows.
// Charging 256 per structural unit covers these operations, fixed-size seeding,
// row collection and Angle's outward trial guard. The charge counts abstract scalar work,
// independent of allocator or libm time.
const SCALAR_WORK_PER_UNIT: usize = 256;

pub(super) fn numeric_budget(residual_work: usize) -> usize {
    residual_work / SCALAR_WORK_PER_UNIT
}
pub(super) fn work_bound(units: usize) -> usize {
    units.saturating_mul(SCALAR_WORK_PER_UNIT)
}

#[derive(Clone, Copy)]
pub(super) enum Equation {
    SurfaceAttachment(AttachmentEquation),
    Coincident {
        scale: f64,
    },
    Length {
        scale: f64,
        target: f64,
    },
    ArcIntrinsic {
        branch: ArcBranch,
        scale: f64,
    },
    Radius {
        branch: ArcBranch,
        target: f64,
        scale: f64,
    },
    EqualRadius {
        branches: [ArcBranch; 2],
        scale: f64,
    },
    PointOnLine {
        scale: f64,
        reference: FrameReference,
    },
    Colinear {
        direction: ParallelDirection,
        reference: FrameReference,
        scale: f64,
        independent_offset: bool,
    },
    Parallel {
        direction: ParallelDirection,
        reference: FrameReference,
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
    FitTangent {
        reference: FrameReference,
        tangent: CubicFitTangent,
        weights: [f64; 4],
    },
    Continuity {
        reference: FrameReference,
        cubic_reference: bool,
        curvature_scale: Option<f64>,
    },
}

impl Equation {
    pub(super) fn refreshed_chart(self, direction: [f64; 3]) -> Option<Self> {
        match self {
            Self::FitTangent {
                reference,
                tangent,
                weights,
            } if reference.needs_refresh(direction) => Some(Self::FitTangent {
                reference: FrameReference::from_direction(direction),
                tangent,
                weights,
            }),
            Self::PointOnLine { reference, scale } if reference.needs_refresh(direction) => {
                Some(Self::PointOnLine {
                    scale,
                    reference: FrameReference::from_direction(direction),
                })
            }
            Self::Colinear {
                reference,
                direction: branch,
                scale,
                independent_offset,
            } if reference.needs_refresh(direction) => Some(Self::Colinear {
                direction: branch,
                scale,
                independent_offset,
                reference: FrameReference::from_direction(direction),
            }),
            Self::Continuity {
                reference,
                cubic_reference,
                curvature_scale,
            } if reference.needs_refresh(direction) => Some(Self::Continuity {
                reference: FrameReference::from_direction(direction),
                cubic_reference,
                curvature_scale,
            }),
            _ => None,
        }
    }

    pub(super) fn rows(self) -> usize {
        match self {
            Self::Coincident { .. } | Self::SurfaceAttachment(_) => 3,
            Self::Length { .. }
            | Self::Plane(_)
            | Self::ArcIntrinsic { .. }
            | Self::Radius { .. }
            | Self::EqualRadius { .. }
            | Self::Perpendicular
            | Self::EqualLength { .. }
            | Self::Angle { .. } => 1,
            Self::Colinear {
                independent_offset: true,
                ..
            }
            | Self::Continuity {
                curvature_scale: Some(_),
                ..
            } => 4,
            Self::Colinear { .. }
            | Self::PointOnLine { .. }
            | Self::Parallel { .. }
            | Self::Axis(..)
            | Self::FitTangent { .. }
            | Self::Continuity { .. } => 2,
        }
    }

    fn width(self) -> usize {
        match self {
            Self::EqualRadius { .. } => 18,
            Self::FitTangent { .. } => 15,
            Self::Continuity {
                cubic_reference: true,
                curvature_scale: Some(_),
                ..
            } => 15,
            Self::Parallel { .. }
            | Self::Colinear { .. }
            | Self::Perpendicular
            | Self::EqualLength { .. }
            | Self::Angle { .. }
            | Self::Continuity {
                curvature_scale: Some(_),
                ..
            } => 12,
            Self::ArcIntrinsic { .. }
            | Self::Radius { .. }
            | Self::PointOnLine { .. }
            | Self::Continuity { .. } => 9,
            Self::SurfaceAttachment(_) => 5,
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
        let first = || displacement(v, 0);
        match *self {
            Self::SurfaceAttachment(source) => source.evaluate(v),
            Self::Coincident { scale } => Ok(first()?.scaled(scale)?.components().to_vec()),
            Self::Length { scale, target } => Ok(vec![
                checked_length(&first()?)?.shifted(-target)?.scaled(scale)?,
            ]),
            Self::ArcIntrinsic { branch, scale } => {
                let chart = arc::Sagitta::new(v, 0, first()?, branch)?;
                Ok(vec![chart.intrinsic()?.scaled(scale)?])
            }
            Self::Radius {
                branch,
                target,
                scale,
            } => {
                let chart = arc::Sagitta::new(v, 0, first()?, branch)?;
                Ok(vec![chart.radius()?.shifted(-target)?.scaled(scale)?])
            }
            Self::EqualRadius {
                branches: [a, b],
                scale,
            } => {
                let first = arc::Sagitta::new(v, 0, first()?, a)?.radius()?;
                let second = arc::Sagitta::new(v, 9, displacement(v, 9)?, b)?.radius()?;
                Ok(vec![first.minus(&second)?.scaled(scale)?])
            }
            Self::PointOnLine { scale, reference } => {
                let basis = reference.basis(&unit(&first()?)?, "point-on-line")?;
                let offset = vector(v, 6)?.minus(&vector(v, 0)?)?;
                Ok(length_rows(&basis, &offset, scale)?.into())
            }
            Self::Colinear {
                direction,
                reference,
                scale,
                independent_offset,
            } => colinear_rows(
                v,
                &first()?,
                direction,
                reference,
                scale,
                independent_offset,
            ),
            Self::Parallel {
                direction,
                reference,
            } => {
                let a = unit(&first()?)?;
                let b = unit(&displacement(v, 6)?)?;
                directed(&a, &b, direction)?;
                Ok(components(&reference.basis(&a, "parallel")?, &b)?.into())
            }
            Self::Axis(axis, direction) => axis_rows(unit(&first()?)?, axis, direction),
            Self::Plane(normal) => Ok(vec![
                unit(&first()?)?.dot(&unit(&constant_vector(normal)?)?)?,
            ]),
            Self::Perpendicular => Ok(vec![unit(&first()?)?.dot(&unit(&displacement(v, 6)?)?)?]),
            Self::EqualLength { scale } => Ok(vec![
                checked_length(&first()?)?
                    .minus(&checked_length(&displacement(v, 6)?)?)?
                    .scaled(scale)?,
            ]),
            Self::Angle {
                target,
                minimum_sine,
            } => angle_row(v, &first()?, target, minimum_sine),
            Self::FitTangent {
                reference,
                tangent,
                weights,
            } => continuity::evaluate_fit(v, &first()?, reference, tangent, weights),
            Self::Continuity {
                reference,
                cubic_reference,
                curvature_scale,
            } => continuity::evaluate(v, &first()?, reference, cubic_reference, curvature_scale),
        }
    }
}

fn colinear_rows(
    v: &[DualNumber],
    first: &Vector,
    direction: ParallelDirection,
    reference: FrameReference,
    scale: f64,
    independent_offset: bool,
) -> Result<Vec<DualNumber>, ResidualError> {
    let a = unit(first)?;
    let b = unit(&displacement(v, 6)?)?;
    directed(&a, &b, direction)?;
    let basis = reference.basis(&a, "colinear")?;
    let mut rows = components(&basis, &b)?.to_vec();
    if independent_offset {
        let offset = vector(v, 6)?.minus(&vector(v, 0)?)?;
        rows.extend(length_rows(&basis, &offset, scale)?);
    }
    Ok(rows)
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

fn directed(a: &Vector, b: &Vector, direction: ParallelDirection) -> Result<(), ResidualError> {
    let dot = a.dot(b)?.value();
    if !branch(direction, dot) {
        return Err(ResidualError::Domain(format!(
            "directed dot {dot}: expected the {direction:?} branch"
        )));
    }
    Ok(())
}

fn components(
    [first, second]: &[Vector; 2],
    value: &Vector,
) -> Result<[DualNumber; 2], ResidualError> {
    Ok([first.dot(value)?, second.dot(value)?])
}

fn length_rows(
    basis: &[Vector; 2],
    offset: &Vector,
    scale: f64,
) -> Result<[DualNumber; 2], ResidualError> {
    let [first, second] = components(basis, offset)?;
    Ok([first.scaled(scale)?, second.scaled(scale)?])
}

pub(super) fn branch(direction: ParallelDirection, dot: f64) -> bool {
    match direction {
        ParallelDirection::Same => dot > 0.,
        ParallelDirection::Opposite => dot < 0.,
    }
}

#[cfg(test)]
mod tests;
