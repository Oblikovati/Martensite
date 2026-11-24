// SPDX-License-Identifier: GPL-2.0-only
//! Bernstein numerator recurrence for derivatives of c / sqrt(q).
use crate::predicates::roots::certificate_arithmetic::bernstein_bounds::difference;
use crate::predicates::roots::certificate_arithmetic::bernstein_bounds::lower;
use crate::predicates::roots::certificate_arithmetic::bernstein_bounds::magnitude;
use crate::predicates::roots::certificate_arithmetic::bernstein_bounds::product;
use crate::predicates::roots::certificate_arithmetic::bernstein_curve::derivative;
use crate::predicates::roots::certificate_arithmetic::bounded_dyadic::Arithmetic;
use crate::predicates::roots::certificate_arithmetic::bounded_dyadic::Interval;
use crate::predicates::roots::certificate_arithmetic::bounded_failure::BoundedArithmeticFailure;

use crate::predicates::roots::certificate_arithmetic::bounded_dyadic::PositiveSqrtFailure;
pub(super) type Bounded<T> = Result<T, PositiveSqrtFailure>;

#[derive(Debug)]
pub(super) enum SourceBoundFailure {
    Source(super::ConicalSourceFailure),
    Arithmetic(BoundedArithmeticFailure),
    SquareRoot(PositiveSqrtFailure),
    UnresolvedDistance(DyadicBound),
    InvalidLevel(u8),
    InvalidError(f64),
    InvalidPolicy {
        fractional_bits: u16,
        max_operations: u32,
    },
    ErrorLimit {
        width: DyadicBound,
        limit: f64,
    },
}
impl std::fmt::Display for SourceBoundFailure {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SquareRoot(cause) => write!(out, "positive square root: {cause:?}"),
            Self::Source(cause) => write!(out, "authored source: {cause:?}"),
            Self::Arithmetic(cause) => write!(out, "bounded derivative arithmetic: {cause:?}"),
            Self::InvalidLevel(level) => write!(out, "quadrature level={level}; expected <= 12"),
            Self::InvalidError(value) => {
                write!(out, "error limit={value}; expected finite and positive")
            }
            Self::InvalidPolicy {
                fractional_bits,
                max_operations,
            } => policy_message(out, *fractional_bits, *max_operations),
            Self::ErrorLimit { width, limit } => width_message(out, width, *limit),
            Self::UnresolvedDistance(lower) => {
                write!(out, "squared distance lower={lower}; expected > 0")
            }
        }
    }
}
pub(super) struct SourceBound {
    pub(super) fourth_derivative_upper: Interval,
}
pub(super) fn source_polynomials(
    poles: &[[f64; 2]],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(Vec<Interval>, Vec<Interval>), SourceBoundFailure> {
    let coordinates = super::admit_source(poles, arithmetic).map_err(SourceBoundFailure::Source)?;
    let q = super::squared_distance(&coordinates, arithmetic).map_err(SourceBoundFailure::from)?;
    let c = cross_polynomial(&coordinates, poles.len(), arithmetic)
        .map_err(SourceBoundFailure::from)?;
    Ok((q, c))
}
pub(super) fn bound_polynomials(
    q: &[Interval],
    c: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<SourceBound, SourceBoundFailure> {
    let q_lower = lower(q);
    let exponent = -i32::from(arithmetic.fractional_bits());
    let distance_squared_lower = DyadicBound::new(q_lower.clone(), exponent);
    if q_lower <= 0.into() {
        return Err(SourceBoundFailure::UnresolvedDistance(
            distance_squared_lower,
        ));
    }
    let numerator = fourth_numerator(q, c, arithmetic).map_err(SourceBoundFailure::from)?;
    let upper = derivative_upper(&q_lower, &magnitude(&numerator), arithmetic)
        .map_err(SourceBoundFailure::from)?;
    Ok(SourceBound {
        fourth_derivative_upper: upper,
    })
}
pub(super) fn derivative_upper(
    q_lower: &num_bigint::BigInt,
    magnitude: &num_bigint::BigInt,
    arithmetic: &mut Arithmetic<'_>,
) -> Bounded<Interval> {
    let q = Interval {
        lower: q_lower.clone(),
        upper: q_lower.clone(),
    };
    let square = arithmetic.multiply(&q, &q)?;
    let fourth = arithmetic.multiply(&square, &square)?;
    let root = arithmetic.positive_sqrt(&q)?;
    let denominator = arithmetic.multiply(&fourth, &root)?;
    let numerator = Interval {
        lower: 0.into(),
        upper: magnitude.clone(),
    };
    arithmetic
        .divide(&numerator, &denominator)
        .map_err(Into::into)
}
pub(super) fn cross_polynomial(
    coordinates: &[Vec<Interval>; 2],
    count: usize,
    arithmetic: &mut Arithmetic<'_>,
) -> Bounded<Vec<Interval>> {
    let x = super::elevate_constant(coordinates[0].clone(), count);
    let y = super::elevate_constant(coordinates[1].clone(), count);
    let dx = derivative(&x, arithmetic)?;
    let dy = derivative(&y, arithmetic)?;
    let left = product(&x, &dy, arithmetic)?;
    let right = product(&y, &dx, arithmetic)?;
    difference(&left, &right, arithmetic).map_err(Into::into)
}

// Inputs are admitted, nonempty Bernstein coefficients; q positivity is a separate obligation.
pub(super) fn fourth_numerator(
    q: &[Interval],
    c: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Bounded<Vec<Interval>> {
    let mut numerator = c.to_vec();
    for order in 0..4 {
        numerator = next_numerator(q, &numerator, order, arithmetic)?;
    }
    Ok(numerator)
}
pub(super) fn next_numerator(
    q: &[Interval],
    numerator: &[Interval],
    order: u64,
    arithmetic: &mut Arithmetic<'_>,
) -> Bounded<Vec<Interval>> {
    if q.len() == 1 {
        return product(q, &derivative(numerator, arithmetic)?, arithmetic).map_err(Into::into);
    }
    let dq = derivative(q, arithmetic)?;
    let right = product(&dq, numerator, arithmetic)?;
    let right = scaled_half(&right, 2 * order + 1, arithmetic)?;
    if numerator.len() == 1 {
        return negate(&right, arithmetic);
    }
    let left = product(q, &derivative(numerator, arithmetic)?, arithmetic)?;
    difference(&left, &right, arithmetic).map_err(Into::into)
}
pub(super) fn scaled_half(
    coefficients: &[Interval],
    numerator: u64,
    arithmetic: &mut Arithmetic<'_>,
) -> Bounded<Vec<Interval>> {
    coefficients
        .iter()
        .map(|value| arithmetic.fraction(value, numerator, 2).map_err(Into::into))
        .collect()
}
pub(super) fn negate(
    coefficients: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Bounded<Vec<Interval>> {
    let zero = arithmetic.zero();
    coefficients
        .iter()
        .map(|value| arithmetic.subtract(&zero, value).map_err(Into::into))
        .collect()
}

pub use super::quadrature::ConicalAuthoredContour;
pub use super::quadrature::ConicalAuthoredFlux;
pub use super::quadrature::ConicalAuthoredFluxError;
pub use super::quadrature::ConicalAuthoredFluxQuery;
pub use super::quadrature::ConicalAuthoredFluxStage;
pub use super::quadrature::certify_conical_authored_flux;

impl From<BoundedArithmeticFailure> for SourceBoundFailure {
    fn from(cause: BoundedArithmeticFailure) -> Self {
        Self::Arithmetic(cause)
    }
}
impl From<PositiveSqrtFailure> for SourceBoundFailure {
    fn from(cause: PositiveSqrtFailure) -> Self {
        match cause {
            PositiveSqrtFailure::Arithmetic(cause) => Self::Arithmetic(cause),
            other => Self::SquareRoot(other),
        }
    }
}

use crate::predicates::roots::DyadicBound;

fn policy_message(
    out: &mut std::fmt::Formatter<'_>,
    fractional_bits: u16,
    max_operations: u32,
) -> std::fmt::Result {
    write!(
        out,
        "precision={fractional_bits}, work={max_operations}; expected precision 8..=4096 and work <=1000000"
    )
}

fn width_message(
    out: &mut std::fmt::Formatter<'_>,
    width: &DyadicBound,
    limit: f64,
) -> std::fmt::Result {
    write!(
        out,
        "enclosure width={width}, limit={limit}; expected width <= limit"
    )
}
