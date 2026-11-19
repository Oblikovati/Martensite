// SPDX-License-Identifier: GPL-2.0-only
use super::*;
pub(super) fn compare(
    field: &'static str,
    actual: Vec<u64>,
    expected: Vec<u64>,
) -> Result<(), ConicalDevelopedMaterialFailure> {
    if actual == expected {
        return Ok(());
    }
    Err(ConicalDevelopedMaterialFailure::Binding {
        field,
        actual,
        expected,
    })
}
pub(super) fn boundary_authority(
    context: &ConicalDevelopedBoundarySpan<'_>,
    domain: &ConicalChartDomainEvidence,
    refinement: ConicalPrimitiveRefinement,
) -> Result<(), ConicalDevelopedMaterialFailure> {
    let actual = context
        .boundary
        .chart
        .as_ref()
        .map(|authority| authority.diagnostic_bits())
        .unwrap_or_default();
    compare(
        "issued boundary chart authority",
        actual,
        context.evidence.domain.clone(),
    )?;
    let actual_turn = context
        .boundary
        .source_anchor_turn_adjustment()
        .map(|turn| turn as i64 as u64)
        .into_iter()
        .collect();
    compare("source anchor same developed branch", actual_turn, vec![0])?;
    boundary_policy(context, domain, refinement)
}
fn boundary_policy(
    context: &ConicalDevelopedBoundarySpan<'_>,
    domain: &ConicalChartDomainEvidence,
    refinement: ConicalPrimitiveRefinement,
) -> Result<(), ConicalDevelopedMaterialFailure> {
    let options = context.boundary.options();
    let expected = context.evidence.options;
    compare(
        "executed boundary arithmetic policy",
        vec![
            u64::from(options.fractional_bits),
            u64::from(options.max_operations),
        ],
        vec![
            u64::from(expected.fractional_bits),
            u64::from(expected.max_operations),
        ],
    )?;
    retained_capture(context, domain, refinement)
}
fn retained_capture(
    context: &ConicalDevelopedBoundarySpan<'_>,
    domain: &ConicalChartDomainEvidence,
    refinement: ConicalPrimitiveRefinement,
) -> Result<(), ConicalDevelopedMaterialFailure> {
    if context
        .boundary
        .retained_in_chart_with_refinement(
            &context.source,
            domain,
            context.boundary.options().fractional_bits,
            refinement,
        )
        .is_none()
    {
        return compare("one actual retained chart capture", vec![0], vec![1]);
    }
    Ok(())
}
pub(super) fn material_operands(
    query: &ConicalMaterialQuery<'_>,
    span: &ConicalMaterialSpan<'_>,
    context: &ConicalDevelopedBoundarySpan<'_>,
) -> Result<(), ConicalDevelopedMaterialFailure> {
    common_uv(context)?;
    original_operands(span, context)?;
    compare(
        "material support, depth, reference and precision",
        support(query),
        expected_support(context),
    )
}
fn original_operands(
    span: &ConicalMaterialSpan<'_>,
    context: &ConicalDevelopedBoundarySpan<'_>,
) -> Result<(), ConicalDevelopedMaterialFailure> {
    let source = &context.source;
    let original = source.original.map_or(source.source, |span| span.poles);
    let (index, level) = source
        .original
        .map_or((0, 0), |span| (span.index, span.level));
    let mut actual = span_bits(span.original, span.interval.index(), span.interval.level());
    actual.extend(span.uv.iter().flatten().map(|v| v.to_bits()));
    let mut expected = span_bits(original, index, level);
    expected.extend(context.evidence.uv_bits[0].iter().flatten());
    compare(
        "actual original dyadic source and decoded UV",
        actual,
        expected,
    )?;
    Ok(())
}
fn common_uv(
    context: &ConicalDevelopedBoundarySpan<'_>,
) -> Result<(), ConicalDevelopedMaterialFailure> {
    compare(
        "common actual UV for both decoded caps",
        context.evidence.uv_bits[1]
            .iter()
            .flatten()
            .copied()
            .collect(),
        context.evidence.uv_bits[0]
            .iter()
            .flatten()
            .copied()
            .collect(),
    )
}

fn span_bits(poles: &[[f64; 2]], index: u64, level: u8) -> Vec<u64> {
    let mut bits = vec![poles.len() as u64, index, u64::from(level)];
    bits.extend(poles.iter().flatten().map(|v| v.to_bits()));
    bits
}
fn support(query: &ConicalMaterialQuery<'_>) -> Vec<u64> {
    let mut bits = vec![
        query.reference_slant.to_bits(),
        query.chart_origin.to_bits(),
        u64::from(query.options.fractional_bits),
    ];
    bits.extend(query.depths.map(f64::to_bits));
    for carrier in query.carriers {
        bits.extend([
            carrier.radius.to_bits(),
            carrier.axial_origin.to_bits(),
            carrier.semi_angle.to_bits(),
        ]);
    }
    bits
}
fn expected_support(context: &ConicalDevelopedBoundarySpan<'_>) -> Vec<u64> {
    let mut bits = vec![
        context.evidence.reference,
        context.source.chart_origin.to_bits(),
        u64::from(context.evidence.options.fractional_bits),
    ];
    bits.extend(context.source.depths.map(f64::to_bits));
    bits.extend(context.evidence.carriers.into_iter().flatten());
    bits
}
