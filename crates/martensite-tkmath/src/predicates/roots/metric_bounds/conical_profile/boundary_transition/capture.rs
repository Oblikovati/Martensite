// SPDX-License-Identifier: GPL-2.0-only
//! Identical positive weights turn actual row displacement into a convex combination.
use super::*;
type CapturedRow = (Vec<Interval>, Interval);
pub(super) fn execute(
    input: &ConicalBoundaryZRowTransition<'_>,
    snapshot: BoundarySnapshot,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<ConicalBoundaryZRowEvidence, ConicalBoundaryZRowFailure> {
    let capture = rows(input, snapshot, arithmetic)?;
    let result = compose(input, &capture, arithmetic)?;
    complete(input, capture, result, arithmetic)
}
fn complete(
    input: &ConicalBoundaryZRowTransition<'_>,
    capture: DirectedRowCapture,
    result: combined::CombinedEvidence,
    arithmetic: &Arithmetic<'_>,
) -> Result<ConicalBoundaryZRowEvidence, ConicalBoundaryZRowFailure> {
    let incremental = arithmetic.operations();
    let cumulative = u64::from(input.parent.operations()) + u64::from(incremental);
    if cumulative > u64::from(input.parent.options().max_operations) {
        return Err(ConicalBoundaryZRowFailure::Ancestry {
            actual: cumulative,
            limit: input.parent.options().max_operations,
        });
    }
    let boundary = issued_boundary(input, result, cumulative as u32);
    let remaining = ConeClearanceOptions {
        fractional_bits: arithmetic.fractional_bits(),
        max_operations: arithmetic.policy().max_operations,
    };
    Ok(ConicalBoundaryZRowEvidence {
        boundary: Arc::new(boundary),
        capture,
        incremental,
        cumulative,
        remaining,
    })
}
fn issued_boundary(
    input: &ConicalBoundaryZRowTransition<'_>,
    result: combined::CombinedEvidence,
    operations: u32,
) -> ConicalBoundaryEvidence {
    let mut boundary = boundary::evidence(
        result,
        input.parent.options(),
        BoundarySnapshot::capture(&input.current),
        input.parent.authored_level,
    );
    boundary.operations = operations;
    boundary.chart = Some(boundary_chart::ChartBinding::capture_selected(
        input.domain,
        input.refinement,
    ));
    boundary
}
fn rows(
    input: &ConicalBoundaryZRowTransition<'_>,
    snapshot: BoundarySnapshot,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<DirectedRowCapture, ConicalBoundaryZRowFailure> {
    let authority = binding::checked_span(input, arithmetic.fractional_bits())?;
    let first = row(input, 0, arithmetic)?;
    let last = row(input, 1, arithmetic)?;
    let storage = [
        arithmetic.add(&authority.parent_storage[0], &magnitude(&first.1))?,
        arithmetic.add(&authority.parent_storage[1], &magnitude(&last.1))?,
    ];
    captured(
        input,
        snapshot,
        [first, last],
        authority.parent_storage.clone(),
        storage,
    )
}
fn captured(
    input: &ConicalBoundaryZRowTransition<'_>,
    snapshot: BoundarySnapshot,
    rows: [CapturedRow; 2],
    parent_storage: [Interval; 2],
    storage: [Interval; 2],
) -> Result<DirectedRowCapture, ConicalBoundaryZRowFailure> {
    let retained = input
        .parent
        .retained
        .as_ref()
        .ok_or_else(|| error::mismatch("retained spatial", vec![0], vec![1]))?;
    let [first, last] = rows;
    Ok(DirectedRowCapture {
        span: input.span,
        new_z: [first.0, last.0],
        signed_shifts: [first.1, last.1],
        storage,
        parent_storage,
        retained: Arc::clone(retained),
        current: snapshot,
        chart: boundary_chart::ChartBinding::capture_selected(input.domain, input.refinement),
        gamma: input.gamma.clone(),
    })
}
fn row(
    input: &ConicalBoundaryZRowTransition<'_>,
    depth: usize,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<CapturedRow> {
    let mut new_z = Vec::new();
    let mut shifts = Vec::new();
    let pairs = input.previous.edges[depth]
        .stored_poles
        .iter()
        .zip(input.current.edges[depth].stored_poles);
    for (old, new) in pairs {
        let old = arithmetic.scalar(old[2])?;
        let new = arithmetic.scalar(new[2])?;
        shifts.push(signed_delta(depth, &old, &new, arithmetic)?);
        new_z.push(new);
    }
    Ok((new_z, hull(&shifts)))
}
fn signed_delta(
    depth: usize,
    old: &Interval,
    new: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let delta = arithmetic.subtract(new, old)?;
    if depth == 0 {
        return Ok(Interval {
            lower: -delta.upper,
            upper: -delta.lower,
        });
    }
    Ok(delta)
}
fn hull(values: &[Interval]) -> Interval {
    Interval {
        lower: values
            .iter()
            .map(|v| v.lower.clone())
            .min()
            .unwrap_or_default(),
        upper: values
            .iter()
            .map(|v| v.upper.clone())
            .max()
            .unwrap_or_default(),
    }
}
fn magnitude(value: &Interval) -> Interval {
    Interval {
        lower: 0.into(),
        upper: value
            .lower
            .clone()
            .max(-value.lower.clone())
            .max(value.upper.clone())
            .max(-value.upper.clone()),
    }
}
fn composed_total(
    capture: &DirectedRowCapture,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let maximum = Interval {
        lower: 0.into(),
        upper: capture.storage[0]
            .upper
            .clone()
            .max(capture.storage[1].upper.clone()),
    };
    let spatial = &capture.retained.evidence;
    let ideal = arithmetic.add(&spatial.bound, &spatial.original)?;
    arithmetic.add(&ideal, &maximum)
}
fn compose(
    input: &ConicalBoundaryZRowTransition<'_>,
    capture: &DirectedRowCapture,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<combined::CombinedEvidence, ConicalBoundaryZRowFailure> {
    let spatial = capture.retained.evidence.clone();
    let total = composed_total(capture, arithmetic)?;
    let derivative = super::super::trim_bridge::bounds::captured_storage(
        input.parent.total_angular_derivative(),
        arithmetic,
    )?;
    combined::accept(
        Some(Arc::clone(&capture.retained)),
        total,
        spatial,
        derivative,
        capture.storage.clone(),
        input.current.spatial_limit,
        arithmetic,
    )
    .map_err(|cause| composition_error(input, cause, arithmetic.operations()))
}
fn composition_error(
    input: &ConicalBoundaryZRowTransition<'_>,
    cause: combined::CombinedFailure,
    operations: u32,
) -> ConicalBoundaryZRowFailure {
    ConicalBoundaryZRowFailure::Boundary(boundary::proof_error(
        cause,
        operations,
        input.parent.options(),
        BoundarySnapshot::capture(&input.current),
    ))
}
