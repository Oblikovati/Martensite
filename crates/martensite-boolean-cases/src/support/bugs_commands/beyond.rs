//! Checks a case keeps beyond its script, as `super::super::beyond_script`: an earlier test of the
//! same case made them, they pass, and they write nothing to the graded stream.
use super::options_bop::ScriptBop;
use crate::support::{commands::bound_shape, session::Session};
use martensite_tkbo::bop_algo::{BOPAlgoArgumentAnalyzer, CheckStatus};
use martensite_tkbrep::topods::ShapeIterator;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};

use crate::boolean_section_endpoints as section_endpoints;

/// The direct children of `name` have these kinds, in order.
/// Example: `child_kinds(s, "a", &[ShapeEnum::Solid, ShapeEnum::Shell])`.
pub fn child_kinds(s: &dyn Session, name: &str, expected: &[ShapeEnum]) {
    let shape = bound_shape(s, name);
    let mut iterator = ShapeIterator::new(&shape, true, true);
    let mut kinds = Vec::new();
    while let Ok(child) = iterator.value() {
        kinds.push(child.shape_type().unwrap());
        iterator.next();
    }
    assert_eq!(kinds, expected, "children of {name}");
}

/// `name` has `expected` direct children; `child_count(s, "c", 2)`.
pub fn child_count(s: &dyn Session, name: &str, expected: usize) {
    let shape = bound_shape(s, name);
    let mut iterator = ShapeIterator::new(&shape, true, true);
    let mut count = 0;
    while iterator.value().is_ok() {
        count += 1;
        iterator.next();
    }
    assert_eq!(count, expected, "children of {name}");
}

/// `name` has `expected` distinct parts of `kind`; `part_count(s, "b", ShapeEnum::Face, 6)`.
pub fn part_count(s: &dyn Session, name: &str, kind: ShapeEnum, expected: usize) {
    let found = crate::support::commands::distinct_parts(&bound_shape(s, name), kind).len();
    assert_eq!(found, expected, "{kind:?} parts of {name}");
}

/// `name` holds a shape and `other` none; `bound_not_other(s, "x1", "x0")`.
pub fn bound_not_other(s: &dyn Session, name: &str, other: &str) {
    assert!(s.shape(name).is_some(), "{name}: expected a shape");
    assert!(s.shape(other).is_none(), "{other}: expected no shape");
}

/// `bopargcheck name` finds faults of exactly these kinds, in order.
/// Example: `argcheck_statuses(s, "result", &[CheckStatus::SelfIntersect])`.
pub fn argcheck_statuses(s: &dyn Session, name: &str, expected: &[CheckStatus]) {
    let mut checker = BOPAlgoArgumentAnalyzer::new();
    checker.set_shape1(&bound_shape(s, name));
    *checker.argument_type_mode() = true;
    *checker.self_inter_mode() = true;
    *checker.small_edge_mode() = true;
    *checker.rebuild_face_mode() = true;
    *checker.continuity_mode() = true;
    *checker.curve_on_surface_mode() = true;
    checker.perform();
    let found: Vec<CheckStatus> = checker
        .check_result()
        .iter()
        .map(|one| one.check_status())
        .collect();
    assert_eq!(found, expected, "faults of {name}");
}

/// `checksection name` finds exactly this many vertices left alone.
/// Example: `alone_count(s, "result", 4)`.
pub fn alone_count(s: &dyn Session, name: &str, expected: usize) {
    let found = section_endpoints::odd_incidence_vertices(&bound_shape(s, name)).len();
    assert_eq!(found, expected, "vertices left alone in {name}");
}

/// Both names hold the same shape; `same_shape(s, "result", "r_1")`.
pub fn same_shape(s: &dyn Session, name: &str, other: &str) {
    assert!(
        bound_shape(s, name).is_same(&bound_shape(s, other)),
        "{name} and {other}: expected the same shape"
    );
}

/// `name` is of this kind and orientation; `is_oriented(s, "b", ShapeEnum::Wire, Orientation::Reversed)`.
pub fn is_oriented(s: &dyn Session, name: &str, kind: ShapeEnum, orientation: Orientation) {
    let shape = bound_shape(s, name);
    assert_eq!(shape.shape_type().unwrap(), kind, "kind of {name}");
    assert_eq!(shape.orientation(), orientation, "orientation of {name}");
}

/// `name` is of this kind; `is_kind(s, "fx_1", ShapeEnum::Face)`.
pub fn is_kind(s: &dyn Session, name: &str, kind: ShapeEnum) {
    assert_eq!(
        bound_shape(s, name).shape_type().unwrap(),
        kind,
        "kind of {name}"
    );
}

/// The session holds this many arguments in all; `argument_count(&bop, 638)`.
pub fn argument_count(bop: &ScriptBop, expected: usize) {
    assert_eq!(bop.argument_total(), expected, "arguments of the session");
}
