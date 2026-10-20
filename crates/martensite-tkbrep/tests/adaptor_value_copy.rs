//! Adaptor value copies retain cache owners while shallow copies request fresh caches.
use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_adaptor::{BRepAdaptorCurve, BRepAdaptorSurface},
    topods::{Edge, Face},
};
use martensite_tkernel::{
    ncollection::{Array1, Array2},
    standard::Handle,
};
use martensite_tkg3d::{
    adaptor3d::{Adaptor3dCurve, Adaptor3dSurface},
    geom::{GeomBezierCurve, GeomBezierSurface, GeomCurve, GeomPlane, GeomSurface},
};
use martensite_tkmath::gp::{Axis3, Point3};

fn curve_source() -> (Handle<GeomBezierCurve>, BRepAdaptorCurve) {
    let points = [Point3::default(), Point3::new(2., 0., 0.)];
    let geometry = Handle::new(GeomBezierCurve::new(&Array1::from_shared(&points, 1)).unwrap());
    geometry.clear_eval_representation();
    let builder = BRepBuilder::default();
    let mut edge = Edge::default();
    builder
        .make_edge_with_curve(&mut edge, &geometry.cast::<dyn GeomCurve>(), 1e-7)
        .unwrap();
    builder.range(&edge, 0., 1., true).unwrap();
    let adaptor = BRepAdaptorCurve::with_edge(&edge).unwrap();
    (geometry, adaptor)
}

fn patch_poles() -> Array2<'static, Point3> {
    let mut poles = Array2::with_bounds(1, 2, 1, 2).unwrap();
    for u in 1..=2 {
        for v in 1..=2 {
            poles
                .set_value(
                    u,
                    v,
                    Point3::new(2. * f64::from(u - 1), 2. * f64::from(v - 1), 0.),
                )
                .unwrap();
        }
    }
    poles
}

fn surface_source() -> (Handle<GeomBezierSurface>, BRepAdaptorSurface) {
    let geometry = Handle::new(GeomBezierSurface::new(&patch_poles()).unwrap());
    geometry.clear_eval_representation();
    let mut face = Face::default();
    BRepBuilder::default()
        .make_face_on_surface(&mut face, &geometry.cast::<dyn GeomSurface>(), 1e-7)
        .unwrap();
    let adaptor = BRepAdaptorSurface::with_face(&face, false).unwrap();
    (geometry, adaptor)
}

#[test]
fn curve_value_copy_retains_warm_values_and_shallow_copy_rebuilds() {
    let (geometry, source) = curve_source();
    let expected = Point3::new(0.5, 0., 0.);
    assert_eq!(source.value(0.25).unwrap(), expected);
    let copied = source.clone();
    geometry.set_pole(1, Point3::new(0., 0., 4.)).unwrap();
    geometry.set_pole(2, Point3::new(2., 0., 4.)).unwrap();
    assert_eq!(copied.value(0.25).unwrap(), expected);
    assert_eq!(source.value(0.25).unwrap(), expected);
    let shallow = source.shallow_copy().unwrap();
    assert_eq!(shallow.value(0.25).unwrap(), Point3::new(0.5, 0., 4.));
    copied.reset();
    assert!(copied.edge().is_null());
    assert!(!source.edge().is_null());
    assert_eq!(source.value(0.25).unwrap(), expected);
}

fn raise_patch(geometry: &GeomBezierSurface) {
    for u in 1..=2 {
        for v in 1..=2 {
            geometry
                .set_pole(
                    u,
                    v,
                    Point3::new(2. * f64::from(u - 1), 2. * f64::from(v - 1), 4.),
                )
                .unwrap();
        }
    }
}

#[test]
fn surface_value_copy_retains_warm_values_and_shallow_copy_rebuilds() {
    let (geometry, source) = surface_source();
    let expected = Point3::new(0.5, 1., 0.);
    assert_eq!(source.value(0.25, 0.5).unwrap(), expected);
    let copied = source.clone();
    raise_patch(&geometry);
    assert_eq!(copied.value(0.25, 0.5).unwrap(), expected);
    assert_eq!(source.value(0.25, 0.5).unwrap(), expected);
    let shallow = source.shallow_copy().unwrap();
    assert_eq!(shallow.value(0.25, 0.5).unwrap(), Point3::new(0.5, 1., 4.));
    assert!(copied.face().is_same(&source.face()));
    let plane: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(Axis3::default())).cast();
    copied.transformed_surface().surface().load(&plane).unwrap();
    assert_eq!(copied.value(0.25, 0.5).unwrap(), Point3::new(0.25, 0.5, 0.));
    assert_eq!(source.value(0.25, 0.5).unwrap(), expected);
}
