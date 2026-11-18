// SPDX-License-Identifier: GPL-2.0-only
//! Exact affine restriction and tensor Bernstein conversion; only strict coefficient signs exclude.
use super::super::equations::BivariateEquation;
use super::*;
use num_bigint::BigInt;
type Matrix = Vec<Vec<ExactRatio>>;
#[derive(Debug)]
pub(super) struct BernsteinWitness {
    pub(super) degrees: [usize; 2],
    pub(super) values: Matrix,
    pub(super) sign: Ordering,
}
pub(super) fn certify(
    equation: &BivariateEquation,
    rectangle: [[f64; 2]; 2],
    limit: usize,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Option<BernsteinWitness>, RootIsolationError> {
    let Some(degrees) = degrees(equation) else {
        return Ok(None);
    };
    if degrees.iter().any(|degree| *degree > limit) {
        return Ok(None);
    }
    let monomial = restricted(equation, degrees, rectangle, cancelled)?;
    let values = bernstein(monomial, degrees, cancelled)?;
    let Some(sign) = strict_sign(&values, cancelled)? else {
        return Ok(None);
    };
    Ok(Some(BernsteinWitness {
        degrees,
        values,
        sign,
    }))
}
fn degrees(equation: &BivariateEquation) -> Option<[usize; 2]> {
    let u = equation.0.len().checked_sub(1)?;
    let t = equation.0.iter().filter_map(|p| p.degree()).max()?;
    Some([t, u])
}
fn restricted(
    equation: &BivariateEquation,
    degrees: [usize; 2],
    rectangle: [[f64; 2]; 2],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Matrix, RootIsolationError> {
    let mut columns = Vec::new();
    for row in &equation.0 {
        let values = integer_ratios(&row.0);
        let mut values = restrict(&values, rectangle[0], cancelled)?;
        values.resize(degrees[0] + 1, zero());
        columns.push(values);
    }
    let mut result = Vec::new();
    for t in 0..=degrees[0] {
        let row = columns
            .iter()
            .map(|column| column[t].clone())
            .collect::<Vec<_>>();
        result.push(restrict(&row, rectangle[1], cancelled)?);
    }
    Ok(result)
}
fn restrict(
    powers: &[ExactRatio],
    interval: [f64; 2],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Vec<ExactRatio>, RootIsolationError> {
    let origin = ExactRatio::from_finite(interval[0]);
    let width = ExactRatio::from_finite(interval[1]).subtract(&origin);
    let mut result = Vec::new();
    for coefficient in powers.iter().rev() {
        poll(cancelled)?;
        result = horner(&result, &origin, &width, coefficient, cancelled)?;
    }
    if result.is_empty() {
        result.push(zero());
    }
    Ok(result)
}
fn horner(
    powers: &[ExactRatio],
    origin: &ExactRatio,
    width: &ExactRatio,
    constant: &ExactRatio,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Vec<ExactRatio>, RootIsolationError> {
    let mut next = vec![zero(); powers.len() + 1];
    for (power, value) in powers.iter().enumerate() {
        poll(cancelled)?;
        next[power] = next[power].add(&value.multiply(origin));
        next[power + 1] = next[power + 1].add(&value.multiply(width));
    }
    next[0] = next[0].add(constant);
    Ok(next)
}
fn bernstein(
    monomial: Matrix,
    degrees: [usize; 2],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Matrix, RootIsolationError> {
    let mut columns = Vec::new();
    for u in 0..=degrees[1] {
        let column = monomial
            .iter()
            .map(|row| row[u].clone())
            .collect::<Vec<_>>();
        columns.push(convert(&column, cancelled)?);
    }
    let mut result = Vec::new();
    for t in 0..=degrees[0] {
        let row = columns
            .iter()
            .map(|column| column[t].clone())
            .collect::<Vec<_>>();
        result.push(convert(&row, cancelled)?);
    }
    Ok(result)
}
fn convert(
    powers: &[ExactRatio],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Vec<ExactRatio>, RootIsolationError> {
    let degree = powers.len() - 1;
    let mut result = Vec::with_capacity(powers.len());
    for index in 0..=degree {
        result.push(coefficient(powers, index, degree, cancelled)?);
    }
    Ok(result)
}
fn coefficient(
    powers: &[ExactRatio],
    index: usize,
    degree: usize,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<ExactRatio, RootIsolationError> {
    let mut result = zero();
    let mut factor = ExactRatio::new(1.into(), 1.into());
    for (power, value) in powers.iter().enumerate().take(index + 1) {
        poll(cancelled)?;
        result = result.add(&value.multiply(&factor));
        if power < index {
            let step = ExactRatio::new(BigInt::from(index - power), BigInt::from(degree - power));
            factor = factor.multiply(&step);
        }
    }
    Ok(result)
}
fn strict_sign(
    values: &Matrix,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Option<Ordering>, RootIsolationError> {
    let Some(first) = values.first().and_then(|row| row.first()) else {
        return Ok(None);
    };
    let sign = first.sign();
    if sign == Ordering::Equal {
        return Ok(None);
    }
    for value in values.iter().flatten() {
        poll(cancelled)?;
        if value.sign() != sign {
            return Ok(None);
        }
    }
    Ok(Some(sign))
}
fn zero() -> ExactRatio {
    ExactRatio::new(0.into(), 1.into())
}

#[cfg(test)]
mod tests;

fn integer_ratios(values: &[BigInt]) -> Vec<ExactRatio> {
    values
        .iter()
        .map(|value| ExactRatio::new(value.clone(), 1.into()))
        .collect()
}
