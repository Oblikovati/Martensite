// SPDX-License-Identifier: GPL-2.0-only
//! The common reference proof composes independently centered, periodically disjoint members.
use super::*;
mod phase;

/// Chart authority and ordered authored controls for one selected region.
#[derive(Clone, Copy)]
pub struct ConicalAtlasReferenceDomain<'a> {
    /// Chart proof for this member's complete original boundary.
    pub domain: &'a ConicalChartDomainEvidence,
    /// Original unsplit controls in the order certified by the member chart.
    pub domain_spans: &'a [&'a [[f64; 2]]],
}

/// Reference composition for a family of disconnected regions with separate fit centers.
pub struct ConicalAtlasReferenceQuery<'a> {
    /// Selected members in stable source order.
    pub members: &'a [ConicalReferenceMember<'a>],
    /// One executed chart authority for each member, in the same source order.
    pub domains: &'a [ConicalAtlasReferenceDomain<'a>],
    /// Original nonadjacent boundary clearance for the complete family.
    pub clearance: &'a CertifiedCylindricalClearance,
    /// Original spatial error limit.
    pub spatial_limit: f64,
    /// Remaining aggregate arithmetic allowance.
    pub options: ConeClearanceOptions,
}

/// Certify an atlas reference and exclude its periodic overlaps.
/// Example: `certify_conical_atlas_reference(query, refinement, || false)?`.
pub fn certify_conical_atlas_reference(
    query: ConicalAtlasReferenceQuery<'_>,
    refinement: ConicalPrimitiveRefinement,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<ConicalReferenceEvidence, ConicalReferenceError> {
    let input = compose(&query, refinement)?;
    binding::admit(&input)?;
    let mut arithmetic = Arithmetic::new(sector_api::policy(input.options), &mut cancelled);
    let result = geometry::certify(&input, &mut arithmetic)
        .map_err(|cause| error(cause, arithmetic.operations()))?;
    Ok(binding::evidence(&input, result, arithmetic.operations()))
}
impl ConicalReferenceEvidence {
    /// Rebind each member to its own executed atlas chart and the complete family inputs.
    /// Example: `proof.matches_atlas(&query, refinement)`.
    pub fn matches_atlas(
        &self,
        query: &ConicalAtlasReferenceQuery<'_>,
        refinement: ConicalPrimitiveRefinement,
    ) -> bool {
        compose(query, refinement).is_ok_and(|input| binding::matches(self, &input))
    }
}

fn compose<'a>(
    query: &ConicalAtlasReferenceQuery<'a>,
    refinement: ConicalPrimitiveRefinement,
) -> Result<ReferenceInputs<'a>, ConicalReferenceError> {
    let first = query
        .domains
        .first()
        .ok_or_else(|| error(binding_failure("nonempty cone atlas"), 0))?;
    Ok(ReferenceInputs {
        members: query.members,
        clearance: query.clearance,
        sector: ReferenceDomain::Chart(first.domain),
        sector_spans: &[],
        atlas: Some(query.domains),
        spatial_limit: query.spatial_limit,
        options: query.options,
        refinement,
    })
}

pub(super) fn bind_domains(
    query: &ReferenceInputs<'_>,
    domains: &[ConicalAtlasReferenceDomain<'_>],
) -> Result<(), ConicalReferenceFailure> {
    for (member, owned) in query.members.iter().zip(domains) {
        bind_member(member, owned, query.options.fractional_bits)?;
    }
    Ok(())
}
fn bind_member(
    member: &ConicalReferenceMember<'_>,
    owned: &ConicalAtlasReferenceDomain<'_>,
    bits: u16,
) -> Result<(), ConicalReferenceFailure> {
    let Some(first) = member.spans.first() else {
        return Err(binding_failure("nonempty atlas member"));
    };
    let domain = owned.domain;
    if domain.version() != ConicalChartDomainVersion::AtlasMemberV5
        || domain.options().fractional_bits != bits
        || !domain.matches_with_version(&domain_query(first.query, owned), domain.version())
    {
        return Err(binding_failure(
            "exact atlas member chart and original source order",
        ));
    }
    let fits = member
        .spans
        .iter()
        .filter_map(|span| span.query.original.map(|span| span.poles));
    if !domain
        .sector
        .original_index
        .covers(&domain.sector.source, fits)
    {
        return Err(binding_failure(
            "complete original-boundary coverage in atlas member",
        ));
    }
    Ok(())
}
fn domain_query<'a>(
    first: ConicalBoundaryQuery<'_>,
    owned: &ConicalAtlasReferenceDomain<'a>,
) -> ConicalChartDomainQuery<'a> {
    ConicalChartDomainQuery {
        spans: owned.domain_spans,
        semi_angle: first.semi_angle,
        chart_origin: first.chart_origin,
        fit_center: first.fit_center,
        developed_turn: first.developed_turn,
        physical_turn: first.physical_turn,
        options: owned.domain.options(),
    }
}

pub(super) fn separate(
    members: &[ConicalReferenceMember<'_>],
    domains: &[ConicalAtlasReferenceDomain<'_>],
    displacement: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<(), ConicalReferenceFailure> {
    let mut ranges = Vec::with_capacity(domains.len());
    for (member, owned) in members.iter().zip(domains) {
        chart::angular_members(std::slice::from_ref(member), owned.domain, arithmetic)?;
        ranges.push(chart::homotopy_range(
            owned.domain,
            displacement,
            arithmetic,
        )?);
    }
    phase::separation(&ranges, arithmetic)
}
