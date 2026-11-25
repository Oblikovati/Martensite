// SPDX-License-Identifier: GPL-2.0-only
//! Bernstein hulls of the ideal source at the same native parameter.
use super::{
    Arithmetic, HelixCylindricalSpanLaw, HelixTraceLaw, HelixTraceLead, Interval, Poles, ResultOf,
    algebra, polynomial,
};
use crate::predicates::roots::{
    certificate_arithmetic::{ExactRatio, bernstein_curve::restrict_enclosed, circular_constants},
    metric_bounds::cylindrical_profile::trigonometric,
};

pub(super) struct PreparedLaw {
    origin: [Interval; 3],
    basis: [[Interval; 3]; 3],
    first_radius: Interval,
    radial_change: Interval,
    axial: Axial,
    first_angle: Option<Interval>,
    angle: Interval,
    parameter_start: Option<Interval>,
    parameter_width: Interval,
    positive_rotation: bool,
}

enum Axial {
    LinearRise(Interval),
    Controls(Vec<Interval>),
}

impl PreparedLaw {
    pub(super) fn prepare(law: HelixTraceLaw, arithmetic: &mut Arithmetic<'_>) -> ResultOf<Self> {
        let [height, turns] = lead(law.lead, arithmetic)?;
        let pi = circular_constants::pi(arithmetic)?;
        let period = arithmetic.fraction(&pi, 2, 1)?;
        let initial = arithmetic.scalar(law.radii[0])?;
        let final_radius = arithmetic.scalar(law.radii[1])?;
        Ok(Self {
            origin: point(law.origin, arithmetic)?,
            basis: [
                point(law.basis[0], arithmetic)?,
                point(law.basis[1], arithmetic)?,
                point(law.basis[2], arithmetic)?,
            ],
            radial_change: arithmetic.subtract(&final_radius, &initial)?,
            first_radius: initial,
            axial: Axial::LinearRise(height),
            first_angle: None,
            angle: arithmetic.multiply(&period, &turns)?,
            parameter_start: None,
            parameter_width: arithmetic.scalar(law.parameter_end)?,
            positive_rotation: law.positive_rotation,
        })
    }

    pub(super) fn prepare_cylindrical(
        law: HelixCylindricalSpanLaw<'_>,
        arithmetic: &mut Arithmetic<'_>,
    ) -> ResultOf<Self> {
        let [first, last] = law.native_parameter;
        let parameter_start = arithmetic.scalar(first)?;
        let parameter_end = arithmetic.scalar(last)?;
        let turn = arithmetic.rational(&ExactRatio::from_dyadic(law.turns[0]))?;
        let width = law.turns[1].difference(law.turns[0]);
        let turns = arithmetic.rational(&ExactRatio::from_dyadic(&width))?;
        let period = circular_constants::pi(arithmetic)?;
        let period = arithmetic.fraction(&period, 2, 1)?;
        Ok(Self {
            origin: point(law.origin, arithmetic)?,
            basis: [
                point(law.basis[0], arithmetic)?,
                point(law.basis[1], arithmetic)?,
                point(law.basis[2], arithmetic)?,
            ],
            first_radius: arithmetic.scalar(law.radius)?,
            radial_change: arithmetic.zero(),
            axial: Axial::Controls(
                law.axial_controls
                    .iter()
                    .map(|coefficient| arithmetic.rational(&ExactRatio::from_dyadic(coefficient)))
                    .collect::<Result<_, _>>()?,
            ),
            first_angle: Some(arithmetic.multiply(&period, &turn)?),
            angle: arithmetic.multiply(&period, &turns)?,
            parameter_width: arithmetic.subtract(&parameter_end, &parameter_start)?,
            parameter_start: Some(parameter_start),
            positive_rotation: law.positive_rotation,
        })
    }

    pub(super) fn z_axis(&self) -> &[Interval; 3] {
        &self.basis[2]
    }

    pub(super) fn fraction(
        &self,
        parameter: &Interval,
        arithmetic: &mut Arithmetic<'_>,
    ) -> ResultOf<Interval> {
        let offset = match &self.parameter_start {
            Some(start) => arithmetic.subtract(parameter, start)?,
            None => parameter.clone(),
        };
        Ok(arithmetic.divide(&offset, &self.parameter_width)?)
    }

    pub(super) fn centered(
        &self,
        xyz: Vec<[Interval; 3]>,
        arithmetic: &mut Arithmetic<'_>,
    ) -> ResultOf<Poles> {
        let mut poles = std::array::from_fn(|_| Vec::with_capacity(xyz.len()));
        for point in xyz {
            for (axis, (value, origin)) in point.into_iter().zip(&self.origin).enumerate() {
                poles[axis].push(arithmetic.subtract(&value, origin)?);
            }
        }
        Ok(poles)
    }

    pub(super) fn ideal(
        &self,
        parameters: &[Interval; 2],
        arithmetic: &mut Arithmetic<'_>,
    ) -> ResultOf<Poles> {
        let mut phase = polynomial::scale(parameters, &self.angle, arithmetic)?;
        if let Some(first) = &self.first_angle {
            phase = algebra::sum(&phase, std::slice::from_ref(first), arithmetic)?;
        }
        let [cosine, sine] = trigonometric::enclose(&phase, arithmetic)?;
        let radii = polynomial::scale(parameters, &self.radial_change, arithmetic)?;
        let radii = algebra::sum(&radii, std::slice::from_ref(&self.first_radius), arithmetic)?;
        let signed_radius = if self.positive_rotation {
            radii.clone()
        } else {
            polynomial::scale(&radii, &arithmetic.scalar(-1.)?, arithmetic)?
        };
        let local = [
            polynomial::product(&radii, &cosine, arithmetic)?,
            polynomial::product(&signed_radius, &sine, arithmetic)?,
            self.axial.ideal(parameters, arithmetic)?,
        ];
        Ok([
            self.world(0, &local, arithmetic)?,
            self.world(1, &local, arithmetic)?,
            self.world(2, &local, arithmetic)?,
        ])
    }

    fn world(
        &self,
        axis: usize,
        local: &Poles,
        arithmetic: &mut Arithmetic<'_>,
    ) -> ResultOf<Vec<Interval>> {
        let x = polynomial::scale(&local[0], &self.basis[0][axis], arithmetic)?;
        let y = polynomial::scale(&local[1], &self.basis[1][axis], arithmetic)?;
        let z = polynomial::scale(&local[2], &self.basis[2][axis], arithmetic)?;
        let xy = algebra::sum(&x, &y, arithmetic)?;
        Ok(algebra::sum(&xy, &z, arithmetic)?)
    }
}

impl Axial {
    fn ideal(
        &self,
        [first, last]: &[Interval; 2],
        arithmetic: &mut Arithmetic<'_>,
    ) -> ResultOf<Vec<Interval>> {
        match self {
            Self::LinearRise(height) => Ok(polynomial::scale(
                &[first.clone(), last.clone()],
                height,
                arithmetic,
            )?),
            Self::Controls(poles) => Ok(restrict_enclosed(poles, [first, last], arithmetic)?),
        }
    }
}

pub(super) fn point(
    [x, y, z]: [f64; 3],
    arithmetic: &mut Arithmetic<'_>,
) -> ResultOf<[Interval; 3]> {
    Ok([
        arithmetic.scalar(x)?,
        arithmetic.scalar(y)?,
        arithmetic.scalar(z)?,
    ])
}

fn lead(source: HelixTraceLead, arithmetic: &mut Arithmetic<'_>) -> ResultOf<[Interval; 2]> {
    let [first, second] = source.values();
    let a = arithmetic.scalar(first)?;
    let b = arithmetic.scalar(second)?;
    match source {
        HelixTraceLead::PitchAndTurns(_) => Ok([arithmetic.multiply(&a, &b)?, b]),
        HelixTraceLead::HeightAndTurns(_) => Ok([a, b]),
        HelixTraceLead::PitchAndHeight(_) => Ok([b.clone(), arithmetic.divide(&b, &a)?]),
    }
}
