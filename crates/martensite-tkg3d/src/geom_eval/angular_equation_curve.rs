// SPDX-License-Identifier: GPL-2.0-only
//! Materialize an ordered certified Hermite cover as detached native cubics.
use super::equation_curve::retained_bezier;
use crate::geom::GeomBezierCurve;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::predicates::roots::metric_bounds::angular_equation::{
    AngularBezierCertificate, AngularEquationCurve, AngularEquationError,
};
use std::{error::Error, fmt};

/// Immutable proof and ordered native cubic spans of an angular equation.
/// Adjacent native spans share identical endpoint bits.
pub struct AngularEquationExport {
    curves: Vec<GeomBezierCurve>,
    certificate: AngularBezierCertificate,
}

impl AngularEquationExport {
    /// Read the actual controls, span parameter maps and achieved geometric error.
    /// Example: `export.certificate().spans()`.
    pub fn certificate(&self) -> &AngularBezierCertificate {
        &self.certificate
    }

    /// Transfer the ordered cubic carriers. Each has local parameter [0, 1].
    /// Example: `let cubics = export.into_native();`.
    pub fn into_native(self) -> Vec<GeomBezierCurve> {
        self.curves
    }
}

/// Why the certified native angular equation export could not be completed.
#[derive(Debug)]
pub enum AngularEquationExportError {
    /// The model-space fit, native control proof or cancellation refused.
    Certificate(AngularEquationError),
    /// Native construction or readback failed for this span.
    Native {
        /// Zero-based index in the output ordered cover.
        span: usize,
        /// Failure returned by the native curve implementation.
        cause: Box<Failure>,
    },
    /// Native degrees, ranges or pole bits disagree with the certified controls.
    Representation(usize),
    /// Ordered native spans disagree at their parameter or position join.
    Join(usize),
}

impl fmt::Display for AngularEquationExportError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Certificate(cause) => cause.fmt(output),
            Self::Native { span, cause } => write!(output, "native angular span {span}: {cause}"),
            Self::Representation(span) => write!(
                output,
                "native angular span {span} differs from its certified representation"
            ),
            Self::Join(span) => write!(
                output,
                "native angular spans before and at {span} must share a parameter and endpoint"
            ),
        }
    }
}

impl Error for AngularEquationExportError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Certificate(cause) => Some(cause),
            Self::Native { cause, .. } => Some(cause.as_ref()),
            _ => None,
        }
    }
}

/// Fit and read back native cubic spans under the original inclusive distance limit.
/// Example: `export_angular_equation(&equation, 1e-6, || false)?.certificate().distance_bound()`.
pub fn export_angular_equation(
    equation: &AngularEquationCurve,
    limit: f64,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<AngularEquationExport, AngularEquationExportError> {
    let certificate = equation
        .certify_bezier(limit, &mut cancelled)
        .map_err(AngularEquationExportError::Certificate)?;
    let curves = materialize(&certificate, &mut cancelled)?;
    check_cancelled(&mut cancelled)?;
    Ok(AngularEquationExport {
        curves,
        certificate,
    })
}

fn materialize(
    certificate: &AngularBezierCertificate,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Vec<GeomBezierCurve>, AngularEquationExportError> {
    let mut curves = Vec::with_capacity(certificate.spans().len());
    for (index, span) in certificate.spans().iter().enumerate() {
        check_cancelled(cancelled)?;
        if let Some(previous) = index
            .checked_sub(1)
            .and_then(|i| certificate.spans().get(i))
            && (previous.normalized_interval()[1] != span.normalized_interval()[0]
                || previous.poles()[3].map(f64::to_bits) != span.poles()[0].map(f64::to_bits))
        {
            return Err(AngularEquationExportError::Join(index));
        }
        let curve = retained_bezier(span.poles())
            .map_err(|cause| AngularEquationExportError::Native {
                span: index,
                cause: Box::new(cause),
            })?
            .ok_or(AngularEquationExportError::Representation(index))?;
        curves.push(curve);
    }
    Ok(curves)
}

fn check_cancelled(cancelled: &mut impl FnMut() -> bool) -> Result<(), AngularEquationExportError> {
    if cancelled() {
        return Err(AngularEquationExportError::Certificate(
            AngularEquationError::Cancelled,
        ));
    }
    Ok(())
}
