// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::metric_bounds::{
    CylindricalProfileFailure, cylindrical_profile::trigonometric,
};
use crate::predicates::roots::{
    DyadicBound,
    certificate_arithmetic::bounded_dyadic::{Arithmetic, ArithmeticPolicy, Interval},
};

pub(in super::super) fn coefficients(
    query: AnalyticConicalFluxQuery<'_>,
    cancelled: &mut (impl FnMut() -> bool + Send),
    operations: &mut u32,
) -> Checked<[Density; 2]> {
    let policy = ArithmeticPolicy {
        fractional_bits: query.arithmetic.fractional_bits,
        max_operations: query.arithmetic.max_operations,
    };
    let mut arithmetic = Arithmetic::new(policy, cancelled);
    let result = both(query, &mut arithmetic);
    *operations = arithmetic.operations();
    result
}
fn both(
    query: AnalyticConicalFluxQuery<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[Density; 2]> {
    let first = density(query.carriers[0], 0, arithmetic)?;
    let last = density(query.carriers[1], 1, arithmetic)?;
    Ok([first, last])
}

fn density(
    carrier: ConicalTrimCarrier,
    depth: usize,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Density> {
    density_intervals(carrier, arithmetic)
        .map(|[constant, meridian]| Density {
            constant: bounds(constant, arithmetic.fractional_bits()),
            meridian: bounds(meridian, arithmetic.fractional_bits()),
        })
        .map_err(|cause| AnalyticConicalFluxFailure::Coefficients {
            depth,
            cause: Box::new(cause),
        })
}

fn density_intervals(
    carrier: ConicalTrimCarrier,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<[Interval; 2], CylindricalProfileFailure> {
    let alpha = arithmetic.scalar(carrier.semi_angle)?;
    let [cosine, sine] = trigonometric::scalar(&alpha, arithmetic)?;
    let radius = arithmetic.scalar(carrier.radius)?;
    let origin = arithmetic.scalar(carrier.axial_origin)?;
    let radial = arithmetic.multiply(&cosine, &radius)?;
    let axial = arithmetic.multiply(&sine, &origin)?;
    let offset = arithmetic.subtract(&radial, &axial)?;
    let constant = arithmetic.multiply(&radius, &offset)?;
    let meridian = arithmetic.multiply(&sine, &offset)?;
    Ok([
        arithmetic.fraction(&constant, 1, 3)?,
        arithmetic.fraction(&meridian, 1, 3)?,
    ])
}

fn bounds(interval: Interval, fractional_bits: u16) -> ContributionBounds {
    let exponent = -i32::from(fractional_bits);
    ContributionBounds {
        lower: ExactRatio::from_dyadic(&DyadicBound::new(interval.lower, exponent)),
        upper: ExactRatio::from_dyadic(&DyadicBound::new(interval.upper, exponent)),
    }
}
