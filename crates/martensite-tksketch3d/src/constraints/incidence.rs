// SPDX-License-Identifier: GPL-2.0-only
//! Endpoint identity determines which incidence equations are independent.
use super::ParallelDirection;
use crate::LineGeometry;

#[derive(Clone, Copy)]
pub(super) enum SharedEnds {
    None,
    One,
    Both(ParallelDirection),
}

pub(super) fn shared_ends([a, b]: [LineGeometry; 2]) -> SharedEnds {
    let [from, to] = a.line.endpoints;
    let other = b.line.endpoints;
    match (other.contains(&from), other.contains(&to)) {
        (false, false) => SharedEnds::None,
        (true, true) if [from, to] == other => SharedEnds::Both(ParallelDirection::Same),
        (true, true) => SharedEnds::Both(ParallelDirection::Opposite),
        _ => SharedEnds::One,
    }
}
