//! Native replay of the OCC487 two-edge fillet regression.
#![allow(non_snake_case)]

use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_tools, topexp,
    topods::{self, Edge, Shape},
    toptools::IndexedMapOfShape,
};
use martensite_tkernel::{message::ProgressRange, storage::IoStream};
use martensite_tkfillet::{brep_fillet_api::BRepFilletAPIMakeFillet, chfi3d::FilletShape};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::{
    brep_builder_api::BRepBuilderAPIMakeShape, brep_check::BRepCheckAnalyzer, brep_gprop::BRepGProp,
};

const SOURCE: &str = "tests/bugs/modalg_2/bug487";

fn restore_source(bytes: &[u8]) -> Shape {
    let header_end = bytes
        .iter()
        .position(u8::is_ascii_whitespace)
        .expect("source CFI fixture header");
    let mut shape = Shape::default();
    let mut stream = IoStream::from_bytes(bytes[header_end..].to_vec());
    brep_tools::read(
        &mut shape,
        &mut stream,
        &BRepBuilder::default(),
        &ProgressRange::default(),
    )
    .expect("source restore CFI_pro15441.rle");
    shape
}

fn source_edges(shape: &Shape) -> [Edge; 2] {
    let mut edges = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Edge, &mut edges).expect("source explode m e");
    [36, 21].map(|index| {
        topods::edge(edges.find_key(index).expect("source selected edge"))
            .expect("source edge type")
    })
}

fn source_blend(shape: &Shape, edges: [Edge; 2]) -> Shape {
    let mut blend = BRepFilletAPIMakeFillet::new(shape, FilletShape::Rational)
        .expect("source blend result setup");
    for edge in edges {
        blend
            .add_with_radius(2., &edge)
            .expect("source blend result selected edge");
    }
    blend
        .build(&ProgressRange::default())
        .expect("source blend result build");
    assert!(blend.is_done(), "{SOURCE}: blend incomplete");
    first_shell(blend.shape().expect("source blend result"))
}

fn first_shell(shape: &Shape) -> Shape {
    let mut shells = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Shell, &mut shells)
        .expect("source explode result sh");
    shells.find_key(1).expect("source result_1").clone()
}

fn assert_source_area(shape: &Shape) {
    let mut properties = GProps::default();
    BRepGProp::surface_properties(shape, &mut properties, false, false)
        .expect("source checkprops result -s 6265.68");
    let expected = 6_265.68;
    assert!(
        (properties.mass() - expected).abs() / expected <= 0.01,
        "{SOURCE}: expected surface {expected}, got {}",
        properties.mass()
    );
}

#[test]
fn Modalg2__Bug487() {
    let source = restore_source(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CFI_pro15441.rle"
    ));
    let result = source_blend(&source, source_edges(&source));
    let checker =
        BRepCheckAnalyzer::new(&result, true, false, false).expect("source checkshape result");
    assert!(
        checker.is_valid().expect("source checkshape validity"),
        "{SOURCE}: checkshape result"
    );
    assert_source_area(&result);
}
