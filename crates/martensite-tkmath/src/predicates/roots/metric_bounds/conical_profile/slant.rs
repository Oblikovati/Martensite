// SPDX-License-Identifier: GPL-2.0-only
//! Bounded distance between source slant and a stored homogeneous rational fit.
use super::*;
use crate::predicates::roots::certificate_arithmetic::bernstein_bounds::lower;
use crate::predicates::roots::certificate_arithmetic::bernstein_bounds::product;
use crate::predicates::roots::metric_bounds::CylindricalProfileFailure;
use crate::predicates::roots::metric_bounds::cylindrical_profile::algebra;

pub(super) struct SlantRequest<'a> {
    pub(super) numerator: &'a [f64],
    pub(super) weights: &'a [f64],
}
#[derive(Debug)]
pub(super) enum SlantFailure {
    Shape {
        numerator: usize,
        weights: usize,
    },
    Nonfinite {
        role: &'static str,
        index: usize,
        bits: u64,
    },
    Nonpositive {
        role: &'static str,
        lower: DyadicBound,
    },
    Polynomial(CylindricalProfileFailure),
    Arithmetic(BoundedArithmeticFailure),
}
impl From<BoundedArithmeticFailure> for SlantFailure {
    fn from(cause: BoundedArithmeticFailure) -> Self {
        Self::Arithmetic(cause)
    }
}
impl std::fmt::Display for SlantFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Shape { numerator, weights } => write!(
                f,
                "numerator={numerator}, weights={weights}; expected equal counts in 1..=10"
            ),
            Self::Nonfinite { role, index, bits } => {
                write!(f, "{role}[{index}] bits={bits}; expected finite")
            }
            Self::Nonpositive { role, lower } => {
                write!(f, "{role} lower={lower:?}; expected strictly positive")
            }
            Self::Polynomial(cause) => write!(f, "polynomial: {cause:?}"),
            Self::Arithmetic(cause) => write!(f, "arithmetic: {cause:?}"),
        }
    }
}

pub(super) fn slant_from_captured(
    q: &[Interval],
    numerator: &[Interval],
    weights: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, SlantFailure> {
    let numerator_min = positive_lower("numerator", numerator, arithmetic)?;
    let weight_min = positive_lower("weight", weights, arithmetic)?;
    let denominator = arithmetic.multiply(&numerator_min, &weight_min)?;
    let residual = slant_residual(q, numerator, weights, arithmetic)?;
    Ok(arithmetic.divide(&magnitude_interval(&residual), &denominator)?)
}

pub(super) fn admit_slant(request: &SlantRequest<'_>) -> Result<(), SlantFailure> {
    if !(1..=10).contains(&request.numerator.len())
        || request.numerator.len() != request.weights.len()
    {
        return Err(SlantFailure::Shape {
            numerator: request.numerator.len(),
            weights: request.weights.len(),
        });
    }
    admit_finite_coefficients("numerator", request.numerator)?;
    admit_finite_coefficients("weights", request.weights)
}
pub(super) fn admit_finite_coefficients(
    role: &'static str,
    values: &[f64],
) -> Result<(), SlantFailure> {
    for (index, value) in values.iter().enumerate() {
        if !value.is_finite() {
            return Err(SlantFailure::Nonfinite {
                role,
                index,
                bits: value.to_bits(),
            });
        }
    }
    Ok(())
}

pub(super) fn capture(
    values: &[f64],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Vec<Interval>, SlantFailure> {
    Ok(values
        .iter()
        .map(|v| arithmetic.scalar(*v))
        .collect::<Checked<_>>()?)
}
pub(super) fn positive_lower(
    role: &'static str,
    values: &[Interval],
    arithmetic: &Arithmetic<'_>,
) -> Result<Interval, SlantFailure> {
    let value = lower(values);
    if value <= 0.into() {
        return Err(SlantFailure::Nonpositive {
            role,
            lower: DyadicBound::new(value, -i32::from(arithmetic.fractional_bits())),
        });
    }
    Ok(Interval {
        lower: value.clone(),
        upper: value,
    })
}
pub(super) fn slant_residual(
    q: &[Interval],
    numerator: &[Interval],
    weights: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Vec<Interval>, SlantFailure> {
    let a_squared = product(numerator, numerator, arithmetic)?;
    let w_squared = product(weights, weights, arithmetic)?;
    let reference = product(q, &w_squared, arithmetic)?;
    algebra::difference(&a_squared, &reference, arithmetic).map_err(SlantFailure::Polynomial)
}
