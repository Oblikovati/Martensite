//! Exact power-to-Bernstein conversion proves signs over the entire closed square.
use super::{
    Ratio,
    budget::Budget,
    polynomial::{Polynomial, integer},
    types::SurfaceQueryCause as Cause,
};
use std::cmp::Ordering;
fn binomial(n: usize, k: usize) -> u64 {
    (0..k).fold(1, |value, i| value * (n - i) as u64 / (i + 1) as u64)
}
fn factor(i: usize, k: usize, degree: usize) -> Ratio {
    Ratio::new(binomial(i, k).into(), binomial(degree, k).into())
}
fn coefficient(
    poly: &Polynomial,
    index: [usize; 2],
    degree: [usize; 2],
    budget: &mut Budget<'_>,
) -> Result<Ratio, Cause> {
    let mut sum = integer(0);
    for (power, value) in &poly.0 {
        if power[0] > index[0] || power[1] > index[1] {
            continue;
        }
        budget.step()?;
        let first = budget.retain(value.multiply(&factor(index[0], power[0], degree[0])))?;
        let term = budget.retain(first.multiply(&factor(index[1], power[1], degree[1])))?;
        sum = budget.retain(sum.add(&term))?;
    }
    Ok(sum)
}
pub(super) fn positive(poly: &Polynomial, budget: &mut Budget<'_>) -> Result<bool, Cause> {
    let degree = poly.degree();
    for u in 0..=degree[0] {
        for v in 0..=degree[1] {
            if coefficient(poly, [u, v], degree, budget)?.sign() != Ordering::Greater {
                return Ok(false);
            }
        }
    }
    budget.work.positive_hulls += 1;
    Ok(true)
}
/// Exactly remove one vanishing V endpoint factor; both signs remain source-derived.
pub(super) fn endpoint_positive(poly: &Polynomial, budget: &mut Budget<'_>) -> Result<bool, Cause> {
    if positive(poly, budget)? {
        return Ok(true);
    }
    if poly.endpoint(1, 0, budget)?.0.is_empty() {
        let quotient = Polynomial(
            poly.0
                .iter()
                .map(|(p, c)| ([p[0], p[1] - 1], c.clone()))
                .collect(),
        );
        if positive(&quotient, budget)? {
            return Ok(true);
        }
    }
    divide_upper(poly, budget)
}
fn divide_upper(poly: &Polynomial, budget: &mut Budget<'_>) -> Result<bool, Cause> {
    if !poly.endpoint(1, 1, budget)?.0.is_empty() {
        return Ok(false);
    }
    let degree = poly.degree();
    if degree[1] == 0 {
        return Ok(false);
    }
    let mut quotient = Polynomial::default();
    for u in 0..=degree[0] {
        let mut sum = integer(0);
        for v in 0..degree[1] {
            budget.step()?;
            sum = budget.retain(sum.add(poly.0.get(&[u, v]).unwrap_or(&integer(0))))?;
            quotient.insert([u, v], sum.clone());
        }
    }
    positive(&quotient, budget)
}
