// SPDX-License-Identifier: GPL-2.0-only
//! Integrated angular error and pointwise derivative retain distinct rigorous bounds.
use super::*;
mod primitive;
pub(in super::super) use primitive::weighted_primitive_error_selected;
#[cfg(test)]
mod tests;
pub(in super::super) fn angular_in_chart_selected(
    source: &[Vec<Interval>; 2],
    fit: &[Interval],
    q: &[Interval],
    anchor: Interval,
    sine: &Interval,
    refinement: ConicalPrimitiveRefinement,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<AngularEvidence, AngularSpanFailure> {
    let denominator = full_denominator(q, sine, fit, arithmetic)?;
    let residual = angular_residual(source, fit, sine, q, arithmetic)?;
    let mut proof = from_residual(&residual, anchor.clone(), &denominator, arithmetic)?;
    let tighter = primitive::bound_selected(
        &residual,
        q,
        fit,
        &denominator,
        &anchor,
        refinement,
        arithmetic,
    )?;
    proof.bound.upper = proof.bound.upper.min(tighter.upper);
    Ok(proof)
}
fn full_denominator(
    q: &[Interval],
    sine: &Interval,
    fit: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, AngularSpanFailure> {
    let base = angular_denominator(q, sine, arithmetic)?;
    let Some(distance) = zero_distance(fit)? else {
        return Ok(base);
    };
    let square = arithmetic.multiply(&distance, &distance)?;
    let one = arithmetic.scalar(1.)?;
    let factor = arithmetic.add(&one, &square)?;
    Ok(arithmetic.multiply(&base, &factor)?)
}
fn zero_distance(fit: &[Interval]) -> Result<Option<Interval>, AngularSpanFailure> {
    let Interval {
        lower: minimum,
        upper: maximum,
    } = fit_hull(fit)?;
    if minimum <= 0.into() && maximum >= 0.into() {
        return Ok(None);
    }
    let endpoint = if minimum > 0.into() { minimum } else { maximum };
    let absolute = Interval {
        lower: endpoint.clone(),
        upper: endpoint,
    }
    .magnitude();
    Ok(Some(Interval {
        lower: absolute.clone(),
        upper: absolute,
    }))
}
fn fit_hull(fit: &[Interval]) -> Result<Interval, AngularSpanFailure> {
    let first = fit.first().ok_or(AngularSpanFailure::InvalidFit {
        count: 0,
        nonfinite: None,
    })?;
    let minimum = fit
        .iter()
        .fold(first.lower.clone(), |a, b| a.min(b.lower.clone()));
    let maximum = fit
        .iter()
        .fold(first.upper.clone(), |a, b| a.max(b.upper.clone()));
    Ok(Interval {
        lower: minimum,
        upper: maximum,
    })
}
fn from_residual(
    residual: &[Interval],
    anchor: Interval,
    denominator: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<AngularEvidence, AngularSpanFailure> {
    let derivative = arithmetic.divide(&magnitude_interval(residual), denominator)?;
    let mean = absolute_coefficient_mean(residual, arithmetic)?;
    let integral = arithmetic.divide(&mean, denominator)?;
    let bound = arithmetic.add(&magnitude_interval(&[anchor]), &integral)?;
    Ok(AngularEvidence { bound, derivative })
}
pub(in super::super) fn absolute_coefficient_mean(
    residual: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let first = residual
        .first()
        .ok_or(BoundedArithmeticFailure::CoefficientCount { left: 0, right: 1 })?;
    let mut sum = magnitude_interval(std::slice::from_ref(first));
    for coefficient in &residual[1..] {
        sum = arithmetic.add(&sum, &magnitude_interval(std::slice::from_ref(coefficient)))?;
    }
    // Every Bernstein basis function is nonnegative and integrates to 1/(degree+1).
    arithmetic.fraction(&sum, 1, residual.len() as u64)
}
