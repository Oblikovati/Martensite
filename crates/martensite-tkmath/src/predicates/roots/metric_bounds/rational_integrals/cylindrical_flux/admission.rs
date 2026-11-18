// SPDX-License-Identifier: GPL-2.0-only
use super::*;

pub(super) fn admit(query: CylindricalFluxQuery<'_>) -> Checked<()> {
    policy(query.options)?;
    if query.spans.is_empty() || query.spans.len() > query.options.max_spans as usize {
        return Err(CylindricalFluxFailure::SpanCount {
            actual: query.spans.len(),
            maximum: query.options.max_spans,
        });
    }
    if query.radii.iter().any(|r| !r.is_finite() || *r <= 0.) || query.radii[0] >= query.radii[1] {
        return Err(CylindricalFluxFailure::Radii(query.radii));
    }
    for (index, span) in query.spans.iter().enumerate() {
        admit_span(index, *span, query.options)?;
    }
    Ok(())
}

fn policy(options: CylindricalFluxOptions) -> Checked<()> {
    if options.behavior_version != 1
        || options.max_degree > 16
        || !(1..=4096).contains(&options.max_spans)
        || !(1..=65536).contains(&options.max_subintervals_per_span)
        || options.max_total_subintervals > 1_000_000
        || !options.absolute_error_limit.is_finite()
        || options.absolute_error_limit < 0.
    {
        return Err(CylindricalFluxFailure::Policy(options));
    }
    Ok(())
}

fn admit_span(
    index: usize,
    span: CylindricalFluxSpan<'_>,
    options: CylindricalFluxOptions,
) -> Checked<()> {
    let roles = ["first space curve", "last space curve"];
    admit_pair(index, span.boundaries, roles, options)?;
    let caps = span.cap_pcurves.map(|poles| MetricBezier {
        poles,
        weights: None,
    });
    let roles = ["first cap pcurve", "last cap pcurve"];
    admit_pair(index, caps, roles, options)?;
    matching_counts(index, span.boundaries)?;
    common_weights(index, span.boundaries)
}

fn matching_counts(index: usize, curves: [MetricBezier<'_, 3>; 2]) -> Checked<()> {
    let counts = curves.map(|c| c.poles.len());
    if counts[0] != counts[1] {
        return Err(CylindricalFluxFailure::PairCounts {
            span: index,
            counts,
        });
    }
    Ok(())
}

fn admit_pair<const DIM: usize>(
    index: usize,
    curves: [MetricBezier<'_, DIM>; 2],
    roles: [&'static str; 2],
    options: CylindricalFluxOptions,
) -> Checked<()> {
    for (depth, curve) in curves.into_iter().enumerate() {
        admit_curve(index, curve, roles[depth], options)?;
    }
    Ok(())
}

fn admit_curve<const DIM: usize>(
    span: usize,
    curve: MetricBezier<'_, DIM>,
    role: &'static str,
    options: CylindricalFluxOptions,
) -> Checked<()> {
    coefficients::validate_curve(curve, role).map_err(|cause| error::geometry(span, cause))?;
    let degree = curve.poles.len().saturating_sub(1);
    if degree > usize::from(options.max_degree) {
        return Err(error::integration(
            Some(span),
            RationalIntegralFailure::Degree {
                actual: degree,
                limit: options.max_degree,
            },
        ));
    }
    coordinates(span, curve, role)
}

fn coordinates<const DIM: usize>(
    span: usize,
    curve: MetricBezier<'_, DIM>,
    role: &'static str,
) -> Checked<()> {
    for (index, value) in curve.poles.iter().flatten().enumerate() {
        value_check(span, role, index, *value, false)?;
    }
    for (index, value) in curve.weights.into_iter().flatten().enumerate() {
        value_check(span, "weight", index, *value, true)?;
    }
    Ok(())
}

fn value_check(
    span: usize,
    role: &'static str,
    index: usize,
    value: f64,
    positive: bool,
) -> Checked<()> {
    if !value.is_finite() || (positive && value <= 0.) {
        return Err(error::geometry(
            span,
            ResidualError::Value { role, index, value },
        ));
    }
    Ok(())
}

fn common_weights(span: usize, curves: [MetricBezier<'_, 3>; 2]) -> Checked<()> {
    for pole in 0..curves[0].poles.len() {
        let weights = curves.map(|curve| curve.weights.map_or(1., |values| values[pole]));
        if weights[0].to_bits() != weights[1].to_bits() {
            return Err(CylindricalFluxFailure::PairWeights {
                span,
                pole,
                weights,
            });
        }
    }
    Ok(())
}
