// SPDX-License-Identifier: GPL-2.0-only
//! Deterministic cold initialization for the flat normalized-dot authoring rows.
use super::{
    ConstraintFailure, binding::EquationBlock, metric::angle::nonsingular_coordinates, poll,
    residual::Equation,
};
use martensite_tkmath::gp::Point3;

type Coordinate = [f64; 3];

pub(super) fn initialize(
    values: &mut [f64],
    blocks: &[EquationBlock],
    fixed: &[(usize, f64)],
    rank_tolerance: f64,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(), ConstraintFailure> {
    // Separate truly flat directions before the first solve. The machine cap keeps a loose
    // rank policy from moving a regular authored seed. This guard never accepts geometry.
    let flatness = rank_tolerance.min(f64::EPSILON.sqrt());
    for block in blocks {
        match *block.source {
            Equation::Plane(normal) => {
                poll(cancelled)?;
                if let Some(line) = slots(block) {
                    seed_plane(values, line, normal, fixed, flatness);
                }
            }
            Equation::Perpendicular => {
                poll(cancelled)?;
                if let Some([a, b, c, d]) = slots(block) {
                    seed_perpendicular(values, [[a, b], [c, d]], fixed, flatness);
                }
            }
            _ => (),
        }
    }
    Ok(())
}

fn slots<const N: usize>(block: &EquationBlock) -> Option<[usize; N]> {
    let (points, remainder) = block.variables.as_chunks::<3>();
    if !remainder.is_empty() {
        return None;
    }
    let points: &[[usize; 3]; N] = points.try_into().ok()?;
    Some(points.map(|point| point[0]))
}

fn seed_plane(
    values: &mut [f64],
    line: [usize; 2],
    normal: Coordinate,
    fixed: &[(usize, f64)],
    flatness: f64,
) {
    let points = [line_points(values, line), [point([0.; 3]), point(normal)]];
    if nonsingular_coordinates(points, flatness) {
        return;
    }
    if let Some(end) = line.into_iter().rev().find(|&start| movable(fixed, start)) {
        bend(values, line, end);
    }
}

fn seed_perpendicular(
    values: &mut [f64],
    lines: [[usize; 2]; 2],
    fixed: &[(usize, f64)],
    flatness: f64,
) {
    if nonsingular_coordinates(lines.map(|line| line_points(values, line)), flatness) {
        return;
    }
    if let Some((line, endpoint)) = exclusive_endpoint(lines, fixed) {
        bend(values, line, endpoint);
    } else if let Some((start, other)) = shared_only(lines, fixed)
        && let Some(position) =
            perpendicular_corner(read(values, start), other.map(|i| read(values, i)))
    {
        values[start..start + 3].copy_from_slice(&position);
    }
}

fn exclusive_endpoint(
    [first, second]: [[usize; 2]; 2],
    fixed: &[(usize, f64)],
) -> Option<([usize; 2], usize)> {
    for (line, other) in [(second, first), (first, second)] {
        if let Some(endpoint) = line
            .into_iter()
            .rev()
            .find(|&p| movable(fixed, p) && !other.contains(&p))
        {
            return Some((line, endpoint));
        }
    }
    None
}

fn shared_only(
    [first, second]: [[usize; 2]; 2],
    fixed: &[(usize, f64)],
) -> Option<(usize, [usize; 2])> {
    let common = first
        .into_iter()
        .find(|&p| second.contains(&p) && movable(fixed, p))?;
    let a = first.into_iter().find(|&p| p != common)?;
    let b = second.into_iter().find(|&p| p != common && p != a)?;
    Some((common, [a, b]))
}

fn movable(fixed: &[(usize, f64)], start: usize) -> bool {
    (start..start + 3).all(|column| {
        fixed
            .binary_search_by_key(&column, |&(index, _)| index)
            .is_err()
    })
}

fn bend(values: &mut [f64], [start, end]: [usize; 2], moved: usize) {
    let displacement = difference(read(values, end), read(values, start));
    let axis = least_aligned(displacement);
    // A one-eighth transverse step separates the derivative without pinning the line length.
    let delta = 0.125 * magnitude(displacement);
    let component = &mut values[moved + axis];
    *component = inward_component(*component, delta);
}

fn inward_component(value: f64, amount: f64) -> f64 {
    if !amount.is_finite() {
        return value;
    }
    let direction = if value > 0. { -1. } else { 1. };
    let moved = value + direction * amount;
    if moved != value {
        moved
    } else if direction > 0. {
        value.next_up()
    } else {
        value.next_down()
    }
}

fn perpendicular_corner(prior: Coordinate, [a, b]: [Coordinate; 2]) -> Option<Coordinate> {
    // With only the shared point free, the perpendicular locus is the sphere whose
    // diameter joins the other endpoints. Use one equatorial point on that sphere.
    let half_a = a.map(|value| value * 0.5);
    let half_b = b.map(|value| value * 0.5);
    let center = std::array::from_fn(|i| half_a[i] + half_b[i]);
    let half_chord = difference(half_b, half_a);
    let side = sphere_side(half_chord)?;
    let sign = if dot(difference(prior, center), side) < 0. {
        -1.
    } else {
        1.
    };
    let seed = std::array::from_fn(|i| center[i] + sign * side[i]);
    seed.iter().all(|value| value.is_finite()).then_some(seed)
}

fn sphere_side(half_chord: Coordinate) -> Option<Coordinate> {
    let scale = magnitude(half_chord);
    if scale <= 0. || !scale.is_finite() {
        return None;
    }
    let chord = half_chord.map(|value| value / scale);
    let i = least_aligned(chord);
    let mut perpendicular = [0.; 3];
    perpendicular[(i + 1) % 3] = -chord[(i + 2) % 3];
    perpendicular[(i + 2) % 3] = chord[(i + 1) % 3];
    let factor = (dot(chord, chord) / dot(perpendicular, perpendicular)).sqrt();
    Some(perpendicular.map(|value| value * factor * scale))
}

fn line_points(values: &[f64], line: [usize; 2]) -> [Point3; 2] {
    line.map(|start| point(read(values, start)))
}

fn point([x, y, z]: Coordinate) -> Point3 {
    Point3::new(x, y, z)
}
fn read(values: &[f64], start: usize) -> Coordinate {
    [values[start], values[start + 1], values[start + 2]]
}
fn difference(a: Coordinate, b: Coordinate) -> Coordinate {
    std::array::from_fn(|i| a[i] - b[i])
}
fn magnitude([x, y, z]: Coordinate) -> f64 {
    x.abs().max(y.abs()).max(z.abs())
}
fn dot([x, y, z]: Coordinate, [a, b, c]: Coordinate) -> f64 {
    x * a + y * b + z * c
}

fn least_aligned(value: Coordinate) -> usize {
    let [x, y, z] = value.map(f64::abs);
    if x <= y && x <= z {
        0
    } else if y <= z {
        1
    } else {
        2
    }
}
