//! Native replay of the OCC22695 degenerate-corner 2D fillet regression.
#![allow(non_snake_case)]

use martensite_tkbrep::{
    topexp,
    topods::{self, Edge, Face, Shape},
    toptools::IndexedMapOfShape,
};
use martensite_tkernel::message::ProgressRange;
use martensite_tkfillet::{brep_fillet_api::BRepFilletAPIMakeFillet2d, chfi2d::common_vertex};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::{
    brep_builder_api::{
        BRepBuilderAPIMakeFace, BRepBuilderAPIMakePolygon, BRepBuilderAPIMakeShape,
    },
    brep_check::BRepCheckAnalyzer,
    brep_gprop::BRepGProp,
};

const SOURCE: &str = "tests/bugs/modalg_2/bug22695";

#[test]
fn Modalg2__Bug22695() {
    let source = square_face();
    let result = [[1, 2], [3, 4], [1, 5], [4, 5]]
        .into_iter()
        .fold(source, |face, edges| fillet(&face, edges));
    let result = result.into_shape();
    assert_valid(&result);
    assert_perimeter(&result);
}

fn square_face() -> Face {
    let mut polygon = BRepBuilderAPIMakePolygon::new();
    for (x, y) in [(0., 0.), (10., 0.), (10., 10.), (0., 10.)] {
        polygon.add_point(Point3::new(x, y, 0.)).unwrap();
    }
    polygon.close().unwrap();
    BRepBuilderAPIMakeFace::from_wire(&polygon.wire().unwrap(), false)
        .and_then(|maker| maker.face())
        .unwrap()
}

fn fillet(face: &Face, source_edges: [usize; 2]) -> Face {
    let edges = edges(face);
    let corner = common_vertex(&edges[source_edges[0] - 1], &edges[source_edges[1] - 1])
        .expect("source chfi2d edges share a vertex");
    let mut maker = BRepFilletAPIMakeFillet2d::with_face(face).unwrap();
    maker.add_fillet(&corner, 5.).unwrap();
    maker.build(&ProgressRange::default()).unwrap();
    assert!(maker.is_done(), "{SOURCE}: chfi2d result incomplete");
    topods::face(maker.shape().unwrap()).unwrap()
}

fn edges(face: &Face) -> Vec<Edge> {
    let mut indexed = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(face, ShapeEnum::Edge, &mut indexed).unwrap();
    (1..=indexed.extent())
        .map(|index| topods::edge(indexed.find_key(index).unwrap()).unwrap())
        .collect()
}

fn assert_valid(shape: &Shape) {
    assert!(
        BRepCheckAnalyzer::new(shape, true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );
}

fn assert_perimeter(shape: &Shape) {
    let mut properties = GProps::default();
    BRepGProp::linear_properties(shape, &mut properties, false, false).unwrap();
    assert!(
        (properties.mass() - 31.4159).abs() <= 1.0e-4,
        "{SOURCE}: checkprops result -l 31.4159; got {}",
        properties.mass()
    );
}
