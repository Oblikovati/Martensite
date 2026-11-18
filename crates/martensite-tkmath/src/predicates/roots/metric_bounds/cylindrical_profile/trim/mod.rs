// SPDX-License-Identifier: GPL-2.0-only
//! Direct physical residual; no angular normalization or sampled surface evaluation.
use super::*;
use crate::predicates::roots::certificate_arithmetic::bernstein_curve::capture_planar;
mod types;
pub use types::{CertifiedCylindricalTrim, CylindricalTrimError, CylindricalTrimQuery};
#[cfg(test)]
mod tests;

/// Certify the complete `[0,1]` same-parameter trace on a local +Z cylinder.
/// Actual retained polynomial UV controls feed the outward trigonometric enclosure;
/// actual rational space coefficients feed its numerator/weight residual.
/// Example: `let proof = certify_cylindrical_trim(query, || false)?;`.
pub fn certify_cylindrical_trim(
    query: CylindricalTrimQuery<'_>,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<CertifiedCylindricalTrim, CylindricalTrimError> {
    execute(query, &mut cancelled).map_err(|cause| CylindricalTrimError {
        radius: query.radius,
        requested_spatial_limit: query.spatial_limit,
        requested_options: query.options,
        cause,
    })
}

fn admit(query: CylindricalTrimQuery<'_>) -> Checked<()> {
    admission::policy(query.options)?;
    admission::count("retained UV poles", query.pcurve.len(), 4)?;
    admission::count("retained space poles", query.curve.poles.len(), 10)?;
    admission::value("cylinder radius", 0, query.radius, true)?;
    admission::limit(query.spatial_limit)?;
    admission::coordinates(query.pcurve, query.curve.poles)?;
    admission::weights(query.curve)
}

fn execute(
    query: CylindricalTrimQuery<'_>,
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> Checked<CertifiedCylindricalTrim> {
    admit(query)?;
    let policy = ArithmeticPolicy {
        fractional_bits: query.options.fractional_bits,
        max_operations: query.options.max_operations,
    };
    let mut arithmetic = Arithmetic::new(policy, cancelled);
    let bounds = residual(query, &mut arithmetic)?;
    accept_position(bounds[2], query.spatial_limit)?;
    Ok(CertifiedCylindricalTrim {
        bounds,
        bits: types::source_bits(query),
        limit: query.spatial_limit,
        options: query.options,
        operations: arithmetic.operations(),
    })
}

fn residual(query: CylindricalTrimQuery<'_>, arithmetic: &mut Arithmetic<'_>) -> Checked<[f64; 3]> {
    let uv = capture_planar(query.pcurve, arithmetic)?;
    let stored = source::stored(query.curve, arithmetic)?;
    let weight = proof::minimum_weight(&stored);
    let trig = trigonometric::enclose(&uv[0], arithmetic)?;
    let radius = arithmetic.scalar(query.radius)?;
    let xy = proof::xy_residual(&radius, &stored, &trig, &weight, arithmetic)?;
    let height = proof::height_residual(&uv[1], &stored, &weight, arithmetic)?;
    let total = arithmetic.add(&xy, &height)?;
    Ok([
        arithmetic.upper_f64(xy.upper)?,
        arithmetic.upper_f64(height.upper)?,
        arithmetic.upper_f64(total.upper)?,
    ])
}
