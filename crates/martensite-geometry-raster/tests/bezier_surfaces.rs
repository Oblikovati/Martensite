//! Surface facade, actual style cells and native images pinned before extraction.
mod surface_api {
    pub(super) use martensite_geometry_raster::BezierSurfaceDrawable as Surface;
    #[cfg(all(target_os = "linux", feature = "x11-integration-tests"))]
    pub(super) use martensite_geometry_raster::SurfaceLook as Look;
    pub(super) fn controls(surface: &Surface) -> &martensite_geometry_raster::CurveSampling {
        surface.surface_part().sampling()
    }
    pub(super) fn settings(surface: &Surface) -> (i32, f64, i32) {
        let c = controls(surface);
        (c.discretisation(), c.deflection(), c.draw_mode())
    }
    #[cfg(all(target_os = "linux", feature = "x11-integration-tests"))]
    mod native_api {
        use super::Surface;
        use martensite_raster_core::{
            RasterFailureContext, RasterViewer, VirtualWindowSystem, drawable::RasterDrawable,
        };
        use std::rc::Rc;
        pub type Viewer = RasterViewer;
        pub type DrawableHandle = Rc<dyn RasterDrawable>;
        pub fn handle(surface: Rc<Surface>) -> DrawableHandle {
            surface
        }
        pub fn reset_visibility(handle: &DrawableHandle) {
            handle.set_visible(false);
        }
        pub fn viewer(zoom: f64) -> Viewer {
            let system = VirtualWindowSystem::<RasterFailureContext>::connect_virtual().unwrap();
            let mut viewer = RasterViewer::with_adapter(Some(Rc::new(system)));
            viewer.make_view(1, "+X+Y", 0, 0, 400, 400).unwrap();
            viewer.set_zoom(1, zoom);
            viewer
        }
    }
    #[cfg(all(target_os = "linux", feature = "x11-integration-tests"))]
    pub(super) use native_api::{Viewer, handle, reset_visibility, viewer};
}
#[path = "bezier_surface_support/mod.rs"]
mod bezier_surface_support;
