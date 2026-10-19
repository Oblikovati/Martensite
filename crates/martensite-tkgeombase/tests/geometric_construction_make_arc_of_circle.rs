//! Gc make arc of circle contracts.
#![allow(non_snake_case)]
use martensite_tkernel::precision;
use martensite_tkg3d::geom::GeomBoundedCurve;
use martensite_tkgeombase::gc::{ErrorType, MakeArcOfCircle, Root};
use martensite_tkmath::gp::{Point3, Vector3};

#[test]
fn make_arc_of_circle_from_points_and_tangent_done() {
    let aP1 = Point3::new(1.0, 0.0, 0.0);
    let aP2 = Point3::new(0.0, 1.0, 0.0);
    let aTangentAtP1 = Vector3::new(0.0, 1.0, 0.0);

    let aMaker = MakeArcOfCircle::from_tangent(aP1, aTangentAtP1, aP2).unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);
    assert!(!aMaker.value().unwrap().is_null());
}

#[test]
fn make_arc_of_circle_from_three_points_done() {
    let aMaker = MakeArcOfCircle::through_points(
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(-1.0, 0.0, 0.0),
    )
    .unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);
    assert!(!aMaker.value().unwrap().is_null());
}

#[test]
fn make_arc_of_circle_from_points_and_tangent_confusion_scale_done() {
    let aScale: f64 = 2.0 * precision::confusion();
    let aP1 = Point3::new(aScale, 0.0, 0.0);
    let aP2 = Point3::new(0.0, aScale, 0.0);
    let aTangentAtP1 = Vector3::new(0.0, 1.0, 0.0);

    let aMaker = MakeArcOfCircle::from_tangent(aP1, aTangentAtP1, aP2).unwrap();
    assert!(aMaker.is_done());
    assert_eq!(aMaker.status(), ErrorType::Done);
    assert!(!aMaker.value().unwrap().is_null());
    let aStart: Point3 = aMaker.value().unwrap().start_point().unwrap();
    assert!(aStart.is_equal(&aP1, precision::confusion()));
}
