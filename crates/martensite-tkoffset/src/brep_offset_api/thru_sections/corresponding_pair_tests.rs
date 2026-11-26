// SPDX-License-Identifier: GPL-2.0-only
use super::{BRepOffsetAPIThruSections, CorrespondingRuledPairFailure};
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Edge, Vertex, Wire},
};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeEdge;

#[test]
fn two_mixed_wires_refuse_matching_punctual_edges_before_ruled_history() {
    let first = square(0., true, [0, 1, 2, 3]);
    let last = square(3., true, [0, 1, 2, 3]);
    assert_eq!(first.nb_children(), last.nb_children());
    assert!(matches!(
        BRepOffsetAPIThruSections::corresponding_ruled_pair([&first, &last], 1e-7),
        Err(CorrespondingRuledPairFailure::DegenerateSectionEdge {
            section: 0,
            edge: 1
        })
    ));
}

#[test]
fn malformed_closed_occurrence_order_refuses_and_actual_order_completes() {
    let first = square(0., false, [0, 1, 2, 3]);
    let last = square(3., false, [0, 2, 1, 3]);
    assert!(matches!(
        BRepOffsetAPIThruSections::corresponding_ruled_pair([&first, &last], 1e-7),
        Err(CorrespondingRuledPairFailure::SectionOrder(1))
    ));
    let last = square(3., false, [0, 1, 2, 3]);
    let done = BRepOffsetAPIThruSections::corresponding_ruled_pair([&first, &last], 1e-7).unwrap();
    for section in done.sections {
        assert_eq!(section.edges.len(), 4);
        assert_eq!(section.vertices.len(), 4);
    }
}

fn square(height: f64, punctual: bool, order: [usize; 4]) -> Wire {
    let builder = BRepBuilder::default();
    let points = [[0., 0.], [1., 0.], [1., 1.], [0., 1.]].map(|[x, y]| {
        let mut vertex = Vertex::default();
        builder
            .make_vertex_at(&mut vertex, Point3::new(x, y, height), 1e-7)
            .unwrap();
        vertex
    });
    let mut wire = Wire::default();
    builder.make_wire(&mut wire);
    for index in order {
        let edge = segment(&points[index], &points[(index + 1) % 4]);
        builder.add(&mut wire, &edge).unwrap();
        if punctual && index == 0 {
            builder.add(&mut wire, &degenerate(&points[1])).unwrap();
        }
    }
    wire.set_closed(true).unwrap();
    wire
}

fn segment(from: &Vertex, to: &Vertex) -> Edge {
    BRepBuilderAPIMakeEdge::from_vertices_with_tolerance(from, to, 1e-7)
        .unwrap()
        .edge()
        .unwrap()
}

fn degenerate(point: &Vertex) -> Edge {
    let builder = BRepBuilder::default();
    let mut edge = Edge::default();
    builder.make_edge(&mut edge).unwrap();
    builder.degenerated(&edge, true).unwrap();
    builder
        .add(&mut edge, &point.oriented(Orientation::Forward))
        .unwrap();
    builder
        .add(&mut edge, &point.oriented(Orientation::Reversed))
        .unwrap();
    edge
}
