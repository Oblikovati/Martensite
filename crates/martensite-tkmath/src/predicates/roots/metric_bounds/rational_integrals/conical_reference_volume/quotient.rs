// SPDX-License-Identifier: GPL-2.0-only
//! Raw exact monomial coefficients preserve the scales of q and N/W; no primitive-content reduction.
use super::*;
use crate::predicates::roots::metric_bounds::arithmetic::product;
use crate::predicates::roots::{
    bernstein,
    polynomial::{IntegerPolynomial, integer_coefficients},
};
use num_bigint::BigInt;
pub(super) struct Quotients {
    pub(super) numerators: [Vec<BigInt>; 2],
    pub(super) denominator: Vec<BigInt>,
}
pub(super) fn capture(
    span: ConicalReferenceVolumeSpan<'_>,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Quotients, RationalIntegralFailure> {
    let q = power(span.angular_fit, cancelled)?;
    let unit = vec![BigInt::from(1) << 1074];
    let [q, unit] = normalize([q, unit]);
    let n = power(span.slant_numerator, cancelled)?;
    let w = power(span.slant_weights, cancelled)?;
    let [n, w] = normalize([n, w]);
    assemble(q, unit[0].clone(), n, w, cancelled)
}
fn power(
    values: &[f64],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Vec<BigInt>, RationalIntegralFailure> {
    Ok(bernstein::power_coefficients(
        integer_coefficients(values)?,
        cancelled,
    )?)
}
fn normalize(mut polynomials: [Vec<BigInt>; 2]) -> [Vec<BigInt>; 2] {
    let zeros = polynomials
        .iter()
        .flatten()
        .filter_map(BigInt::trailing_zeros)
        .min()
        .unwrap_or(0);
    for coefficient in polynomials.iter_mut().flatten() {
        *coefficient >>= zeros;
    }
    polynomials
}
fn assemble(
    q: Vec<BigInt>,
    unit: BigInt,
    n: Vec<BigInt>,
    w: Vec<BigInt>,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Quotients, RationalIntegralFailure> {
    let dq = IntegerPolynomial(q.clone()).derivative().0;
    let mut angular = product(&q, &q, cancelled)?;
    angular[0] += &unit * &unit;
    let w2 = product(&w, &w, cancelled)?;
    let denominator = product(&w2, &angular, cancelled)?;
    let n2 = product(&n, &n, cancelled)?;
    let nw = product(&n, &w, cancelled)?;
    let squared = numerator(&n2, &dq, &unit, cancelled)?;
    let linear = numerator(&nw, &dq, &unit, cancelled)?;
    Ok(Quotients {
        numerators: [squared, linear],
        denominator,
    })
}
fn numerator(
    slant: &[BigInt],
    dq: &[BigInt],
    unit: &BigInt,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Vec<BigInt>, RationalIntegralFailure> {
    // quotient_integral integrates HALF its quotient: 2 * unit explicitly restores the full integrand.
    Ok(product(slant, dq, cancelled)?
        .into_iter()
        .map(|value| value * unit * 2)
        .collect())
}
