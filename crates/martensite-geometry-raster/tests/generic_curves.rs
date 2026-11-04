//! Owned generic curves run the exact pre-move independent geometry and image contracts.
mod curve_api {
    use martensite_geometry_raster::CurveSampling;
    pub(super) use martensite_geometry_raster::{
        PlanarCurveDrawable as PlanarCurve, SpatialCurveDrawable as SpatialCurve,
    };
    pub(super) use martensite_raster_core::drawable::RasterDrawable as CurveDrawableContract;
    pub(super) fn spatial_controls(curve: &SpatialCurve) -> &CurveSampling {
        curve.sampling()
    }
    pub(super) fn planar_controls(curve: &PlanarCurve) -> &CurveSampling {
        curve.sampling()
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
    pub(super) use display_api::{Viewer, planar_handle, spatial_handle, viewer};
}
#[path = "generic_curve_support/mod.rs"]
mod generic_curve_support;
