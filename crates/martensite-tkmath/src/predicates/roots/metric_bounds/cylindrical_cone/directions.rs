// SPDX-License-Identifier: GPL-2.0-only
//! Direction heuristics reuse exact hull axes; acceptance always uses outward projections.
use super::*;
use crate::predicates::roots::certificate_arithmetic::{ExactRatio, rounding::outward_binary64};
use crate::predicates::roots::{DyadicBound, bezier_pair::hull_separation::hull_normals};
use std::cmp::Ordering;

pub(super) fn candidates(
    derivatives: &[Vec<[Interval; 2]>],
    bits: u16,
) -> impl Iterator<Item = [f64; 2]> + '_ {
    [[1., 0.], [-1., 0.], [0., 1.], [0., -1.]]
        .into_iter()
        .chain(std::iter::once_with(move || hull_candidates(derivatives, bits)).flatten())
}

fn hull_candidates(derivatives: &[Vec<[Interval; 2]>], bits: u16) -> Vec<[f64; 2]> {
    let mut points: Vec<_> = derivatives
        .iter()
        .flatten()
        .map(|p| p.each_ref().map(|v| midpoint(v, bits)))
        .collect();
    points.push([ExactRatio::from_finite(0.), ExactRatio::from_finite(0.)]);
    let mut result = Vec::new();
    for direction in hull_normals(&points) {
        if let Some(normal) = normalize(direction) {
            result.push(normal);
            result.push(normal.map(|v| -v));
        }
    }
    result
}

fn midpoint(value: &Interval, bits: u16) -> ExactRatio {
    let grid = (&value.lower + &value.upper) / 2;
    ExactRatio::from_dyadic(&DyadicBound::new(grid, -i32::from(bits)))
}

fn normalize(direction: [ExactRatio; 2]) -> Option<[f64; 2]> {
    let maximum = absolute(&direction[0]).max(absolute(&direction[1]));
    if maximum.sign() == Ordering::Equal {
        return None;
    }
    let x = outward_binary64(&direction[0].divide(&maximum))?[0];
    let y = outward_binary64(&direction[1].divide(&maximum))?[0];
    Some([x, y])
}

fn absolute(value: &ExactRatio) -> ExactRatio {
    if value.sign() == Ordering::Less {
        return value.negated();
    }
    value.clone()
}
