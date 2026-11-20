// SPDX-License-Identifier: GPL-2.0-only
//! A local chart may cover an exact dyadic restriction of an admitted source.
use super::*;
use crate::predicates::roots::certificate_arithmetic::bernstein_curve::{
    align_planar, capture_planar, restrict,
};
use crate::predicates::roots::certificate_arithmetic::circular_constants;
use num_bigint::BigInt;

#[cfg(test)]
mod tests;

/// Refusal to certify a dyadic source interval in a local cone chart.
#[derive(Debug)]
pub enum ConicalSubspanChartError {
    /// The supplied dyadic address is outside the source parameter domain.
    Interval(ConicalOriginalFailure),
    /// The source, chart, or bounded arithmetic could not be certified.
    Domain(ConicalChartDomainError),
}
impl std::fmt::Display for ConicalSubspanChartError {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Interval(cause) => cause.fmt(out),
            Self::Domain(cause) => cause.fmt(out),
        }
    }
}
impl std::error::Error for ConicalSubspanChartError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Interval(cause) => Some(cause),
            Self::Domain(cause) => Some(cause),
        }
    }
}

/// Certify a recentered chart over one exact dyadic interval of one original span.
/// Example: `certify_conical_subspan_chart(query, 1, 2, || false)?` covers [1/4, 1/2].
pub fn certify_conical_subspan_chart(
    query: ConicalChartDomainQuery<'_>,
    index: u64,
    level: u8,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<ConicalChartDomainEvidence, ConicalSubspanChartError> {
    let version = ConicalChartDomainVersion::RecenteredV4;
    let origin = admit(&query, index, level, version)?;
    let mut arithmetic = Arithmetic::new(sector_api::policy(query.options), &mut cancelled);
    bounds::certify(&query, version, Some(origin), &mut arithmetic).map_err(|cause| {
        ConicalSubspanChartError::Domain(domain_error(cause, arithmetic.operations()))
    })
}

fn admit<'a>(
    query: &ConicalChartDomainQuery<'a>,
    index: u64,
    level: u8,
    version: ConicalChartDomainVersion,
) -> Result<ConicalOriginalSpan<'a>, ConicalSubspanChartError> {
    let [poles] = query.spans else {
        let cause = ConicalChartDomainFailure::SourceCount {
            actual: query.spans.len(),
            limit: 1,
        };
        return Err(ConicalSubspanChartError::Domain(domain_error(cause, 0)));
    };
    let origin = ConicalOriginalSpan {
        poles,
        index,
        level,
    };
    original::admit_interval(origin).map_err(ConicalSubspanChartError::Interval)?;
    bounds::admit(query, version)
        .map_err(|cause| ConicalSubspanChartError::Domain(domain_error(cause, 0)))?;
    Ok(origin)
}

pub(super) fn covers((index, level): (u64, u8), span: ConicalOriginalSpan<'_>) -> bool {
    original::admit_interval(span).is_ok()
        && span
            .level
            .checked_sub(level)
            .is_some_and(|extra| span.index >> extra == index)
}

pub(super) fn restricted_range(
    original: ConicalOriginalSpan<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(Interval, Interval), ConicalChartDomainFailure> {
    let [x, y] = capture_planar(original.poles, arithmetic)?;
    let x = restrict(&x, original.index, original.level, arithmetic)?;
    let y = restrict(&y, original.index, original.level, arithmetic)?;
    let (slopes, minimum_x) = slope_hull(&x, &y, arithmetic)?;
    let lower = slope_angle(slopes.lower, arithmetic)?;
    let upper = slope_angle(slopes.upper, arithmetic)?;
    Ok((
        Interval {
            lower: lower.lower,
            upper: upper.upper,
        },
        minimum_x,
    ))
}

fn slope_hull(
    x: &[Interval],
    y: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(Interval, Interval), ConicalChartDomainFailure> {
    let (mut slopes, mut coordinates) = (None, None);
    // For positive x, every curve slope is a convex average of its pole slopes.
    for [x, y] in align_planar(x, y) {
        bounds::positive_lower(&x, "strictly positive restricted control x", arithmetic)?;
        bounds::include(&mut slopes, arithmetic.divide(&y, &x)?);
        bounds::include(&mut coordinates, x);
    }
    Ok((
        slopes.unwrap_or_else(|| arithmetic.zero()),
        coordinates.unwrap_or_else(|| arithmetic.zero()),
    ))
}

fn slope_angle(
    slope: BigInt,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, ConicalChartDomainFailure> {
    let negative = slope.sign() == num_bigint::Sign::Minus;
    let slope = if negative { -slope } else { slope };
    let slope = Interval {
        lower: slope.clone(),
        upper: slope,
    };
    let angle = positive_angle(&slope, arithmetic)
        .map_err(|cause| angle_error(cause, &slope, arithmetic))?;
    if negative {
        return Ok(arithmetic.subtract(&arithmetic.zero(), &angle)?);
    }
    Ok(angle)
}

fn positive_angle(
    slope: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> angle::AngleResult<Interval> {
    if slope.upper <= arithmetic.scalar(0.5)?.lower {
        return angle::atan_series(slope, arithmetic);
    }
    // Centering around pi/4 keeps the atan series conditioned throughout the wedge.
    let reduced = angle::reduce_ratio(slope, arithmetic)?;
    let correction = angle::atan_series(&reduced, arithmetic)?;
    let pi = circular_constants::pi(arithmetic)?;
    let quarter = arithmetic.fraction(&pi, 1, 4)?;
    Ok(arithmetic.add(&quarter, &correction)?)
}

fn angle_error(
    cause: angle::InitialAngleFailure,
    slope: &Interval,
    arithmetic: &Arithmetic<'_>,
) -> ConicalChartDomainFailure {
    match cause {
        angle::InitialAngleFailure::Arithmetic(cause) => {
            ConicalChartDomainFailure::Arithmetic(cause)
        }
        angle::InitialAngleFailure::Trigonometric(cause) => {
            ConicalChartDomainFailure::Trigonometric(cause)
        }
        _ => ConicalChartDomainFailure::Point {
            actual: [
                1.,
                DyadicBound::new(
                    slope.upper.clone(),
                    -i32::from(arithmetic.fractional_bits()),
                )
                .binary64_bounds()
                .map_or(f64::INFINITY, |bound| bound[1]),
            ],
            expected: "slope with a bounded principal angle",
        },
    }
}
