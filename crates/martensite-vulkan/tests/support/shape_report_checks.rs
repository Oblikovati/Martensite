//! Typed checks of a modelling result that print the regressions' report lines: sub-shape
//! counts, the maximum tolerance, free boundary edges, lone section vertices and validity.
use crate::shape_mesh::check_real;
use martensite_tkbrep::brep_tools::BRepToolsShapeSet;
use martensite_tkbrep::topexp::{Explorer, map_shapes_of_type};
use martensite_tkbrep::topods::Shape;
use martensite_tkbrep::toptools::{IndexedMapOfShape, MapOfShape, ShapeSet};
use martensite_tkernel::standard::Failure;
use martensite_tkernel::storage::format_general;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkshhealing::shape_analysis::{
    ShapeAnalysisFreeBounds, ShapeAnalysisShapeTolerance,
};
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;

/// The kinds a count report lists, vertices first, the total last.
const COUNT_ORDER: [(ShapeEnum, &str); 9] = [
    (ShapeEnum::Vertex, "VERTEX"),
    (ShapeEnum::Edge, "EDGE"),
    (ShapeEnum::Wire, "WIRE"),
    (ShapeEnum::Face, "FACE"),
    (ShapeEnum::Shell, "SHELL"),
    (ShapeEnum::Solid, "SOLID"),
    (ShapeEnum::CompSolid, "COMPSOLID"),
    (ShapeEnum::Compound, "COMPOUND"),
    (ShapeEnum::Shape, "SHAPE"),
];

/// Expected sub-shape counts in vertex-first order (`None` is not compared), whether
/// sub-shapes with different locations count apart, and the report's subject.
/// Example: `ShapeCounts::all([3, 2, 0, 0, 0, 0, 0, 1, 6])`.
#[derive(Clone, Copy, Debug, Default)]
pub struct ShapeCounts {
    /// Expected counts, vertex-first, the total last.
    pub expected: [Option<usize>; 9],
    /// Count a sub-shape once per location.
    pub per_location: bool,
    /// The subject named in each report line.
    pub subject: &'static str,
}

impl ShapeCounts {
    /// Every count given, vertex-first; `ShapeCounts::all([4, 4, 2, 2, 1, 0, 0, 0, 13])`.
    pub fn all(counts: [usize; 9]) -> Self {
        Self {
            expected: counts.map(Some),
            ..Self::default()
        }
    }

    /// Only the shell count; `ShapeCounts::shells(2)`.
    pub fn shells(count: usize) -> Self {
        let mut expected = [None; 9];
        expected[4] = Some(count);
        Self {
            expected,
            ..Self::default()
        }
    }
}

/// Compare the shape's sub-shape counts with the expectation, printing an OK or an error
/// line per compared kind; `check_shape_counts(&result, "result", &ShapeCounts::shells(1))?`.
pub fn check_shape_counts(
    shape: &Shape,
    name: &str,
    expected: &ShapeCounts,
) -> Result<[usize; 9], Failure> {
    let observed = if expected.per_location {
        located_counts(shape)?
    } else {
        shape_set_counts(shape)?
    };
    for (index, (_, label)) in COUNT_ORDER.iter().enumerate() {
        let Some(count) = expected.expected[index] else {
            continue;
        };
        report_count(expected.subject, label, name, observed[index], count);
    }
    Ok(observed)
}

fn report_count(subject: &str, label: &str, name: &str, observed: usize, expected: usize) {
    if observed == expected {
        println!(
            "OK : {subject} is GOOD because number of {label} entities is equal to number of expected {label} entities"
        );
    } else {
        println!(
            "Error : {subject} is WRONG because number of {label} entities in shape \"{name}\" is {observed}"
        );
    }
}

/// Location-normalized counts of a shape set holding the shape, vertex-first.
fn shape_set_counts(shape: &Shape) -> Result<[usize; 9], Failure> {
    let mut set = BRepToolsShapeSet::default();
    set.add(shape)?;
    let mut counts = [0; 9];
    for index in 1..=set.nb_shapes() {
        let kind = set.shape(index)?.shape_type()?;
        counts[count_slot(kind)] += 1;
    }
    counts[8] = set.nb_shapes() as usize;
    Ok(counts)
}

/// Counts of distinct located sub-shapes of each kind, vertex-first.
fn located_counts(shape: &Shape) -> Result<[usize; 9], Failure> {
    let mut counts = [0; 9];
    for (index, (kind, _)) in COUNT_ORDER[..8].iter().enumerate() {
        let mut map = IndexedMapOfShape::default();
        map_shapes_of_type(shape, *kind, &mut map)?;
        counts[index] = map.extent();
        counts[8] += map.extent();
    }
    Ok(counts)
}

fn count_slot(kind: ShapeEnum) -> usize {
    COUNT_ORDER
        .iter()
        .position(|(slot, _)| *slot == kind)
        .expect("every shape kind has a count slot")
}

/// The largest vertex, edge or face tolerance of the shape, as the 17-digit text the
/// report reads, compared with `reference` (absolute 1e-4, relative 1%).
/// Example: `check_max_tolerance(&result, "1.05")?`.
pub fn check_max_tolerance(shape: &Shape, reference: &str) -> Result<f64, Failure> {
    let mut tolerances = ShapeAnalysisShapeTolerance::new();
    tolerances.init_tolerance();
    tolerances.add_tolerance(shape, ShapeEnum::Shape)?;
    let printed: f64 = format_general(tolerances.global_tolerance(1), 17)
        .parse()
        .expect("a formatted tolerance parses back");
    let expected: f64 = reference
        .parse()
        .unwrap_or_else(|_| panic!("reference tolerance {reference:?}; expected a real"));
    check_real("Max tolerance", printed, expected, 0.0001, 0.01);
    Ok(printed)
}

/// Count the edges of the closed free boundaries of the shape's shells and compare them
/// with `expected`; `check_free_bounds(&result, 0)?`.
pub fn check_free_bounds(shape: &Shape, expected: usize) -> Result<usize, Failure> {
    let bounds = ShapeAnalysisFreeBounds::from_shape(shape, false, false, false)?;
    let edges = distinct_count(bounds.closed_wires().as_ref(), ShapeEnum::Edge)?;
    if edges == expected {
        println!("OK : Number of free edges is {edges}");
    } else {
        println!("Error : Number of free edges is not equal to reference data");
    }
    Ok(edges)
}

/// The distinct sub-shapes of `kind`, as an explode lists them.
fn distinct_count(shape: &Shape, kind: ShapeEnum) -> Result<usize, Failure> {
    let mut seen = MapOfShape::default();
    let mut explorer = Explorer::new(shape, kind, ShapeEnum::Shape)?;
    while explorer.more() {
        seen.add(explorer.current().clone());
        explorer.next()?;
    }
    Ok(seen.extent())
}

/// Print the vertices met an odd number of times in the shape (lone section ends) and
/// return their count; `check_section_vertices(&result)?`.
pub fn check_section_vertices(shape: &Shape) -> Result<usize, Failure> {
    let mut alone = MapOfShape::default();
    let mut explorer = Explorer::new(shape, ShapeEnum::Vertex, ShapeEnum::Shape)?;
    while explorer.more() {
        let vertex = explorer.current().clone();
        if !alone.add(vertex.clone()) {
            alone.remove(&vertex);
        }
        explorer.next()?;
    }
    println!(" nb alone Vertices : {}", alone.extent());
    let names: Vec<String> = (1..=alone.extent())
        .map(|n| format!("alone_{n} "))
        .collect();
    println!("{}", names.concat());
    Ok(alone.extent())
}

/// Check topology and geometry of the shape; prints "This shape seems to be valid" or the
/// faulty-shape line with the number of distinct faulty sub-shapes.
/// Example: `check_shape_structure(&result)?`.
pub fn check_shape_structure(shape: &Shape) -> Result<bool, Failure> {
    let analyzer = BRepCheckAnalyzer::new(shape, true, false, false)?;
    if analyzer.is_valid()? {
        println!("This shape seems to be valid");
        return Ok(true);
    }
    let faulty = faulty_count(&analyzer, shape)?;
    println!("Faulty shapes in variables faulty_1 to faulty_{faulty} ");
    Ok(false)
}

fn faulty_count(analyzer: &BRepCheckAnalyzer, shape: &Shape) -> Result<usize, Failure> {
    let mut faulty = 0;
    for (kind, _) in &COUNT_ORDER[..8] {
        let mut map = IndexedMapOfShape::default();
        map_shapes_of_type(shape, *kind, &mut map)?;
        for index in 1..=map.extent() {
            faulty += usize::from(!analyzer.is_valid_shape(map.find_key(index)?)?);
        }
    }
    Ok(faulty)
}

/// Print the maximum, average and minimum tolerance of all sub-shapes, then of faces, edges
/// and vertices, as a tolerance analysis reports them; `print_tolerances(&tool)?`.
pub fn print_tolerances(shape: &Shape) -> Result<(), Failure> {
    let rows = [
        ("Tolerance MAX=", ShapeEnum::Shape),
        ("FACE    : MAX=", ShapeEnum::Face),
        ("EDGE    : MAX=", ShapeEnum::Edge),
        ("VERTEX  : MAX=", ShapeEnum::Vertex),
    ];
    let mut tolerances = ShapeAnalysisShapeTolerance::new();
    for (title, kind) in rows {
        tolerances.init_tolerance();
        tolerances.add_tolerance(shape, kind)?;
        let [max, average, min] =
            [1, 0, -1].map(|mode| format_general(tolerances.global_tolerance(mode), 17));
        println!("{title}{max} AVG={average} MIN={min}");
    }
    Ok(())
}
