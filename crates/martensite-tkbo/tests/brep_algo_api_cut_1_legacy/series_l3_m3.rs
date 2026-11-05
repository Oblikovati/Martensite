//! BRepAlgoAPICut_Test_1 cases RolexCaseForwardForward_L3 .. BoxCutByPreviousResult_M3.
#![allow(non_snake_case)]

use super::*;

// Test bcut_simple/L3: Rolex case - complex profile operations (forward/forward)
#[test]
fn BCutSimpleTest1__RolexCaseForwardForward_L3() {
    // Create profile f1: "profile f1 c 60 360"
    let aPlane1 = Plane3::new(
        Point3::new(0.0, 0.0, 0.0),
        Direction3::new(0.0, 0.0, 1.0).unwrap(),
    )
    .unwrap();
    let aProfileOps1 = [ProfileOperation::new(ProfileCmd::C, &[60.0, 360.0])];
    let aProfile1 = CreateProfile(&aPlane1, &aProfileOps1);
    let aPrism1 = CreatePrism(&aProfile1, Vector3::new(0.0, 0.0, 20.0));

    // Create profile f2: "profile f2 o 0 0 20 f 10 -20 c 40 360"
    let aPlane2 = Plane3::new(
        Point3::new(0.0, 0.0, 20.0),
        Direction3::new(0.0, 0.0, 1.0).unwrap(),
    )
    .unwrap();
    let aProfileOps2 = [
        ProfileOperation::new(ProfileCmd::O, &[0.0, 0.0, 20.0]),
        ProfileOperation::new(ProfileCmd::F, &[10.0, -20.0]),
        ProfileOperation::new(ProfileCmd::C, &[40.0, 360.0]),
    ];
    let aProfile2 = CreateProfile(&aPlane2, &aProfileOps2);
    let aPrism2 = CreatePrism(&aProfile2, Vector3::new(0.0, 0.0, -6.0));

    // First cut operation: "bcut r1 p1 p2"
    let aCutOp1 = BRepAlgoAPICut::from_shapes(&aPrism1, &aPrism2).unwrap();
    assert!(aCutOp1.is_done(), "First cut operation failed");
    let aIntermediate = aCutOp1.shape().clone();

    // Create profile f3: "profile f3 p 0 0 -1 1 0 0 o 0 0 23 f 50 -10 c -30 360"
    let aPlane3 = Plane3::new(
        Point3::new(0.0, 0.0, -1.0),
        Direction3::new(1.0, 0.0, 0.0).unwrap(),
    )
    .unwrap();
    let aProfileOps3 = [
        ProfileOperation::new(ProfileCmd::O, &[0.0, 0.0, 23.0]),
        ProfileOperation::new(ProfileCmd::F, &[50.0, -10.0]),
        ProfileOperation::new(ProfileCmd::C, &[-30.0, 360.0]),
    ];
    let aProfile3 = CreateProfile(&aPlane3, &aProfileOps3);
    let aPrism3 = CreatePrism(&aProfile3, Vector3::new(0.0, 0.0, -9.0));

    // Final cut operation: "bcut result r1 p3"
    let aResult = PerformCut(&aIntermediate, &aPrism3);
    ValidateResult(&aResult, 30153.0, -1.0, false);
}

// Test bcut_simple/L4: Rolex case - forward object, reversed tool
#[test]
fn BCutSimpleTest1__RolexCaseForwardObjectReversedTool_L4() {
    // Create cylinder f1: "profile f1 c 60 360; prism p1 f1 0 0 20"
    let aCylinder1 = CreateCylinder(60.0, 20.0);

    // Create profile f2 with interior cylinder: "profile f2 o 0 0 20 f 10 -20 c 40 360"
    let mut aTrsf2 = Transform3::default();
    aTrsf2.set_translation(Vector3::new(0.0, 0.0, 20.0));
    let aCylinder2 = CreateCylinder(40.0, 6.0)
        .moved(&Location::new(aTrsf2), false)
        .unwrap();

    // First cut operation: "bcut r1 p1 p2"
    let aCutOp1 = BRepAlgoAPICut::from_shapes(&aCylinder1, &aCylinder2).unwrap();
    assert!(aCutOp1.is_done(), "First cut operation failed");
    let aIntermediate = aCutOp1.shape().clone();

    // Create profile f3: "profile f3 p 0 0 -1 1 0 0 o 0 0 14 f 50 -10 c -30 360"
    // This represents a cylinder at z=14 with radius 30 and height 9
    let mut aTrsf3 = Transform3::default();
    aTrsf3.set_translation(Vector3::new(0.0, 0.0, 14.0));
    let aCylinder3 = CreateCylinder(30.0, 9.0)
        .moved(&Location::new(aTrsf3), false)
        .unwrap();

    // Final cut operation: "bcut result r1 p3"
    let aResult = PerformCut(&aIntermediate, &aCylinder3);
    ValidateResult(&aResult, 30153.0, -1.0, false);
}

// Test bcut_simple/L5: Rolex case - forward/reversed orientation
#[test]
fn BCutSimpleTest1__RolexCaseForwardReversed_L5() {
    // Create cylinder f1: "profile f1 c 60 360; prism p1 f1 0 0 20"
    let aCylinder1 = CreateCylinder(60.0, 20.0);

    // Create profile f2 with interior cylinder: "profile f2 o 0 0 20 f 10 -20 c 40 360"
    let mut aTrsf2 = Transform3::default();
    aTrsf2.set_translation(Vector3::new(0.0, 0.0, 20.0));
    let aCylinder2 = CreateCylinder(40.0, 6.0)
        .moved(&Location::new(aTrsf2), false)
        .unwrap();

    // First cut operation: "bcut r1 p1 p2"
    let aCutOp1 = BRepAlgoAPICut::from_shapes(&aCylinder1, &aCylinder2).unwrap();
    assert!(aCutOp1.is_done(), "First cut operation failed");
    let aIntermediate = aCutOp1.shape().clone();

    // Create profile f3: "profile f3 o 0 0 23 f 50 10 c 30 360"
    // This represents a cylinder at z=23 with radius 30 and height -9
    let mut aTrsf3 = Transform3::default();
    aTrsf3.set_translation(Vector3::new(0.0, 0.0, 23.0));
    let aCylinder3 = CreateCylinder(30.0, 9.0)
        .moved(&Location::new(aTrsf3), false)
        .unwrap();

    // Final cut operation: "bcut result r1 p3"
    let aResult = PerformCut(&aIntermediate, &aCylinder3);
    ValidateResult(&aResult, 30153.0, -1.0, false);
}

// Test bcut_simple/L6: Rolex case - forward/reversed variation
#[test]
fn BCutSimpleTest1__RolexCaseForwardReversedVar_L6() {
    // Create cylinder f1: "profile f1 c 60 360; prism p1 f1 0 0 20"
    let aCylinder1 = CreateCylinder(60.0, 20.0);

    // Create profile f2 with interior cylinder: "profile f2 o 0 0 20 f 10 -20 c 40 360"
    let mut aTrsf2 = Transform3::default();
    aTrsf2.set_translation(Vector3::new(0.0, 0.0, 20.0));
    let aCylinder2 = CreateCylinder(40.0, 6.0)
        .moved(&Location::new(aTrsf2), false)
        .unwrap();

    // First cut operation: "bcut r1 p1 p2"
    let aCutOp1 = BRepAlgoAPICut::from_shapes(&aCylinder1, &aCylinder2).unwrap();
    assert!(aCutOp1.is_done(), "First cut operation failed");
    let aIntermediate = aCutOp1.shape().clone();

    // Create profile f3: "profile f3 o 0 0 14 f 50 10 c 30 360"
    // This represents a cylinder at z=14 with radius 30 and height 9
    let mut aTrsf3 = Transform3::default();
    aTrsf3.set_translation(Vector3::new(0.0, 0.0, 14.0));
    let aCylinder3 = CreateCylinder(30.0, 9.0)
        .moved(&Location::new(aTrsf3), false)
        .unwrap();

    // Final cut operation: "bcut result r1 p3"
    let aResult = PerformCut(&aIntermediate, &aCylinder3);
    ValidateResult(&aResult, 30153.0, -1.0, false);
}

// Test bcut_simple/L8: Simple cylinder cut operation (CTS21801)
#[test]
fn BCutSimpleTest1__SimpleCylinderCutOperation_L8() {
    // Create cylinder c1: "pcylinder c1 20 100"
    let aCylinder1 = CreateCylinder(20.0, 100.0);

    // Create cylinder c2: "pcylinder c2 20 100; ttranslate c2 0 0 50"
    let mut aTrsf = Transform3::default();
    aTrsf.set_translation(Vector3::new(0.0, 0.0, 50.0));
    let aCylinder2 = CreateCylinder(20.0, 100.0)
        .moved(&Location::new(aTrsf), false)
        .unwrap();

    // Cut operation: "bcut result c1 c2"
    let aResult = PerformCut(&aCylinder1, &aCylinder2);
    ValidateResult(&aResult, 8796.46, -1.0, false);
}

// Face sewing regressions from bcut_simple/L9 and M1 (JAP60271).
use martensite_tkbrep::{
    brep::BRepBuilder,
    topexp::Explorer,
    topods::{CompSolid, Solid},
};
use martensite_tkernel::message::ProgressRange;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPISewing;

fn sewn_compsolid(second_origin: Point3, second_face: usize) -> CompSolid {
    let first = CreateBox(Point3::new(0., 0., 0.), 10., 10., 10.);
    let second = CreateBox(second_origin, 5., 5., 5.);
    let trimmed = PerformCut(
        &GetFaceByIndex(&first, 6).into_shape(),
        &GetFaceByIndex(&second, second_face).into_shape(),
    );
    let mut sewing = BRepBuilderAPISewing::new(1e-6, true, true, true, false);
    for index in 1..=5 {
        sewing
            .add(&GetFaceByIndex(&first, index).into_shape())
            .unwrap();
    }
    sewing
        .add(&GetFaceByIndex(&trimmed, 1).into_shape())
        .unwrap();
    sewing
        .add(&GetFaceByIndex(&second, second_face).into_shape())
        .unwrap();
    sewing.perform(&ProgressRange::default()).unwrap();
    let builder = BRepBuilder::default();
    let mut shells =
        Explorer::new(sewing.sewed_shape(), ShapeEnum::Shell, ShapeEnum::Shape).unwrap();
    let shell = std::iter::Iterator::next(&mut shells).expect("sewing must create a shell");
    let mut solid = Solid::default();
    builder.make_solid(&mut solid);
    builder.add(&mut solid, &shell).unwrap();
    let mut result = CompSolid::default();
    builder.make_comp_solid(&mut result);
    builder.add(&mut result, &second).unwrap();
    builder.add(&mut result, &solid).unwrap();
    result
}

#[test]
fn BCutSimpleTest1__ComplexFaceBasedOperation_L9() {
    let result = sewn_compsolid(Point3::new(0., 0., 10.), 5);
    ValidateResult(&result.into_shape(), 750., -1., false);
}

#[test]
fn BCutSimpleTest1__ComplexFaceBasedCompSolidOperation_M1() {
    let result = sewn_compsolid(Point3::new(2., 2., 10.), 6);
    ValidateResult(&result.into_shape(), 750., -1., false);
}

// Test bcut_simple/M2: Box cut by cylinder, then result cut by original box (empty result)
#[test]
fn BCutSimpleTest1__BoxCutCylinderThenCutByBox_M2() {
    // Create box b: "box b 10 10 10"
    let aBox = CreateBox(Point3::new(0.0, 0.0, 0.0), 10.0, 10.0, 10.0);

    // Copy box: "copy b c"
    let aBoxCopy = aBox.clone();

    // Create cylinder: "pcylinder s 2 4; ttranslate s 5 5 -2"
    let mut aTrsf = Transform3::default();
    aTrsf.set_translation(Vector3::new(5.0, 5.0, -2.0));
    let aCylinder = CreateCylinder(2.0, 4.0)
        .moved(&Location::new(aTrsf), false)
        .unwrap();

    // First cut operation: "bcut rr c s"
    let aIntermediate = PerformCut(&aBoxCopy, &aCylinder);

    // Second cut operation: "bcut result rr_1 c" - cutting intermediate result by original box
    // This should result in empty shape since we're cutting the modified box by the original box
    let aResult = PerformCut(&aIntermediate, &aBox);

    // The original test expects empty result: "checkprops result -s empty"
    // For empty result, we expect very small or zero surface area
    assert!(
        aResult.is_null() || GetSurfaceArea(&aResult) < 0.1,
        "Expected empty or very small result"
    );
}

// Test bcut_simple/M3: Box cut by previous result
#[test]
fn BCutSimpleTest1__BoxCutByPreviousResult_M3() {
    // Create box b: "box b 10 10 10"
    let aBox = CreateBox(Point3::new(0.0, 0.0, 0.0), 10.0, 10.0, 10.0);

    // Copy box: "copy b c"
    let aBoxCopy = aBox.clone();

    // Create cylinder: "pcylinder s 2 4; ttranslate s 5 5 -2"
    let mut aTrsf = Transform3::default();
    aTrsf.set_translation(Vector3::new(5.0, 5.0, -2.0));
    let aCylinder = CreateCylinder(2.0, 4.0)
        .moved(&Location::new(aTrsf), false)
        .unwrap();

    // First cut operation: "bcut rr c s"
    let aIntermediate = PerformCut(&aBoxCopy, &aCylinder);

    // Second cut operation: "bcut result c rr_1" - cutting original box by intermediate result
    let aResult = PerformCut(&aBox, &aIntermediate);

    // Validate result: "checkprops result -s 50.2655"
    ValidateResult(&aResult, 50.2655, -1.0, false);
}
