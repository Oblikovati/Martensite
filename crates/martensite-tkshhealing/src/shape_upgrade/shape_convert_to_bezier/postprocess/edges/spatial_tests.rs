use super::*;
use martensite_tkg2d::geom2d::arrays;
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeEdge;

fn straight_bezier_edge() -> (Edge, Handle<GeomBezierCurve>) {
    let mut poles = arrays::one_based(2).unwrap();
    arrays::put(&mut poles, 1, Point3::new(0., 0., 0.)).unwrap();
    arrays::put(&mut poles, 2, Point3::new(2., 0., 0.)).unwrap();
    let curve = Handle::new(GeomBezierCurve::new(&poles).unwrap());
    let edge = BRepBuilderAPIMakeEdge::from_curve(&curve.clone().cast())
        .unwrap()
        .edge()
        .unwrap();
    (edge, curve)
}

#[test]
fn reversed_edge_retains_parameter_ordered_bezier_poles() {
    let (mut edge, curve) = straight_bezier_edge();
    edge.set_orientation(Orientation::Reversed);
    process_edge(&ShapeUpgradeShapeConvertToBezier::empty(), &edge).unwrap();
    assert_eq!(curve.pole(1).unwrap(), Point3::new(0., 0., 0.));
    assert_eq!(curve.pole(2).unwrap(), Point3::new(2., 0., 0.));
}

#[test]
fn unit_range_spatial_postprocess_marks_edge_modified_and_unchecked() {
    let (edge, _) = straight_bezier_edge();
    edge.set_modified(false).unwrap();
    edge.set_checked(true).unwrap();
    process_edge(&ShapeUpgradeShapeConvertToBezier::empty(), &edge).unwrap();
    assert!(edge.modified().unwrap());
    assert!(!edge.checked().unwrap());
}

#[test]
fn unit_range_locked_edge_reports_the_final_publication_failure() {
    let (edge, _) = straight_bezier_edge();
    edge.set_locked(true).unwrap();
    let failure = process_edge(&ShapeUpgradeShapeConvertToBezier::empty(), &edge)
        .expect_err("locked edge must reject the final curve publication");
    assert_eq!(failure.exception_type(), "TopoDS_LockedShape");
}

fn triangular_face_with_reversed_bezier() -> martensite_tkbrep::topods::Face {
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;
    let (mut curved, _) = straight_bezier_edge();
    curved.reverse();
    let edges = [
        curved,
        line_edge(Point3::new(0., 0., 0.), Point3::new(0., 2., 0.)),
        line_edge(Point3::new(0., 2., 0.), Point3::new(2., 0., 0.)),
    ];
    let wire = triangle_wire(edges);
    BRepBuilderAPIMakeFace::from_plane_wire(martensite_tkmath::gp::Plane3::default(), &wire, true)
        .unwrap()
        .face()
        .unwrap()
}

fn triangle_wire(edges: [Edge; 3]) -> martensite_tkbrep::topods::Wire {
    use martensite_tkbrep::topods::{Builder, Wire};
    let mut wire = Wire::default();
    Builder.make_wire(&mut wire);
    for edge in edges {
        Builder
            .add(&mut wire.clone().into_shape(), &edge.into_shape())
            .unwrap();
    }
    wire
}

fn line_edge(first: Point3, last: Point3) -> Edge {
    BRepBuilderAPIMakeEdge::from_points(first, last)
        .unwrap()
        .edge()
        .unwrap()
}

#[test]
fn public_conversion_keeps_spatial_endpoints_on_parameter_ordered_vertices() {
    use martensite_tkbrep::topexp::Explorer;
    use martensite_tkg3d::top_abs::ShapeEnum;
    let converter =
        ShapeUpgradeShapeConvertToBezier::new(triangular_face_with_reversed_bezier().into_shape());
    converter.set_3d_conversion(true);
    converter.perform(true).unwrap();
    let mut edges = Explorer::new(&converter.result(), ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
    let mut count = 0;
    while edges.more() {
        assert_parameter_endpoints(edges.current());
        count += 1;
        edges.next().unwrap();
    }
    assert_eq!(count, 3);
}

fn assert_parameter_endpoints(shape: &martensite_tkbrep::topods::Shape) {
    use martensite_tkbrep::{topexp, topods};
    let edge = topods::edge(shape).unwrap();
    let (curve, first, last) = BRepTool::curve(&edge).unwrap();
    let (v1, v2) = topexp::vertices(&edge, false);
    for (parameter, vertex) in [(first, v1), (last, v2)] {
        let distance = curve
            .value(parameter)
            .unwrap()
            .distance(&BRepTool::pnt(&vertex).unwrap());
        assert!(
            distance < precision::confusion(),
            "endpoint distance {distance}"
        );
    }
}
