// SPDX-License-Identifier: GPL-2.0-only
//! Shared monotone projections cover self/adjacent pairs; the exact complement uses clearance.
use super::*;
pub(super) fn bind(query: &ConicalReferenceQuery<'_>) -> Result<(), ConicalReferenceFailure> {
    let sources = sources(query)?;
    let clearance = query.clearance;
    if !clearance.matches_sources(&sources)
        || clearance.scope() != CylindricalClearanceScope::Within
        || clearance.support_radius() != 1.
        || clearance.angular_origin() != 0.
    {
        return Err(binding_failure(
            "complete source-bound original nonadjacent clearance",
        ));
    }
    if clearance.excluded_pairs() != exclusions(query) {
        return Err(binding_failure("only source-keyed adjacent pairs excluded"));
    }
    sector(query)
}
fn sources<'a>(
    query: &ConicalReferenceQuery<'a>,
) -> Result<Vec<CylindricalClearanceSource<'a>>, ConicalReferenceFailure> {
    query
        .members
        .iter()
        .flat_map(|member| member.spans)
        .map(|span| {
            let original = span
                .query
                .original
                .ok_or_else(|| binding_failure("original clearance interval"))?;
            let interval = CylindricalProfileInterval::new(original.index, original.level)
                .map_err(|_| binding_failure("valid original dyadic interval"))?;
            Ok(CylindricalClearanceSource {
                authored: original.poles,
                interval,
            })
        })
        .collect()
}
fn exclusions(query: &ConicalReferenceQuery<'_>) -> Vec<[usize; 2]> {
    let mut result = std::collections::BTreeSet::new();
    let mut offset = 0;
    for count in query
        .members
        .iter()
        .flat_map(|member| member.contour_lengths)
    {
        adjacent(*count, offset, &mut result);
        offset += count;
    }
    result.into_iter().collect()
}
fn adjacent(count: usize, offset: usize, result: &mut std::collections::BTreeSet<[usize; 2]>) {
    for index in 0..count {
        let mut pair = [offset + index, offset + (index + 1) % count];
        pair.sort();
        result.insert(pair);
    }
}
fn sector(query: &ConicalReferenceQuery<'_>) -> Result<(), ConicalReferenceFailure> {
    let policy = query.sector.sector.policy;
    let sector = ConicalSectorQuery {
        spans: query.sector_spans,
        semi_angle: query.members[0].spans[0].query.semi_angle,
        options: ConeClearanceOptions {
            fractional_bits: policy.fractional_bits,
            max_operations: policy.max_operations,
        },
    };
    if !query.sector.matches(&sector) || query.sector.wedge() != Some(ConicalSectorWedge::PositiveX)
    {
        return Err(binding_failure(
            "executed original PositiveX quarter-turn wedge",
        ));
    }
    if !covers_originals(query) {
        return Err(binding_failure("sector and original-boundary coverage"));
    }
    Ok(())
}
fn covers_originals(query: &ConicalReferenceQuery<'_>) -> bool {
    let originals: Vec<_> = query
        .members
        .iter()
        .flat_map(|member| member.spans)
        .filter_map(|span| span.query.original.map(|original| original.poles))
        .collect();
    query
        .sector_spans
        .iter()
        .all(|original| originals.contains(original))
        && originals
            .iter()
            .all(|original| query.sector_spans.contains(original))
}
pub(super) fn separation(
    query: &ConicalReferenceQuery<'_>,
    displacement: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(), ConicalReferenceFailure> {
    let epsilon = arithmetic.scalar(query.spatial_limit)?;
    let required = arithmetic.fraction(&epsilon, 4, 1)?;
    let clearance = arithmetic.scalar(query.clearance.clearance_lower_bound())?;
    let tubes = arithmetic.fraction(displacement, 2, 1)?;
    if clearance.lower < required.upper || clearance.lower <= tubes.upper {
        return Err(binding_failure(
            "four-epsilon nonadjacent clearance exceeds both homotopy tubes",
        ));
    }
    let bound = DyadicBound::new(
        displacement.upper.clone(),
        -i32::from(arithmetic.fractional_bits()),
    );
    if !query.sector.admits_bound(&bound) {
        return Err(binding_failure(
            "perturbed ideal fit remains inside strict source wedge",
        ));
    }
    Ok(())
}
