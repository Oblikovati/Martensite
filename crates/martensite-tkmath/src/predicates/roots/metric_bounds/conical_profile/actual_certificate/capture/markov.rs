// SPDX-License-Identifier: GPL-2.0-only
//! Unit-dual Markov bounds transfer issued C0 error without expanding degree18 products.
use super::*;
pub(super) fn factor(
    fit: &FitCapture,
    actual: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<Interval> {
    let wa = numeric::hull(actual);
    let wa_rate = numeric::max_magnitude(&derivative(actual, arithmetic)?);
    let w_rate = numeric::max_magnitude(&derivative(&fit.weights, arithmetic)?);
    let first = arithmetic.divide(&wa_rate, &numeric::lower_point(&wa))?;
    let second = arithmetic.divide(&w_rate, &numeric::lower_point(&fit.weight_hull))?;
    let p_rate = numeric::max_magnitude(&fit.p_rate);
    let p_product = arithmetic.multiply(&numeric::magnitude(&fit.p_hull), &p_rate)?;
    let twice = arithmetic.fraction(&p_product, 2, 1)?;
    let third = arithmetic.divide(&twice, &numeric::lower_point(&fit.b))?;
    let sum = arithmetic.add(&first, &second)?;
    let rate = arithmetic.add(&sum, &third)?;
    let ratio = ratio(&wa, fit, arithmetic)?;
    let degree = (actual.len() - 1)
        + (fit.numerator.len().max(fit.weights.len()) - 1)
        + 2 * (fit.p.len() - 1);
    let majorant = arithmetic.fraction(&ratio, 2 * (degree as u64) * (degree as u64), 1)?;
    Ok(arithmetic.add(&majorant, &rate)?)
}
fn ratio(
    actual: &Interval,
    fit: &FitCapture,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<Interval> {
    let first = arithmetic.divide(&numeric::upper_point(actual), &numeric::lower_point(actual))?;
    let second = arithmetic.divide(
        &numeric::upper_point(&fit.weight_hull),
        &numeric::lower_point(&fit.weight_hull),
    )?;
    let third = arithmetic.divide(&numeric::upper_point(&fit.b), &numeric::lower_point(&fit.b))?;
    let product = arithmetic.multiply(&second, &third)?;
    Ok(arithmetic.multiply(&first, &product)?)
}
pub(super) fn du_bound(
    side: &SideCapture,
    sine: &Interval,
    cosine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<Interval> {
    let storage = numeric::max_magnitude(&side.storage);
    let radial = arithmetic.subtract(&numeric::lower_point(&side.radius), &storage)?;
    numeric::positive("actual radial margin", &radial, arithmetic)?;
    let first = arithmetic.multiply(sine, &numeric::magnitude(&side.slant_rate))?;
    let second = arithmetic.multiply(
        &numeric::upper_point(&side.radius),
        &numeric::magnitude(&side.angle_rate),
    )?;
    let horizontal = arithmetic.add(&first, &second)?;
    let displacement = arithmetic.multiply(cosine, &storage)?;
    let displacement = arithmetic.fraction(&displacement, 2, 1)?;
    let direction = arithmetic.divide(&displacement, &radial)?;
    let term = arithmetic.multiply(&direction, &horizontal)?;
    Ok(arithmetic.add(&side.derivative_error, &term)?)
}

#[cfg(test)]
mod tests;
