//! The scripts every boolean case inherits: the group `begin` and `end`, and the grid `end`s.
use super::{
    checkshape::checkshape,
    checkview::{CheckView, checkview},
    commands::explode,
    properties::Prop,
    rules::{BOOLEAN_RULES, Rule},
    session::Session,
};
use crate::case_bopcheck::{complete_response, configured_checker};
use martensite_tkg3d::top_abs::ShapeEnum;

/// The group `begin`'s `dset SCALE 100`. `protect` only guards the variable against being
/// rebound to another drawable: a case's own `dset SCALE x` changes the value and writes
/// nothing else, so a body just uses its own local value after that line.
pub const SCALE: f64 = 100.;

/// What every case of one group inherits.
pub struct CaseGroup {
    /// The group's CPU and elapsed budget, in seconds.
    pub cpu_seconds: i64,
    /// The group's parse rules, before the common ones.
    pub rules: &'static [Rule],
    /// The group's `begin` script.
    pub begin: fn(&mut dyn Session),
    /// The group's `end` script.
    pub end: fn(&mut dyn Session),
}

/// `tests/boolean`: `cpulimit 300`, its `begin` and `end`.
pub const BOOLEAN: CaseGroup = CaseGroup {
    cpu_seconds: 300,
    rules: BOOLEAN_RULES,
    begin: boolean_begin,
    end: boolean_end,
};

/// The group `begin`: what the budget command writes. The budget itself is in force already:
/// the child process installs it before the case starts, and checks that here.
/// Example: `boolean_begin(session)`.
pub fn boolean_begin(_session: &mut dyn Session) {
    let budget = martensite_process_policy::reported_process_limit();
    assert_eq!(budget, BOOLEAN.cpu_seconds, "group begin: CPU budget");
    println!("CPU and elapsed time limit set to {budget} seconds");
}

/// The group `end`: `checkshape result` when bound, and the one completion line.
/// Example: `boolean_end(session)`.
pub fn boolean_end(session: &mut dyn Session) {
    if let Some(result) = session.shape("result") {
        checkshape(result);
    }
    println!("TEST COMPLETED");
}

/// The grid `end` of the grids that have none; `no_grid_end(session)`.
pub fn no_grid_end(_session: &mut dyn Session) {}

/// The grid `end` of `cells_test`, `gdml_private` and `gdml_public`: `checkview -display result -2d`.
/// Example: `result_view_end(session)`.
pub fn result_view_end(session: &mut dyn Session) {
    checkview(session, &CheckView::of("result"));
}

/// The grid `end` of the `*_2d` grids: the result with its vertices, otherwise `a` and `b`.
/// Example: `planar_end(session)`.
pub fn planar_end(session: &mut dyn Session) {
    let names = explode_vertices(session, "result");
    let names: Vec<&str> = names.iter().map(String::as_str).collect();
    let view = CheckView::of("result")
        .otherwise(&["a", "b"], Prop::Length)
        .with(&names);
    checkview(session, &view);
}

/// The grid `end` of `volumemaker`: `bopcheck result`, then the view of the result.
/// Example: `volumemaker_end(session)`.
pub fn volumemaker_end(session: &mut dyn Session) {
    let response = session.shape("result").map_or_else(
        || "null shapes are not allowed here!".to_owned(),
        |result| {
            let mut checker = configured_checker(result);
            checker.perform();
            complete_response(&checker)
        },
    );
    // Twice, as a reference log shows: the result of `set bcheck [bopcheck result]`, then `puts`.
    println!("{response}");
    println!("{response}");
    if !response.contains("This shape seems to be OK.") {
        println!("Error: bopcheck failed");
    }
    result_view_end(session);
}

/// `explode name v`: the [`explode`] of the vertices; `explode_vertices(session, "result")`.
pub fn explode_vertices(session: &mut dyn Session, name: &str) -> Vec<String> {
    explode(session, name, Some(ShapeEnum::Vertex))
}

/// `if {![regexp {This shape seems to be OK.} [bopcheck name]]} {puts message}`.
/// Example: `require_bopcheck_ok(s, "result", "Error: result shape is self intersected")`.
pub fn require_bopcheck_ok(session: &dyn Session, name: &str, message: &str) {
    let shape = session
        .shape(name)
        .unwrap_or_else(|| panic!("bopcheck {name}: expected a bound shape"));
    let mut checker = configured_checker(shape);
    checker.perform();
    let response = complete_response(&checker);
    println!("{response}");
    if !response.contains("This shape seems to be OK.") {
        println!("{message}");
    }
}

/// `if {![regexp word [whatis name]]} {puts message}`: `word` a shape type like `SHELL`.
/// Example: `require_whatis(s, "result_1", "SHELL", "Error: Shell has been destroyed")`.
pub fn require_whatis(session: &dyn Session, name: &str, word: &str, message: &str) {
    let kind = session
        .shape(name)
        .map(|shape| format!("{:?}", shape.shape_type().unwrap()).to_uppercase());
    if kind.as_deref() != Some(word) {
        println!("{message}");
    }
}
