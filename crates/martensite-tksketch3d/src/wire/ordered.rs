// SPDX-License-Identifier: GPL-2.0-only
//! Shared directed connectivity and bounded exhaustive pair traversal.
use super::{WireClosure, WireFailure, WirePhase, poll};
use crate::PointId;

pub(super) fn check_chain<T>(
    uses: &[T],
    closure: WireClosure,
    ends: impl Fn(&T) -> [PointId; 2],
    disconnected: impl Fn([&T; 2], [PointId; 2]) -> WireFailure,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(), WireFailure> {
    let (first, last) = uses.first().zip(uses.last()).ok_or(WireFailure::Empty)?;
    for pair in uses.windows(2) {
        poll(cancelled, WirePhase::Resolve)?;
        let [a, b] = [&pair[0], &pair[1]];
        let points = [ends(a)[1], ends(b)[0]];
        if points[0] != points[1] {
            return Err(disconnected([a, b], points));
        }
    }
    let points = [ends(first)[0], ends(last)[1]];
    if (points[0] == points[1]) != (closure == WireClosure::Closed) {
        return Err(WireFailure::WrongClosure {
            expected: closure,
            points,
        });
    }
    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Adjacency {
    Separate,
    Consecutive,
    Wrap,
}

pub(super) fn check_pairs<T>(
    uses: &[T],
    closure: WireClosure,
    limit: usize,
    cancelled: &mut impl FnMut() -> bool,
    mut check: impl FnMut([&T; 2], Adjacency) -> Result<(), WireFailure>,
    exhausted: impl Fn([&T; 2]) -> WireFailure,
) -> Result<usize, WireFailure> {
    let mut checks = 0;
    for (left, first) in uses.iter().enumerate() {
        for (right, second) in uses.iter().enumerate().skip(left + 1) {
            poll(cancelled, WirePhase::Incidence)?;
            if checks == limit {
                return Err(exhausted([first, second]));
            }
            let adjacency = if right == left + 1 {
                Adjacency::Consecutive
            } else if left == 0 && right + 1 == uses.len() && closure == WireClosure::Closed {
                Adjacency::Wrap
            } else {
                Adjacency::Separate
            };
            check([first, second], adjacency)?;
            checks += 1;
        }
    }
    poll(cancelled, WirePhase::Incidence)?;
    Ok(checks)
}
