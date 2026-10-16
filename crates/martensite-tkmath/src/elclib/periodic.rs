//! Period folding shared by elementary-curve evaluator and elementary-surface evaluator.
use martensite_tkernel::{precision, standard::real};

const TWO_PI: f64 = std::f64::consts::PI + std::f64::consts::PI;

/// Folds an angle into `[0, 2·PI]`, keeping values within machine epsilon below zero and
/// exactly `2·PI` (the closing seam) as they are, then clamping residual negatives to zero.
/// Inputs are the bounded outputs of angle functions, so the loops terminate quickly.
pub(crate) fn normalize_angle(angle: f64) -> f64 {
    let mut angle = angle;
    while angle < -precision::computational() {
        angle += TWO_PI;
    }
    // 1 + DBL_MIN rounds to 1, so the bound is exactly 2·PI, as in the expression.
    while angle > TWO_PI * (1.0 + f64::MIN_POSITIVE) {
        angle -= TWO_PI;
    }
    if angle < 0.0 { 0.0 } else { angle }
}

/// Returns `u` shifted by whole periods into `[first, first + period)`; infinite inputs or a
/// period below `Epsilon(last)` return `u` unchanged. `last` itself is never returned, but
/// rounding of `last - first` can make the effective period differ from the intended one.
/// Example: `assert_eq!(in_period(4.7, 1.5, 4.5), 1.7000000000000002)`.
pub fn in_period(u: f64, first: f64, last: f64) -> f64 {
    if precision::is_infinite(u) || precision::is_infinite(first) || precision::is_infinite(last) {
        // Avoids an overflow in the period arithmetic, as the guard does.
        return u;
    }
    let period = last - first;
    if period < real::epsilon(last) {
        return u;
    }
    let shifted = u + period * ((first - u) / period).ceil();
    // std::max(first, shifted) returns `first` unless `first < shifted`.
    if first < shifted { shifted } else { first }
}

/// Adjusts `u1` into `[first, last)` and then `u2` into `(u1, u1 + period]`, moving either
/// value by one more period when it lies within `precision` of the upper end; infinite bounds
/// or a period below `Epsilon(last)` return `(first, last)`.
/// Example: `adjust_periodic(0.0, 2.0*PI, 1e-7, -0.5, 0.7)` returns `(2·PI-0.5, 0.7+2·PI)`.
pub fn adjust_periodic(first: f64, last: f64, precision: f64, u1: f64, u2: f64) -> (f64, f64) {
    if precision::is_infinite(first) || precision::is_infinite(last) {
        return (first, last);
    }
    let period = last - first;
    if period < real::epsilon(last) {
        // Avoids an overflow in the period arithmetic.
        return (first, last);
    }
    let mut u1 = u1 - ((u1 - first) / period).floor() * period;
    if last - u1 < precision {
        u1 -= period;
    }
    let mut u2 = u2 - ((u2 - u1) / period).floor() * period;
    if u2 - u1 < precision {
        u2 += period;
    }
    (u1, u2)
}
