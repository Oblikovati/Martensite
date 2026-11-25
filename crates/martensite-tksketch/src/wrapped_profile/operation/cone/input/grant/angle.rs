//! Bound the physical phase occupied by the captured polynomial control hulls.
use super::*;
use martensite_cmath::{atan2, sin};
use std::f64::consts::{PI, TAU};

pub(super) const BASE_PER_SPAN_LEVEL: u32 = 2048;
const PER_RADIAN_LEVEL: f64 = 10_000.;

pub(super) fn ceiling(
    originals: &[DevelopedRegionRequest],
    spans: u32,
    level: u8,
    semi_angle: f64,
) -> u32 {
    let sine = sin(semi_angle).next_down();
    if sine <= 0. {
        return INPUT_HARD_CEILING;
    }
    let angular = originals
        .iter()
        .flat_map(|region| &region.loops)
        .flatten()
        .map(|piece| width(&piece.poles))
        .fold(0., |sum, width| (sum + width).next_up());
    let physical = (angular / sine).next_up();
    let angular_credit = (physical * PER_RADIAN_LEVEL).next_up().ceil() as u32;
    let base = spans * BASE_PER_SPAN_LEVEL;
    base.saturating_add(angular_credit)
        .saturating_mul(u32::from(level) + 1)
        .clamp(ORIGINAL_CEILING, INPUT_HARD_CEILING)
}

fn width(poles: &[[f64; 2]]) -> f64 {
    let mut angles = [0.; 4];
    let Some(angles) = angles.get_mut(..poles.len()) else {
        return TAU.next_up();
    };
    if angles.is_empty() || poles.contains(&[0., 0.]) {
        return TAU.next_up();
    }
    for (angle, &[x, y]) in angles.iter_mut().zip(poles) {
        *angle = atan2(y, x);
    }
    angles.sort_unstable_by(f64::total_cmp);
    let Some((&first, &last)) = angles.first().zip(angles.last()) else {
        return TAU.next_up();
    };
    let wrap = (TAU.next_down() + gap(last, first)).next_down();
    let largest = angles
        .windows(2)
        .fold(wrap, |largest, pair| largest.max(gap(pair[0], pair[1])));
    let enclosing = (TAU.next_up() - largest).next_up();
    // A control hull inside an open angular half-plane stays in that convex sector.
    if enclosing < PI.next_down() {
        return enclosing;
    }
    TAU.next_up()
}

fn gap(from: f64, to: f64) -> f64 {
    (to.next_down() - from.next_up()).next_down()
}

#[cfg(test)]
mod tests;
