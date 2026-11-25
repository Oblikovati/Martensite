// SPDX-License-Identifier: GPL-2.0-only
//! Deterministic cold initialization for the flat normalized-dot authoring rows.
mod support;

use super::{
    ConstraintFailure, binding::EquationBlock, metric::angle::nonsingular_coordinates, poll,
    residual::Equation,
};
use crate::{PointId, Snapshot};
use martensite_tkmath::gp::Point3;
use std::collections::BTreeMap;
use support::SeedSupport;

type Coordinate = [f64; 3];

#[derive(Clone, Copy)]
struct Seed {
    slot: usize,
    position: Coordinate,
}

#[derive(Clone, Copy)]
enum FlatRow {
    Plane {
        line: [usize; 2],
        normal: Coordinate,
    },
    Perpendicular([[usize; 2]; 2]),
}

pub(super) fn initialize(
    values: &mut [f64],
    blocks: &[EquationBlock],
    fixed: &[(usize, f64)],
    rank_tolerance: f64,
    geometry: &Snapshot,
    point_columns: &BTreeMap<PointId, usize>,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(), ConstraintFailure> {
    // The machine cap keeps a loose rank policy from moving a regular authored seed.
    let flatness = rank_tolerance.min(f64::EPSILON.sqrt());
    let mut support = None;
    for block in blocks {
        let Some(row) = FlatRow::from_block(block) else {
            continue;
        };
        poll(cancelled)?;
        if row.separated(values, None, flatness) {
            continue;
        }
        let index = match &mut support {
            Some(index) => index,
            empty @ None => empty.insert(SeedSupport::new(
                geometry,
                point_columns,
                blocks,
                cancelled,
            )?),
        };
        if let Some(seed) = row.choose(values, fixed, flatness, index, cancelled)? {
            values[seed.slot..seed.slot + 3].copy_from_slice(&seed.position);
        }
    }
    Ok(())
}

impl FlatRow {
    fn from_block(block: &EquationBlock) -> Option<Self> {
        match *block.source {
            Equation::Plane(normal) => Some(Self::Plane {
                line: slots(block)?,
                normal,
            }),
            Equation::Perpendicular => {
                let [a, b, c, d] = slots(block)?;
                Some(Self::Perpendicular([[a, b], [c, d]]))
            }
            _ => None,
        }
    }

    fn separated(self, values: &[f64], seed: Option<Seed>, flatness: f64) -> bool {
        let points = match self {
            Self::Plane { line, normal } => [
                line_points(values, line, seed),
                [point([0.; 3]), point(normal)],
            ],
            Self::Perpendicular(lines) => lines.map(|line| line_points(values, line, seed)),
        };
        nonsingular_coordinates(points, flatness)
    }

    fn choose(
        self,
        values: &[f64],
        fixed: &[(usize, f64)],
        flatness: f64,
        support: &SeedSupport<'_>,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Option<Seed>, ConstraintFailure> {
        let mut controls = self.exclusive(fixed);
        if controls.is_empty() {
            return self.choose_shared(values, fixed, flatness, support, cancelled);
        }
        // Prefer the free endpoint that disturbs the fewest lines. Equal degrees retain
        // operand and endpoint order. There are at most four endpoints, three axes and two signs.
        controls.sort_by_key(|&(_, slot)| support.degree(slot));
        for (line, slot) in controls {
            let displacement = difference(read(values, line[1]), read(values, line[0]));
            for position in bends(read(values, slot), displacement) {
                let seed = Seed { slot, position };
                if self.qualifies(values, seed, flatness, support, cancelled)? {
                    return Ok(Some(seed));
                }
            }
        }
        Ok(None)
    }

    fn exclusive(self, fixed: &[(usize, f64)]) -> Vec<([usize; 2], usize)> {
        let mut controls = Vec::new();
        match self {
            Self::Plane { line, .. } => controls.extend(
                line.into_iter()
                    .rev()
                    .filter(|&i| movable(fixed, i))
                    .map(|slot| (line, slot)),
            ),
            Self::Perpendicular([first, second]) => {
                for (line, other) in [(second, first), (first, second)] {
                    controls.extend(
                        line.into_iter()
                            .rev()
                            .filter(|&i| movable(fixed, i) && !other.contains(&i))
                            .map(|slot| (line, slot)),
                    );
                }
            }
        }
        controls
    }

    fn choose_shared(
        self,
        values: &[f64],
        fixed: &[(usize, f64)],
        flatness: f64,
        support: &SeedSupport<'_>,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Option<Seed>, ConstraintFailure> {
        let Self::Perpendicular(lines) = self else {
            return Ok(None);
        };
        let Some((slot, other)) = shared_only(lines, fixed) else {
            return Ok(None);
        };
        // The shared perpendicular locus is the sphere whose diameter is the other endpoints.
        let [a, b] = other.map(|i| read(values, i).map(|value| value * 0.5));
        let center = std::array::from_fn(|i| a[i] + b[i]);
        let half_chord = difference(b, a);
        for axis in ordered_axes(half_chord) {
            let Some(side) = sphere_side(half_chord, axis) else {
                continue;
            };
            let sign = if dot(difference(read(values, slot), center), side) < 0. {
                -1.
            } else {
                1.
            };
            for sign in [sign, -sign] {
                let seed = Seed {
                    slot,
                    position: std::array::from_fn(|i| center[i] + sign * side[i]),
                };
                if self.qualifies(values, seed, flatness, support, cancelled)? {
                    return Ok(Some(seed));
                }
            }
        }
        Ok(None)
    }

    fn qualifies(
        self,
        values: &[f64],
        seed: Seed,
        flatness: f64,
        support: &SeedSupport<'_>,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<bool, ConstraintFailure> {
        poll(cancelled)?;
        if seed.position.iter().any(|v| !v.is_finite())
            || !self.separated(values, Some(seed), flatness)
        {
            return Ok(false);
        }
        support.admits(values, seed.slot, seed.position, cancelled)
    }
}

fn slots<const N: usize>(block: &EquationBlock) -> Option<[usize; N]> {
    let (points, remainder) = block.variables.as_chunks::<3>();
    if !remainder.is_empty() {
        return None;
    }
    let points: &[[usize; 3]; N] = points.try_into().ok()?;
    Some(points.map(|point| point[0]))
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

fn bends(prior: Coordinate, displacement: Coordinate) -> impl Iterator<Item = Coordinate> {
    let amount = 0.125 * magnitude(displacement);
    ordered_axes(displacement)
        .into_iter()
        .flat_map(move |axis| {
            let inward = if prior[axis] > 0. { -1. } else { 1. };
            [inward, -inward].into_iter().map(move |sign| {
                let mut position = prior;
                position[axis] = stepped_component(prior[axis], amount, sign);
                position
            })
        })
}

fn stepped_component(value: f64, amount: f64, sign: f64) -> f64 {
    let moved = value + sign * amount;
    if moved != value {
        moved
    } else if sign > 0. {
        value.next_up()
    } else {
        value.next_down()
    }
}

fn sphere_side(half_chord: Coordinate, axis: usize) -> Option<Coordinate> {
    let scale = magnitude(half_chord);
    if scale <= 0. || !scale.is_finite() {
        return None;
    }
    let chord = half_chord.map(|value| value / scale);
    let mut perpendicular = [0.; 3];
    perpendicular[(axis + 1) % 3] = -chord[(axis + 2) % 3];
    perpendicular[(axis + 2) % 3] = chord[(axis + 1) % 3];
    let factor = (dot(chord, chord) / dot(perpendicular, perpendicular)).sqrt();
    factor
        .is_finite()
        .then(|| perpendicular.map(|value| value * factor * scale))
}

fn line_points(values: &[f64], line: [usize; 2], seed: Option<Seed>) -> [Point3; 2] {
    line.map(|slot| {
        point(match seed {
            Some(seed) if seed.slot == slot => seed.position,
            _ => read(values, slot),
        })
    })
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

fn ordered_axes(value: Coordinate) -> [usize; 3] {
    let magnitudes = value.map(f64::abs);
    let mut axes = [0, 1, 2];
    axes.sort_by(|&a, &b| magnitudes[a].total_cmp(&magnitudes[b]));
    axes
}
