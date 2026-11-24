// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::metric_bounds::CylindricalProfileFailure;
type Failure = ConicalReferenceVolumeFailure;
pub(super) fn admit(
    query: ConicalReferenceVolumeQuery<'_>,
) -> Result<(), ConicalReferenceVolumeError> {
    policy(query).map_err(|cause| invalid(query, None, None, cause))?;
    support(query)?;
    for (index, span) in query.spans.iter().copied().enumerate() {
        admit_span(span, query.options.max_degree)
            .map_err(|cause| invalid(query, Some(index), None, cause))?;
    }
    Ok(())
}
fn policy(query: ConicalReferenceVolumeQuery<'_>) -> Result<(), Failure> {
    integration_policy(query.options)?;
    if !(8..=4096).contains(&query.arithmetic.fractional_bits)
        || query.arithmetic.max_operations > 1_000_000
    {
        return Err(Failure::Trigonometric(Box::new(
            CylindricalProfileFailure::Policy(query.arithmetic),
        )));
    }
    count(
        "reference spans",
        query.spans.len(),
        query.options.max_spans as usize,
    )
}
fn support(query: ConicalReferenceVolumeQuery<'_>) -> Result<(), ConicalReferenceVolumeError> {
    for (role, value) in [
        ("semi-angle in (0,pi/2)", query.semi_angle),
        ("chart origin", query.chart_origin),
        ("fit center", query.fit_center),
    ] {
        finite(role, 0, value).map_err(|cause| invalid(query, None, None, cause))?;
    }
    if query.semi_angle <= 0. || query.semi_angle >= std::f64::consts::FRAC_PI_2 {
        return Err(invalid(
            query,
            None,
            None,
            input("semi-angle in (0,pi/2)", 0, query.semi_angle),
        ));
    }
    depths(query)
}

fn admit_span(span: ConicalReferenceVolumeSpan<'_>, max_degree: u16) -> Result<(), Failure> {
    admit_span_shape(span, max_degree)?;
    for (role, values, positive) in [
        ("angular fit", span.angular_fit, false),
        ("slant numerator", span.slant_numerator, false),
        ("positive slant weights", span.slant_weights, true),
    ] {
        admit_values(role, values, positive)?;
    }
    Ok(())
}
fn admit_values(role: &'static str, values: &[f64], positive: bool) -> Result<(), Failure> {
    for (index, value) in values.iter().copied().enumerate() {
        finite(role, index, value)?;
        if positive && value <= 0. {
            return Err(Failure::Input { role, index, value });
        }
    }
    Ok(())
}
fn finite(role: &'static str, index: usize, value: f64) -> Result<(), Failure> {
    if !value.is_finite() {
        return Err(Failure::Input { role, index, value });
    }
    Ok(())
}
fn count(role: &'static str, count: usize, maximum: usize) -> Result<(), Failure> {
    if count == 0 || count > maximum {
        return Err(Failure::Shape {
            role,
            count,
            expected: "nonempty and within the requested stored-degree/occurrence cap",
        });
    }
    Ok(())
}
fn invalid(
    query: ConicalReferenceVolumeQuery<'_>,
    span: Option<usize>,
    depth: Option<usize>,
    cause: Failure,
) -> ConicalReferenceVolumeError {
    ConicalReferenceVolumeError::new(query, span, depth, (0, 0), cause)
}

fn integration_policy(o: CylindricalFluxOptions) -> Result<(), Failure> {
    if o.behavior_version != 1
        || o.max_degree > 16
        || !(1..=4096).contains(&o.max_spans)
        || !(1..=65536).contains(&o.max_subintervals_per_span)
        || o.max_total_subintervals > 1_000_000
        || !o.absolute_error_limit.is_finite()
        || o.absolute_error_limit < 0.
    {
        return Err(Failure::Policy(o));
    }
    Ok(())
}

fn depths(query: ConicalReferenceVolumeQuery<'_>) -> Result<(), ConicalReferenceVolumeError> {
    for (index, depth) in query.depths.into_iter().enumerate() {
        finite("normal depth", index, depth)
            .map_err(|cause| invalid(query, None, Some(index), cause))?;
    }
    if query.depths[0] >= query.depths[1] {
        return Err(invalid(
            query,
            None,
            Some(1),
            input("last depth strictly above first", 1, query.depths[1]),
        ));
    }
    Ok(())
}

fn admit_span_shape(span: ConicalReferenceVolumeSpan<'_>, max_degree: u16) -> Result<(), Failure> {
    count(
        "angular fit coefficients",
        span.angular_fit.len(),
        4.min(usize::from(max_degree) + 1),
    )?;
    count(
        "slant numerator coefficients",
        span.slant_numerator.len(),
        10.min(usize::from(max_degree) + 1),
    )?;
    if span.slant_weights.len() != span.slant_numerator.len() {
        return Err(Failure::Shape {
            role: "slant weights",
            count: span.slant_weights.len(),
            expected: "same count as slant numerator",
        });
    }
    Ok(())
}
fn input(role: &'static str, index: usize, value: f64) -> Failure {
    Failure::Input { role, index, value }
}
