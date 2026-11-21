// SPDX-License-Identifier: GPL-2.0-only
//! #544: all-root exact bracket decisions precede the unchanged eager fallback.
use super::super::{
    RootBracket, RootIsolation,
    coefficient_enclosure::{parameter_interval, power_enclosure},
    error::poll,
    polynomial::IntegerPolynomial,
};
use super::{RootSignError, RootSignEvaluation, RootSignStage, evaluate_isolated, tarski};
use std::cmp::Ordering;
pub(super) fn evaluate(
    source: IntegerPolynomial,
    query: IntegerPolynomial,
    isolation: RootIsolation,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<RootSignEvaluation, RootSignError> {
    match attempt(&query, &isolation, cancelled)? {
        Some(signs) => Ok(RootSignEvaluation { isolation, signs }),
        None => evaluate_isolated(source, query, isolation, cancelled),
    }
}
fn attempt(
    query: &IntegerPolynomial,
    isolation: &RootIsolation,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Option<Vec<Ordering>>, RootSignError> {
    poll(cancelled).map_err(evaluation_failure)?;
    let mut signs = Vec::with_capacity(isolation.roots().len());
    for root in isolation.roots() {
        poll(cancelled).map_err(evaluation_failure)?;
        let Some(sign) = bracket_sign(query, root, cancelled)? else {
            return Ok(None);
        };
        signs.push(sign);
    }
    Ok(Some(signs))
}
fn bracket_sign(
    query: &IntegerPolynomial,
    root: &RootBracket,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Option<Ordering>, RootSignError> {
    if query.0.is_empty() {
        return Ok(Some(Ordering::Equal));
    }
    if root.bounds().0 == root.bounds().1 {
        return Ok(Some(tarski::point_sign(query, root)));
    }
    let enclosure = power_enclosure(&query.0, &parameter_interval(root), cancelled)
        .map_err(evaluation_failure)?;
    Ok(strict_sign(enclosure.lower.sign(), enclosure.upper.sign()))
}
fn strict_sign(lower: Ordering, upper: Ordering) -> Option<Ordering> {
    match (lower, upper) {
        (Ordering::Greater, _) => Some(Ordering::Greater),
        (_, Ordering::Less) => Some(Ordering::Less),
        _ => None,
    }
}
fn evaluation_failure(cause: super::super::RootIsolationError) -> RootSignError {
    RootSignStage::Evaluation.failure(cause)
}
#[cfg(test)]
mod tests;
