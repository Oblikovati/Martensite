// SPDX-License-Identifier: GPL-2.0-only
//! Checked immutable spatial receipts feed the existing developed-coordinate join theorem.
use super::*;
#[cfg(test)]
mod endpoint_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
pub(in super::super) fn prepare_projection_in_chart<'a>(
    request: &'a SpatialRequest<'a>,
    span: &ConicalJoinSpan<'_>,
    domain: &ConicalChartDomainEvidence,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<PreparedProjection<'a>, CombinedFailure> {
    prepare_projection_in_chart_selected(
        request,
        span,
        domain,
        ConicalPrimitiveRefinement::UnsplitV1,
        arithmetic,
    )
}
pub(in super::super) fn prepare_projection_in_chart_selected<'a>(
    request: &'a SpatialRequest<'a>,
    span: &ConicalJoinSpan<'_>,
    domain: &ConicalChartDomainEvidence,
    refinement: ConicalPrimitiveRefinement,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<PreparedProjection<'a>, CombinedFailure> {
    let query = checked_query(request, &span.query).ok_or(CombinedFailure::JoinMismatch)?;
    let retained = span
        .boundary
        .retained_in_chart_with_refinement(&query, domain, arithmetic.fractional_bits(), refinement)
        .ok_or(CombinedFailure::JoinMismatch)?;
    let bounds = source_bounds(request, retained, arithmetic)?;
    let error = derivative_error(&bounds, arithmetic)?;
    let poles = source_poles(&retained.capture, arithmetic)?;
    let authored = endpoint_pair(source_controls(&retained.capture)?);
    Ok(PreparedProjection {
        request,
        poles,
        error,
        authored: Some(authored),
    })
}
fn checked_query<'a>(
    request: &SpatialRequest<'a>,
    original: &ConicalBoundaryQuery<'a>,
) -> Option<ConicalBoundaryQuery<'a>> {
    if request.source.first().map(|p| p.map(f64::to_bits))
        != Some(request.anchor.point.map(f64::to_bits))
        || request.angular_fit.first().map(|v| v.to_bits())
            != Some(request.anchor.fit_half_angle_tangent.to_bits())
    {
        return None;
    }
    if original.original.is_none() && !normalized_original(request) {
        return None;
    }
    Some(request_query(request, original))
}
fn request_query<'a>(
    request: &SpatialRequest<'a>,
    original: &ConicalBoundaryQuery<'a>,
) -> ConicalBoundaryQuery<'a> {
    let mut query = *original;
    query.original = original.original.and(request.original);
    query.source = request.source;
    query.angular_fit = request.angular_fit;
    query.slant_numerator = request.slant_numerator;
    query.slant_weights = request.weights;
    query.semi_angle = request.anchor.semi_angle;
    query.chart_origin = request.anchor.origin;
    query.fit_center = request.anchor.fit_center;
    query.developed_turn = request.anchor.developed_turn;
    query.physical_turn = request.anchor.physical_turn;
    query.depths = request.depths;
    query.outward = request.outward;
    query.spatial_limit = request.limit;
    query
}
fn normalized_original(request: &SpatialRequest<'_>) -> bool {
    request.original.is_some_and(|source| {
        source.index == 0
            && source.level == 0
            && source.poles.len() == request.source.len()
            && source
                .poles
                .iter()
                .flatten()
                .map(|v| v.to_bits())
                .eq(request.source.iter().flatten().map(|v| v.to_bits()))
    })
}
fn source_bounds(
    request: &SpatialRequest<'_>,
    retained: &RetainedSpatial,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Bounds, CombinedFailure> {
    let capture = &retained.capture;
    let (fit_rate, rate) = derivative_bounds(capture, arithmetic)?;
    let fit = fit_bounds_metric(
        request,
        &capture.numerator,
        &capture.weights,
        rate,
        arithmetic,
    )
    .map_err(CombinedFailure::Spatial)?;
    let (radial, angular, original_rate) = original_bounds(capture, arithmetic)?;
    let radial_rate_error = arithmetic.add(&fit_rate, &original_rate)?;
    assemble_bounds(
        &retained.evidence,
        radial_rate_error,
        fit,
        (radial, angular),
        (&capture.slant, &capture.angular),
        arithmetic,
    )
}
fn derivative_bounds(
    capture: &SpatialCapture,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(Interval, Option<Interval>), CombinedFailure> {
    if let Some(proof) = &capture.slant_proof {
        let result = super::super::slant_derivative::retained_derivative(
            &capture.q,
            &capture.numerator,
            proof,
            arithmetic,
        )
        .map_err(|cause| CombinedFailure::Spatial(SpatialFailure::Original(cause)))?;
        if let Some((error, rate)) = result {
            return Ok((error, Some(rate)));
        }
    }
    let error =
        radial_derivative_bound(&capture.q, &capture.numerator, &capture.weights, arithmetic)
            .map_err(|cause| CombinedFailure::Spatial(SpatialFailure::Slant(cause)))?;
    Ok((error, None))
}
fn original_bounds(
    capture: &SpatialCapture,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(Interval, Interval, Interval), CombinedFailure> {
    if let Some(comparison) = &capture.comparison {
        if comparison.displacement.anchor.turn_adjustment != Some(0) {
            return Err(CombinedFailure::UnalignedAnchor);
        }
        if comparison.displacement.delta.upper == 0.into()
            && comparison.displacement.derivative.upper == 0.into()
        {
            return Ok((arithmetic.zero(), arithmetic.zero(), arithmetic.zero()));
        }
    }
    let Some(comparison) = &capture.comparison else {
        return Ok((arithmetic.zero(), arithmetic.zero(), arithmetic.zero()));
    };
    let (radial, angular, rate, _) =
        original::polar_from_retained(comparison, &capture.source, &capture.sine, arithmetic)
            .map_err(|cause| CombinedFailure::Spatial(SpatialFailure::Original(cause)))?;
    Ok((radial, angular, rate))
}
fn source_poles(
    capture: &SpatialCapture,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Vec<[Interval; 2]>, CombinedFailure> {
    let source = source_controls(capture)?;
    let x = derivative(&source[0], arithmetic)?;
    let y = derivative(&source[1], arithmetic)?;
    Ok(crate::predicates::roots::certificate_arithmetic::bernstein_curve::align_planar(&x, &y))
}

fn source_controls(capture: &SpatialCapture) -> Result<&[Vec<Interval>; 2], CombinedFailure> {
    let source = match &capture.comparison {
        Some(comparison) => comparison
            .restricted
            .as_ref()
            .ok_or(CombinedFailure::MissingOriginal)?,
        None => &capture.source,
    };
    Ok(source)
}
fn endpoint_pair(source: &[Vec<Interval>; 2]) -> [[Interval; 2]; 2] {
    [
        std::array::from_fn(|axis| source[axis][0].clone()),
        std::array::from_fn(|axis| source[axis][source[axis].len() - 1].clone()),
    ]
}
