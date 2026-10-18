//! Owned point-curve projectors retain adaptors and independently copied result state.
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkg3d::geom::{GeomCurve, GeomLine};
use martensite_tkg3d::geom_adaptor::GeomAdaptorCurve;
use martensite_tkgeombase::extrema::ExtPC;
use martensite_tkgeombase::failure_capture::FailureCapture;
use martensite_tkmath::gp::{Direction3, Point3};

fn retained_line_projector() -> ExtPC<'static> {
    let line: Handle<dyn GeomCurve> = Handle::new(GeomLine::from_point(
        Point3::default(),
        Direction3::new(1.0, 0.0, 0.0).unwrap(),
    ))
    .cast();
    let adaptor: Handle<dyn Adaptor3dCurve> =
        Handle::new(GeomAdaptorCurve::with_curve(&line).unwrap()).cast();
    let mut projector = ExtPC::new();
    projector
        .initialize_shared(&adaptor, -1.0, 2.0, 1e-10)
        .unwrap();
    projector
}

#[test]
fn retained_adaptor_survives_dropped_constructor_handles() {
    let mut projector = retained_line_projector();
    projector.perform(Point3::new(0.0, 3.0, 0.0)).unwrap();
    assert!(projector.is_done());
    assert_eq!(projector.nb_ext().unwrap(), 1);
    assert_eq!(projector.square_distance(1).unwrap(), 9.0);
    assert_eq!(
        projector.trimmed_square_distances(),
        (
            10.0,
            13.0,
            Point3::new(-1.0, 0.0, 0.0),
            Point3::new(2.0, 0.0, 0.0)
        )
    );
}

#[test]
fn cloned_projectors_copy_results_and_preserve_independent_completion() {
    let mut projector = retained_line_projector();
    projector.perform(Point3::new(0.0, 3.0, 0.0)).unwrap();
    let mut cloned = projector.clone();
    projector.perform(Point3::new(1.0, 4.0, 0.0)).unwrap();
    assert_eq!(projector.square_distance(1).unwrap(), 16.0);
    assert_eq!(cloned.square_distance(1).unwrap(), 9.0);
    cloned.perform(Point3::new(0.0, 5.0, 0.0)).unwrap();
    assert_eq!(cloned.square_distance(1).unwrap(), 25.0);
    assert_eq!(projector.square_distance(1).unwrap(), 16.0);
}

#[test]
fn null_owned_curve_fails_without_replacing_completed_state() {
    let mut projector = retained_line_projector();
    projector.perform(Point3::new(0.0, 3.0, 0.0)).unwrap();
    let failure = projector
        .initialize_shared(&Handle::default(), -1.0, 2.0, 1e-10)
        .unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_NullObject");
    assert!(projector.is_done());
    assert_eq!(projector.square_distance(1).unwrap(), 9.0);
}

#[test]
fn copied_callback_failure_is_consumed_independently() {
    let mut original = FailureCapture::default();
    original.keep::<()>(Err(Failure::with_message(
        "Standard_DomainError",
        "invalid parameter 7",
    )));
    let mut cloned = original.clone();
    assert_eq!(
        original.take().unwrap_err().message(),
        "invalid parameter 7"
    );
    assert!(original.take().is_ok());
    assert_eq!(
        cloned.take().unwrap_err().exception_type(),
        "Standard_DomainError"
    );
    assert!(cloned.take().is_ok());
}

#[test]
fn planar_shared_projector_and_its_copy_keep_independent_results() {
    use martensite_tkg2d::adaptor2d::Adaptor2dCurve2d;
    use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dLine};
    use martensite_tkg2d::geom2d_adaptor::Geom2dAdaptorCurve;
    use martensite_tkgeombase::extrema::ExtPC2d;
    use martensite_tkmath::gp::{Direction2, Point2};
    let line: Handle<dyn Geom2dCurve> = Handle::new(Geom2dLine::from_point(
        Point2::default(),
        Direction2::new(1.0, 0.0).unwrap(),
    ))
    .cast();
    let adaptor: Handle<dyn Adaptor2dCurve2d> =
        Handle::new(Geom2dAdaptorCurve::with_curve(&line).unwrap()).cast();
    let mut projector = ExtPC2d::new();
    projector
        .initialize_shared(&adaptor, -1.0, 2.0, 1e-10)
        .unwrap();
    projector.perform(Point2::new(0.0, 3.0)).unwrap();
    let mut cloned = projector.clone();
    cloned.perform(Point2::new(1.0, 4.0)).unwrap();
    assert_eq!(projector.square_distance(1).unwrap(), 9.0);
    assert_eq!(cloned.square_distance(1).unwrap(), 16.0);
}
