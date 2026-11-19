// SPDX-License-Identifier: GPL-2.0-only
use super::*;
pub(super) fn retained_selected<'a>(
    source: &ConicalBoundaryQuery<'_>,
    boundary: &'a ConicalBoundaryEvidence,
    domain: &ConicalChartDomainEvidence,
    bits: u16,
    refinement: ConicalPrimitiveRefinement,
) -> Result<(&'a spatial::SpatialCapture, &'a Interval), ConicalChartTrimPairError> {
    shape(source)?;
    source_binding(source, boundary)?;
    chart_authority(boundary, domain, bits, refinement)?;
    let retained = boundary
        .retained_in_chart_with_refinement(source, domain, bits, refinement)
        .ok_or_else(|| {
            error::binding(
                None,
                0,
                "executed retained spatial capture",
                vec![0],
                vec![1],
            )
        })?;
    let fit_phase =
        retained.capture.fit_phase.as_ref().ok_or_else(|| {
            error::binding(None, 0, "executed ideal fit anchor", vec![0], vec![1])
        })?;
    Ok((&retained.capture, fit_phase))
}
fn chart_authority(
    boundary: &ConicalBoundaryEvidence,
    domain: &ConicalChartDomainEvidence,
    bits: u16,
    refinement: ConicalPrimitiveRefinement,
) -> Result<(), ConicalChartTrimPairError> {
    grid(boundary, domain, bits)?;
    if boundary
        .chart
        .as_ref()
        .is_some_and(|old| old.matches_selected(domain, refinement))
    {
        return Ok(());
    }
    chart_mismatch(boundary, domain, refinement)
}
fn source_binding(
    source: &ConicalBoundaryQuery<'_>,
    boundary: &ConicalBoundaryEvidence,
) -> Result<(), ConicalChartTrimPairError> {
    if boundary.matches(source) {
        return Ok(());
    }
    let actual = boundary_snapshot::BoundarySnapshot::capture(source).bits;
    Err(error::binding(
        None,
        0,
        "executed boundary query",
        actual,
        boundary.input.bits.clone(),
    ))
}
fn grid(
    boundary: &ConicalBoundaryEvidence,
    domain: &ConicalChartDomainEvidence,
    bits: u16,
) -> Result<(), ConicalChartTrimPairError> {
    let expected = boundary.options().fractional_bits;
    if expected != bits || domain.options().fractional_bits != bits {
        return Err(error::binding(
            None,
            0,
            "fixed-grid authority",
            vec![u64::from(bits), u64::from(domain.options().fractional_bits)],
            vec![u64::from(expected); 2],
        ));
    }
    Ok(())
}
fn shape(source: &ConicalBoundaryQuery<'_>) -> Result<(), ConicalChartTrimPairError> {
    if let Err(ConicalBoundaryFailure::Shape {
        role,
        actual,
        expected,
    }) = boundary::admit_shape(source)
    {
        let expected = match expected {
            "1..=4" => vec![1, 4],
            "2..=4" => vec![2, 4],
            "1..=10" => vec![1, 10],
            "2..=16" => vec![2, 16],
            _ => Vec::new(),
        };
        return Err(error::binding(None, 0, role, vec![actual as u64], expected));
    }
    Ok(())
}
pub(super) fn edge(
    query: ConicalTrimQuery<'_>,
    depth: usize,
    source: &ConicalBoundaryQuery<'_>,
    operations: u32,
) -> Result<(), ConicalChartTrimPairError> {
    let edge = &source.edges[depth];
    if query.carrier.semi_angle.to_bits() != source.semi_angle.to_bits() {
        return Err(error::binding(
            Some(depth),
            operations,
            "actual carrier semi-angle",
            vec![query.carrier.semi_angle.to_bits()],
            vec![source.semi_angle.to_bits()],
        ));
    }
    if !same_poles(query.curve.poles, edge.stored_poles) {
        return Err(error::binding(
            Some(depth),
            operations,
            "decoded space poles",
            pole_bits(query.curve.poles),
            pole_bits(edge.stored_poles),
        ));
    }
    weights(query.curve.weights, edge.stored_weights, depth, operations)
}
fn same_poles(a: &[[f64; 3]], b: &[[f64; 3]]) -> bool {
    a.len() == b.len()
        && a.iter()
            .flatten()
            .zip(b.iter().flatten())
            .all(|(a, b)| a.to_bits() == b.to_bits())
}
fn pole_bits(poles: &[[f64; 3]]) -> Vec<u64> {
    let mut bits = vec![poles.len() as u64];
    bits.extend(poles.iter().flatten().map(|value| value.to_bits()));
    bits
}
fn weights(
    actual: Option<&[f64]>,
    expected: &[f64],
    depth: usize,
    operations: u32,
) -> Result<(), ConicalChartTrimPairError> {
    if actual.is_some_and(|weights| {
        weights.len() == expected.len()
            && weights
                .iter()
                .zip(expected)
                .all(|(a, b)| a.to_bits() == b.to_bits())
    }) {
        return Ok(());
    }
    Err(error::binding(
        Some(depth),
        operations,
        "decoded retained weights",
        weight_bits(actual),
        weight_bits(Some(expected)),
    ))
}
fn weight_bits(weights: Option<&[f64]>) -> Vec<u64> {
    let mut bits = vec![
        u64::from(weights.is_some()),
        weights.map_or(0, |w| w.len()) as u64,
    ];
    bits.extend(weights.into_iter().flatten().map(|value| value.to_bits()));
    bits
}

fn chart_mismatch(
    boundary: &ConicalBoundaryEvidence,
    domain: &ConicalChartDomainEvidence,
    refinement: ConicalPrimitiveRefinement,
) -> Result<(), ConicalChartTrimPairError> {
    let actual =
        boundary_chart::ChartBinding::capture_selected(domain, refinement).diagnostic_bits();
    let expected = boundary
        .chart
        .as_ref()
        .map_or_else(|| vec![0], |old| old.diagnostic_bits());
    Err(error::binding(
        None,
        0,
        "executed chart authority",
        actual,
        expected,
    ))
}
