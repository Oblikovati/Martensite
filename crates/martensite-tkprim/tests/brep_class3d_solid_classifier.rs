//! Tests of `BRepClass3d_SolidClassifier`; original builders and assertions.
#![allow(non_snake_case)]
mod primitive_legacy_support;
use martensite_tkernel::precision;
use martensite_tkg3d::top_abs::State;
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSphere;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use martensite_tktopalgo::brep_class3d::BRepClass3dSolidClassifier;
use primitive_legacy_support::legacy_box_at;

fn assert_box_point(point: Point3, expected: State) {
    let shape = legacy_box_at(Point3::new(0.0, 0.0, 0.0));
    let mut classifier = BRepClass3dSolidClassifier::with_shape(&shape).unwrap();
    classifier.perform(&point, precision::confusion()).unwrap();
    assert_eq!(classifier.state(), expected);
}

#[test]
fn BRepClass3d_SolidClassifierTest__PointInside_Box() {
    assert_box_point(Point3::new(5.0, 5.0, 5.0), State::In);
}

#[test]
fn BRepClass3d_SolidClassifierTest__PointOutside_Box() {
    assert_box_point(Point3::new(20.0, 20.0, 20.0), State::Out);
}

#[test]
fn BRepClass3d_SolidClassifierTest__PointOnFace_Box() {
    assert_box_point(Point3::new(5.0, 5.0, 0.0), State::On);
}

#[test]
fn BRepClass3d_SolidClassifierTest__PointInside_Sphere() {
    let mut maker = BRepPrimAPIMakeSphere::new(10.0).unwrap();
    let sphere = maker.shape().unwrap().clone();
    assert!(maker.is_done(), "Sphere creation failed");
    let mut classifier = BRepClass3dSolidClassifier::with_shape(&sphere).unwrap();
    classifier
        .perform(&Point3::new(0.0, 0.0, 0.0), precision::confusion())
        .unwrap();
    assert_eq!(classifier.state(), State::In);
}

#[test]
fn BRepClass3d_SolidClassifierTest__PerformInfinitePoint() {
    let shape = legacy_box_at(Point3::new(0.0, 0.0, 0.0));
    let mut classifier = BRepClass3dSolidClassifier::with_shape(&shape).unwrap();
    classifier
        .perform_infinite_point(precision::confusion())
        .unwrap();
    assert_eq!(classifier.state(), State::Out);
}
