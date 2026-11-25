// SPDX-License-Identifier: GPL-2.0-only
//! Read-only Bernstein evaluation of the captured polynomial and its local derivative nets.
use super::ControlSplineGeometry;
use martensite_tkmath::gp::{Point3, Vector3, Xyz};
use std::{error::Error, fmt};

/// Rounded value and derivatives with respect to the normalized cubic parameter.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ControlSplineJet {
    /// Curve position.
    pub position: Point3,
    /// First derivative, in model units per parameter.
    pub first: Vector3,
    /// Second derivative, in model units per parameter squared.
    pub second: Vector3,
}

/// A request or computed jet that cannot supply finite directed geometry.
#[derive(Debug)]
#[non_exhaustive]
pub enum ControlSplineJetFailure {
    /// Parameter is outside the closed normalized range.
    Parameter(f64),
    /// A rounded position or derivative coordinate is not finite.
    NonFinite {
        /// Requested normalized parameter.
        parameter: f64,
        /// Position (0) or derivative order (1–2).
        order: usize,
        /// Zero-based coordinate index.
        axis: usize,
        /// Returned nonfinite value.
        value: f64,
    },
    /// All rounded first derivative components are zero despite exact regularity.
    RoundedStationary(f64),
}

impl fmt::Display for ControlSplineJetFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parameter(t) => {
                write!(formatter, "control spline parameter {t} must lie in [0,1]")
            }
            Self::NonFinite {
                parameter,
                order,
                axis,
                value,
            } => write!(
                formatter,
                "control spline at {parameter}, derivative order {order}, axis {axis}: {value} must be finite"
            ),
            Self::RoundedStationary(t) => write!(
                formatter,
                "control spline at {t} requires a representable nonzero derivative"
            ),
        }
    }
}

impl Error for ControlSplineJetFailure {}

impl ControlSplineGeometry {
    /// Evaluate a rounded position on the closed span. Endpoints return their authored values.
    /// Example: `curve.value(0.5)?`.
    pub fn value(&self, parameter: f64) -> Result<Point3, ControlSplineJetFailure> {
        check_parameter(parameter)?;
        let position = Point3::from(bernstein(self.poles.map(|pole| pole.xyz()), parameter));
        check_coordinate_row(parameter, 0, position.coordinates())?;
        Ok(position)
    }

    /// Evaluate the rounded first derivative with respect to the normalized parameter.
    /// Example: `curve.first_derivative(0.5)?`.
    pub fn first_derivative(&self, parameter: f64) -> Result<Vector3, ControlSplineJetFailure> {
        check_parameter(parameter)?;
        velocity(self.local_differences(), parameter)
    }

    /// Evaluate the rounded value and two derivatives on the closed span.
    /// Example: `curve.jet(0.)?.first`.
    pub fn jet(&self, parameter: f64) -> Result<ControlSplineJet, ControlSplineJetFailure> {
        let position = self.value(parameter)?;
        let [ab, bc, cd] = self.local_differences();
        let first = velocity([ab, bc, cd], parameter)?;
        let second = Vector3::from(bernstein([bc - ab, cd - bc], parameter) * 6.);
        check_coordinate_row(parameter, 2, second.xyz().coordinates())?;
        Ok(ControlSplineJet {
            position,
            first,
            second,
        })
    }

    fn local_differences(&self) -> [Xyz; 3] {
        let [a, b, c, d] = self.poles.map(|pole| pole.xyz());
        // The endpoint derivatives depend only on nearby point differences.
        [b - a, c - b, d - c]
    }
}

fn bernstein<const N: usize>(mut controls: [Xyz; N], parameter: f64) -> Xyz {
    if parameter == 0. {
        return controls[0];
    }
    if parameter == 1. {
        return controls[N - 1];
    }
    let left = 1. - parameter;
    // Recovering t from 1 - left would erase tiny, nonzero endpoint contributions.
    for remaining in (1..N).rev() {
        for index in 0..remaining {
            controls[index] = controls[index] * left + controls[index + 1] * parameter;
        }
    }
    controls[0]
}

fn velocity(differences: [Xyz; 3], parameter: f64) -> Result<Vector3, ControlSplineJetFailure> {
    let derivative = Vector3::from(bernstein(differences, parameter) * 3.);
    let coordinates = derivative.xyz().coordinates();
    check_coordinate_row(parameter, 1, coordinates)?;
    if coordinates.iter().all(|value| *value == 0.) {
        return Err(ControlSplineJetFailure::RoundedStationary(parameter));
    }
    Ok(derivative)
}

fn check_parameter(parameter: f64) -> Result<(), ControlSplineJetFailure> {
    if (0. ..=1.).contains(&parameter) {
        return Ok(());
    }
    Err(ControlSplineJetFailure::Parameter(parameter))
}

fn check_coordinate_row(
    parameter: f64,
    order: usize,
    coordinates: &[f64; 3],
) -> Result<(), ControlSplineJetFailure> {
    for (axis, &value) in coordinates.iter().enumerate() {
        if !value.is_finite() {
            return Err(ControlSplineJetFailure::NonFinite {
                parameter,
                order,
                axis,
                value,
            });
        }
    }
    Ok(())
}
