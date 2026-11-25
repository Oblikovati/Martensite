// SPDX-License-Identifier: GPL-2.0-only
//! Direct physical residual; no angular normalization or sampled surface evaluation.
use super::*;
use crate::predicates::roots::certificate_arithmetic::bernstein_curve::{
    capture_planar, restrict_range,
};
pub(in crate::predicates::roots::metric_bounds) mod conical;
mod types;
pub use conical::*;
pub use types::{CertifiedCylindricalTrim, CylindricalTrimError, CylindricalTrimQuery};
#[cfg(test)]
mod tests;

/// Certify the complete `[0,1]` same-parameter trace on a local +Z cylinder.
/// Actual retained polynomial UV controls feed the outward trigonometric enclosure;
/// actual rational space coefficients feed its numerator/weight residual.
/// Example: `let proof = certify_cylindrical_trim(query, || false)?;`.
pub fn certify_cylindrical_trim(
    query: CylindricalTrimQuery<'_>,
    cancelled: impl FnMut() -> bool + Send,
) -> Result<CertifiedCylindricalTrim, CylindricalTrimError> {
    with_ranges(query, [[0., 1.]; 2], 4, cancelled)
}

/// Certify the active ranges of same-parameter Bezier traces on a cylinder.
/// Supply the spatial and UV ranges in the respective parent curves' normalized
/// coordinates, in that order. Both oriented intervals represent the same
/// physical edge parameter. The UV representation may be degree five.
/// Example: `let proof = certify_cylindrical_trim_ranges(query, [[0.3, 0.9], [0.0, 1.0]], || false)?;`.
pub fn certify_cylindrical_trim_ranges(
    query: CylindricalTrimQuery<'_>,
    ranges: [[f64; 2]; 2],
    cancelled: impl FnMut() -> bool + Send,
) -> Result<CertifiedCylindricalTrim, CylindricalTrimError> {
    with_ranges(query, ranges, 6, cancelled)
}

fn with_ranges(
    query: CylindricalTrimQuery<'_>,
    ranges: [[f64; 2]; 2],
    maximum_uv_poles: usize,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<CertifiedCylindricalTrim, CylindricalTrimError> {
    execute(query, ranges, maximum_uv_poles, &mut cancelled).map_err(|cause| CylindricalTrimError {
        radius: query.radius,
        requested_spatial_limit: query.spatial_limit,
        requested_options: query.options,
        cause,
    })
}

fn admit(query: CylindricalTrimQuery<'_>, maximum_uv_poles: usize) -> Checked<()> {
    admission::policy(query.options)?;
    admission::count("retained UV poles", query.pcurve.len(), maximum_uv_poles)?;
    admission::count("retained space poles", query.curve.poles.len(), 10)?;
    admission::value("cylinder radius", 0, query.radius, true)?;
    admission::limit(query.spatial_limit)?;
    admission::coordinates(query.pcurve, query.curve.poles)?;
    admission::weights(query.curve)
}

fn valid_range([start, end]: [f64; 2]) -> bool {
    0. <= start && start < end && end <= 1.
}

fn execute(
    query: CylindricalTrimQuery<'_>,
    ranges: [[f64; 2]; 2],
    maximum_uv_poles: usize,
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> Checked<CertifiedCylindricalTrim> {
    admit(query, maximum_uv_poles)?;
    for (index, range) in ranges.into_iter().enumerate() {
        if !valid_range(range) {
            return Err(CylindricalProfileFailure::Value {
                role: "active cylinder trim range",
                index,
                value: range[1] - range[0],
            });
        }
    }
    let policy = ArithmeticPolicy {
        fractional_bits: query.options.fractional_bits,
        max_operations: query.options.max_operations,
    };
    let mut arithmetic = Arithmetic::new(policy, cancelled);
    let bounds = residual(query, ranges, &mut arithmetic)?;
    accept_position(bounds[2], query.spatial_limit)?;
    Ok(CertifiedCylindricalTrim {
        bounds,
        bits: types::source_bits(query),
        ranges,
        limit: query.spatial_limit,
        options: query.options,
        operations: arithmetic.operations(),
    })
}

fn residual(
    query: CylindricalTrimQuery<'_>,
    [space, chart]: [[f64; 2]; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[f64; 3]> {
    let uv = narrowed(capture_planar(query.pcurve, arithmetic)?, chart, arithmetic)?;
    let stored = narrowed(source::stored(query.curve, arithmetic)?, space, arithmetic)?;
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

fn narrowed<const N: usize>(
    mut coefficients: [Vec<Interval>; N],
    range: [f64; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[Vec<Interval>; N]> {
    if range != [0., 1.] {
        for channel in &mut coefficients {
            *channel = restrict_range(channel, range, arithmetic)?;
        }
    }
    Ok(coefficients)
}
