// SPDX-License-Identifier: GPL-2.0-only
//! Deterministic stronger extrema are captured at one precision and one real grant.
use super::*;
pub(super) fn issue(
    query: &ConicalDirectedGammaQuery<'_>,
    caps: CapIndex,
    arithmetic: &mut Arithmetic<'_>,
    position: &mut error::FailurePosition,
) -> Outcome<ConicalDirectedGammaEvidence> {
    let (records, gaps) = records(query, &caps, arithmetic, position)?;
    position.clear();
    let gamma = select(&records, &gaps, arithmetic)?;
    arithmetic.check_cancelled()?;
    Ok(publish(
        query,
        records,
        caps,
        gamma,
        arithmetic.operations(),
    ))
}
fn capture(
    span: &ConicalDirectedGammaSpan<'_>,
    query: &ConicalDirectedGammaQuery<'_>,
    caps: &CapIndex,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<GammaSpanCapture> {
    let storage = [
        numeric::import(&span.parent.storage()[0], arithmetic)?,
        numeric::import(&span.parent.storage()[1], arithmetic)?,
    ];
    let depths = [
        arithmetic.scalar(span.previous.depths[0])?,
        arithmetic.scalar(span.previous.depths[1])?,
    ];
    let sine = span.domain.sine.clone();
    let cosine = span.domain.cosine.clone();
    numeric::positive("family sine", &sine, arithmetic)?;
    numeric::positive("family cosine", &cosine, arithmetic)?;
    let levels = levels(span, query, caps, &sine, &cosine, arithmetic)?;
    Ok(record(
        span,
        query.refinement,
        (storage, depths, levels),
        (sine, cosine),
    ))
}
fn cap_level(
    id: ConicalActualCapId,
    query: &ConicalDirectedGammaQuery<'_>,
    caps: &CapIndex,
    sine: &Interval,
    cosine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<Interval> {
    let cap = caps
        .get(&id)
        .and_then(|&index| query.caps.get(index))
        .ok_or_else(|| error::shape("carrier", id.0 as usize, "declared actual cap"))?;
    let radius = arithmetic.scalar(cap.radius)?;
    let origin = arithmetic.scalar(cap.axial_origin)?;
    let radial = arithmetic.multiply(cosine, &radius)?;
    let axial = arithmetic.multiply(sine, &origin)?;
    Ok(arithmetic.subtract(&radial, &axial)?)
}
fn select(
    records: &[GammaSpanCapture],
    gaps: &[Vec<Interval>; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<[f64; 2]> {
    let sine = numeric::lower_point(&records[0].sine);
    let mut result = [0.; 2];
    for row in 0..2 {
        let storage = numeric::max_magnitude(
            &records
                .iter()
                .map(|r| r.parent_storage[row].clone())
                .collect::<Vec<_>>(),
        );
        let gap = numeric::max_magnitude(&gaps[row]);
        let sum = arithmetic.add(&storage, &gap)?;
        let ratio = arithmetic.divide(&sum, &sine)?;
        result[row] = gamma_amount(ratio, arithmetic)?;
    }
    Ok(result)
}
pub(super) fn transform(gamma: [f64; 2], row: usize, old: f64) -> Outcome<f64> {
    transform_input(row, old)?;
    let value = if row == 0 {
        (old - gamma[0]).next_down()
    } else {
        (old + gamma[1]).next_up()
    };
    if !value.is_finite() {
        return Err(ConicalActualPatchFailure::Value {
            field: "directed Z",
            actual: value,
            expected: "finite representable coordinate",
        });
    }
    Ok(value)
}

fn records(
    query: &ConicalDirectedGammaQuery<'_>,
    caps: &CapIndex,
    arithmetic: &mut Arithmetic<'_>,
    position: &mut error::FailurePosition,
) -> Outcome<(Vec<GammaSpanCapture>, [Vec<Interval>; 2])> {
    let mut records = Vec::with_capacity(query.spans.len());
    let mut gaps: [Vec<Interval>; 2] = std::array::from_fn(|_| Vec::new());
    for span in query.spans {
        position.side(span.id);
        let record = capture(span, query, caps, arithmetic)?;
        for (row, values) in gaps.iter_mut().enumerate() {
            values.push(numeric::magnitude(
                &arithmetic.subtract(&record.cap_levels[row], &record.depths[row])?,
            ));
        }
        records.push(record);
    }
    Ok((records, gaps))
}
fn publish(
    query: &ConicalDirectedGammaQuery<'_>,
    records: Vec<GammaSpanCapture>,
    cap_index: CapIndex,
    gamma: [f64; 2],
    operations: u32,
) -> ConicalDirectedGammaEvidence {
    let index = records
        .iter()
        .enumerate()
        .map(|(i, v)| (v.id, i))
        .collect::<BTreeMap<_, _>>();
    ConicalDirectedGammaEvidence {
        gamma,
        options: query.options,
        refinement: query.refinement,
        operations,
        records: records.into(),
        index: std::sync::Arc::new(index),
        caps: query.caps.to_vec().into(),
        cap_index: std::sync::Arc::new(cap_index),
    }
}

fn levels(
    span: &ConicalDirectedGammaSpan<'_>,
    query: &ConicalDirectedGammaQuery<'_>,
    caps: &CapIndex,
    sine: &Interval,
    cosine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<[Interval; 2]> {
    Ok([
        cap_level(span.caps[0], query, caps, sine, cosine, arithmetic)?,
        cap_level(span.caps[1], query, caps, sine, cosine, arithmetic)?,
    ])
}
fn record(
    span: &ConicalDirectedGammaSpan<'_>,
    refinement: ConicalPrimitiveRefinement,
    measurements: ([Interval; 2], [Interval; 2], [Interval; 2]),
    trig: (Interval, Interval),
) -> GammaSpanCapture {
    GammaSpanCapture {
        parent_storage: measurements.0,
        depths: measurements.1,
        cap_levels: measurements.2,
        sine: trig.0,
        cosine: trig.1,
        id: span.id,
        member: span.member,
        cap_ids: span.caps,
        input: BoundarySnapshot::capture(&span.previous),
        chart: ChartBinding::capture_selected(span.domain, refinement),
        parent_options: span.parent.options(),
        parent_operations: span.parent.operations(),
        parent_bounds: span.parent.storage().clone(),
    }
}

fn gamma_amount(ratio: Interval, arithmetic: &Arithmetic<'_>) -> Outcome<f64> {
    let value = arithmetic.upper_f64(ratio.upper)?.next_up();
    if !value.is_finite() {
        return Err(ConicalActualPatchFailure::Value {
            field: "directed gamma",
            actual: value,
            expected: "finite strictly positive",
        });
    }
    Ok(value)
}

fn transform_input(row: usize, old: f64) -> Outcome<()> {
    if row > 1 {
        return Err(error::shape("physical row", row, "zero or one"));
    }
    if !old.is_finite() {
        return Err(ConicalActualPatchFailure::Value {
            field: "old Z",
            actual: old,
            expected: "finite",
        });
    }
    Ok(())
}
