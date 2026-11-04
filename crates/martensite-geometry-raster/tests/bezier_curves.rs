//! Specialized retained curves share independent profiles and native pixel contracts.
mod bezier_api {
    pub(super) use PlanarCurve as Planar;
    pub(super) use SpatialCurve as Spatial;
    use martensite_geometry_raster::CurveSampling;
    pub(super) use martensite_geometry_raster::{
        PlanarBezierDrawable as PlanarCurve, SpatialBezierDrawable as SpatialCurve,
    };
    fn spatial_controls(curve: &SpatialCurve) -> &CurveSampling {
        curve.curve_part().sampling()
    }
    fn planar_controls(curve: &PlanarCurve) -> &CurveSampling {
        curve.curve_part().sampling()
    }
    #[cfg(all(target_os = "linux", feature = "x11-integration-tests"))]
    mod display_api {
        use super::{PlanarCurve, SpatialCurve};
        use martensite_raster_core::{
            RasterFailureContext, RasterViewer, VirtualWindowSystem, drawable::RasterDrawable,
        };
        use std::rc::Rc;
        pub type Viewer = RasterViewer;
        pub fn spatial_handle(curve: Rc<SpatialCurve>) -> Rc<dyn RasterDrawable> {
            curve
        }
        pub fn planar_handle(curve: Rc<PlanarCurve>) -> Rc<dyn RasterDrawable> {
            curve
        }
        pub fn viewer(spatial: bool) -> Viewer {
            let system = VirtualWindowSystem::<RasterFailureContext>::connect_virtual().unwrap();
            let mut viewer = RasterViewer::with_adapter(Some(Rc::new(system)));
            viewer
                .make_view(1, if spatial { "+X+Y" } else { "-2D-" }, 0, 0, 400, 400)
                .unwrap();
            viewer.set_zoom(1, 10.);
            viewer
        }
    }
    #[cfg(all(target_os = "linux", feature = "x11-integration-tests"))]
    pub(super) use display_api::{Viewer, viewer};
    #[cfg(all(target_os = "linux", feature = "x11-integration-tests"))]
    pub(super) type DrawableHandle =
        std::rc::Rc<dyn martensite_raster_core::drawable::RasterDrawable>;
    #[cfg(all(target_os = "linux", feature = "x11-integration-tests"))]
    pub(super) use display_api::{planar_handle as plane_handle, spatial_handle as space_handle};
    #[cfg(all(target_os = "linux", feature = "x11-integration-tests"))]
    pub(super) fn reset_visibility(handle: &DrawableHandle) {
        handle.set_visible(false);
    }
    pub(super) fn settings_space(curve: &Spatial) -> (i32, f64, i32) {
        let controls = spatial_controls(curve);
        (
            controls.discretisation(),
            controls.deflection(),
            controls.draw_mode(),
        )
    }
    pub(super) fn settings_plane(curve: &Planar) -> (i32, f64, i32) {
        let controls = planar_controls(curve);
        (
            controls.discretisation(),
            controls.deflection(),
            controls.draw_mode(),
        )
    }
    #[cfg(all(target_os = "linux", feature = "x11-integration-tests"))]
    pub(super) fn set_plane_mode(curve: &Planar, mode: i32) {
        planar_controls(curve).set_draw_mode(mode);
    }
}

#[path = "bezier_support/mod.rs"]
mod bezier_support;
