// SPDX-License-Identifier: GPL-2.0-only
use super::*;

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
    let mut source: [Vec<Interval>; 2] = std::array::from_fn(|_| Vec::new());
    for pole in authored {
        source[0].push(arithmetic.scalar(pole[0])?);
        source[1].push(arithmetic.scalar(pole[1])?);
    }
    for axis in 0..2 {
        if authored.iter().all(|pole| pole[axis] == authored[0][axis]) {
            source[axis].truncate(1);
        }
    }
    let source = [
        restrict(&source[0], interval, arithmetic)?,
        restrict(&source[1], interval, arithmetic)?,
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

fn restrict(
    poles: &[Interval],
    span: CylindricalProfileInterval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Vec<Interval>> {
    let mut result = poles.to_vec();
    for bit in (0..span.level()).rev() {
        let [left, right] = bisect(&result, arithmetic)?;
        result = if span.index() >> bit & 1 == 0 {
            left
        } else {
            right
        };
    }
    Ok(result)
}

pub(super) fn bisect(
    poles: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[Vec<Interval>; 2]> {
    let mut work = poles.to_vec();
    let mut left = vec![work[0].clone()];
    let mut right = vec![work[work.len() - 1].clone()];
    for count in (1..work.len()).rev() {
        for i in 0..count {
            let sum = arithmetic.add(&work[i], &work[i + 1])?;
            work[i] = arithmetic.fraction(&sum, 1, 2)?;
        }
        left.push(work[0].clone());
        right.push(work[count - 1].clone());
    }
    right.reverse();
    Ok([left, right])
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
