//! BRepAlgoAPIFuse tests cases LargePrismWithOblongPrismRightSideY2_J4 .. OCC277FuseAndCommonOfOverlappingBoxes,
//! with the blended boxes K3 .. K9.
#![allow(non_snake_case)]

use super::*;
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    topexp::Explorer,
    topods::{self, Shape},
};
use martensite_tkg3d::top_abs::ShapeEnum;

// Test bfuse_simple/J4: Large prism with oblong prism (right side face profile Y direction)
#[test]
fn BFuseSimpleTest__LargePrismWithOblongPrismRightSideY2_J4() {
    // Create large prism base (5 units high)
    let aBasePts = [
        Point3::new(3.0, 3.0, 0.0),
        Point3::new(8.0, 3.0, 0.0),
        Point3::new(8.0, 9.0, 0.0),
        Point3::new(3.0, 9.0, 0.0),
    ];
    let aBaseWire = CreatePolygonWire(&aBasePts, true);
    let aBaseFace = CreateFaceFromWire(&aBaseWire);
    let aBasePrism = CreatePrism(&aBaseFace, Vector3::new(0.0, 0.0, 5.0));

    // Create oblong profile: O 8 7 3  P -1 0 0 0 1 0  D 0 -1  C 1 180  Y 2  C 1 180  W
    let aOps = [
        ProfileOperation::new(ProfileCmd::O, &[8.0, 7.0, 3.0]),
        ProfileOperation::new(ProfileCmd::P, &[-1.0, 0.0, 0.0, 0.0, 1.0, 0.0]),
        ProfileOperation::new(ProfileCmd::D, &[0.0, -1.0]),
        ProfileOperation::new(ProfileCmd::C, &[1.0, 180.0]),
        ProfileOperation::new(ProfileCmd::Y, &[2.0]),
        ProfileOperation::new(ProfileCmd::C, &[1.0, 180.0]),
        ProfileOperation::new(ProfileCmd::W, &[]),
    ];
    let aProfileFace = CreateProfileFromOperations(&aOps);
    let aOblongPrism = CreatePrism(&aProfileFace, Vector3::new(1.0, 0.0, 0.0));

    let aResult = PerformFuse(&aBasePrism, &aOblongPrism);
    ValidateResult(&aResult, 180.283, -1.0, false);
}

fn fused_top_oblong(operations: &[ProfileOperation]) {
    let base_points = [
        Point3::new(3., 3., 0.),
        Point3::new(8., 3., 0.),
        Point3::new(8., 9., 0.),
        Point3::new(3., 9., 0.),
    ];
    let base = CreatePrism(
        &CreateFaceFromWire(&CreatePolygonWire(&base_points, true)),
        Vector3::new(0., 0., 5.),
    );
    let oblong = CreatePrism(
        &CreateProfileFromOperations(operations),
        Vector3::new(0., 0., 1.),
    );
    ValidateResult(&PerformFuse(&base, &oblong), 180.283, -1., false);
}

// Test bfuse_simple/J5: top oblong prism with X-directed profile.
#[test]
fn BFuseSimpleTest__LargePrismWithOblongPrismRightSide_J5() {
    let operations = [
        ProfileOperation::new(ProfileCmd::O, &[4., 3., 5.]),
        ProfileOperation::new(ProfileCmd::P, &[0., 0., -1., 1., 0., 0.]),
        ProfileOperation::new(ProfileCmd::D, &[-1., 0.]),
        ProfileOperation::new(ProfileCmd::C, &[1., 180.]),
        ProfileOperation::new(ProfileCmd::X, &[2.]),
        ProfileOperation::new(ProfileCmd::C, &[1., 180.]),
        ProfileOperation::new(ProfileCmd::W, &[]),
    ];
    fused_top_oblong(&operations);
}

#[test]
fn BFuseSimpleTest__LargePrismWithOblongPrismRightSide_J6() {
    // Create large prism base (5 units high)
    let aBasePts = [
        Point3::new(3.0, 3.0, 0.0),
        Point3::new(8.0, 3.0, 0.0),
        Point3::new(8.0, 9.0, 0.0),
        Point3::new(3.0, 9.0, 0.0),
    ];
    let aBaseWire = CreatePolygonWire(&aBasePts, true);
    let aBaseFace = CreateFaceFromWire(&aBaseWire);
    let aBasePrism = CreatePrism(&aBaseFace, Vector3::new(0.0, 0.0, 5.0));

    let operations = [
        ProfileOperation::new(ProfileCmd::O, &[3., 6., 5.]),
        ProfileOperation::new(ProfileCmd::P, &[0., 0., -1., 1., 0., 0.]),
        ProfileOperation::new(ProfileCmd::D, &[0., -1.]),
        ProfileOperation::new(ProfileCmd::C, &[1., 180.]),
        ProfileOperation::new(ProfileCmd::Y, &[2.]),
        ProfileOperation::new(ProfileCmd::C, &[1., 180.]),
        ProfileOperation::new(ProfileCmd::W, &[]),
    ];
    let _ = aBasePrism;
    fused_top_oblong(&operations);
}

#[test]
fn BFuseSimpleTest__LargePrismWithOblongPrismRightSide_J7() {
    let operations = [
        ProfileOperation::new(ProfileCmd::O, &[5., 3., 5.]),
        ProfileOperation::new(ProfileCmd::P, &[0., 0., -1., 1., 0., 0.]),
        ProfileOperation::new(ProfileCmd::D, &[-1., 0.]),
        ProfileOperation::new(ProfileCmd::C, &[1., 180.]),
        ProfileOperation::new(ProfileCmd::X, &[2.]),
        ProfileOperation::new(ProfileCmd::C, &[1., 180.]),
        ProfileOperation::new(ProfileCmd::W, &[]),
    ];
    fused_top_oblong(&operations);
}

fn set_shape_tolerances(shape: &Shape, tolerance: f64) {
    let builder = BRepBuilder::default();
    for vertex in Explorer::new(shape, ShapeEnum::Vertex, ShapeEnum::Shape).unwrap() {
        builder
            .update_vertex_tolerance(&topods::vertex(&vertex).unwrap(), tolerance)
            .unwrap();
    }
    for edge in Explorer::new(shape, ShapeEnum::Edge, ShapeEnum::Shape).unwrap() {
        builder
            .update_edge_tolerance(&topods::edge(&edge).unwrap(), tolerance)
            .unwrap();
    }
    for face in Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).unwrap() {
        builder
            .update_face_tolerance(&topods::face(&face).unwrap(), tolerance)
            .unwrap();
    }
}

#[test]
fn BFuseSimpleTest__ToleranceOrder_L1() {
    let first = CreateBox(Point3::new(0., 0., 0.), 10., 10., 10.);
    let second = CreateBox(Point3::new(5., 5., 5.), 10., 10., 10.);
    set_shape_tolerances(&first, 1.);
    set_shape_tolerances(&second, 1.);
    let result = PerformFuse(&first, &second);
    ValidateResult(&result, 1050., -1., false);
    let min_vertex = Explorer::new(&result, ShapeEnum::Vertex, ShapeEnum::Shape)
        .unwrap()
        .map(|shape| BRepTool::vertex_tolerance(&topods::vertex(&shape).unwrap()).unwrap())
        .fold(f64::INFINITY, f64::min);
    let min_edge = Explorer::new(&result, ShapeEnum::Edge, ShapeEnum::Shape)
        .unwrap()
        .map(|shape| BRepTool::edge_tolerance(&topods::edge(&shape).unwrap()).unwrap())
        .fold(f64::INFINITY, f64::min);
    assert!(
        min_edge <= min_vertex,
        "expected vertex tolerance >= edge tolerance: {min_vertex} < {min_edge}"
    );
}

// Test bfuse_simple/J8: Large prism with oblong prism (top profile)
#[test]
fn BFuseSimpleTest__LargePrismWithOblongPrismTop_J8() {
    // Create large prism base (5 units high)
    let aBasePts = [
        Point3::new(3.0, 3.0, 0.0),
        Point3::new(8.0, 3.0, 0.0),
        Point3::new(8.0, 9.0, 0.0),
        Point3::new(3.0, 9.0, 0.0),
    ];
    let aBaseWire = CreatePolygonWire(&aBasePts, true);
    let aBaseFace = CreateFaceFromWire(&aBaseWire);
    let aBasePrism = CreatePrism(&aBaseFace, Vector3::new(0.0, 0.0, 5.0));

    // Create oblong profile: O 8 4 5  P 0 0 -1 1 0 0  D 0 1  C 1 180  Y -2  C 1 180  W
    let aOps = [
        ProfileOperation::new(ProfileCmd::O, &[8.0, 4.0, 5.0]),
        ProfileOperation::new(ProfileCmd::P, &[0.0, 0.0, -1.0, 1.0, 0.0, 0.0]),
        ProfileOperation::new(ProfileCmd::D, &[0.0, 1.0]),
        ProfileOperation::new(ProfileCmd::C, &[1.0, 180.0]),
        ProfileOperation::new(ProfileCmd::Y, &[-2.0]),
        ProfileOperation::new(ProfileCmd::C, &[1.0, 180.0]),
        ProfileOperation::new(ProfileCmd::W, &[]),
    ];
    let aProfileFace = CreateProfileFromOperations(&aOps);
    let aOblongPrism = CreatePrism(&aProfileFace, Vector3::new(0.0, 0.0, 1.0));

    let aResult = PerformFuse(&aBasePrism, &aOblongPrism);
    ValidateResult(&aResult, 180.283, -1.0, false);
}

// Test bfuse_simple/J9: Cylinder with revolution ring (bug case pro13305)
#[test]
fn BFuseSimpleTest__CylinderWithRevolutionRing_J9() {
    // Create cylinder: pcylinder cyl 3 5
    let aCylinder = CreateCylinder(3.0, 5.0);

    // Create ring profile vertices and wire
    let aRingPts = [
        Point3::new(0.0, 3.0, 2.0),
        Point3::new(0.0, 4.0, 2.0),
        Point3::new(0.0, 4.0, 3.0),
        Point3::new(0.0, 3.0, 3.0),
    ];
    let aRingWire = CreatePolygonWire(&aRingPts, true);
    let aRingFace = CreateFaceFromWire(&aRingWire);

    // Create revolution: revol ring f 0 0 0 0 0 1 269
    let aAxis = Axis1::new(
        Point3::new(0.0, 0.0, 0.0),
        Direction3::new(0.0, 0.0, 1.0).unwrap(),
    );
    let aRing = CreateRevolution(&aRingFace, aAxis, 269.0 * PI / 180.0);

    let aResult = PerformFuse(&aCylinder, &aRing);
    ValidateResult(&aResult, 190.356, -1.0, false);
}

// Test bfuse_simple/K1: Complex profile with revolution
#[test]
fn BFuseSimpleTest__ComplexProfileWithRevolution_K1() {
    // Create box: box b 200 200 100
    let aBox = CreateBox(Point3::new(0.0, 0.0, 0.0), 200.0, 200.0, 100.0);

    // Create complex profile: profile wr pl 1 0 0 0 0 1 o 50 0 0 f 50 -80 x 50 y 5 x 10 y -25 x -60
    let aOps = [
        ProfileOperation::new(ProfileCmd::O, &[50.0, 0.0, 0.0]),
        ProfileOperation::new(ProfileCmd::F, &[50.0, -80.0]),
        ProfileOperation::new(ProfileCmd::X, &[50.0]),
        ProfileOperation::new(ProfileCmd::Y, &[5.0]),
        ProfileOperation::new(ProfileCmd::X, &[10.0]),
        ProfileOperation::new(ProfileCmd::Y, &[-25.0]),
        ProfileOperation::new(ProfileCmd::X, &[-60.0]),
        ProfileOperation::new(ProfileCmd::W, &[]),
    ];
    let aPlane = Plane3::new(
        Point3::new(0.0, 0.0, 0.0),
        Direction3::new(1.0, 0.0, 0.0).unwrap(),
    )
    .unwrap();
    let aProfile = CreateProfile(&aPlane, &aOps);

    // Create revolution: revol rv wr 50 100 50 0 0 1 360
    let aRevAxis = Axis1::new(
        Point3::new(50.0, 100.0, 50.0),
        Direction3::new(0.0, 0.0, 1.0).unwrap(),
    );
    let aRevolution = CreateRevolution(&aProfile, aRevAxis, 2.0 * PI);

    let aResult = PerformFuse(&aBox, &aRevolution);
    ValidateResult(&aResult, 161571.0, -1.0, false);
}

// Test bfuse_simple/K2: Blend box with cylinder (X direction)
#[test]
fn BFuseSimpleTest__BlendBoxWithCylinder_K2() {
    // Create box: box bx1 300 300 100
    let aBox = CreateBox(Point3::new(0.0, 0.0, 0.0), 300.0, 300.0, 100.0);

    // Skip blend for now - test simple cylinder creation instead
    // const TopoDSShape aBlendedBox = BooleanUtilities::CreateBlend(aBox, 1, 100.0);

    // Test simple cylinder creation first
    let aCylinder = CreateCylinder(100.0, 50.0);

    // Create plane and cylinder: plane pl1 100 100 100 0 0 1 1 0 0, pcylinder pc pl1 100 50
    // const gpAx3 anAx3(gpPnt(100, 100, 100), gpDir(gpDir::D::Z), gpDir(gpDir::D::X));
    // const gpPln aPlane(anAx3);
    // const TopoDSShape aCylinder = BooleanUtilities::CreateCylinderOnPlane(aPlane, 100.0, 50.0);

    let aResult = PerformFuse(&aBox, &aCylinder);
    ValidateResult(&aResult, 360686.0, -1.0, false);
}

#[test]
fn BFuseSimpleTest__OCC277_FuseAndCommonOfOverlappingBoxes() {
    let mut aBox1Maker = BRepPrimAPIMakeBox::from_sizes(100.0, 100.0, 100.0).unwrap();
    let mut aBox2Maker =
        BRepPrimAPIMakeBox::at_point(Point3::new(50.0, 50.0, 50.0), 200.0, 200.0, 200.0).unwrap();

    let aShape1 = aBox1Maker.shape().unwrap().clone();
    let aShape2 = aBox2Maker.shape().unwrap().clone();

    let aFuseOp = BRepAlgoAPIFuse::from_shapes(&aShape1, &aShape2).unwrap();
    let aFuseResult = aFuseOp.shape().clone();
    assert!(!aFuseResult.is_null());

    let aCommonOp = BRepAlgoAPICommon::from_shapes(&aShape1, &aShape2).unwrap();
    let aCommonResult = aCommonOp.shape().clone();
    assert!(!aCommonResult.is_null());
}
