// SPDX-License-Identifier: GPL-2.0-only
//! At most one admission plus 64 refinement queries; the existing limit never rises.
use super::{conformance::ResidualInput, types::*};
use martensite_tkmath::predicates::roots::{
    DyadicBound,
    metric_bounds::{ResidualDecision, certify_planar_residual},
};

pub(super) fn certified_upper(
    input: &ResidualInput,
    index: usize,
    limit: f64,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(DyadicBound, usize), BindingFailure> {
    certify_decisions(index, limit, |radius| {
        Ok(certify_planar_residual(
            input.query(radius),
            &mut *cancelled,
        )?)
    })
}
pub(super) fn certify_decisions(
    index: usize,
    limit: f64,
    mut decide: impl FnMut(f64) -> Result<ResidualDecision, BindingFailure>,
) -> Result<(DyadicBound, usize), BindingFailure> {
    let upper = admission(index, limit, &mut decide)?;
    let zero = DyadicBound::from_f64(0.).expect("finite zero");
    if upper == zero {
        return Ok((zero, 1));
    }
    if matches!(decide(0.)?, ResidualDecision::Within { .. }) {
        return Ok((zero, 2));
    }
    refine(limit, upper, &mut decide)
}
fn admission(
    index: usize,
    limit: f64,
    decide: &mut impl FnMut(f64) -> Result<ResidualDecision, BindingFailure>,
) -> Result<DyadicBound, BindingFailure> {
    match decide(limit)? {
        ResidualDecision::Within {
            proven_squared_upper,
        } => Ok(proven_squared_upper),
        decision => Err(BindingFailure::Conformance {
            occurrence: index,
            limit,
            decision,
        }),
    }
}
fn refine(
    limit: f64,
    upper_bound: DyadicBound,
    decide: &mut impl FnMut(f64) -> Result<ResidualDecision, BindingFailure>,
) -> Result<(DyadicBound, usize), BindingFailure> {
    let mut state = BoundSearch {
        lower: 0,
        upper: limit.to_bits(),
        upper_bound,
        work: 2,
    };
    while state.upper - state.lower > 1 {
        let middle = state.lower + (state.upper - state.lower) / 2;
        state.advance(middle, decide(f64::from_bits(middle))?);
    }
    Ok((state.upper_bound, state.work))
}
struct BoundSearch {
    lower: u64,
    upper: u64,
    upper_bound: DyadicBound,
    work: usize,
}
impl BoundSearch {
    fn advance(&mut self, middle: u64, decision: ResidualDecision) {
        self.work += 1;
        match decision {
            ResidualDecision::Within {
                proven_squared_upper,
            } => {
                self.upper = middle;
                self.upper_bound = proven_squared_upper;
            }
            ResidualDecision::Exceeds { .. } => self.lower = middle,
        }
    }
}
