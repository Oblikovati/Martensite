//! Each endpoint reads the currently bound p-curve after prior evaluation callbacks.
#[path = "curve_update_rebinding/rebinding_curve.rs"]
pub(super) mod rebinding_curve;
use martensite_tkbrep::brep::{
    BRepCurveOnClosedSurface, BRepCurveOnSurface, BRepCurveRepresentation, BRepGCurve,
};
use martensite_tkernel::standard::{Handle, SharedState, WeakHandle};
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dLine};
use martensite_tkmath::{
    geom_abs::Shape,
    gp::{Direction2, Point2},
    toploc::Location,
};
use rebinding_curve::RebindingParameterCurve;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

type SurfaceOwner = Arc<SharedState<WeakHandle<BRepCurveOnSurface>>>;

fn replacement_line() -> Handle<dyn Geom2dCurve> {
    Handle::new(Geom2dLine::from_point(
        Point2::new(10., 20.),
        Direction2::new(1., 0.).unwrap(),
    ))
    .cast()
}

fn rebinding_hook(
    owner: SurfaceOwner,
) -> Arc<dyn Fn(f64) -> Result<Point2, martensite_tkernel::standard::Failure> + Send + Sync> {
    let replacement = replacement_line();
    Arc::new(move |u| {
        owner.read().upgrade().set_pcurve(&replacement)?;
        Ok(Point2::new(u, 0.))
    })
}

fn rebinding_fixture() -> (Handle<BRepCurveOnSurface>, Arc<AtomicUsize>) {
    let evaluations = Arc::new(AtomicUsize::new(0));
    let owner = Arc::new(SharedState::new(WeakHandle::default()));
    let curve: Handle<dyn Geom2dCurve> = Handle::new(RebindingParameterCurve::new(
        evaluations.clone(),
        rebinding_hook(owner.clone()),
    ))
    .cast();
    let value = Handle::new(
        BRepCurveOnSurface::new(&curve, &Handle::default(), &Location::default()).unwrap(),
    );
    *owner.write() = value.downgrade();
    (value, evaluations)
}

#[test]
fn last_endpoint_reads_pcurve_replaced_during_first_endpoint_evaluation() {
    let (value, evaluations) = rebinding_fixture();
    value.set_range(2., 4.).unwrap();
    assert_eq!(evaluations.load(Ordering::SeqCst), 1);
    assert_eq!(
        value.uv_points(),
        (Point2::new(2., 0.), Point2::new(14., 20.))
    );
}

fn seam_from_second(curve: &Handle<dyn Geom2dCurve>) -> Handle<BRepCurveOnClosedSurface> {
    let first: Handle<dyn Geom2dCurve> =
        Handle::new(Geom2dLine::new(martensite_tkmath::gp::Axis2d::default())).cast();
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

type SeamOwner = Arc<SharedState<WeakHandle<BRepCurveOnClosedSurface>>>;

fn seam_rebinding_hook(
    owner: SeamOwner,
) -> Arc<dyn Fn(f64) -> Result<Point2, martensite_tkernel::standard::Failure> + Send + Sync> {
    let replacement = replacement_line();
    Arc::new(move |u| {
        owner.read().upgrade().set_pcurve2(&replacement)?;
        Ok(Point2::new(u, 0.))
    })
}

fn seam_rebinding_fixture() -> (Handle<BRepCurveOnClosedSurface>, Arc<AtomicUsize>) {
    let evaluations = Arc::new(AtomicUsize::new(0));
    let owner = Arc::new(SharedState::new(WeakHandle::default()));
    let curve: Handle<dyn Geom2dCurve> = Handle::new(RebindingParameterCurve::new(
        evaluations.clone(),
        seam_rebinding_hook(owner.clone()),
    ))
    .cast();
    let value = seam_from_second(&curve);
    *owner.write() = value.downgrade();
    (value, evaluations)
}

#[test]
fn seam_last_endpoint_reads_second_pcurve_replaced_during_first_evaluation() {
    let (value, evaluations) = seam_rebinding_fixture();
    value.set_range(2., 4.).unwrap();
    assert_eq!(evaluations.load(Ordering::SeqCst), 1);
    assert_eq!(
        value.uv_points2(),
        (Point2::new(2., 0.), Point2::new(14., 20.))
    );
    assert_eq!(
        value.uv_points(),
        (Point2::new(2., 0.), Point2::new(4., 0.))
    );
}

type ParameterCalls = Arc<SharedState<Vec<f64>>>;

fn changing_last_hook(
    owner: SeamOwner,
    count: Arc<AtomicUsize>,
    calls: ParameterCalls,
) -> Arc<dyn Fn(f64) -> Result<Point2, martensite_tkernel::standard::Failure> + Send + Sync> {
    Arc::new(move |u| {
        calls.write().push(u);
        if count.load(Ordering::SeqCst) == 1 {
            owner.read().upgrade().set_last(8.)?;
        }
        Ok(Point2::new(u, 0.))
    })
}

fn changing_last_fixture(calls: ParameterCalls) -> Handle<BRepCurveOnClosedSurface> {
    let count = Arc::new(AtomicUsize::new(0));
    let owner = Arc::new(SharedState::new(WeakHandle::default()));
    let hook = changing_last_hook(owner.clone(), count.clone(), calls);
    let curve: Handle<dyn Geom2dCurve> =
        Handle::new(RebindingParameterCurve::new(count, hook)).cast();
    let value = seam_from_second(&curve);
    *owner.write() = value.downgrade();
    value
}

#[test]
fn seam_last_parameter_observes_nested_public_range_update() {
    let calls = Arc::new(SharedState::new(Vec::new()));
    let value = changing_last_fixture(calls.clone());
    value.set_range(2., 4.).unwrap();
    assert_eq!(value.range(), (2., 8.));
    assert_eq!(*calls.read(), [2., 2., 8., 8.]);
    assert_eq!(
        value.uv_points2(),
        (Point2::new(2., 0.), Point2::new(8., 0.))
    );
}
