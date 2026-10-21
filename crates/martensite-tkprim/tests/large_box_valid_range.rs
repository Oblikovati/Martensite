//! Original huge-coordinate box and edge-range limits from bugs/modalg_7/bug30154_2.
use martensite_tkbrep::{
    brep::BRepTool,
    topexp,
    topods::{self, Edge, Shape},
    toptools::IndexedMapOfShape,
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::{brep_builder_api::BRepBuilderAPIMakeShape, brep_lib};

fn huge_coordinate_box() -> Shape {
    BRepPrimAPIMakeBox::at_point(
        Point3::new(-94190864., -46229000., -17178478.4),
        519231048.,
        93653000.,
        120025348.8,
    )
    .unwrap()
    .shape()
    .unwrap()
    .clone()
}

fn assert_original_valid_range(edge: &Edge) {
    let curve = BRepTool::curve_with_location(edge).unwrap();
    let range = brep_lib::find_valid_range_of_edge(edge).unwrap();
    assert!(range.is_valid);
    let first_delta = range.first.unwrap() - curve.first;
    let last_delta = curve.last - range.last.unwrap();
    assert!(
        (1e-7..=3e-7).contains(&first_delta),
        "first delta {first_delta}"
    );
    assert!(
        (1e-7..=3e-7).contains(&last_delta),
        "last delta {last_delta}"
    );
}

#[test]
fn huge_coordinate_box_edges_keep_original_valid_range_limits() {
    crate::isolated_primitive_case::run_primitive_case(
        "large_box_valid_range::huge_coordinate_box_edges_keep_original_valid_range_limits",
        huge_coordinate_box_edges_keep_original_valid_range_limits_body,
    );
}

fn huge_coordinate_box_edges_keep_original_valid_range_limits_body() {
    let shape = huge_coordinate_box();
    let mut edges = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(&shape, ShapeEnum::Edge, &mut edges).unwrap();
    assert_eq!(edges.extent(), 12);
    for shape in edges.iter() {
        assert_original_valid_range(&topods::edge(shape).unwrap());
    }
}
