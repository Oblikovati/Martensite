//! Virtual bound queries must run without a representation lock across callbacks.
#[path = "curve3d_copy_callbacks/space_curve.rs"]
pub(super) mod space_curve;
use martensite_tkbrep::brep::{BRepCurve3D, BRepCurveRepresentation, BRepGCurve};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomCurve, GeomLine};
use martensite_tkmath::{
    gp::{Axis1, Point3},
    toploc::Location,
};
use space_curve::RebindingSpaceCurve;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

fn binding_fixture(
    replacement: Handle<dyn GeomCurve>,
) -> (Handle<BRepCurve3D>, Arc<AtomicUsize>, Handle<dyn GeomCurve>) {
    let curve = Handle::new(RebindingSpaceCurve::new(
        Arc::new(AtomicUsize::new(0)),
        Arc::new(|u| Ok(Point3::new(u, 0., 0.))),
    ));
    let projected: Handle<dyn GeomCurve> = curve.cast();
    let source = Handle::new(BRepCurve3D::new(&projected, &Location::default()));
    source.set_range(2., 4.).unwrap();
    let count = Arc::new(AtomicUsize::new(0));
    let owner = source.downgrade();
    let hook_count = count.clone();
    let new_curve = replacement.clone();
    curve.set_parameter_hook(Arc::new(move || {
        hook_count.fetch_add(1, Ordering::SeqCst);
        owner.upgrade().set_curve3d(&new_curve).unwrap();
    }));
    (source, count, replacement)
}

#[test]
fn copy_rebinds_primary_curve_during_parameter_query_without_deadlock() {
    let replacement = Handle::new(GeomLine::new(Axis1::default())).cast();
    let (source, count, replacement) = binding_fixture(replacement);
    let copied: Handle<BRepCurve3D> = source.copy().unwrap().cast();
    assert_eq!(count.load(Ordering::SeqCst), 1);
    assert_eq!(copied.curve3d().unwrap(), replacement);
    assert_eq!(copied.range(), (2., 4.));
}

fn translated_location() -> Location {
    let mut transform = martensite_tkmath::gp::Transform3::default();
    transform.set_translation(martensite_tkmath::gp::Vector3::new(8., -4., 2.));
    Location::new(transform)
}

fn location_fixture() -> (Handle<BRepCurve3D>, Arc<AtomicUsize>) {
    let curve = Handle::new(RebindingSpaceCurve::new(
        Arc::new(AtomicUsize::new(0)),
        Arc::new(|u| Ok(Point3::new(u, 0., 0.))),
    ));
    let source = Handle::new(BRepCurve3D::new(&curve.cast(), &Location::default()));
    let owner = source.downgrade();
    let count = Arc::new(AtomicUsize::new(0));
    let queries = count.clone();
    let location = translated_location();
    curve.set_parameter_hook(Arc::new(move || {
        queries.fetch_add(1, Ordering::SeqCst);
        owner.upgrade().set_location(&location);
    }));
    (source, count)
}

#[test]
fn copy_consumes_live_source_location_after_parameter_queries() {
    let (source, count) = location_fixture();
    let copy: Handle<BRepCurve3D> = source.copy().unwrap().cast();
    assert_eq!(count.load(Ordering::SeqCst), 2);
    assert_eq!(
        source
            .location()
            .transformation()
            .transforms_coordinates(0., 0., 0.),
        (8., -4., 2.)
    );
    assert_eq!(copy.location(), source.location());
}

#[test]
fn copy_accepts_curve_cleared_during_parameter_query() {
    let (source, count, _) = binding_fixture(Handle::default());
    let copied: Handle<BRepCurve3D> = source.copy().unwrap().cast();
    assert_eq!(count.load(Ordering::SeqCst), 1);
    assert!(source.curve3d().unwrap().is_null());
    assert!(copied.curve3d().unwrap().is_null());
    assert_eq!(copied.range(), (2., 4.));
}

#[test]
fn null_constructor_range_and_curve_setter_keep_passive_storage_contract() {
    let value = BRepCurve3D::new(&Handle::default(), &Location::default());
    assert_eq!(value.range(), (-f64::MAX, f64::MAX));
    let line: Handle<dyn GeomCurve> = Handle::new(GeomLine::new(Axis1::default())).cast();
    value.set_curve3d(&line).unwrap();
    assert_eq!(value.range(), (-f64::MAX, f64::MAX));
    assert_eq!(value.curve3d().unwrap(), line);
}

#[test]
fn curve_value_does_not_apply_representation_location() {
    let line: Handle<dyn GeomCurve> = Handle::new(GeomLine::from_point(
        Point3::new(1., 2., 3.),
        martensite_tkmath::gp::Direction3::new(1., 0., 0.).unwrap(),
    ))
    .cast();
    let value = BRepCurve3D::new(&line, &translated_location());
    assert_eq!(value.d0(4.).unwrap(), Point3::new(5., 2., 3.));
}

#[test]
fn public_space_constructor_keeps_first_then_last_query_order() {
    let curve = Handle::new(RebindingSpaceCurve::new(
        Arc::new(AtomicUsize::new(0)),
        Arc::new(|u| Ok(Point3::new(u, 0., 0.))),
    ));
    BRepCurve3D::new(&curve.cast(), &Location::default());
    assert_eq!(curve.parameter_history(), ["first", "last"]);
}

#[test]
fn dump_json_callback_can_replace_curve_without_deadlock() {
    let replacement = Handle::new(GeomLine::new(Axis1::default())).cast();
    let (source, _, replacement) = binding_fixture(replacement);
    let curve: Handle<RebindingSpaceCurve> = source.curve3d().unwrap().cast();
    let owner = source.downgrade();
    let count = Arc::new(AtomicUsize::new(0));
    let calls = count.clone();
    let new_curve = replacement.clone();
    curve.set_dump_hook(Arc::new(move || {
        calls.fetch_add(1, Ordering::SeqCst);
        owner.upgrade().set_curve3d(&new_curve).unwrap();
    }));
    source
        .dump_json(
            &mut martensite_tkernel::standard::dump::DumpStream::default(),
            -1,
        )
        .unwrap();
    assert_eq!(count.load(Ordering::SeqCst), 1);
    assert_eq!(source.curve3d().unwrap(), replacement);
}
