//! Checks a case keeps beyond its script: an earlier test of the same case made them, they pass,
//! and dropping them would weaken it. They assert directly and write nothing, so the graded
//! stream stays what the script writes. A body calls them after its script's statements, under
//! a `// Beyond the script` comment, on the names the script left bound.
use super::{
    commands::distinct_parts,
    nbshapes::{Counting, shape_counts},
    properties::Prop,
    session::Session,
};
use martensite_tkbrep::{
    brep::BRepTool,
    topexp::{self, Explorer},
    topods::{self, Shape, ShapeIterator},
    toptools::IndexedMapOfShape,
};
use martensite_tkernel::storage::format_general;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::{brep_check::BRepCheckAnalyzer, brep_gprop::BRepGProp};

/// The epsilon the adaptive properties of a check run with.
const EPSILON: f64 = 1.0e-4;

fn bound<'a>(s: &'a dyn Session, name: &str) -> &'a Shape {
    s.shape(name)
        .unwrap_or_else(|| panic!("beyond the script: {name} is not bound"))
}

fn adaptive_mass(kind: Prop, shape: &Shape) -> f64 {
    let mut properties = GProps::default();
    match kind {
        Prop::Length => {
            BRepGProp::linear_properties(shape, &mut properties, false, false).map(|()| 0.)
        }
        Prop::Area => BRepGProp::surface_properties_eps(shape, &mut properties, EPSILON, false),
        Prop::Volume => {
            BRepGProp::volume_properties_eps(shape, &mut properties, EPSILON, false, false)
        }
    }
    .unwrap_or_else(|failure| panic!("{}: {failure:?}", kind.word()));
    properties.mass()
}

/// The mass prints exactly as the reference run printed it, six significant digits.
/// Example: `printed_mass(s, "pr1", Prop::Area, "88000")`.
pub fn printed_mass(s: &dyn Session, name: &str, kind: Prop, expected: &str) {
    let printed = format_general(adaptive_mass(kind, bound(s, name)), 6);
    assert_eq!(printed, expected, "{} of {name}", kind.word());
}

/// The rule without an epsilon, as `checkview -s` measures, shows `name` rather than its
/// fallback. Example: `view_mass_is_nonzero(s, "result")`.
pub fn view_mass_is_nonzero(s: &dyn Session, name: &str) {
    let mut properties = GProps::default();
    BRepGProp::surface_properties(bound(s, name), &mut properties, false, false)
        .unwrap_or_else(|failure| panic!("area of {name}: {failure:?}"));
    let printed = format_general(properties.mass(), 6);
    let mass: f64 = printed
        .parse()
        .unwrap_or_else(|cause| panic!("area of {name} printed {printed:?}: {cause}"));
    assert_ne!(mass, 0., "area of {name}: expected the view to show it");
}

/// Every count of `nbshapes name`, in the order of its dump.
/// Example: `all_counts(s, "result", [4, 3, 0, 0, 0, 0, 0, 1, 8])`.
pub fn all_counts(s: &dyn Session, name: &str, expected: [usize; 9]) {
    assert_eq!(
        shape_counts(s, name, Counting::Located),
        expected,
        "counts of {name}"
    );
}

/// Length, area and volume are all exactly 0; `masses_are_zero(s, "r_0")`.
pub fn masses_are_zero(s: &dyn Session, name: &str) {
    for kind in [Prop::Length, Prop::Area, Prop::Volume] {
        let mass = adaptive_mass(kind, bound(s, name));
        assert_eq!(mass, 0., "{} of {name}", kind.word());
    }
}

/// Length, area and volume agree within `1e-4 + 1%` of the reference shape's.
/// Example: `same_masses(s, "r1_1", "r_1")`.
pub fn same_masses(s: &dyn Session, name: &str, reference: &str) {
    for kind in [Prop::Length, Prop::Area, Prop::Volume] {
        let actual = adaptive_mass(kind, bound(s, name));
        let wanted = adaptive_mass(kind, bound(s, reference));
        assert!(
            (actual - wanted).abs() <= EPSILON + 0.01 * wanted.abs(),
            "{} of {name}: {actual}, expected that of {reference}, {wanted}",
            kind.word()
        );
    }
}

/// The kind of the shape `name` holds; `kind_is(s, "a", ShapeEnum::Solid)`.
pub fn kind_is(s: &dyn Session, name: &str, expected: ShapeEnum) {
    let kind = bound(s, name)
        .shape_type()
        .unwrap_or_else(|failure| panic!("kind of {name}: {failure:?}"));
    assert_eq!(kind, expected, "kind of {name}");
}

/// How many parts `explode name [kind]` finds; `part_count(s, "a", Some(ShapeEnum::Face), 6)`.
pub fn part_count(s: &dyn Session, name: &str, kind: Option<ShapeEnum>, expected: usize) {
    let shape = bound(s, name);
    let found = match kind {
        Some(kind) => distinct_parts(shape, kind).len(),
        None => ShapeIterator::new(shape, true, true).count(),
    };
    assert_eq!(found, expected, "parts of {name} of kind {kind:?}");
}

/// The rotation matrix of a native run, printed to 15 digits, for the angles the scripts use.
fn native_rotation(degrees: f64) -> [[&'static str; 4]; 3] {
    if degrees == 45. {
        return [
            ["0.707106781186548", "-0.707106781186547", "0", "0"],
            ["0.707106781186547", "0.707106781186548", "0", "0"],
            ["0", "0", "1", "0"],
        ];
    }
    assert_eq!(degrees, 34.73, "native rotation: expected 45 or 34.73");
    [
        ["0.82184585428519", "-0.569709918988819", "0", "0"],
        ["0.569709918988819", "0.82184585428519", "0", "0"],
        ["0", "0", "1", "0"],
    ]
}

/// The location of `name` prints as a native run's rotation by `degrees` about z.
/// Example: `rotation_is(s, "b2", 45.)`.
pub fn rotation_is(s: &dyn Session, name: &str, degrees: f64) {
    let transform = bound(s, name).location().transformation();
    for (row, values) in (1..=3).zip(native_rotation(degrees)) {
        for (column, value) in (1..=4).zip(values) {
            let printed = format_general(transform.value(row, column).unwrap(), 15);
            assert_eq!(
                printed, value,
                "location of {name}, row {row}, column {column}"
            );
        }
    }
}

/// Each tolerance of `kind` in `name`, in the order its unique map keeps them.
/// Example: `tolerances(s, "a", ShapeEnum::Edge)`.
pub fn tolerances(s: &dyn Session, name: &str, kind: ShapeEnum) -> Vec<f64> {
    let mut parts = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(bound(s, name), kind, &mut parts).unwrap();
    parts.iter().map(tolerance_of).collect()
}

fn tolerance_of(part: &Shape) -> f64 {
    match part.shape_type().unwrap() {
        ShapeEnum::Vertex => BRepTool::vertex_tolerance(&topods::vertex(part).unwrap()).unwrap(),
        ShapeEnum::Edge => BRepTool::edge_tolerance(&topods::edge(part).unwrap()).unwrap(),
        ShapeEnum::Face => BRepTool::face_tolerance(&topods::face(part).unwrap()).unwrap(),
        other => panic!("tolerance of {other:?}: expected a vertex, edge or face"),
    }
}

/// Every vertex, edge and face of `name`, and its root, have this lock state and tolerance.
/// Example: `input_state(s, "a", false, 1e-7)`.
pub fn input_state(s: &dyn Session, name: &str, locked: bool, tolerance: f64) {
    let shape = bound(s, name);
    assert_eq!(shape.locked().unwrap(), locked, "lock state of {name}");
    for kind in [ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face] {
        for part in Explorer::new(shape, kind, ShapeEnum::Shape).unwrap() {
            assert_eq!(part.locked().unwrap(), locked, "lock state in {name}");
            assert_eq!(tolerance_of(&part), tolerance, "tolerance in {name}");
        }
    }
}

/// `name` passes the check `checkshape` makes, without writing its answer.
/// Example: `shape_is_valid(s, "result_1")`.
pub fn shape_is_valid(s: &dyn Session, name: &str) {
    let valid = BRepCheckAnalyzer::new(bound(s, name), true, false, false)
        .and_then(|analyzer| analyzer.is_valid())
        .unwrap_or_else(|failure| panic!("validity of {name}: {failure:?}"));
    assert!(valid, "validity of {name}");
}
