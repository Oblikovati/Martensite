//! Retain the selected domain authority without changing the shared operation sequence.
use super::*;
use std::sync::Arc;
pub(in crate::surface_sketch::wrapped_profile) enum DomainEvidence {
    Legacy {
        sector: ConicalSectorEvidence,
        options: ConeClearanceOptions,
    },
    Chart(Arc<ConicalChartDomainEvidence>, WrappedConeDomainPolicy),
    Atlas(BTreeMap<SurfaceRegionId, Arc<ConicalChartDomainEvidence>>),
    Connected {
        reference_slant: f64,
    },
}
impl DomainEvidence {
    pub(in crate::surface_sketch::wrapped_profile) fn policy(&self) -> WrappedConeDomainPolicy {
        match self {
            Self::Legacy { .. } => WrappedConeDomainPolicy::LegacySectorV1,
            Self::Chart(_, policy) => *policy,
            Self::Atlas(_) => WrappedConeDomainPolicy::PositiveXRegionAtlasV5,
            Self::Connected { .. } => WrappedConeDomainPolicy::PositiveXConnectedAtlasV6,
        }
    }
    pub(in crate::surface_sketch::wrapped_profile) fn chart(
        &self,
    ) -> Option<&ConicalChartDomainEvidence> {
        match self {
            Self::Legacy { .. } | Self::Atlas(_) | Self::Connected { .. } => None,
            Self::Chart(proof, _) => Some(proof),
        }
    }
    pub(super) fn owner(
        &self,
        region: SurfaceRegionId,
    ) -> ConeChecked<Option<&Arc<ConicalChartDomainEvidence>>> {
        match self {
            Self::Legacy { .. } | Self::Connected { .. } => Ok(None),
            Self::Chart(proof, _) => Ok(Some(proof)),
            Self::Atlas(charts) => charts
                .get(&region)
                .map(Some)
                .ok_or_else(|| prepare::failure(Some(region), None, ConeCause::Missing(region))),
        }
    }
    pub(in crate::surface_sketch::wrapped_profile) fn region_chart(
        &self,
        region: SurfaceRegionId,
    ) -> Option<&ConicalChartDomainEvidence> {
        self.owner(region).ok().flatten().map(Arc::as_ref)
    }
    pub(super) fn wedge(&self) -> ConeChecked<ConicalSectorWedge> {
        match self {
            Self::Legacy { sector, .. } => prepare::selection::wedge(sector),
            Self::Chart(proof, _) => Ok(proof.wedge()),
            Self::Atlas(_) | Self::Connected { .. } => Ok(ConicalSectorWedge::PositiveX),
        }
    }
}
pub(super) fn certify(
    request: &ConeRun<'_>,
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
        WrappedConeDomainPolicy::PositiveXRecenteredChartV4 => chart(
            request,
            originals,
            policy,
            ConicalChartDomainVersion::RecenteredV4,
            work,
            cancelled,
        ),
        WrappedConeDomainPolicy::PositiveXRegionAtlasV5 => {
            atlas(request, originals, work, cancelled)
        }
        WrappedConeDomainPolicy::PositiveXConnectedAtlasV6 => connected(originals),
    }
}
fn connected(originals: &[DevelopedRegionRequest]) -> ConeChecked<DomainEvidence> {
    admit_atlas_capacity(
        originals,
        ConicalChartDomainVersion::AtlasMemberV5.span_limit(),
    )?;
    let first = originals
        .iter()
        .min_by_key(|source| source.region)
        .ok_or_else(|| {
            prepare::failure(
                None,
                None,
                ConeCause::Scalar {
                    field: "connected cone atlas members",
                    actual: 0.,
                    expected: "at least one source region",
                },
            )
        })?;
    prepare::reference(first).map(|reference_slant| DomainEvidence::Connected { reference_slant })
}
fn chart(
    request: &ConeRun<'_>,
    originals: &[DevelopedRegionRequest],
    policy: WrappedConeDomainPolicy,
    version: ConicalChartDomainVersion,
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<DomainEvidence> {
    chart_proof(request, originals, version, work, cancelled)
        .map(|proof| DomainEvidence::Chart(proof, policy))
}
fn chart_proof(
    request: &ConeRun<'_>,
    originals: &[DevelopedRegionRequest],
    version: ConicalChartDomainVersion,
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<Arc<ConicalChartDomainEvidence>> {
    let spans = original_spans(originals);
    let query = ConicalChartDomainQuery {
        spans: &spans,
        semi_angle: request.support.semi_angle,
        chart_origin: request.support.chart_origin,
        fit_center: fit_center(request.support, &spans, version),
        developed_turn: 0,
        physical_turn: 0,
        options: work.remaining(),
    };
    charged(
        certify_conical_chart_domain_with_version(query, version, cancelled),
        originals,
        work,
    )
}
fn atlas(
    request: &ConeRun<'_>,
    originals: &[DevelopedRegionRequest],
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<DomainEvidence> {
    let version = ConicalChartDomainVersion::AtlasMemberV5;
    admit_atlas_capacity(originals, version.span_limit())?;
    let charts = originals
        .iter()
        .map(|original| {
            let chart = chart_proof(
                request,
                std::slice::from_ref(original),
                version,
                work,
                cancelled,
            )
            .map_err(|mut failure| {
                failure.region = Some(original.region);
                failure
            })?;
            Ok((original.region, chart))
        })
        .collect::<ConeChecked<_>>()?;
    Ok(DomainEvidence::Atlas(charts))
}
fn admit_atlas_capacity(originals: &[DevelopedRegionRequest], limit: usize) -> ConeChecked<()> {
    let actual = originals
        .iter()
        .flat_map(|source| &source.loops)
        .map(Vec::len)
        .sum();
    if !(1..=limit).contains(&actual) {
        let cause = ConicalChartDomainFailure::SourceCount { actual, limit };
        return Err(failure(
            originals,
            ConicalChartDomainError {
                cause,
                operations: 0,
            },
        ));
    }
    Ok(())
}
fn fit_center(
    support: ConeSupport,
    spans: &[&[[f64; 2]]],
    version: ConicalChartDomainVersion,
) -> f64 {
    if !matches!(
        version,
        ConicalChartDomainVersion::RecenteredV4 | ConicalChartDomainVersion::AtlasMemberV5
    ) || !(1..=version.span_limit()).contains(&spans.len())
    {
        return support.chart_origin;
    }
    let (min, max) = spans
        .iter()
        .flat_map(|span| span.iter())
        .map(|[x, y]| y / x)
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), slope| {
            (min.min(slope), max.max(slope))
        });
    // Positive-X angular extrema occur at control poles. This estimate only chooses a
    // center; domain admission certifies the whole angular range using bounded arithmetic.
    let angle = (martensite_cmath::atan(min) + martensite_cmath::atan(max)) * 0.5;
    support.chart_origin + angle / martensite_cmath::sin(support.semi_angle)
}
fn charged(
    result: Result<ConicalChartDomainEvidence, ConicalChartDomainError>,
    originals: &[DevelopedRegionRequest],
    work: &mut storage::Work,
) -> ConeChecked<Arc<ConicalChartDomainEvidence>> {
    let operations = result.as_ref().map_or_else(
        |cause| cause.operations,
        ConicalChartDomainEvidence::operations,
    );
    let charged = prepare::selection::first_region(originals)
        .and_then(|region| work.charge(region, None, operations));
    match (result, charged) {
        (Err(cause), _) => Err(failure(originals, cause)),
        (Ok(_), Err(cause)) => Err(cause),
        (Ok(proof), Ok(())) => Ok(Arc::new(proof)),
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

pub(super) use boundary_metric::{BoundaryMetric, ChartSlot, LocalChart, covers};
mod boundary_metric;

pub(super) fn directed(prepared: &Prepared) -> bool {
    prepared.executed_selection.is_directed()
}
