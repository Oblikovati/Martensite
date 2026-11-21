// SPDX-License-Identifier: GPL-2.0-only
use super::*;

pub(super) struct ComparisonBounds {
    pub enclosure: [f64; 2],
    pub discrepancy: f64,
    pub representative_error: f64,
}
pub(super) fn calculate(
    flux: &CertifiedCylindricalFlux,
    normal: &CertifiedCylindricalNormalMeasure,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<ComparisonBounds> {
    let actual = interval(flux.enclosure(), arithmetic)?;
    let ideal = interval(normal.profile_volume_enclosure(), arithmetic)?;
    let difference = arithmetic.subtract(&actual, &ideal)?;
    let representative = arithmetic.scalar(flux.representative())?;
    let representative_difference = arithmetic.subtract(&representative, &ideal)?;
    Ok(ComparisonBounds {
        enclosure: enclosure(&difference, arithmetic)?,
        discrepancy: arithmetic.upper_f64(difference.magnitude())?,
        representative_error: arithmetic.upper_f64(representative_difference.magnitude())?,
    })
}
fn interval(bounds: [f64; 2], arithmetic: &mut Arithmetic<'_>) -> Checked<Interval> {
    let lo = arithmetic.scalar(bounds[0])?;
    let hi = arithmetic.scalar(bounds[1])?;
    Ok(Interval {
        lower: lo.lower,
        upper: hi.upper,
    })
}
fn enclosure(value: &Interval, arithmetic: &Arithmetic<'_>) -> Checked<[f64; 2]> {
    let lo = arithmetic.upper_f64(-value.lower.clone())?;
    let hi = arithmetic.upper_f64(value.upper.clone())?;
    Ok([-lo, hi])
}
pub(super) fn check_limit(
    limit: CylindricalMeasureLimit,
    bounds: &ComparisonBounds,
) -> Checked<()> {
    let bound = bounds.discrepancy.max(bounds.representative_error);
    if let CylindricalMeasureLimit::Refuse(requested) = limit
        && bound > requested
    {
        return Err(CylindricalMeasureComparisonFailure::Uncertified { bound, requested });
    }
    Ok(())
}
