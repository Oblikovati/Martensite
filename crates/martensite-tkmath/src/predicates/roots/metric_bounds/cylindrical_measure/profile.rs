// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::certificate_arithmetic::bernstein_bounds as polynomial;

pub(super) fn capture(
    source: &CylindricalProfileSource<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[Vec<Interval>; 2]> {
    let poles = curve::capture_planar(source.authored, arithmetic)?;
    let x = curve::restrict(
        &poles[0],
        source.interval.index(),
        source.interval.level(),
        arithmetic,
    )?;
    let z = curve::restrict(
        &poles[1],
        source.interval.index(),
        source.interval.level(),
        arithmetic,
    )?;
    Ok([x, z])
}
pub(super) fn integral(
    poles: &[Vec<Interval>; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let derivative = curve::derivative(&poles[1], arithmetic)?;
    let product = polynomial::product(&poles[0], &derivative, arithmetic)?;
    Ok(polynomial::integrate_unit(&product, arithmetic)?)
}
pub(super) fn discrepancy(
    source: &CylindricalProfileSource<'_>,
    poles: &[Vec<Interval>; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let p = source.certificate;
    let radius = arithmetic.scalar(source.support_radius)?;
    let angular = arithmetic.scalar(p.angular_position_bound())?;
    let x = arithmetic.multiply(&radius, &angular)?;
    let z = arithmetic.scalar(p.height_position_bound())?;
    let dz = arithmetic.scalar(p.height_derivative_bound())?;
    let vx = variation(&poles[0], arithmetic)?;
    let vz = variation(&poles[1], arithmetic)?;
    let a = arithmetic.multiply(&x, &vz)?;
    let b = arithmetic.multiply(&z, &vx)?;
    let c = arithmetic.multiply(&x, &dz)?;
    let sum = arithmetic.add(&a, &b)?;
    Ok(arithmetic.add(&sum, &c)?)
}
fn variation(poles: &[Interval], arithmetic: &mut Arithmetic<'_>) -> Checked<Interval> {
    let mut sum = arithmetic.zero();
    for pair in poles.windows(2) {
        let delta = arithmetic.subtract(&pair[1], &pair[0])?;
        let bound = delta.magnitude();
        sum = arithmetic.add(
            &sum,
            &Interval {
                lower: 0.into(),
                upper: bound,
            },
        )?;
    }
    Ok(sum)
}
