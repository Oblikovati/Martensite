// SPDX-License-Identifier: GPL-2.0-only
//! Global Simpson node conditioning with the existing same-level local Bernstein fallback.
use super::*;
mod forward;
mod majorant;
#[cfg(test)]
mod tests;

pub(in super::super) fn integrate_span(
    poles: &[[f64; 2]],
    level: u8,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, SourceBoundFailure> {
    if level > 12 {
        return Err(SourceBoundFailure::InvalidLevel(level));
    }
    let (q, c) = source_polynomials(poles, arithmetic)?;
    let proof = match majorant::bound(&q, &c, arithmetic) {
        Ok(proof) => proof,
        Err(SourceBoundFailure::UnresolvedDistance(_)) => {
            return integrate_local_bounds(q, c, level, arithmetic);
        }
        Err(cause) => return Err(cause),
    };
    let panels = 1_u64 << level;
    let result = integrate_grid(&q, &c, panels, arithmetic).map_err(SourceBoundFailure::from)?;
    enclose_remainder(result, &proof.fourth_derivative_upper, panels, arithmetic)
        .map_err(SourceBoundFailure::from)
}
fn integrate_grid(
    q: &[Interval],
    c: &[Interval],
    panels: u64,
    arithmetic: &mut Arithmetic<'_>,
) -> Bounded<Interval> {
    let denominator = 2 * panels;
    let mut q = forward::ForwardDifference::new(q, denominator, arithmetic)?;
    let mut c = forward::ForwardDifference::new(c, denominator, arithmetic)?;
    let mut total = arithmetic.zero();
    for index in 0..=denominator {
        let value = node(q.value(), c.value(), arithmetic)?;
        let weight = simpson_weight(index, denominator);
        let weighted = arithmetic.fraction(&value, weight, 1)?;
        total = arithmetic.add(&total, &weighted)?;
        if index < denominator {
            q.advance(arithmetic)?;
            c.advance(arithmetic)?;
        }
    }
    arithmetic
        .fraction(&total, 1, 6 * panels)
        .map_err(Into::into)
}
fn simpson_weight(index: u64, last: u64) -> u64 {
    if index == 0 || index == last {
        return 1;
    }
    if index % 2 == 1 {
        return 4;
    }
    2
}
