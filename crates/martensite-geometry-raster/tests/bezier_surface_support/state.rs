//! Closed-form geometry and constructor contracts do not depend on captures.
use super::{
    fixtures,
    surface_api::{self, Surface},
};
#[test]
fn bezier_surface_default_controls_and_geometry_identity_are_retained() {
    let geometry = fixtures::geometry(1);
    let surface = Surface::new(geometry.clone());
    assert_eq!(surface_api::settings(&surface), (30, 0.05, 0));
    assert_eq!(surface.surface_part().nb_isos(), (1, 1));
    assert!(surface.poles().shown());
    assert_geometry_identity(&surface, &geometry);
    assert_midpoint(&surface);
    assert_send::<Surface>();
}
fn assert_geometry_identity(
    surface: &Surface,
    geometry: &martensite_tkernel::standard::Handle<martensite_tkg3d::geom::GeomBezierSurface>,
) {
    let returned = surface
        .surface_part()
        .surface()
        .cast::<martensite_tkg3d::geom::GeomBezierSurface>();
    assert!(std::ptr::eq(
        geometry.get().unwrap(),
        returned.get().unwrap()
    ));
}
fn assert_midpoint(surface: &Surface) {
    let point = surface
        .surface_part()
        .surface()
        .get()
        .unwrap()
        .d0(0.5, 0.5)
        .unwrap();
    assert_eq!((point.x(), point.y(), point.z()), (0., 2., 1.));
}
fn assert_send<T: Send>() {}
