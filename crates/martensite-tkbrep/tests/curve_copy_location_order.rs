//! Copy's constructor consumes its source location after parameter queries.
use super::curve_update_rebinding::rebinding_curve::RebindingParameterCurve;
use martensite_tkbrep::brep::{
    BRepCurveOnClosedSurface, BRepCurveOnSurface, BRepCurveRepresentation, BRepGCurve,
};
use martensite_tkernel::standard::{Handle, RuntimeTyped, WeakHandle};
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dLine};
use martensite_tkmath::{
    geom_abs::Shape,
    gp::{Axis2d, Point2, Transform3, Vector3},
    toploc::Location,
};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

fn changed_location() -> Location {
    let mut transform = Transform3::default();
    transform.set_translation(Vector3::new(8., -4., 2.));
    Location::new(transform)
}

fn surface_fixture() -> (Handle<BRepCurveOnSurface>, Arc<AtomicUsize>) {
    let evaluations = Arc::new(AtomicUsize::new(0));
    let curve = Handle::new(RebindingParameterCurve::new(
        evaluations,
        Arc::new(|u| Ok(Point2::new(u, 0.))),
    ));
    let projected: Handle<dyn Geom2dCurve> = curve.cast();
    let source = Handle::new(
        BRepCurveOnSurface::new(&projected, &Handle::default(), &Location::default()).unwrap(),
    );
    source.set_range(2., 4.).unwrap();
    let count = Arc::new(AtomicUsize::new(0));
    install_location_hook(&curve, source.downgrade(), count.clone());
    (source, count)
}

fn install_location_hook<T: BRepCurveRepresentation + RuntimeTyped>(
    curve: &RebindingParameterCurve,
    owner: WeakHandle<T>,
    count: Arc<AtomicUsize>,
) {
    let location = changed_location();
    curve.set_parameter_hook(Arc::new(move || {
        count.fetch_add(1, Ordering::SeqCst);
        owner.upgrade().set_location(&location);
    }));
}

#[test]
fn surface_copy_location_observes_parameter_query_callbacks() {
    let (source, count) = surface_fixture();
    let copied: Handle<BRepCurveOnSurface> = source.copy().unwrap().cast();
    assert_eq!(count.load(Ordering::SeqCst), 2);
    assert_eq!(
        source
            .location()
            .transformation()
            .transforms_coordinates(0., 0., 0.),
        (8., -4., 2.)
    );
    assert_eq!(copied.location(), source.location());
}

fn seam_from_first(curve: &Handle<dyn Geom2dCurve>) -> Handle<BRepCurveOnClosedSurface> {
    let second: Handle<dyn Geom2dCurve> = Handle::new(Geom2dLine::new(Axis2d::default())).cast();
    Handle::new(
        BRepCurveOnClosedSurface::new(
            curve,
            &second,
            &Handle::default(),
            &Location::default(),
            Shape::C0,
        )
        .unwrap(),
    )
}

fn seam_fixture() -> (Handle<BRepCurveOnClosedSurface>, Arc<AtomicUsize>) {
    let curve = Handle::new(RebindingParameterCurve::new(
        Arc::new(AtomicUsize::new(0)),
        Arc::new(|u| Ok(Point2::new(u, 0.))),
    ));
    let source = seam_from_first(&curve.cast());
    source.set_range(2., 4.).unwrap();
    let count = Arc::new(AtomicUsize::new(0));
    install_location_hook(&curve, source.downgrade(), count.clone());
    (source, count)
}

#[test]
fn seam_copy_location_observes_parameter_query_callbacks() {
    let (source, count) = seam_fixture();
    let copied: Handle<BRepCurveOnClosedSurface> = source.copy().unwrap().cast();
    assert_eq!(count.load(Ordering::SeqCst), 2);
    assert_eq!(
        source
            .location()
            .transformation()
            .transforms_coordinates(0., 0., 0.),
        (8., -4., 2.)
    );
    assert_eq!(copied.location(), source.location());
}

#[test]
fn seam_copy_second_curve_observes_first_curve_parameter_query_callbacks() {
    let (source, count) = seam_fixture();
    let curve: Handle<RebindingParameterCurve> = source.pcurve().unwrap().cast();
    let original_second = source.pcurve2().unwrap();
    let replacement: Handle<dyn Geom2dCurve> =
        Handle::new(Geom2dLine::new(Axis2d::default())).cast();
    let owner = source.downgrade();
    let query_count = count.clone();
    curve.set_parameter_hook(Arc::new(move || {
        query_count.fetch_add(1, Ordering::SeqCst);
        owner.upgrade().set_pcurve2(&replacement).unwrap();
    }));
    let copied: Handle<BRepCurveOnClosedSurface> = source.copy().unwrap().cast();
    assert_eq!(count.load(Ordering::SeqCst), 2);
    assert_ne!(source.pcurve2().unwrap(), original_second);
    assert_eq!(copied.pcurve2().unwrap(), source.pcurve2().unwrap());
}

fn replace_primary_hook(
    owner: WeakHandle<BRepCurveOnSurface>,
    count: Arc<AtomicUsize>,
    replacement: Handle<dyn Geom2dCurve>,
) -> Arc<dyn Fn() + Send + Sync> {
    Arc::new(move || {
        count.fetch_add(1, Ordering::SeqCst);
        owner.upgrade().set_pcurve(&replacement).unwrap();
    })
}

#[test]
fn surface_copy_reloads_primary_binding_between_parameter_queries() {
    let (source, count) = surface_fixture();
    let curve: Handle<RebindingParameterCurve> = source.pcurve().unwrap().cast();
    let replacement: Handle<dyn Geom2dCurve> =
        Handle::new(Geom2dLine::new(Axis2d::default())).cast();
    curve.set_parameter_hook(replace_primary_hook(
        source.downgrade(),
        count.clone(),
        replacement.clone(),
    ));
    let copied: Handle<BRepCurveOnSurface> = source.copy().unwrap().cast();
    assert_eq!(
        (
            count.load(Ordering::SeqCst),
            copied.pcurve().unwrap() == replacement
        ),
        (1, true)
    );
}

#[test]
fn seam_copy_reloads_primary_binding_between_parameter_queries() {
    let (source, count) = seam_fixture();
    let curve: Handle<RebindingParameterCurve> = source.pcurve().unwrap().cast();
    let replacement: Handle<dyn Geom2dCurve> =
        Handle::new(Geom2dLine::new(Axis2d::default())).cast();
    let expected = replacement.clone();
    let owner = source.downgrade();
    let query_count = count.clone();
    curve.set_parameter_hook(Arc::new(move || {
        query_count.fetch_add(1, Ordering::SeqCst);
        owner.upgrade().set_pcurve(&replacement).unwrap();
    }));
    let copied: Handle<BRepCurveOnClosedSurface> = source.copy().unwrap().cast();
    assert_eq!(
        (
            count.load(Ordering::SeqCst),
            copied.pcurve().unwrap() == expected
        ),
        (1, true)
    );
}

#[test]
fn public_constructor_retains_existing_first_then_last_query_order() {
    let curve = Handle::new(RebindingParameterCurve::new(
        Arc::new(AtomicUsize::new(0)),
        Arc::new(|u| Ok(Point2::new(u, 0.))),
    ));
    let projected: Handle<dyn Geom2dCurve> = curve.cast();
    BRepCurveOnSurface::new(&projected, &Handle::default(), &Location::default()).unwrap();
    assert_eq!(curve.parameter_history(), ["first", "last"]);
}

#[test]
fn seam_copy_captures_value_continuity_before_parameter_callbacks() {
    let (source, count) = seam_fixture();
    let curve: Handle<RebindingParameterCurve> = source.pcurve().unwrap().cast();
    let owner = source.downgrade();
    let query_count = count.clone();
    curve.set_parameter_hook(Arc::new(move || {
        query_count.fetch_add(1, Ordering::SeqCst);
        owner.upgrade().set_continuity(Shape::C1).unwrap();
    }));
    let copied: Handle<BRepCurveOnClosedSurface> = source.copy().unwrap().cast();
    assert_eq!(count.load(Ordering::SeqCst), 2);
    assert_eq!(source.continuity().unwrap(), Shape::C1);
    assert_eq!(copied.continuity().unwrap(), Shape::C0);
}
