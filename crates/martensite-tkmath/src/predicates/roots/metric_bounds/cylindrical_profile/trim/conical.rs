// SPDX-License-Identifier: GPL-2.0-only
//! The cone and cylinder share the retained-coordinate residual enclosure.
use super::*;
use num_bigint::Sign;
mod capture;
mod pair;
mod pair_types;
mod types;
pub use pair::certify_conical_trim_pair;
pub use pair_types::*;
pub use types::*;

/// Bound the retained trace on an analytic +Z cone over its complete interval.
/// This certifies correspondence, not closure or body validity.
/// Example: `let proof = certify_conical_trim(query, || false)?;`.
pub fn certify_conical_trim(
    query: ConicalTrimQuery<'_>,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<CertifiedConicalTrim, ConicalTrimError> {
    execute(query, &mut cancelled).map_err(|cause| ConicalTrimError {
        carrier: query.carrier,
        requested_spatial_limit: query.spatial_limit,
        requested_options: query.options,
        cause,
    })
}

/// Certify the same physical trim on distinct normalized space and UV parent ranges.
/// The actual clamped projected chart may be degree five.
/// Example: `let proof = certify_conical_trim_ranges(query, [[0.25, 1.0], [0.0, 1.0]], || false)?;`.
pub fn certify_conical_trim_ranges(
    query: ConicalTrimQuery<'_>,
    ranges: [[f64; 2]; 2],
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<CertifiedConicalTrim, ConicalTrimError> {
    execute_ranges(query, ranges, &mut cancelled).map_err(|cause| ConicalTrimError {
        carrier: query.carrier,
        requested_spatial_limit: query.spatial_limit,
        requested_options: query.options,
        cause,
    })
}

fn execute_ranges(
    query: ConicalTrimQuery<'_>,
    ranges: [[f64; 2]; 2],
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> Checked<CertifiedConicalTrim> {
    admit_poles(query, 6)?;
    for (index, range) in ranges.into_iter().enumerate() {
        if !valid_range(range) {
            return Err(CylindricalProfileFailure::Value {
                role: "active cone trim range",
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
    let bounds = capture::ranged_residual(query, ranges, &mut arithmetic)?;
    let mut receipt = accepted_trace(query, bounds, arithmetic.operations())?;
    receipt.ranges = ranges;
    Ok(receipt)
}

fn execute(
    query: ConicalTrimQuery<'_>,
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> Checked<CertifiedConicalTrim> {
    admit(query)?;
    let policy = ArithmeticPolicy {
        fractional_bits: query.options.fractional_bits,
        max_operations: query.options.max_operations,
    };
    let mut arithmetic = Arithmetic::new(policy, cancelled);
    let bounds = residual(query, &mut arithmetic)?;
    accept_position(bounds[2], query.spatial_limit)?;
    Ok(CertifiedConicalTrim {
        bounds,
        bits: types::source_bits(query),
        ranges: [[0., 1.]; 2],
        limit: query.spatial_limit,
        options: query.options,
        operations: arithmetic.operations(),
    })
}

pub(in crate::predicates::roots::metric_bounds) fn admit(
    query: ConicalTrimQuery<'_>,
) -> Checked<()> {
    admit_poles(query, 4)
}

fn admit_poles(query: ConicalTrimQuery<'_>, maximum_uv_poles: usize) -> Checked<()> {
    admission::policy(query.options)?;
    admission::count(
        "retained cone UV poles",
        query.pcurve.len(),
        maximum_uv_poles,
    )?;
    admission::count("retained cone space poles", query.curve.poles.len(), 10)?;
    admission::coordinates(query.pcurve, query.curve.poles)?;
    admission::weights(query.curve)?;
    admission::limit(query.spatial_limit)?;
    let carrier = query.carrier;
    admission::value("cone reference radius", 0, carrier.radius, true)?;
    admission::value("cone axial origin", 0, carrier.axial_origin, false)?;
    admit_angle(carrier.semi_angle)
}

fn admit_angle(angle: f64) -> Checked<()> {
    admission::value("cone semi-angle", 0, angle, true)?;
    if angle >= std::f64::consts::FRAC_PI_2 {
        return Err(CylindricalProfileFailure::Value {
            role: "cone semi-angle expected in (0, pi/2)",
            index: 0,
            value: angle,
        });
    }
    Ok(())
}

fn residual(query: ConicalTrimQuery<'_>, arithmetic: &mut Arithmetic<'_>) -> Checked<[f64; 3]> {
    capture::residual(query, None, arithmetic).map(|(bounds, _)| bounds)
}

pub(in crate::predicates::roots::metric_bounds) fn positive_radius(
    radius: &[Interval],
    reference: f64,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<()> {
    let lower = polynomial::lower(radius);
    if lower.sign() == Sign::Plus {
        return Ok(());
    }
    Err(CylindricalProfileFailure::DirectionUnresolved {
        residual: arithmetic.upper_f64(-lower)?,
        radius: reference,
    })
}

fn affine(
    values: &[Interval],
    scale: &Interval,
    offset: f64,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Vec<Interval>> {
    let offset = arithmetic.scalar(offset)?;
    values
        .iter()
        .map(|value| {
            let scaled = arithmetic.multiply(value, scale)?;
            Ok(arithmetic.add(&scaled, &offset)?)
        })
        .collect()
}

#[cfg(test)]
mod tests;

// Only checked metric-bound producers may construct the existing receipt.
pub(in crate::predicates::roots::metric_bounds) fn accepted_trace(
    query: ConicalTrimQuery<'_>,
    bounds: [f64; 3],
    operations: u32,
) -> Checked<CertifiedConicalTrim> {
    accept_position(bounds[2], query.spatial_limit)?;
    Ok(CertifiedConicalTrim {
        bounds,
        bits: types::source_bits(query),
        ranges: [[0., 1.]; 2],
        limit: query.spatial_limit,
        options: query.options,
        operations,
    })
}
pub(in crate::predicates::roots::metric_bounds) fn accepted_pair(
    traces: [CertifiedConicalTrim; 2],
    operations: u32,
) -> CertifiedConicalTrimPair {
    CertifiedConicalTrimPair { traces, operations }
}
