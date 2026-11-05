use martensite_tkbo::bop_ds::DataStructure;
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Shape, Vertex},
};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::gp::Point3;
pub fn vertex_at(x: f64, tolerance: f64) -> Vertex {
    let mut vertex = Vertex::default();
    BRepBuilder::default()
        .make_vertex_at(&mut vertex, Point3::new(x, 2., -3.), tolerance)
        .unwrap();
    vertex
}

pub fn initialized(arguments: &[Shape], fuzz: f64) -> DataStructure {
    let mut structure = DataStructure::default();
    structure.set_arguments(arguments);
    structure.init(fuzz).unwrap();
    structure
}

pub fn degenerated_edge(vertex: &Vertex) -> martensite_tkbrep::topods::Edge {
    let builder = BRepBuilder::default();
    let mut edge = martensite_tkbrep::topods::Edge::default();
    builder.make_edge(&mut edge).unwrap();
    builder.degenerated(&edge, true).unwrap();
    builder.add(&mut edge, vertex).unwrap();
    builder
        .add(&mut edge, &vertex.oriented(Orientation::Reversed))
        .unwrap();
    edge
}

pub fn assert_population(structure: &DataStructure, expected: [i32; 3]) {
    assert_eq!(
        [
            structure.nb_shapes(),
            structure.nb_source_shapes(),
            structure.nb_ranges()
        ],
        expected
    );
}
pub fn assert_indices(structure: &DataStructure, shapes: &[&Shape], expected: &[i32]) {
    let actual: Vec<i32> = shapes.iter().map(|shape| structure.index(shape)).collect();
    assert_eq!(actual, expected);
}
pub fn assert_forward_root(structure: &DataStructure) {
    let stored = structure.shape(0).unwrap();
    assert_eq!(stored.orientation(), Orientation::Forward);
}
pub fn assert_vertex_bounds(structure: &DataStructure, expected: [f64; 6]) {
    let actual = structure.shape_info(0).unwrap().box_bounds().get().unwrap();
    assert_eq!(actual.map(f64::to_bits), expected.map(f64::to_bits));
}
pub fn move_vertex(vertex: &Vertex) {
    BRepBuilder::default()
        .set_vertex_point_and_tolerance(vertex, Point3::new(4., 2., -3.), 1. / 64.)
        .unwrap();
}
pub fn assert_degenerated_children(structure: &DataStructure) {
    for index in [0, 2] {
        let info = structure.shape_info(index).unwrap();
        assert_eq!(info.sub_shapes(), [1]);
        assert_eq!(info.flag(), index);
    }
}
pub fn check_single_vertex_gap(fuzz: f64, tolerance: f64, confusion: f64) {
    let vertex = vertex_at(1., tolerance);
    let structure = initialized(&[vertex.into_shape()], fuzz);
    let half_fuzz = if fuzz <= confusion {
        confusion / 2.
    } else {
        fuzz / 2.
    };
    let gap = tolerance + half_fuzz;
    let bounds = structure.shape_info(0).unwrap().box_bounds();
    assert_eq!(bounds.gap().to_bits(), gap.to_bits());
    assert_vertex_bounds(
        &structure,
        [1. - gap, 2. - gap, -3. - gap, 1. + gap, 2. + gap, -3. + gap],
    );
    assert_population(&structure, [1, 1, 1]);
}

pub fn duplicate_arguments() -> (Shape, Shape, Shape, [Shape; 4]) {
    let first = vertex_at(1., 1. / 128.).into_shape();
    let second = vertex_at(4., 1. / 128.).into_shape();
    let reversed = first.oriented(Orientation::Reversed);
    let arguments = [
        first.clone(),
        reversed.clone(),
        second.clone(),
        first.clone(),
    ];
    (first, second, reversed, arguments)
}
