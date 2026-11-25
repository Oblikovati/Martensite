//! Normalize physical depth sense exactly and resolve directed profile sources once.
use super::*;
use crate::surface_sketch::NormalSense;
use std::collections::BTreeMap;
pub(in crate::surface_sketch::wrapped_profile) struct Inputs {
    pub requests: Vec<Request>,
    pub directions: BTreeMap<SurfaceRegionId, profile_input::CurveDirections>,
    pub placement: Option<Box<placement::Plan>>,
}
pub(in crate::surface_sketch::wrapped_profile) fn read(
    request: &WrappedNormalRequest,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<Inputs> {
    admit_support(request.support)?;
    let mut inputs = read_canonical_chart(request, cancelled)?;
    inputs.placement = placement::prepare(request, &mut inputs.requests)?;
    Ok(inputs)
}

pub(super) fn read_canonical_chart(
    request: &WrappedNormalRequest,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<Inputs> {
    admit_depths(request.depths)?;
    let depths = physical_depths(request.depths, request.support.normal);
    let mut inputs = Inputs {
        requests: Vec::new(),
        directions: BTreeMap::new(),
        placement: None,
    };
    read_regions(&mut inputs, request, depths, cancelled)?;
    poll(cancelled, WrappedStage::Input)?;
    Ok(inputs)
}
fn read_regions(
    inputs: &mut Inputs,
    request: &WrappedNormalRequest,
    depths: [f64; 2],
    cancelled: &mut CancelPoll<'_>,
) -> Checked<()> {
    operation::source::read(
        &request.profile,
        &request.selections,
        cancelled,
        |selection, reader, cancelled| {
            inputs.push_region(request, selection, depths, reader, cancelled)
        },
    )
}
impl Inputs {
    fn push_region(
        &mut self,
        request: &WrappedNormalRequest,
        selection: &WrappedRegionSelection,
        depths: [f64; 2],
        reader: &profile_input::ProfileReader<'_>,
        cancelled: &mut CancelPoll<'_>,
    ) -> Checked<()> {
        let oriented = selected_with_cancel(reader, selection, cancelled)?;
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

pub(in crate::surface_sketch::wrapped_profile) fn selected(
    reader: &profile_input::ProfileReader<'_>,
    selection: &WrappedRegionSelection,
) -> Checked<profile_input::OrientedRegion> {
    selected_with_cancel(reader, selection, &mut || false)
}

pub(in crate::surface_sketch::wrapped_profile) fn selected_with_cancel(
    reader: &profile_input::ProfileReader<'_>,
    selection: &WrappedRegionSelection,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<profile_input::OrientedRegion> {
    let selected = profile_input::ProfileSelection {
        region: selection.region,
        outer: selection.outer.clone(),
        holes: selection.holes.clone(),
    };
    reader
        .select_with_cancel(&selected, cancelled)
        .map_err(|cause| profile_failure(cause, Some(selection.region)))
}
pub(in crate::surface_sketch::wrapped_profile) fn profile_failure(
    cause: profile_input::ProfileInputFailure,
    region: Option<SurfaceRegionId>,
) -> WrappedNormalFailure {
    error::failure(
        WrappedStage::Input,
        WrappedFailureClass::InvalidInput,
        region,
        error::Cause::Input(cause),
    )
}

fn admit_depths(depths: [f64; 2]) -> Checked<()> {
    if !depths.iter().all(|d| d.is_finite()) || depths[0] >= depths[1] {
        return Err(error::failure(
            WrappedStage::Admission,
            WrappedFailureClass::InvalidInput,
            None,
            error::Cause::Depths(depths),
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
pub(in crate::surface_sketch::wrapped_profile) fn poll(
    cancelled: &mut CancelPoll<'_>,
    stage: WrappedStage,
) -> Checked<()> {
    if cancelled() {
        return Err(error::cancelled(stage));
    }
    Ok(())
}
