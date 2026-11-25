// SPDX-License-Identifier: GPL-2.0-only
//! Analytic Taylor jets of products and trigonometric compositions.
use super::{
    super::cylindrical_profile::trigonometric, Arithmetic, interval::negative, refusal::Checked,
};
use crate::predicates::roots::certificate_arithmetic::bounded_dyadic::Interval;
use num_bigint::BigInt;

pub(super) const FACTORIAL: [u64; 5] = [1, 1, 2, 6, 24];
pub(super) type Series = [Interval; 5];
pub(super) type WorldJet = [Series; 3];

pub(super) fn multiply(a: &Series, b: &Series, arithmetic: &mut Arithmetic<'_>) -> Checked<Series> {
    let mut output = std::array::from_fn(|_| arithmetic.zero());
    for (order, value) in output.iter_mut().enumerate() {
        for (i, left) in a[..=order].iter().enumerate() {
            let term = arithmetic.multiply(left, &b[order - i])?;
            *value = arithmetic.add(value, &term)?;
        }
    }
    Ok(output)
}

pub(super) fn trig(angle: &Series, arithmetic: &mut Arithmetic<'_>) -> Checked<[Series; 2]> {
    let [mut cosine, mut sine] =
        std::array::from_fn(|_| std::array::from_fn(|_| arithmetic.zero()));
    [cosine[0], sine[0]] = trig_range(&angle[0], arithmetic)?;
    for order in 1..5 {
        cosine[order] = negative(&chain_coefficient(&sine, angle, order, arithmetic)?);
        sine[order] = chain_coefficient(&cosine, angle, order, arithmetic)?;
    }
    Ok([cosine, sine])
}

fn chain_coefficient(
    other: &Series,
    angle: &Series,
    order: usize,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    // Compare power-series coefficients in sin' = cos * angle' and
    // cos' = -sin * angle'. The coefficients represent derivatives divided by k!.
    let mut sum = arithmetic.zero();
    for (i, value) in other[..order].iter().enumerate() {
        let k = order - i;
        let rate = arithmetic.fraction(&angle[k], k as u64, 1)?;
        let term = arithmetic.multiply(value, &rate)?;
        sum = arithmetic.add(&sum, &term)?;
    }
    Ok(arithmetic.fraction(&sum, 1, order as u64)?)
}

fn trig_range(angle: &Interval, arithmetic: &mut Arithmetic<'_>) -> Checked<[Interval; 2]> {
    let center: BigInt = (&angle.lower + &angle.upper) / 2;
    let radius = (&center - &angle.lower).max(&angle.upper - &center);
    let singleton = Interval {
        lower: center.clone(),
        upper: center,
    };
    let [cosine, sine] = trigonometric::scalar(&singleton, arithmetic)?;
    let unit = arithmetic.scalar(1.)?.upper;
    // Both functions are 1-Lipschitz globally, even if the interval crosses a turn.
    let widen = |value: Interval| Interval {
        lower: (value.lower - &radius).max(-&unit),
        upper: (value.upper + &radius).min(unit.clone()),
    };
    Ok([widen(cosine), widen(sine)])
}
