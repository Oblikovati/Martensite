//! Behavior tests of BRepCheck_Vertex and BRepCheck_Edge on shapes built with
//! BRep_Builder (brep_check_support).
mod brep_check_support;

use brep_check_support::{TOL, cube, solid_of, square, vertex};
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{Edge, Face, Shape, Wire};
use martensite_tkernel::ncollection::{Array1, Array2};
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::Geom2dBezierCurve;
use martensite_tkg3d::geom::{GeomBezierCurve, GeomBezierSurface, GeomCurve, GeomLine};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::gp::{Direction3, Point2, Point3};
use martensite_tkmath::poly::{PolygonOnTriangulation, Triangle, Triangulation};
use martensite_tktopalgo::brep_check::{
    BRepCheckEdge, BRepCheckResult, BRepCheckStatus, BRepCheckVertex,
};

use BRepCheckStatus::*;

fn on(checker: &dyn BRepCheckResult, context: &Shape) -> Vec<BRepCheckStatus> {
    checker.in_context(context).unwrap();
    checker
        .status_on_shape_of(context)
        .unwrap()
        .iter()
        .copied()
        .collect()
}

fn own(checker: &dyn BRepCheckResult) -> Vec<BRepCheckStatus> {
    checker.status().unwrap().iter().copied().collect()
}

#[test]
fn vertex_checks_the_ends_of_its_edges() {
    let sq = square(1.0, &[], &[], 0.0);
    let checker = BRepCheckVertex::new(&sq.vertices[1]).unwrap();
    assert_eq!(own(&checker), [NoError]);
    assert_eq!(on(&checker, &sq.edges[&(0, 1)]), [NoError]);
    assert_eq!(on(&checker, &sq.faces[0]), [NoError]);
    assert_eq!(on(&checker, &sq.edges[&(2, 3)]), [SubshapeNotInShape]);
    // A vertex away from the curve end, beyond its tolerance.
    let far = vertex(Point3::new(1.0, 1e-3, 0.0), TOL);
    let e = sq.edges[&(0, 1)].clone();
    let b = BRepBuilder::default();
    let mut other = Edge::default();
    b.make_edge(&mut other).unwrap();
    let line: Handle<dyn GeomCurve> = Handle::new(GeomLine::from_point(
        Point3::new(0.0, 0.0, 0.0),
        Direction3::new(1.0, 0.0, 0.0).unwrap(),
    ))
    .cast();
    b.update_edge_curve(&other, &line, TOL).unwrap();
    b.range(&other, 0.0, 1.0, false).unwrap();
    b.add(&mut other, &far.oriented(Orientation::Reversed))
        .unwrap();
    let checker = BRepCheckVertex::new(&far).unwrap();
    assert_eq!(on(&checker, &other), [InvalidPointOnCurve]);
    assert_eq!(on(&checker, &e), [SubshapeNotInShape]);
    assert!(checker.tolerance().unwrap() > 0.0);
}

#[test]
fn vertex_parameter_on_its_edge_moves_the_edge_range() {
    let sq = square(1.0, &[], &[], 0.0);
    let v = &sq.vertices[0];
    let base = BRepCheckVertex::new(v).unwrap().tolerance().unwrap();
    assert_eq!(base, (TOL * TOL * 1.05).sqrt());
    // A FORWARD end vertex sets the first parameter of the curves rather than adding a
    // point representation: the tolerance keeps its value, the curve start moves.
    BRepBuilder::default()
        .update_vertex_on_edge(v, 0.25, &sq.edges[&(0, 1)], TOL)
        .unwrap();
    let checker = BRepCheckVertex::new(v).unwrap();
    assert_eq!(checker.tolerance().unwrap(), base);
    assert_eq!(on(&checker, &sq.edges[&(0, 1)]), [InvalidPointOnCurve]);
    // An inner vertex gets a point representation, which the tolerance measures.
    let inner = vertex(Point3::new(0.5, 0.0, 0.0), TOL);
    BRepBuilder::default()
        .update_vertex_on_edge(&inner, 0.75, &sq.edges[&(1, 2)], TOL)
        .unwrap();
    let tolerance = BRepCheckVertex::new(&inner).unwrap().tolerance().unwrap();
    let expected =
        (Point3::new(0.5, 0.0, 0.0).square_distance(&Point3::new(1.0, 0.75, 0.0)) * 1.05).sqrt();
    assert!((tolerance - expected).abs() < 1e-15, "{tolerance}");
}

#[test]
fn edge_minimum_reports_the_reference_curve_defects() {
    let b = BRepBuilder::default();
    let mut empty = Edge::default();
    b.make_edge(&mut empty).unwrap();
    assert_eq!(own(&BRepCheckEdge::new(&empty).unwrap()), [No3DCurve]);
    let sq = square(1.0, &[], &[], 0.0);
    let e = &sq.edges[&(1, 2)];
    b.same_range(e, false).unwrap();
    assert_eq!(
        own(&BRepCheckEdge::new(e).unwrap()),
        [InvalidSameParameterFlag]
    );
    let sq = square(1.0, &[], &[], 0.0);
    let e = &sq.edges[&(1, 2)];
    b.range(e, 0.5, 0.25, true).unwrap();
    assert_eq!(own(&BRepCheckEdge::new(e).unwrap()), [InvalidRange]);
    let checker = BRepCheckEdge::new(&sq.edges[&(0, 1)]).unwrap();
    assert!(checker.geometric_controls() && !checker.is_exact_method());
    checker.set_status(InvalidPolygonOnTriangulation).unwrap();
    assert_eq!(own(&checker), [InvalidPolygonOnTriangulation]);
}

#[test]
fn edge_in_face_validates_its_pcurve() {
    for (shift, expected) in [
        (0.0, vec![NoError]),
        (1e-3, vec![InvalidCurveOnSurface, InvalidSameParameterFlag]),
    ] {
        let sq = square(1.0, &[], &[(0, 1)], shift);
        let checker = BRepCheckEdge::new(&sq.edges[&(0, 1)]).unwrap();
        assert_eq!(on(&checker, &sq.faces[0]), expected);
        let exact = BRepCheckEdge::new(&sq.edges[&(0, 1)]).unwrap();
        exact.set_exact_method(true);
        assert_eq!(on(&exact, &sq.faces[0]), expected);
        let blind = BRepCheckEdge::new(&sq.edges[&(0, 1)]).unwrap();
        blind.set_geometric_controls(false);
        assert_eq!(on(&blind, &sq.faces[0]), [NoError]);
        assert!((checker.tolerance().unwrap() - shift * 1.05).abs() < 1e-12);
    }
    let sq = square(1.0, &[], &[], 0.0);
    BRepBuilder::default()
        .same_parameter(&sq.edges[&(0, 1)], false)
        .unwrap();
    let checker = BRepCheckEdge::new(&sq.edges[&(0, 1)]).unwrap();
    assert_eq!(on(&checker, &sq.faces[0]), [InvalidSameParameterFlag]);
    assert_eq!(on(&checker, &sq.wires[0][0]), [NoError]);
}

fn bezier_wall() -> Face {
    let mut face = Face::default();
    let (a, b) = (Point3::new(0., 0., 0.), Point3::new(1., 0., 0.));
    let mut net = [a, Point3::new(0., 1., 0.), b, Point3::new(1., 1., 0.5)];
    let patch = GeomBezierSurface::new(&Array2::from_buffer(&mut net, 2, 2).unwrap()).unwrap();
    BRepBuilder::default().make_face_on_surface(&mut face, &Handle::new(patch).into_upcast(), TOL).unwrap();
    face
}

fn bezier_wall_edge(shift: f64, trace_v: f64) -> (Edge, Face) {
    let builder = BRepBuilder::default();
    let mut face = bezier_wall();
    let (mut edge, mut wire) = (Edge::default(), Wire::default());
    let rail = [Point3::new(0., 0., 0.), Point3::new(1., 0., shift)];
    let curve = GeomBezierCurve::new(&Array1::from_shared(&rail, 1)).unwrap();
    let trim = [Point2::new(0., trace_v), Point2::new(1., trace_v)];
    let trace = Geom2dBezierCurve::new(&Array1::from_shared(&trim, 1)).unwrap();
    builder.make_edge_with_curve(&mut edge, &Handle::new(curve).into_upcast(), TOL).unwrap();
    builder.update_edge_pcurve_on_face(&edge, &Handle::new(trace).into_upcast(), &face, TOL).unwrap();
    builder.range(&edge, 0., 1., false).unwrap();
    builder.make_wire(&mut wire);
    builder.add(&mut wire, &edge).unwrap();
    builder.add(&mut face, &wire).unwrap();
    (edge, face)
}

#[test]
fn exact_bezier_boundary_preserves_edge_status_and_checks_changed_geometry() {
    for (shift, trace_v, expected) in [
        (0., 0., vec![NoError]),
        (1e-3, 0., vec![InvalidCurveOnSurface, InvalidSameParameterFlag]),
        (0., 1., vec![InvalidCurveOnSurface, InvalidSameParameterFlag]),
    ] {
        let (edge, face) = bezier_wall_edge(shift, trace_v);
        for exact in [false, true] {
            let check = BRepCheckEdge::new(&edge).unwrap();
            check.set_exact_method(exact);
            assert_eq!(on(&check, &face), expected);
        }
    }
}

#[test]
fn edge_in_solid_is_shared_by_two_faces() {
    let closed = cube(&[0, 1, 2, 3, 4, 5]);
    let solid = solid_of(&closed.shell(&closed.faces));
    let checker = BRepCheckEdge::new(&closed.edges[&(0, 1)]).unwrap();
    assert_eq!(on(&checker, &solid), [NoError]);
    let open = cube(&[0, 2]);
    let solid = solid_of(&open.shell(&open.faces));
    assert_eq!(
        on(&BRepCheckEdge::new(&open.edges[&(0, 1)]).unwrap(), &solid),
        [NoError]
    );
    assert_eq!(
        on(&BRepCheckEdge::new(&open.edges[&(1, 2)]).unwrap(), &solid),
        [FreeEdge]
    );
}

#[test]
fn edge_tolerance_of_a_single_representation_is_confusion() {
    let b = BRepBuilder::default();
    let mut e = Edge::default();
    b.make_edge(&mut e).unwrap();
    let line: Handle<dyn GeomCurve> = Handle::new(GeomLine::from_point(
        Point3::new(0.0, 0.0, 0.0),
        Direction3::new(0.0, 1.0, 0.0).unwrap(),
    ))
    .cast();
    b.update_edge_curve(&e, &line, TOL).unwrap();
    assert_eq!(
        BRepCheckEdge::new(&e).unwrap().tolerance().unwrap(),
        precision::confusion()
    );
}

#[test]
fn edge_polygon_on_triangulation_must_follow_the_curve() {
    for (dz, expected) in [(0.0, NoError), (0.5, InvalidPolygonOnTriangulation)] {
        let sq = square(1.0, &[], &[], 0.0);
        let mut nodes = Array1::<Point3>::with_bounds(1, 3).unwrap();
        nodes[1] = Point3::new(0.0, 0.0, 0.0);
        nodes[2] = Point3::new(1.0, 0.0, dz);
        nodes[3] = Point3::new(1.0, 1.0, 0.0);
        let mut triangles = Array1::<Triangle>::with_bounds(1, 1).unwrap();
        triangles[1] = Triangle::new(1, 2, 3);
        let triangulation = Handle::new(Triangulation::from_nodes(&nodes, &triangles).unwrap());
        let mut indices = Array1::<i32>::with_bounds(1, 2).unwrap();
        indices[1] = 1;
        indices[2] = 2;
        let mut parameters = Array1::<f64>::with_bounds(1, 2).unwrap();
        parameters[1] = 0.0;
        parameters[2] = 1.0;
        let polygon = Handle::new(PolygonOnTriangulation::from_nodes_and_parameters(
            &indices,
            &parameters,
        ));
        let e = &sq.edges[&(0, 1)];
        BRepBuilder::default()
            .update_edge_polygon_on_triangulation(e, &polygon, &triangulation, &Default::default())
            .unwrap();
        let checker = BRepCheckEdge::new(e).unwrap();
        assert_eq!(checker.check_polygon_on_triangulation(e).unwrap(), expected);
    }
}
