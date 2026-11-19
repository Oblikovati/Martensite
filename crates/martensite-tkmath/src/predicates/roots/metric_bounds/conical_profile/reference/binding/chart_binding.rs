// SPDX-License-Identifier: GPL-2.0-only
//! Reference admission retains the executed join mode and actual domain authority.
use super::*;
pub(super) fn joins_match_selected(
    member: &ConicalReferenceMember<'_>,
    joins: &ConicalJoinsQuery<'_>,
    domain: ReferenceDomain<'_>,
    refinement: ConicalPrimitiveRefinement,
) -> bool {
    let matches = match domain {
        ReferenceDomain::Legacy(_) => member.joins.matches(joins),
        // #552: reference topology consumes the executed chart authority.
        ReferenceDomain::Chart(domain) => member
            .joins
            .matches_chart_with_refinement(joins, domain, refinement),
    };
    matches && member.joins.directions.len() == member.spans.len()
}
