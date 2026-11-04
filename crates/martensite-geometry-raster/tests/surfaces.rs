//! Surface facade, actual style cells and native images pinned before extraction.
mod surface_api {
    pub(super) use martensite_geometry_raster::{SurfaceDrawable as Surface, SurfaceLook as Look};
    pub(super) fn controls(surface: &Surface) -> &martensite_geometry_raster::CurveSampling {
        surface.sampling()
    }
    pub(super) fn settings(surface: &Surface) -> (i32, f64, i32) {
        let c = controls(surface);
        (c.discretisation(), c.deflection(), c.draw_mode())
    }
    pub(super) fn set_knots(surface: &Surface, counts: (i32, i32)) {
        surface.style().set_knots_isos(counts);
    }
    pub(super) fn knots(surface: &Surface) -> bool {
        surface.style().knots_isos()
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
#[path = "surface_support/mod.rs"]
mod surface_support;

#[test]
fn surface_owned_control_types_keep_explicit_thread_and_profile_contracts() {
    use martensite_geometry_raster::{
        SurfaceDrawable, SurfaceLook, SurfaceRendering, SurfaceStyle,
    };
    let style = SurfaceStyle::new(surface_support::state::look());
    let profile = style.look();
    assert_eq!((profile.nb_u_isos, profile.nb_v_isos), (1, 1));
    assert_eq!(profile.bounds_color, style.bounds_color());
    assert_eq!(profile.isos_color, style.isos_color());
    assert_send::<SurfaceRendering>();
    assert_send::<SurfaceStyle>();
    assert_send::<SurfaceDrawable>();
    assert_send_sync::<SurfaceLook>();
}
fn assert_send<T: Send>() {}
fn assert_send_sync<T: Send + Sync>() {}
