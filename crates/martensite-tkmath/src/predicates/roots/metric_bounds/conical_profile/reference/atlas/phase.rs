// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::certificate_arithmetic::circular_constants;

pub(super) fn separation(
    ranges: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(), ConicalReferenceFailure> {
    let pi = circular_constants::pi(arithmetic)?;
    let period = arithmetic.fraction(&pi, 2, 1)?;
    let mut lifted = ranges
        .iter()
        .map(|range| lift(range, &period, arithmetic))
        .collect::<Result<Vec<_>, _>>()?;
    lifted.sort_unstable_by(|a, b| a.lower.cmp(&b.lower));
    for pair in lifted.windows(2) {
        gap(&pair[0].upper, &pair[1].lower, arithmetic)?;
    }
    if let (Some(first), Some(last)) = (lifted.first(), lifted.last()) {
        gap(&last.upper, &(&first.lower + &period.lower), arithmetic)?;
    }
    Ok(())
}
fn lift(
    phase: &Interval,
    period: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, ConicalReferenceFailure> {
    // Any integer shift is equivalent. Floor keeps the choices near one canonical revolution.
    let mut turn = arithmetic.integer_quotient(&phase.lower, &period.lower)?;
    if phase.lower < 0.into() && &turn * &period.lower > phase.lower {
        turn -= 1;
    }
    let shift = arithmetic.integer_scale(period, &turn)?;
    Ok(arithmetic.subtract(phase, &shift)?)
}
fn gap(
    end: &num_bigint::BigInt,
    next: &num_bigint::BigInt,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(), ConicalReferenceFailure> {
    arithmetic.certificate_step()?;
    if end >= next {
        return Err(ConicalReferenceFailure::Bound {
            field: "nonoverlapping atlas reference phase under all periodic images",
            actual: arithmetic.upper_f64(end - next)?,
            limit: 0.,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests;
