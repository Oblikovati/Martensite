//! Constructor defaults, control identity and bilinear values are independent contracts.
use super::{
    geometry,
    surface_api::{self, Look, Surface},
};
use martensite_raster_core::{PaletteColor, PaletteKind};
#[test]
fn surface_defaults_and_signed_counts_preserve_knot_switch_state() {
    let surface = Surface::new(geometry::fixture(1));
    assert_eq!(surface_api::settings(&surface), (16, 0.01, 1));
    assert_eq!(surface.nb_isos(), (1, 1));
    surface_api::set_knots(&surface, (4, 5));
    assert!(surface_api::knots(&surface));
    surface.show_isos(-2, -3);
    assert_eq!(surface.nb_isos(), (2, 3));
    assert!(!surface_api::knots(&surface));
    surface_api::set_knots(&surface, (4, 5));
    surface.clear_isos();
    assert_eq!(surface.nb_isos(), (0, 0));
    assert!(!surface_api::knots(&surface));
    assert_send::<Surface>();
}
#[test]
fn surface_geometry_is_shared_live_and_bilinear_midpoint_is_closed_form() {
    let geometry = geometry::fixture(4);
    let surface = Surface::with_look(geometry.clone(), look(), (30, 0.01, 0));
    let returned = surface.surface();
    assert!(std::ptr::eq(
        geometry.get().unwrap(),
        returned.get().unwrap()
    ));
    let point = geometry.get().unwrap().d0(0.5, 0.5).unwrap();
    assert_eq!((point.x(), point.y(), point.z()), (0., 0., 0.75));
    assert_eq!(surface.nb_isos(), (1, 1));
}
pub fn look() -> Look {
    Look {
        nb_u_isos: -1,
        nb_v_isos: -1,
        bounds_color: PaletteColor::new(PaletteKind::Jaune),
        isos_color: PaletteColor::new(PaletteKind::Bleu),
    }
}
fn assert_send<T: Send>() {}
