//! Select owned planar material and admit one cone mapping before fitting its spans.
use super::storage::Work;
use super::*;
mod canonicalize;
mod clearance;
mod endpoints;
mod fit;
mod selection;
pub(super) fn select(
    request: &WrappedConeNormalRequest,
    originals: &[DevelopedRegionRequest],
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<Selection> {
    let started = work.consumed;
    let planar = selection::planar(request, originals, work, cancelled)?;
    let sector_options = work.remaining();
    let sector = selection::sector(request, originals, work, cancelled)?;
    let wedge = selection::wedge(&sector)?;
    let chart_center = common_center(request, wedge)?;
    let prepared = prepare_regions(request, originals, wedge, chart_center, work, cancelled)?;
    let clearance = clearance::certify(originals, &prepared, work, cancelled)?;
    Ok(selection::completed(
        planar,
        prepared,
        sector,
        sector_options,
        clearance,
        work.consumed - started,
    ))
}
fn prepare_regions(
    request: &WrappedConeNormalRequest,
    originals: &[DevelopedRegionRequest],
    wedge: ConicalSectorWedge,
    chart_center: f64,
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
                wedge,
                chart_center,
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
    wedge: ConicalSectorWedge,
    chart_center: f64,
    endpoints: &mut endpoints::EndpointPool,
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<Prepared> {
    let started = work.consumed;
    let mut prepared = initial(request, source, wedge, chart_center)?;
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
    wedge: ConicalSectorWedge,
    chart_center: f64,
) -> ConeChecked<Prepared> {
    Ok(Prepared {
        source: source.clone(),
        fits: BTreeMap::new(),
        reference_slant: reference(source)?,
        semi_angle: request.support.semi_angle,
        chart_origin: request.support.chart_origin,
        chart_center,
        arithmetic_operations: 0,
        radii: Vec::new(),
        wedge,
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
