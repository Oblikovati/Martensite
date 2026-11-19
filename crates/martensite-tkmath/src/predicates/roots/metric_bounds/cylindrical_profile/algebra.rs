// SPDX-License-Identifier: GPL-2.0-only
use super::*;

pub(super) use crate::predicates::roots::certificate_arithmetic::bernstein_curve::derivative;

pub(super) fn elevate(
    poles: &[Interval],
    degree: usize,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Vec<Interval>> {
    let one = arithmetic.scalar(1.)?;
    Ok(polynomial::product(
        poles,
        &vec![one; degree + 2 - poles.len()],
        arithmetic,
    )?)
}

pub(in crate::predicates::roots::metric_bounds) fn difference(
    a: &[Interval],
    b: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Vec<Interval>> {
    let degree = a.len().max(b.len()) - 1;
    let a = elevate(a, degree, arithmetic)?;
    let b = elevate(b, degree, arithmetic)?;
    Ok(polynomial::difference(&a, &b, arithmetic)?)
}

pub(in crate::predicates::roots::metric_bounds) fn sum(
    a: &[Interval],
    b: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Vec<Interval>> {
    let degree = a.len().max(b.len()) - 1;
    let a = elevate(a, degree, arithmetic)?;
    let b = elevate(b, degree, arithmetic)?;
    Ok(polynomial::sum(&a, &b, arithmetic)?)
}

pub(in crate::predicates::roots::metric_bounds) fn hull(poles: &[Interval]) -> Interval {
    Interval {
        lower: polynomial::lower(poles),
        upper: poles
            .iter()
            .fold(poles[0].upper.clone(), |a, b| a.max(b.upper.clone())),
    }
}

pub(in crate::predicates::roots::metric_bounds) fn point_upper(value: &Interval) -> Interval {
    Interval {
        lower: value.upper.clone(),
        upper: value.upper.clone(),
    }
}

pub(in crate::predicates::roots::metric_bounds) fn magnitude(poles: &[Interval]) -> Interval {
    let bound = polynomial::magnitude(poles);
    Interval {
        lower: bound.clone(),
        upper: bound,
    }
}

pub(super) fn widen(
    poles: &mut [Interval],
    error: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<()> {
    let delta = Interval {
        lower: -error.upper.clone(),
        upper: error.upper.clone(),
    };
    for pole in poles {
        *pole = arithmetic.add(pole, &delta)?;
    }
    Ok(())
}

pub(in crate::predicates::roots::metric_bounds) fn remainder(
    amplitude: &Interval,
    order: u64,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let mut bound = arithmetic.scalar(1.)?;
    for k in 1..=order {
        bound = arithmetic.multiply(&bound, amplitude)?;
        bound = arithmetic.fraction(&bound, 1, k)?;
    }
    Ok(point_upper(&bound))
}
