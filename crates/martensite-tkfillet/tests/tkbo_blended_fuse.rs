//! Boolean regressions whose input construction requires blended solids.
#![allow(non_snake_case)]
use martensite_tkbo::brep_algo_api::BRepAlgoAPIFuse;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{Edge, Shape};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Axis3, Direction3, Plane3, Point3};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use martensite_topology_test_support::*;
/// Create a blend (fillet) on specified edge of a shape (like the `blend` command):
/// the `edge_index`-th edge of the explorer, the command-host defaults; a null shape when the
/// fillet fails, the shape itself when there is no such edge.
fn CreateBlend(shape: &Shape, edge_index: usize, radius: f64) -> Shape {
    use martensite_tkfillet::brep_fillet_api::BRepFilletAPIMakeFillet;
    use martensite_tkfillet::chfi3d::FilletShape;
    use martensite_tkmath::geom_abs::Shape as Continuity;
    let mut explorer = Explorer::new(shape, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
    let mut current = 1;
    let mut target: Option<Edge> = None;
    while explorer.more() {
        if current == edge_index {
            target = Some(martensite_tkbrep::topods::edge(explorer.current()).unwrap());
            break;
        }
        current += 1;
        explorer.next().unwrap();
    }
    let Some(target) = target else {
        return shape.clone();
    };
    let mut maker = BRepFilletAPIMakeFillet::new(shape, FilletShape::Rational).unwrap();
    maker.set_params(1e-2, 1.0e-4, 1.0e-5, 1.0e-4, 1.0e-5, 1.0e-3);
    maker.set_continuity(Continuity::C1, 1.0e-2);
    maker.add_with_radius(radius, &target).unwrap();
    maker
        .build(&martensite_tkernel::message::ProgressRange::default())
        .unwrap();
    if !maker.is_done() {
        return Shape::default();
    }
    maker.shape().unwrap().clone()
}
fn PerformFuse(first: &Shape, second: &Shape) -> Shape {
    let fuser = BRepAlgoAPIFuse::from_shapes(first, second).unwrap();
    assert!(fuser.is_done(), "BRepAlgoAPIFuse operation failed");
    fuser.shape().clone()
}
// Test bfuse_simple/K2: Blend box with cylinder (+X direction)
#[test]
fn BFuseSimpleTest__BlendBoxWithCylinderX_K2() {
    let box_shape = CreateBox(Point3::new(0., 0., 0.), 300., 300., 100.);
    let blended = CreateBlend(&box_shape, 1, 100.);
    assert!(!blended.is_null(), "blend operation failed");
    let axis = Axis3::new(
        Point3::new(100., 100., 100.),
        Direction3::new(0., 0., 1.).unwrap(),
        Direction3::new(1., 0., 0.).unwrap(),
    )
    .unwrap();
    let cylinder = CreateCylinderOnPlane(&Plane3::from_position(axis), 100., 50.);
    ValidateResult(&PerformFuse(&blended, &cylinder), 322832., -1., false);
}

// Test bfuse_simple/K3: Blend box with cylinder (-X direction)
#[test]
fn BFuseSimpleTest__BlendBoxWithCylinderNegX_K3() {
    // Create box: box bx1 300 300 100
    let aBox = CreateBox(Point3::new(0.0, 0.0, 0.0), 300.0, 300.0, 100.0);

    // Create blend: explode bx1 e, blend bl1 bx1 100 bx1_1
    let aBlendedBox = CreateBlend(&aBox, 1, 100.0);
    assert!(!aBlendedBox.is_null(), "Blend operation failed");

    // Create plane and cylinder: plane pl1 100 100 100 0 0 1 -1 0 0, pcylinder pc pl1 100 50
    let anAx3 = Axis3::new(
        Point3::new(100.0, 100.0, 100.0),
        Direction3::new(0.0, 0.0, 1.0).unwrap(),
        Direction3::new(-1.0, 0.0, 0.0).unwrap(),
    )
    .unwrap();
    let aPlane = Plane3::from_position(anAx3);
    let aCylinder = CreateCylinderOnPlane(&aPlane, 100.0, 50.0);
    assert!(!aCylinder.is_null(), "Cylinder creation failed");

    let aResult = PerformFuse(&aBlendedBox, &aCylinder);
    ValidateResult(&aResult, 322832.0, -1.0, false);
}
// Test bfuse_simple/K4: Blend box with cylinder (Y direction)
#[test]
fn BFuseSimpleTest__BlendBoxWithCylinderY_K4() {
    // Create box: box bx1 300 300 100
    let aBox = CreateBox(Point3::new(0.0, 0.0, 0.0), 300.0, 300.0, 100.0);

    // Create blend: explode bx1 e, blend bl1 bx1 100 bx1_1
    let aBlendedBox = CreateBlend(&aBox, 1, 100.0);
    assert!(!aBlendedBox.is_null(), "Blend operation failed");

    // Create plane and cylinder: plane pl1 100 100 100 0 0 1 0 1 0, pcylinder pc pl1 100 50
    let anAx3 = Axis3::new(
        Point3::new(100.0, 100.0, 100.0),
        Direction3::new(0.0, 0.0, 1.0).unwrap(),
        Direction3::new(0.0, 1.0, 0.0).unwrap(),
    )
    .unwrap();
    let aPlane = Plane3::from_position(anAx3);
    let aCylinder = CreateCylinderOnPlane(&aPlane, 100.0, 50.0);
    assert!(!aCylinder.is_null(), "Cylinder creation failed");

    let aResult = PerformFuse(&aBlendedBox, &aCylinder);
    ValidateResult(&aResult, 322832.0, -1.0, false);
}
// Test bfuse_simple/K5: Blend box with cylinder (-Y direction)
#[test]
fn BFuseSimpleTest__BlendBoxWithCylinderNegY_K5() {
    // Create box: box bx1 300 300 100
    let aBox = CreateBox(Point3::new(0.0, 0.0, 0.0), 300.0, 300.0, 100.0);

    // Create blend: explode bx1 e, blend bl1 bx1 100 bx1_1
    let aBlendedBox = CreateBlend(&aBox, 1, 100.0);
    assert!(!aBlendedBox.is_null(), "Blend operation failed");

    // Create plane and cylinder: plane pl1 100 100 100 0 0 1 0 -1 0, pcylinder pc pl1 100 50
    let anAx3 = Axis3::new(
        Point3::new(100.0, 100.0, 100.0),
        Direction3::new(0.0, 0.0, 1.0).unwrap(),
        Direction3::new(0.0, -1.0, 0.0).unwrap(),
    )
    .unwrap();
    let aPlane = Plane3::from_position(anAx3);
    let aCylinder = CreateCylinderOnPlane(&aPlane, 100.0, 50.0);
    assert!(!aCylinder.is_null(), "Cylinder creation failed");

    let aResult = PerformFuse(&aBlendedBox, &aCylinder);
    ValidateResult(&aResult, 322832.0, -1.0, false);
}
// Test bfuse_simple/K6: Blend box with cylinder (bottom, X direction)
#[test]
fn BFuseSimpleTest__BlendBoxWithCylinderBottomX_K6() {
    // Create box: box bx1 300 300 100
    let aBox = CreateBox(Point3::new(0.0, 0.0, 0.0), 300.0, 300.0, 100.0);

    // Create blend: explode bx1 e, blend bl1 bx1 100 bx1_1
    let aBlendedBox = CreateBlend(&aBox, 1, 100.0);
    assert!(!aBlendedBox.is_null(), "Blend operation failed");

    // Create plane and cylinder: plane pl1 100 100 0 0 0 -1 1 0 0, pcylinder pc pl1 100 50
    let anAx3 = Axis3::new(
        Point3::new(100.0, 100.0, 0.0),
        Direction3::new(0.0, 0.0, -1.0).unwrap(),
        Direction3::new(1.0, 0.0, 0.0).unwrap(),
    )
    .unwrap();
    let aPlane = Plane3::from_position(anAx3);
    let aCylinder = CreateCylinderOnPlane(&aPlane, 100.0, 50.0);
    assert!(!aCylinder.is_null(), "Cylinder creation failed");

    let aResult = PerformFuse(&aBlendedBox, &aCylinder);
    ValidateResult(&aResult, 322832.0, -1.0, false);
}
// Test bfuse_simple/K7: Blend box with cylinder (bottom, -X direction)
#[test]
fn BFuseSimpleTest__BlendBoxWithCylinderBottomNegX_K7() {
    // Create box: box bx1 300 300 100
    let aBox = CreateBox(Point3::new(0.0, 0.0, 0.0), 300.0, 300.0, 100.0);

    // Create blend: explode bx1 e, blend bl1 bx1 100 bx1_1
    let aBlendedBox = CreateBlend(&aBox, 1, 100.0);
    assert!(!aBlendedBox.is_null(), "Blend operation failed");

    // Create plane and cylinder: plane pl1 100 100 0 0 0 -1 -1 0 0, pcylinder pc pl1 100 50
    let anAx3 = Axis3::new(
        Point3::new(100.0, 100.0, 0.0),
        Direction3::new(0.0, 0.0, -1.0).unwrap(),
        Direction3::new(-1.0, 0.0, 0.0).unwrap(),
    )
    .unwrap();
    let aPlane = Plane3::from_position(anAx3);
    let aCylinder = CreateCylinderOnPlane(&aPlane, 100.0, 50.0);
    assert!(!aCylinder.is_null(), "Cylinder creation failed");

    let aResult = PerformFuse(&aBlendedBox, &aCylinder);
    ValidateResult(&aResult, 322832.0, -1.0, false);
}
// Test bfuse_simple/K8: Blend box with cylinder (bottom, Y direction)
#[test]
fn BFuseSimpleTest__BlendBoxWithCylinderBottomY_K8() {
    // Create box: box bx1 300 300 100
    let aBox = CreateBox(Point3::new(0.0, 0.0, 0.0), 300.0, 300.0, 100.0);

    // Create blend: explode bx1 e, blend bl1 bx1 100 bx1_1
    let aBlendedBox = CreateBlend(&aBox, 1, 100.0);
    assert!(!aBlendedBox.is_null(), "Blend operation failed");

    // Create plane and cylinder: plane pl1 100 100 0 0 0 -1 0 1 0, pcylinder pc pl1 100 50
    let anAx3 = Axis3::new(
        Point3::new(100.0, 100.0, 0.0),
        Direction3::new(0.0, 0.0, -1.0).unwrap(),
        Direction3::new(0.0, 1.0, 0.0).unwrap(),
    )
    .unwrap();
    let aPlane = Plane3::from_position(anAx3);
    let aCylinder = CreateCylinderOnPlane(&aPlane, 100.0, 50.0);
    assert!(!aCylinder.is_null(), "Cylinder creation failed");

    let aResult = PerformFuse(&aBlendedBox, &aCylinder);
    ValidateResult(&aResult, 322832.0, -1.0, false);
}
// Test bfuse_simple/K9: Blend box with cylinder (bottom, -Y direction)
#[test]
fn BFuseSimpleTest__BlendBoxWithCylinderBottomNegY_K9() {
    // Create box: box bx1 300 300 100
    let aBox = CreateBox(Point3::new(0.0, 0.0, 0.0), 300.0, 300.0, 100.0);

    // Create blend: explode bx1 e, blend bl1 bx1 100 bx1_1
    let aBlendedBox = CreateBlend(&aBox, 1, 100.0);
    assert!(!aBlendedBox.is_null(), "Blend operation failed");

    // Create plane and cylinder: plane pl1 100 100 0 0 0 -1 0 -1 0, pcylinder pc pl1 100 50
    let anAx3 = Axis3::new(
        Point3::new(100.0, 100.0, 0.0),
        Direction3::new(0.0, 0.0, -1.0).unwrap(),
        Direction3::new(0.0, -1.0, 0.0).unwrap(),
    )
    .unwrap();
    let aPlane = Plane3::from_position(anAx3);
    let aCylinder = CreateCylinderOnPlane(&aPlane, 100.0, 50.0);
    assert!(!aCylinder.is_null(), "Cylinder creation failed");

    let aResult = PerformFuse(&aBlendedBox, &aCylinder);
    ValidateResult(&aResult, 322832.0, -1.0, false);
}
