// SPDX-License-Identifier: GPL-2.0-only
//! Stable cardinal branch names and exact signed ray classification.
use super::{
    AxialProjectionMode as Mode, AxialProjectionWitness, Checked,
    CylinderProjectionBranch as Branch, CylinderProjectionEmpty as Empty,
    CylinderProjectionError as Failure, CylinderProjectionPlan as Plan,
    affine::{AxialSource, Chart, Ratio, Vector, bounds, zero},
    poll,
    trim::{self, SourceInterval},
};
use crate::{gp::Direction3, predicates::roots::metric_bounds::placed_plane::exact_vector};
use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub(super) struct RadialRoot {
    pub branch: Branch,
    pub travel: Ratio,
    pub retained: Vec<SourceInterval>,
}

pub(super) fn plan(
    chart: Chart,
    source: AxialSource,
    mode: Mode,
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<Plan> {
    let Some((choice, signed_direction)) = direction(&chart, &source, mode)? else {
        return Ok(Plan::Empty(Empty::NoSupportRoot));
    };
    let direction = exact_vector(*signed_direction.coordinates(), 3)?;
    let mut roots = radial_roots(&chart, &source, choice, mode)?;
    if roots.is_empty() {
        return Ok(Plan::Empty(Empty::NoSupportRoot));
    }
    for root in &mut roots {
        poll(cancelled)?;
        root.retained = trim::retained(&chart, &source, root.branch)?;
    }
    let selected = trim::select(mode, &source, &roots, cancelled)?;
    if selected.is_empty() {
        return Ok(Plan::Empty(Empty::TrimMiss));
    }
    poll(cancelled)?;
    Ok(Plan::Regular(Box::new(AxialProjectionWitness {
        chart,
        source,
        direction,
        signed_direction,
        roots,
        selected,
        forward_only: matches!(mode, Mode::Forward(_)),
    })))
}

#[derive(Clone, Copy)]
struct DirectionChoice {
    column: usize,
    forward: bool,
}

fn direction(
    chart: &Chart,
    source: &AxialSource,
    mode: Mode,
) -> Checked<Option<(DirectionChoice, Direction3)>> {
    match mode {
        Mode::Forward(direction) | Mode::TwoSided(direction) => {
            let local = chart.direction_coordinates(direction)?;
            Ok(direction_choice(chart, source, &local)?.map(|choice| (choice, direction)))
        }
        Mode::Closest | Mode::Normal => {
            let Some(column) = normal_column(&source.radial)? else {
                return Err(Failure::Overlap);
            };
            let forward = source.radial[column].sign() == Ordering::Greater;
            Ok(Some((
                DirectionChoice { column, forward },
                cardinal_direction(chart, column, forward)?,
            )))
        }
    }
}

fn direction_choice(
    chart: &Chart,
    source: &AxialSource,
    [u, v, h]: &Vector,
) -> Checked<Option<DirectionChoice>> {
    let [p, q] = &source.radial;
    let a = u.multiply(u).add(&v.multiply(v));
    let c = p
        .multiply(p)
        .add(&q.multiply(q))
        .subtract(&chart.radius.multiply(&chart.radius));
    if zero(&a) {
        return match (zero(h), zero(&c)) {
            (true, _) => Err(Failure::Direction),
            (false, true) => Err(Failure::Overlap),
            (false, false) => Ok(None),
        };
    }
    let b = p.multiply(u).add(&q.multiply(v));
    match b.multiply(&b).subtract(&a.multiply(&c)).sign() {
        Ordering::Less => return Ok(None),
        Ordering::Equal => return Err(Failure::Tangent),
        Ordering::Greater => (),
    }
    if !zero(h) || a != Ratio::from_finite(1.) {
        return Err(Failure::Direction);
    }
    cardinal_choice([u, v], [p, q]).map(Some)
}

fn cardinal_choice(direction: [&Ratio; 2], offset: [&Ratio; 2]) -> Checked<DirectionChoice> {
    let [u_zero, v_zero] = direction.map(zero);
    let column = match (u_zero, v_zero) {
        (false, true) => 0,
        (true, false) => 1,
        _ => return Err(Failure::Direction),
    };
    if !zero(offset[1 - column]) {
        return Err(Failure::Direction);
    }
    Ok(DirectionChoice {
        column,
        forward: direction[column].sign() == Ordering::Greater,
    })
}

fn normal_column([u, v]: &[Ratio; 2]) -> Checked<Option<usize>> {
    match (zero(u), zero(v)) {
        (false, true) => Ok(Some(0)),
        (true, false) => Ok(Some(1)),
        (true, true) => Ok(None),
        (false, false) => Err(Failure::Direction),
    }
}

fn cardinal_direction(chart: &Chart, column: usize, forward: bool) -> Checked<Direction3> {
    let axis = &chart.axes[column];
    let side = if forward { 1. } else { -1. };
    let [x, y, z] = axis
        .each_ref()
        .map(|value| bounds(value).map(|bound| side * bound[0]));
    Direction3::new(x?, y?, z?).map_err(|_| Failure::Direction)
}

fn radial_roots(
    chart: &Chart,
    source: &AxialSource,
    choice: DirectionChoice,
    mode: Mode,
) -> Checked<Vec<RadialRoot>> {
    let pair = [true, false].map(|positive| root(chart, source, choice, positive));
    if matches!(mode, Mode::Forward(_)) && pair.iter().any(|root| zero(&root.travel)) {
        return Err(Failure::RayStartsOnSupport);
    }
    Ok(pair
        .into_iter()
        .filter(|root| !matches!(mode, Mode::Forward(_)) || root.travel.sign() == Ordering::Greater)
        .collect())
}

fn root(
    chart: &Chart,
    source: &AxialSource,
    choice: DirectionChoice,
    positive: bool,
) -> RadialRoot {
    let coordinate = if positive {
        chart.radius.clone()
    } else {
        chart.radius.negated()
    };
    let delta = coordinate.subtract(&source.radial[choice.column]);
    let travel = if choice.forward {
        delta
    } else {
        delta.negated()
    };
    let branch = match (choice.column, positive) {
        (0, true) => Branch::PositiveU,
        (0, false) => Branch::NegativeU,
        (_, true) => Branch::PositiveV,
        (_, false) => Branch::NegativeV,
    };
    RadialRoot {
        branch,
        travel,
        retained: Vec::new(),
    }
}
