// SPDX-License-Identifier: GPL-2.0-only
//! Pseudo-remainders retain leading factors: their zero fibers need original-system checks.
use super::super::{RootIsolationError, error::poll};
use super::{equations::BivariateEquation, polynomial::ExactCoefficientPolynomial};

pub(super) struct FiberRowReduction {
    pub linear: Option<BivariateEquation>,
    pub leading_factors: Vec<ExactCoefficientPolynomial>,
}
pub(super) fn linear_row(
    system: &[BivariateEquation; 2],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<FiberRowReduction, RootIsolationError> {
    let mut rows = system.clone();
    let mut leading_factors = Vec::new();
    poll(cancelled)?;
    while rows
        .iter()
        .all(|row| row.degree_u().is_some_and(|degree| degree > 1))
    {
        reduce_rows(&mut rows, &mut leading_factors, cancelled)?;
        poll(cancelled)?;
    }
    let linear = rows.iter().find(|row| row.degree_u() == Some(1)).cloned();
    Ok(FiberRowReduction {
        linear,
        leading_factors,
    })
}
fn reduce_rows(
    rows: &mut [BivariateEquation; 2],
    factors: &mut Vec<ExactCoefficientPolynomial>,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(), RootIsolationError> {
    if rows[0].degree_u() < rows[1].degree_u() {
        rows.swap(0, 1);
    }
    rows[0] = pseudo_remainder(&rows[0], &rows[1], factors, cancelled)?;
    rows.swap(0, 1);
    Ok(())
}
fn pseudo_remainder(
    dividend: &BivariateEquation,
    divisor: &BivariateEquation,
    factors: &mut Vec<ExactCoefficientPolynomial>,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<BivariateEquation, RootIsolationError> {
    let degree = divisor
        .degree_u()
        .expect("nonzero divisor established by row reduction");
    let mut remainder = dividend.clone();
    while remainder.degree_u().is_some_and(|power| power >= degree) {
        poll(cancelled)?;
        let (scale_r, scale_d) = elimination_multipliers(&remainder, divisor, degree, cancelled)?;
        factors.push(scale_r.clone());
        remainder =
            eliminate_leading(&remainder, divisor, degree, (&scale_r, &scale_d), cancelled)?;
    }
    Ok(remainder)
}
/// Cancel any non-constant polynomial factor shared by the two leading `u`-coefficients
/// so pseudo-remainder stays in the ideal without injecting spurious common roots of
/// `alpha` and `beta` (#619). Constant leading factors keep their exact integer scale.
fn elimination_multipliers(
    remainder: &BivariateEquation,
    divisor: &BivariateEquation,
    degree: usize,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(ExactCoefficientPolynomial, ExactCoefficientPolynomial), RootIsolationError> {
    let lead_r = &remainder.0[remainder.0.len() - 1];
    let lead_d = &divisor.0[degree];
    let Some(common) = lead_r.nonconstant_primitive_gcd(lead_d, cancelled)? else {
        return Ok((lead_d.clone(), lead_r.clone()));
    };
    let reduced_d = lead_d.exact_divide(&common, cancelled)?;
    let reduced_r = lead_r.exact_divide(&common, cancelled)?;
    Ok(match (reduced_d, reduced_r) {
        (Some(scale_r), Some(scale_d)) => (scale_r, scale_d),
        _ => (lead_d.clone(), lead_r.clone()),
    })
}
fn eliminate_leading(
    remainder: &BivariateEquation,
    divisor: &BivariateEquation,
    degree: usize,
    (scale_r, scale_d): (&ExactCoefficientPolynomial, &ExactCoefficientPolynomial),
    cancelled: &mut impl FnMut() -> bool,
) -> Result<BivariateEquation, RootIsolationError> {
    let power = remainder.0.len() - 1;
    let offset = power - degree;
    let mut coefficients = Vec::with_capacity(remainder.0.len());
    for (index, coefficient) in remainder.0.iter().enumerate() {
        let scaled = coefficient.multiply(scale_r, cancelled)?;
        let term = shifted_divisor_term(divisor, index, offset, scale_d, cancelled)?;
        coefficients.push(scaled.subtract(&term));
    }
    Ok(BivariateEquation::new(coefficients))
}
fn shifted_divisor_term(
    divisor: &BivariateEquation,
    index: usize,
    offset: usize,
    factor: &ExactCoefficientPolynomial,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<ExactCoefficientPolynomial, RootIsolationError> {
    let Some(power) = index
        .checked_sub(offset)
        .and_then(|power| divisor.0.get(power))
    else {
        return Ok(ExactCoefficientPolynomial::new(Vec::new()));
    };
    power.multiply(factor, cancelled)
}
