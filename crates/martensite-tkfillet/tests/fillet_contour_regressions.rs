//! Direct regressions of defects the grids found. Each builds its input in code, so a failure
//! here names the defect without a data file or a table in the way.
use martensite_tkbo::brep_algo_api::BRepAlgoAPIFuse;
use martensite_tkbrep::{
    topexp,
    topods::{self, Edge, Shape, Wire},
    toptools::IndexedMapOfShape,
};
use martensite_tkernel::{message::ProgressRange, ncollection::Array1, standard::Handle};
use martensite_tkfillet::{brep_fillet_api::BRepFilletAPIMakeFillet, chfi3d::FilletShape};
use martensite_tkg3d::{
    geom::{GeomBezierCurve, GeomCurve},
    top_abs::ShapeEnum,
};
use martensite_tkmath::gp::{Axis2, Point3, Vector3, dz};
use martensite_tkprim::brep_prim_api::{
    BRepPrimAPIMakeBox, BRepPrimAPIMakeCylinder, BRepPrimAPIMakePrism,
};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape, BRepBuilderAPIMakeWire,
};
use std::{sync::mpsc, thread, time::Duration};

/// Well above the 21 s the slowest case of the grids takes unoptimized.
const DEADLOCK_LIMIT: Duration = Duration::from_secs(60);

fn edge_at(shape: &Shape, index: usize) -> Edge {
    let mut edges = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Edge, &mut edges).unwrap();
    topods::edge(edges.find_key(index).unwrap()).unwrap()
}

fn is_done_within_limit(build: impl FnOnce() -> bool + Send + 'static) -> Option<bool> {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || sender.send(build()));
    receiver.recv_timeout(DEADLOCK_LIMIT).ok()
}

fn blend_is_done(shape: &Shape, radius: f64, edges: &[usize]) -> bool {
    let mut maker = BRepFilletAPIMakeFillet::new(shape, FilletShape::Rational).unwrap();
    for index in edges {
        maker
            .add_with_radius(radius, &edge_at(shape, *index))
            .unwrap();
    }
    maker.build(&ProgressRange::default()).unwrap();
    maker.is_done()
}

/// A prism of a closed Bezier loop: one vertex on each end, so a contour meets itself.
fn prism_of_closed_bezier() -> Shape {
    let face = BRepBuilderAPIMakeFace::from_wire(&closed_bezier_wire(), false)
        .unwrap()
        .face()
        .unwrap();
    BRepPrimAPIMakePrism::new(face.as_ref(), Vector3::new(0., 0., 10.), false, true)
        .unwrap()
        .shape()
        .unwrap()
        .clone()
}

fn closed_bezier_wire() -> Wire {
    let points = [(0., 0.), (10., 0.), (0., 10.), (0., 0.)];
    let poles: Vec<Point3> = points
        .iter()
        .map(|(x, y)| Point3::new(*x, *y, 0.))
        .collect();
    let curve: Handle<dyn GeomCurve> =
        Handle::new(GeomBezierCurve::new(&Array1::from_shared(&poles, 1)).unwrap()).cast();
    let edge = BRepBuilderAPIMakeEdge::from_curve(&curve)
        .unwrap()
        .edge()
        .unwrap();
    BRepBuilderAPIMakeWire::from_edge(&edge)
        .unwrap()
        .wire()
        .unwrap()
}

/// The two ends of one contour sharing a corner read the same stripe twice in `IsInFront`;
/// holding the first guard while taking the second locked it forever (bugs/modalg_7/bug30940_4).
#[test]
fn a_contour_meeting_itself_at_a_corner_does_not_deadlock() {
    let finished =
        is_done_within_limit(|| blend_is_done(&prism_of_closed_bezier(), 0.5, &[1, 2, 3]));
    assert_eq!(finished, Some(true), "the blend must finish and be done");
}

/// A closed edge is two entries among the edges of its vertex, one per orientation; counted as
/// one, the vertex looked broken and the corner got a face of its own (bugs/modalg_7/bug30940_2).
#[test]
fn the_closed_edge_of_a_prism_blends_without_an_extra_face() {
    let shape = prism_of_closed_bezier();
    let mut maker = BRepFilletAPIMakeFillet::new(&shape, FilletShape::Rational).unwrap();
    maker.add_with_radius(0.5, &closed_edge_of(&shape)).unwrap();
    maker.build(&ProgressRange::default()).unwrap();
    let mut faces = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(maker.shape().unwrap(), ShapeEnum::Face, &mut faces).unwrap();
    assert_eq!(
        faces.extent(),
        4,
        "the top, the bottom, the side and the fillet"
    );
}

fn closed_edge_of(shape: &Shape) -> Edge {
    (1..=3)
        .map(|index| edge_at(shape, index))
        .find(|edge| {
            let (first, last) = topexp::vertices(edge, false);
            first.is_same(&last) && !is_vertical(edge)
        })
        .expect("the prism has a closed Bezier edge")
}

fn is_vertical(edge: &Edge) -> bool {
    let (first, last) = topexp::vertices(edge, false);
    let z = |vertex| martensite_tkbrep::brep::BRepTool::pnt(vertex).unwrap().z();
    (z(&first) - z(&last)).abs() > 1e-6
}

/// A box under a cylinder: the fillet of a box edge runs into the cylinder's circle.
fn box_with_cylinder_on_top() -> Shape {
    let mut block =
        BRepPrimAPIMakeBox::at_point(Point3::new(-10., -10., -10.), 20., 20., 20.).unwrap();
    let axes = Axis2::new(Point3::new(0., 0., 10.), dz()).unwrap();
    let mut post = BRepPrimAPIMakeCylinder::with_axes(axes, 8., 15.);
    let post_shape = post.shape().unwrap().clone();
    let block_shape = block.shape().unwrap().clone();
    BRepAlgoAPIFuse::from_shapes(&block_shape, &post_shape)
        .unwrap()
        .shape()
        .clone()
}

fn top_edge_of_box(shape: &Shape) -> usize {
    let mut edges = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Edge, &mut edges).unwrap();
    (1..=edges.extent())
        .find(|index| is_box_top_edge_at_minus_y(&edge_at(shape, *index)))
        .expect("the fused solid keeps the box edge at y = -10, z = 10")
}

fn is_box_top_edge_at_minus_y(edge: &Edge) -> bool {
    let (first, last) = topexp::vertices(edge, false);
    [first, last].iter().all(|vertex| {
        let point = martensite_tkbrep::brep::BRepTool::pnt(vertex).unwrap();
        (point.y() + 10.).abs() < 1e-6 && (point.z() - 10.).abs() < 1e-6
    })
}

/// The walk continues on the circle as a restriction. `NextSide` decides that walk's sign from
/// the orientations it changes, and dropping them gave the opposite radius (blend/simple/R9).
#[test]
fn a_fillet_walks_over_a_circular_obstacle() {
    let shape = box_with_cylinder_on_top();
    let edge = top_edge_of_box(&shape);
    assert!(blend_is_done(&shape, 3., &[edge]));
}
