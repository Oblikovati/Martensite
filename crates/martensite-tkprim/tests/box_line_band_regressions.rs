//! Original line/band command regressions from bugs/moddata_1/buc60852 and buc60889.
use martensite_tkbrep::{brep_adaptor::BRepAdaptorCurve, topods::Edge};
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkmath::{
    bnd::BoundingBox3,
    gp::{Direction3, Line3, Point3},
};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakePolygon, BRepBuilderAPIMakeShape,
    BRepBuilderAPIMakeVertex,
};

fn original_edge(first: Point3, second: Point3) -> Edge {
    let first = BRepBuilderAPIMakeVertex::new(first)
        .unwrap()
        .vertex()
        .unwrap();
    let second = BRepBuilderAPIMakeVertex::new(second)
        .unwrap()
        .vertex()
        .unwrap();
    BRepBuilderAPIMakeEdge::from_vertices(&first, &second)
        .unwrap()
        .edge()
        .unwrap()
}

fn original_box_bounds() -> BoundingBox3 {
    let mut bounds = BoundingBox3::default();
    bounds.update_bounds([0., 0., 0.], [30., 10., 20.]);
    bounds
}

fn original_line_output(edge: &Edge) -> &'static str {
    let line = BRepAdaptorCurve::with_edge(edge).unwrap().line().unwrap();
    if original_box_bounds().is_out_line(line) {
        return "Line that lies on edge does not intersect the box\n";
    }
    "Line that lies on edge intersects the box\n"
}

fn original_band_output(first: Point3, second: Point3, edge: &Edge) -> &'static str {
    let direction = BRepAdaptorCurve::with_edge(edge)
        .unwrap()
        .line()
        .unwrap()
        .direction();
    if original_box_bounds().is_out_segment(first, second, direction) {
        return "The band lies out of the box\n";
    }
    "The band intersects the box\n"
}

fn original_auxiliary_box() {
    BRepPrimAPIMakeBox::at_point(Point3::default(), 30., 10., 20.)
        .unwrap()
        .shape()
        .unwrap();
}

fn original_auxiliary_line(first: Point3, second: Point3) -> Line3 {
    let direction = Direction3::new(
        second.x() - first.x(),
        second.y() - first.y(),
        second.z() - first.z(),
    )
    .unwrap();
    Line3::new(first, direction)
}

fn original_line_setup(first: Point3, second: Point3) -> Edge {
    let edge = original_edge(first, second);
    // The script's auxiliary line starts at the first vertex but runs opposite to the edge.
    let direction = original_auxiliary_line(second, first).direction();
    let _line = Line3::new(first, direction);
    edge
}

#[test]
fn original_edge_lines_retain_exact_box_intersection_messages() {
    crate::isolated_primitive_case::run_primitive_case(
        "box_line_band_regressions::original_edge_lines_retain_exact_box_intersection_messages",
        original_edge_lines_retain_exact_box_intersection_messages_body,
    );
}

fn original_edge_lines_retain_exact_box_intersection_messages_body() {
    let first = original_line_setup(Point3::new(-20., -5., 10.), Point3::new(50., 15., 10.));
    let second = original_line_setup(Point3::new(-20., -10., 10.), Point3::new(50., -10., 10.));
    original_auxiliary_box();
    assert_eq!(
        original_line_output(&first),
        "Line that lies on edge intersects the box\n"
    );
    assert_eq!(
        original_line_output(&second),
        "Line that lies on edge does not intersect the box\n"
    );
}

fn original_band_setup(first: Point3, second: Point3, third: Point3) -> Edge {
    let mut polyline = BRepBuilderAPIMakePolygon::new();
    polyline.add_point(first).unwrap();
    polyline.add_point(second).unwrap();
    polyline.wire().unwrap();
    let direction = original_auxiliary_line(first, third).direction();
    let _lines = [Line3::new(first, direction), Line3::new(second, direction)];
    original_edge(first, third)
}

#[test]
fn original_edge_directed_bands_retain_exact_box_intersection_messages() {
    crate::isolated_primitive_case::run_primitive_case(
        "box_line_band_regressions::original_edge_directed_bands_retain_exact_box_intersection_messages",
        original_edge_directed_bands_retain_exact_box_intersection_messages_body,
    );
}

fn original_edge_directed_bands_retain_exact_box_intersection_messages_body() {
    let first = [Point3::new(-30., -10., 5.), Point3::new(-30., -10., 15.)];
    let second = [Point3::new(30., 10., -5.), Point3::new(30., 10., -15.)];
    let first_edge = original_band_setup(first[0], first[1], Point3::new(-15., -5., 5.));
    let second_edge = original_band_setup(second[0], second[1], Point3::new(0., 10., -5.));
    original_auxiliary_box();
    assert_eq!(
        original_band_output(first[0], first[1], &first_edge),
        "The band intersects the box\n"
    );
    assert_eq!(
        original_band_output(second[0], second[1], &second_edge),
        "The band lies out of the box\n"
    );
}
