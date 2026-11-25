// SPDX-License-Identifier: GPL-2.0-only
//! Bind two simple roots to disjoint parts of one original quadratic.
use super::*;

#[derive(Clone, Copy)]
pub(super) struct Event {
    pub parameter: [f64; 2],
    pub transition: SheetTransition,
    pub derivative: [[f64; 2]; 2],
}

pub(super) fn isolate_two(controls: [[f64; 2]; 3], lifts: [i8; 2]) -> Checked<[Event; 2]> {
    let [low, high] =
        [0, 1].map(|i| isolate(controls.map(|coefficient| coefficient[i]).as_slice(), 2));
    let [[lo0, lo1], [hi0, hi1]] = [pair(low?)?, pair(high?)?];
    let first = outer(lo0, hi0);
    let last = outer(lo1, hi1);
    if first[1] >= last[0] || first[0] <= 0. || last[1] >= 1. {
        return Err(incomplete(
            "disjoint original quadratic crossing intervals",
            format!("{first:?} {last:?}"),
        ));
    }
    let derivative = std::array::from_fn(|i| {
        let [lo, hi] = sub(controls[i + 1], controls[i]);
        [(2. * lo).next_down(), (2. * hi).next_up()]
    });
    Ok([
        directed_event(first, lifts[0], &controls, derivative)?,
        directed_event(last, -lifts[1], &controls, derivative)?,
    ])
}

fn pair(roots: Vec<[f64; 2]>) -> Checked<[[f64; 2]; 2]> {
    roots
        .try_into()
        .map_err(|roots| incomplete("two distinct simple original roots", format!("{roots:?}")))
}

fn outer(a: [f64; 2], b: [f64; 2]) -> [f64; 2] {
    const ROUNDING_PAD: f64 = 32. * f64::EPSILON;
    [
        (a[0].min(b[0]) - ROUNDING_PAD).next_down(),
        (a[1].max(b[1]) + ROUNDING_PAD).next_up(),
    ]
}

fn directed_event(
    parameter: [f64; 2],
    approach: i8,
    controls: &[[f64; 2]; 3],
    derivative: [[f64; 2]; 2],
) -> Checked<Event> {
    let actual = parameter.map(|t| sign(bezier_value(*controls, [t; 2]), 0.));
    if actual != [approach, -approach] {
        return Err(incomplete(
            "opposite certified signs around each original root",
            format!("{parameter:?}: {actual:?}"),
        ));
    }
    // Opposite signs in two disjoint intervals consume the two possible roots of the quadratic.
    let transition = if approach < 0 {
        SheetTransition::Increasing
    } else {
        SheetTransition::Decreasing
    };
    Ok(Event {
        parameter,
        transition,
        derivative,
    })
}

pub(super) fn speed(
    derivative: [[f64; 2]; 2],
    range: [f64; 2],
    transition: SheetTransition,
    scale: f64,
) -> Checked<f64> {
    let [a, b] = range.map(|t| bezier_value(derivative, [t; 2]));
    let derivative_lower = match transition {
        SheetTransition::Increasing => a[0].min(b[0]),
        SheetTransition::Decreasing => -a[1].max(b[1]),
    };
    cylinder_admission::require(
        derivative_lower.is_finite() && derivative_lower > 0.,
        "one transverse root over its fitted span",
        derivative_lower,
        "certified strict derivative sign on the entire original fitter interval",
    )?;
    Ok((derivative_lower / scale).next_down())
}

pub(super) fn bezier_value<const N: usize>(mut controls: [[f64; 2]; N], t: [f64; 2]) -> [f64; 2] {
    let opposite = sub([1.; 2], t);
    for remaining in (1..N).rev() {
        for i in 0..remaining {
            controls[i] = add(
                multiply(controls[i], opposite),
                multiply(controls[i + 1], t),
            );
        }
    }
    controls[0]
}

pub(super) fn multiply([a, b]: [f64; 2], [c, d]: [f64; 2]) -> [f64; 2] {
    let products = [a * c, a * d, b * c, b * d];
    let min = products.into_iter().fold(f64::INFINITY, f64::min);
    let max = products.into_iter().fold(f64::NEG_INFINITY, f64::max);
    [min.next_down(), max.next_up()]
}

fn add([a, b]: [f64; 2], [c, d]: [f64; 2]) -> [f64; 2] {
    [(a + c).next_down(), (b + d).next_up()]
}

fn sub([a, b]: [f64; 2], [c, d]: [f64; 2]) -> [f64; 2] {
    [(a - d).next_down(), (b - c).next_up()]
}
