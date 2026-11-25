// SPDX-License-Identifier: GPL-2.0-only
//! Read-only native evaluation of the captured polynomial.
use super::ControlSplineGeometry;
use martensite_tkernel::{ncollection::Array1, standard::Failure};
use martensite_tkmath::{
    bsplclib::{bezier_d0, bezier_d1, bezier_d2},
    gp::{Point3, Vector3},
};
use std::{error::Error, fmt};

/// Rounded native value and derivatives with respect to the normalized cubic parameter.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ControlSplineJet {
    /// Curve position.
    pub position: Point3,
    /// First derivative, in model units per parameter.
    pub first: Vector3,
    /// Second derivative, in model units per parameter squared.
    pub second: Vector3,
}

/// A request or returned native jet that cannot supply finite directed geometry.
#[derive(Debug)]
#[non_exhaustive]
pub enum ControlSplineJetFailure {
    /// Parameter is outside the closed normalized range.
    Parameter(f64),
    /// Native evaluation refused a parameter in the closed range.
    Native {
        /// Requested normalized parameter.
        parameter: f64,
        /// Native evaluator diagnostic.
        source: Failure,
    },
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
            Self::Native { parameter, source } => {
                write!(formatter, "control spline at {parameter}: {source}")
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

impl Error for ControlSplineJetFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Native { source, .. } => Some(source),
            _ => None,
        }
    }
}

impl ControlSplineGeometry {
    /// Evaluate a rounded position on the closed span. Endpoints return their authored values.
    /// Example: `curve.value(0.5)?`.
    pub fn value(&self, parameter: f64) -> Result<Point3, ControlSplineJetFailure> {
        check_parameter(parameter)?;
        if let Some(endpoint) = self.endpoint_at(parameter) {
            return Ok(endpoint);
        }
        let poles = Array1::from_shared(&self.poles, 1);
        let position = bezier_d0(parameter, &poles, None)
            .map_err(|source| ControlSplineJetFailure::Native { parameter, source })?;
        check_coordinate_row(parameter, 0, position.coordinates())?;
        Ok(position)
    }

    /// Evaluate the rounded first derivative with respect to the normalized parameter.
    /// Example: `curve.first_derivative(0.5)?`.
    pub fn first_derivative(&self, parameter: f64) -> Result<Vector3, ControlSplineJetFailure> {
        check_parameter(parameter)?;
        let poles = Array1::from_shared(&self.poles, 1);
        let (_, first) = bezier_d1(parameter, &poles, None)
            .map_err(|source| ControlSplineJetFailure::Native { parameter, source })?;
        check_velocity(parameter, first)?;
        Ok(first)
    }

    /// Evaluate the rounded value and two derivatives on the closed span.
    /// Example: `curve.jet(0.)?.first`.
    pub fn jet(&self, parameter: f64) -> Result<ControlSplineJet, ControlSplineJetFailure> {
        check_parameter(parameter)?;
        let poles = Array1::from_shared(&self.poles, 1);
        let (position, first, second) = bezier_d2(parameter, &poles, None)
            .map_err(|source| ControlSplineJetFailure::Native { parameter, source })?;
        let position = self.endpoint_at(parameter).unwrap_or(position);
        check_coordinate_row(parameter, 0, position.coordinates())?;
        check_velocity(parameter, first)?;
        check_coordinate_row(parameter, 2, second.xyz().coordinates())?;
        Ok(ControlSplineJet {
            position,
            first,
            second,
        })
    }

    fn endpoint_at(&self, parameter: f64) -> Option<Point3> {
        match parameter {
            0. => Some(self.poles[0]),
            1. => Some(self.poles[3]),
            _ => None,
        }
    }
}

fn check_velocity(parameter: f64, derivative: Vector3) -> Result<(), ControlSplineJetFailure> {
    let coordinates = derivative.xyz().coordinates();
    check_coordinate_row(parameter, 1, coordinates)?;
    if coordinates.iter().all(|value| *value == 0.) {
        return Err(ControlSplineJetFailure::RoundedStationary(parameter));
    }
    Ok(())
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
