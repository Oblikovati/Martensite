// SPDX-License-Identifier: GPL-2.0-only
use super::super::admission as ruled;
use super::*;
use crate::predicates::roots::metric_bounds::CylindricalProfileBehavior;

pub(super) fn admit(query: AnalyticConicalFluxQuery<'_>) -> Checked<()> {
    parent_policy(query.options)?;
    arithmetic_policy(query.arithmetic)?;
    span_count(query.spans.len(), query.options.max_spans)?;
    for (depth, carrier) in query.carriers.into_iter().enumerate() {
        admit_carrier(depth, carrier)?;
    }
    for (index, span) in query.spans.iter().copied().enumerate() {
        admit_span(index, span, query.options)?;
    }
    Ok(())
}
fn arithmetic_policy(options: CylindricalProfileOptions) -> Checked<()> {
    if options.behavior != CylindricalProfileBehavior::V1
        || !(8..=4096).contains(&options.fractional_bits)
        || options.max_operations > 1_000_000
    {
        return Err(AnalyticConicalFluxFailure::ArithmeticPolicy(options));
    }
    Ok(())
}
fn span_count(actual: usize, maximum: u32) -> Checked<()> {
    if actual == 0 || actual > maximum as usize {
        return Err(error::input(
            None,
            None,
            CylindricalFluxFailure::SpanCount { actual, maximum },
        ));
    }
    Ok(())
}
fn admit_carrier(depth: usize, actual: ConicalTrimCarrier) -> Checked<()> {
    if !actual.radius.is_finite()
        || actual.radius <= 0.
        || !actual.axial_origin.is_finite()
        || !actual.semi_angle.is_finite()
        || actual.semi_angle <= 0.
        || actual.semi_angle >= std::f64::consts::FRAC_PI_2
    {
        return Err(AnalyticConicalFluxFailure::Carrier { depth, actual });
    }
    Ok(())
}
fn admit_span(
    index: usize,
    span: AnalyticConicalFluxSpan<'_>,
    options: CylindricalFluxOptions,
) -> Checked<()> {
    let roles = ["first space curve", "last space curve"];
    admit_pair(index, span.boundaries, roles, options)?;
    let caps = span.cap_pcurves.map(|poles| MetricBezier {
        poles,
        weights: None,
    });
    admit_pair(
        index,
        caps,
        ["first cap pcurve", "last cap pcurve"],
        options,
    )?;
    ruled::matching_counts(index, span.boundaries)
        .map_err(|cause| error::input(Some(index), None, cause))?;
    ruled::common_weights(index, span.boundaries)
        .map_err(|cause| error::input(Some(index), None, cause))
}
fn admit_pair<const DIM: usize>(
    index: usize,
    curves: [MetricBezier<'_, DIM>; 2],
    roles: [&'static str; 2],
    options: CylindricalFluxOptions,
) -> Checked<()> {
    for (depth, curve) in curves.into_iter().enumerate() {
        ruled::admit_curve(index, curve, roles[depth], options)
            .map_err(|cause| error::input(Some(index), Some(depth), cause))?;
    }
    Ok(())
}

fn parent_policy(options: CylindricalFluxOptions) -> Checked<()> {
    let invalid = || error::input(None, None, CylindricalFluxFailure::Policy(options));
    if !(1..=2).contains(&options.behavior_version) {
        return Err(invalid());
    }
    // Child scalar/cylinder policy remains1; an invalid parent still reports its original2.
    ruled::policy(CylindricalFluxOptions {
        behavior_version: 1,
        ..options
    })
    .map_err(|_| invalid())
}
