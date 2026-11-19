// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::certificate_arithmetic::circular_constants;
type Failure = ConicalChartDomainFailure;
pub(super) fn admit(query: &ConicalChartDomainQuery<'_>) -> Result<(), Failure> {
    sector::admit(
        query.spans,
        query.semi_angle,
        sector_api::policy(query.options),
    )
    .map_err(Failure::Sector)?;
    if !query.chart_origin.is_finite()
        || query.chart_origin.to_bits() != query.fit_center.to_bits()
        || query.developed_turn != 0
        || query.physical_turn != 0
    {
        return Err(Failure::Chart {
            centers: [query.chart_origin, query.fit_center],
            turns: [query.developed_turn, query.physical_turn],
        });
    }
    Ok(())
}
pub(super) fn certify(
    query: &ConicalChartDomainQuery<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<ConicalChartDomainEvidence, Failure> {
    let (wedge, margin) = sector::common_wedge(query.spans, arithmetic).map_err(Failure::Sector)?;
    self::wedge(wedge)?;
    let (physical, sine, cosine, minimum_x) = ranges(query, arithmetic)?;
    width(&physical, arithmetic)?;
    arithmetic.check_cancelled()?;
    Ok(evidence(
        query, margin, physical, sine, cosine, minimum_x, arithmetic,
    ))
}
fn evidence(
    query: &ConicalChartDomainQuery<'_>,
    margin: Option<num_bigint::BigInt>,
    physical: Interval,
    sine: Interval,
    cosine: Interval,
    minimum_x: Interval,
    arithmetic: &Arithmetic<'_>,
) -> ConicalChartDomainEvidence {
    ConicalChartDomainEvidence {
        sector: sector::SectorEvidence {
            source: sector::source_bits(query.spans),
            angle: query.semi_angle.to_bits(),
            policy: sector_api::policy(query.options),
            operations: arithmetic.operations(),
            wedge: 1,
            margin: DyadicBound::new(
                margin.unwrap_or_default(),
                -i32::from(query.options.fractional_bits),
            ),
        },
        chart: [query.chart_origin.to_bits(), query.fit_center.to_bits()],
        turns: [query.developed_turn, query.physical_turn],
        physical,
        sine,
        cosine,
        minimum_x,
    }
}
fn ranges(
    query: &ConicalChartDomainQuery<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(Interval, Interval, Interval, Interval), Failure> {
    let alpha = arithmetic.scalar(query.semi_angle)?;
    let [cosine, sine] =
        crate::predicates::roots::metric_bounds::cylindrical_profile::trigonometric::scalar(
            &alpha, arithmetic,
        )
        .map_err(Failure::Trigonometric)?;
    positive_lower(&sine, "strictly positive certified sine", arithmetic)?;
    let (developed, minimum_x) = authored_range(query.spans, arithmetic)?;
    let physical = arithmetic.divide(&developed, &sine)?;
    chart_membership(&physical, arithmetic)?;
    Ok((physical, sine, cosine, minimum_x))
}
fn authored_range(
    spans: &[&[[f64; 2]]],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(Interval, Interval), Failure> {
    let mut angles = Vec::new();
    let mut coordinates = Vec::new();
    for point in spans.iter().flat_map(|span| span.iter()) {
        angles.push(
            angle::initial_angle(*point, arithmetic)
                .map_err(|cause| angle_failure(cause, *point))?,
        );
        coordinates.push(arithmetic.scalar(point[0])?);
    }
    Ok((hull(&angles), hull(&coordinates)))
}
pub(super) fn hull(values: &[Interval]) -> Interval {
    Interval {
        lower: values
            .iter()
            .map(|v| v.lower.clone())
            .min()
            .unwrap_or_default(),
        upper: values
            .iter()
            .map(|v| v.upper.clone())
            .max()
            .unwrap_or_default(),
    }
}
pub(in super::super) fn width(
    range: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(), Failure> {
    let pi = circular_constants::pi(arithmetic)?;
    let period = arithmetic.fraction(&pi, 2, 1)?;
    if &range.upper - &range.lower < period.lower {
        return Ok(());
    }
    let limit = DyadicBound::new(
        period.lower,
        -i32::from(arithmetic.policy().fractional_bits),
    );
    Err(unresolved(
        "physical angular width strictly below 2pi",
        range,
        ConicalChartDomainRequirement::WidthStrictlyBelow(limit),
        arithmetic,
    ))
}
pub(in super::super) fn chart_membership(
    relative: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(), Failure> {
    let pi = circular_constants::pi(arithmetic)?;
    if relative.lower > -pi.lower.clone() && relative.upper < pi.lower {
        return Ok(());
    }
    let limits = Interval {
        lower: -pi.lower.clone(),
        upper: pi.lower,
    };
    let expected = ConicalChartDomainRequirement::StrictlyInside(exact_enclosure(
        &limits,
        arithmetic.policy().fractional_bits,
    ));
    Err(unresolved(
        "physical lift strictly inside the common chart (-pi,pi)",
        relative,
        expected,
        arithmetic,
    ))
}
pub(in super::super) fn positive_lower(
    actual: &Interval,
    field: &'static str,
    arithmetic: &Arithmetic<'_>,
) -> Result<(), Failure> {
    if actual.lower > 0.into() {
        return Ok(());
    }
    Err(unresolved(
        field,
        actual,
        ConicalChartDomainRequirement::PositiveLower,
        arithmetic,
    ))
}
pub(in super::super) fn unresolved(
    field: &'static str,
    actual: &Interval,
    expected: ConicalChartDomainRequirement,
    arithmetic: &Arithmetic<'_>,
) -> Failure {
    Failure::Unresolved {
        bound: Box::new(ConicalChartDomainBound {
            field,
            actual: exact_enclosure(actual, arithmetic.policy().fractional_bits),
            expected,
        }),
    }
}
pub(super) fn wedge(actual_mask: u8) -> Result<(), Failure> {
    if actual_mask == 1 {
        return Ok(());
    }
    Err(Failure::Wedge {
        actual_mask,
        expected: ConicalSectorWedge::PositiveX,
    })
}

pub(in super::super) fn angle_failure(
    cause: angle::InitialAngleFailure,
    actual: [f64; 2],
) -> Failure {
    match cause {
        angle::InitialAngleFailure::Arithmetic(cause) => Failure::Arithmetic(cause),
        angle::InitialAngleFailure::Trigonometric(cause) => Failure::Trigonometric(cause),
        _ => Failure::Point {
            actual,
            expected: "finite nonzero point with a bounded principal angle",
        },
    }
}
