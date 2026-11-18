// SPDX-License-Identifier: GPL-2.0-only
use super::*;
pub(super) fn admit(query: &ConicalReferenceQuery<'_>) -> Result<(), ConicalReferenceError> {
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
    query: &ConicalReferenceQuery<'_>,
) -> Result<(), ConicalReferenceError> {
    let joins = ConicalJoinsQuery {
        spans: member.spans,
        contour_lengths: member.contour_lengths,
        options: member.joins.options,
    };
    if !member.joins.matches(&joins) || member.joins.directions.len() != member.spans.len() {
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
    query: &ConicalReferenceQuery<'_>,
) -> Result<(), ConicalReferenceFailure> {
    let input = span.query;
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
fn first<'a>(
    query: &ConicalReferenceQuery<'a>,
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
    query: &ConicalReferenceQuery<'_>,
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
fn snapshots(query: &ConicalReferenceQuery<'_>) -> Vec<Vec<BoundarySnapshot>> {
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

pub(super) fn matches(proof: &ConicalReferenceEvidence, query: &ConicalReferenceQuery<'_>) -> bool {
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

fn components(query: &ConicalReferenceQuery<'_>) -> Vec<u64> {
    let policy = query.clearance.requested_options();
    let sector = query.sector.sector.policy;
    let mut bits = vec![
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
    ];
    for member in query.members {
        join_bits(member.joins, &mut bits);
        boundary_bits(member.spans, &mut bits);
    }
    bits
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
