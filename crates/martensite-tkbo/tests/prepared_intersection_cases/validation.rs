#[path = "../boolean_section_endpoints.rs"]
mod boolean_section_endpoints;
use boolean_section_endpoints::alone_vertices;
#[path = "../boolean_length_assertion.rs"]
mod boolean_length_assertion;
use boolean_length_assertion::check_length;
const KINDS: [ShapeEnum; 8] = [
    ShapeEnum::Vertex,
    ShapeEnum::Edge,
    ShapeEnum::Wire,
    ShapeEnum::Face,
    ShapeEnum::Shell,
    ShapeEnum::Solid,
    ShapeEnum::CompSolid,
    ShapeEnum::Compound,
];
use martensite_tkbrep::{
    brep::BRepTool,
    topexp,
    topods::{self, Shape, ShapeIterator},
    toptools::IndexedMapOfShape,
};
use martensite_tkernel::storage::format_general;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;

pub(super) fn check_section(result: &Shape) {
    assert!(
        BRepCheckAnalyzer::new(result, true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );
    check_length(result, 67.026);
    check_section_counts(result);
    // The source reports checksection without -r; the independent observation is zero.
    assert_eq!(
        alone_vertices(result),
        0,
        "bug23998: section endpoint report"
    );
}

fn check_section_counts(result: &Shape) {
    let mut shapes = IndexedMapOfShape::default();
    topexp::map_shapes(result, &mut shapes, true, true);
    assert_eq!(shapes.extent(), 3, "bug23998: total shapes");
    for kind in KINDS {
        let count = (1..=shapes.extent())
            .filter(|i| shapes.find_key(*i).unwrap().shape_type().unwrap() == kind)
            .count();
        let expected = usize::from(matches!(
            kind,
            ShapeEnum::Vertex | ShapeEnum::Edge | ShapeEnum::Compound
        ));
        assert_eq!(count, expected, "bug23998: {kind:?}");
    }
}

pub(super) fn check_cut(result: &Shape) {
    let edges: Vec<Shape> = ShapeIterator::new(result, true, true).collect();
    assert_eq!(edges.len(), 4, "bug24811: expected four immediate children");
    let vertices: Vec<Shape> = ShapeIterator::new(&edges[1], true, true).collect();
    assert_eq!(
        vertices.len(),
        2,
        "bug24811: result_2 must contain two endpoints"
    );
    check_endpoint(&vertices[0], [-2.22458486160362e-16, 1., 0.]);
    check_endpoint(&vertices[1], [0.0202691578002498, 0.999794559518151, 0.]);
}

fn check_endpoint(shape: &Shape, expected: [f64; 3]) {
    let vertex = topods::vertex(shape).unwrap();
    let actual = BRepTool::pnt(&vertex).unwrap();
    let tolerance = printed(BRepTool::vertex_tolerance(&vertex).unwrap(), 15);
    for (reference, coordinate) in expected
        .into_iter()
        .zip([actual.x(), actual.y(), actual.z()])
    {
        let coordinate = printed(coordinate, 15);
        assert!(
            reference >= coordinate - tolerance && reference <= coordinate + tolerance,
            "bug24811: coordinate {coordinate}, expected {reference} within vertex tolerance {tolerance}"
        );
    }
}

fn printed(value: f64, precision: usize) -> f64 {
    format_general(value, precision)
        .parse()
        .expect("original command numeric text")
}
