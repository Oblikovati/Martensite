// SPDX-License-Identifier: GPL-2.0-only
//! Exact constant-weight conditioning retains the original elevated residual basis.
use super::*;
use crate::predicates::roots::certificate_arithmetic::bernstein_bounds::{difference, scale};
#[cfg(test)]
mod tests;
pub(super) fn constant_point(weights: &[Interval]) -> Option<&Interval> {
    let first = weights.first()?;
    if first.lower <= 0.into() || first.lower != first.upper {
        return None;
    }
    weights
        .iter()
        .all(|weight| weight == first)
        .then_some(first)
}
pub(in super::super) fn residual(
    q: &[Interval],
    numerator: &[Interval],
    weights: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Vec<Interval>, SlantFailure> {
    let Some(weight) = constant_point(weights) else {
        return slant_residual(q, numerator, weights, arithmetic);
    };
    let squared = product(numerator, numerator, arithmetic)?;
    let constant_squared = arithmetic.multiply(weight, weight)?;
    let reference = scale(q, &constant_squared, arithmetic)?;
    let reduced = common_difference(&squared, &reference, arithmetic)?;
    let target = (2 * (numerator.len() - 1)).max(q.len() - 1 + 2 * (weights.len() - 1));
    if target + 1 == reduced.len() {
        return Ok(reduced);
    }
    let one = arithmetic.scalar(1.)?;
    Ok(product(
        &reduced,
        &vec![one; target + 2 - reduced.len()],
        arithmetic,
    )?)
}
fn common_difference(
    a: &[Interval],
    b: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Vec<Interval>, SlantFailure> {
    if a.len() == b.len() {
        return Ok(difference(a, b, arithmetic)?);
    }
    algebra::difference(a, b, arithmetic).map_err(SlantFailure::Polynomial)
}
