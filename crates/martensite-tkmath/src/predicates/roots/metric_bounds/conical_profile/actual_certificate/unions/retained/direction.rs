// SPDX-License-Identifier: GPL-2.0-only
//! Pick a direction from all oriented pole steps before checking exact inequalities.
use super::*;
use martensite_cmath::{atan2, cos, sin};
use std::f64::consts::TAU;

pub(super) fn choose(
    pair: &ConicalActualAdjacent,
    sides: [&SideCapture; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<Point> {
    let join = pair.u_slots[0];
    let ends = sides[0]
        .source
        .rows
        .each_ref()
        .map(|row| row[endpoint(join, row.len())]);
    let (basis_x, basis_y) = basis(difference(ends[1], ends[0])).ok_or_else(|| {
        error::shape("shared ruling projection frame", 0, "nonzero finite ruling")
    })?;
    let signs = [
        if join == 1 { 1. } else { -1. },
        if pair.u_slots[1] == 0 { 1. } else { -1. },
    ];
    let mut angles = Vec::new();
    append_angles(
        sides[0],
        signs[0],
        basis_x,
        basis_y,
        &mut angles,
        arithmetic,
    )?;
    append_angles(
        sides[1],
        signs[1],
        basis_x,
        basis_y,
        &mut angles,
        arithmetic,
    )?;
    let angle = narrowest_arc_midpoint(&mut angles);
    let axis = std::array::from_fn(|i| basis_x[i] * cos(angle) + basis_y[i] * sin(angle));
    capture(axis, arithmetic)
}

fn append_angles(
    side: &SideCapture,
    sign: f64,
    x: [f64; 3],
    y: [f64; 3],
    angles: &mut Vec<f64>,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<()> {
    for [a, b] in side.source.rows.iter().flat_map(|row| row.array_windows()) {
        arithmetic.certificate_step()?;
        let step = difference(*b, *a).map(|value| value * sign);
        angles.push(atan2(dot(step, y), dot(step, x)));
    }
    Ok(())
}

fn capture(axis: [f64; 3], arithmetic: &mut Arithmetic<'_>) -> Outcome<Point> {
    let [x, y, z] = axis;
    Ok([
        arithmetic.scalar(x)?,
        arithmetic.scalar(y)?,
        arithmetic.scalar(z)?,
    ])
}

pub(super) fn narrowest_arc_midpoint(angles: &mut [f64]) -> f64 {
    angles.sort_by(f64::total_cmp);
    let mut largest = angles[0] + TAU - angles[angles.len() - 1];
    let mut start = angles[0];
    for [a, b] in angles.array_windows() {
        if b - a > largest {
            largest = b - a;
            start = *b;
        }
    }
    start + (TAU - largest) / 2.
}

fn basis(v: [f64; 3]) -> Option<([f64; 3], [f64; 3])> {
    let scale = v.map(f64::abs).into_iter().fold(0., f64::max);
    if scale == 0. || !scale.is_finite() {
        return None;
    }
    let direction = unit(v.map(|value| value / scale));
    let axis = (0..3).min_by(|&a, &b| direction[a].abs().total_cmp(&direction[b].abs()))?;
    let coordinate = std::array::from_fn(|i| if i == axis { 1. } else { 0. });
    let x = unit(cross(direction, coordinate));
    Some((x, cross(direction, x)))
}

fn unit(v: [f64; 3]) -> [f64; 3] {
    let length = dot(v, v).sqrt();
    v.map(|value| value / length)
}
fn difference(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    std::array::from_fn(|i| a[i] - b[i])
}
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
