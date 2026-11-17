//! Parent identities are checked against actual incidences, not just key counts.
use crate::acceptance::geometry::entity;
use martensite_tkbrep::{topexp, topods};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tksketch::surface_sketch::*;

fn key(source: HistorySource, role: HistoryRole) -> HistoryKey {
    HistoryKey { source, role }
}

pub(super) fn assert_semantic_incidence(body: &DetachedNormalBody, request: &ConeSector) {
    for index in 0..4 {
        let vertex = HistorySource::Vertex(request.corners[index]);
        let next = HistorySource::Vertex(request.corners[(index + 1) % 4]);
        let curve = HistorySource::Curve(request.boundaries[index]);
        assert_edge(
            body,
            key(vertex, HistoryRole::NormalEdge),
            [
                key(vertex, HistoryRole::FirstDepthVertex),
                key(vertex, HistoryRole::LastDepthVertex),
            ],
        );
        for (edge_role, vertex_role) in [
            (HistoryRole::FirstDepthEdge, HistoryRole::FirstDepthVertex),
            (HistoryRole::LastDepthEdge, HistoryRole::LastDepthVertex),
        ] {
            assert_edge(
                body,
                key(curve, edge_role),
                [key(vertex, vertex_role), key(next, vertex_role)],
            );
        }
        assert_face(
            body,
            key(curve, HistoryRole::SideFace),
            [
                key(curve, HistoryRole::FirstDepthEdge),
                key(curve, HistoryRole::LastDepthEdge),
                key(vertex, HistoryRole::NormalEdge),
                key(next, HistoryRole::NormalEdge),
            ],
        );
    }
    assert_depth_faces(body, request);
}
fn assert_depth_faces(body: &DetachedNormalBody, request: &ConeSector) {
    for (face_role, edge_role) in [
        (HistoryRole::FirstDepthFace, HistoryRole::FirstDepthEdge),
        (HistoryRole::LastDepthFace, HistoryRole::LastDepthEdge),
    ] {
        let edges = request
            .boundaries
            .map(|id| key(HistorySource::Curve(id), edge_role));
        assert_face(
            body,
            key(HistorySource::Region(request.region), face_role),
            edges,
        );
    }
}
fn assert_edge(body: &DetachedNormalBody, edge_key: HistoryKey, ends: [HistoryKey; 2]) {
    let edge = topods::edge(entity(body, edge_key)).unwrap();
    let (first, last) = topexp::vertices(&edge, true);
    let expected = [entity(body, ends[0]), entity(body, ends[1])];
    assert!(
        (first.is_same(expected[0]) && last.is_same(expected[1]))
            || (first.is_same(expected[1]) && last.is_same(expected[0])),
        "{edge_key:?}"
    );
}
fn assert_face(body: &DetachedNormalBody, face_key: HistoryKey, edges: [HistoryKey; 4]) {
    let mut map = martensite_tkbrep::toptools::IndexedMapOfShape::default();
    topexp::map_shapes_of_type(entity(body, face_key), ShapeEnum::Edge, &mut map).unwrap();
    assert_eq!(map.extent(), 4, "{face_key:?}");
    for edge in edges {
        assert!(
            (1..=map.extent()).any(|i| map.find_key(i).unwrap().is_same(entity(body, edge))),
            "{face_key:?} missing {edge:?}"
        );
    }
}
