// SPDX-License-Identifier: GPL-2.0-only
//! Scalar derivative majorants for C*Q^(-1/2), retaining the existing Simpson remainder.
use super::*;
use crate::predicates::roots::certificate_arithmetic::bernstein_curve::derivative;

// D^(n-1)(Q*g' + Q'*g/2)=0 gives these doubled positive coefficients for g=Q^(-1/2).
const DOUBLED_LOG_COEFFICIENTS: [&[u64]; 4] = [&[1], &[3, 1], &[5, 4, 1], &[7, 9, 5, 1]];
const FOURTH_LEIBNIZ: [u64; 5] = [1, 4, 6, 4, 1];
pub(super) fn bound(
    q: &[Interval],
    c: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<SourceBound, SourceBoundFailure> {
    let minimum = lower(q);
    admit_local_radius(&minimum, arithmetic.fractional_bits())?;
    let q_min = Interval {
        lower: minimum.clone(),
        upper: minimum,
    };
    let q = derivative_magnitudes(q, arithmetic).map_err(SourceBoundFailure::from)?;
    let c = derivative_magnitudes(c, arithmetic).map_err(SourceBoundFailure::from)?;
    let g = inverse_majorants(&q_min, &q, arithmetic).map_err(SourceBoundFailure::from)?;
    let fourth_derivative_upper =
        fourth_majorant(&c, &g, arithmetic).map_err(SourceBoundFailure::from)?;
    Ok(SourceBound {
        fourth_derivative_upper,
    })
}
fn derivative_magnitudes(
    poles: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Bounded<Vec<Interval>> {
    let mut coefficients = poles.to_vec();
    let mut result = vec![nonnegative_magnitude(&coefficients)];
    for _ in 0..4 {
        coefficients = derivative(&coefficients, arithmetic)?;
        result.push(nonnegative_magnitude(&coefficients));
    }
    Ok(result)
}
fn nonnegative_magnitude(poles: &[Interval]) -> Interval {
    Interval {
        lower: 0.into(),
        upper: magnitude(poles),
    }
}
fn inverse_majorants(
    minimum: &Interval,
    q: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Bounded<Vec<Interval>> {
    let root = arithmetic.positive_sqrt(minimum)?;
    let one = arithmetic.scalar(1.)?;
    let mut g = vec![arithmetic.divide(&one, &root)?];
    for weights in DOUBLED_LOG_COEFFICIENTS {
        let next = next_inverse_majorant(minimum, q, &g, weights, arithmetic)?;
        g.push(next);
    }
    Ok(g)
}
fn next_inverse_majorant(
    minimum: &Interval,
    q: &[Interval],
    g: &[Interval],
    weights: &[u64],
    arithmetic: &mut Arithmetic<'_>,
) -> Bounded<Interval> {
    let mut numerator = arithmetic.zero();
    let order = weights.len();
    for (index, weight) in weights.iter().enumerate() {
        let product = arithmetic.multiply(&q[index + 1], &g[order - index - 1])?;
        let term = arithmetic.fraction(&product, *weight, 2)?;
        numerator = arithmetic.add(&numerator, &term)?;
    }
    arithmetic.divide(&numerator, minimum).map_err(Into::into)
}
fn fourth_majorant(
    c: &[Interval],
    g: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Bounded<Interval> {
    let mut result = arithmetic.zero();
    for (index, weight) in FOURTH_LEIBNIZ.iter().enumerate() {
        let product = arithmetic.multiply(&c[index], &g[4 - index])?;
        let term = arithmetic.fraction(&product, *weight, 1)?;
        result = arithmetic.add(&result, &term)?;
    }
    Ok(result)
}
