// SPDX-License-Identifier: GPL-2.0-only
//! Bound the depth over which actual stored sides may contact the stock cylinder radially.
mod contract;
#[cfg(test)]
mod tests;

use super::super::CertifiedCylindricalTrim;
use super::{Arithmetic, CertifiedCylindricalRadialSweep, CylindricalRadialSense, Interval};
use super::{admission, arithmetic_policy};
use CylindricalContactPrefixFailure as Failure;
pub use contract::*;
use num_bigint::Sign;

/// Enclose possible radial contact with the stock cylinder by a strict prefix of the ruled side.
///
/// The sweep and trim must describe the actual stored rails and first-depth seam in the same
/// local +Z frame with origin zero. All depths strictly after the outward upper bound are
/// outside the supplied radial contact allowance. The prefix requires its own contact checks.
/// Example: `let prefix = certify_cylindrical_contact_prefix(query, &sweep, &trim, || false)?;`.
pub fn certify_cylindrical_contact_prefix(
    query: CylindricalContactPrefixQuery<'_>,
    sweep: &CertifiedCylindricalRadialSweep,
    trim: &CertifiedCylindricalTrim,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<CertifiedCylindricalContactPrefix, CylindricalContactPrefixError> {
    execute(query, sweep, trim, &mut cancelled).map_err(|cause| CylindricalContactPrefixError {
        sense: query.sense,
        radius: query.seam.radius,
        contact_allowance: query.contact_allowance,
        max_normalized_depth: query.max_normalized_depth,
        requested_options: query.options,
        cause,
    })
}

fn execute(
    query: CylindricalContactPrefixQuery<'_>,
    sweep: &CertifiedCylindricalRadialSweep,
    trim: &CertifiedCylindricalTrim,
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> Result<CertifiedCylindricalContactPrefix, Failure> {
    admit(query)?;
    let mut arithmetic = Arithmetic::new(arithmetic_policy(query.options), cancelled);
    arithmetic.check_cancelled()?;
    check_authority(query, sweep, trim)?;
    let error = trim.xy_residual_bound();
    let derivative = sweep.signed_squared_radius_derivative_lower();
    let upper = normalized_upper(query, error, derivative, &mut arithmetic)?;
    if upper >= query.max_normalized_depth {
        return Err(Failure::Uncertified { upper });
    }
    Ok(CertifiedCylindricalContactPrefix::new(
        query,
        error,
        derivative,
        upper,
        arithmetic.operations(),
    ))
}

fn admit(query: CylindricalContactPrefixQuery<'_>) -> Result<(), Failure> {
    admission::admit_policy(query.options)?;
    for (depth, rail) in query.rails().into_iter().enumerate() {
        admission::admit_curve(rail, depth, query.options.max_degree)?;
    }
    if !query.seam.radius.is_finite() || query.seam.radius <= 0. {
        return Err(Failure::Radius(query.seam.radius));
    }
    if !query.contact_allowance.is_finite() || query.contact_allowance < 0. {
        return Err(Failure::ContactAllowance(query.contact_allowance));
    }
    let max = query.max_normalized_depth;
    if !(max > 0. && max <= 1.) {
        return Err(Failure::MaximumDepth(max));
    }
    Ok(())
}

fn check_authority(
    query: CylindricalContactPrefixQuery<'_>,
    sweep: &CertifiedCylindricalRadialSweep,
    trim: &CertifiedCylindricalTrim,
) -> Result<(), Failure> {
    if !sweep.matches_source(query.rails(), [0.; 2], query.sense) {
        return Err(Failure::SweepSource);
    }
    if !trim.matches_source(query.seam)
        || trim.requested_spatial_limit().to_bits() != query.seam.spatial_limit.to_bits()
        || trim.requested_options() != query.seam.options
    {
        return Err(Failure::TrimSource);
    }
    Ok(())
}

fn normalized_upper(
    query: CylindricalContactPrefixQuery<'_>,
    seam_error: f64,
    derivative: f64,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<f64, Failure> {
    let radius = arithmetic.scalar(query.seam.radius)?;
    let error = arithmetic.scalar(seam_error)?;
    let contact = arithmetic.scalar(query.contact_allowance)?;
    let inner = arithmetic.subtract(&radius, &error)?;
    let inner_contact = arithmetic.subtract(&radius, &contact)?;
    if inner.lower.sign() != Sign::Plus || inner_contact.lower.sign() != Sign::Plus {
        return Err(Failure::RadiusAllowance { seam_error });
    }
    let numerator = squared_gap(query.sense, &radius, &error, &contact, arithmetic)?;
    let derivative = arithmetic.scalar(derivative)?;
    Ok(arithmetic.upper_quotient(&numerator.upper, &derivative.lower)?)
}

fn squared_gap(
    sense: CylindricalRadialSense,
    radius: &Interval,
    error: &Interval,
    contact: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, Failure> {
    // Reverse triangle inequality places the seam radius in [R-e,R+e].
    // Integration of the signed derivative then separates it from the contact band.
    // Factor the difference of squares to preserve allowances far below the radius.
    let sum = arithmetic.add(error, contact)?;
    let delta = match sense {
        CylindricalRadialSense::Outward => arithmetic.subtract(contact, error)?,
        CylindricalRadialSense::Inward => arithmetic.subtract(error, contact)?,
    };
    let double_radius = arithmetic.add(radius, radius)?;
    let factor = arithmetic.add(&double_radius, &delta)?;
    Ok(arithmetic.multiply(&sum, &factor)?)
}
