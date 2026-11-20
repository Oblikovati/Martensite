// SPDX-License-Identifier: GPL-2.0-only
use super::{
    bounded_dyadic::{Arithmetic, Interval},
    bounded_failure::{BoundedArithmeticFailure, Checked},
};

pub(in crate::predicates::roots) fn product(
    a: &[Interval],
    b: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Vec<Interval>> {
    let mut result = vec![arithmetic.zero(); a.len() + b.len() - 1];
    for (i, left) in a.iter().enumerate() {
        accumulate(&mut result, (a.len() - 1, i, left), b, arithmetic)?;
    }
    Ok(result)
}

fn accumulate(
    result: &mut [Interval],
    left: (usize, usize, &Interval),
    b: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<()> {
    let (m, i, left) = left;
    let n = b.len() - 1;
    for (j, right) in b.iter().enumerate() {
        let numerator = arithmetic.binomial(m, i) * arithmetic.binomial(n, j);
        let denominator = arithmetic.binomial(m + n, i + j);
        let term = arithmetic.multiply(left, right)?;
        let term = arithmetic.fraction(&term, numerator, denominator)?;
        result[i + j] = arithmetic.add(&result[i + j], &term)?;
    }
    Ok(())
}

pub(in crate::predicates::roots) fn integrate_unit(
    coefficients: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let mut sum = arithmetic.zero();
    for coefficient in coefficients {
        sum = arithmetic.add(&sum, coefficient)?;
    }
    // Every Bernstein basis function of degree n has integral 1/(n+1).
    arithmetic.fraction(&sum, 1, coefficients.len() as u64)
}

pub(in crate::predicates::roots) fn scale(
    a: &[Interval],
    factor: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Vec<Interval>> {
    a.iter()
        .map(|coefficient| arithmetic.multiply(coefficient, factor))
        .collect()
}

pub(in crate::predicates::roots) fn sum(
    a: &[Interval],
    b: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Vec<Interval>> {
    common_degree(a, b)?;
    a.iter().zip(b).map(|(a, b)| arithmetic.add(a, b)).collect()
}

pub(in crate::predicates::roots) fn difference(
    a: &[Interval],
    b: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Vec<Interval>> {
    common_degree(a, b)?;
    a.iter()
        .zip(b)
        .map(|(a, b)| arithmetic.subtract(a, b))
        .collect()
}

fn common_degree(a: &[Interval], b: &[Interval]) -> Checked<()> {
    if a.len() != b.len() {
        return Err(BoundedArithmeticFailure::CoefficientCount {
            left: a.len(),
            right: b.len(),
        });
    }
    Ok(())
}

pub(in crate::predicates::roots) fn magnitude(coefficients: &[Interval]) -> num_bigint::BigInt {
    coefficients.iter().fold(0.into(), |bound, coefficient| {
        bound.max(coefficient.magnitude())
    })
}

pub(in crate::predicates::roots) fn lower(coefficients: &[Interval]) -> num_bigint::BigInt {
    coefficients
        .iter()
        .fold(coefficients[0].lower.clone(), |bound, coefficient| {
            bound.min(coefficient.lower.clone())
        })
}
