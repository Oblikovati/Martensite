// SPDX-License-Identifier: GPL-2.0-only
//! Integrating by parts retains the actual denominator's logarithmic variation.
use super::*;
#[cfg(test)]
pub(super) fn bound(
    residual: &AngularResidual,
    anchor: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, CylindricalProfileFailure> {
    bound_selected(
        residual,
        anchor,
        ConicalPrimitiveRefinement::UnsplitV1,
        arithmetic,
    )
}
pub(super) fn bound_selected(
    residual: &AngularResidual,
    anchor: &Interval,
    refinement: ConicalPrimitiveRefinement,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, CylindricalProfileFailure> {
    let minimum = denominator_minimum(residual);
    let mean = angular::absolute_coefficient_mean(&residual.numerator, arithmetic)?;
    let integral = arithmetic.divide(&mean, &minimum)?;
    let ordinary = arithmetic.add(&algebra::magnitude(std::slice::from_ref(anchor)), &integral)?;
    let rate = logarithmic_rate(residual, &minimum, arithmetic)?;
    let primitive = angular::weighted_primitive_error_selected(
        &residual.numerator,
        anchor,
        &minimum,
        &rate,
        refinement,
        arithmetic,
    )?;
    Ok(Interval {
        lower: 0.into(),
        upper: ordinary.upper.min(primitive.upper),
    })
}
fn denominator_minimum(residual: &AngularResidual) -> Interval {
    // The actual polynomial is 1+p², even if its Bernstein hull crosses below one.
    let minimum = polynomial::lower(&residual.denominator).max(residual.unit.lower.clone());
    Interval {
        lower: minimum.clone(),
        upper: minimum,
    }
}
fn logarithmic_rate(
    residual: &AngularResidual,
    minimum: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let derivative = derivative(&residual.denominator, arithmetic)?;
    arithmetic.divide(&algebra::magnitude(&derivative), minimum)
}
