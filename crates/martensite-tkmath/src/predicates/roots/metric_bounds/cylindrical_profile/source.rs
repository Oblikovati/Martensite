// SPDX-License-Identifier: GPL-2.0-only
use super::*;
pub(super) use crate::predicates::roots::certificate_arithmetic::bernstein_curve::bisect;
use crate::predicates::roots::certificate_arithmetic::bernstein_curve::{capture_planar, restrict};

pub(super) fn authored(
    query: CylindricalProfileQuery<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[Vec<Interval>; 2]> {
    developed(
        query.authored,
        query.interval,
        query.support_radius,
        query.angular_origin,
        arithmetic,
    )
}

pub(super) fn developed(
    authored: &[[f64; 2]],
    interval: CylindricalProfileInterval,
    support_radius: f64,
    angular_origin: f64,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[Vec<Interval>; 2]> {
    let source = capture_planar(authored, arithmetic)?;
    let source = [
        restrict(&source[0], interval.index(), interval.level(), arithmetic)?,
        restrict(&source[1], interval.index(), interval.level(), arithmetic)?,
    ];
    let radius = arithmetic.scalar(support_radius)?;
    let origin = arithmetic.scalar(angular_origin)?;
    let angles = source[0]
        .iter()
        .map(|s| {
            let angle = arithmetic.divide(s, &radius)?;
            Ok(arithmetic.add(&angle, &origin)?)
        })
        .collect::<Checked<_>>()?;
    Ok([angles, source[1].clone()])
}

pub(super) fn stored(
    source: super::super::MetricBezier<'_, 3>,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[Vec<Interval>; 4]> {
    let mut result: [Vec<Interval>; 4] = std::array::from_fn(|_| Vec::new());
    for (i, pole) in source.poles.iter().enumerate() {
        let weight = arithmetic.scalar(source.weights.map_or(1., |w| w[i]))?;
        for axis in 0..3 {
            let point = arithmetic.scalar(pole[axis])?;
            result[axis].push(arithmetic.multiply(&point, &weight)?);
        }
        result[3].push(weight);
    }
    Ok(result)
}
