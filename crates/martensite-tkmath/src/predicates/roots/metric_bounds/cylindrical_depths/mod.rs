// SPDX-License-Identifier: GPL-2.0-only
//! Exact endpoint radius arithmetic and conservative storage-budget composition.
#[cfg(test)]
mod tests;
mod types;
use super::CertifiedCylindricalStorage;
use crate::predicates::roots::DyadicBound;
use crate::predicates::roots::certificate_arithmetic::{ExactRatio, rounding::outward_binary64};
use std::cmp::Ordering;
pub use types::{CertifiedCylindricalDepths, CylindricalDepthFailure};
type Checked<T> = Result<T, CylindricalDepthFailure>;

/// Include radius-plus-depth rounding in the retained sweep's existing storage bound.
/// Both exact requested radii must be positive and depths strictly increasing.
/// Example: `let proof = certify_cylindrical_depths(10., [-1., 2.], &storage, 1e-8)?;`.
pub fn certify_cylindrical_depths(
    radius: f64,
    depths: [f64; 2],
    storage: &CertifiedCylindricalStorage,
    limit: f64,
) -> Checked<CertifiedCylindricalDepths> {
    admit(radius, depths, limit)?;
    let stored = storage.stored_radii();
    let errors = sweep_endpoint_errors(radius, depths, stored)?;
    let maximum = errors[0].clone().max(errors[1].clone());
    let combined = maximum.add(&ExactRatio::from_finite(storage.normal_sweep_error_bound()));
    let combined = upper(&combined)?;
    if combined > limit {
        return Err(CylindricalDepthFailure::Uncertified {
            bound: combined,
            requested: limit,
        });
    }
    Ok(CertifiedCylindricalDepths::assemble(
        radius,
        depths,
        stored,
        [upper(&errors[0])?, upper(&errors[1])?],
        combined,
        limit,
    ))
}

fn admit(radius: f64, depths: [f64; 2], limit: f64) -> Checked<()> {
    finite("support radius", 0, radius)?;
    finite("storage limit", 0, limit)?;
    for (index, depth) in depths.into_iter().enumerate() {
        finite("depth", index, depth)?;
    }
    admit_depth_ranges(radius, depths, limit)
}

#[inline(always)]
fn admit_depth_ranges(radius: f64, depths: [f64; 2], limit: f64) -> Checked<()> {
    if radius <= 0. {
        return Err(CylindricalDepthFailure::Value {
            field: "positive support radius",
            index: 0,
            value: radius,
        });
    }
    if limit < 0. {
        return Err(CylindricalDepthFailure::Value {
            field: "nonnegative storage limit",
            index: 0,
            value: limit,
        });
    }
    if depths[0] >= depths[1] {
        return Err(CylindricalDepthFailure::DepthOrder { depths });
    }
    Ok(())
}

#[inline(always)]
fn sweep_endpoint_errors(
    radius: f64,
    depths: [f64; 2],
    stored: [f64; 2],
) -> Checked<[ExactRatio; 2]> {
    Ok([
        endpoint(radius, depths[0], stored[0], 0)?,
        endpoint(radius, depths[1], stored[1], 1)?,
    ])
}

fn finite(field: &'static str, index: usize, value: f64) -> Checked<()> {
    if !value.is_finite() {
        return Err(CylindricalDepthFailure::Value {
            field,
            index,
            value,
        });
    }
    Ok(())
}

fn endpoint(radius: f64, depth: f64, stored: f64, index: usize) -> Checked<ExactRatio> {
    let requested = ExactRatio::from_finite(radius).add(&ExactRatio::from_finite(depth));
    if requested.sign() != Ordering::Greater {
        return Err(CylindricalDepthFailure::NonpositiveRadius {
            index,
            radius,
            depth,
        });
    }
    let error = ExactRatio::from_finite(stored).subtract(&requested);
    Ok(if error.sign() == Ordering::Less {
        error.negated()
    } else {
        error
    })
}

fn upper(value: &ExactRatio) -> Checked<f64> {
    // Every input and operation here is dyadic, so the reduced denominator is a power of two.
    let exponent = 1 - value.denominator().bits() as i32;
    outward_binary64(value).map(|pair| pair[1]).ok_or_else(|| {
        CylindricalDepthFailure::UnrepresentableBound {
            bound: DyadicBound::new(value.numerator().clone(), exponent),
        }
    })
}
