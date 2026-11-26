// SPDX-License-Identifier: GPL-2.0-only
//! Validate the authored and evaluated radial intervals independently.
use martensite_tkernel::precision;
use martensite_tkernel::standard::Failure;
use martensite_tkernel::standard::exceptions::{
    construction_error, numeric_error, overflow, range_error, underflow,
};
use martensite_tkmath::predicates::compare_line_projections_2d;
use std::cmp::Ordering;
use std::f64::consts::TAU;

#[derive(Clone, Copy, Debug)]
pub(super) struct SpiralRadial {
    pub(super) initial_radius: f64,
    pub(super) radial_pitch: f64,
    pub(super) turns: f64,
    pub(super) last: f64,
    pub(super) rate: f64,
}

impl SpiralRadial {
    pub(super) fn new(initial_radius: f64, radial_pitch: f64, turns: f64) -> Result<Self, Failure> {
        let last = checked_last(turns)?;
        check_radius_and_pitch(initial_radius, radial_pitch)?;
        let rate = radial_pitch / TAU;
        if radial_pitch != 0.0 && rate == 0.0 {
            return Err(underflow(&format!(
                "planar spiral radial pitch {radial_pitch:e} needs a nonzero binary64 rate per radian"
            )));
        }
        check_end_radius(initial_radius, radial_pitch, turns, "authored turns")?;
        check_end_radius(initial_radius, rate, last, "radian interval")?;
        Ok(Self {
            initial_radius,
            radial_pitch,
            turns,
            last,
            rate,
        })
    }

    pub(super) fn radius_at(self, u: f64) -> Result<f64, Failure> {
        if !u.is_finite() || !(0.0..=self.last).contains(&u) {
            return Err(range_error(&format!(
                "planar spiral parameter {u} must be finite and within [0, {}]",
                self.last
            )));
        }
        Ok(self.rate.mul_add(u, self.initial_radius))
    }
}

fn checked_last(turns: f64) -> Result<f64, Failure> {
    if !turns.is_finite() || turns <= 0.0 {
        return Err(range_error(&format!(
            "planar spiral turns {turns} must be finite and positive"
        )));
    }
    let last = TAU * turns;
    if !last.is_finite() {
        return Err(overflow(&format!(
            "planar spiral turns {turns:e} require an end angle that fits binary64"
        )));
    }
    if precision::is_positive_infinite(last) {
        return Err(range_error(&format!(
            "planar spiral end angle {last:e} must be below the unbounded-parameter threshold {:e}",
            0.5 * precision::infinite()
        )));
    }
    Ok(last)
}

fn check_radius_and_pitch(initial_radius: f64, radial_pitch: f64) -> Result<(), Failure> {
    if !initial_radius.is_finite() || initial_radius <= 0.0 {
        return Err(construction_error(&format!(
            "planar spiral initial radius {initial_radius} must be finite and positive"
        )));
    }
    if !radial_pitch.is_finite() {
        return Err(construction_error(&format!(
            "planar spiral radial pitch {radial_pitch} must be finite"
        )));
    }
    Ok(())
}

fn check_end_radius(initial: f64, rate: f64, end: f64, basis: &str) -> Result<(), Failure> {
    // This exact dot orders the affine endpoint radius against each threshold.
    let compare = |bound| {
        compare_line_projections_2d([0., 0.], [1., end], [initial, rate], [bound, 0.])
            .map_err(|error| numeric_error(&error.to_string()))
    };
    if compare(0.0)? != Ordering::Greater {
        return Err(construction_error(&format!(
            "planar spiral endpoint radius at {basis} {end:e} with initial radius {initial:e} and rate {rate:e} must be positive"
        )));
    }
    if compare(f64::MAX)? == Ordering::Greater {
        return Err(overflow(&format!(
            "planar spiral endpoint radius at {basis} {end:e} with initial radius {initial:e} and rate {rate:e} exceeds binary64"
        )));
    }
    if rate.mul_add(end, initial) == 0.0 {
        return Err(underflow(&format!(
            "planar spiral endpoint radius at {basis} {end:e} with initial radius {initial:e} and rate {rate:e} rounds to zero"
        )));
    }
    Ok(())
}
