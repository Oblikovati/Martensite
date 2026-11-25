// SPDX-License-Identifier: GPL-2.0-only
use super::super::super::super::{certificate_arithmetic::ExactRatio, error::poll};
use super::super::{
    curve,
    publish::{self, BoundsSum},
    subinterval::ContributionBounds,
};
use super::*;

pub(super) struct State {
    pub options: IntegralProgramOptions,
    pub position: (Option<usize>, Option<usize>, Option<usize>),
    pub completed: usize,
    pub cells: u64,
}
impl State {
    pub(super) fn error(&self, cause: IntegralProgramFailure) -> IntegralProgramError {
        IntegralProgramError {
            requested_options: Box::new(self.options),
            loop_index: self.position.0,
            component_index: self.position.1,
            occurrence_index: self.position.2,
            completed_occurrences: self.completed,
            integration_subintervals: self.cells,
            cause,
        }
    }
    pub(super) fn numerical(&self, cause: RationalIntegralFailure) -> IntegralProgramError {
        self.error(IntegralProgramFailure::Numerical(Box::new(cause)))
    }
    fn poll(&self, cancelled: &mut impl FnMut() -> bool) -> Result<(), IntegralProgramError> {
        poll(cancelled).map_err(|cause| self.numerical(cause.into()))
    }
}
pub(super) fn certify(
    loops: &[IntegralProgramLoop<'_>],
    options: IntegralProgramOptions,
    face: bool,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<CertifiedIntegralProgram, IntegralProgramError> {
    let mut state = State {
        options,
        position: (None, None, None),
        completed: 0,
        cells: 0,
    };
    state.poll(cancelled)?;
    let count = admission::admit(loops, &mut state)?;
    let share = publish::width_share(options.integration, count, &ExactRatio::from_finite(1.));
    let sum = integrate(loops, face, &share, &mut state, cancelled)?;
    finish(sum, loops, face, state, cancelled)
}
fn integrate(
    loops: &[IntegralProgramLoop<'_>],
    face: bool,
    share: &ExactRatio,
    state: &mut State,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<BoundsSum, IntegralProgramError> {
    let mut total = BoundsSum::zero();
    for (index, current) in loops.iter().enumerate() {
        state.position.0 = Some(index);
        let first = state.completed;
        let sum = integrate_loop(current, share, state, cancelled)?;
        let (bounds, cells) = sum.into_parts();
        let bounds = if face {
            state.position = (Some(index), None, None);
            magnitude(bounds, first, state)?
        } else {
            bounds
        };
        total.accumulate(publish::reversed_bounds(bounds, face && index != 0), cells);
    }
    Ok(total)
}
fn integrate_loop(
    current: &IntegralProgramLoop<'_>,
    share: &ExactRatio,
    state: &mut State,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<BoundsSum, IntegralProgramError> {
    let mut sum = BoundsSum::zero();
    for (index, component) in current.components.iter().enumerate() {
        state.position.1 = Some(index);
        for occurrence in *component {
            integrate_occurrence(occurrence, share, state, &mut sum, cancelled)?;
        }
    }
    Ok(sum)
}
fn integrate_occurrence(
    occurrence: &OrientedMetricCurve<'_>,
    share: &ExactRatio,
    state: &mut State,
    sum: &mut BoundsSum,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(), IntegralProgramError> {
    state.position.2 = Some(state.completed);
    let remaining = state.options.max_total_subintervals - state.cells;
    let (result, cells) = curve::curve_integral_accounted(
        occurrence.curve,
        state.options.integration,
        share,
        remaining,
        cancelled,
    );
    state.cells += cells;
    let part = result.map_err(|cause| curve_failure(cause, remaining, state))?;
    sum.accumulate(
        publish::reversed_bounds(part.bounds, occurrence.reversed),
        cells,
    );
    state.completed += 1;
    Ok(())
}
fn curve_failure(
    cause: RationalIntegralFailure,
    remaining: u64,
    state: &State,
) -> IntegralProgramError {
    if matches!(cause, RationalIntegralFailure::WorkLimit { .. })
        && remaining < u64::from(state.options.integration.max_subintervals)
    {
        return state.error(IntegralProgramFailure::AggregateWork {
            limit: state.options.max_total_subintervals,
            cause: Box::new(cause),
        });
    }
    state.numerical(cause)
}
fn magnitude(
    bounds: ContributionBounds,
    first_curve: usize,
    state: &State,
) -> Result<ContributionBounds, IntegralProgramError> {
    let zero = ExactRatio::from_finite(0.);
    if bounds.lower > zero {
        return Ok(bounds);
    }
    if bounds.upper < zero {
        return Ok(publish::reversed_bounds(bounds, true));
    }
    Err(state.numerical(RationalIntegralFailure::IndeterminateOrientation { first_curve }))
}

fn finish(
    sum: BoundsSum,
    loops: &[IntegralProgramLoop<'_>],
    face: bool,
    mut state: State,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<CertifiedIntegralProgram, IntegralProgramError> {
    state.position = (None, None, None);
    state.poll(cancelled)?;
    let scalar = publish::rounded_scalar(sum, state.options.integration)
        .map_err(|cause| state.numerical(cause))?;
    state.poll(cancelled)?;
    Ok(CertifiedIntegralProgram {
        scalar,
        options: state.options,
        bits: binding::bits(loops, face),
        occurrences: state.completed,
    })
}
