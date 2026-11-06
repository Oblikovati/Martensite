//! The order of a wire on a surface periodic in both directions, where all the ends of its edges
//! can lie at one point in space and only the gaps on the surface tell the edges apart.
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::topods::{self, Builder, Edge, Face, Vertex, Wire};
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dLine};
use martensite_tkg3d::geom::{GeomSurface, GeomToroidalSurface};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{Axis3, Direction2, Point2, Point3, Xy, Xyz};
use martensite_tkshhealing::shape_analysis::{
    ShapeAnalysisEdge, ShapeAnalysisWire, ShapeAnalysisWireOrder,
};
use martensite_tkshhealing::shape_extend::{ShapeExtendStatus, ShapeExtendWireData};
use martensite_tkshhealing::shape_fix::ShapeFixWire;
use martensite_tktopalgo::brep_gprop::BRepGProp;

/// A full turn as a file with ten digits holds it: 1.8e-10 short of two pi.
#[expect(
    clippy::approx_constant,
    reason = "these are the digits such a file holds"
)]
const TURN: f64 = 6.283185307;
/// Where the p-curves of the other seam side start when a file was read back once.
const SHORT_TURN: f64 = 6.283185306820;
const MAJOR_RADIUS: f64 = 5.0;
const MINOR_RADIUS: f64 = 1.0;

/// One edge of the wire: its point in space, where it starts on the surface, and its direction there.
struct Side {
    point: Xyz,
    start: Xy,
    direction: (f64, f64),
    iso_u: Option<f64>,
    iso_v: Option<f64>,
}

/// Each edge closes in space on its own; the four closed edges lie about 1e-9 apart.
fn sides() -> [Side; 4] {
    [
        Side {
            point: Xyz::new(6.0, 0.0, -1.796e-10),
            start: Xy::new(SHORT_TURN, TURN),
            direction: (-1.0, 0.0),
            iso_u: None,
            iso_v: Some(TURN),
        },
        Side {
            point: Xyz::new(6.0, 1.0775e-9, -1.796e-10),
            start: Xy::new(0.0, TURN),
            direction: (0.0, -1.0),
            iso_u: Some(0.0),
            iso_v: None,
        },
        Side {
            point: Xyz::new(6.0, 1.0775e-9, 0.0),
            start: Xy::new(0.0, 0.0),
            direction: (1.0, 0.0),
            iso_u: None,
            iso_v: Some(0.0),
        },
        Side {
            point: Xyz::new(6.0, 0.0, 0.0),
            start: Xy::new(SHORT_TURN, 0.0),
            direction: (0.0, 1.0),
            iso_u: Some(SHORT_TURN),
            iso_v: None,
        },
    ]
}

fn end_on_surface(side: &Side) -> Xy {
    Xy::new(
        side.start.x() + side.direction.0 * TURN,
        side.start.y() + side.direction.1 * TURN,
    )
}

fn ordered(order: &ShapeAnalysisWireOrder) -> Vec<i32> {
    (1..=order.nb_edges())
        .map(|rank| order.ordered(rank).expect("a rank"))
        .collect()
}

#[test]
fn the_gaps_on_the_surface_keep_edges_that_meet_at_one_point_in_order() {
    let mut order = ShapeAnalysisWireOrder::with_mode(true, 1e-7, true);
    for side in sides() {
        order.add_both(side.point, side.point, side.start, end_on_surface(&side));
    }
    order.perform(true).expect("an order");
    assert_eq!(ordered(&order), [1, 2, 3, 4]);
    assert_eq!(order.status(), 0);
}

#[test]
fn the_gaps_in_space_alone_cannot_tell_those_edges_apart() {
    let mut order = ShapeAnalysisWireOrder::new();
    for side in sides() {
        order.add_3d(side.point, side.point);
    }
    order.perform(true).expect("an order");
    assert_eq!(ordered(&order), [1, 4, 3, 2]);
}

/// A joint needs the 3D and the surface gap closed: a surface gap of a whole period is no joint.
#[test]
fn a_gap_of_a_period_on_the_surface_leaves_the_order_to_the_gaps_in_space() {
    let point = |x: f64| Xyz::new(x, 0.0, 0.0);
    let uv = |u: f64| Xy::new(u, 0.0);
    let mut order = ShapeAnalysisWireOrder::with_mode(true, 1e-7, true);
    order.add_both(point(0.0), point(1.0), uv(0.0), uv(1.0));
    order.add_both(point(2.0), point(3.0), uv(2.0), uv(3.0));
    order.add_both(point(1.0), point(2.0), uv(1.0 + TURN), uv(2.0 + TURN));
    order.perform(true).expect("an order");
    assert_eq!(ordered(&order), [1, 3, 2]);
}

fn torus_face() -> (Handle<dyn GeomSurface>, Face) {
    let surface: Handle<dyn GeomSurface> = Handle::new(
        GeomToroidalSurface::new(Axis3::default(), MAJOR_RADIUS, MINOR_RADIUS).unwrap(),
    )
    .cast();
    let mut face = Face::default();
    BRepBuilder::default()
        .make_face_on_surface(&mut face, &surface, 1e-7)
        .unwrap();
    (surface, face)
}

fn vertex_at(point: Xyz, orientation: Orientation) -> Vertex {
    let mut vertex = Vertex::default();
    BRepBuilder::default()
        .make_vertex_at(&mut vertex, Point3::from(point), 1e-7)
        .unwrap();
    topods::vertex(&vertex.into_shape().oriented(orientation)).unwrap()
}

/// An edge along an iso-line of the surface, closed in space on one point.
fn edge_of(side: &Side, surface: &Handle<dyn GeomSurface>, face: &Face, with_pcurve: bool) -> Edge {
    let geometry = surface.get().expect("a surface");
    let curve = match (side.iso_u, side.iso_v) {
        (Some(u), _) => geometry.u_iso(u).unwrap(),
        (_, Some(v)) => geometry.v_iso(v).unwrap(),
        _ => unreachable!("every side lies on an iso-line"),
    };
    let builder = BRepBuilder::default();
    let mut edge = Edge::default();
    builder
        .make_edge_with_curve(&mut edge, &curve, 1e-7)
        .unwrap();
    let mut shape = edge.clone().into_shape();
    for orientation in [Orientation::Forward, Orientation::Reversed] {
        let vertex = vertex_at(side.point, orientation).into_shape();
        Builder.add(&mut shape, &vertex).unwrap();
    }
    builder.range(&edge, 0.0, TURN, true).unwrap();
    if with_pcurve {
        let direction = Direction2::new(side.direction.0, side.direction.1).unwrap();
        let start = Point2::new(side.start.x(), side.start.y());
        let line: Handle<dyn Geom2dCurve> =
            Handle::new(Geom2dLine::from_point(start, direction)).cast();
        builder
            .update_edge_pcurve_on_face(&edge, &line, face, 1e-7)
            .unwrap();
        builder.range_on_face(&edge, face, 0.0, TURN).unwrap();
    }
    edge
}

fn wire_of(edges: &[Edge]) -> Wire {
    let mut wire = Wire::default();
    Builder.make_wire(&mut wire);
    let mut shape = wire.clone().into_shape();
    for edge in edges {
        Builder.add(&mut shape, &edge.clone().into_shape()).unwrap();
    }
    topods::wire(&shape).unwrap()
}

fn area_inside(face: &Face, wire: &Wire) -> f64 {
    let mut shape = face.clone().into_shape();
    Builder.add(&mut shape, &wire.clone().into_shape()).unwrap();
    let mut properties = GProps::new();
    BRepGProp::surface_properties(&shape, &mut properties, false, false).unwrap();
    properties.mass()
}

#[test]
fn healing_keeps_the_area_of_a_torus_whose_edges_meet_at_one_point() {
    let (surface, face) = torus_face();
    let edges = sides().map(|side| edge_of(&side, &surface, &face, true));
    let fixer = ShapeFixWire::from_wire_face(&wire_of(&edges), &face, 1e-7).unwrap();
    fixer.perform().unwrap();
    assert!(!fixer.status_reorder(ShapeExtendStatus::Done));
    let found = area_inside(&face, &fixer.wire().unwrap());
    let expected = 4.0 * std::f64::consts::PI * std::f64::consts::PI * MAJOR_RADIUS * MINOR_RADIUS;
    assert!(
        (found - expected).abs() <= 1.0e-5 * expected,
        "an area of {found}, expected {expected}"
    );
}

/// Unconnected, the ends stay apart through every stage of the healing, the last one included.
#[test]
fn every_reordering_stage_keeps_edges_that_stay_unconnected() {
    let (surface, face) = torus_face();
    let edges = sides().map(|side| edge_of(&side, &surface, &face, true));
    let fixer = ShapeFixWire::from_wire_face(&wire_of(&edges), &face, 1e-7).unwrap();
    fixer.set_fix_connected_mode(0);
    fixer.perform().unwrap();
    let wire_data = fixer.wire_data().unwrap();
    let kept: Vec<bool> = edges
        .iter()
        .enumerate()
        .map(|(index, edge)| wire_data.edge(index as i32 + 1).unwrap().is_same(edge))
        .collect();
    assert_eq!(kept, [true; 4]);
}

/// A patch that crosses the seam twice: its p-curves are apart by a period where its ends meet, so
/// the two orders give one chain from different first edges. The wire keeps the 3D order then.
#[test]
fn a_wire_that_both_orders_chain_alike_keeps_its_first_edge() {
    let (surface, face) = torus_face();
    let geometry = surface.get().expect("a surface");
    let builder = BRepBuilder::default();
    let at = |u: f64, v: f64| {
        let point = geometry.value(u, v).unwrap();
        Xyz::new(point.x(), point.y(), point.z())
    };
    let half_v = std::f64::consts::FRAC_PI_2;
    let full = std::f64::consts::TAU;
    let side = |from: (f64, f64), to: (f64, f64)| {
        let circle = if from.1 == to.1 {
            geometry.v_iso(from.1).unwrap()
        } else {
            geometry.u_iso(from.0).unwrap()
        };
        let mut edge = Edge::default();
        builder
            .make_edge_with_curve(&mut edge, &circle, 1e-7)
            .unwrap();
        let mut shape = edge.clone().into_shape();
        Builder
            .add(
                &mut shape,
                &vertex_at(at(from.0, from.1), Orientation::Forward).into_shape(),
            )
            .unwrap();
        Builder
            .add(
                &mut shape,
                &vertex_at(at(to.0, to.1), Orientation::Reversed).into_shape(),
            )
            .unwrap();
        let along = (to.0 - from.0, to.1 - from.1);
        let length = martensite_cmath::hypot(along.0, along.1);
        let curve: Handle<dyn Geom2dCurve> = Handle::new(Geom2dLine::from_point(
            Point2::new(from.0, from.1),
            Direction2::new(along.0, along.1).unwrap(),
        ))
        .cast();
        builder
            .update_edge_pcurve_on_face(&edge, &curve, &face, 1e-7)
            .unwrap();
        builder.range_on_face(&edge, &face, 0.0, length).unwrap();
        edge
    };
    let wire_data = Handle::new(ShapeExtendWireData::new());
    let edges = [
        side((0.0, 0.0), (full, 0.0)),
        side((0.0, 0.0), (0.0, half_v)),
        side((1.5, half_v), (0.0, half_v)),
        side((full, half_v), (1.5, half_v)),
        side((0.0, half_v), (0.0, 0.0)),
    ];
    for edge in &edges {
        wire_data.add_edge(edge, 0).unwrap();
    }
    let analyzer = Handle::new(ShapeAnalysisWire::from_wire_data(&wire_data, &face, 1e-7).unwrap());
    let mut in_space = ShapeAnalysisWireOrder::new();
    analyzer
        .check_order_with(&mut in_space, true, true, false)
        .unwrap();
    let mut on_surface = ShapeAnalysisWireOrder::new();
    analyzer
        .check_order_with(&mut on_surface, true, true, true)
        .unwrap();
    assert_eq!(ordered(&in_space), [1, 2, 4, 3, 5]);
    assert_eq!(ordered(&on_surface), [2, 4, 3, 5, 1]);
    let fixer = ShapeFixWire::new();
    fixer.init(&analyzer);
    fixer.set_fix_add_pcurve_mode(0);
    fixer.set_fix_connected_mode(0);
    fixer.set_fix_self_intersection_mode(0);
    fixer.perform().unwrap();
    let wire_data = fixer.wire_data().unwrap();
    for (rank, kept) in [0, 1, 3, 2, 4].into_iter().enumerate() {
        assert!(
            wire_data
                .edge(rank as i32 + 1)
                .unwrap()
                .is_same(&edges[kept])
        );
    }
}

/// The surface gaps need a p-curve on every edge; without them the 3D gaps still order the wire.
#[test]
fn a_wire_without_pcurves_is_still_ordered_by_the_gaps_in_space() {
    let (surface, face) = torus_face();
    let turn = |u: f64| {
        Xyz::new(
            6.0 * martensite_cmath::cos(u),
            6.0 * martensite_cmath::sin(u),
            0.0,
        )
    };
    let geometry = surface.get().expect("a surface");
    let builder = BRepBuilder::default();
    let circle = geometry.v_iso(0.0).unwrap();
    let edge_between = |from: f64, to: f64| {
        let mut edge = Edge::default();
        builder
            .make_edge_with_curve(&mut edge, &circle, 1e-7)
            .unwrap();
        let mut shape = edge.clone().into_shape();
        Builder
            .add(
                &mut shape,
                &vertex_at(turn(from), Orientation::Forward).into_shape(),
            )
            .unwrap();
        Builder
            .add(
                &mut shape,
                &vertex_at(turn(to), Orientation::Reversed).into_shape(),
            )
            .unwrap();
        builder.range(&edge, from, to, true).unwrap();
        edge
    };
    let edges = [
        edge_between(0.0, 1.0),
        edge_between(2.0, 3.0),
        edge_between(1.0, 2.0),
    ];
    let wire_data = Handle::new(ShapeExtendWireData::new());
    for edge in &edges {
        wire_data.add_edge(edge, 0).unwrap();
    }
    let analyzer = Handle::new(ShapeAnalysisWire::from_wire_data(&wire_data, &face, 1e-7).unwrap());
    let fixer = ShapeFixWire::new();
    fixer.init(&analyzer);
    fixer.set_fix_add_pcurve_mode(0);
    // Connecting the edges would close the gaps of a wire left out of order, and the last stage
    // would order it after the stages that needed the order.
    fixer.set_fix_connected_mode(0);
    fixer.set_fix_self_intersection_mode(0);
    fixer.perform().unwrap();
    let wire_data = fixer.wire_data().unwrap();
    let tool = ShapeAnalysisEdge::new();
    let ends: Vec<(Point3, Point3)> = (1..=wire_data.nb_edges())
        .map(|index| {
            let edge = wire_data.edge(index).unwrap();
            (
                BRepTool::pnt(&tool.first_vertex(&edge)).unwrap(),
                BRepTool::pnt(&tool.last_vertex(&edge)).unwrap(),
            )
        })
        .collect();
    assert_eq!(ends.len(), 3);
    for pair in ends.windows(2) {
        assert!(
            pair[0].1.distance(&pair[1].0) < 1e-7,
            "an edge ends at {:?} and the next one starts at {:?}",
            pair[0].1,
            pair[1].0
        );
    }
}
