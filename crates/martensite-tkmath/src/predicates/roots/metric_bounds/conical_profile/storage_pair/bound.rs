// SPDX-License-Identifier: GPL-2.0-only
//! Convex Bernstein coefficient perturbations bound the actual rational functions.
use super::*;
pub(super) fn difference_bound(
    ideal: &[Interval],
    stored: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Option<Interval>, Failure> {
    if ideal.len() != stored.len() {
        return Ok(None);
    }
    let mut maximum: num_bigint::BigInt = 0.into();
    for (a, b) in ideal.iter().zip(stored) {
        maximum = maximum.max(arithmetic.subtract(a, b)?.magnitude());
    }
    Ok(Some(Interval {
        lower: 0.into(),
        upper: maximum,
    }))
}
fn aligned(ideal: &Coefficients, stored: &Coefficients) -> bool {
    let count = ideal[3].len();
    count > 0 && ideal.iter().chain(stored).all(|axis| axis.len() == count)
}
pub(super) fn conditioned(
    ideal: &Coefficients,
    stored: &Coefficients,
    context: &PairStorage,
    cutoff: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Option<Interval>, Failure> {
    if !aligned(ideal, stored) {
        return Ok(None);
    }
    let Some(delta_weight) = &context.denominators.error else {
        return Ok(None);
    };
    let Some((delta, magnitude)) = coordinate_bounds(ideal, stored, arithmetic)? else {
        return Ok(None);
    };
    // H/D-Hs/Ds = (H-Hs)/Ds + H*(Ds-D)/(D*Ds).
    let direct = arithmetic.divide(&delta, &context.denominators.stored_minimum)?;
    let weighted = arithmetic.multiply(&magnitude, delta_weight)?;
    let weighted = arithmetic.divide(&weighted, &context.denominators.product)?;
    let bound = arithmetic.add(&direct, &weighted)?;
    Ok((bound.upper <= cutoff.lower).then_some(bound))
}
fn coordinate_bounds(
    ideal: &Coefficients,
    stored: &Coefficients,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Option<(Interval, Interval)>, Failure> {
    let mut delta = arithmetic.zero();
    let mut magnitude = arithmetic.zero();
    for axis in 0..3 {
        let Some(difference) = difference_bound(&ideal[axis], &stored[axis], arithmetic)? else {
            return Ok(None);
        };
        delta = arithmetic.add(&delta, &difference)?;
        magnitude = arithmetic.add(&magnitude, &magnitude_interval(&ideal[axis]))?;
    }
    Ok(Some((delta, magnitude)))
}
