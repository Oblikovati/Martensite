// SPDX-License-Identifier: GPL-2.0-only
//! Signed residual primitives retain the denominator-variation remainder.
use super::*;
mod half_split;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod weighted_tests;
#[cfg(test)]
pub(super) fn bound(
    residual: &[Interval],
    q: &[Interval],
    fit: &[Interval],
    denominator: &Interval,
    anchor: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, AngularSpanFailure> {
    bound_selected(
        residual,
        q,
        fit,
        denominator,
        anchor,
        ConicalPrimitiveRefinement::UnsplitV1,
        arithmetic,
    )
}
pub(super) fn bound_selected(
    residual: &[Interval],
    q: &[Interval],
    fit: &[Interval],
    denominator: &Interval,
    anchor: &Interval,
    refinement: ConicalPrimitiveRefinement,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, AngularSpanFailure> {
    let primitive = primitive(residual, arithmetic)?;
    let relative_rate = denominator_rate(q, fit, arithmetic)?;
    Ok(weighted_from_primitive_selected(
        &primitive,
        anchor,
        denominator,
        &relative_rate,
        refinement,
        arithmetic,
    )?)
}
// The caller supplies a certified positive denominator lower bound and a bound on |B'/B|.
#[cfg(test)]
pub(in super::super::super) fn weighted_primitive_error(
    residual: &[Interval],
    anchor: &Interval,
    positive_b_min: &Interval,
    logarithmic_rate: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    weighted_primitive_error_selected(
        residual,
        anchor,
        positive_b_min,
        logarithmic_rate,
        ConicalPrimitiveRefinement::UnsplitV1,
        arithmetic,
    )
}
pub(in super::super::super) fn weighted_primitive_error_selected(
    residual: &[Interval],
    anchor: &Interval,
    positive_b_min: &Interval,
    logarithmic_rate: &Interval,
    refinement: ConicalPrimitiveRefinement,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let primitive = primitive(residual, arithmetic)?;
    weighted_from_primitive_selected(
        &primitive,
        anchor,
        positive_b_min,
        logarithmic_rate,
        refinement,
        arithmetic,
    )
}
fn weighted_from_primitive_selected(
    primitive: &[Interval],
    anchor: &Interval,
    denominator: &Interval,
    relative_rate: &Interval,
    refinement: ConicalPrimitiveRefinement,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    if refinement == ConicalPrimitiveRefinement::UnsplitV1 {
        return weighted_from_primitive(primitive, anchor, denominator, relative_rate, arithmetic);
    }
    let mean = absolute_coefficient_mean(primitive, arithmetic)?;
    let (mean, magnitude) = half_split::metrics(primitive, &mean, arithmetic)?;
    finish_weighted(
        &mean,
        &magnitude,
        anchor,
        denominator,
        relative_rate,
        arithmetic,
    )
}
fn weighted_from_primitive(
    primitive: &[Interval],
    anchor: &Interval,
    denominator: &Interval,
    relative_rate: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let mean = absolute_coefficient_mean(primitive, arithmetic)?;
    let remainder = arithmetic.multiply(&mean, relative_rate)?;
    let numerator = arithmetic.add(&magnitude_interval(primitive), &remainder)?;
    finish_integral(numerator, anchor, denominator, arithmetic)
}
fn finish_weighted(
    mean: &Interval,
    magnitude: &Interval,
    anchor: &Interval,
    denominator: &Interval,
    relative_rate: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let remainder = arithmetic.multiply(mean, relative_rate)?;
    let numerator = arithmetic.add(magnitude, &remainder)?;
    finish_integral(numerator, anchor, denominator, arithmetic)
}
fn finish_integral(
    numerator: Interval,
    anchor: &Interval,
    denominator: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let integral = arithmetic.divide(&numerator, denominator)?;
    arithmetic.add(&magnitude_interval(std::slice::from_ref(anchor)), &integral)
}

fn primitive(residual: &[Interval], arithmetic: &mut Arithmetic<'_>) -> Checked<Vec<Interval>> {
    if residual.is_empty() {
        return Err(BoundedArithmeticFailure::CoefficientCount { left: 0, right: 1 });
    }
    let mut prefix = arithmetic.zero();
    let mut result = vec![arithmetic.zero()];
    for coefficient in residual {
        prefix = arithmetic.add(&prefix, coefficient)?;
        result.push(arithmetic.fraction(&prefix, 1, residual.len() as u64)?);
    }
    Ok(result)
}
fn denominator_rate(
    q: &[Interval],
    fit: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, AngularSpanFailure> {
    let minimum = lower(q);
    if minimum <= 0.into() {
        return Err(AngularSpanFailure::UnresolvedDistance {
            lower: DyadicBound::new(minimum, -i32::from(arithmetic.fractional_bits())),
        });
    }
    let minimum = Interval {
        lower: minimum.clone(),
        upper: minimum,
    };
    let rate = derivative(q, arithmetic)?;
    let first = arithmetic.divide(&magnitude_interval(&rate), &minimum)?;
    let second = fit_rate(fit, arithmetic)?;
    Ok(arithmetic.add(&first, &second)?)
}
fn fit_rate(
    fit: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, AngularSpanFailure> {
    let rate = magnitude_interval(&derivative(fit, arithmetic)?);
    let magnitude = magnitude_interval(fit);
    let product = arithmetic.multiply(&rate, &magnitude)?;
    let numerator = arithmetic.fraction(&product, 2, 1)?;
    let one = arithmetic.scalar(1.)?;
    let denominator = match zero_distance(fit)? {
        Some(distance) => {
            let squared = arithmetic.multiply(&distance, &distance)?;
            arithmetic.add(&one, &squared)?
        }
        None => one,
    };
    Ok(arithmetic.divide(&numerator, &denominator)?)
}
