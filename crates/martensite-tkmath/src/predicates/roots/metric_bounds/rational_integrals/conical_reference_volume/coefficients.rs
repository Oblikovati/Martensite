// SPDX-License-Identifier: GPL-2.0-only
//! Enclose sin(alpha) delta(d) and signed cos(alpha) delta(d²), without rounded libm coefficients.
use super::*;
use crate::predicates::roots::certificate_arithmetic::bounded_failure::BoundedArithmeticFailure;
use crate::predicates::roots::metric_bounds::{
    CylindricalProfileFailure, cylindrical_profile::trigonometric,
};
use crate::predicates::roots::{
    DyadicBound,
    certificate_arithmetic::bounded_dyadic::{Arithmetic, ArithmeticPolicy, Interval},
};
struct Failure {
    depth: Option<usize>,
    cause: ConicalReferenceVolumeFailure,
}
impl From<BoundedArithmeticFailure> for Failure {
    fn from(cause: BoundedArithmeticFailure) -> Self {
        Self {
            depth: None,
            cause: ConicalReferenceVolumeFailure::Arithmetic(Box::new(cause)),
        }
    }
}
impl From<CylindricalProfileFailure> for Failure {
    fn from(cause: CylindricalProfileFailure) -> Self {
        Self {
            depth: None,
            cause: ConicalReferenceVolumeFailure::Trigonometric(Box::new(cause)),
        }
    }
}
pub(super) fn compute(
    query: ConicalReferenceVolumeQuery<'_>,
    cancelled: &mut (impl FnMut() -> bool + Send),
) -> Result<([ContributionBounds; 2], u32), ConicalReferenceVolumeError> {
    let policy = ArithmeticPolicy {
        fractional_bits: query.arithmetic.fractional_bits,
        max_operations: query.arithmetic.max_operations,
    };
    let mut arithmetic = Arithmetic::new(policy, cancelled);
    let coefficients = intervals(query, &mut arithmetic);
    let operations = arithmetic.operations();
    let coefficients = coefficients.map_err(|failure| {
        ConicalReferenceVolumeError::new(query, None, failure.depth, (operations, 0), failure.cause)
    })?;
    Ok((
        coefficients.map(|value| bounds(value, query.arithmetic.fractional_bits)),
        operations,
    ))
}
fn intervals(
    query: ConicalReferenceVolumeQuery<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<[Interval; 2], Failure> {
    let alpha = arithmetic.scalar(query.semi_angle)?;
    let [cosine, sine] = trigonometric::scalar(&alpha, arithmetic)?;
    let low = depth(query.depths[0], 0, arithmetic)?;
    let high = depth(query.depths[1], 1, arithmetic)?;
    let delta = arithmetic.subtract(&high, &low)?;
    let sum = arithmetic.add(&high, &low)?;
    let square_difference = arithmetic.multiply(&delta, &sum)?;
    let squared_slant = arithmetic.multiply(&sine, &delta)?;
    let slant = arithmetic.multiply(&cosine, &square_difference)?;
    let slant = if query.outward {
        slant
    } else {
        arithmetic.subtract(&arithmetic.zero(), &slant)?
    };
    Ok([squared_slant, slant])
}
fn depth(value: f64, index: usize, arithmetic: &mut Arithmetic<'_>) -> Result<Interval, Failure> {
    arithmetic.scalar(value).map_err(|cause| Failure {
        depth: Some(index),
        cause: ConicalReferenceVolumeFailure::Arithmetic(Box::new(cause)),
    })
}
fn bounds(interval: Interval, bits: u16) -> ContributionBounds {
    let exponent = -i32::from(bits);
    ContributionBounds {
        lower: ExactRatio::from_dyadic(&DyadicBound::new(interval.lower, exponent)),
        upper: ExactRatio::from_dyadic(&DyadicBound::new(interval.upper, exponent)),
    }
}
