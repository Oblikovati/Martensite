//! Original fixture-free wire and planar-face regression cases, using the actual command operations.
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topexp::{self, Explorer};
use martensite_tkbrep::topods::{Edge, Face, Shape, Vertex, Wire};
use martensite_tkbrep::toptools::IndexedMapOfShape;
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::Failure;
use martensite_tkernel::storage::IoStream;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::geom_tools;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape,
    BRepBuilderAPIMakeVertex, BRepBuilderAPIMakeWire,
};
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;
use std::io::{Read, Seek, SeekFrom};

const UNSORTED_EDGE_ORDERS: [[usize; 4]; 6] = [
    [0, 1, 2, 3],
    [0, 3, 2, 1],
    [1, 0, 2, 3],
    [3, 2, 1, 0],
    [3, 2, 0, 1],
    [2, 0, 1, 3],
];

fn command_vertices(points: &[Point3]) -> Result<Vec<Vertex>, Failure> {
    points
        .iter()
        .map(|point| BRepBuilderAPIMakeVertex::new(*point)?.vertex())
        .collect()
}

fn command_edges(vertices: &[Vertex], pairs: &[(usize, usize)]) -> Result<Vec<Edge>, Failure> {
    pairs
        .iter()
        .map(|&(first, last)| {
            BRepBuilderAPIMakeEdge::from_vertices(&vertices[first], &vertices[last])?.edge()
        })
        .collect()
}

fn command_wire(edges: &[Edge]) -> Result<Wire, Failure> {
    let mut maker = BRepBuilderAPIMakeWire::new();
    for edge in edges {
        maker.add_edge(edge)?;
    }
    assert!(maker.is_done());
    maker.wire()
}

fn closed_wire(points: &[Point3]) -> Result<Wire, Failure> {
    let vertices = command_vertices(points)?;
    let pairs: Vec<(usize, usize)> = (0..points.len())
        .map(|index| (index, (index + 1) % points.len()))
        .collect();
    command_wire(&command_edges(&vertices, &pairs)?)
}

fn unsorted_wire(edges: &[Edge], order: &[usize; 4]) -> Result<Wire, Failure> {
    let mut entries: List<Shape> = List::default();
    for &index in order {
        entries.append(edges[index].clone().into_shape());
    }
    let mut maker = BRepBuilderAPIMakeWire::new();
    maker.add_list(&entries)?;
    assert!(maker.is_done());
    maker.wire()
}

fn assert_branch_wire(wire: &Wire) -> Result<(), Failure> {
    assert!(BRepCheckAnalyzer::new(wire, true, false, false)?.is_valid()?);
    for (kind, expected) in [(ShapeEnum::Vertex, 5), (ShapeEnum::Edge, 4)] {
        let mut unique = IndexedMapOfShape::default();
        topexp::map_shapes_of_type(wire, kind, &mut unique)?;
        assert_eq!(unique.extent(), expected);
    }
    Ok(())
}

#[test]
fn bug27552_1_all_original_unsorted_wire_orders_are_valid() -> Result<(), Failure> {
    crate::isolated_topology_case::run_isolated_case(
        "wire_face_cases::bug27552_1_all_original_unsorted_wire_orders_are_valid",
        || Ok(bug27552_1_all_original_unsorted_wire_orders_are_valid_body()?),
    );
    Ok(())
}

fn bug27552_1_all_original_unsorted_wire_orders_are_valid_body() -> Result<(), Failure> {
    let points = [
        Point3::new(0., 0., 0.),
        Point3::new(-100., 0., 100.),
        Point3::new(100., 0., 100.),
        Point3::new(0., 0., -100.),
        Point3::new(-100., 0., -200.),
    ];
    let vertices = command_vertices(&points)?;
    let edges = command_edges(&vertices, &[(1, 0), (2, 0), (0, 3), (3, 4)])?;
    for order in UNSORTED_EDGE_ORDERS {
        assert_branch_wire(&unsorted_wire(&edges, &order)?)?;
    }
    Ok(())
}

fn dumped_surface(face: &Face) -> Result<String, Failure> {
    let surface = BRepTool::surface(face)?;
    let mut stream = IoStream::from_bytes(Vec::new());
    geom_tools::dump_surface(&surface, &mut stream)?;
    let mut device = stream.into_device().unwrap();
    device.seek(SeekFrom::Start(0)).unwrap();
    let mut bytes = Vec::new();
    device.read_to_end(&mut bytes).unwrap();
    Ok(String::from_utf8(bytes).unwrap())
}

#[test]
fn bug32859_original_trapezoid_plane_dump_has_positive_z_axis() -> Result<(), Failure> {
    crate::isolated_topology_case::run_isolated_case(
        "wire_face_cases::bug32859_original_trapezoid_plane_dump_has_positive_z_axis",
        || Ok(bug32859_original_trapezoid_plane_dump_has_positive_z_axis_body()?),
    );
    Ok(())
}

fn bug32859_original_trapezoid_plane_dump_has_positive_z_axis_body() -> Result<(), Failure> {
    let wire = closed_wire(&[
        Point3::new(-500., 25., -6.),
        Point3::new(-450., -25., -6.),
        Point3::new(450., -25., -6.),
        Point3::new(500., 25., -6.),
    ])?;
    let maker = BRepBuilderAPIMakeFace::from_wire(&wire, false)?;
    assert!(maker.is_done());
    let dump = dumped_surface(&maker.face()?)?;
    assert!(
        dump.contains("Axis   :0, 0, 1"),
        "{dump}; expected Axis   :0, 0, 1"
    );
    Ok(())
}

#[test]
fn bug33156_2_planar_face_retains_original_wire() -> Result<(), Failure> {
    crate::isolated_topology_case::run_isolated_case(
        "wire_face_cases::bug33156_2_planar_face_retains_original_wire",
        || Ok(bug33156_2_planar_face_retains_original_wire_body()?),
    );
    Ok(())
}

fn bug33156_2_planar_face_retains_original_wire_body() -> Result<(), Failure> {
    let original_wire = closed_wire(&[
        Point3::new(0., 0., 0.),
        Point3::new(10., 0., 0.),
        Point3::new(0., 10., 0.),
    ])?;
    let maker = BRepBuilderAPIMakeFace::from_wire(&original_wire, true)?;
    assert!(maker.is_done());
    let face = maker.face()?;
    assert!(!face.is_null());
    assert!(
        Explorer::new(&face, ShapeEnum::Wire, ShapeEnum::Shape)?
            .any(|wire| wire.is_same(&original_wire)),
        "w1 is NOT sub-shape of f1"
    );
    Ok(())
}
