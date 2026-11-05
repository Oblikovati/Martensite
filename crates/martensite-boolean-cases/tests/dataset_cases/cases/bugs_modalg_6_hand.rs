//! `bugs_modalg_6` bodies written by hand: their scripts keep surfaces in variables.
use crate::support::{
    beyond_script::all_counts,
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::screenshot,
    commands::explode,
    construction::{NO_VALID_RANGE, valid_range_answer},
    nbshapes::{Counting, NbCount, checknbshapes},
    one_step::bsection,
    properties::{Prop, props},
    restore::restore,
    script_error::script_error,
    session::{Session, ViewStep},
    surface_commands::{cone, mkface, plane, trimv},
    tolerance_check::{checkmaxtol_ref, tolerance_report},
    two_step::TwoStepBoolean,
};
use martensite_tkernel::storage::format_general;

const TRIM: f64 = 42.4264068711929;

fn banner() {
    println!("========");
    println!("OCC28626");
    println!("========");
}

fn conical_faces(s: &mut dyn Session, second_x: f64) {
    let c1 = cone([0., 0., 0.], [0., -1., 0.], 45., 0.);
    let c2 = cone([second_x, -20., 10.], [1., 0., 0.], 45., 0.);
    mkface(s, "f1", &trimv(&c1, 0., TRIM));
    mkface(s, "f2", &trimv(&c2, 0., TRIM));
}

fn section_of_faces(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    two_step.bop(s, "f1", "f2");
    two_step.section(s, "result");
    checkshape_named(s, "result");
}

fn view_of_faces(s: &mut dyn Session) {
    s.view(ViewStep::SmallView);
    s.view(ViewStep::Donly(&["f1", "f2", "result"]));
    s.view(ViewStep::Fit);
    screenshot(s, "photo");
}

pub fn bug28626_2(s: &mut dyn Session) {
    banner();
    conical_faces(s, -23.);
    section_of_faces(s);
    let counts = [NbCount::Edges(3), NbCount::Vertices(4)];
    checknbshapes(s, "result", Counting::Located, &counts);
    checkmaxtol_ref(s, "result", "5.21731e-007");
    checkprops(s, "result", Prop::Length, Expected::Mass(88.9692));
    view_of_faces(s);
    // Beyond the script: every count, not only the two it names.
    all_counts(s, "result", [4, 3, 0, 0, 0, 0, 0, 1, 8]);
}

pub fn bug28626_3(s: &mut dyn Session) {
    banner();
    conical_faces(s, -22.);
    s.view(ViewStep::Donly(&["f1", "f2"]));
    s.view(ViewStep::Fit);
    section_of_faces(s);
    let counts = [NbCount::Edges(4), NbCount::Vertices(5)];
    checknbshapes(s, "result", Counting::Located, &counts);
    checkmaxtol_ref(s, "result", "6.02982e-007");
    checkprops(s, "result", Prop::Length, Expected::Mass(94.3164));
    view_of_faces(s);
    // Beyond the script: every count, not only the two it names.
    all_counts(s, "result", [5, 4, 0, 0, 0, 0, 0, 1, 10]);
}

/// The body of the script's loop over one edge: the line it writes for one without a range.
pub fn too_small_diagnostic(s: &dyn Session, edge: &str) -> Option<String> {
    let shape = s
        .shape(edge)
        .unwrap_or_else(|| panic!("{edge}: expected a bound shape"));
    let answer =
        valid_range_answer(shape).unwrap_or_else(|failure| script_error(&format!("{failure:?}")));
    println!("{answer}");
    if answer != NO_VALID_RANGE {
        return None;
    }
    let length = format_general(props(Prop::Length, shape, None), 6);
    let tolerance = tolerance_report(s, edge);
    Some(format!(
        "Error: edge {edge} (length={length}, tolerance={tolerance}) is too small"
    ))
}

pub fn bug28690_2(s: &mut dyn Session) {
    println!("========");
    println!("OCC28690");
    println!("========");
    restore(s, "bug28690_m9_faces.brep", "a");
    mkface(s, "f", &plane([0., 0., 0.], [0., 1., 0.]));
    bsection(s, "r", "a", "f");
    let counts = [NbCount::Edges(3), NbCount::Vertices(4)];
    checknbshapes(s, "r", Counting::Located, &counts);
    checkshape_named(s, "r");
    checkprops(s, "r", Prop::Length, Expected::Mass(7976.14));
    for edge in explode(s, "r", None) {
        if let Some(line) = too_small_diagnostic(s, &edge) {
            println!("{line}");
        }
    }
}
