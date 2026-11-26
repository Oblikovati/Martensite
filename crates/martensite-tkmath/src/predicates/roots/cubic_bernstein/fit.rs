// SPDX-License-Identifier: GPL-2.0-only
//! Exact use and independent measurement of a fixed four-station affine operator.
use super::{
    CubicBernsteinFailure, DyadicBound, ExactCubicBernstein3, ExactRatio, ExactRows,
    NonFiniteCoordinate, nearest_binary64,
};
use crate::predicates::roots::certificate_arithmetic::rounding::{outward_binary64, outward_sqrt};
use std::{cmp::Ordering, fmt};

mod tangent;
pub use tangent::{CubicFitEnd, CubicFitTangent};

/// A finite four-station map with two derived Bernstein controls.
///
/// Each retained interior row operates relative to station zero. Its implicit first
/// coefficient is exactly one minus the sum of the other three, even when that
/// complement cannot be stored as binary64.
#[derive(Clone, Debug, PartialEq)]
pub struct CubicFitOperator {
    interiors: [[f64; 3]; 2],
    columns: ExactCubicBernstein3,
}

/// Rounded poles and an independent bound on the effect of materialization.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CubicFitMaterialization {
    /// Actually stored Bernstein controls, with exact input endpoints.
    pub poles: [[f64; 3]; 4],
    /// Maximum outward L1 pole rounding error, also bounding the positional
    /// difference from the real affine polynomial over the whole closed span.
    pub pole_rounding_upper: f64,
}

/// Cause of a refused bounded four-station computation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CubicFitNumericError {
    /// A point or basis coefficient is nonfinite.
    NonFinite(NonFiniteCoordinate),
    /// A requested parameter is outside the closed normalized interval.
    Parameter(CubicBernsteinFailure),
    /// A normalized station parameter interval has no positive length.
    StationOrder(usize),
    /// A chord between two successive stations has zero length.
    ZeroChord(usize),
    /// A required bound cannot be represented by a finite binary64 upper endpoint.
    Unbounded,
    /// A materialized control coordinate would round to infinity.
    PoleRange {
        /// Zero-based pole index.
        pole: usize,
        /// Zero-based coordinate index.
        axis: usize,
    },
}

impl fmt::Display for CubicFitNumericError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite(source) => source.fmt(formatter),
            Self::Parameter(source) => source.fmt(formatter),
            Self::StationOrder(index) => {
                write!(formatter, "parameter interval {index} must increase")
            }
            Self::ZeroChord(index) => {
                write!(formatter, "station chord {index} must have positive length")
            }
            Self::Unbounded => write!(
                formatter,
                "cubic fit measure must have a finite upper bound"
            ),
            Self::PoleRange { pole, axis } => write!(
                formatter,
                "cubic fit pole {pole}, axis {axis} must round to finite binary64"
            ),
        }
    }
}

impl std::error::Error for CubicFitNumericError {}

impl CubicFitOperator {
    /// Capture the last three coefficients of each interior pole row.
    /// The endpoint rows are exact identity. Example: `CubicFitOperator::new(interiors)?`.
    pub fn new(interiors: [[f64; 3]; 2]) -> Result<Self, CubicFitNumericError> {
        let [first, second] = interiors;
        let columns = ExactCubicBernstein3::new([[0.; 3], first, second, [0., 0., 1.]])
            .map_err(CubicFitNumericError::NonFinite)?;
        Ok(Self { interiors, columns })
    }

    /// Materialize each interior control with one nearest-even rounding per coordinate.
    /// Authored endpoints, including signed zeros, retain their bits.
    /// Example: `operator.materialize(stations)?`.
    pub fn materialize(
        &self,
        stations: [[f64; 3]; 4],
    ) -> Result<CubicFitMaterialization, CubicFitNumericError> {
        let station_values = dyadic_stations(stations)?;
        let [first, second] = self.interiors.map(|row| dyadic_point(row, 0));
        let (first, first_error) = materialize_pole(&station_values, &first?, 1)?;
        let (second, second_error) = materialize_pole(&station_values, &second?, 2)?;
        Ok(CubicFitMaterialization {
            poles: [stations[0], first, second, stations[3]],
            pole_rounding_upper: upper(&ExactRatio::from_dyadic(&first_error.max(second_error)))?,
        })
    }

    /// Return rounded D0, D1, D2 sensitivities to the four station positions.
    /// Each scalar multiplies a perturbation of the corresponding XYZ station.
    /// This differentiates the real affine map before materialization rounds its poles.
    /// Example: `operator.weight_jet(0.5)?[1][2]`.
    pub fn weight_jet(&self, parameter: f64) -> Result<[[f64; 4]; 3], CubicFitNumericError> {
        let values = self
            .columns
            .evaluate(parameter)
            .map_err(CubicFitNumericError::Parameter)?;
        let [d0, d1, d2] = [0, 1, 2].map(|order| round_weights(exact_weights(&values, order)));
        Ok([d0?, d1?, d2?])
    }

    /// Bound the maximum row L1 norm of the retained four-by-four affine operator.
    /// Example: `operator.gain_upper()?`.
    pub fn gain_upper(&self) -> Result<f64, CubicFitNumericError> {
        let [first, second] = self
            .interiors
            .map(|row| l1(complete_weights(row.map(ExactRatio::from_finite), 0)));
        upper(&first.max(second).max(ExactRatio::from_finite(1.)))
    }

    /// Bound the maximum row L1 error of station reproduction in the real affine model.
    /// Example: `operator.station_identity_upper([0., 0.25, 0.75, 1.])?`.
    pub fn station_identity_upper(
        &self,
        parameters: [f64; 4],
    ) -> Result<f64, CubicFitNumericError> {
        let mut worst = ExactRatio::from_finite(0.);
        for (station, parameter) in parameters.into_iter().enumerate() {
            let values = self
                .columns
                .evaluate(parameter)
                .map_err(CubicFitNumericError::Parameter)?;
            let mut weights = exact_weights(&values, 0);
            weights[station] = weights[station].subtract(&ExactRatio::from_finite(1.));
            worst = worst.max(l1(weights));
        }
        upper(&worst)
    }
}

impl ExactCubicBernstein3 {
    /// Bound the L1 miss from the exact stored polynomial at a finite target.
    /// This is also a conservative Euclidean-distance upper bound.
    /// Example: `curve.position_miss_upper(0.5, target)?`.
    pub fn position_miss_upper(
        &self,
        parameter: f64,
        target: [f64; 3],
    ) -> Result<f64, CubicFitNumericError> {
        let target = exact_point(target, 0)?;
        let values = self
            .evaluate(parameter)
            .map_err(CubicFitNumericError::Parameter)?;
        let differences = [0, 1, 2].map(|axis| values[axis].exact(0).subtract(&target[axis]));
        upper(&l1(differences))
    }
}

/// Full chord-per-parameter spread for the three adjacent station chords.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CubicChordDistortion {
    /// Zero-based adjacent chord index attaining the least speed.
    pub slowest: usize,
    /// Zero-based adjacent chord index attaining the greatest speed.
    pub fastest: usize,
    /// Outward upper bound on the greatest speed divided by the least speed.
    pub ratio_upper: f64,
}

/// Measure adjacent chord speeds over fixed parameters with a certified ratio bound.
/// Example: `cubic_chord_distortion(stations, [0., 0.25, 0.75, 1.])?`.
pub fn cubic_chord_distortion(
    stations: [[f64; 3]; 4],
    parameters: [f64; 4],
) -> Result<CubicChordDistortion, CubicFitNumericError> {
    let stations = exact_stations(stations)?;
    let parameters = exact_parameters(parameters)?;
    let [a, b, c] = [0, 1, 2].map(|index| squared_speed(&stations, &parameters, index));
    let speeds = [a?, b?, c?];
    let (slowest, fastest) = extrema(&speeds);
    let squared = speeds[fastest].divide(&speeds[slowest]);
    let ratio_upper = outward_sqrt(&squared).ok_or(CubicFitNumericError::Unbounded)?;
    Ok(CubicChordDistortion {
        slowest,
        fastest,
        ratio_upper,
    })
}

fn extrema(speeds: &[ExactRatio; 3]) -> (usize, usize) {
    let mut slowest = 0;
    let mut fastest = 0;
    for index in [1, 2] {
        if speeds[index] < speeds[slowest] {
            slowest = index;
        }
        if speeds[index] > speeds[fastest] {
            fastest = index;
        }
    }
    (slowest, fastest)
}

fn squared_speed(
    stations: &[[ExactRatio; 3]; 4],
    parameters: &[ExactRatio; 4],
    index: usize,
) -> Result<ExactRatio, CubicFitNumericError> {
    let step = parameters[index + 1].subtract(&parameters[index]);
    if step.sign() != Ordering::Greater {
        return Err(CubicFitNumericError::StationOrder(index));
    }
    let chord = [0, 1, 2].map(|axis| stations[index + 1][axis].subtract(&stations[index][axis]));
    let squared = chord.iter().fold(ExactRatio::from_finite(0.), |sum, x| {
        sum.add(&x.multiply(x))
    });
    if squared.sign() == Ordering::Equal {
        return Err(CubicFitNumericError::ZeroChord(index));
    }
    Ok(squared.divide(&step.multiply(&step)))
}

fn exact_parameters(parameters: [f64; 4]) -> Result<[ExactRatio; 4], CubicFitNumericError> {
    let [a, b, c, d] = parameters.map(|t| {
        if !(0. ..=1.).contains(&t) {
            return Err(CubicFitNumericError::Parameter(
                CubicBernsteinFailure::Parameter(t),
            ));
        }
        Ok(ExactRatio::from_finite(t))
    });
    Ok([a?, b?, c?, d?])
}

fn materialize_pole(
    stations: &[[DyadicBound; 3]; 4],
    row: &[DyadicBound; 3],
    pole: usize,
) -> Result<([f64; 3], DyadicBound), CubicFitNumericError> {
    let [x, y, z] = [0, 1, 2].map(|axis| {
        let position = stations[0][axis].sum(&affine_displacement(stations, row, axis));
        let rounded = nearest_binary64(&ExactRatio::from_dyadic(&position))
            .ok_or(CubicFitNumericError::PoleRange { pole, axis })?;
        let stored = dyadic_coordinate(rounded, pole, axis)?;
        Ok((rounded, position.difference(&stored).absolute()))
    });
    let (x, dx) = x?;
    let (y, dy) = y?;
    let (z, dz) = z?;
    Ok(([x, y, z], dx.sum(&dy).sum(&dz)))
}

fn affine_displacement(
    stations: &[[DyadicBound; 3]; 4],
    weights: &[DyadicBound; 3],
    axis: usize,
) -> DyadicBound {
    let origin = &stations[0][axis];
    weights
        .iter()
        .zip(&stations[1..])
        .fold(DyadicBound::new(0.into(), 0), |sum, (weight, station)| {
            sum.sum(&weight.product(&station[axis].difference(origin)))
        })
}

fn exact_stations(stations: [[f64; 3]; 4]) -> Result<[[ExactRatio; 3]; 4], CubicFitNumericError> {
    let [a, b, c, d] = [0, 1, 2, 3].map(|point| exact_point(stations[point], point));
    Ok([a?, b?, c?, d?])
}

fn exact_point(position: [f64; 3], point: usize) -> Result<[ExactRatio; 3], CubicFitNumericError> {
    Ok(dyadic_point(position, point)?
        .each_ref()
        .map(ExactRatio::from_dyadic))
}

fn dyadic_stations(stations: [[f64; 3]; 4]) -> Result<[[DyadicBound; 3]; 4], CubicFitNumericError> {
    let [a, b, c, d] = [0, 1, 2, 3].map(|point| dyadic_point(stations[point], point));
    Ok([a?, b?, c?, d?])
}

fn dyadic_point(
    position: [f64; 3],
    point: usize,
) -> Result<[DyadicBound; 3], CubicFitNumericError> {
    let [x, y, z] = [0, 1, 2].map(|axis| dyadic_coordinate(position[axis], point, axis));
    Ok([x?, y?, z?])
}

fn dyadic_coordinate(
    value: f64,
    point: usize,
    axis: usize,
) -> Result<DyadicBound, CubicFitNumericError> {
    DyadicBound::from_f64(value)
        .map_err(|_| CubicFitNumericError::NonFinite(NonFiniteCoordinate { point, axis, value }))
}

fn exact_weights(values: &[ExactRows; 3], order: usize) -> [ExactRatio; 4] {
    complete_weights(values.each_ref().map(|axis| axis.exact(order)), order)
}

fn complete_weights(weights: [ExactRatio; 3], order: usize) -> [ExactRatio; 4] {
    let basis_sum = DyadicBound::new(u8::from(order == 0).into(), 0);
    // A finite coefficient and every stored cubic evaluation have power-of-two denominators.
    // Align their grids directly to avoid general rational reduction of the same weights.
    let first = weights.iter().fold(basis_sum, |sum, weight| {
        let exponent = 1 - weight.denominator().bits() as i32;
        let term = DyadicBound::new(weight.numerator().clone(), exponent);
        sum.difference(&term)
    });
    let [second, third, fourth] = weights;
    [ExactRatio::from_dyadic(&first), second, third, fourth]
}

fn round_weights(weights: [ExactRatio; 4]) -> Result<[f64; 4], CubicFitNumericError> {
    let [a, b, c, d] =
        weights.map(|weight| nearest_binary64(&weight).ok_or(CubicFitNumericError::Unbounded));
    Ok([a?, b?, c?, d?])
}

fn l1<const N: usize>(values: [ExactRatio; N]) -> ExactRatio {
    values
        .into_iter()
        .fold(ExactRatio::from_finite(0.), |sum, value| {
            if value.sign() == Ordering::Less {
                sum.subtract(&value)
            } else {
                sum.add(&value)
            }
        })
}

fn upper(value: &ExactRatio) -> Result<f64, CubicFitNumericError> {
    outward_binary64(value)
        .map(|bounds| bounds[1])
        .ok_or(CubicFitNumericError::Unbounded)
}

#[cfg(test)]
mod tests;
