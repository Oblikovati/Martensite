//! Four direct children are explicit authored role slots in the one serialized graph.
use super::spec::FixtureSpec;
use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_tools,
    topods::{self, Edge, Face, Shape, ShapeIterator},
};
use martensite_tkernel::{message::ProgressRange, storage::IoStream};

pub(super) struct GraphRoles {
    pub root: Shape,
    pub complete: Shape,
    pub edge: Edge,
    pub first: Face,
    pub second: Face,
}

fn read_root(spec: &FixtureSpec) -> Shape {
    let header = b"DBRep_DrawableShape";
    assert!(
        spec.brep.starts_with(header),
        "{}: expected graph header",
        spec.name
    );
    let mut stream = IoStream::from_bytes(spec.brep[header.len()..].to_vec());
    let mut root = Shape::default();
    brep_tools::read(
        &mut root,
        &mut stream,
        &BRepBuilder::default(),
        &ProgressRange::default(),
    )
    .unwrap();
    root
}

/// Resolve the fixed serialization slots, retaining shared TShape identity; `read(spec)`.
pub(super) fn read(spec: &FixtureSpec) -> GraphRoles {
    let root = read_root(spec);
    let roles: Vec<Shape> = ShapeIterator::new(&root, true, true).collect();
    assert_eq!(
        roles.len(),
        4,
        "{}: expected four authored roles",
        spec.name
    );
    GraphRoles {
        root,
        complete: roles[0].clone(),
        edge: topods::edge(&roles[1]).unwrap(),
        first: topods::face(&roles[2]).unwrap(),
        second: topods::face(&roles[3]).unwrap(),
    }
}

/// Snapshot the full graph before and after query execution; `serialized(&roles.root)`.
pub(super) fn serialized(shape: &Shape) -> String {
    let mut output = IoStream::from_bytes(Vec::new());
    brep_tools::write(shape, &mut output, &ProgressRange::default()).unwrap();
    crate::geometry_text::printed_text(output)
}
