// SPDX-License-Identifier: GPL-2.0-only
use super::*;
pub(super) fn admit(
    input: &ConicalBoundaryZRowTransition<'_>,
    remaining: ConeClearanceOptions,
) -> Result<BoundarySnapshot, ConicalBoundaryZRowError> {
    let current = boundary::admit_query(&input.current, remaining).map_err(|cause| {
        error::failed(ConicalBoundaryZRowFailure::Boundary(cause), remaining, 0)
    })?;
    authority(input, remaining)
        .and_then(|()| unchanged(input))
        .and_then(|()| rows(input))
        .map_err(|cause| error::failed(cause, remaining, 0))?;
    Ok(current)
}
fn authority(
    input: &ConicalBoundaryZRowTransition<'_>,
    remaining: ConeClearanceOptions,
) -> Result<(), ConicalBoundaryZRowFailure> {
    checked_span(input, remaining.fractional_bits)?;
    if input.parent.retained.is_none() {
        return Err(error::mismatch(
            "complete retained spatial receipt",
            vec![0],
            vec![1],
        ));
    }
    Ok(())
}
fn authority_bits(
    input: &ConicalBoundaryZRowTransition<'_>,
    remaining: ConeClearanceOptions,
) -> Vec<u64> {
    let mut bits = BoundarySnapshot::capture(&input.previous).bits;
    bits.extend(
        boundary_chart::ChartBinding::capture_selected(input.domain, input.refinement)
            .diagnostic_bits(),
    );
    bits.extend([
        input.span.0,
        u64::from(remaining.fractional_bits),
        u64::from(input.parent.options().max_operations),
        u64::from(input.parent.operations()),
    ]);
    bits
}
fn expected_bits(input: &ConicalBoundaryZRowTransition<'_>) -> Vec<u64> {
    input
        .gamma
        .span_binding_bits(input.span)
        .unwrap_or_else(|| input.gamma.span_ids())
}

fn unchanged(input: &ConicalBoundaryZRowTransition<'_>) -> Result<(), ConicalBoundaryZRowFailure> {
    let mut normalized = input.current;
    normalized.edges[0].stored_poles = input.previous.edges[0].stored_poles;
    normalized.edges[1].stored_poles = input.previous.edges[1].stored_poles;
    if !input.parent.matches(&normalized) {
        return Err(error::mismatch(
            "unchanged source/support/weights/policy",
            BoundarySnapshot::capture(&normalized).bits,
            input.parent.input.bits.clone(),
        ));
    }
    for depth in 0..2 {
        row_xy(input, depth)?;
    }
    Ok(())
}
fn row_xy(
    input: &ConicalBoundaryZRowTransition<'_>,
    depth: usize,
) -> Result<(), ConicalBoundaryZRowFailure> {
    let actual = input.current.edges[depth].stored_poles;
    let expected = input.previous.edges[depth].stored_poles;
    let same = actual.len() == expected.len()
        && actual
            .iter()
            .zip(expected)
            .all(|(a, b)| a[0].to_bits() == b[0].to_bits() && a[1].to_bits() == b[1].to_bits());
    if !same {
        return Err(error::mismatch(
            "unchanged actual row XY",
            xy_bits(actual),
            xy_bits(expected),
        ));
    }
    Ok(())
}
fn xy_bits(poles: &[[f64; 3]]) -> Vec<u64> {
    let mut bits = vec![poles.len() as u64];
    bits.extend(poles.iter().flat_map(|p| [p[0].to_bits(), p[1].to_bits()]));
    bits
}
fn rows(input: &ConicalBoundaryZRowTransition<'_>) -> Result<(), ConicalBoundaryZRowFailure> {
    let weights = input.current.edges[0].stored_weights;
    if weights.len() != input.current.edges[0].stored_poles.len()
        || weights.iter().any(|v| !v.is_finite() || *v <= 0.)
    {
        return Err(error::mismatch(
            "finite strictly positive common stored weights",
            weights.iter().map(|v| v.to_bits()).collect(),
            vec![0f64.to_bits(), f64::INFINITY.to_bits()],
        ));
    }
    for depth in 0..2 {
        directed_row(input, depth)?;
    }
    Ok(())
}
fn directed_row(
    input: &ConicalBoundaryZRowTransition<'_>,
    depth: usize,
) -> Result<(), ConicalBoundaryZRowFailure> {
    for (index, (old, new)) in input.previous.edges[depth]
        .stored_poles
        .iter()
        .zip(input.current.edges[depth].stored_poles)
        .enumerate()
    {
        let expected = input
            .gamma
            .transform_z(depth, old[2])
            .map_err(ConicalBoundaryZRowFailure::Gamma)?;
        if new[2].to_bits() != expected.to_bits() {
            return Err(ConicalBoundaryZRowFailure::Row {
                depth,
                index,
                actual: new[2],
                expected,
            });
        }
    }
    Ok(())
}

pub(super) fn checked_span<'a>(
    input: &'a ConicalBoundaryZRowTransition<'_>,
    bits: u16,
) -> Result<&'a super::super::actual_certificate::GammaSpanCapture, ConicalBoundaryZRowFailure> {
    if input.gamma.span_capture(input.span).is_none() {
        return Err(error::mismatch(
            "issued source side",
            vec![input.span.0],
            input.gamma.span_ids(),
        ));
    }
    lookup(input, bits).ok_or_else(|| authority_mismatch(input, bits))
}

fn lookup<'a>(
    input: &'a ConicalBoundaryZRowTransition<'_>,
    bits: u16,
) -> Option<&'a super::super::actual_certificate::GammaSpanCapture> {
    input.gamma.checked_span(
        input.span,
        &input.previous,
        input.parent,
        input.domain,
        input.refinement,
        bits,
    )
}
fn authority_mismatch(
    input: &ConicalBoundaryZRowTransition<'_>,
    bits: u16,
) -> ConicalBoundaryZRowFailure {
    let policy = ConeClearanceOptions {
        fractional_bits: bits,
        max_operations: input.parent.options().max_operations,
    };
    error::mismatch(
        "issued span/source/domain/refinement/grant",
        authority_bits(input, policy),
        expected_bits(input),
    )
}
