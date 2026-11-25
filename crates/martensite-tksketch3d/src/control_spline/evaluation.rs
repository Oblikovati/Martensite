// SPDX-License-Identifier: GPL-2.0-only
//! Read-only jet evaluation of the exact captured polynomial.
use super::{ControlSplineGeometry, CubicEvidence};
use martensite_tkmath::{
    gp::{Point3, Vector3},
    predicates::roots::CubicBernsteinFailure,
};
use std::{error::Error, fmt};

/// Exact Bernstein value and derivatives, each rounded once to binary64.
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
    /// An exact position or derivative coordinate would round to infinity.
    NonFinite {
        /// Requested normalized parameter.
        parameter: f64,
        /// Position (0) or derivative order (1–2).
        order: usize,
        /// Zero-based coordinate index.
        axis: usize,
        /// Signed infinite representation of that out-of-range coordinate.
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
        self.evidence
            .value([self.poles[0], self.poles[3]], parameter)
    }

    /// Evaluate the rounded first derivative with respect to the normalized parameter.
    /// Example: `curve.first_derivative(0.5)?`.
    pub fn first_derivative(&self, parameter: f64) -> Result<Vector3, ControlSplineJetFailure> {
        self.evidence.first_derivative(parameter)
    }

    /// Evaluate the rounded value and two derivatives on the closed span.
    /// Example: `curve.jet(0.)?.first`.
    pub fn jet(&self, parameter: f64) -> Result<ControlSplineJet, ControlSplineJetFailure> {
        self.evidence.jet([self.poles[0], self.poles[3]], parameter)
    }
}

impl CubicEvidence {
    pub(crate) fn value(
        &self,
        endpoints: [Point3; 2],
        parameter: f64,
    ) -> Result<Point3, ControlSplineJetFailure> {
        if let Some(endpoint) = endpoint(endpoints, parameter) {
            return Ok(endpoint);
        }
        let [x, y, z] = self
            .polynomial
            .position(parameter)
            .map_err(|failure| evaluation_refusal(parameter, failure))?;
        Ok(Point3::new(x, y, z))
    }

    pub(crate) fn first_derivative(
        &self,
        parameter: f64,
    ) -> Result<Vector3, ControlSplineJetFailure> {
        let first = self
            .polynomial
            .first_derivative(parameter)
            .map_err(|failure| evaluation_refusal(parameter, failure))?;
        velocity(first, parameter)
    }

    pub(crate) fn jet(
        &self,
        endpoints: [Point3; 2],
        parameter: f64,
    ) -> Result<ControlSplineJet, ControlSplineJetFailure> {
        let jet = self
            .polynomial
            .jet(parameter)
            .map_err(|failure| evaluation_refusal(parameter, failure))?;
        let [x, y, z] = jet.position;
        let position = endpoint(endpoints, parameter).unwrap_or_else(|| Point3::new(x, y, z));
        let first = velocity(jet.first, parameter)?;
        let [dx, dy, dz] = jet.second;
        Ok(ControlSplineJet {
            position,
            first,
            second: Vector3::new(dx, dy, dz),
        })
    }
}

fn endpoint(endpoints: [Point3; 2], parameter: f64) -> Option<Point3> {
    match parameter {
        0. => Some(endpoints[0]),
        1. => Some(endpoints[1]),
        _ => None,
    }
}

fn velocity([x, y, z]: [f64; 3], parameter: f64) -> Result<Vector3, ControlSplineJetFailure> {
    if x == 0. && y == 0. && z == 0. {
        return Err(ControlSplineJetFailure::RoundedStationary(parameter));
    }
    Ok(Vector3::new(x, y, z))
}

fn evaluation_refusal(parameter: f64, failure: CubicBernsteinFailure) -> ControlSplineJetFailure {
    match failure {
        CubicBernsteinFailure::Parameter(value) => ControlSplineJetFailure::Parameter(value),
        CubicBernsteinFailure::Unrepresentable {
            order,
            axis,
            negative,
        } => ControlSplineJetFailure::NonFinite {
            parameter,
            order,
            axis,
            value: if negative {
                f64::NEG_INFINITY
            } else {
                f64::INFINITY
            },
        },
    }
}
