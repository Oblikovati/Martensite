//! Retain the selected domain authority without changing the shared operation sequence.
use super::*;
pub(in crate::surface_sketch::wrapped_profile) enum DomainEvidence {
    Legacy {
        sector: ConicalSectorEvidence,
        options: ConeClearanceOptions,
    },
    Chart(
        std::sync::Arc<ConicalChartDomainEvidence>,
        WrappedConeDomainPolicy,
    ),
}
impl DomainEvidence {
    pub(in crate::surface_sketch::wrapped_profile) fn policy(&self) -> WrappedConeDomainPolicy {
        match self {
            Self::Legacy { .. } => WrappedConeDomainPolicy::LegacySectorV1,
            Self::Chart(_, policy) => *policy,
        }
    }
    pub(in crate::surface_sketch::wrapped_profile) fn chart(
        &self,
    ) -> Option<&ConicalChartDomainEvidence> {
        match self {
            Self::Legacy { .. } => None,
            Self::Chart(proof, _) => Some(proof),
        }
    }
    pub(super) fn chart_owner(&self) -> Option<std::sync::Arc<ConicalChartDomainEvidence>> {
        match self {
            Self::Legacy { .. } => None,
            Self::Chart(proof, _) => Some(std::sync::Arc::clone(proof)),
        }
    }
    pub(super) fn wedge(&self) -> ConeChecked<ConicalSectorWedge> {
        match self {
            Self::Legacy { sector, .. } => prepare::selection::wedge(sector),
            Self::Chart(proof, _) => Ok(proof.wedge()),
        }
    }
}
pub(super) fn certify(
    request: &WrappedConeNormalRequest,
    originals: &[DevelopedRegionRequest],
    policy: WrappedConeDomainPolicy,
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<DomainEvidence> {
    let options = work.remaining();
    match policy {
        WrappedConeDomainPolicy::LegacySectorV1 => Ok(DomainEvidence::Legacy {
            sector: prepare::selection::sector(request, originals, work, cancelled)?,
            options,
        }),
        WrappedConeDomainPolicy::PositiveXCommonChartV2 => chart(
            request,
            originals,
            policy,
            ConicalChartDomainVersion::OriginalV2,
            work,
            cancelled,
        ),
        WrappedConeDomainPolicy::PositiveXFamilyChartV3 => chart(
            request,
            originals,
            policy,
            ConicalChartDomainVersion::FamilyV3,
            work,
            cancelled,
        ),
    }
}
fn chart(
    request: &WrappedConeNormalRequest,
    originals: &[DevelopedRegionRequest],
    policy: WrappedConeDomainPolicy,
    version: ConicalChartDomainVersion,
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<DomainEvidence> {
    let spans = original_spans(originals);
    let query = ConicalChartDomainQuery {
        spans: &spans,
        semi_angle: request.support.semi_angle,
        chart_origin: request.support.chart_origin,
        fit_center: request.support.chart_origin,
        developed_turn: 0,
        physical_turn: 0,
        options: work.remaining(),
    };
    charged(
        certify_conical_chart_domain_with_version(query, version, cancelled),
        originals,
        policy,
        work,
    )
}
fn charged(
    result: Result<ConicalChartDomainEvidence, ConicalChartDomainError>,
    originals: &[DevelopedRegionRequest],
    policy: WrappedConeDomainPolicy,
    work: &mut storage::Work,
) -> ConeChecked<DomainEvidence> {
    let operations = result.as_ref().map_or_else(
        |cause| cause.operations,
        ConicalChartDomainEvidence::operations,
    );
    let charged = prepare::selection::first_region(originals)
        .and_then(|region| work.charge(region, None, operations));
    match (result, charged) {
        (Err(cause), _) => Err(failure(originals, cause)),
        (Ok(_), Err(cause)) => Err(cause),
        (Ok(proof), Ok(())) => Ok(DomainEvidence::Chart(std::sync::Arc::new(proof), policy)),
    }
}
pub(super) fn original_spans(originals: &[DevelopedRegionRequest]) -> Vec<&[[f64; 2]]> {
    originals
        .iter()
        .flat_map(|source| source.loops.iter().flatten())
        .map(|piece| piece.poles.as_slice())
        .collect()
}
fn failure(originals: &[DevelopedRegionRequest], cause: ConicalChartDomainError) -> ConeFailure {
    let index = match &cause.cause {
        ConicalChartDomainFailure::Sector(cause) => prepare::selection::sector_index(cause),
        _ => None,
    };
    let owner = index.and_then(|index| source_owner(originals, index));
    prepare::failure(
        owner.map(|p| p.0),
        owner.map(|p| p.1),
        ConeCause::Domain(Box::new(cause)),
    )
}

fn source_owner(
    originals: &[DevelopedRegionRequest],
    index: usize,
) -> Option<(SurfaceRegionId, CurveSpan)> {
    originals
        .iter()
        .flat_map(|source| {
            source
                .loops
                .iter()
                .flatten()
                .map(move |piece| (source.region, piece.span))
        })
        .nth(index)
}

#[cfg(test)]
mod tests;

pub(super) fn certify_boundary(
    prepared: &Prepared,
    query: ConicalBoundaryQuery<'_>,
    options: ConeClearanceOptions,
    cancelled: &mut CancelPoll<'_>,
) -> Result<ConicalBoundaryEvidence, ConicalBoundaryError> {
    match prepared.chart_domain.as_deref() {
        Some(domain) if directed(prepared) => certify_conical_boundary_in_chart_with_refinement(
            query,
            domain,
            options,
            ConicalPrimitiveRefinement::HalfSplitV1,
            cancelled,
        ),
        Some(domain) => certify_conical_boundary_in_chart(query, domain, options, cancelled),
        None => certify_conical_boundary(query, options, cancelled),
    }
}
pub(super) fn matches_boundary(
    prepared: &Prepared,
    proof: &ConicalBoundaryEvidence,
    query: &ConicalBoundaryQuery<'_>,
) -> bool {
    match prepared.chart_domain.as_deref() {
        Some(domain) if directed(prepared) => proof.matches_chart_with_refinement(
            query,
            domain,
            ConicalPrimitiveRefinement::HalfSplitV1,
        ),
        Some(domain) => proof.matches_chart(query, domain),
        None => proof.matches_legacy(query),
    }
}

pub(super) fn directed(prepared: &Prepared) -> bool {
    prepared.executed_selection.is_directed()
}
