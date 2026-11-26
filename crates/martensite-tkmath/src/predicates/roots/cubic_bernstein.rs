// SPDX-License-Identifier: GPL-2.0-only
//! Fixed-size exact polynomial evaluation and final binary64 rounding.
//! A common pole grid uses at most 2098 significand bits, and each of the three
//! blend levels adds at most 1074 bits. Both differentiated rows are shorter.
use super::{
    DyadicBound,
    certificate_arithmetic::{ExactRatio, rounding::nearest_binary64},
};
use crate::predicates::{NonFiniteCoordinate, binary64_parts};
use num_bigint::BigInt;
use std::{cmp::Ordering, error::Error, fmt};

mod fit;
pub use fit::{
    CubicChordDistortion, CubicFitEnd, CubicFitMaterialization, CubicFitNumericError,
    CubicFitOperator, CubicFitTangent, QuadraticFitMaterialization, cubic_chord_distortion,
    evaluate_fit_quadratic, materialize_fit_quadratic, quadratic_fit_chord_distortion,
};

mod endpoint;
pub use endpoint::{
    CubicEndJet, CubicEndJetFailure, CubicEndStencil, RoundedCubicEndJet, TangentDirectionBand,
};

/// Owned exact coordinates of one unweighted spatial cubic Bernstein polynomial.
///
/// The degree and binary64 source range bound work and intermediate integer size.
/// No floating arithmetic is used between coefficient capture and final rounding.
#[derive(Clone, Debug, PartialEq)]
pub struct ExactCubicBernstein3 {
    coordinates: [Coordinate; 3],
}

/// D0, D1 and D2 in the normalized Bernstein parameter, rounded independently.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CubicBernsteinJet {
    /// Final rounded position.
    pub position: [f64; 3],
    /// Final rounded first derivative.
    pub first: [f64; 3],
    /// Final rounded second derivative.
    pub second: [f64; 3],
}

/// A request or exact result outside the supported binary64 domain.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CubicBernsteinFailure {
    /// The parameter must be finite and inside the closed normalized span.
    Parameter(f64),
    /// An exact coordinate would round to infinity.
    Unrepresentable {
        /// Requested derivative order, zero for the position.
        order: usize,
        /// Zero-based coordinate index.
        axis: usize,
        /// Sign of the exact value.
        negative: bool,
    },
}

impl fmt::Display for CubicBernsteinFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parameter(value) => {
                write!(formatter, "cubic parameter {value} must lie in [0,1]")
            }
            Self::Unrepresentable {
                order,
                axis,
                negative,
            } => write!(
                formatter,
                "cubic derivative order {order}, axis {axis}, negative={negative} must fit finite binary64"
            ),
        }
    }
}

impl Error for CubicBernsteinFailure {}

impl ExactCubicBernstein3 {
    /// Capture the exact finite coefficients. Example: `ExactCubicBernstein3::new(poles)?`.
    pub fn new(poles: [[f64; 3]; 4]) -> Result<Self, NonFiniteCoordinate> {
        let [x, y, z] =
            [0, 1, 2].map(|axis| Coordinate::capture(poles.map(|pole| pole[axis]), axis));
        Ok(Self {
            coordinates: [x?, y?, z?],
        })
    }

    /// Evaluate the position, rounded to nearest with ties to even. Exact zero is positive.
    /// Example: `curve.position(0.5)?`.
    pub fn position(&self, parameter: f64) -> Result<[f64; 3], CubicBernsteinFailure> {
        let values = self.evaluate(parameter)?;
        rounded_row(&values, 0)
    }

    /// Evaluate the first derivative, rounded to nearest with ties to even.
    /// Example: `curve.first_derivative(0.5)?`.
    pub fn first_derivative(&self, parameter: f64) -> Result<[f64; 3], CubicBernsteinFailure> {
        let values = self.evaluate(parameter)?;
        rounded_row(&values, 1)
    }

    /// Evaluate and independently round all three jet rows. Example: `curve.jet(0.5)?.second`.
    pub fn jet(&self, parameter: f64) -> Result<CubicBernsteinJet, CubicBernsteinFailure> {
        let values = self.evaluate(parameter)?;
        Ok(CubicBernsteinJet {
            position: rounded_row(&values, 0)?,
            first: rounded_row(&values, 1)?,
            second: rounded_row(&values, 2)?,
        })
    }

    fn evaluate(&self, t: f64) -> Result<[ExactRows; 3], CubicBernsteinFailure> {
        let parameter = Parameter::capture(t)?;
        Ok(self
            .coordinates
            .each_ref()
            .map(|axis| axis.evaluate(&parameter)))
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Coordinate {
    Constant(f64),
    Polynomial { poles: [BigInt; 4], exponent: i32 },
}

impl Coordinate {
    fn capture(values: [f64; 4], axis: usize) -> Result<Self, NonFiniteCoordinate> {
        let [a, b, c, d] = [0, 1, 2, 3].map(|pole| binary64_parts(values[pole], pole, axis));
        let parts = [a?, b?, c?, d?];
        if values.iter().all(|&value| value == values[0]) {
            let value = if values[0] == 0. { 0. } else { values[0] };
            return Ok(Self::Constant(value));
        }
        // Ignore zeros when choosing the grid so typical poles stay near one machine word.
        let shift = parts
            .iter()
            .filter(|part| part.significand != 0)
            .map(|part| part.shift)
            .min()
            .unwrap_or(0);
        let poles = parts.map(|part| {
            let mantissa = if part.negative {
                -(part.significand as i64)
            } else {
                part.significand as i64
            };
            BigInt::from(mantissa) << part.shift.saturating_sub(shift)
        });
        Ok(Self::Polynomial {
            poles,
            exponent: shift as i32 - 1074,
        })
    }

    fn evaluate(&self, t: &Parameter) -> ExactRows {
        match self {
            Self::Constant(value) => ExactRows::Constant(*value),
            Self::Polynomial { poles, exponent } => polynomial_rows(poles, *exponent, t),
        }
    }
}

struct Parameter {
    left: BigInt,
    right: BigInt,
    exponent: i32,
}

impl Parameter {
    fn capture(t: f64) -> Result<Self, CubicBernsteinFailure> {
        if !(0. ..=1.).contains(&t) {
            return Err(CubicBernsteinFailure::Parameter(t));
        }
        let exact = DyadicBound::from_f64(t).map_err(|_| CubicBernsteinFailure::Parameter(t))?;
        let (right, scale) = exact.ratio();
        Ok(Self {
            left: &scale - &right,
            right,
            exponent: 1 - scale.bits() as i32,
        })
    }

    fn blend(&self, left: &BigInt, right: &BigInt) -> BigInt {
        &self.left * left + &self.right * right
    }
}

enum ExactRows {
    Constant(f64),
    Polynomial {
        numerator: [BigInt; 3],
        exponent: [i32; 3],
    },
}

fn polynomial_rows(poles: &[BigInt; 4], exponent: i32, t: &Parameter) -> ExactRows {
    let [a, b, c] = [0, 1, 2].map(|i| t.blend(&poles[i], &poles[i + 1]));
    let d = t.blend(&a, &b);
    let e = t.blend(&b, &c);
    // Lower de Casteljau levels give derivative nets before any coordinate is rounded.
    let position = t.blend(&d, &e);
    let first = (e - d) * 3;
    let second = (c - (b << 1_usize) + a) * 6;
    ExactRows::Polynomial {
        numerator: [position, first, second],
        exponent: [3, 2, 1].map(|level| exponent + level * t.exponent),
    }
}

impl ExactRows {
    fn exact(&self, order: usize) -> ExactRatio {
        match self {
            Self::Constant(value) => ExactRatio::from_finite(if order == 0 { *value } else { 0. }),
            Self::Polynomial {
                numerator,
                exponent,
            } => ExactRatio::from_dyadic(&DyadicBound::new(
                numerator[order].clone(),
                exponent[order],
            )),
        }
    }

    fn round(&self, order: usize, axis: usize) -> Result<f64, CubicBernsteinFailure> {
        match self {
            Self::Constant(value) => Ok(if order == 0 { *value } else { 0. }),
            Self::Polynomial { .. } => {
                let value = self.exact(order);
                nearest_binary64(&value).ok_or_else(|| CubicBernsteinFailure::Unrepresentable {
                    order,
                    axis,
                    negative: value.sign() == Ordering::Less,
                })
            }
        }
    }
}

fn rounded_row(values: &[ExactRows; 3], order: usize) -> Result<[f64; 3], CubicBernsteinFailure> {
    let [x, y, z] = [0, 1, 2].map(|axis| values[axis].round(order, axis));
    Ok([x?, y?, z?])
}

#[cfg(test)]
mod tests;
