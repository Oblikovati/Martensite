//! Seam alignment and wire repairs on faces built by hand, with the recorded results of each repair.
use super::{FixRoot, ShapeFixFace, ShapeFixWire};
use crate::shape_build::ShapeBuildReShape;
use crate::shape_extend::{BasicMsgRegistrator, ShapeExtendMsgRegistrator};
use martensite_cmath::{cos, sin};
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::brep_tools;
use martensite_tkbrep::topexp::{self, Explorer};
use martensite_tkbrep::topods::{self, Builder, Edge, Face, Shape, ShapeIterator, Vertex, Wire};
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{Geom2dBSplineCurve, Geom2dCircle, Geom2dCurve, Geom2dLine};
use martensite_tkg3d::geom::{
    GeomBSplineCurve, GeomCircle, GeomCurve, GeomCylindricalSurface, GeomLine, GeomSurface,
    GeomSurfaceOfLinearExtrusion, GeomToroidalSurface,
};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::geom_abs::Shape as Continuity;
use martensite_tkmath::gp::{Axis1, Axis2, Axis2d, Axis3, Direction2, Direction3, Point2, Point3};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeEdge;
use martensite_tktopalgo::brep_gprop::BRepGProp;
use martensite_tktopalgo::brep_lib;
use std::f64::consts::TAU;

const TOLERANCE: f64 = 1e-7;

fn empty_face(surface: &Handle<dyn GeomSurface>) -> Face {
    let mut face = Face::default();
    BRepBuilder::default()
        .make_face_on_surface(&mut face, surface, TOLERANCE)
        .unwrap();
    face
}

fn wire_of(edges: &[&Edge]) -> Wire {
    let mut wire = Wire::default();
    Builder.make_wire(&mut wire);
    for edge in edges {
        Builder
            .add(&mut wire, &(*edge).clone().into_shape())
            .unwrap();
    }
    wire
}

/// A full turn along U at fixed V on a surface of revolution about Z.
fn add_band(face: &mut Face, radius: f64, height: f64, v: f64, forward: bool) {
    let builder = BRepBuilder::default();
    let normal = if forward { 1.0 } else { -1.0 };
    let axis = Axis2::with_x_direction(
        Point3::new(0.0, 0.0, height),
        Direction3::new(0.0, 0.0, normal).unwrap(),
        Direction3::new(1.0, 0.0, 0.0).unwrap(),
    )
    .unwrap();
    let circle: Handle<dyn GeomCurve> = Handle::new(GeomCircle::new(axis, radius).unwrap()).cast();
    let edge = BRepBuilderAPIMakeEdge::from_curve(&circle)
        .unwrap()
        .edge()
        .unwrap();
    let start = Point2::new(if forward { 0.0 } else { TAU }, v);
    let direction = Direction2::new(normal, 0.0).unwrap();
    let pcurve: Handle<dyn Geom2dCurve> =
        Handle::new(Geom2dLine::from_point(start, direction)).cast();
    builder
        .update_edge_pcurve_on_face(&edge, &pcurve, face, TOLERANCE)
        .unwrap();
    builder.range_on_face(&edge, face, 0.0, TAU).unwrap();
    Builder.add(face, &wire_of(&[&edge]).into_shape()).unwrap();
}

fn torus_with_two_bands(major: f64, minor: f64, v1: f64, v2: f64) -> Face {
    let surface: Handle<dyn GeomSurface> =
        Handle::new(GeomToroidalSurface::new(Axis3::default(), major, minor).unwrap()).cast();
    let mut face = empty_face(&surface);
    add_band(
        &mut face,
        major + minor * cos(v1),
        minor * sin(v1),
        v1,
        false,
    );
    add_band(
        &mut face,
        major + minor * cos(v2),
        minor * sin(v2),
        v2,
        true,
    );
    face
}

fn first_face(shape: &Shape) -> Face {
    let explorer = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    topods::face(explorer.current()).unwrap()
}

fn area(face: &Face) -> f64 {
    let mut properties = Default::default();
    BRepGProp::surface_properties_eps(&face.clone().into_shape(), &mut properties, 1e-6, false)
        .unwrap();
    properties.mass()
}

fn close(found: f64, expected: f64, what: &str) {
    assert!(
        (found - expected).abs() <= 1e-6 * expected.abs().max(1.0),
        "{what}: {found}, expected {expected}"
    );
}

fn assert_seam_result(face: Face, bounds: (f64, f64, f64, f64), expected_area: f64) {
    let fixer = ShapeFixFace::new(face.into_shape());
    assert!(fixer.fix_missing_seam().unwrap());
    let result = first_face(&fixer.result());
    let (u1, u2, v1, v2) = brep_tools::uv_bounds(&result).unwrap();
    close(u1, bounds.0, "u min");
    close(u2, bounds.1, "u max");
    close(v1, bounds.2, "v min");
    close(v2, bounds.3, "v max");
    close(area(&result), expected_area, "area");
}

/// The two open wires of a torus are aligned along V, with the second one half a period off the
/// first; aligned along U or by a whole period the face takes the complementary V range.
#[test]
fn torus_seam_aligns_the_wires_along_v_by_half_a_period() {
    let face = torus_with_two_bands(40.0, 8.0, 1.316563313, 5.235986997);
    assert_seam_result(face, (0.0, TAU, 5.235986997, 7.59974862018), 5490.07254993);
}

/// With two open wires a spindle torus is a regular torus: its wires are aligned too.
#[test]
fn spindle_torus_with_two_wires_is_aligned_like_a_regular_one() {
    let face = torus_with_two_bands(2.0, 3.0, 1.590405427, 0.5);
    assert_seam_result(
        face,
        (0.0, TAU, 6.78318530718, 7.87359073418),
        70.5342368027,
    );
}

/// On a reversed face a hole across the seam keeps its orientation, so that it is split by the seam
/// and joins the outer wire.
#[test]
fn hole_across_the_seam_of_a_reversed_face_is_split_by_it() {
    let builder = BRepBuilder::default();
    let surface: Handle<dyn GeomSurface> =
        Handle::new(GeomCylindricalSurface::new(Axis3::default(), 1.0).unwrap()).cast();
    let mut face = empty_face(&surface);
    add_band(&mut face, 1.0, 0.0, 0.0, true);
    add_band(&mut face, 1.0, 1.0, 1.0, false);
    let axis = Axis2d::new(Point2::new(TAU, 0.5), Direction2::X);
    let hole: Handle<dyn Geom2dCurve> =
        Handle::new(Geom2dCircle::from_axis(axis, 0.25, true).unwrap()).cast();
    let mut edge = Edge::default();
    builder.make_edge(&mut edge).unwrap();
    builder
        .update_edge_pcurve_on_face(&edge, &hole, &face, TOLERANCE)
        .unwrap();
    builder.range_on_face(&edge, &face, 0.0, TAU).unwrap();
    let start = hole.value(0.0).unwrap();
    let mut vertex = Vertex::default();
    builder
        .make_vertex_at(
            &mut vertex,
            surface.value(start.x(), start.y()).unwrap(),
            TOLERANCE,
        )
        .unwrap();
    vertex.set_orientation(Orientation::Forward);
    Builder
        .add(&mut edge, &vertex.clone().into_shape())
        .unwrap();
    vertex.set_orientation(Orientation::Reversed);
    Builder.add(&mut edge, &vertex.into_shape()).unwrap();
    brep_lib::build_curve3d(&edge, 1e-5, Continuity::C1, 14, 0).unwrap();
    Builder
        .add(&mut face, &wire_of(&[&edge]).into_shape())
        .unwrap();
    face.reverse();
    let fixer = ShapeFixFace::new(face.into_shape());
    assert!(fixer.fix_missing_seam().unwrap());
    let result = first_face(&fixer.result());
    let (u1, u2, v1, v2) = brep_tools::uv_bounds(&result).unwrap();
    close(u1, 0.0, "u min");
    close(u2, TAU, "u max");
    close(v1, 0.0, "v min");
    close(v2, 1.0, "v max");
    let wires = ShapeIterator::new(&result.clone().into_shape(), true, true).count();
    assert_eq!(wires, 1, "the hole joined the outer wire");
    close(area(&result), 6.08683576633, "area");
}

/// `(x, z)` on the surface is the point `(x, 0, z)`: a 3D curve and a p-curve can share their poles.
fn ruled_surface() -> Handle<dyn GeomSurface> {
    let axis = Axis1::new(
        Point3::new(0.0, 0.0, 0.0),
        Direction3::new(1.0, 0.0, 0.0).unwrap(),
    );
    let line: Handle<dyn GeomCurve> = Handle::new(GeomLine::new(axis)).cast();
    Handle::new(
        GeomSurfaceOfLinearExtrusion::new(&line, Direction3::new(0.0, 0.0, 1.0).unwrap()).unwrap(),
    )
    .cast()
}

fn spline_edge(face: &Face, points: &[(f64, f64)], degree: i32) -> Edge {
    let knot_count = points.len() - degree as usize + 1;
    let knots: Vec<f64> = (0..knot_count).map(|index| index as f64).collect();
    let mut mults = vec![1; knot_count];
    mults[0] = degree + 1;
    mults[knot_count - 1] = degree + 1;
    let poles_2d: Vec<Point2> = points.iter().map(|p| Point2::new(p.0, p.1)).collect();
    let poles_3d: Vec<Point3> = points.iter().map(|p| Point3::new(p.0, 0.0, p.1)).collect();
    let curve_3d: Handle<dyn GeomCurve> = Handle::new(
        GeomBSplineCurve::new(
            &Array1::from_shared(&poles_3d, 1),
            &Array1::from_shared(&knots, 1),
            &Array1::from_shared(&mults, 1),
            degree,
            false,
        )
        .unwrap(),
    )
    .cast();
    let curve_2d: Handle<dyn Geom2dCurve> = Handle::new(
        Geom2dBSplineCurve::new(
            &Array1::from_shared(&poles_2d, 1),
            &Array1::from_shared(&knots, 1),
            &Array1::from_shared(&mults, 1),
            degree,
            false,
        )
        .unwrap(),
    )
    .cast();
    let edge = BRepBuilderAPIMakeEdge::from_curve(&curve_3d)
        .unwrap()
        .edge()
        .unwrap();
    let builder = BRepBuilder::default();
    builder
        .update_edge_pcurve_on_face(&edge, &curve_2d, face, TOLERANCE)
        .unwrap();
    builder
        .range_on_face(&edge, face, 0.0, (knot_count - 1) as f64)
        .unwrap();
    edge
}

fn message_count(registrator: &Handle<ShapeExtendMsgRegistrator>) -> usize {
    registrator.map_shape(|messages| messages.extent())
}

fn with_messages(fixer: &ShapeFixWire) -> Handle<ShapeExtendMsgRegistrator> {
    let registrator = Handle::new(ShapeExtendMsgRegistrator::new());
    let shared: Handle<dyn BasicMsgRegistrator> = registrator.clone().cast();
    fixer.set_msg_registrator(&shared);
    fixer.set_max_tolerance(1.0);
    registrator
}

/// A loop beside a vertex: cutting it leaves a p-curve further from the 3D curve than the vertex is
/// from the crossing, so the cut is refused and the vertex tolerance raised. Expected: 1.001 * the
/// distance to the crossing, the p-curve kept, and "Edge was self-intersecting, corrected".
#[test]
fn loop_cut_beyond_the_callers_precision_raises_the_vertex_instead() {
    let surface = ruled_surface();
    let mut face = empty_face(&surface);
    let points = [
        (1.1, 0.0),
        (1.6, 0.0),
        (2.0, 0.0),
        (2.0, 0.9),
        (1.0, 0.9),
        (1.25, -0.5),
        (1.3, -1.0),
    ];
    let edge = spline_edge(&face, &points, 2);
    let wire = wire_of(&[&edge]);
    Builder.add(&mut face, &wire.clone().into_shape()).unwrap();
    let fixer = ShapeFixWire::from_wire_face(&wire, &face, TOLERANCE).unwrap();
    let messages = with_messages(&fixer);
    // Wide enough that only the caller's precision can refuse the cut.
    fixer.set_max_tolerance(10.0);
    assert!(fixer.fix_self_intersection().unwrap());
    let (first, last) = topexp::vertices(&fixer.wire_data().unwrap().edge(1).unwrap(), false);
    close(
        BRepTool::vertex_tolerance(&first).unwrap(),
        0.0599379187593821,
        "first vertex",
    );
    close(
        BRepTool::vertex_tolerance(&last).unwrap(),
        TOLERANCE,
        "last vertex",
    );
    assert_eq!(message_count(&messages), 1);
}

fn adjacent_crossing(topology: bool) {
    let builder = BRepBuilder::default();
    let surface = ruled_surface();
    let mut face = empty_face(&surface);
    let (mut start, mut shared, mut end) =
        (Vertex::default(), Vertex::default(), Vertex::default());
    builder
        .make_vertex_at(&mut start, Point3::new(0.0, 0.0, 0.0), TOLERANCE)
        .unwrap();
    builder
        .make_vertex_at(&mut shared, Point3::new(1.001, 0.0, -0.001), TOLERANCE)
        .unwrap();
    builder
        .make_vertex_at(&mut end, Point3::new(1.0, 0.0, 1.0), TOLERANCE)
        .unwrap();
    let first = with_vertices(
        spline_edge(
            &face,
            &[(0.0, 0.0), (0.5, 0.0), (1.0, 0.002), (1.001, 0.0)],
            3,
        ),
        &start,
        &shared,
    );
    let second = line_edge(&face, (1.0, -0.001), (1.0, 1.0), &shared, &end);
    let wire = wire_of(&[&first, &second]);
    Builder.add(&mut face, &wire.clone().into_shape()).unwrap();
    let fixer = ShapeFixWire::from_wire_face(&wire, &face, TOLERANCE).unwrap();
    let messages = with_messages(&fixer);
    fixer.set_closed_mode(false);
    fixer.set_topology_mode(topology);
    fixer.set_fix_intersecting_edges_mode(1);
    fixer.set_fix_self_intersecting_edge_mode(0);
    fixer.set_fix_nonadjacent_intersecting_edges_mode(0);
    assert!(fixer.fix_self_intersection().unwrap());
    let data = fixer.wire_data().unwrap();
    close(
        BRepTool::vertex_tolerance(&shared).unwrap(),
        0.000677200798302296,
        "shared vertex",
    );
    close(
        BRepTool::edge_tolerance(&data.edge(1).unwrap()).unwrap(),
        0.000655823783320412,
        "first edge",
    );
    close(
        BRepTool::edge_tolerance(&data.edge(2).unwrap()).unwrap(),
        0.000677200798302296,
        "second edge",
    );
    assert_eq!(message_count(&messages), 1);
}

fn with_vertices(edge: Edge, first: &Vertex, last: &Vertex) -> Edge {
    let mut replaced = topods::edge(&edge.into_shape().empty_copied().unwrap()).unwrap();
    replaced.set_orientation(Orientation::Forward);
    let mut vertex = first.clone();
    vertex.set_orientation(Orientation::Forward);
    Builder.add(&mut replaced, &vertex.into_shape()).unwrap();
    let mut vertex = last.clone();
    vertex.set_orientation(Orientation::Reversed);
    Builder.add(&mut replaced, &vertex.into_shape()).unwrap();
    replaced
}

/// Raised edge tolerances still leave the shared vertex at the larger one, and the wire says so.
#[test]
fn adjacent_crossing_raises_edges_and_shared_vertex() {
    adjacent_crossing(false);
}

/// The same in topology mode: edge tolerances are enough, so nothing is cut.
#[test]
fn adjacent_crossing_in_topology_mode_raises_the_shared_vertex_too() {
    adjacent_crossing(true);
}

fn line_edge(face: &Face, a: (f64, f64), b: (f64, f64), first: &Vertex, last: &Vertex) -> Edge {
    let builder = BRepBuilder::default();
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let length = (dx * dx + dy * dy).sqrt();
    let axis = Axis1::new(
        Point3::new(a.0, 0.0, a.1),
        Direction3::new(dx, 0.0, dy).unwrap(),
    );
    let curve: Handle<dyn GeomCurve> = Handle::new(GeomLine::new(axis)).cast();
    let pcurve: Handle<dyn Geom2dCurve> = Handle::new(Geom2dLine::from_point(
        Point2::new(a.0, a.1),
        Direction2::new(dx, dy).unwrap(),
    ))
    .cast();
    let mut edge = Edge::default();
    builder
        .make_edge_with_curve(&mut edge, &curve, TOLERANCE)
        .unwrap();
    let mut vertex = first.clone();
    vertex.set_orientation(Orientation::Forward);
    Builder.add(&mut edge, &vertex.into_shape()).unwrap();
    let mut vertex = last.clone();
    vertex.set_orientation(Orientation::Reversed);
    Builder.add(&mut edge, &vertex.into_shape()).unwrap();
    builder.range(&edge, 0.0, length, true).unwrap();
    builder
        .update_edge_pcurve_on_face(&edge, &pcurve, face, TOLERANCE)
        .unwrap();
    builder.range_on_face(&edge, face, 0.0, length).unwrap();
    edge
}

/// Both junctions of a two-edge wire lack an edge: the first repair adds one and the second must
/// still be reached. The result has 4 edges.
#[test]
fn lacking_repairs_reach_the_junction_after_an_inserted_edge() {
    let builder = BRepBuilder::default();
    let surface = ruled_surface();
    let mut face = empty_face(&surface);
    let (mut a, mut b) = (Vertex::default(), Vertex::default());
    builder
        .make_vertex_at(&mut a, Point3::new(0.0, 0.0, 0.05), TOLERANCE)
        .unwrap();
    builder
        .make_vertex_at(&mut b, Point3::new(1.0, 0.0, 0.5), TOLERANCE)
        .unwrap();
    let first = line_edge(&face, (0.0, 0.0), (1.0, 0.45), &a, &b);
    let second = line_edge(&face, (1.0, 0.55), (0.0, 0.1), &b, &a);
    let wire = wire_of(&[&first, &second]);
    Builder.add(&mut face, &wire.clone().into_shape()).unwrap();
    let fixer = ShapeFixWire::from_wire_face(&wire, &face, TOLERANCE).unwrap();
    fixer.set_max_tolerance(1.0);
    fixer.set_topology_mode(true);
    assert!(fixer.fix_lacking(false).unwrap());
    assert_eq!(fixer.nb_edges(), 4);
}

/// A wire through a vertex twice is split, into two faces here, and says so on the wire.
#[test]
fn wire_split_at_a_repeated_vertex_says_so() {
    let builder = BRepBuilder::default();
    let surface = ruled_surface();
    let mut face = empty_face(&surface);
    let mut vertices = Vec::new();
    for point in [
        (0.0, 0.0),
        (1.0, 0.0),
        (1.0, 1.0),
        (-1.0, 0.0),
        (-1.0, -1.0),
    ] {
        let mut vertex = Vertex::default();
        builder
            .make_vertex_at(&mut vertex, Point3::new(point.0, 0.0, point.1), TOLERANCE)
            .unwrap();
        vertices.push((point, vertex));
    }
    let edges: Vec<Edge> = [(0, 1), (1, 2), (2, 0), (0, 3), (3, 4), (4, 0)]
        .iter()
        .map(|&(a, b)| {
            line_edge(
                &face,
                vertices[a].0,
                vertices[b].0,
                &vertices[a].1,
                &vertices[b].1,
            )
        })
        .collect();
    let wire = wire_of(&edges.iter().collect::<Vec<_>>());
    Builder.add(&mut face, &wire.into_shape()).unwrap();
    let registrator = Handle::new(ShapeExtendMsgRegistrator::new());
    let shared: Handle<dyn BasicMsgRegistrator> = registrator.clone().cast();
    let fixer = ShapeFixFace::new(face.into_shape());
    fixer.set_msg_registrator(&shared);
    fixer.set_max_tolerance(1.0);
    fixer.perform().unwrap();
    let result = fixer.result();
    assert_eq!(
        Explorer::new(&result, ShapeEnum::Face, ShapeEnum::Shape)
            .unwrap()
            .count(),
        2
    );
    assert_eq!(
        Explorer::new(&result, ShapeEnum::Wire, ShapeEnum::Shape)
            .unwrap()
            .count(),
        2
    );
    let keys: Vec<String> = registrator.map_shape(|messages| {
        messages
            .items()
            .flat_map(|(_, list)| list.iter().map(|message| message.original().to_string()))
            .collect()
    });
    assert_eq!(keys.len(), 1, "{keys:?}");
    assert!(keys[0].contains("FixAdvFace.FixLoopWire.MSG0"), "{keys:?}");
}

/// After a seam the second pass works on the face the seam made, not on the one it started from.
#[test]
fn second_pass_gives_the_wire_tool_the_face_after_the_seam() {
    let input = torus_with_two_bands(40.0, 8.0, 1.316563313, 5.235986997);
    let fixer = ShapeFixFace::new(input.clone().into_shape());
    fixer.set_context(&Handle::new(ShapeBuildReShape::new()));
    fixer.perform().unwrap();
    let worked_on = fixer.fix_wire_tool().working_analyzer().face();
    assert!(!worked_on.is_same(&input));
    let wires = ShapeIterator::new(&worked_on.into_shape(), true, true).count();
    assert_eq!(
        wires, 1,
        "expected the one wire the seam joined the two into"
    );
}

/// The first edge ends on the crossing, so it cannot be cut, and the edge tolerances already cover
/// the deviation: in topology mode the vertex takes the tolerance of its distance to the crossing,
/// not that of the deviation. Expected: 0.00141435498372925.
#[test]
fn uncuttable_adjacent_crossing_uses_the_distance_in_topology_mode() {
    let builder = BRepBuilder::default();
    let surface = ruled_surface();
    let mut face = empty_face(&surface);
    let mut vertices = Vec::new();
    for point in [(0.0, 0.0), (1.001, -0.001), (1.0, 1.0), (2.0, 1.0)] {
        let mut vertex = Vertex::default();
        builder
            .make_vertex_at(&mut vertex, Point3::new(point.0, 0.0, point.1), TOLERANCE)
            .unwrap();
        vertices.push(vertex);
    }
    let first = line_edge(&face, (0.0, 0.0), (1.0, 0.0), &vertices[0], &vertices[1]);
    let second = line_edge(&face, (1.0, -0.001), (1.0, 1.0), &vertices[1], &vertices[2]);
    let third = line_edge(&face, (1.0, 1.0), (2.0, 1.0), &vertices[2], &vertices[3]);
    builder.update_edge_tolerance(&first, 1e-3).unwrap();
    builder.update_edge_tolerance(&second, 1e-3).unwrap();
    let wire = wire_of(&[&first, &second, &third]);
    Builder.add(&mut face, &wire.clone().into_shape()).unwrap();
    let fixer = ShapeFixWire::from_wire_face(&wire, &face, TOLERANCE).unwrap();
    fixer.set_max_tolerance(1.0);
    fixer.set_closed_mode(false);
    fixer.set_topology_mode(true);
    fixer.set_fix_intersecting_edges_mode(1);
    fixer.set_fix_self_intersecting_edge_mode(0);
    fixer.set_fix_nonadjacent_intersecting_edges_mode(0);
    assert!(fixer.fix_self_intersection().unwrap());
    let (_, shared) = topexp::vertices(&fixer.wire_data().unwrap().edge(1).unwrap(), false);
    close(
        BRepTool::vertex_tolerance(&shared).unwrap(),
        0.00141435498372925,
        "shared vertex",
    );
}
