// SPDX-License-Identifier: GPL-2.0-only
//! Bound the shared projection directly on the retained rational side rows.
use super::*;
mod direction;
#[cfg(test)]
mod tests;

pub(super) fn projected(
    pair: &ConicalActualAdjacent,
    sides: [&SideCapture; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<[[Interval; 2]; 2]> {
    let axis = direction::choose(pair, sides, arithmetic)?;
    let signs = [pair.u_slots[0] == 1, pair.u_slots[1] == 0];
    Ok([
        bounds(sides[0], &axis, signs[0], arithmetic)?,
        bounds(sides[1], &axis, signs[1], arithmetic)?,
    ])
}

fn bounds(
    side: &SideCapture,
    axis: &Point,
    forward: bool,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<[Interval; 2]> {
    let lower = minimum_step(&side.coordinates, axis, forward, arithmetic)?;
    let derivative = weighted_step(&lower, &side.source.weights, arithmetic)?;
    let transverse = ruling_bounds(&side.coordinates, axis, arithmetic)?;
    Ok([derivative, transverse])
}

fn weighted_step(
    step: &Interval,
    weights: &[f64],
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<Interval> {
    let weight = numeric::hull(&numeric::capture(weights, arithmetic)?);
    numeric::positive("retained projection weight", &weight, arithmetic)?;
    // The Bernstein pairwise derivative numerator is at least
    // degree * step * min(weights)^2 after removing u*(1-u).
    let ratio = arithmetic.divide(
        &numeric::lower_point(&weight),
        &numeric::upper_point(&weight),
    )?;
    let factor = numeric::square(&ratio, arithmetic)?;
    let derivative = arithmetic.multiply(step, &factor)?;
    Ok(arithmetic.fraction(&derivative, (weights.len() - 1) as u64, 1)?)
}

type Point = [Interval; 3];
fn minimum_step(
    rows: &[Vec<Point>; 2],
    axis: &Point,
    forward: bool,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<Interval> {
    let mut minimum: Option<Interval> = None;
    for [first, last] in rows.iter().flat_map(|row| row.array_windows()) {
        let (a, b) = if forward {
            (first, last)
        } else {
            (last, first)
        };
        let step = dot_difference(b, a, axis, arithmetic)?;
        if minimum.as_ref().is_none_or(|old| step.lower < old.lower) {
            minimum = Some(numeric::lower_point(&step));
        }
    }
    let minimum =
        minimum.ok_or_else(|| error::shape("retained side poles", 0, "at least two per row"))?;
    numeric::positive("retained side projected control step", &minimum, arithmetic)?;
    Ok(minimum)
}

fn ruling_bounds(
    rows: &[Vec<Point>; 2],
    axis: &Point,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<Interval> {
    // Common positive weights make the ruling a convex average of these differences.
    let bounds = rows[1]
        .iter()
        .zip(&rows[0])
        .map(|(last, first)| dot_difference(last, first, axis, arithmetic))
        .collect::<Checked<Vec<_>>>()?;
    Ok(numeric::hull(&bounds))
}

fn dot_difference(
    a: &Point,
    b: &Point,
    axis: &Point,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let mut dot = arithmetic.zero();
    for i in 0..3 {
        let difference = arithmetic.subtract(&a[i], &b[i])?;
        let term = arithmetic.multiply(&difference, &axis[i])?;
        dot = arithmetic.add(&dot, &term)?;
    }
    Ok(dot)
}
