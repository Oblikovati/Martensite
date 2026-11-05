//! Two authored disconnected split squares exercise real irregular-block tasks on one plane.
use crate::bop_algo::{WireEdgeSet, WireSplitter};
use martensite_tkbrep::topods::{Face, Shape, ShapeIterator, Vertex};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::gp::{Plane3, Point3};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeVertex,
};

/// Build authored task inputs; `split_square_splitter(true)` retains all twelve input slots.
pub(super) fn split_square_splitter(parallel: bool) -> (WireSplitter, Vec<Shape>) {
    let face = planar_face();
    let mut edges = split_square_edges(&split_square_vertices(0.));
    edges.extend(split_square_edges(&split_square_vertices(5.)));
    let mut wes = WireEdgeSet::new();
    wes.set_face(&face);
    for edge in &edges {
        wes.add_start_element(edge);
    }
    let mut splitter = WireSplitter::new();
    splitter.set_wes(wes);
    splitter.options_mut().set_run_parallel(parallel);
    (splitter, edges)
}

fn planar_face() -> Face {
    let maker =
        BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (0., 10., 0., 10.)).unwrap();
    maker.face().unwrap().clone()
}

fn split_square_vertices(offset: f64) -> [Vertex; 4] {
    [
        (1. + offset, 1.),
        (3. + offset, 1.),
        (3. + offset, 3.),
        (1. + offset, 3.),
    ]
    .map(|(x, y)| {
        BRepBuilderAPIMakeVertex::new(Point3::new(x, y, 0.))
            .unwrap()
            .vertex()
            .unwrap()
            .clone()
    })
}

fn split_square_edges(vertices: &[Vertex; 4]) -> Vec<Shape> {
    let mut edges = Vec::new();
    for index in 0..4 {
        let edge =
            BRepBuilderAPIMakeEdge::from_vertices(&vertices[index], &vertices[(index + 1) % 4])
                .unwrap()
                .edge()
                .unwrap();
        edges.push(edge.oriented(Orientation::Forward));
    }
    let diagonal = BRepBuilderAPIMakeEdge::from_vertices(&vertices[0], &vertices[2])
        .unwrap()
        .edge()
        .unwrap();
    edges.push(diagonal.oriented(Orientation::Forward));
    edges.push(diagonal.oriented(Orientation::Reversed));
    edges
}

/// Record real output incidences against authored identities; `input_identity_records(splitter, inputs)`.
pub(super) fn input_identity_records(
    splitter: &WireSplitter,
    inputs: &[Shape],
) -> Vec<Vec<(usize, Orientation)>> {
    splitter.wes().unwrap().shapes().iter().map(|wire| {
        ShapeIterator::new(wire, true, true).map(|edge| {
            let index = inputs.iter().position(|input| input.is_same(&edge)).unwrap_or_else(|| {
                panic!("unexpected edge {edge:?}; expected one of {} authored input identities", inputs.len())
            });
            (index, edge.orientation())
        }).collect()
    }).collect()
}

/// Prepare an explicitly empty edge set; `empty_splitter()` separates empty completion from failure.
pub(super) fn empty_splitter() -> WireSplitter {
    let mut splitter = WireSplitter::new();
    splitter.set_wes(WireEdgeSet::new());
    splitter
}
