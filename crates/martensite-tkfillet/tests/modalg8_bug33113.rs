//! Regression of `tests/bugs/modalg_8/bug33113`.
#![allow(non_snake_case)]

use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_tools,
    topexp::Explorer,
    topods::{self, Edge, Shape},
};
use martensite_tkernel::{message::ProgressRange, storage::IoStream};
use martensite_tkfillet::{brep_fillet_api::BRepFilletAPIMakeFillet, chfi3d::FilletShape};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::{
    brep_builder_api::BRepBuilderAPIMakeShape, brep_check::BRepCheckAnalyzer,
};

const SOURCE: &str = "tests/bugs/modalg_8/bug33113";

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
    .expect("source restore bug33113.brep");
    shape
}

fn exploded_edges(shape: &Shape) -> Vec<Edge> {
    let mut explorer =
        Explorer::new(shape, ShapeEnum::Edge, ShapeEnum::Shape).expect("source explode sh e");
    let mut edges = Vec::new();
    while explorer.more() {
        edges.push(topods::edge(explorer.current()).expect("source explode sh e edge"));
        explorer.next().expect("source explode sh e next");
    }
    edges
}

fn first_solid(shape: &Shape) -> Shape {
    let explorer =
        Explorer::new(shape, ShapeEnum::Solid, ShapeEnum::Shape).expect("source explode sh So");
    assert!(
        explorer.more(),
        "{SOURCE}: source explode sh So must find sh_1"
    );
    explorer.current().clone()
}

#[test]
fn Modalg8__Bug33113_FilletBuildsWithoutCrash() {
    let source = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/bug33113.brep"
    ));
    let edge = exploded_edges(&source)
        .get(3)
        .cloned()
        .expect("tests/bugs/modalg_8/bug33113: source copy sh_4 e");
    let solid = first_solid(&source);

    let mut fillet = BRepFilletAPIMakeFillet::new(&solid, FilletShape::Rational)
        .expect("source fillet res s 0.1 e setup");
    fillet
        .add_with_radius(0.1, &edge)
        .expect("source fillet res s 0.1 e add");
    fillet
        .build(&ProgressRange::default())
        .expect("source fillet res s 0.1 e must not crash");
    assert!(
        fillet.is_done(),
        "{SOURCE}: source fillet res is incomplete"
    );
    let result = fillet.shape().expect("source fillet result");
    let checker =
        BRepCheckAnalyzer::new(result, true, false, false).expect("source checkshape res");
    assert!(
        checker.is_valid().expect("source checkshape validity"),
        "{SOURCE}: checkshape res"
    );
}
