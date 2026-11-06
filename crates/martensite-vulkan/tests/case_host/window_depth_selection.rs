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
mod tests {
    use super::*;
    #[test]
    fn default_and_explicit_operation_keep_projection_and_units_independent() {
        assert_eq!(selected(None).unwrap(), Behavior::HardwareV1);
        assert_eq!(
            selected(Some(OsStr::new("hardware-v1"))).unwrap(),
            Behavior::HardwareV1
        );
        let selected_v2 = selected(Some(OsStr::new("grid12-over8-v2"))).unwrap();
        let old = RenderBehaviorConfiguration::default()
            .with_projection(martensite_vulkan::RenderProjectionBehavior::FactorizedTrianglesV2)
            .with_depth_bias(martensite_vulkan::RenderDepthBiasBehavior::Native24UnitsV2);
        assert_eq!(
            capture_configuration(old, Some(OsStr::new("grid12-over8-v2"))).unwrap(),
            old.with_window_depth(selected_v2)
        );
        assert!(metadata(selected_v2).contains("\"reference_fractional_bits\":12"));
        assert!(metadata(Behavior::HardwareV1).contains("\"behavior_version\":1"));
    }
    #[test]
    fn float64_metadata_preserves_case_arithmetic_and_raw_clip_contract() {
        let float64 = selected(Some(OsStr::new("float64-grid12-over8-v5"))).unwrap();
        assert!(metadata(float64).contains("Float64ReferenceWindowDepthV5"));
        assert!(metadata(float64).contains("\"behavior_version\":5"));
        assert_eq!(
            metadata(float64),
            concat!(
                "{\"window_depth_behavior\":\"Float64ReferenceWindowDepthV5\",\"behavior_version\":5,",
                "\"reference_fractional_bits\":12,\"raster_fractional_bits\":8,\"grid_rounding\":\"nearest-even\",",
                "\"arithmetic\":\"ordered-binary64-no-contraction; float-controls-unspecified\",",
                "\"input_contract\":\"actual-binary32-raw-GL-clip-before-remap\"}"
            )
        );
    }
    #[test]
    fn malformed_or_case_specific_names_are_not_fallbacks() {
        for value in [
            "",
            "true",
            "amd",
            "dragg",
            "grid8-over12-v2",
            "grid12-over8-v1",
        ] {
            let error = selected(Some(OsStr::new(value))).unwrap_err();
            assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
            assert!(error.to_string().contains(value));
        }
    }
    #[test]
    fn direct_fragment_depth_is_an_explicit_successor_not_an_alias_for_geometry_replay() {
        let v2 = selected(Some(OsStr::new("grid12-over8-v2"))).unwrap();
        let v3 = selected(Some(OsStr::new("grid12-over8-v3"))).unwrap();
        assert_ne!(v2, v3);
        assert_eq!(v3.behavior_version(), 3);
        let v4 = selected(Some(OsStr::new("clipped-fan-grid8-v4"))).unwrap();
        assert_eq!(v4.behavior_version(), 4);
        assert!(metadata(v4).contains("unsnapped-framebuffer-origin"));
        assert!(metadata(v3).contains("ReferenceWindowDepthV3"));
        assert!(metadata(v3).contains("\"behavior_version\":3"));
    }
    #[test]
    fn shared_actual_incidence_is_version_six_and_never_relabels_v4_replay() {
        let shared = selected(Some(OsStr::new("clipped-incidence-grid12-v6"))).unwrap();
        let old = selected(Some(OsStr::new("clipped-fan-grid8-v4"))).unwrap();
        assert_eq!(shared.behavior_version(), 6);
        assert_eq!(old.behavior_version(), 4);
        assert!(metadata(shared).contains("actual-vs-clip-distance"));
        assert!(selected(Some(OsStr::new("clipped-incidence-grid8-v6"))).is_err());
        assert_ne!(shared, old);
    }
}
