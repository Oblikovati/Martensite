//! Parameter evaluation must precede any surface access in representation D0.
#[path = "curve_on_surface_order/failing_curve.rs"]
pub(super) mod failing_curve;
use failing_curve::FailingParameterCurve;
use martensite_tkbrep::brep::{BRepCurveOnSurface, BRepGCurve};
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dLine};
use martensite_tkmath::toploc::Location;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

#[test]
fn parameter_failure_precedes_missing_surface_and_preserves_owned_error() {
    let evaluations = Arc::new(AtomicUsize::new(0));
    let curve: Handle<dyn Geom2dCurve> =
        Handle::new(FailingParameterCurve::new(evaluations.clone())).cast();
    let representation =
        BRepCurveOnSurface::new(&curve, &Handle::default(), &Location::default()).unwrap();
    let error = representation.d0(3.5).unwrap_err();
    assert_eq!(error.exception_type(), "Standard_RangeError");
    assert_eq!(evaluations.load(Ordering::SeqCst), 1);
    assert_eq!(
        error.message(),
        "parameter 3.5; expected an accepted extension parameter"
    );
}

#[test]
fn successful_parameter_evaluation_still_reports_missing_surface() {
    let curve: Handle<dyn Geom2dCurve> =
        Handle::new(Geom2dLine::new(martensite_tkmath::gp::Axis2d::default())).cast();
    let representation =
        BRepCurveOnSurface::new(&curve, &Handle::default(), &Location::default()).unwrap();
    assert_eq!(
        representation.d0(3.5).unwrap_err().exception_type(),
        "Standard_NullObject"
    );
}

#[test]
fn located_surface_evaluation_retains_exact_cartesian_result_through_runtime_view() {
    use martensite_tkg3d::geom::{GeomPlane, GeomSurface};
    use martensite_tkmath::gp::{Axis3, Point3, Transform3, Vector3};
    let curve: Handle<dyn Geom2dCurve> =
        Handle::new(Geom2dLine::new(martensite_tkmath::gp::Axis2d::default())).cast();
    let surface: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(Axis3::default())).cast();
    let mut translation = Transform3::default();
    translation.set_translation(Vector3::new(8., -4., 2.));
    let concrete = Handle::new(
        BRepCurveOnSurface::new(&curve, &surface, &Location::new(translation)).unwrap(),
    );
    let view: Handle<dyn BRepGCurve> = concrete.cast();
    assert_eq!(view.d0(3.5).unwrap(), Point3::new(11.5, -4., 2.));
}
