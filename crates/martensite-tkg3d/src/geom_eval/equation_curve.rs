// SPDX-License-Identifier: GPL-2.0-only
//! Retain a native degree-one to degree-three carrier with certified geometric error.
use crate::geom::{GeomBezierCurve, GeomCurve};
use martensite_tkernel::{ncollection::Array1, standard::Failure};
use martensite_tkmath::{
    gp::Point3,
    predicates::roots::cartesian_equation::{
        CartesianEquationCurve, CartesianEquationError, EquationBezierCertificate,
    },
};
use std::{error::Error, fmt};

/// Immutable proof and native Bézier of one Cartesian equation export.
pub struct CartesianEquationExport {
    curve: GeomBezierCurve,
    certificate: EquationBezierCertificate,
}

impl CartesianEquationExport {
    /// Read the achieved discrepancy, authored parameter map and retained controls.
    /// Example: `export.certificate().distance_bound()`.
    pub fn certificate(&self) -> &EquationBezierCertificate {
        &self.certificate
    }

    /// Transfer the native degree-one to degree-three carrier into exclusive caller ownership.
    /// Example: `let native = export.into_native();`.
    pub fn into_native(self) -> GeomBezierCurve {
        self.curve
    }
}

/// Why a supported Cartesian equation could not be exported as a native carrier.
#[derive(Debug)]
pub enum CartesianEquationExportError {
    /// The polynomial or native control certificate refused the request.
    Certificate(CartesianEquationError),
    /// The native geometry builder or reader failed.
    Native(Box<Failure>),
    /// The actual native degree, range, weights or pole bits differ from the certificate.
    Representation,
}

impl fmt::Display for CartesianEquationExportError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Certificate(cause) => cause.fmt(output),
            Self::Native(cause) => write!(output, "Cartesian equation Bézier export: {cause}"),
            Self::Representation => {
                output.write_str("retained Bézier differs from the certified equation controls")
            }
        }
    }
}

impl Error for CartesianEquationExportError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Certificate(cause) => Some(cause),
            Self::Native(cause) => Some(cause.as_ref()),
            Self::Representation => None,
        }
    }
}

impl From<CartesianEquationError> for CartesianEquationExportError {
    fn from(cause: CartesianEquationError) -> Self {
        Self::Certificate(cause)
    }
}

impl From<Failure> for CartesianEquationExportError {
    fn from(cause: Failure) -> Self {
        Self::Native(Box::new(cause))
    }
}

/// Export an owned native Bézier after qualifying its actual controls and independent regularity.
/// Example: `let export = export_cartesian_equation(&equation, 1e-9, || false)?;`.
pub fn export_cartesian_equation(
    equation: &CartesianEquationCurve,
    distance_limit: f64,
    mut cancelled: impl FnMut() -> bool,
) -> Result<CartesianEquationExport, CartesianEquationExportError> {
    let certificate = equation.certify_bezier(distance_limit, &mut cancelled)?;
    let curve = retained_bezier(certificate.poles())?
        .ok_or(CartesianEquationExportError::Representation)?;
    if cancelled() {
        return Err(CartesianEquationError::Cancelled.into());
    }
    Ok(CartesianEquationExport { curve, certificate })
}

pub(super) fn retained_bezier(poles: &[[f64; 3]]) -> Result<Option<GeomBezierCurve>, Failure> {
    let points: Vec<_> = poles
        .iter()
        .map(|&[x, y, z]| Point3::new(x, y, z))
        .collect();
    let curve = GeomBezierCurve::new(&Array1::from_shared(&points, 1))?;
    if !matches_controls(&curve, poles)? {
        return Ok(None);
    }
    Ok(Some(curve))
}

fn matches_controls(curve: &GeomBezierCurve, poles: &[[f64; 3]]) -> Result<bool, Failure> {
    let Ok(degree) = i32::try_from(poles.len().saturating_sub(1)) else {
        return Ok(false);
    };
    if curve.degree() != degree
        || curve.is_rational()
        || [curve.first_parameter(), curve.last_parameter()] != [0., 1.]
    {
        return Ok(false);
    }
    for (index, expected) in (1..=degree + 1).zip(poles) {
        if curve.pole(index)?.coordinates().map(f64::to_bits) != expected.map(f64::to_bits) {
            return Ok(false);
        }
    }
    Ok(true)
}
