// SPDX-License-Identifier: GPL-2.0-only
//! Conditional Simpson enclosure for one admitted polynomial boundary span.
use super::inverse::*;
use super::*;
use crate::predicates::roots::certificate_arithmetic::bernstein_curve::bisect;

pub(super) type Cell = (Vec<Interval>, Vec<Interval>, u8);

pub(super) fn integrate_span(
    poles: &[[f64; 2]],
    level: u8,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, SourceBoundFailure> {
    if level > 12 {
        return Err(SourceBoundFailure::InvalidLevel(level));
    }
    let (q, c) = source_polynomials(poles, arithmetic)?;
    let proof = match bound_polynomials(&q, &c, arithmetic) {
        Ok(proof) => proof,
        Err(SourceBoundFailure::UnresolvedDistance(_)) => {
            return integrate_local_bounds(q, c, level, arithmetic);
        }
        Err(cause) => return Err(cause),
    };
    let panels = 1_u64 << level;
    let result =
        integrate_panels(q, c, level, panels, arithmetic).map_err(SourceBoundFailure::from)?;
    enclose_remainder(result, &proof.fourth_derivative_upper, panels, arithmetic)
        .map_err(SourceBoundFailure::from)
}

// Each local derivative is with respect to the cell parameter, so the integral
// and its Simpson remainder both receive the same global cell-width factor.
pub(super) fn integrate_local_bounds(
    q: Vec<Interval>,
    c: Vec<Interval>,
    level: u8,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, SourceBoundFailure> {
    let panels = 1_u64 << level;
    let mut pending = vec![(q, c, level)];
    let mut total = arithmetic.zero();
    while let Some((q, c, remaining)) = pending.pop() {
        if remaining > 0 {
            split_cell(&mut pending, &q, &c, remaining, arithmetic)
                .map_err(SourceBoundFailure::from)?;
            continue;
        }
        let panel = local_panel(&q, &c, panels, arithmetic)?;
        total = arithmetic
            .add(&total, &panel)
            .map_err(SourceBoundFailure::from)?;
    }
    Ok(total)
}

pub(super) fn local_panel(
    q: &[Interval],
    c: &[Interval],
    panels: u64,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, SourceBoundFailure> {
    let q_lower = lower(q);
    admit_local_radius(&q_lower, arithmetic.fractional_bits())?;
    let numerator = fourth_numerator(q, c, arithmetic).map_err(SourceBoundFailure::from)?;
    let derivative = derivative_upper(&q_lower, &magnitude(&numerator), arithmetic)
        .map_err(SourceBoundFailure::from)?;
    let estimate = simpson_panel(q, c, panels, arithmetic).map_err(SourceBoundFailure::from)?;
    let error = arithmetic
        .fraction(&derivative, 1, 2880 * panels)
        .map_err(SourceBoundFailure::from)?;
    let remainder = Interval {
        lower: -&error.upper,
        upper: error.upper,
    };
    arithmetic
        .add(&estimate, &remainder)
        .map_err(SourceBoundFailure::from)
}
pub(super) fn integrate_panels(
    q: Vec<Interval>,
    c: Vec<Interval>,
    level: u8,
    panels: u64,
    arithmetic: &mut Arithmetic<'_>,
) -> Bounded<Interval> {
    let mut pending: Vec<Cell> = vec![(q, c, level)];
    let mut result = arithmetic.zero();
    while let Some((q, c, remaining)) = pending.pop() {
        if remaining > 0 {
            split_cell(&mut pending, &q, &c, remaining, arithmetic)?;
            continue;
        }
        let panel = simpson_panel(&q, &c, panels, arithmetic)?;
        result = arithmetic.add(&result, &panel)?;
    }
    Ok(result)
}
pub(super) fn split_cell(
    pending: &mut Vec<Cell>,
    q: &[Interval],
    c: &[Interval],
    level: u8,
    arithmetic: &mut Arithmetic<'_>,
) -> Bounded<()> {
    let [left_q, right_q] = bisect(q, arithmetic)?;
    let [left_c, right_c] = bisect(c, arithmetic)?;
    pending.push((right_q, right_c, level - 1));
    pending.push((left_q, left_c, level - 1));
    Ok(())
}
pub(super) fn simpson_panel(
    q: &[Interval],
    c: &[Interval],
    panels: u64,
    arithmetic: &mut Arithmetic<'_>,
) -> Bounded<Interval> {
    let [half_q, _] = bisect(q, arithmetic)?;
    let [half_c, _] = bisect(c, arithmetic)?;
    let first = node(&q[0], &c[0], arithmetic)?;
    let middle = node(
        &half_q[half_q.len() - 1],
        &half_c[half_c.len() - 1],
        arithmetic,
    )?;
    let last = node(&q[q.len() - 1], &c[c.len() - 1], arithmetic)?;
    let weighted = arithmetic.fraction(&middle, 4, 1)?;
    let start = arithmetic.add(&first, &weighted)?;
    let total = arithmetic.add(&start, &last)?;
    arithmetic
        .fraction(&total, 1, 6 * panels)
        .map_err(Into::into)
}
pub(super) fn node(
    q: &Interval,
    c: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Bounded<Interval> {
    let root = arithmetic.positive_sqrt(q)?;
    arithmetic.divide(c, &root).map_err(Into::into)
}
pub(super) fn enclose_remainder(
    estimate: Interval,
    derivative: &Interval,
    panels: u64,
    arithmetic: &mut Arithmetic<'_>,
) -> Bounded<Interval> {
    let denominator = 2880 * panels.pow(4);
    let error = arithmetic.fraction(derivative, 1, denominator)?;
    let enclosure = Interval {
        lower: -&error.upper,
        upper: error.upper,
    };
    arithmetic.add(&estimate, &enclosure).map_err(Into::into)
}

pub use super::volume_boundary::ConicalAuthoredContour;
pub use super::volume_boundary::ConicalAuthoredFlux;
pub use super::volume_boundary::ConicalAuthoredFluxError;
pub use super::volume_boundary::ConicalAuthoredFluxQuery;
pub use super::volume_boundary::ConicalAuthoredFluxStage;
pub use super::volume_boundary::certify_conical_authored_flux;

use crate::predicates::roots::certificate_arithmetic::bernstein_bounds::magnitude;

fn admit_local_radius(
    q_lower: &num_bigint::BigInt,
    fractional_bits: u16,
) -> Result<(), SourceBoundFailure> {
    if *q_lower <= 0.into() {
        return Err(SourceBoundFailure::UnresolvedDistance(
            crate::predicates::roots::DyadicBound::new(
                q_lower.clone(),
                -i32::from(fractional_bits),
            ),
        ));
    }
    Ok(())
}
