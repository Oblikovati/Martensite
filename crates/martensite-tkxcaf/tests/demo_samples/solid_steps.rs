//! Modelling steps of the demonstration documents, with the default settings of each operation.
use martensite_tkbo::bop_algo::{BOPAlgoBOP, Glue, Operation};
use martensite_tkbrep::brep_tools::BRepToolsShapeSet;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{Shape, ShapeIterator};
use martensite_tkbrep::toptools::{MapOfShape, ShapeSet as _};
use martensite_tkernel::precision;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{Axis1, Axis2, Direction3, Point3, Transform3, Vector3};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeShape as _, BRepBuilderAPITransform,
};
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;
use martensite_tktopalgo::brep_gprop::BRepGProp;

/// Counts in the order vertex, edge, wire, face, shell, solid, compsolid, compound, shape.
pub type TopologyCounts = [usize; 9];

const COUNT_LABELS: [&str; 9] = [
    "VERTEX",
    "EDGE",
    "WIRE",
    "FACE",
    "SHELL",
    "SOLID",
    "COMPSOLID",
    "COMPOUND",
    "SHAPE",
];

/// A boolean with every option at its default; `boolean(&a, &b, Operation::Cut)`.
pub fn boolean(object: &Shape, tool: &Shape, operation: Operation) -> Shape {
    let mut algorithm = BOPAlgoBOP::new();
    algorithm.add_argument(object);
    algorithm.add_tool(tool);
    algorithm.set_operation(operation);
    algorithm.set_glue(Glue::Off);
    algorithm
        .options_mut()
        .set_fuzzy_value(precision::confusion());
    algorithm.set_non_destructive(false);
    algorithm.options_mut().set_run_parallel(false);
    algorithm.options_mut().set_use_obb(false);
    algorithm.set_check_inverted(true);
    algorithm.set_to_fill_history(true);
    algorithm.perform();
    assert!(
        !algorithm.options().has_errors(),
        "boolean {operation:?} reported errors"
    );
    let result = algorithm.shape().clone();
    assert!(!result.is_null(), "boolean {operation:?} returned no shape");
    result
}

/// Translate a shape as a transformation without copying; `translated(&shape, 0., 0., 7.)`.
pub fn translated(shape: &Shape, x: f64, y: f64, z: f64) -> Shape {
    let mut transform = Transform3::default();
    transform.set_translation(Vector3::new(x, y, z));
    transformed(shape, transform)
}

/// Rotate a shape about the axis through the origin, by degrees; `rotated(&s, [0., 0., 1.], -45.)`.
pub fn rotated(shape: &Shape, axis: [f64; 3], degrees: f64) -> Shape {
    let direction = Direction3::new(axis[0], axis[1], axis[2]).unwrap();
    let mut transform = Transform3::default();
    transform
        .set_rotation(
            Axis1::new(Point3::new(0., 0., 0.), direction),
            degrees.to_radians(),
        )
        .unwrap();
    transformed(shape, transform)
}

/// Mirror a shape in the plane through the origin normal to `normal`; the result has new geometry.
/// Example: `mirrored(&shape, [0., 0., 1.])`.
pub fn mirrored(shape: &Shape, normal: [f64; 3]) -> Shape {
    let direction = Direction3::new(normal[0], normal[1], normal[2]).unwrap();
    let mut transform = Transform3::default();
    transform.set_mirror_plane(Axis2::new(Point3::new(0., 0., 0.), direction).unwrap());
    transformed(shape, transform)
}

fn transformed(shape: &Shape, transform: Transform3) -> Shape {
    let mut transformer = BRepBuilderAPITransform::new(transform);
    transformer.perform(shape, false, false).unwrap();
    transformer.shape().unwrap().clone()
}

/// The children of a shape, with cumulated orientation and location; `children(&compound)`.
pub fn children(shape: &Shape) -> Vec<Shape> {
    ShapeIterator::new(shape, true, true).collect()
}

/// The sub-shapes of one kind, each once, the shape itself excluded; `distinct(&s, ShapeEnum::Face)`.
pub fn distinct(shape: &Shape, kind: ShapeEnum) -> Vec<Shape> {
    let mut seen = MapOfShape::default();
    seen.add(shape.clone());
    let mut parts = Vec::new();
    let mut explorer = Explorer::new(shape, kind, ShapeEnum::Shape).unwrap();
    while explorer.more() {
        let current = explorer.current().clone();
        if seen.add(current.clone()) {
            parts.push(current);
        }
        explorer.next().unwrap();
    }
    parts
}

/// Assert the unique topology of a shape; `assert_counts("body", &s, [2, 3, 3, 3, 1, 1, 0, 0, 13])`.
pub fn assert_counts(name: &str, shape: &Shape, expected: TopologyCounts) {
    let mut set = BRepToolsShapeSet::default();
    set.add(shape).unwrap();
    let mut text = AsciiString::default();
    set.dump_extent_string(&mut text).unwrap();
    let text = String::from_utf8_lossy(text.as_bytes()).into_owned();
    let counts: TopologyCounts = COUNT_LABELS.map(|label| count_of(&text, label));
    assert_eq!(counts, expected, "{name}: {text}");
}

fn count_of(extent: &str, label: &str) -> usize {
    let line = extent
        .lines()
        .find(|line| line.trim_start().starts_with(&format!("{label} ")))
        .unwrap_or_else(|| panic!("no {label} line in {extent:?}"));
    line.rsplit(':').next().unwrap().trim().parse().unwrap()
}

/// Assert a valid shape with its volume and area to the six digits they were measured with.
/// Example: `assert_valid_measures("pin", &shape, 31.4159, 69.115)`.
pub fn assert_valid_measures(name: &str, shape: &Shape, volume: f64, area: f64) {
    let check = BRepCheckAnalyzer::new(shape, true, false, false).unwrap();
    assert!(check.is_valid().unwrap(), "{name} is not valid");
    let mut properties = GProps::default();
    BRepGProp::volume_properties_eps(shape, &mut properties, 1e-6, false, false).unwrap();
    assert_six_digits(name, "volume", properties.mass(), volume);
    let mut properties = GProps::default();
    BRepGProp::surface_properties_eps(shape, &mut properties, 1e-6, false).unwrap();
    assert_six_digits(name, "area", properties.mass(), area);
}

fn assert_six_digits(name: &str, what: &str, actual: f64, expected: f64) {
    let allowed = 1e-5 * expected.abs().max(1.);
    assert!(
        (actual - expected).abs() <= allowed,
        "{name} {what}: {actual} against {expected} within {allowed}"
    );
}
