// SPDX-License-Identifier: GPL-2.0-only
//! Bound authored-profile discrepancy using retained coefficients, without an atan evaluator.
mod admission;
pub(in crate::predicates::roots::metric_bounds) mod algebra;
mod chart_planning;
mod chart_types;
mod error;
mod lift;
mod planning;
#[cfg(test)]
mod planning_oracle_tests;
mod planning_types;
mod proof;
mod source;
#[cfg(test)]
mod tests;
pub(in crate::predicates::roots::metric_bounds) mod trigonometric;
mod trim;
mod types;
use crate::predicates::roots::certificate_arithmetic::{
    bernstein_bounds as polynomial,
    bounded_dyadic::{Arithmetic, ArithmeticPolicy, Interval},
};
pub use chart_planning::plan_cylindrical_charts;
pub use chart_types::{CylindricalChartPlan, CylindricalFitChart};
use error::Checked;
pub use error::{CylindricalProfileError, CylindricalProfileFailure};
pub use lift::{CertifiedCylindricalLift, CylindricalLiftError, certify_cylindrical_lift};
pub use planning::plan_cylindrical_fit;
pub use planning_types::{CylindricalFitPlan, CylindricalFitQuery};
pub use trim::{
    CertifiedConicalTrim, CertifiedConicalTrimPair, CertifiedCylindricalTrim, ConicalTrimCarrier,
    ConicalTrimError, ConicalTrimPairError, ConicalTrimPairQuery, ConicalTrimQuery,
    CylindricalTrimError, CylindricalTrimQuery, certify_conical_trim, certify_conical_trim_pair,
    certify_cylindrical_trim,
};
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
