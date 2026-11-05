//! Owned analytic inputs make external entry observations independent of runtime references.
use martensite_tkbo::bop_algo::BuilderFace;
use martensite_tkbrep::{
    brep::BRepTool,
    brep_tools::uv_bounds,
    topexp::Explorer,
    topods::{Face, Shape, Vertex},
};
use martensite_tkernel::ncollection::List;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Plane3;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;

pub(super) struct SquareInput {
    pub builder: BuilderFace,
    pub face: Face,
    pub edges: Vec<Shape>,
}

pub(super) fn square_input() -> SquareInput {
    let face = BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (0., 10., 0., 10.))
        .unwrap()
        .face()
        .unwrap()
        .clone();
    let edges: Vec<Shape> = Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape)
        .unwrap()
        .collect();
    let mut builder = BuilderFace::new();
    builder.set_face(&face);
    builder.set_shapes(&edges.iter().cloned().collect::<List<Shape>>());
    SquareInput {
        builder,
        face,
        edges,
    }
}

#[derive(Debug, PartialEq)]
pub(super) struct SourceObservation {
    bounds: [u64; 4],
    flags: Vec<[bool; 4]>,
    vertices: Vec<[u64; 3]>,
}

pub(super) fn observe_source(input: &SquareInput) -> SourceObservation {
    let (u0, u1, v0, v1) = uv_bounds(&input.face).unwrap();
    let mut shapes = vec![input.face.clone().into_shape()];
    shapes.extend(input.edges.iter().cloned());
    let flags = shapes.iter().map(observe_flags).collect();
    let vertices = Explorer::new(&input.face, ShapeEnum::Vertex, ShapeEnum::Shape)
        .unwrap()
        .map(|shape| observe_vertex(&Vertex::unchecked(shape)))
        .collect();
    SourceObservation {
        bounds: [u0, u1, v0, v1].map(f64::to_bits),
        flags,
        vertices,
    }
}

fn observe_flags(shape: &Shape) -> [bool; 4] {
    [
        shape.free().unwrap(),
        shape.modified().unwrap(),
        shape.checked().unwrap(),
        shape.closed().unwrap(),
    ]
}

fn observe_vertex(vertex: &Vertex) -> [u64; 3] {
    let point = BRepTool::pnt(vertex).unwrap();
    [point.x(), point.y(), point.z()].map(f64::to_bits)
}

pub(super) fn assert_square_result(input: &SquareInput) {
    assert_eq!(input.builder.areas().len(), 1);
    let result = input.builder.areas().first().unwrap();
    assert_eq!(
        uv_bounds(&Face::unchecked(result.clone())).unwrap(),
        (0., 10., 0., 10.)
    );
    let edges: Vec<Shape> = Explorer::new(result, ShapeEnum::Edge, ShapeEnum::Shape)
        .unwrap()
        .collect();
    assert_eq!(edges.len(), 4);
    for source in &input.edges {
        assert_eq!(edges.iter().filter(|edge| edge.is_same(source)).count(), 1);
    }
    assert_single_outer_wire(result);
}

fn assert_single_outer_wire(result: &Shape) {
    let wires: Vec<Shape> = Explorer::new(result, ShapeEnum::Wire, ShapeEnum::Shape)
        .unwrap()
        .collect();
    assert_eq!(wires.len(), 1);
    assert!(
        wires[0].closed().unwrap(),
        "square result must retain a closed boundary wire"
    );
}
