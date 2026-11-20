// SPDX-License-Identifier: GPL-2.0-only
//! Midpoint restriction operates on weighted coordinates and the common denominator.
use super::*;
use crate::predicates::roots::certificate_arithmetic::bernstein_curve;
type Rows = [[Vec<Interval>; 3]; 2];

pub(super) struct Patch {
    rows: Rows,
    weights: Vec<Interval>,
    pub(super) hull: Hull,
}

impl Patch {
    pub(super) fn capture(
        coordinates: &[Vec<[Interval; 3]>; 2],
        weights: &[f64],
        arithmetic: &mut Arithmetic<'_>,
    ) -> Outcome<Self> {
        let weights = numeric::capture(weights, arithmetic)?;
        let rows = [
            weighted_row(&coordinates[0], &weights, arithmetic)?,
            weighted_row(&coordinates[1], &weights, arithmetic)?,
        ];
        Self::new(rows, weights, arithmetic)
    }

    fn new(rows: Rows, weights: Vec<Interval>, arithmetic: &mut Arithmetic<'_>) -> Outcome<Self> {
        let hull = bounds(&rows, &weights, arithmetic)?;
        Ok(Self {
            rows,
            weights,
            hull,
        })
    }

    pub(super) fn bisect(
        &self,
        along_u: bool,
        arithmetic: &mut Arithmetic<'_>,
    ) -> Outcome<[Self; 2]> {
        let (rows, weights) = if along_u {
            self.along_u(arithmetic)?
        } else {
            self.along_v(arithmetic)?
        };
        let [left, right] = rows;
        let [first, last] = weights;
        Ok([
            Self::new(left, first, arithmetic)?,
            Self::new(right, last, arithmetic)?,
        ])
    }

    fn along_u(&self, arithmetic: &mut Arithmetic<'_>) -> Checked<([Rows; 2], [Vec<Interval>; 2])> {
        let [first_a, first_b] = split_curve(&self.rows[0], arithmetic)?;
        let [last_a, last_b] = split_curve(&self.rows[1], arithmetic)?;
        let weights = bernstein_curve::bisect(&self.weights, arithmetic)?;
        Ok(([[first_a, last_a], [first_b, last_b]], weights))
    }

    fn along_v(&self, arithmetic: &mut Arithmetic<'_>) -> Checked<([Rows; 2], [Vec<Interval>; 2])> {
        let middle = [
            midpoint_row(&self.rows, 0, arithmetic)?,
            midpoint_row(&self.rows, 1, arithmetic)?,
            midpoint_row(&self.rows, 2, arithmetic)?,
        ];
        let [first, last] = &self.rows;
        let rows = [[first.clone(), middle.clone()], [middle, last.clone()]];
        Ok((rows, [self.weights.clone(), self.weights.clone()]))
    }
}

fn split_curve(
    curve: &[Vec<Interval>; 3],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[[Vec<Interval>; 3]; 2]> {
    let [xa, xb] = bernstein_curve::bisect(&curve[0], arithmetic)?;
    let [ya, yb] = bernstein_curve::bisect(&curve[1], arithmetic)?;
    let [za, zb] = bernstein_curve::bisect(&curve[2], arithmetic)?;
    Ok([[xa, ya, za], [xb, yb, zb]])
}
fn midpoint_row(
    rows: &Rows,
    axis: usize,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Vec<Interval>> {
    rows[0][axis]
        .iter()
        .zip(&rows[1][axis])
        .map(|(a, b)| {
            let sum = arithmetic.add(a, b)?;
            arithmetic.fraction(&sum, 1, 2)
        })
        .collect()
}
fn weighted_row(
    points: &[[Interval; 3]],
    weights: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[Vec<Interval>; 3]> {
    let mut row = std::array::from_fn(|_| Vec::with_capacity(weights.len()));
    for (point, weight) in points.iter().zip(weights) {
        for (axis, target) in row.iter_mut().enumerate() {
            target.push(arithmetic.multiply(&point[axis], weight)?);
        }
    }
    Ok(row)
}
fn bounds(rows: &Rows, weights: &[Interval], arithmetic: &mut Arithmetic<'_>) -> Outcome<Hull> {
    numeric::positive(
        "subpatch positive weights",
        &numeric::hull(weights),
        arithmetic,
    )?;
    let axes = [
        coordinate_bounds(rows, weights, 0, arithmetic)?,
        coordinate_bounds(rows, weights, 1, arithmetic)?,
        coordinate_bounds(rows, weights, 2, arithmetic)?,
    ];
    Ok(Hull {
        lower: axes.each_ref().map(|axis| axis.lower.clone()),
        upper: axes.map(|axis| axis.upper),
    })
}
fn coordinate_bounds(
    rows: &Rows,
    weights: &[Interval],
    axis: usize,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let poles = rows
        .iter()
        .flat_map(|row| row[axis].iter().zip(weights))
        .map(|(value, weight)| arithmetic.divide(value, weight))
        .collect::<Checked<Vec<_>>>()?;
    Ok(numeric::hull(&poles))
}
