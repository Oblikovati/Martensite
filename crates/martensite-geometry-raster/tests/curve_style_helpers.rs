//! Shared style contracts execute without a command or source-renderer dependency.
mod style_api {
    pub(super) use martensite_geometry_raster::curve_properties::{
        CurvatureDisplay, CurvatureProps, finite_range,
    };
    pub(super) use martensite_geometry_raster::spline_style::draw_pole_net;
    #[cfg(all(target_os = "linux", feature = "x11-integration-tests"))]
    pub(super) use martensite_geometry_raster::spline_style::{
        draw_polygon, find_index, find_net_index, scaled_target,
    };
    pub(super) use martensite_geometry_raster::{CurveLook, CurveStyle, KnotsStyle, PolesStyle};
}
#[path = "style_support/mod.rs"]
mod style_support;
