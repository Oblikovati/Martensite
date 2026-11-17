//! Normalize physical depth sense exactly and resolve directed profile sources once.
use super::*;
use crate::surface_sketch::NormalSense;
use std::collections::BTreeMap;
pub(super) struct Inputs {
    pub requests: Vec<Request>,
    pub directions: BTreeMap<SurfaceRegionId, profile_input::CurveDirections>,
    pub placement: Option<Box<placement::Plan>>,
}
pub(super) fn read(
    request: &WrappedNormalRequest,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<Inputs> {
    admit(request).map_err(|failure| failure_projection::selected(failure, request))?;
    let depths = physical_depths(request.depths, request.support.normal);
    let mut inputs = Inputs {
        requests: Vec::new(),
        directions: BTreeMap::new(),
        placement: None,
    };
    read_regions(&mut inputs, request, depths, cancelled)?;
    poll(cancelled, WrappedStage::Input)?;
    inputs.placement = placement::prepare(request, &mut inputs.requests)
        .map_err(|failure| failure_projection::selected(failure, request))?;
    Ok(inputs)
}
fn read_regions(
    inputs: &mut Inputs,
    request: &WrappedNormalRequest,
    depths: [f64; 2],
    cancelled: &mut CancelPoll<'_>,
) -> Checked<()> {
    let Some(first) = request.selections.first() else {
        return Ok(());
    };
    poll(cancelled, WrappedStage::Input)?;
    let reader = profile_input::ProfileReader::new(&request.profile)
        .map_err(|cause| profile_failure(cause, first.region))?;
    inputs.push_region(request, first, depths, &reader)?;
    for selection in request.selections.iter().skip(1) {
        poll(cancelled, WrappedStage::Input)?;
        inputs.push_region(request, selection, depths, &reader)?;
    }
    Ok(())
}
impl Inputs {
    fn push_region(
        &mut self,
        request: &WrappedNormalRequest,
        selection: &WrappedRegionSelection,
        depths: [f64; 2],
        reader: &profile_input::ProfileReader<'_>,
    ) -> Checked<()> {
        let oriented = selected(reader, selection)?;
        let (source, directions) = oriented.into_developed(
            depths,
            request.options.spatial_error,
            request.options.maximum_fit_level,
        );
        self.directions.insert(source.region, directions);
        self.requests.push(Request {
            source,
            radius: request.support.radius,
        });
        Ok(())
    }
}

fn selected(
    reader: &profile_input::ProfileReader<'_>,
    selection: &WrappedRegionSelection,
) -> Checked<profile_input::OrientedRegion> {
    let selected = profile_input::ProfileSelection {
        region: selection.region,
        outer: selection.outer.clone(),
        holes: selection.holes.clone(),
    };
    reader
        .select(&selected)
        .map_err(|cause| profile_failure(cause, selection.region))
}
fn profile_failure(
    cause: profile_input::ProfileInputFailure,
    region: SurfaceRegionId,
) -> WrappedNormalFailure {
    error::failure(
        WrappedStage::Input,
        WrappedFailureClass::InvalidInput,
        Some(region),
        error::Cause::Input(cause),
    )
}

fn admit(request: &WrappedNormalRequest) -> Checked<()> {
    admit_support(request.support)?;
    if !request.depths.iter().all(|d| d.is_finite()) || request.depths[0] >= request.depths[1] {
        return Err(error::failure(
            WrappedStage::Admission,
            WrappedFailureClass::InvalidInput,
            None,
            error::Cause::Depths(request.depths),
        ));
    }
    Ok(())
}
fn admit_support(support: crate::surface_sketch::CylinderSupport) -> Checked<()> {
    if support.chart_origin != 0. {
        return Err(error::failure(
            WrappedStage::Admission,
            WrappedFailureClass::Unsupported,
            None,
            error::Cause::Support(support),
        ));
    }
    Ok(())
}

fn physical_depths([first, last]: [f64; 2], normal: NormalSense) -> [f64; 2] {
    match normal {
        NormalSense::Outward => [first, last],
        NormalSense::Inward => [-last, -first],
    }
}
pub(super) fn poll(cancelled: &mut CancelPoll<'_>, stage: WrappedStage) -> Checked<()> {
    if cancelled() {
        return Err(error::cancelled(stage));
    }
    Ok(())
}
