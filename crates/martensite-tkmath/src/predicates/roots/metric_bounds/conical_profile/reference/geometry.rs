// SPDX-License-Identifier: GPL-2.0-only
//! The original/ideal homotopy controls topology and filled material before native storage.
use super::*;
pub(super) fn certify(
    query: &ConicalReferenceQuery<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<[f64; 3], ConicalReferenceFailure> {
    let [displacement, storage] = envelopes(query, arithmetic)?;
    allocations(query, &displacement, &storage, arithmetic)?;
    let filled = filled(query, &displacement, arithmetic)?;
    allocated(
        &filled,
        query.spatial_limit,
        1,
        "filled reference normal-body discrepancy",
        arithmetic,
    )?;
    Ok([
        arithmetic.upper_f64(displacement.upper)?,
        arithmetic.upper_f64(filled.upper)?,
        arithmetic.upper_f64(storage.upper)?,
    ])
}
fn allocations(
    query: &ConicalReferenceQuery<'_>,
    displacement: &Interval,
    storage: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(), ConicalReferenceFailure> {
    allocated(
        displacement,
        query.spatial_limit,
        4,
        "original-to-ideal developed displacement",
        arithmetic,
    )?;
    allocated(
        storage,
        query.spatial_limit,
        1000,
        "decoded rational edge and full-depth ruling roundoff",
        arithmetic,
    )?;
    topology::separation(query, displacement, arithmetic)?;
    angular_chart(query, arithmetic)?;
    Ok(())
}

fn envelopes(
    query: &ConicalReferenceQuery<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<[Interval; 2], ConicalReferenceFailure> {
    let mut result = [arithmetic.zero(), arithmetic.zero()];
    for span in query.members.iter().flat_map(|member| member.spans) {
        // Existing ideal angular terms use (maxL+max|d|)*physicalAngleError;
        // development uses maxL*sin(alpha)*physicalAngleError. The former dominates.
        let ideal = capture(span.boundary.ideal(), arithmetic)?;
        let original = capture(span.boundary.original(), arithmetic)?;
        let developed = arithmetic.add(&ideal, &original)?;
        result[0].upper = result[0].upper.clone().max(developed.upper);
        result[1].upper = result[1]
            .upper
            .clone()
            .max(stored(span.boundary, arithmetic)?.upper);
    }
    Ok(result)
}
fn stored(
    proof: &ConicalBoundaryEvidence,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, ConicalReferenceFailure> {
    let first = capture(&proof.storage()[0], arithmetic)?;
    let last = capture(&proof.storage()[1], arithmetic)?;
    Ok(Interval {
        lower: 0.into(),
        upper: first.upper.max(last.upper),
    })
}
fn capture(
    bound: &DyadicBound,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, ConicalReferenceFailure> {
    let upper = bound
        .binary64_bounds()
        .map(|range| range[1])
        .ok_or_else(|| binding_failure("finite nonnegative component enclosure"))?;
    if upper < 0. {
        return Err(binding_failure("nonnegative component enclosure"));
    }
    Ok(arithmetic.scalar(upper)?)
}
pub(super) fn allocated(
    bound: &Interval,
    epsilon: f64,
    denominator: u64,
    field: &'static str,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(), ConicalReferenceFailure> {
    let epsilon = arithmetic.scalar(epsilon)?;
    let allowance = arithmetic.fraction(&epsilon, 1, denominator)?;
    if bound.upper > allowance.lower {
        return Err(ConicalReferenceFailure::Bound {
            field,
            actual: arithmetic.upper_f64(bound.upper.clone())?,
            limit: arithmetic.upper_f64(allowance.lower)?,
        });
    }
    Ok(())
}
fn filled(
    query: &ConicalReferenceQuery<'_>,
    displacement: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, ConicalReferenceFailure> {
    let minimum = minimum_x(query, arithmetic)?;
    let halfplane = arithmetic.subtract(&minimum, displacement)?;
    positive(
        &halfplane,
        "positive halfplane for both filled domains",
        arithmetic,
    )?;
    let input = query.members[0].spans[0].query;
    let angle = arithmetic.scalar(input.semi_angle)?;
    let [cosine, sine] = trigonometric::scalar(&angle, arithmetic)
        .map_err(ConicalReferenceFailure::Trigonometric)?;
    radius(input.depths, &halfplane, &sine, &cosine, arithmetic)?;
    let factor = distortion(input.depths, &halfplane, &sine, &cosine, arithmetic)?;
    // Shared monotone joins and disjoint nonadjacent tubes give an isotopy. Winding
    // then bounds filled planar Hausdorff error; this global Lipschitz map transports it.
    Ok(arithmetic.multiply(displacement, &factor)?)
}
fn minimum_x(
    query: &ConicalReferenceQuery<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, ConicalReferenceFailure> {
    let mut minimum = None;
    for x in query
        .sector_spans
        .iter()
        .flat_map(|span| span.iter().map(|point| point[0]))
    {
        let value = arithmetic.scalar(x)?;
        minimum = Some(
            minimum.map_or(value.lower.clone(), |old: num_bigint::BigInt| {
                old.min(value.lower)
            }),
        );
    }
    let lower = minimum.ok_or_else(|| binding_failure("nonempty original filled material"))?;
    Ok(Interval {
        lower: lower.clone(),
        upper: lower,
    })
}
pub(super) fn radius(
    depths: [f64; 2],
    halfplane: &Interval,
    sine: &Interval,
    cosine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(), ConicalReferenceFailure> {
    let radial = arithmetic.multiply(halfplane, sine)?;
    let depth = arithmetic.scalar(depths[0])?;
    let offset = arithmetic.multiply(&depth, cosine)?;
    positive(
        &arithmetic.add(&radial, &offset)?,
        "positive full-depth filled-reference normal-map radius",
        arithmetic,
    )
}
pub(super) fn distortion(
    depths: [f64; 2],
    halfplane: &Interval,
    sine: &Interval,
    cosine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, ConicalReferenceFailure> {
    let depth = arithmetic.scalar(depths[0].abs().max(depths[1].abs()))?;
    let numerator = arithmetic.multiply(&depth, cosine)?;
    let denominator = arithmetic.multiply(halfplane, sine)?;
    let quotient = arithmetic.divide(&numerator, &denominator)?;
    let one = arithmetic.scalar(1.)?;
    Ok(arithmetic.add(&one, &quotient)?)
}
fn positive(
    bound: &Interval,
    field: &'static str,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(), ConicalReferenceFailure> {
    if bound.lower <= 0.into() {
        return Err(ConicalReferenceFailure::Bound {
            field,
            actual: arithmetic.upper_f64(bound.lower.clone())?,
            limit: 0.,
        });
    }
    Ok(())
}

fn angular_chart(
    query: &ConicalReferenceQuery<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(), ConicalReferenceFailure> {
    let one = arithmetic.scalar(1.)?;
    for value in query
        .members
        .iter()
        .flat_map(|member| member.spans)
        .flat_map(|span| span.query.angular_fit)
    {
        angular_coefficient(*value, &one, arithmetic)?;
    }
    // The common center equals the chart origin. Thus theta-center=2atan(q)
    // stays in (-pi/2,pi/2), and its continuous development has no turn ambiguity.
    // On the compact PositiveX material, arg has no interior critical point;
    // its extrema occur on this boundary, so the filled domain inherits the range.
    Ok(())
}

pub(super) fn angular_coefficient(
    value: f64,
    one: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(), ConicalReferenceFailure> {
    let bound = arithmetic.scalar(value)?;
    if bound.magnitude() >= one.lower {
        return Err(binding_failure(
            "strict common ideal half-angle coefficient bounds inside (-1,1)",
        ));
    }
    Ok(())
}
