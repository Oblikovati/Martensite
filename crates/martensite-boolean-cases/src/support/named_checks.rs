//! The check commands of the bug scripts, on named shapes: each writes what the script's stream
//! holds, so the parse rules grade them. The numbers are read from the Rust results.
use super::{
    bop_session::BopOptions,
    properties::{Prop, props},
    session::Session,
};
use crate::{
    boolean_section_endpoints::odd_incidence_vertices,
    case_bopcheck::{complete_response, configured_checker},
    case_checksection_report::checksection_response,
};
use martensite_tkbo::bop_algo::BOPAlgoArgumentAnalyzer;
use martensite_tkbrep::{
    brep::BRepTool,
    brep_tools::BRepToolsShapeSet,
    topexp,
    topods::Shape,
    toptools::{IndexedMapOfShape, ShapeSet},
};
use martensite_tkernel::storage::format_general;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_extrema::BRepExtremaDistShapeShape;

/// `nbshapes` and `checknbshapes` print these, in this order, with their total last.
const ENTITIES: [(&str, ShapeEnum); 8] = [
    ("VERTEX", ShapeEnum::Vertex),
    ("EDGE", ShapeEnum::Edge),
    ("WIRE", ShapeEnum::Wire),
    ("FACE", ShapeEnum::Face),
    ("SHELL", ShapeEnum::Shell),
    ("SOLID", ShapeEnum::Solid),
    ("COMPSOLID", ShapeEnum::CompSolid),
    ("COMPOUND", ShapeEnum::Compound),
];

/// How `nbshapes` counts: by default each located entity of the shape set, with `-t` each
/// sub-shape once however it is located.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Counting {
    /// `nbshapes name`.
    Default,
    /// `nbshapes name -t`.
    Total,
}

fn entity_count(shape: &Shape, kind: ShapeEnum, counting: Counting) -> usize {
    if counting == Counting::Total {
        let mut found = IndexedMapOfShape::default();
        topexp::map_shapes_of_type(shape, kind, &mut found).expect("nbshapes -t");
        return found.extent();
    }
    let mut entries = BRepToolsShapeSet::default();
    entries.add(shape).expect("nbshapes entries");
    (1..=entries.nb_shapes())
        .filter(|index| entries.shape(*index).unwrap().shape_type().unwrap() == kind)
        .count()
}

fn counts_of(shape: &Shape, counting: Counting) -> [usize; 8] {
    ENTITIES.map(|(_, kind)| entity_count(shape, kind, counting))
}

fn print_counts(name: &str, counts: &[usize; 8]) {
    println!("Number of shapes in {name}");
    for ((label, _), count) in ENTITIES.iter().zip(counts) {
        println!(" {label:<9} : {count}");
    }
}

fn compare_entity(name: &str, label: &str, actual: usize, expected: usize) {
    if actual != expected {
        println!(
            "Error :  is WRONG because number of {label} entities in shape \"{name}\" is {actual}"
        );
    } else {
        println!(
            "OK :  is GOOD because number of {label} entities is equal to number of expected {label} entities"
        );
    }
}

/// `checknbshapes name -vertex v ...`, the listed entities with their expected numbers; a missing
/// shape stops the script in native, so it panics here.
/// Example: `checknbshapes(s, "result", Counting::Default, &[("VERTEX", 7), ("EDGE", 6)])`.
pub fn checknbshapes(
    session: &dyn Session,
    name: &str,
    counting: Counting,
    expected: &[(&str, usize)],
) {
    let shape = session
        .shape(name)
        .unwrap_or_else(|| panic!("nbshapes {name}: no shape"));
    let counts = counts_of(shape, counting);
    print_counts(name, &counts);
    for ((label, _), actual) in ENTITIES.iter().zip(counts) {
        if let Some((_, value)) = expected.iter().find(|(wanted, _)| wanted == label) {
            compare_entity(name, label, actual, *value);
        }
    }
}

/// `checknbshapes name -ref [nbshapes other]`, every entity against the other shape's count.
/// Example: `checknbshapes_ref(s, "r12_0", "r21_0")`.
pub fn checknbshapes_ref(session: &dyn Session, name: &str, other: &str) {
    let shape = session
        .shape(name)
        .unwrap_or_else(|| panic!("nbshapes {name}: no shape"));
    let reference = session
        .shape(other)
        .unwrap_or_else(|| panic!("nbshapes {other}: no shape"));
    let counts = counts_of(shape, Counting::Default);
    let expected = counts_of(reference, Counting::Default);
    // The stream holds the reference twice: its command's report, then the same text echoed as
    // an argument of `checknbshapes`.
    print_counts(other, &expected);
    print_counts(other, &expected);
    print_counts(name, &counts);
    for (index, (label, _)) in ENTITIES.iter().enumerate() {
        compare_entity(name, label, counts[index], expected[index]);
    }
}

/// `checksection name [-r reference]`: binds `alone_N` and returns how many there are; a name
/// with no shape reports none, as in native. Example: `checksection(s, "r_4", Some(0))`.
pub fn checksection(session: &mut dyn Session, name: &str, reference: Option<i32>) -> usize {
    let vertices = session
        .shape(name)
        .map_or_else(Vec::new, odd_incidence_vertices);
    print!("{}", checksection_response(&vertices, reference));
    for (index, vertex) in vertices.iter().enumerate() {
        session.bind(&format!("alone_{}", index + 1), vertex);
    }
    vertices.len()
}

/// `tolerance name`: writes and returns the largest tolerance of its vertices, edges and faces.
/// Example: `maximal_tolerance(&rs) < 5.0e-5`.
pub fn maximal_tolerance(shape: &Shape) -> f64 {
    let maximal = [ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face]
        .into_iter()
        .map(|kind| BRepTool::max_tolerance(shape, kind).expect("tolerance"))
        .fold(0.0, f64::max);
    println!("Tolerance MAX={}", format_general(maximal, 17));
    maximal
}

/// `checkmaxtol name -min_tol limit`; `checkmaxtol(s, "result", 1.0e-4)`.
pub fn checkmaxtol(session: &dyn Session, name: &str, limit: f64) {
    let shape = session
        .shape(name)
        .unwrap_or_else(|| panic!("checkmaxtol {name}: no shape"));
    let maximal = maximal_tolerance(shape);
    if maximal > limit {
        println!(
            "Error: tolerance of \"{name}\" ({maximal}) is greater than checking tolerance ({limit})"
        );
    }
}

/// `bopcheck name` at the session's options, written and returned; `bopcheck(&shape, &options)`.
pub fn bopcheck(shape: &Shape, options: &BopOptions) -> String {
    let mut checker = configured_checker(shape);
    checker.options_mut().set_run_parallel(options.run_parallel);
    checker.options_mut().set_fuzzy_value(options.fuzzy);
    checker.perform();
    let response = complete_response(&checker);
    println!("{response}");
    response
}

/// `bopargcheck name` on one shape, all its single-shape modes on: `true` for "Faulties".
/// Example: `if bopargcheck_faulties(&shape, &options) { println!("Error: ..."); }`.
pub fn bopargcheck_faulties(shape: &Shape, options: &BopOptions) -> bool {
    let mut checker = BOPAlgoArgumentAnalyzer::new();
    checker.options_mut().set_run_parallel(options.run_parallel);
    checker.options_mut().set_fuzzy_value(options.fuzzy);
    checker.set_shape1(shape);
    *checker.argument_type_mode() = true;
    *checker.self_inter_mode() = true;
    *checker.small_edge_mode() = true;
    *checker.rebuild_face_mode() = true;
    *checker.continuity_mode() = true;
    *checker.curve_on_surface_mode() = true;
    checker.perform();
    let faulty = checker.has_faulty();
    let verdict = if faulty {
        "Faulties, that can not be treated by BOP, are detected."
    } else {
        "Shape(s) seem(s) to be valid for BOP."
    };
    println!("{verdict}");
    faulty
}

/// `checkprops first -equal second`: the option selects the length, measured on both, and the
/// printed masses must be equal. Example: `checkprops_equal(s, "r12_0", "r21_0")`.
pub fn checkprops_equal(session: &dyn Session, first: &str, second: &str) {
    let shape = session
        .shape(first)
        .unwrap_or_else(|| panic!("checkprops {first}: no shape"));
    let other = session
        .shape(second)
        .unwrap_or_else(|| panic!("checkprops {second}: no shape"));
    let measured = props(Prop::Length, shape, None);
    let compared = props(Prop::Length, other, None);
    if compared != measured {
        println!("Error: Shape {second} is not equal to shape {first}");
    }
}

/// `distmini di first second`, then the value of `di_val`; `distmini(s, "result_2", "a_3")`.
pub fn distmini(session: &dyn Session, first: &str, second: &str) -> f64 {
    let first = session
        .shape(first)
        .unwrap_or_else(|| panic!("distmini {first}: no shape"));
    let second = session
        .shape(second)
        .unwrap_or_else(|| panic!("distmini {second}: no shape"));
    let distance = BRepExtremaDistShapeShape::with_shapes(first, second, Default::default())
        .expect("distmini");
    assert!(distance.is_done(), "distmini: expected completion");
    let value = distance.value().expect("distmini value");
    println!("{}", format_general(value, 17));
    value
}
