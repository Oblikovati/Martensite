// SPDX-License-Identifier: GPL-2.0-only
use super::*;

pub(super) fn admit(query: CylindricalMeasureComparisonQuery<'_>) -> Checked<()> {
    super::super::admission::admit_policy(query.options)
        .and_then(|()| super::super::admission::admit_limit(query.limit))
        .map_err(|_| CylindricalMeasureComparisonFailure::Policy)?;
    let count = query.flux_source.spans.len();
    if count == 0
        || count != query.normal_source.sources.len()
        || count > usize::from(query.options.max_sources)
    {
        return Err(CylindricalMeasureComparisonFailure::Count);
    }
    Ok(())
}
pub(super) fn bind(
    query: CylindricalMeasureComparisonQuery<'_>,
    flux: &CertifiedCylindricalFlux,
    normal: &CertifiedCylindricalNormalMeasure,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<()> {
    bind_reports(query, flux, normal)?;
    for (index, (span, source)) in query
        .flux_source
        .spans
        .iter()
        .zip(query.normal_source.sources)
        .enumerate()
    {
        // Each paired source consumes work and observes cancellation before its bit comparison.
        arithmetic.scalar(0.)?;
        bind_span(index, span, source, query.flux_source.radii[0])?;
    }
    Ok(())
}
fn bind_reports(
    query: CylindricalMeasureComparisonQuery<'_>,
    flux: &CertifiedCylindricalFlux,
    normal: &CertifiedCylindricalNormalMeasure,
) -> Checked<()> {
    if !flux.matches_source(query.flux_source) {
        return Err(CylindricalMeasureComparisonFailure::FluxBinding);
    }
    if !normal.matches_sources(query.normal_source) {
        return Err(CylindricalMeasureComparisonFailure::NormalBinding);
    }
    Ok(())
}

fn bind_span(
    index: usize,
    span: &CylindricalFluxSpan<'_>,
    source: &CylindricalProfileSource<'_>,
    stored_radius: f64,
) -> Checked<()> {
    let query = profile_query(span, source, stored_radius);
    if !source.certificate.matches_source(query) {
        return Err(CylindricalMeasureComparisonFailure::SpanBinding { index });
    }
    Ok(())
}

fn profile_query<'a>(
    span: &CylindricalFluxSpan<'a>,
    source: &CylindricalProfileSource<'a>,
    stored_radius: f64,
) -> CylindricalProfileQuery<'a> {
    let p = source.certificate;
    CylindricalProfileQuery {
        authored: source.authored,
        stored: span.boundaries[0],
        interval: source.interval,
        support_radius: source.support_radius,
        stored_radius,
        angular_origin: source.angular_origin,
        spatial_limit: p.requested_spatial_limit(),
        options: p.requested_options(),
    }
}
