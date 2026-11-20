//! The retained domain chooses a reference proof, never a second construction pipeline.
use super::*;
use crate::surface_sketch::wrapped_profile::operation::cone::domain::DomainEvidence;
pub(super) enum ReferenceQuery<'a> {
    Legacy(ConicalReferenceQuery<'a>),
    Chart(ConicalChartReferenceQuery<'a>),
    Refined(ConicalChartReferenceQuery<'a>, ConicalPrimitiveRefinement),
}
impl ReferenceQuery<'_> {
    pub(super) fn certify(
        self,
        cancelled: &mut CancelPoll<'_>,
    ) -> Result<ConicalReferenceEvidence, ConicalReferenceError> {
        match self {
            Self::Legacy(query) => certify_conical_reference(query, cancelled),
            Self::Chart(query) => certify_conical_chart_reference(query, cancelled),
            Self::Refined(query, refinement) => {
                certify_conical_chart_reference_with_refinement(query, refinement, cancelled)
            }
        }
    }
    pub(super) fn matches(&self, proof: &ConicalReferenceEvidence) -> bool {
        match self {
            Self::Legacy(query) => proof.matches(query),
            Self::Chart(query) => proof.matches_chart(query),
            Self::Refined(query, refinement) => {
                proof.matches_chart_with_refinement(query, *refinement)
            }
        }
    }
}
impl Inputs<'_> {
    pub(super) fn query<'a>(
        &'a self,
        members: &'a [ConicalReferenceMember<'a>],
        family: &'a OwnedFamily<Selection, Geometry>,
        spatial_limit: f64,
        options: ConeClearanceOptions,
    ) -> ReferenceQuery<'a> {
        let source = QuerySource {
            members,
            clearance: &family.selection.clearance,
            spans: &self.sector,
            spatial_limit,
            options,
        };
        match &family.selection.domain {
            DomainEvidence::Legacy { sector, .. } => source.legacy(sector),
            DomainEvidence::Chart(proof, _) => {
                let query = source.chart(proof);
                match query {
                    ReferenceQuery::Chart(query)
                        if family.selection.executed_selection.is_directed() =>
                    {
                        ReferenceQuery::Refined(query, ConicalPrimitiveRefinement::HalfSplitV1)
                    }
                    query => query,
                }
            }
        }
    }
}
struct QuerySource<'a> {
    members: &'a [ConicalReferenceMember<'a>],
    clearance: &'a CertifiedCylindricalClearance,
    spans: &'a [&'a [[f64; 2]]],
    spatial_limit: f64,
    options: ConeClearanceOptions,
}
impl<'a> QuerySource<'a> {
    fn legacy(self, sector: &'a ConicalSectorEvidence) -> ReferenceQuery<'a> {
        ReferenceQuery::Legacy(ConicalReferenceQuery {
            members: self.members,
            clearance: self.clearance,
            sector,
            sector_spans: self.spans,
            spatial_limit: self.spatial_limit,
            options: self.options,
        })
    }
    fn chart(self, domain: &'a ConicalChartDomainEvidence) -> ReferenceQuery<'a> {
        ReferenceQuery::Chart(ConicalChartReferenceQuery {
            members: self.members,
            clearance: self.clearance,
            domain,
            domain_spans: self.spans,
            spatial_limit: self.spatial_limit,
            options: self.options,
        })
    }
}
