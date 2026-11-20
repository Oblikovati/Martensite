// SPDX-License-Identifier: GPL-2.0-only
//! One private composition view keeps both public admission behaviors separate.
use super::*;
pub(super) struct ReferenceInputs<'a> {
    pub members: &'a [ConicalReferenceMember<'a>],
    pub clearance: &'a CertifiedCylindricalClearance,
    pub sector: ReferenceDomain<'a>,
    pub sector_spans: &'a [&'a [[f64; 2]]],
    pub spatial_limit: f64,
    pub options: ConeClearanceOptions,
    pub refinement: ConicalPrimitiveRefinement,
}
#[derive(Clone, Copy)]
pub(super) enum ReferenceDomain<'a> {
    Legacy(&'a ConicalSectorEvidence),
    Chart(&'a ConicalChartDomainEvidence),
}
impl<'a> From<&ConicalReferenceQuery<'a>> for ReferenceInputs<'a> {
    fn from(query: &ConicalReferenceQuery<'a>) -> Self {
        Self {
            members: query.members,
            clearance: query.clearance,
            sector: ReferenceDomain::Legacy(query.sector),
            sector_spans: query.sector_spans,
            spatial_limit: query.spatial_limit,
            options: query.options,
            refinement: ConicalPrimitiveRefinement::UnsplitV1,
        }
    }
}
impl<'a> From<&ConicalChartReferenceQuery<'a>> for ReferenceInputs<'a> {
    fn from(query: &ConicalChartReferenceQuery<'a>) -> Self {
        Self {
            members: query.members,
            clearance: query.clearance,
            sector: ReferenceDomain::Chart(query.domain),
            sector_spans: query.domain_spans,
            spatial_limit: query.spatial_limit,
            options: query.options,
            refinement: ConicalPrimitiveRefinement::UnsplitV1,
        }
    }
}
impl ReferenceDomain<'_> {
    pub fn covers_originals<'a>(self, fitted: impl Iterator<Item = &'a [[f64; 2]]>) -> bool {
        let sector = match self {
            Self::Legacy(proof) => &proof.sector,
            Self::Chart(proof) => &proof.sector,
        };
        sector.original_index.covers(&sector.source, fitted)
    }
    pub fn policy(self) -> ArithmeticPolicy {
        match self {
            Self::Legacy(proof) => proof.sector.policy,
            Self::Chart(proof) => proof.sector.policy,
        }
    }
    pub fn operations(self) -> u32 {
        match self {
            Self::Legacy(proof) => proof.operations(),
            Self::Chart(proof) => proof.operations(),
        }
    }
    pub fn admits_bound(self, bound: &DyadicBound) -> bool {
        match self {
            Self::Legacy(proof) => proof.admits_bound(bound),
            Self::Chart(proof) => proof.admits_bound(bound),
        }
    }
    pub fn wedge(self) -> Option<ConicalSectorWedge> {
        match self {
            Self::Legacy(proof) => proof.wedge(),
            Self::Chart(proof) => Some(proof.wedge()),
        }
    }
    pub fn matches(self, query: &ConicalSectorQuery<'_>) -> bool {
        match self {
            Self::Legacy(proof) => proof.matches(query),
            Self::Chart(proof) => proof.sector.matches(
                query.spans,
                query.semi_angle,
                sector_api::policy(query.options),
            ),
        }
    }
}
