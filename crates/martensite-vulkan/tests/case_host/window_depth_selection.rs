//! A worker selects an owned window-depth operation without changing authored commands.
use martensite_vulkan::{
    RenderBehaviorConfiguration, RenderViewportGridPrecision as Grid,
    RenderWindowDepthBehavior as Behavior,
};
use std::{ffi::OsStr, io};
pub(crate) const SELECTION: &str = "MARTENSITE_CASE_WINDOW_DEPTH";

pub(super) fn selected(value: Option<&OsStr>) -> io::Result<Behavior> {
    match value {
        None => Ok(Behavior::HardwareV1),
        Some(value) if value == "hardware-v1" => Ok(Behavior::HardwareV1),
        Some(value) if value == "grid12-over8-v2" => Ok(Behavior::UnsnappedWindowDepthV2 {
            reference_grid: Grid::Fractional12,
            raster_grid: Grid::Fractional8,
        }),
        Some(value) if value == "grid12-over8-v3" => Ok(Behavior::ReferenceWindowDepthV3 {
            reference_grid: Grid::Fractional12,
            raster_grid: Grid::Fractional8,
        }),
        Some(value) if value == "float64-grid12-over8-v5" => {
            Ok(Behavior::Float64ReferenceWindowDepthV5 {
                reference_grid: Grid::Fractional12,
                raster_grid: Grid::Fractional8,
            })
        }
        Some(value) if value == "clipped-fan-grid8-v4" => Ok(Behavior::ClippedFanFramebufferV4 {
            fan_grid: Grid::Fractional8,
            raster_grid: Grid::Fractional8,
        }),
        Some(value) if value == "clipped-incidence-grid12-v6" => {
            Ok(Behavior::ClippedIncidenceFramebufferV6 {
                fan_grid: Grid::Fractional12,
                raster_grid: Grid::Fractional8,
            })
        }
        actual => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "{SELECTION}={actual:?}; expected unset, hardware-v1, grid12-over8-v2, grid12-over8-v3, clipped-fan-grid8-v4, float64-grid12-over8-v5 or clipped-incidence-grid12-v6"
            ),
        )),
    }
}

pub(crate) fn capture_configuration(
    configuration: RenderBehaviorConfiguration,
    value: Option<&OsStr>,
) -> io::Result<RenderBehaviorConfiguration> {
    Ok(configuration.with_window_depth(selected(value)?))
}

pub(super) fn metadata(behavior: Behavior) -> String {
    match behavior {
        Behavior::HardwareV1 => {
            "{\"window_depth_behavior\":\"HardwareV1\",\"behavior_version\":1}".into()
        }
        Behavior::Float64ReferenceWindowDepthV5 {
            reference_grid,
            raster_grid,
        } => float64_metadata(reference_grid, raster_grid),
        Behavior::ClippedFanFramebufferV4 {
            fan_grid,
            raster_grid,
        } => format!(
            "{{\"window_depth_behavior\":\"ClippedFanFramebufferV4\",\"behavior_version\":4,\"fan_fractional_bits\":{},\"raster_fractional_bits\":{},\"plane_coordinates\":\"unsnapped-framebuffer-origin\",\"selected_family_clip_stage\":\"RasterClipDistanceV2\",\"unselected_family_clip_stage\":\"requested-policy\"}}",
            fan_grid.fractional_bits(),
            raster_grid.fractional_bits(),
        ),
        Behavior::ClippedIncidenceFramebufferV6 {
            fan_grid,
            raster_grid,
        } => format!(
            "{{\"window_depth_behavior\":\"ClippedIncidenceFramebufferV6\",\"behavior_version\":6,\"fan_fractional_bits\":{},\"raster_fractional_bits\":{},\"plane_coordinates\":\"unsnapped-framebuffer-origin\",\"incidence_source\":\"actual-vs-clip-distance\",\"selected_family_clip_stage\":\"RasterClipDistanceV2\",\"unselected_family_clip_stage\":\"requested-policy\"}}",
            fan_grid.fractional_bits(),
            raster_grid.fractional_bits(),
        ),
        Behavior::UnsnappedWindowDepthV2 {
            reference_grid,
            raster_grid,
        }
        | Behavior::ReferenceWindowDepthV3 {
            reference_grid,
            raster_grid,
        } => format!(
            "{{\"window_depth_behavior\":\"{}\",\"behavior_version\":{},\"reference_fractional_bits\":{},\"raster_fractional_bits\":{},\"rounding\":\"nearest-even\"}}",
            if behavior.behavior_version() == 3 {
                "ReferenceWindowDepthV3"
            } else {
                "UnsnappedWindowDepthV2"
            },
            behavior.behavior_version(),
            reference_grid.fractional_bits(),
            raster_grid.fractional_bits()
        ),
    }
}

fn float64_metadata(reference: Grid, raster: Grid) -> String {
    format!(
        "{{\"window_depth_behavior\":\"Float64ReferenceWindowDepthV5\",\"behavior_version\":5,\"reference_fractional_bits\":{},\"raster_fractional_bits\":{},\"grid_rounding\":\"nearest-even\",\"arithmetic\":\"ordered-binary64-no-contraction; float-controls-unspecified\",\"input_contract\":\"actual-binary32-raw-GL-clip-before-remap\"}}",
        reference.fractional_bits(),
        raster.fractional_bits()
    )
}

#[cfg(test)]
support_unit_tests! {
    #[path = "window_depth_selection_tests.rs"]
    mod tests;
}
