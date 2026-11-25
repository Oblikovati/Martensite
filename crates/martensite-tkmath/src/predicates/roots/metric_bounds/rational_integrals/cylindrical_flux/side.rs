// SPDX-License-Identifier: GPL-2.0-only
//! Integrating depth first gives det(N0, N0'+N1', N1-N0)/(6 W^3).
use super::*;
use crate::predicates::roots::metric_bounds::arithmetic::{accumulate, difference, product};
use crate::predicates::roots::polynomial::IntegerPolynomial;
use num_bigint::BigInt;
type Homogeneous = coefficients::HomogeneousCurve;

#[inline(always)]
pub(super) fn integrate(
    span: CylindricalFluxSpan<'_>,
    options: RationalIntegralOptions,
    share: &ExactRatio,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<curve::CurveIntegral, RationalIntegralFailure> {
    integrate_boundaries(span.boundaries, options, share, cancelled)
}

pub(super) fn integrate_boundaries(
    boundaries: [MetricBezier<'_, 3>; 2],
    options: RationalIntegralOptions,
    share: &ExactRatio,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<curve::CurveIntegral, RationalIntegralFailure> {
    let (numerator, denominator) = prepared_boundaries(boundaries, cancelled)?;
    curve::quotient_integral(numerator, denominator, options, share, cancelled)
}

fn capture(
    curve: MetricBezier<'_, 3>,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Homogeneous, ResidualError> {
    let frame = MetricEmbedding {
        linear: [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
        origin: [0.; 3],
        scale: 1.,
    };
    coefficients::embedded(
        MetricCurve::Bezier(curve),
        frame,
        "ruled flux curve",
        cancelled,
    )
}

fn align(a: &mut Homogeneous, b: &mut Homogeneous) {
    // Capture removes only uniform powers of two; equal source weights fix their exact ratio.
    let bits = [a[3][0].bits(), b[3][0].bits()];
    for value in a.iter_mut().flatten() {
        *value <<= bits[1].saturating_sub(bits[0]);
    }
    for value in b.iter_mut().flatten() {
        *value <<= bits[0].saturating_sub(bits[1]);
    }
}

fn numerator(
    a: &Homogeneous,
    b: &Homogeneous,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Vec<BigInt>, RationalIntegralFailure> {
    let derivatives: [Vec<BigInt>; 3] = std::array::from_fn(|i| {
        let mut sum = IntegerPolynomial(a[i].clone()).derivative().0;
        accumulate(&mut sum, IntegerPolynomial(b[i].clone()).derivative().0);
        sum
    });
    let delta: [Vec<BigInt>; 3] = std::array::from_fn(|i| difference(b[i].clone(), a[i].clone()));
    let mut total = Vec::new();
    for axis in 0..3 {
        accumulate(
            &mut total,
            determinant_term(a, &derivatives, &delta, axis, cancelled)?,
        );
    }
    Ok(total)
}

fn determinant_term(
    a: &Homogeneous,
    derivatives: &[Vec<BigInt>; 3],
    delta: &[Vec<BigInt>; 3],
    axis: usize,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Vec<BigInt>, RationalIntegralFailure> {
    let (j, k) = ((axis + 1) % 3, (axis + 2) % 3);
    let positive = product(&derivatives[j], &delta[k], cancelled)?;
    let negative = product(&derivatives[k], &delta[j], cancelled)?;
    Ok(product(
        &a[axis],
        &difference(positive, negative),
        cancelled,
    )?)
}

fn prepared_boundaries(
    boundaries: [MetricBezier<'_, 3>; 2],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(Vec<BigInt>, Vec<BigInt>), RationalIntegralFailure> {
    let mut a = capture(boundaries[0], cancelled)?;
    let mut b = capture(boundaries[1], cancelled)?;
    align(&mut a, &mut b);
    let numerator = numerator(&a, &b, cancelled)?;
    let squared = product(&a[3], &a[3], cancelled)?;
    let denominator = product(&squared, &a[3], cancelled)?
        .into_iter()
        .map(|c| c * 3)
        .collect();
    Ok((numerator, denominator))
}
pub(super) fn integrate_boundaries_accounted(
    boundaries: [MetricBezier<'_, 3>; 2],
    options: RationalIntegralOptions,
    share: &ExactRatio,
    cancelled: &mut impl FnMut() -> bool,
) -> (Result<curve::CurveIntegral, RationalIntegralFailure>, u64) {
    let (numerator, denominator) = match prepared_boundaries(boundaries, cancelled) {
        Ok(values) => values,
        Err(cause) => return (Err(cause), 0),
    };
    curve::quotient_integral_accounted(numerator, denominator, options, share, cancelled)
}

#[cfg(test)]
mod accounting_tests;
