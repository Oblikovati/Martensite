//! Sampling profiles and linear interpolation do not derive expectations from pixels.
use super::{
    fixtures,
    spline_api::{self, Planar, Spatial},
};
use martensite_tkg3d::geom::GeomCurve;
#[test]
fn spline_defaults_preserve_dimension_profiles_and_closed_form_values() {
    let space = Spatial::new(fixtures::space(0));
    let plane = Planar::new(fixtures::plane(0));
    assert_eq!(spline_api::settings_space(&space), (16, 0.05, 1));
    assert_eq!(spline_api::settings_plane(&plane), (100, 0.01, 0));
    assert!(space.poles().shown() && space.knots().shown());
    assert!(plane.poles().shown() && plane.knots().shown());
    let point = space.curve_part().curve().get().unwrap().d0(0.5).unwrap();
    assert_eq!((point.x(), point.y(), point.z()), (1., 2., 0.));
    let point = plane.curve_part().curve().get().unwrap().d0(0.5).unwrap();
    assert_eq!((point.x(), point.y()), (1., 2.));
    assert_send::<Spatial>();
    assert_send::<Planar>();
}
fn assert_send<T: Send>() {}
#[test]
fn spline_specialized_geometry_preserves_live_handle_identity() {
    let geometry = fixtures::space(1);
    let curve = Spatial::with_style(geometry.clone(), &fixtures::look(), (30, 0.01, 0));
    let returned = curve
        .curve_part()
        .curve()
        .cast::<martensite_tkg3d::geom::GeomBSplineCurve>();
    assert!(std::ptr::eq(
        geometry.get().unwrap(),
        returned.get().unwrap()
    ));
    planar_identity();
}

#[test]
fn unclamped_knots_and_periodic_geometry_distinguish_normal_and_explicit_domains() {
    let handle = fixtures::space(2);
    let curve = handle.get().unwrap();
    assert_eq!(curve.nb_knots(), 8);
    assert_eq!(
        (
            curve.first_u_knot_index().unwrap(),
            curve.last_u_knot_index().unwrap()
        ),
        (3, 6)
    );
    let handle = fixtures::space(3);
    let curve = handle.get().unwrap();
    assert!(curve.is_periodic());
    assert_eq!(curve.nb_poles(), 4);
}

fn planar_identity() {
    let geometry = fixtures::plane(1);
    let curve = Planar::with_style(geometry.clone(), &fixtures::look(), 30);
    let returned = curve
        .curve_part()
        .curve()
        .cast::<martensite_tkg2d::geom2d::Geom2dBSplineCurve>();
    assert!(std::ptr::eq(
        geometry.get().unwrap(),
        returned.get().unwrap()
    ));
}
