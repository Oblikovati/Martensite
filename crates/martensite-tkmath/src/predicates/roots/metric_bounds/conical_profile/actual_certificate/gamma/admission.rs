// SPDX-License-Identifier: GPL-2.0-only
//! Binding precedes arithmetic setup and cancellation polls.
use super::*;
pub(in super::super) fn same_carrier(
    a: &ConicalActualCapCarrier,
    b: &ConicalActualCapCarrier,
) -> bool {
    a.id == b.id && a.member == b.member && carrier_bits(a) == carrier_bits(b)
}
pub(in super::super) fn carrier_bits(cap: &ConicalActualCapCarrier) -> Vec<u64> {
    cap.frame
        .iter()
        .flatten()
        .chain([&cap.radius, &cap.axial_origin, &cap.semi_angle])
        .map(|v| v.to_bits())
        .collect()
}
pub(in super::super) fn carrier(cap: &ConicalActualCapCarrier) -> Outcome<()> {
    frame(cap)?;
    for (field, value) in carrier_scalars(cap) {
        if !value.is_finite() {
            return Err(ConicalActualPatchFailure::Value {
                field,
                actual: value,
                expected: "finite",
            });
        }
    }
    if cap.radius <= 0. {
        return Err(ConicalActualPatchFailure::Value {
            field: "radius",
            actual: cap.radius,
            expected: "strictly positive",
        });
    }
    Ok(())
}
fn frame(cap: &ConicalActualCapCarrier) -> Outcome<()> {
    let expected = [
        [0., 0., cap.axial_origin],
        [1., 0., 0.],
        [0., 1., 0.],
        [0., 0., 1.],
    ];
    // Preserve identical payloads for scalar validation while identifying geometric signed zeros.
    let matches = cap
        .frame
        .iter()
        .flatten()
        .zip(expected.iter().flatten())
        .all(|(a, b)| a == b || a.to_bits() == b.to_bits());
    if !matches {
        let actual = cap.frame.iter().flatten().map(|v| v.to_bits()).collect();
        let expected = expected.iter().flatten().map(|v| v.to_bits()).collect();
        return Err(error::binding("cardinal cone frame", actual, expected));
    }
    Ok(())
}
pub(super) fn admit(
    query: &ConicalDirectedGammaQuery<'_>,
    position: &mut error::FailurePosition,
) -> Outcome<CapIndex> {
    numeric::policy(query.options)?;
    if query.spans.is_empty() {
        return Err(error::shape("gamma spans", 0, "nonempty complete family"));
    }
    let caps = cap_inventory(query, position)?;
    position.clear();
    let mut sides = BTreeSet::new();
    for span in query.spans {
        position.side(span.id);
        if !sides.insert(span.id) {
            return Err(error::shape(
                "duplicate side ID",
                span.id.0 as usize,
                "unique",
            ));
        }
        span_binding(span, query, &caps)?;
    }
    position.clear();
    Ok(caps)
}
fn cap_inventory(
    query: &ConicalDirectedGammaQuery<'_>,
    position: &mut error::FailurePosition,
) -> Outcome<CapIndex> {
    let mut indices = CapIndex::new();
    for (index, cap) in query.caps.iter().enumerate() {
        position.cap(cap.id);
        carrier(cap)?;
        if indices.insert(cap.id, index).is_some() {
            return Err(error::shape(
                "duplicate cap ID",
                cap.id.0 as usize,
                "unique",
            ));
        }
    }
    Ok(indices)
}
fn span_binding(
    span: &ConicalDirectedGammaSpan<'_>,
    query: &ConicalDirectedGammaQuery<'_>,
    caps: &CapIndex,
) -> Outcome<()> {
    let q = &span.previous;
    if !span
        .parent
        .matches_chart_with_refinement(q, span.domain, query.refinement)
        || span.parent.options().fractional_bits != query.options.fractional_bits
    {
        return Err(parent_failure(span, query));
    }
    chart(q, span.domain)?;
    for cap_id in span.caps {
        cap_ownership(cap_id, span, query, caps)?;
    }
    let first = &query.spans[0];
    center_mode(span, first)?;
    uniform(q, &first.previous)
}
fn parent_failure(
    span: &ConicalDirectedGammaSpan<'_>,
    query: &ConicalDirectedGammaQuery<'_>,
) -> ConicalActualPatchFailure {
    let mut actual = BoundarySnapshot::capture(&span.previous).bits;
    actual.extend(ChartBinding::capture_selected(span.domain, query.refinement).diagnostic_bits());
    actual.push(u64::from(query.options.fractional_bits));
    let mut expected = span.parent.input.bits.clone();
    if let Some(chart) = &span.parent.chart {
        expected.extend(chart.diagnostic_bits());
    }
    expected.push(u64::from(span.parent.options().fractional_bits));
    error::binding("nearest boundary authority", actual, expected)
}
fn chart(q: &ConicalBoundaryQuery<'_>, domain: &ConicalChartDomainEvidence) -> Outcome<()> {
    let center = if domain.version().allows_offset() {
        domain.fit_center().to_bits()
    } else {
        0
    };
    let actual = chart_bits(q);
    let expected = [0, center, 0, 0, 1];
    if actual != expected {
        return Err(error::binding(
            "qualified common chart",
            actual.to_vec(),
            expected.to_vec(),
        ));
    }
    Ok(())
}
fn chart_bits(q: &ConicalBoundaryQuery<'_>) -> [u64; 5] {
    [
        q.chart_origin.to_bits(),
        q.fit_center.to_bits(),
        q.developed_turn as u64,
        q.physical_turn as u64,
        u64::from(q.outward),
    ]
}
fn cap_ownership(
    id: ConicalActualCapId,
    span: &ConicalDirectedGammaSpan<'_>,
    query: &ConicalDirectedGammaQuery<'_>,
    caps: &CapIndex,
) -> Outcome<()> {
    let cap = caps
        .get(&id)
        .and_then(|&index| query.caps.get(index))
        .ok_or_else(|| error::shape("missing cap", id.0 as usize, "declared carrier"))?;
    if cap.member != span.member || cap.semi_angle.to_bits() != span.previous.semi_angle.to_bits() {
        return Err(error::binding(
            "cap ownership/support",
            vec![cap.member.0, cap.semi_angle.to_bits()],
            vec![span.member.0, span.previous.semi_angle.to_bits()],
        ));
    }
    Ok(())
}
fn center_mode(
    current: &ConicalDirectedGammaSpan<'_>,
    first: &ConicalDirectedGammaSpan<'_>,
) -> Outcome<()> {
    let actual = current.domain.version() == ConicalChartDomainVersion::AtlasMemberV5;
    let expected = first.domain.version() == ConicalChartDomainVersion::AtlasMemberV5;
    if actual != expected {
        return Err(error::binding(
            "one family chart mode",
            vec![u64::from(actual)],
            vec![u64::from(expected)],
        ));
    }
    if actual {
        return Ok(());
    }
    uniform_center(current.previous.fit_center, first.previous.fit_center)
}
fn uniform_center(actual: f64, expected: f64) -> Outcome<()> {
    if actual.to_bits() != expected.to_bits() {
        return Err(error::binding(
            "family fit center",
            vec![actual.to_bits()],
            vec![expected.to_bits()],
        ));
    }
    Ok(())
}
fn uniform(a: &ConicalBoundaryQuery<'_>, b: &ConicalBoundaryQuery<'_>) -> Outcome<()> {
    let actual = [
        a.semi_angle.to_bits(),
        a.depths[0].to_bits(),
        a.depths[1].to_bits(),
    ];
    let expected = [
        b.semi_angle.to_bits(),
        b.depths[0].to_bits(),
        b.depths[1].to_bits(),
    ];
    if actual != expected {
        return Err(error::binding(
            "family angle/physical depths",
            actual.to_vec(),
            expected.to_vec(),
        ));
    }
    Ok(())
}

fn carrier_scalars(cap: &ConicalActualCapCarrier) -> [(&'static str, f64); 3] {
    [
        ("radius", cap.radius),
        ("axial_origin", cap.axial_origin),
        ("semi_angle", cap.semi_angle),
    ]
}
