// SPDX-License-Identifier: GPL-2.0-only
//! Bind a span enclosure to its original coefficients and unchanged error request.
use super::inverse::*;
use super::*;

pub(super) fn admit_options(
    limit: f64,
    arithmetic: &Arithmetic<'_>,
) -> Result<(), SourceBoundFailure> {
    if !limit.is_finite() || limit <= 0. {
        return Err(SourceBoundFailure::InvalidError(limit));
    }
    let policy = arithmetic.policy();
    if !(8..=4096).contains(&policy.fractional_bits) || policy.max_operations > 1_000_000 {
        return Err(SourceBoundFailure::InvalidPolicy {
            fractional_bits: policy.fractional_bits,
            max_operations: policy.max_operations,
        });
    }
    Ok(())
}
pub(super) fn check_width(
    enclosure: &Interval,
    limit: f64,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(), SourceBoundFailure> {
    let low = Interval {
        lower: enclosure.lower.clone(),
        upper: enclosure.lower.clone(),
    };
    let high = Interval {
        lower: enclosure.upper.clone(),
        upper: enclosure.upper.clone(),
    };
    let width = arithmetic
        .subtract(&high, &low)
        .map_err(SourceBoundFailure::from)?;
    let allowed = arithmetic.scalar(limit).map_err(SourceBoundFailure::from)?;
    admit_width(width, allowed, limit, arithmetic.fractional_bits())
}

fn admit_width(
    width: Interval,
    allowed: Interval,
    limit: f64,
    fractional_bits: u16,
) -> Result<(), SourceBoundFailure> {
    if width.upper > allowed.lower {
        return Err(SourceBoundFailure::ErrorLimit {
            width: crate::predicates::roots::DyadicBound::new(
                width.upper,
                -i32::from(fractional_bits),
            ),
            limit,
        });
    }
    Ok(())
}
