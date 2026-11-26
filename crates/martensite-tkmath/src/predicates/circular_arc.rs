// SPDX-License-Identifier: GPL-2.0-only
//! Exact metric checks on the three authored points of a circular arc.
use super::compare::compare_radical_gap;
use super::direction_band::compare_directions;
use super::{
    NonFiniteCoordinate, exact_coordinate, exact_vector3, triangle::dot3,
    triple_product::exact_cross,
};
use num_bigint::{BigInt, Sign};
use std::{cmp::Ordering, fmt};

/// Invalid inputs to an exact three-point circle query.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ArcTripleFailure {
    /// A coordinate or numeric limit was nonfinite.
    NonFinite(NonFiniteCoordinate),
    /// The supplied points do not determine a finite circle.
    Collinear,
}

impl fmt::Display for ArcTripleFailure {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite(source) => source.fmt(output),
            Self::Collinear => write!(
                output,
                "three noncollinear points are required for a circle"
            ),
        }
    }
}

impl std::error::Error for ArcTripleFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::NonFinite(source) => Some(source),
            Self::Collinear => None,
        }
    }
}

/// Exact branch, chord midpoint offset and radius evidence for a three-point circle.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArcTripleMetrics {
    chord_squared: BigInt,
    midpoint_projection_squared: BigInt,
    branch: Ordering,
    radius_numerator: BigInt,
    radius_denominator: BigInt,
}

impl ArcTripleMetrics {
    /// Capture endpoint and witness coordinates. Example: `ArcTripleMetrics::new([start, end, witness])?`.
    pub fn new([start, end, witness]: [[f64; 3]; 3]) -> Result<Self, ArcTripleFailure> {
        let start = point(start, 0)?;
        let end = point(end, 1)?;
        let witness = point(witness, 2)?;
        let first = difference(&witness, &start);
        let second = difference(&witness, &end);
        let chord = difference(&end, &start);
        Self::from_displacements(first, second, chord)
    }

    fn from_displacements(
        first: [BigInt; 3],
        second: [BigInt; 3],
        chord: [BigInt; 3],
    ) -> Result<Self, ArcTripleFailure> {
        let normal = exact_cross(&first, &second);
        let normal_squared = squared(&normal);
        if normal_squared.sign() == Sign::NoSign {
            return Err(ArcTripleFailure::Collinear);
        }
        let twice_offset = std::array::from_fn(|axis| &first[axis] + &second[axis]);
        let projection = dot3(&twice_offset, &chord);
        let chord_squared = squared(&chord);
        Ok(Self {
            branch: dot3(&first, &second).cmp(&BigInt::from(0)),
            midpoint_projection_squared: &projection * &projection,
            radius_numerator: &chord_squared * squared(&first) * squared(&second),
            radius_denominator: normal_squared << 2_usize,
            chord_squared,
        })
    }

    /// Compare the directed through-arc sweep with a semicircle. Example: `metrics.branch().is_lt()`.
    pub const fn branch(&self) -> Ordering {
        self.branch
    }

    /// Compare the witness offset from the midpoint normal plane with an absolute length limit.
    /// Example: `metrics.midpoint_offset(0.)?.is_eq()`.
    pub fn midpoint_offset(&self, limit: f64) -> Result<Ordering, ArcTripleFailure> {
        let limit = scalar(limit, 3)?;
        let allowed = (&limit * &limit * &self.chord_squared) << 2_usize;
        Ok(self.midpoint_projection_squared.cmp(&allowed))
    }

    /// Compare the absolute difference between exact circle radii with an absolute length limit.
    /// Example: `first.compare_radius_difference(&second, 1e-7)?`.
    pub fn compare_radius_difference(
        &self,
        other: &Self,
        limit: f64,
    ) -> Result<Ordering, ArcTripleFailure> {
        let limit = scalar(limit, 6)?;
        // Put both squared radii and the squared cap over the product of the denominators.
        let first = &self.radius_numerator * &other.radius_denominator;
        let second = &other.radius_numerator * &self.radius_denominator;
        let cap_squared = &limit * &limit * &self.radius_denominator * &other.radius_denominator;
        Ok(compare_radical_gap(first, second, cap_squared))
    }

    /// Check the exact circle radius against the closed band around a nonnegative target.
    /// Example: `metrics.radius_in_band(4., 1e-7)?`. The tolerance is an absolute length.
    pub fn radius_in_band(&self, target: f64, tolerance: f64) -> Result<bool, ArcTripleFailure> {
        let target = scalar(target, 3)?;
        let tolerance = scalar(tolerance.abs(), 4)?;
        if target.sign() == Sign::Minus {
            return Ok(false);
        }
        let lower = (&target - &tolerance).max(BigInt::from(0));
        let upper = target + tolerance;
        // In common coordinate-grid units the squared circumradius is
        // |SE|² |SW|² |EW|² / (4 |SW × EW|²).
        Ok(
            &lower * &lower * &self.radius_denominator <= self.radius_numerator
                && self.radius_numerator <= &upper * &upper * &self.radius_denominator,
        )
    }
}

/// Compare the exact outward through-arc tangent with the directed incoming line.
///
/// The first arc point is the selected endpoint; the other endpoint and the
/// through witness follow. The incoming line runs toward the selected endpoint.
/// A collinear arc is refused, and the caller admits a nonzero line.
/// Example: `compare_arc_endpoint_tangent_3d(arc, line, 1e-8)?`.
pub fn compare_arc_endpoint_tangent_3d(
    [selected, other, witness]: [[f64; 3]; 3],
    [from, to]: [[f64; 3]; 2],
    sine: f64,
) -> Result<(Ordering, Ordering), ArcTripleFailure> {
    let selected = point(selected, 0)?;
    let chord = difference(&point(other, 1)?, &selected);
    let through = difference(&point(witness, 2)?, &selected);
    let from = point(from, 3)?;
    let line = difference(&point(to, 4)?, &from);
    let sine = scalar(sine, 5)?;
    let tangent = endpoint_tangent(&chord, &through)?;
    Ok(compare_directions(&line, &tangent, &sine))
}

fn endpoint_tangent(
    chord: &[BigInt; 3],
    through: &[BigInt; 3],
) -> Result<[BigInt; 3], ArcTripleFailure> {
    let plane = exact_cross(chord, through);
    if plane.iter().all(|axis| axis.sign() == Sign::NoSign) {
        return Err(ArcTripleFailure::Collinear);
    }
    // Crossing the endpoint radial vector with the oriented circle normal
    // reduces to the two terms below, up to one strictly positive factor.
    let d2 = squared(chord);
    let w2 = squared(through);
    Ok(std::array::from_fn(|axis| {
        &d2 * &through[axis] - &w2 * &chord[axis]
    }))
}

fn point(values: [f64; 3], operand: usize) -> Result<[BigInt; 3], ArcTripleFailure> {
    exact_vector3(values, operand).map_err(ArcTripleFailure::NonFinite)
}

fn scalar(value: f64, operand: usize) -> Result<BigInt, ArcTripleFailure> {
    exact_coordinate(value, operand, 0).map_err(ArcTripleFailure::NonFinite)
}

fn difference(first: &[BigInt; 3], second: &[BigInt; 3]) -> [BigInt; 3] {
    std::array::from_fn(|axis| &first[axis] - &second[axis])
}

fn squared(value: &[BigInt; 3]) -> BigInt {
    dot3(value, value)
}

#[cfg(test)]
mod tests;
