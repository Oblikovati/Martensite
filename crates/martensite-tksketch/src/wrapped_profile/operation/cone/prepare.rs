//! Select owned planar material and admit one cone mapping before fitting its spans.
use super::storage::Work;
use super::*;
mod canonicalize;
mod clearance;
pub(super) mod directed;
mod endpoints;
mod fit;
pub(super) mod selection;
pub(super) fn select(
    request: &ConeRun<'_>,
    originals: &[DevelopedRegionRequest],
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<Selection> {
    select_with_policy(
        request,
        originals,
        WrappedConeDomainPolicy::LegacySectorV1,
        work,
        cancelled,
    )
}
pub(super) fn select_with_policy(
    request: &ConeRun<'_>,
    originals: &[DevelopedRegionRequest],
    policy: WrappedConeDomainPolicy,
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<Selection> {
    select_with_execution(
        request,
        originals,
        WrappedConeExecutionSelection::RoundedProxyNativeSearchV1 { domain: policy },
        work,
        cancelled,
    )
}
pub(super) fn select_with_execution(
    request: &ConeRun<'_>,
    originals: &[DevelopedRegionRequest],
    execution: WrappedConeExecutionSelection,
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<Selection> {
    directed::admit(request, execution)?;
    let policy = execution_domain(execution);
    let started = work.consumed;
    let planar = selection::planar(request, originals, work, cancelled)?;
    let domain = domain::certify(request, originals, policy, work, cancelled)?;
    let mut prepared = prepare_regions(request, originals, &domain, execution, work, cancelled)?;
    let clearance = clearance::certify(originals, &prepared, work, cancelled)?;
    directed::finalize(execution, originals, &mut prepared, work, cancelled)?;
    Ok(selection::completed(
        execution,
        planar,
        prepared,
        domain,
        clearance,
        work.consumed - started,
    ))
}

struct AdmittedConeChart<'a> {
    wedge: ConicalSectorWedge,
    center: f64,
    proof: Option<&'a std::sync::Arc<ConicalChartDomainEvidence>>,
    local_reference_slant: Option<f64>,
}
fn admitted_chart<'a>(
    request: &ConeRun<'_>,
    domain: &'a domain::DomainEvidence,
    region: SurfaceRegionId,
    wedge: ConicalSectorWedge,
) -> ConeChecked<AdmittedConeChart<'a>> {
    if let domain::DomainEvidence::Connected { reference_slant } = domain {
        return Ok(AdmittedConeChart {
            wedge,
            center: request.support.chart_origin,
            proof: None,
            local_reference_slant: Some(*reference_slant),
        });
    }
    let proof = domain.owner(region)?;
    let center = match proof {
        Some(chart) => chart.fit_center(),
        None => common_center(request, wedge)?,
    };
    Ok(AdmittedConeChart {
        wedge,
        center,
        proof,
        local_reference_slant: None,
    })
}

fn prepare_regions(
    request: &ConeRun<'_>,
    originals: &[DevelopedRegionRequest],
    domain: &domain::DomainEvidence,
    execution: WrappedConeExecutionSelection,
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<Vec<Prepared>> {
    let wedge = domain.wedge()?;
    let mut endpoints = endpoints::EndpointPool::default();
    let prepared = originals
        .iter()
        .map(|source| {
            let chart = admitted_chart(request, domain, source.region, wedge)?;
            prepare_region(
                request,
                source,
                &chart,
                execution,
                &mut endpoints,
                work,
                cancelled,
            )
        })
        .collect::<ConeChecked<Vec<Prepared>>>()?;
    Ok(prepared)
}

fn prepare_region(
    request: &ConeRun<'_>,
    source: &DevelopedRegionRequest,
    chart: &AdmittedConeChart<'_>,
    execution: WrappedConeExecutionSelection,
    endpoints: &mut endpoints::EndpointPool,
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<Prepared> {
    let started = work.consumed;
    let mut prepared = initial(request, source, chart, execution)?;
    let mut loops = Vec::new();
    for contour in &source.loops {
        loops.push(prepare_contour(
            &mut prepared,
            contour,
            chart.local_reference_slant.is_some(),
            endpoints,
            work,
            cancelled,
        )?);
    }
    prepared.source.loops = loops;
    prepared.arithmetic_operations = work.consumed - started;
    canonicalize::uv_endpoints(&mut prepared)?;
    Ok(prepared)
}
fn initial(
    request: &ConeRun<'_>,
    source: &DevelopedRegionRequest,
    chart: &AdmittedConeChart<'_>,
    execution: WrappedConeExecutionSelection,
) -> ConeChecked<Prepared> {
    Ok(Prepared {
        executed_selection: execution,
        source: source.clone(),
        authored_positions: source_positions::AuthoredPositions::capture(source),
        fits: BTreeMap::new(),
        final_endpoints: None,
        directed_family: None,
        reference_slant: match chart.local_reference_slant {
            Some(slant) => slant,
            None => reference(source)?,
        },
        semi_angle: request.support.semi_angle,
        chart_origin: request.support.chart_origin,
        chart_center: chart.center,
        chart_domain: chart.proof.cloned(),
        local_charts: Vec::new(),
        arithmetic_operations: 0,
        radii: Vec::new(),
        wedge: chart.wedge,
    })
}
fn prepare_contour(
    prepared: &mut Prepared,
    contour: &[AuthoredPiece],
    local_charts: bool,
    endpoints: &mut endpoints::EndpointPool,
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<Vec<AuthoredPiece>> {
    let mut pieces = Vec::new();
    for source in contour {
        radius(prepared, source, work, cancelled)?;
        if local_charts {
            fit::plan_local(prepared, source, endpoints, work, cancelled, &mut pieces)?;
        } else {
            pieces.extend(fit::plan(
                prepared, source, None, endpoints, work, cancelled,
            )?);
        }
    }
    Ok(pieces)
}
pub(super) fn reference(source: &DevelopedRegionRequest) -> ConeChecked<f64> {
    let point = source
        .loops
        .first()
        .and_then(|c| c.first())
        .and_then(|p| p.poles.first())
        .ok_or_else(|| failure(Some(source.region), None, ConeCause::Missing(source.region)))?;
    Ok(martensite_cmath::hypot(point[0], point[1]))
}
fn radius(
    prepared: &mut Prepared,
    source: &AuthoredPiece,
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<()> {
    let query = radius_query(prepared, source, work.remaining());
    let receipt = certify_cone_request(query, cancelled).map_err(|cause| {
        failure(
            Some(prepared.source.region),
            Some(source.span),
            ConeCause::Radius(Box::new(cause)),
        )
    })?;
    work.charge(
        prepared.source.region,
        Some(source.span),
        receipt.result().proof().operations(),
    )?;
    prepared.radii.push(receipt);
    Ok(())
}
fn radius_query<'a>(
    prepared: &Prepared,
    source: &'a AuthoredPiece,
    options: ConeClearanceOptions,
) -> ConeRequest<'a> {
    ConeRequest {
        poles: &source.poles,
        semi_angle: prepared.semi_angle,
        depths: prepared.source.depths,
        outward: true,
        clearance: prepared.source.tolerance,
        max_depth: 16,
        options,
    }
}
pub(super) fn failure(
    region: Option<SurfaceRegionId>,
    span: Option<CurveSpan>,
    cause: ConeCause,
) -> ConeFailure {
    ConeFailure::new(region, span, WrappedFailureClass::Incomplete, cause)
        .at(WrappedStage::Construction)
}

fn common_center(request: &ConeRun<'_>, wedge: ConicalSectorWedge) -> ConeChecked<f64> {
    if wedge == ConicalSectorWedge::PositiveX {
        return Ok(request.support.chart_origin);
    }
    Err(ConeFailure::new(
        None,
        None,
        WrappedFailureClass::Unsupported,
        ConeCause::Scalar {
            field: "Phase A developed-sector wedge",
            actual: f64::from(wedge as u8),
            expected: "PositiveX for the single common chart",
        },
    ))
}

fn execution_domain(execution: WrappedConeExecutionSelection) -> WrappedConeDomainPolicy {
    match execution {
        WrappedConeExecutionSelection::RoundedProxyNativeSearchV1 { domain }
        | WrappedConeExecutionSelection::DirectedTransportedPlacementV5 { domain, .. } => domain,
        WrappedConeExecutionSelection::DirectedCommonChartRequireCertifiedV1 => {
            WrappedConeDomainPolicy::PositiveXCommonChartV2
        }
        WrappedConeExecutionSelection::DirectedFamilyChartRequireCertifiedV2 { .. } => {
            WrappedConeDomainPolicy::PositiveXFamilyChartV3
        }
        WrappedConeExecutionSelection::DirectedRecenteredChartRequireCertifiedV3 { .. } => {
            WrappedConeDomainPolicy::PositiveXRecenteredChartV4
        }
        WrappedConeExecutionSelection::DirectedRegionAtlasRequireCertifiedV4 { .. } => {
            WrappedConeDomainPolicy::PositiveXRegionAtlasV5
        }
        WrappedConeExecutionSelection::ConnectedChartAtlasV6 { .. } => {
            WrappedConeDomainPolicy::PositiveXConnectedAtlasV6
        }
    }
}
