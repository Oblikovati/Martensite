//! Copy restores source UV values after its range evaluation callbacks finish.
use super::curve_update_rebinding::rebinding_curve::RebindingParameterCurve;
use martensite_tkbrep::brep::{
    BRepCurveOnClosedSurface, BRepCurveOnSurface, BRepCurveRepresentation, BRepGCurve,
};
use martensite_tkernel::standard::{Failure, Handle, SharedState, WeakHandle};
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dLine};
use martensite_tkmath::{
    geom_abs::Shape,
    gp::{Axis2d, Point2},
    toploc::Location,
};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

type Evaluator =
    Arc<dyn Fn(f64) -> Result<Point2, martensite_tkernel::standard::Failure> + Send + Sync>;
const COPY_FAILURE: &str = "copy callback 2; expected successful evaluation";
const INITIAL_UV: (Point2, Point2) = (Point2::new(2., 3.), Point2::new(5., 7.));
const UPDATED_UV: (Point2, Point2) = (Point2::new(17., 19.), Point2::new(23., 29.));

fn surface_hook(
    owner: Arc<SharedState<WeakHandle<BRepCurveOnSurface>>>,
    count: Arc<AtomicUsize>,
    fail: bool,
) -> Evaluator {
    Arc::new(move |u| {
        if let Some(value) = owner.read().upgrade().get() {
            value.set_uv_points(UPDATED_UV.0, UPDATED_UV.1);
        }
        if fail && !owner.read().is_null() && count.load(Ordering::SeqCst) == 2 {
            return Err(Failure::with_type("Standard_RangeError", COPY_FAILURE, ""));
        }
        Ok(Point2::new(u, 0.))
    })
}

fn surface_fixture(fail: bool) -> (Handle<BRepCurveOnSurface>, Arc<AtomicUsize>) {
    let count = Arc::new(AtomicUsize::new(0));
    let owner = Arc::new(SharedState::new(WeakHandle::default()));
    let curve: Handle<dyn Geom2dCurve> = Handle::new(RebindingParameterCurve::new(
        count.clone(),
        surface_hook(owner.clone(), count.clone(), fail),
    ))
    .cast();
    let value = Handle::new(
        BRepCurveOnSurface::new(&curve, &Handle::default(), &Location::default()).unwrap(),
    );
    value.set_range(2., 4.).unwrap();
    value.set_uv_points(INITIAL_UV.0, INITIAL_UV.1);
    *owner.write() = value.downgrade();
    count.store(0, Ordering::SeqCst);
    (value, count)
}

#[test]
fn surface_copy_reads_source_uv_after_range_callbacks() {
    let (value, count) = surface_fixture(false);
    let copied: Handle<BRepCurveOnSurface> = value.copy().unwrap().cast();
    assert_eq!(count.load(Ordering::SeqCst), 2);
    assert_eq!(value.uv_points(), UPDATED_UV);
    assert_eq!(copied.uv_points(), UPDATED_UV);
    assert!(value != copied);
    assert_eq!(value.pcurve().unwrap(), copied.pcurve().unwrap());
    copied.set_uv_points(INITIAL_UV.0, INITIAL_UV.1);
    assert_eq!(value.uv_points(), UPDATED_UV);
}

fn seam_hook(owner: Arc<SharedState<WeakHandle<BRepCurveOnClosedSurface>>>) -> Evaluator {
    Arc::new(move |u| {
        if let Some(value) = owner.read().upgrade().get() {
            value.set_uv_points2(UPDATED_UV.0, UPDATED_UV.1);
        }
        Ok(Point2::new(u, 0.))
    })
}

fn seam_from_curve(curve: &Handle<dyn Geom2dCurve>) -> Handle<BRepCurveOnClosedSurface> {
    let first: Handle<dyn Geom2dCurve> = Handle::new(Geom2dLine::new(Axis2d::default())).cast();
    Handle::new(
        BRepCurveOnClosedSurface::new(
            &first,
            curve,
            &Handle::default(),
            &Location::default(),
            Shape::C0,
        )
        .unwrap(),
    )
}

fn seam_fixture() -> (Handle<BRepCurveOnClosedSurface>, Arc<AtomicUsize>) {
    let count = Arc::new(AtomicUsize::new(0));
    let owner = Arc::new(SharedState::new(WeakHandle::default()));
    let curve: Handle<dyn Geom2dCurve> = Handle::new(RebindingParameterCurve::new(
        count.clone(),
        seam_hook(owner.clone()),
    ))
    .cast();
    let value = seam_from_curve(&curve);
    value.set_range(2., 4.).unwrap();
    value.set_uv_points2(INITIAL_UV.0, INITIAL_UV.1);
    *owner.write() = value.downgrade();
    count.store(0, Ordering::SeqCst);
    (value, count)
}

#[test]
fn seam_copy_reads_second_source_uv_after_range_callbacks() {
    let (value, count) = seam_fixture();
    let copied: Handle<BRepCurveOnClosedSurface> = value.copy().unwrap().cast();
    assert_eq!(count.load(Ordering::SeqCst), 2);
    assert_eq!(value.uv_points2(), UPDATED_UV);
    assert_eq!(copied.uv_points2(), UPDATED_UV);
}

#[test]
fn copy_failure_preserves_source_callback_mutation_and_owned_error() {
    let (value, count) = surface_fixture(true);
    let error = value.copy().unwrap_err();
    assert_eq!(error.exception_type(), "Standard_RangeError");
    assert_eq!(
        error.message(),
        "copy callback 2; expected successful evaluation"
    );
    assert_eq!(count.load(Ordering::SeqCst), 2);
    assert_eq!(value.uv_points(), UPDATED_UV);
    assert_eq!(value.range(), (2., 4.));
}
