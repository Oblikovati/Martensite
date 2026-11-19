// SPDX-License-Identifier: GPL-2.0-only
//! Capture once after complete transition binding; Markov operates on real issued C0 bounds.
use super::*;
use crate::predicates::roots::certificate_arithmetic::bernstein_curve::derivative;
mod frame;
mod markov;
pub(super) struct SideCapture {
    pub(super) source: ConicalActualSideSnapshot,
    pub(super) coordinates: [Vec<[Interval; 3]>; 2],
    pub(super) p: Interval,
    pub(super) slant_rate: Interval,
    pub(super) angle_denominator: Interval,
    pub(super) angle_rate: Interval,
    pub(super) radius: Interval,
    pub(super) depth_gap: Interval,
    pub(super) depths: [Interval; 2],
    pub(super) storage: [Interval; 2],
    pub(super) shifts: [Interval; 2],
    pub(super) derivative_error: Interval,
    pub(super) ruling_error: Interval,
    pub(super) du_error: Interval,
    pub(super) parent_storage: [Interval; 2],
    pub(super) cap_levels: [Interval; 2],
}
pub(super) fn side(
    query: &ConicalActualSideQuery<'_>,
    family: &ConicalActualPatchQuery<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<SideCapture> {
    let (row, gamma) = authorities(query, family, arithmetic.fractional_bits())?;
    let fits = frame::capture(&query.current, arithmetic)?;
    let stored = stored_weights(&query.current, arithmetic)?;
    let factor = markov::factor(&fits, &stored, arithmetic)?;
    let derivative_error = arithmetic.multiply(&numeric::max_magnitude(&row.storage), &factor)?;
    let ruling_error = arithmetic.add(&row.storage[0], &row.storage[1])?;
    let mut result = frame::assemble(
        query,
        row,
        gamma,
        fits,
        [derivative_error, ruling_error],
        arithmetic,
    )?;
    result.du_error = markov::du_bound(&result, &gamma.sine, &gamma.cosine, arithmetic)?;
    Ok(result)
}
fn stored_weights(
    query: &ConicalBoundaryQuery<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<Vec<Interval>> {
    let first = query.edges[0].stored_weights;
    let last = query.edges[1].stored_weights;
    if !first
        .iter()
        .map(|v| v.to_bits())
        .eq(last.iter().map(|v| v.to_bits()))
    {
        return Err(error::binding(
            "common actual weights",
            last.iter().map(|v| v.to_bits()).collect(),
            first.iter().map(|v| v.to_bits()).collect(),
        ));
    }
    let result = numeric::capture(first, arithmetic)?;
    numeric::positive("actual denominator", &numeric::hull(&result), arithmetic)?;
    Ok(result)
}
pub(super) struct FitCapture {
    p: Vec<Interval>,
    numerator: Vec<Interval>,
    weights: Vec<Interval>,
    p_hull: Interval,
    weight_hull: Interval,
    b: Interval,
    p_rate: Vec<Interval>,
}

fn authorities<'a>(
    query: &ConicalActualSideQuery<'a>,
    family: &ConicalActualPatchQuery<'a>,
    bits: u16,
) -> Outcome<(
    &'a super::super::boundary_transition::DirectedRowCapture,
    &'a GammaSpanCapture,
)> {
    let row = query
        .transition
        .checked_actual_capture(&query.current, query.domain, family.refinement, bits)
        .ok_or_else(|| transition_mismatch(query))?;
    if row.span != query.id || !row.matches_gamma(family.gamma) {
        return Err(error::binding(
            "transition family/span",
            vec![query.id.0],
            vec![row.span.0],
        ));
    }
    let gamma = family
        .gamma
        .span_capture(query.id)
        .ok_or_else(|| error::binding("family side", vec![query.id.0], family.gamma.span_ids()))?;
    ownership(query, gamma)?;
    Ok((row, gamma))
}
fn transition_mismatch(query: &ConicalActualSideQuery<'_>) -> ConicalActualPatchFailure {
    error::binding(
        "final transition source",
        BoundarySnapshot::capture(&query.current).bits,
        query.transition.boundary().input.bits.clone(),
    )
}
fn ownership(query: &ConicalActualSideQuery<'_>, gamma: &GammaSpanCapture) -> Outcome<()> {
    if query.member != gamma.member || query.caps != gamma.cap_ids {
        return Err(error::binding(
            "side ownership/rows",
            vec![query.member.0, query.caps[0].0, query.caps[1].0],
            vec![gamma.member.0, gamma.cap_ids[0].0, gamma.cap_ids[1].0],
        ));
    }
    Ok(())
}
