// SPDX-License-Identifier: GPL-2.0-only
//! Differentiate the actual retained constant-weight residual with a geometric radius floor.
use super::*;
#[cfg(test)]
mod tests;
pub(in super::super) fn retained_derivative(
    q: &[Interval],
    numerator: &[Interval],
    proof: &slant::SlantCapture,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Option<(Interval, Interval)>, ConicalOriginalFailure> {
    let Some(c) = &proof.constant else {
        return Ok(None);
    };
    let minimum = original::minimum_radius(q, arithmetic)?;
    let residual_rate = derivative(&proof.residual, arithmetic)?;
    let numerator_rate = derivative(numerator, arithmetic)?;
    let error = error_bound(
        c,
        &minimum,
        &proof.error,
        &residual_rate,
        &numerator_rate,
        arithmetic,
    )?;
    let fit_rate = arithmetic.divide(&magnitude_interval(&numerator_rate), c)?;
    Ok(Some((error, fit_rate)))
}
fn error_bound(
    c: &Interval,
    minimum: &Interval,
    slant_error: &Interval,
    residual_rate: &[Interval],
    numerator_rate: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, ConicalOriginalFailure> {
    let squared = arithmetic.multiply(c, c)?;
    let denominator = arithmetic.multiply(&squared, minimum)?;
    let denominator = arithmetic.fraction(&denominator, 2, 1)?;
    let first = arithmetic.divide(&magnitude_interval(residual_rate), &denominator)?;
    let numerator = arithmetic.multiply(slant_error, &magnitude_interval(numerator_rate))?;
    let denominator = arithmetic.multiply(c, minimum)?;
    let second = arithmetic.divide(&numerator, &denominator)?;
    Ok(arithmetic.add(&first, &second)?)
}
