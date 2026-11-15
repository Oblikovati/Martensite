//! Independent replay controls isolate viewport orientation from provoking-vertex setup.
use martensite_vulkan::{
    RenderCoordinateConvention as Frame, RenderPipelinePolicies, RenderRasterPolicy as Raster,
};
use std::{ffi::OsStr, io};

pub(super) const FRAME_SELECTION: &str = "MARTENSITE_CASE_COORDINATE_CONVENTION";
pub(super) const RASTER_SELECTION: &str = "MARTENSITE_CASE_RASTER_POLICY";

pub(super) fn selected(
    combined: bool,
    frame: Option<&OsStr>,
    raster: Option<&OsStr>,
) -> io::Result<(RenderPipelinePolicies, Frame)> {
    let (mut policies, default_frame) = super::run_coordinate_policies(combined);
    let coordinates = selected_frame(frame, default_frame)?;
    policies.raster = selected_raster(raster, policies.raster)?;
    Ok((policies, coordinates))
}

fn selected_frame(value: Option<&OsStr>, default: Frame) -> io::Result<Frame> {
    match value {
        None => Ok(default),
        Some(v) if v == "top-left-logical-v2" => Ok(Frame::TopLeftLogicalV2),
        Some(v) if v == "legacy-bottom-up-v1" => Ok(Frame::LegacyBottomUpV1),
        actual => Err(invalid(
            FRAME_SELECTION,
            actual,
            "top-left-logical-v2 or legacy-bottom-up-v1",
        )),
    }
}

fn selected_raster(value: Option<&OsStr>, default: Raster) -> io::Result<Raster> {
    match value {
        None => Ok(default),
        Some(v) if v == "legacy-first-v1" => Ok(Raster::LegacyFirstV1),
        Some(v) if v == "last-provoking-vertex-v2" => Ok(Raster::LastProvokingVertexV2),
        actual => Err(invalid(
            RASTER_SELECTION,
            actual,
            "legacy-first-v1 or last-provoking-vertex-v2",
        )),
    }
}

fn invalid(name: &str, value: Option<&OsStr>, expected: &str) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidInput,
        format!("{name}={value:?}; expected unset or {expected}"),
    )
}

pub(super) fn from_environment() -> io::Result<(RenderPipelinePolicies, Frame)> {
    let frame = std::env::var_os(FRAME_SELECTION);
    let raster = std::env::var_os(RASTER_SELECTION);
    let selected = selected(
        std::env::var_os(super::BOTTOM_UP_CONTROL).is_some(),
        frame.as_deref(),
        raster.as_deref(),
    )?;
    if frame.is_some() || raster.is_some() {
        eprintln!(
            "{{\"coordinate_convention\":\"{:?}\",\"raster_policy\":\"{:?}\"}}",
            selected.1, selected.0.raster
        );
    }
    Ok(selected)
}

#[cfg(test)]
support_unit_tests! {
    #[path = "coordinate_policy_selection_tests.rs"]
    mod tests;
}
