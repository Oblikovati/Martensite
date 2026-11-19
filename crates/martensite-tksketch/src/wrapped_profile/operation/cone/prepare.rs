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
    request: &WrappedConeNormalRequest,
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
    request: &WrappedConeNormalRequest,
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
    request: &WrappedConeNormalRequest,
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
    let chart = admitted_chart(request, &domain)?;
    let mut prepared = prepare_regions(request, originals, &chart, execution, work, cancelled)?;
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
    domain: &'a domain::DomainEvidence,
}
fn admitted_chart<'a>(
    request: &WrappedConeNormalRequest,
    domain: &'a domain::DomainEvidence,
) -> ConeChecked<AdmittedConeChart<'a>> {
    let wedge = domain.wedge()?;
    Ok(AdmittedConeChart {
        wedge,
        center: common_center(request, wedge)?,
        domain,
    })
}

fn prepare_regions(
    request: &WrappedConeNormalRequest,
    originals: &[DevelopedRegionRequest],
    chart: &AdmittedConeChart<'_>,
    execution: WrappedConeExecutionSelection,
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<Vec<Prepared>> {
    let mut endpoints = endpoints::EndpointPool::default();
    let prepared = originals
        .iter()
        .map(|source| {
            prepare_region(
                request,
                source,
                chart,
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
    request: &WrappedConeNormalRequest,
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
    request: &WrappedConeNormalRequest,
    source: &DevelopedRegionRequest,
    chart: &AdmittedConeChart<'_>,
    execution: WrappedConeExecutionSelection,
) -> ConeChecked<Prepared> {
    Ok(Prepared {
        executed_selection: execution,
        source: source.clone(),
        fits: BTreeMap::new(),
        final_endpoints: None,
        directed_family: None,
        reference_slant: reference(source)?,
        semi_angle: request.support.semi_angle,
        chart_origin: request.support.chart_origin,
        chart_center: chart.center,
        chart_domain: chart.domain.chart_owner(),
        arithmetic_operations: 0,
        radii: Vec::new(),
        wedge: chart.wedge,
    })
}
fn prepare_contour(
    prepared: &mut Prepared,
    contour: &[AuthoredPiece],
    endpoints: &mut endpoints::EndpointPool,
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<Vec<AuthoredPiece>> {
    let mut pieces = Vec::new();
    for source in contour {
        radius(prepared, source, work, cancelled)?;
        pieces.extend(fit::plan(prepared, source, endpoints, work, cancelled)?);
    }
    Ok(pieces)
}
fn reference(source: &DevelopedRegionRequest) -> ConeChecked<f64> {
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

fn common_center(
    request: &WrappedConeNormalRequest,
    wedge: ConicalSectorWedge,
) -> ConeChecked<f64> {
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
        WrappedConeExecutionSelection::RoundedProxyNativeSearchV1 { domain } => domain,
        WrappedConeExecutionSelection::DirectedCommonChartRequireCertifiedV1 => {
            WrappedConeDomainPolicy::PositiveXCommonChartV2
        }
    }
}
