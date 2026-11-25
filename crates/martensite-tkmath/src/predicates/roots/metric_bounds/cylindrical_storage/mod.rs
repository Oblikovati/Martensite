// SPDX-License-Identifier: GPL-2.0-only
//! Storage bounds in the support's local cylinder chart; no topology or angular claim (#418).
mod admission;
mod error;
mod proof;
mod radial_slice;
mod radial_sweep;
#[cfg(test)]
mod tests;
mod types;

use super::MetricBezier;
use crate::predicates::roots::certificate_arithmetic::{
    bernstein_bounds as polynomial,
    bounded_dyadic::{Arithmetic, ArithmeticPolicy, Interval},
    bounded_failure::BoundedArithmeticFailure,
};
use error::Checked;
pub use error::{CylindricalStorageError, CylindricalStorageFailure};
pub use radial_slice::{
    CertifiedCylindricalSlice, CylindricalSliceQuery, CylindricalSliceRefusal,
    certify_cylindrical_slice_direction,
};
pub use radial_sweep::{
    CertifiedCylindricalRadialSweep, CylindricalRadialSense, CylindricalRadialSweepError,
    CylindricalRadialSweepFailure, CylindricalRadialSweepQuery, certify_cylindrical_radial_sweep,
};
pub use types::*;

/// Bound radial distance, scaled radial rulings and height drift for two stored curves.
/// Coordinates are local to a cylinder at the origin with axis +Z on parameter `[0,1]`.
/// This certifies supplied coefficients, not authored-profile conformance, joins, topology or a published body.
/// Example: `let proof = certify_cylindrical_storage(query, || false)?;`.
pub fn certify_cylindrical_storage(
    query: CylindricalStorageQuery<'_>,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<CertifiedCylindricalStorage, CylindricalStorageError> {
    execute(query, &mut cancelled).map_err(|cause| CylindricalStorageError {
        requested_options: query.options,
        requested_limits: query.limits,
        cause,
    })
}

fn execute(
    query: CylindricalStorageQuery<'_>,
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> Checked<CertifiedCylindricalStorage> {
    admission::admit(&query)?;
    let mut arithmetic = Arithmetic::new(arithmetic_policy(query.options), cancelled);
    let curves = [
        capture(query.curves[0], &mut arithmetic)?,
        capture(query.curves[1], &mut arithmetic)?,
    ];
    let bounds = proof::bounds(&curves, query.radii, &mut arithmetic)?;
    admission::accept(bounds, query.limits)?;
    Ok(CertifiedCylindricalStorage::assemble(
        query,
        bounds,
        arithmetic.operations(),
    ))
}

fn capture(
    source: MetricBezier<'_, 3>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<[Vec<Interval>; 4], BoundedArithmeticFailure> {
    let mut result: [Vec<Interval>; 4] =
        std::array::from_fn(|_| Vec::with_capacity(source.poles.len()));
    for (i, pole) in source.poles.iter().enumerate() {
        let weight = arithmetic.scalar(source.weights.map_or(1., |w| w[i]))?;
        for axis in 0..3 {
            let coordinate = arithmetic.scalar(pole[axis])?;
            result[axis].push(arithmetic.multiply(&coordinate, &weight)?);
        }
        result[3].push(weight);
    }
    Ok(result)
}

fn arithmetic_policy(options: CylindricalStorageOptions) -> ArithmeticPolicy {
    ArithmeticPolicy {
        fractional_bits: options.fractional_bits,
        max_operations: options.max_operations,
    }
}
