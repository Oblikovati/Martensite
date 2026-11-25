// SPDX-License-Identifier: GPL-2.0-only
//! Linear extrema on compact material occur on its boundary, enclosed by Bernstein hulls.
use super::*;
pub(super) struct Domain {
    pub(super) uv: [Interval; 2],
    pub(super) halfplane: Interval,
    pub(super) error: Interval,
}
pub(super) fn certify(
    query: &ConicalMaterialQuery<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<f64, ConicalMaterialError> {
    let angle = arithmetic
        .scalar(query.carriers[0].semi_angle)
        .map_err(|cause| error(None, cause.into(), arithmetic.operations()))?;
    let [cosine, sine] = trigonometric::scalar(&angle, arithmetic)
        .map_err(|cause| error(None, cause, arithmetic.operations()))?;
    let domain = domain(query, &sine, arithmetic)?;
    finish(query, domain, &sine, &cosine, arithmetic)
        .map_err(|cause| error(None, cause, arithmetic.operations()))
}
fn domain(
    query: &ConicalMaterialQuery<'_>,
    sine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Domain, ConicalMaterialError> {
    let mut values = Vec::new();
    for (index, span) in query.spans.iter().enumerate() {
        values.push(
            source::certify(query, span, sine, arithmetic)
                .map_err(|cause| error(Some(index), cause, arithmetic.operations()))?,
        );
    }
    Ok(Domain {
        uv: std::array::from_fn(|axis| domain_hull(&values, |value| value.uv[axis].clone())),
        halfplane: domain_hull(&values, |value| value.halfplane.clone()),
        error: domain_hull(&values, |value| value.error.clone()),
    })
}
fn domain_hull(
    values: &[source::SpanBounds],
    select: impl Fn(&source::SpanBounds) -> Interval,
) -> Interval {
    algebra::hull(&values.iter().map(select).collect::<Vec<_>>())
}

fn finish(
    query: &ConicalMaterialQuery<'_>,
    domain: Domain,
    sine: &Interval,
    cosine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<f64, CylindricalProfileFailure> {
    regular(query, &domain, sine, cosine, arithmetic)?;
    positive(
        "common developed filled-material halfplane",
        &domain.halfplane,
        arithmetic,
    )?;
    common_radius(query, &domain.halfplane, sine, cosine, arithmetic)?;
    original_angular_width(&domain, sine, arithmetic)?;
    // Both keyed boundaries enclose independently certified compact material. Their
    // straight parameter homotopy stays in the reported boundary-error bands, so
    // winding agrees outside those bands; this bounds filled-domain Hausdorff error.
    let distortion = distortion(query.depths, &domain.halfplane, sine, cosine, arithmetic)?;
    let error = arithmetic.multiply(&domain.error, &distortion)?;
    let meridian = meridian_error(query, sine, cosine, arithmetic)?;
    let total = arithmetic.add(&error, &meridian)?;
    accept(arithmetic.upper_f64(total.upper)?, query.spatial_limit)
}
fn accept(upper: f64, limit: f64) -> Result<f64, CylindricalProfileFailure> {
    if upper > limit {
        return Err(CylindricalProfileFailure::Uncertified {
            bound: upper,
            requested: limit,
        });
    }
    Ok(upper)
}
fn regular(
    query: &ConicalMaterialQuery<'_>,
    domain: &Domain,
    sine: &Interval,
    cosine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(), CylindricalProfileFailure> {
    let reference = arithmetic.scalar(query.reference_slant)?;
    let slant = arithmetic.add(&reference, &domain.uv[1])?;
    positive("filled-material slant", &slant, arithmetic)?;
    let depths = depth_hull(query.depths, arithmetic)?;
    let radial = arithmetic.multiply(&slant, sine)?;
    let offset = arithmetic.multiply(&depths, cosine)?;
    positive(
        "filled-material normal-map radius",
        &arithmetic.add(&radial, &offset)?,
        arithmetic,
    )?;
    actual_radii(query, &domain.uv[1], sine, arithmetic)?;
    angular_width(&domain.uv[0], arithmetic)
}
fn angular_width(
    u: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(), CylindricalProfileFailure> {
    let width = Interval {
        lower: &u.upper - &u.lower,
        upper: &u.upper - &u.lower,
    };
    below_period(
        width,
        "filled-material angular width strictly below 2pi",
        arithmetic,
    )
}
fn below_period(
    width: Interval,
    role: &'static str,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(), CylindricalProfileFailure> {
    let pi = crate::predicates::roots::certificate_arithmetic::circular_constants::pi(arithmetic)?;
    let period = arithmetic.fraction(&pi, 2, 1)?;
    if width.upper >= period.lower {
        return Err(input::value(role, arithmetic.upper_f64(width.upper)?));
    }
    Ok(())
}

fn distortion(
    depths: [f64; 2],
    halfplane: &Interval,
    sine: &Interval,
    cosine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, CylindricalProfileFailure> {
    // On the common convex halfplane L>=c, the normal offset map has singular values
    // 1 and |1+d*cot(alpha)/L|. A straight segment therefore gives this global bound.
    let depth = arithmetic.scalar(depths[0].abs().max(depths[1].abs()))?;
    let lower = Interval {
        lower: halfplane.lower.clone(),
        upper: halfplane.lower.clone(),
    };
    let denominator = arithmetic.multiply(&lower, sine)?;
    let numerator = arithmetic.multiply(&depth, cosine)?;
    let ratio = arithmetic.divide(&numerator, &denominator)?;
    let one = arithmetic.scalar(1.)?;
    Ok(arithmetic.add(&one, &ratio)?)
}
fn meridian_error(
    query: &ConicalMaterialQuery<'_>,
    sine: &Interval,
    cosine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, CylindricalProfileFailure> {
    let reference = arithmetic.scalar(query.reference_slant)?;
    let radius = arithmetic.multiply(&reference, sine)?;
    let height = arithmetic.multiply(&reference, cosine)?;
    let mut errors = Vec::new();
    for (index, carrier) in query.carriers.iter().enumerate() {
        errors.push(cap_error(
            *carrier,
            query.depths[index],
            (&radius, &height),
            (sine, cosine),
            arithmetic,
        )?);
    }
    Ok(algebra::hull(&errors))
}
fn cap_error(
    carrier: ConicalTrimCarrier,
    depth: f64,
    reference: (&Interval, &Interval),
    trig: (&Interval, &Interval),
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, CylindricalProfileFailure> {
    let depth = arithmetic.scalar(depth)?;
    let r_offset = arithmetic.multiply(&depth, trig.1)?;
    let z_offset = arithmetic.multiply(&depth, trig.0)?;
    let radius = arithmetic.add(reference.0, &r_offset)?;
    let height = arithmetic.subtract(reference.1, &z_offset)?;
    let actual_r = arithmetic.scalar(carrier.radius)?;
    let actual_z = arithmetic.scalar(carrier.axial_origin)?;
    let dr = arithmetic.subtract(&actual_r, &radius)?;
    let dz = arithmetic.subtract(&actual_z, &height)?;
    Ok(arithmetic.add(&algebra::magnitude(&[dr]), &algebra::magnitude(&[dz]))?)
}
fn depth_hull(
    depths: [f64; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, CylindricalProfileFailure> {
    Ok(algebra::hull(&[
        arithmetic.scalar(depths[0])?,
        arithmetic.scalar(depths[1])?,
    ]))
}
fn positive(
    role: &'static str,
    value: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(), CylindricalProfileFailure> {
    if value.lower <= 0.into() {
        return Err(input::value(
            role,
            arithmetic.upper_f64(value.lower.clone())?,
        ));
    }
    Ok(())
}

fn actual_radii(
    query: &ConicalMaterialQuery<'_>,
    v: &Interval,
    sine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(), CylindricalProfileFailure> {
    let scaled = arithmetic.multiply(v, sine)?;
    for carrier in query.carriers {
        let reference = arithmetic.scalar(carrier.radius)?;
        let radius = arithmetic.add(&reference, &scaled)?;
        positive("filled actual cap radius", &radius, arithmetic)?;
    }
    Ok(())
}

fn common_radius(
    query: &ConicalMaterialQuery<'_>,
    halfplane: &Interval,
    sine: &Interval,
    cosine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(), CylindricalProfileFailure> {
    let minimum = Interval {
        lower: halfplane.lower.clone(),
        upper: halfplane.lower.clone(),
    };
    let base = arithmetic.multiply(&minimum, sine)?;
    let depths = depth_hull(query.depths, arithmetic)?;
    let offset = arithmetic.multiply(&depths, cosine)?;
    let radius = arithmetic.add(&base, &offset)?;
    positive(
        "both filled domains full-depth normal radius",
        &radius,
        arithmetic,
    )
}

fn original_angular_width(
    domain: &Domain,
    sine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(), CylindricalProfileFailure> {
    // Angle has gradient norm 1/L on the common convex halfplane. Boundary
    // correspondence therefore bounds the original angular range as well.
    let minimum = halfplane_minimum(&domain.halfplane);
    let denominator = arithmetic.multiply(&minimum, sine)?;
    let margin = arithmetic.divide(&domain.error, &denominator)?;
    let margin = arithmetic.fraction(&margin, 2, 1)?;
    let width = Interval {
        lower: &domain.uv[0].upper - &domain.uv[0].lower,
        upper: &domain.uv[0].upper - &domain.uv[0].lower,
    };
    let width = arithmetic.add(&width, &margin)?;
    below_period(
        width,
        "original filled-material angular width plus certified margin below 2pi",
        arithmetic,
    )
}

fn halfplane_minimum(halfplane: &Interval) -> Interval {
    Interval {
        lower: halfplane.lower.clone(),
        upper: halfplane.lower.clone(),
    }
}

pub(super) fn certify_developed(
    query: &ConicalMaterialQuery<'_>,
    contexts: &[ConicalDevelopedBoundarySpan<'_>],
    domain: &ConicalChartDomainEvidence,
    refinement: ConicalPrimitiveRefinement,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<f64, ConicalDevelopedMaterialError> {
    let half_pi = super::super::conical_profile::developed_boundary::half_pi(arithmetic)
        .map_err(|cause| developed::error::numerical(None, cause, arithmetic.operations()))?;
    let values = developed_values(query, contexts, domain, refinement, &half_pi, arithmetic)?;
    let bounds = Domain {
        uv: std::array::from_fn(|axis| domain_hull(&values, |value| value.uv[axis].clone())),
        halfplane: domain_hull(&values, |value| value.halfplane.clone()),
        error: domain_hull(&values, |value| value.error.clone()),
    };
    let [sine, cosine] = super::super::conical_profile::developed_boundary::trig(domain);
    finish(query, bounds, sine, cosine, arithmetic)
        .map_err(|cause| developed::error::numerical(None, cause, arithmetic.operations()))
}
fn developed_values(
    query: &ConicalMaterialQuery<'_>,
    contexts: &[ConicalDevelopedBoundarySpan<'_>],
    domain: &ConicalChartDomainEvidence,
    refinement: ConicalPrimitiveRefinement,
    half_pi: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Vec<source::SpanBounds>, ConicalDevelopedMaterialError> {
    query
        .spans
        .iter()
        .zip(contexts)
        .enumerate()
        .map(|(index, (span, context))| {
            developed_value(
                query, span, context, domain, refinement, half_pi, arithmetic,
            )
            .map_err(|cause| {
                developed::error::numerical(Some(index), cause, arithmetic.operations())
            })
        })
        .collect()
}
fn developed_value(
    query: &ConicalMaterialQuery<'_>,
    span: &ConicalMaterialSpan<'_>,
    context: &ConicalDevelopedBoundarySpan<'_>,
    domain: &ConicalChartDomainEvidence,
    refinement: ConicalPrimitiveRefinement,
    half_pi: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<source::SpanBounds, CylindricalProfileFailure> {
    let (uv, halfplane, error) =
        super::super::conical_profile::developed_boundary::span_bounds_with_refinement(
            query, span, context, domain, half_pi, arithmetic, refinement,
        )?;
    Ok(source::SpanBounds {
        uv,
        halfplane,
        error,
    })
}
