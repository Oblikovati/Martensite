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
    for selection in &request.selections {
        poll(cancelled, WrappedStage::Input)?;
        inputs.push_region(request, selection, depths)?;
    }
    poll(cancelled, WrappedStage::Input)?;
    inputs.placement = placement::prepare(request, &mut inputs.requests)
        .map_err(|failure| failure_projection::selected(failure, request))?;
    Ok(inputs)
}
impl Inputs {
    fn push_region(
        &mut self,
        request: &WrappedNormalRequest,
        selection: &WrappedRegionSelection,
        depths: [f64; 2],
    ) -> Checked<()> {
        let oriented = selected(&request.profile, selection)?;
        self.requests
            .push(geometry(request, oriented.region, oriented.loops, depths));
        self.directions.insert(oriented.region, oriented.directions);
        Ok(())
    }
}

fn selected(
    profile: &martensite_tkbo::profile_extrusion::SketchProfile,
    selection: &WrappedRegionSelection,
) -> Checked<profile_input::OrientedRegion> {
    let selected = profile_input::ProfileSelection {
        region: selection.region,
        outer: selection.outer.clone(),
        holes: selection.holes.clone(),
    };
    profile_input::from_profile(profile, &selected).map_err(|cause| {
        error::failure(
            WrappedStage::Input,
            WrappedFailureClass::InvalidInput,
            Some(selection.region),
            error::Cause::Input(cause),
        )
    })
}
fn geometry(
    request: &WrappedNormalRequest,
    region: SurfaceRegionId,
    loops: Vec<Vec<AuthoredPiece>>,
    depths: [f64; 2],
) -> Request {
    Request {
        region,
        loops,
        radius: request.support.radius,
        depths,
        tolerance: request.options.spatial_error,
        max_fit_level: request.options.maximum_fit_level,
    }
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
