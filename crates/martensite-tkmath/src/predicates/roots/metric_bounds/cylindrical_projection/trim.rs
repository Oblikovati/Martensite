// SPDX-License-Identifier: GPL-2.0-only
//! Original axial source intervals and deterministic per-interval root selection.
use super::{
    AxialProjectionMode as Mode, Checked, CylinderProjectionBranch as Branch,
    CylinderProjectionError as Failure, ProjectionSourceInterval,
    affine::{AxialSource, Chart, Ratio, bounds},
    poll,
    roots::RadialRoot,
};
use std::{cmp::Ordering, f64::consts::PI};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct SourceInterval {
    pub ends: [Ratio; 2],
    pub included: [bool; 2],
}

impl SourceInterval {
    fn closed(a: &Ratio, b: &Ratio) -> Self {
        Self {
            ends: [a.clone(), b.clone()],
            included: [true; 2],
        }
    }

    pub fn enclose(&self) -> Checked<ProjectionSourceInterval> {
        Ok(ProjectionSourceInterval {
            endpoints: [bounds(&self.ends[0])?, bounds(&self.ends[1])?],
            included: self.included,
        })
    }

    fn contains(&self, value: &Ratio) -> bool {
        let [a, b] = &self.ends;
        (value > a || (self.included[0] && value == a))
            && (value < b || (self.included[1] && value == b))
    }
}

#[derive(Clone, Debug)]
pub(super) struct SelectedSpan {
    pub index: usize,
    pub interval: SourceInterval,
}

pub(super) fn angle(branch: Branch) -> [Ratio; 2] {
    let factor = match branch {
        Branch::PositiveU => return [Ratio::from_finite(0.), Ratio::from_finite(0.)],
        Branch::PositiveV => 0.5,
        Branch::NegativeU => 1.,
        Branch::NegativeV => 1.5,
    };
    let factor = Ratio::from_finite(factor);
    [PI, PI.next_up()].map(|endpoint| Ratio::from_finite(endpoint).multiply(&factor))
}

pub(super) fn retained(
    chart: &Chart,
    source: &AxialSource,
    branch: Branch,
) -> Checked<Vec<SourceInterval>> {
    let [start, end] = &source.range;
    let Some(hole) = chart.hole else {
        return Ok(vec![SourceInterval::closed(start, end)]);
    };
    let [lo, hi] = hole.angle.map(Ratio::from_finite);
    let [angle_lo, angle_hi] = angle(branch);
    if angle_hi < lo || angle_lo > hi {
        return Ok(vec![SourceInterval::closed(start, end)]);
    }
    if angle_lo <= lo || angle_hi >= hi {
        return Err(Failure::HoleAngularBoundary);
    }
    let mut excluded = hole
        .axial
        .map(|height| source.parameter_at(&Ratio::from_finite(height)));
    excluded.sort_unstable();
    subtract_hole(start, end, &excluded)
}

fn subtract_hole(
    start: &Ratio,
    end: &Ratio,
    [hole_lo, hole_hi]: &[Ratio; 2],
) -> Checked<Vec<SourceInterval>> {
    if end <= hole_lo || start >= hole_hi {
        return Ok(vec![SourceInterval::closed(start, end)]);
    }
    if start == hole_lo || end == hole_hi {
        return Err(Failure::PointOnly);
    }
    let mut retained = Vec::with_capacity(2);
    if start < hole_lo {
        retained.push(SourceInterval::closed(start, hole_lo));
    }
    if end > hole_hi {
        retained.push(SourceInterval::closed(hole_hi, end));
    }
    Ok(retained)
}

pub(super) fn select(
    mode: Mode,
    source: &AxialSource,
    roots: &[RadialRoot],
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<Vec<SelectedSpan>> {
    match mode {
        Mode::Normal => Ok(all(roots)),
        Mode::Closest => select_closest(source, roots),
        Mode::Forward(_) | Mode::TwoSided(_) => nearest(roots, cancelled),
    }
}

fn all(roots: &[RadialRoot]) -> Vec<SelectedSpan> {
    roots
        .iter()
        .enumerate()
        .flat_map(|(index, root)| {
            root.retained
                .iter()
                .cloned()
                .map(move |interval| SelectedSpan { index, interval })
        })
        .collect()
}

fn select_closest(source: &AxialSource, roots: &[RadialRoot]) -> Checked<Vec<SelectedSpan>> {
    // At this axial height the same-side radial point attains the full cylinder's
    // Euclidean minimum. The trimmed domain keeps that minimum only if it remains.
    let index = closest_root(roots, |_| true)?.ok_or(Failure::ClosestTrim)?;
    let complete = SourceInterval::closed(&source.range[0], &source.range[1]);
    if roots[index].retained.as_slice() != std::slice::from_ref(&complete) {
        return Err(Failure::ClosestTrim);
    }
    Ok(vec![SelectedSpan {
        index,
        interval: complete,
    }])
}

fn nearest(
    roots: &[RadialRoot],
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<Vec<SelectedSpan>> {
    let mut cuts: Vec<_> = roots
        .iter()
        .flat_map(|root| &root.retained)
        .flat_map(|interval| interval.ends.iter())
        .collect();
    cuts.sort_unstable();
    cuts.dedup();
    let mut selected = Vec::new();
    for ends in cuts.windows(2) {
        poll(cancelled)?;
        if let Some(span) = select_cell(roots, [ends[0], ends[1]])? {
            append(&mut selected, span);
        }
    }
    Ok(selected)
}

fn select_cell(roots: &[RadialRoot], [a, b]: [&Ratio; 2]) -> Checked<Option<SelectedSpan>> {
    let center = a.add(b).multiply(&Ratio::from_finite(0.5));
    let Some(index) = winner(roots, &center)? else {
        return Ok(None);
    };
    let included = [
        winner(roots, a)? == Some(index),
        winner(roots, b)? == Some(index),
    ];
    Ok(Some(SelectedSpan {
        index,
        interval: SourceInterval {
            ends: [a.clone(), b.clone()],
            included,
        },
    }))
}

fn append(selected: &mut Vec<SelectedSpan>, span: SelectedSpan) {
    if let Some(last) = selected.last_mut()
        && last.index == span.index
        && last.interval.ends[1] == span.interval.ends[0]
        && (last.interval.included[1] || span.interval.included[0])
    {
        last.interval.ends[1] = span.interval.ends[1].clone();
        last.interval.included[1] = span.interval.included[1];
    } else {
        selected.push(span);
    }
}

fn winner(roots: &[RadialRoot], parameter: &Ratio) -> Checked<Option<usize>> {
    closest_root(roots, |root| {
        root.retained
            .iter()
            .any(|interval| interval.contains(parameter))
    })
}

fn closest_root(
    roots: &[RadialRoot],
    eligible: impl Fn(&RadialRoot) -> bool,
) -> Checked<Option<usize>> {
    let mut choices = roots.iter().enumerate().filter(|(_, root)| eligible(root));
    let Some((a, root_a)) = choices.next() else {
        return Ok(None);
    };
    let Some((b, root_b)) = choices.next() else {
        return Ok(Some(a));
    };
    // Equal absolute travel is exactly the equal-square case even across signed roots.
    match root_a
        .travel
        .multiply(&root_a.travel)
        .cmp(&root_b.travel.multiply(&root_b.travel))
    {
        Ordering::Less => Ok(Some(a)),
        Ordering::Greater => Ok(Some(b)),
        Ordering::Equal => Err(Failure::RootTie),
    }
}
