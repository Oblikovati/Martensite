// SPDX-License-Identifier: GPL-2.0-only
use super::*;
mod chart_binding;
pub(super) fn admit(query: &ReferenceInputs<'_>) -> Result<(), ConicalReferenceError> {
    if query.members.is_empty() || !query.spatial_limit.is_finite() || query.spatial_limit <= 0. {
        return Err(error(
            binding_failure("nonempty family and finite positive epsilon"),
            0,
        ));
    }
    if !(8..=4096).contains(&query.options.fractional_bits)
        || query.options.max_operations > 1_000_000
    {
        return Err(error(
            binding_failure("supported remaining arithmetic policy"),
            0,
        ));
    }
    for (index, member) in query.members.iter().enumerate() {
        admit_member(member, index, query)?;
    }
    topology::bind(query).map_err(|cause| error(cause, 0))
}
fn admit_member(
    member: &ConicalReferenceMember<'_>,
    index: usize,
    query: &ReferenceInputs<'_>,
) -> Result<(), ConicalReferenceError> {
    let joins = ConicalJoinsQuery {
        spans: member.spans,
        contour_lengths: member.contour_lengths,
        options: member.joins.options,
    };
    if !chart_binding::joins_match_selected(member, &joins, query.sector, query.refinement) {
        return Err(error(
            binding_failure("executed source-bound common monotone ideal joins"),
            0,
        ));
    }
    for (span, input) in member.spans.iter().enumerate() {
        admit_span(input, query).map_err(|cause| ConicalReferenceError {
            offender: Some([index, span]),
            cause,
            operations: 0,
        })?;
    }
    Ok(())
}
fn admit_span(
    span: &ConicalJoinSpan<'_>,
    query: &ReferenceInputs<'_>,
) -> Result<(), ConicalReferenceFailure> {
    let input = span.query;
    admit_chart_authority(span, query)?;
    let first = first(query)?;
    if !span.boundary.matches(&input) || !same_chart(input, first) {
        return Err(binding_failure(
            "one canonical outward ideal q/L chart, support and depths",
        ));
    }
    if input.original.is_none()
        || span.boundary.options.fractional_bits != query.options.fractional_bits
        || span.boundary.anchor.turn_adjustment != Some(0)
    {
        return Err(binding_failure(
            "original intervals and source-angle anchors",
        ));
    }
    Ok(())
}
fn admit_chart_authority(
    span: &ConicalJoinSpan<'_>,
    query: &ReferenceInputs<'_>,
) -> Result<(), ConicalReferenceFailure> {
    let input = span.query;
    if let ReferenceDomain::Chart(domain) = query.sector
        && !span
            .boundary
            .matches_chart_with_refinement(&input, domain, query.refinement)
    {
        return Err(binding_failure("executed chart-metric boundary authority"));
    }
    Ok(())
}
fn first<'a>(
    query: &ReferenceInputs<'a>,
) -> Result<ConicalBoundaryQuery<'a>, ConicalReferenceFailure> {
    query
        .members
        .first()
        .and_then(|member| member.spans.first())
        .map(|span| span.query)
        .ok_or_else(|| binding_failure("nonempty first member"))
}

fn same_chart(input: ConicalBoundaryQuery<'_>, first: ConicalBoundaryQuery<'_>) -> bool {
    input.semi_angle == first.semi_angle
        && input.chart_origin == first.chart_origin
        && input.depths == first.depths
        && input.outward
        && input.fit_center == input.chart_origin
        && input.physical_turn == 0
        && input.developed_turn == 0
}

pub(super) fn evidence(
    query: &ReferenceInputs<'_>,
    bounds: [f64; 3],
    operations: u32,
) -> ConicalReferenceEvidence {
    ConicalReferenceEvidence {
        input: snapshots(query),
        lengths: query
            .members
            .iter()
            .map(|member| member.contour_lengths.to_vec())
            .collect(),
        components: components(query),
        spatial_limit: query.spatial_limit.to_bits(),
        options: query.options,
        developed: bounds[0],
        filled: bounds[1],
        storage: bounds[2],
        operations,
    }
}
fn snapshots(query: &ReferenceInputs<'_>) -> Vec<Vec<BoundarySnapshot>> {
    query
        .members
        .iter()
        .map(|member| {
            member
                .spans
                .iter()
                .map(|span| BoundarySnapshot::capture(&span.query))
                .collect()
        })
        .collect()
}

pub(super) fn matches(proof: &ConicalReferenceEvidence, query: &ReferenceInputs<'_>) -> bool {
    proof.spatial_limit == query.spatial_limit.to_bits()
        && proof.options == query.options
        && proof.components == components(query)
        && proof.input.len() == query.members.len()
        && admit(query).is_ok()
        && proof
            .input
            .iter()
            .zip(&proof.lengths)
            .zip(query.members)
            .all(|((spans, lengths), member)| {
                lengths == member.contour_lengths
                    && spans.len() == member.spans.len()
                    && spans
                        .iter()
                        .zip(member.spans)
                        .all(|(old, span)| old.matches(&span.query))
            })
}

fn components(query: &ReferenceInputs<'_>) -> Vec<u64> {
    let mut bits = policy_bits(query);
    if query.refinement != ConicalPrimitiveRefinement::UnsplitV1 {
        bits.extend([3, query.refinement.tag()]);
    }
    if let ReferenceDomain::Chart(proof) = query.sector {
        chart_bits(proof, &mut bits);
    }
    for member in query.members {
        join_bits(member.joins, &mut bits);
        boundary_bits(member.spans, &mut bits);
    }
    bits
}
fn policy_bits(query: &ReferenceInputs<'_>) -> Vec<u64> {
    let policy = query.clearance.requested_options();
    let sector = query.sector.policy();
    vec![
        query.clearance.clearance_lower_bound().to_bits(),
        u64::from(policy.fractional_bits),
        u64::from(policy.max_domains),
        u64::from(policy.max_curve_nodes),
        u64::from(policy.max_depth),
        u64::from(policy.max_operations),
        u64::from(sector.fractional_bits),
        u64::from(sector.max_operations),
        u64::from(query.clearance.operations()),
        u64::from(query.sector.operations()),
    ]
}
fn join_bits(joins: &ConicalJoinsEvidence, bits: &mut Vec<u64>) {
    bits.extend([
        u64::from(joins.options.fractional_bits),
        u64::from(joins.options.max_operations),
        joins.directions.len() as u64,
        u64::from(joins.operations()),
    ]);
    bits.extend(
        joins
            .directions
            .iter()
            .flatten()
            .map(|value| value.to_bits()),
    );
}

fn boundary_bits(spans: &[ConicalJoinSpan<'_>], bits: &mut Vec<u64>) {
    for span in spans {
        let policy = span.boundary.options();
        bits.extend([
            u64::from(policy.fractional_bits),
            u64::from(policy.max_operations),
            u64::from(span.boundary.operations()),
        ]);
    }
}

// The executed component owns its source order even when coverage is the same set.
pub(super) fn chart_bits(proof: &ConicalChartDomainEvidence, bits: &mut Vec<u64>) {
    bits.extend([
        2,
        proof.chart_origin().to_bits(),
        proof.fit_center().to_bits(),
        proof.developed_turn() as u64,
        proof.physical_turn() as u64,
        proof.sector.source.len() as u64,
    ]);
    for span in &proof.sector.source {
        bits.push(span.len() as u64);
        bits.extend(span.iter().flatten().copied());
    }
}
