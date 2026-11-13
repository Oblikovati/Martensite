// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::DyadicBound;
use num_bigint::Sign;

pub(super) fn margins(
    direction: [f64; 2],
    derivatives: &[Vec<[Interval; 2]>],
    query: CylindricalConeQuery<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Option<Vec<DyadicBound>>> {
    let n = [
        arithmetic.scalar(direction[0])?,
        arithmetic.scalar(direction[1])?,
    ];
    let magnitude = n[0].magnitude().max(n[1].magnitude());
    let norm = Interval {
        lower: magnitude.clone(),
        upper: magnitude,
    };
    let mut margins = Vec::new();
    for (poles, source) in derivatives.iter().zip(query.sources) {
        let margin = margin(poles, source, &n, &norm, arithmetic)?;
        if margin.lower.sign() != Sign::Plus {
            return Ok(None);
        }
        margins.push(DyadicBound::new(
            margin.lower,
            -i32::from(query.options.fractional_bits),
        ));
    }
    Ok(Some(margins))
}

fn margin(
    poles: &[[Interval; 2]],
    source: &CylindricalConeSource<'_>,
    n: &[Interval; 2],
    norm: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let mut lower = None;
    for point in poles {
        let x = arithmetic.multiply(&point[0], &n[0])?;
        let z = arithmetic.multiply(&point[1], &n[1])?;
        let projection = arithmetic.add(&x, &z)?;
        lower = Some(
            lower.map_or(projection.lower.clone(), |old: num_bigint::BigInt| {
                old.min(projection.lower)
            }),
        );
    }
    let lower = lower.unwrap_or_default();
    let projection = Interval {
        lower: lower.clone(),
        upper: lower,
    };
    let error = arithmetic.scalar(source.certificate.developed_derivative_bound())?;
    let error = arithmetic.multiply(norm, &error)?;
    Ok(arithmetic.subtract(&projection, &error)?)
}
