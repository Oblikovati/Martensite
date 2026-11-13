// SPDX-License-Identifier: GPL-2.0-only
//! Bound authored-profile discrepancy using retained coefficients, without an atan evaluator.
mod admission;
mod algebra;
mod error;
mod planning;
mod planning_types;
mod proof;
mod source;
#[cfg(test)]
mod tests;
mod trigonometric;
mod types;
use crate::predicates::roots::certificate_arithmetic::{
    bernstein_bounds as polynomial,
    bounded_dyadic::{Arithmetic, ArithmeticPolicy, Interval},
};
use error::Checked;
pub use error::{CylindricalProfileError, CylindricalProfileFailure};
pub use planning::plan_cylindrical_fit;
pub use planning_types::{CylindricalFitPlan, CylindricalFitQuery};
pub use types::*;

/// Certify developed position error and local-parameter derivative discrepancy.
/// The reference curve uses the stored XY direction normalized to the support radius
/// and its retained height. No regularity, endpoint fixation or topology claim is made.
/// Example: `let proof = certify_cylindrical_profile(query, || false)?;`.
pub fn certify_cylindrical_profile(
    query: CylindricalProfileQuery<'_>,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<CertifiedCylindricalProfile, CylindricalProfileError> {
    execute(query, &mut cancelled).map_err(|cause| CylindricalProfileError {
        requested_options: query.options,
        requested_spatial_limit: query.spatial_limit,
        cause,
    })
}

fn execute(
    query: CylindricalProfileQuery<'_>,
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> Checked<CertifiedCylindricalProfile> {
    admission::admit(&query)?;
    let policy = ArithmeticPolicy {
        fractional_bits: query.options.fractional_bits,
        max_operations: query.options.max_operations,
    };
    let mut arithmetic = Arithmetic::new(policy, cancelled);
    let authored = source::authored(query, &mut arithmetic)?;
    let stored = source::stored(query.stored, &mut arithmetic)?;
    let bounds = proof::bounds(query, &authored, &stored, &mut arithmetic)?;
    accept_position(bounds[2], query.spatial_limit)?;
    Ok(CertifiedCylindricalProfile::assemble(
        query,
        bounds,
        arithmetic.operations(),
    ))
}

fn accept_position(bound: f64, requested: f64) -> Checked<()> {
    if bound > requested {
        return Err(CylindricalProfileFailure::Uncertified { bound, requested });
    }
    Ok(())
}
