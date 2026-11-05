//! Native replay of the OCC140 attached-shape fillet case.
#![allow(non_snake_case)]

use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_tools, topexp,
    topexp::Explorer,
    topods::{self, Edge, Shape},
    toptools::IndexedMapOfShape,
};
use martensite_tkernel::{message::ProgressRange, storage::IoStream};
use martensite_tkfillet::{brep_fillet_api::BRepFilletAPIMakeFillet, chfi3d::FilletShape};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{Point3, Transform3};
use martensite_tktopalgo::{
    brep_builder_api::{BRepBuilderAPIMakeShape, BRepBuilderAPITransform},
    brep_check::BRepCheckAnalyzer,
    brep_gprop::BRepGProp,
    brep_lib,
};

const SCALE: f64 = 5.;
const RADIUS: f64 = 4.5 * SCALE;
const AREA: f64 = 3_654_240.;

fn restored_shape(bytes: &[u8]) -> Shape {
    let header_end = bytes
        .iter()
        .position(u8::is_ascii_whitespace)
        .expect("BREP fixture header");
    let mut shape = Shape::default();
    let mut stream = IoStream::from_bytes(bytes[header_end..].to_vec());
    brep_tools::read(
        &mut shape,
        &mut stream,
        &BRepBuilder::default(),
        &ProgressRange::default(),
    )
    .expect("source restore shading_137.brep");
    shape
}

fn scaled_shape(shape: &Shape) -> Shape {
    let mut scale = Transform3::default();
    scale
        .set_scale(Point3::default(), SCALE)
        .expect("source tscale s 0 0 0 SCALE1");
    BRepBuilderAPITransform::with_shape(shape, scale, false, false)
        .expect("source tscale result")
        .shape()
        .expect("source scaled shape")
        .clone()
}

fn source_edges(shape: &Shape) -> [Edge; 3] {
    let mut edges = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Edge, &mut edges).expect("source explode s e");
    [4, 5, 1].map(|index| {
        topods::edge(edges.find_key(index).expect("source edge index")).expect("source edge")
    })
}

fn source_result(shape: &Shape) -> Shape {
    let mut blend =
        BRepFilletAPIMakeFillet::new(shape, FilletShape::Rational).expect("source blend sh setup");
    for edge in source_edges(shape) {
        blend
            .add_with_radius(RADIUS, &edge)
            .expect("source blend selected edge");
    }
    blend
        .build(&ProgressRange::default())
        .expect("source blend build");
    assert!(blend.is_done(), "OCC140: blend incomplete");
    blend.shape().expect("source blend result").clone()
}

fn first_solid(shape: &Shape) -> Shape {
    let explorer =
        Explorer::new(shape, ShapeEnum::Solid, ShapeEnum::Shape).expect("source explode sh so");
    assert!(
        explorer.more(),
        "OCC140: source explode sh so must find sh_1"
    );
    explorer.current().clone()
}

#[test]
fn Modalg1__Bug140() {
    let input = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/shading_137.brep"
    ));
    let result = first_solid(&source_result(&scaled_shape(&input)));
    brep_lib::same_parameter_shape(&result, 1.0e-7, true).expect("source fsameparameter result");
    let mut properties = GProps::default();
    BRepGProp::surface_properties(&result, &mut properties, false, false)
        .expect("source checkprops");
    let check = BRepCheckAnalyzer::new(&result, true, false, false).expect("source checkshape");
    assert!(
        (properties.mass() - AREA).abs() / AREA <= 0.01,
        "source area mismatch"
    );
    assert!(
        check.is_valid().expect("source validity"),
        "OCC140 result invalid"
    );
}
