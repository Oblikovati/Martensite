//! Behaviour of BRepAlgoAPISimplify tests.
#![allow(non_snake_case)]

use martensite_tkbo::brep_algo_api::{BRepAlgoAPIFuse, BRepAlgoAPISection};
use martensite_tkbrep::topexp;
use martensite_tkbrep::topods::Shape;
use martensite_tkbrep::toptools::IndexedMapOfShape;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::List;
use martensite_tkernel::precision;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{self, Axis2, Plane3, Point3};
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakeBox, BRepPrimAPIMakeCylinder};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape, BRepBuilderAPISewing,
};
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;
use martensite_tktopalgo::brep_gprop::BRepGProp;

fn CountShapes(shape: &Shape, kind: ShapeEnum) -> usize {
    let mut map = IndexedMapOfShape::default();
    topexp::map_shapes(shape, &mut map, true, true);
    map.iter()
        .filter(|sub_shape| sub_shape.shape_type().unwrap() == kind)
        .count()
}

fn MakePlaneFace(plane: Plane3, bounds: (f64, f64, f64, f64)) -> Shape {
    BRepBuilderAPIMakeFace::from_plane_bounds(plane, bounds)
        .unwrap()
        .face()
        .unwrap()
        .into_shape()
}

fn MakeThreeFaceShell() -> Shape {
    let plane = Plane3::new(Point3::new(0.0, 0.0, 0.0), gp::dz()).unwrap();
    let mut sewing = BRepBuilderAPISewing::default();
    for bounds in [
        (-10.0, 10.0, -10.0, 10.0),
        (10.0, 30.0, -10.0, 10.0),
        (30.0, 50.0, -10.0, 10.0),
    ] {
        sewing.add(&MakePlaneFace(plane, bounds)).unwrap();
    }
    sewing.perform(&ProgressRange::default()).unwrap();
    sewing.sewed_shape().clone()
}

fn GetLength(shape: &Shape) -> f64 {
    let mut properties = GProps::default();
    BRepGProp::linear_properties(shape, &mut properties, false, false).unwrap();
    properties.mass()
}

fn IsValid(shape: &Shape) -> bool {
    BRepCheckAnalyzer::new(shape, true, false, false)
        .unwrap()
        .is_valid()
        .unwrap()
}

fn one(shape: &Shape) -> List<Shape> {
    [shape.clone()].into_iter().collect()
}

fn expect_counts(shape: &Shape, kinds: &[(ShapeEnum, usize)]) {
    assert!(IsValid(shape));
    for (kind, count) in kinds {
        assert_eq!(CountShapes(shape, *kind), *count, "{kind:?}");
    }
}

fn box_at(corner: Point3, z: f64) -> Shape {
    BRepPrimAPIMakeBox::at_point(corner, 10.0, 10.0, z)
        .unwrap()
        .shape()
        .unwrap()
        .clone()
}

// Equivalent to tests/boolean/simplify/A1.
#[test]
fn BRepAlgoAPISimplifyTest__A1_FuseCoplanarBoxes_SimplifiesFaces() {
    let box1 = box_at(Point3::new(0.0, 0.0, 0.0), 15.0);
    let box2 = box_at(Point3::new(3.0, 7.0, 0.0), 15.0);
    let mut fuse = BRepAlgoAPIFuse::new();
    fuse.set_arguments(&one(&box1));
    fuse.set_tools(&one(&box2));
    fuse.set_run_parallel(false);
    fuse.build().unwrap();
    assert!(!fuse.has_errors());
    assert!(!fuse.shape().is_null());
    let faces = |count| [(ShapeEnum::Wire, count), (ShapeEnum::Face, count)];
    expect_counts(
        fuse.shape(),
        &[(ShapeEnum::Vertex, 20), (ShapeEnum::Edge, 32)],
    );
    expect_counts(fuse.shape(), &faces(14));

    fuse.simplify_result(true, true, precision::angular())
        .unwrap();
    expect_counts(
        fuse.shape(),
        &[(ShapeEnum::Vertex, 16), (ShapeEnum::Edge, 24)],
    );
    expect_counts(fuse.shape(), &faces(10));
}

// Equivalent to tests/boolean/simplify/A2.
#[test]
fn BRepAlgoAPISimplifyTest__A2_SectionThroughSewnShell_SimplifiesEdges() {
    let shell = MakeThreeFaceShell();
    let cutting_plane = Plane3::new(Point3::new(0.0, 5.0, 0.0), gp::dy()).unwrap();
    let cutting_face = MakePlaneFace(cutting_plane, (-100.0, 100.0, -100.0, 100.0));
    let mut section = BRepAlgoAPISection::new();
    section.set_arguments(&one(&shell));
    section.set_tools(&one(&cutting_face));
    section.set_run_parallel(false);
    section.build().unwrap();
    assert!(!section.has_errors());
    assert!(!section.shape().is_null());
    expect_counts(
        section.shape(),
        &[(ShapeEnum::Vertex, 4), (ShapeEnum::Edge, 3)],
    );
    assert!((GetLength(section.shape()) - 60.0).abs() <= precision::confusion());

    section
        .simplify_result(true, true, precision::angular())
        .unwrap();
    expect_counts(
        section.shape(),
        &[(ShapeEnum::Vertex, 2), (ShapeEnum::Edge, 1)],
    );
    assert!((GetLength(section.shape()) - 60.0).abs() <= precision::confusion());
}

fn surface_and_volume(shape: &Shape) -> (f64, f64) {
    let mut surface = GProps::default();
    let mut volume = GProps::default();
    BRepGProp::surface_properties(shape, &mut surface, false, false).unwrap();
    BRepGProp::volume_properties(shape, &mut volume, false, false, false).unwrap();
    (surface.mass(), volume.mass())
}

// Equivalent to tests/boolean/simplify/A4.
#[test]
fn BRepAlgoAPISimplifyTest__A4_SequentialTopEdgeCutsPreserveTopology() {
    let mut plate = BRepPrimAPIMakeBox::at_point(Point3::new(0., 0., 0.), 100., 20., 20.)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    for index in 0..100 {
        let x = index as f64 * 0.9;
        let tool = BRepPrimAPIMakeBox::at_point(Point3::new(x, 0., 15.), 10., 10., 10.)
            .unwrap()
            .shape()
            .unwrap()
            .clone();
        let mut cut =
            martensite_tkbo::brep_algo_api::BRepAlgoAPICut::from_shapes(&plate, &tool).unwrap();
        assert!(cut.is_done(), "iteration {index}: cut completion");
        cut.simplify_result(true, true, precision::angular())
            .unwrap();
        plate = cut.shape().clone();
        expect_counts(
            &plate,
            &[
                (ShapeEnum::Vertex, 14),
                (ShapeEnum::Edge, 21),
                (ShapeEnum::Wire, 9),
                (ShapeEnum::Face, 9),
                (ShapeEnum::Shell, 1),
                (ShapeEnum::Solid, 1),
            ],
        );
    }
    let (surface, volume) = surface_and_volume(&plate);
    assert!((surface - 8800.).abs() / 8800. <= 0.01, "surface {surface}");
    assert!((volume - 35045.).abs() / 35045. <= 0.01, "volume {volume}");
}

// Equivalent to tests/boolean/simplify/A3.
#[test]
fn BRepAlgoAPISimplifyTest__A3_SequentialRimCylinderCutsPreserveTopology() {
    let mut plate = BRepPrimAPIMakeCylinder::new(50., 10.)
        .shape()
        .unwrap()
        .clone();
    for index in 0..100 {
        let angle = std::f64::consts::TAU * index as f64 / 100.;
        let tool_axis = Axis2::new(
            Point3::new(
                50. * martensite_cmath::cos(angle),
                50. * martensite_cmath::sin(angle),
                -5.,
            ),
            gp::dz(),
        )
        .unwrap();
        let tool = BRepPrimAPIMakeCylinder::with_axes(tool_axis, 10., 10.)
            .shape()
            .unwrap()
            .clone();
        let mut cut =
            martensite_tkbo::brep_algo_api::BRepAlgoAPICut::from_shapes(&plate, &tool).unwrap();
        assert!(cut.is_done(), "iteration {index}: cut completion");
        cut.simplify_result(true, true, precision::angular())
            .unwrap();
        plate = cut.shape().clone();
    }
    expect_counts(
        &plate,
        &[
            (ShapeEnum::Vertex, 202),
            (ShapeEnum::Edge, 303),
            (ShapeEnum::Wire, 105),
            (ShapeEnum::Face, 104),
            (ShapeEnum::Shell, 1),
            (ShapeEnum::Solid, 1),
        ],
    );
    let (surface, volume) = surface_and_volume(&plate);
    assert!(
        (surface - 18541.7).abs() / 18541.7 <= 0.01,
        "surface {surface}"
    );
    assert!(
        (volume - 64444.2).abs() / 64444.2 <= 0.01,
        "volume {volume}"
    );
}
