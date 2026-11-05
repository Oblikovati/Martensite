//! Regressions of box and half-space direct-cut regressions.
#![allow(non_snake_case)]
#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkbo::bop_algo::Operation;
use martensite_tkbrep::topods::{Shape, face};
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeHalfSpace;

fn xy_half_space(reference_z: f64) -> Shape {
    let face_shape = CreatePolygonFace(
        Point3::new(250., 250., 0.),
        Point3::new(-250., 250., 0.),
        Point3::new(-250., -250., 0.),
        Point3::new(250., -250., 0.),
    );
    BRepPrimAPIMakeHalfSpace::with_face(
        &face(&face_shape).unwrap(),
        Point3::new(0., 0., reference_z),
    )
    .unwrap()
    .solid()
    .unwrap()
    .clone()
    .into_shape()
}
fn cut_box_half_space(box_z: f64, reference_z: f64, area: f64) {
    let box_shape = CreateBox(Point3::new(0., 0., box_z), 150., 200., 200.);
    ValidateResult(
        &PerformDirectBOP(&box_shape, &xy_half_space(reference_z), Operation::Cut),
        area,
        -1.,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__BoxCylinder_ZP1() {
    let b = CreateBox(Point3::new(0., 0., 0.), 4., 4., 4.);
    let c = Translate(&CreateCylinder(1., 4.), 1., 1., 4.);
    ValidateResult(&PerformDirectBOP(&b, &c, Operation::Cut), 96., -1., false);
}
#[test]
fn BOPCutSimpleTest__BoxCylinder_ZP2() {
    let b = CreateBox(Point3::new(0., 0., 0.), 4., 4., 4.);
    let c = Translate(&CreateCylinder(2., 6.), 0., 0., -2.);
    ValidateResult(
        &PerformDirectBOP(&b, &c, Operation::Cut),
        86.2832,
        -1.,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__BoxHalfSpace_ZP3() {
    cut_box_half_space(-30., -100., 179000.);
}
#[test]
fn BOPCutSimpleTest__BoxHalfSpace_ZP4() {
    cut_box_half_space(-30., 100., 81000.);
}
#[test]
fn BOPCutSimpleTest__BoxHalfSpace_ZP5() {
    cut_box_half_space(-80., -100., 144000.);
}
#[test]
fn BOPCutSimpleTest__BoxHalfSpace_ZP6() {
    cut_box_half_space(-80., 100., 116000.);
}

#[test]
fn BOPCutSimpleTest__ConeFacePrism_ZP8() {
    use martensite_tkmath::gp::Vector3;
    let cone = CreateCone(10.0, 0.0, 20.0);
    let face = GetFaceByIndex(&cone, 2);
    let prism = CreatePrism(&face.into_shape(), Vector3::new(0.0, 0.0, 10.0));
    ValidateResult(
        &PerformDirectBOP(&cone, &prism, Operation::Cut),
        254.16,
        -1.0,
        false,
    );
}

#[test]
fn BOPCutSimpleTest__PrismRevolution_ZP7() {
    use martensite_tkbrep::{brep::BRepBuilder, topods::Solid};
    use martensite_tkmath::gp::{Axis1, Direction3, Vector3};
    let edge = CreateEdge(Point3::new(-1.0, -1.0, 0.0), Point3::new(2.0, -1.0, 0.0));
    let face = CreatePrism(&edge.into_shape(), Vector3::new(0.0, 3.5, 0.0));
    let base = CreatePrism(&face, Vector3::new(0.0, 0.0, 2.0));
    let wire = CreatePolygonWire(
        &[
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 2.0),
            Point3::new(0.0, 0.0, 2.0),
        ],
        false,
    );
    let revolved = CreateRevolution(
        &wire.into_shape(),
        Axis1::new(
            Point3::new(0.0, 0.0, 0.0),
            Direction3::new(0.0, 0.0, 1.0).unwrap(),
        ),
        std::f64::consts::TAU,
    );
    let builder = BRepBuilder::default();
    let mut tool = Solid::default();
    builder.make_solid(&mut tool);
    builder.add(&mut tool, &revolved).unwrap();
    ValidateResult(
        &PerformDirectBOP(&base, &tool.into_shape(), Operation::Cut),
        53.2832,
        -1.0,
        false,
    );
}

#[test]
fn BOPCutSimpleTest__PlanarCircleCut_ZQ1() {
    use martensite_tkbo::brep_algo_api::{BRepAlgoAPICut, BRepAlgoAPIFuse};
    use martensite_tkbrep::topods::Face;
    use martensite_tkernel::standard::Handle;
    use martensite_tkg3d::geom::{GeomCircle, GeomCurve};
    use martensite_tkmath::gp::{Axis2, Direction3};
    use martensite_tktopalgo::brep_builder_api::{
        BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeWire,
    };
    fn circle_face(radius: f64) -> Face {
        let curve: Handle<dyn GeomCurve> = Handle::new(
            GeomCircle::new(
                Axis2::new(
                    Point3::new(0.0, 0.0, 0.0),
                    Direction3::new(0.0, 0.0, 1.0).unwrap(),
                )
                .unwrap(),
                radius,
            )
            .unwrap(),
        )
        .cast();
        let edge = BRepBuilderAPIMakeEdge::from_curve(&curve)
            .unwrap()
            .edge()
            .unwrap();
        let wire = BRepBuilderAPIMakeWire::from_edge(&edge)
            .unwrap()
            .wire()
            .unwrap();
        BRepBuilderAPIMakeFace::from_wire(&wire, false)
            .unwrap()
            .face()
            .unwrap()
    }
    let plane = BRepBuilderAPIMakeFace::from_plane_bounds(
        martensite_tkmath::gp::Plane3::default(),
        (-100.0, 100.0, -100.0, 100.0),
    )
    .unwrap()
    .face()
    .unwrap()
    .into_shape();
    let inner = circle_face(20.0).into_shape();
    let outer = circle_face(30.0).into_shape();
    let annulus = BRepAlgoAPICut::from_shapes(&outer, &inner)
        .unwrap()
        .shape()
        .clone();
    let disk = BRepAlgoAPIFuse::from_shapes(&annulus, &inner)
        .unwrap()
        .shape()
        .clone();
    let result = BRepAlgoAPICut::from_shapes(&plane, &disk)
        .unwrap()
        .shape()
        .clone();
    ValidateResult(&result, 37172.6, -1.0, false);
}
