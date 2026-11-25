use super::*;
use Orientation::{Forward, Reversed};

fn indexed(attempt: Attempt, root: Shape) -> DetachedCylinderFeature {
    let index = body::Index::new(attempt, &root, CylinderFeatureStage::Encoding).unwrap();
    let entities = index
        .members()
        .map(|item| {
            let (id, shape) = item.unwrap();
            (id, shape.clone())
        })
        .collect();
    DetachedCylinderFeature {
        body: root,
        entities,
    }
}

fn solid(faces: impl IntoIterator<Item = Shape>) -> Shape {
    let mut shell = topods::Shell::default();
    Builder.make_shell(&mut shell);
    for face in faces {
        Builder.add(&mut shell, &face).unwrap();
    }
    shell.set_closed(true).unwrap();
    let mut root = topods::Solid::default();
    Builder.make_solid(&mut root);
    Builder.add(&mut root, &shell).unwrap();
    root.to_shape()
}

fn attempt() -> Attempt {
    Attempt {
        attachment: FaceAttachmentId(1),
        revision: SupportRevision(2),
        kind: CylinderFeatureKind::Boss,
    }
}

fn vertex() -> topods::Vertex {
    let mut vertex = topods::Vertex::default();
    BRepBuilder::default().make_vertex(&mut vertex);
    vertex
}

fn edge(from: &topods::Vertex, to: &topods::Vertex) -> topods::Edge {
    let mut edge = topods::Edge::default();
    BRepBuilder::default().make_edge(&mut edge).unwrap();
    Builder.add(&mut edge, &from.oriented(Forward)).unwrap();
    Builder.add(&mut edge, &to.oriented(Reversed)).unwrap();
    edge
}

fn pillow(anchor: &topods::Vertex, other_sense: Orientation) -> [Shape; 2] {
    let [v1, v2] = [vertex(), vertex()];
    let edges = [edge(anchor, &v1), edge(&v1, &v2), edge(&v2, anchor)];
    [Forward, other_sense].map(|sense| {
        let mut wire = topods::Wire::default();
        Builder.make_wire(&mut wire);
        for edge in &edges {
            Builder.add(&mut wire, edge).unwrap();
        }
        wire.set_closed(true).unwrap();
        let mut face = topods::Face::default();
        BRepBuilder::default().make_face(&mut face);
        Builder.add(&mut face, &wire).unwrap();
        face.oriented(sense)
    })
}

fn topology(faces: impl IntoIterator<Item = Shape>) -> Checked<Topology> {
    let attempt = attempt();
    let body = indexed(attempt, solid(faces));
    let index = body::Index::new(attempt, &body.body, CylinderFeatureStage::Encoding).unwrap();
    Topology::capture_indexed(attempt, &body, &index, None)
}

#[test]
fn equal_edge_use_is_typed_from_actual_native_faces() {
    assert!(topology(pillow(&vertex(), Reversed)).is_ok());
    assert!(matches!(
        topology(pillow(&vertex(), Forward)),
        Err(Refusal::EdgeSenses {
            actual: [Forward, Forward],
            ..
        })
    ));
}

#[test]
fn disconnected_native_vertex_fan_is_refused_before_shell_component() {
    let anchor = vertex();
    let faces = pillow(&anchor, Reversed)
        .into_iter()
        .chain(pillow(&anchor, Reversed));
    assert!(matches!(
        topology(faces),
        Err(Refusal::VertexFan {
            connected: 2,
            actual: 4,
            ..
        })
    ));
}

#[test]
fn archive_reference_must_keep_native_occurrence_orientation() {
    let attempt = attempt();
    let mut owner = indexed(attempt, solid(pillow(&vertex(), Reversed)));
    let entry = owner
        .entities
        .iter_mut()
        .find(|(id, _)| id.kind == ShapeEnum::Edge)
        .unwrap();
    let (id, shape) = entry;
    let id = *id;
    *shape = shape.reversed();
    let index = body::Index::new(attempt, &owner.body, CylinderFeatureStage::Encoding).unwrap();
    assert!(
        matches!(Topology::capture_indexed(attempt, &owner, &index, None),
        Err(Refusal::Inventory(identity)) if identity == id)
    );
}
