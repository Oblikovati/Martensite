//! `geometry_project::A1`..A8 bodies:
//! restore a curve and a surface, `project result curv1 surf1 -v 50` and `checklength
//! result -l L`. `-v 50` only prints the 3D distance at 50 parameters; it asserts nothing.
use crate::support::{
    checks::checklength2d,
    commands::project,
    restore::restore_as,
    session::{Session, set},
};
use martensite_tkernel::precision;

fn project_case(s: &mut dyn Session, curve: &str, surface: &str, expected: f64) {
    let curv1 = restore_as(s, curve, "curv1").curve();
    let surf1 = restore_as(s, surface, "surf1").surface();
    let result = project(&curv1, &surf1, precision::confusion()).expect("Projection Failed");
    checklength2d(&set(s, "result", result), expected);
}

const PROJ4_SURFACE: &str = "pro5075_bug-proj4-surface.rle";
const PROJ4_CURVE1: &str = "pro5075_bug-proj4-curve1.rle";
const PROJ4_CURVE2: &str = "pro5075_bug-proj4-curve2.rle";
const PROJ5: (&str, &str) = (
    "pro5102_bug-proj5-curve.rle",
    "pro5102_bug-proj5-surface.rle",
);
const PROJ6: (&str, &str) = (
    "pro5103_bug-proj6-curve.rle",
    "pro5103_bug-proj6-surface.rle",
);

pub fn a1(s: &mut dyn Session) {
    project_case(s, PROJ4_CURVE1, PROJ4_SURFACE, 0.062450996195736283);
}

pub fn a2(s: &mut dyn Session) {
    project_case(s, PROJ4_CURVE2, PROJ4_SURFACE, 1.2463576545507855);
}

pub fn a3(s: &mut dyn Session) {
    project_case(s, PROJ5.0, PROJ5.1, 3.1415926535897993);
}

pub fn a4(s: &mut dyn Session) {
    project_case(s, PROJ6.0, PROJ6.1, 0.0074999999675910405);
}

// A5..A8 repeat A1..A4 line for line in the original grid.
pub fn a5(s: &mut dyn Session) {
    project_case(s, PROJ4_CURVE1, PROJ4_SURFACE, 0.062450996195736283);
}

pub fn a6(s: &mut dyn Session) {
    project_case(s, PROJ4_CURVE2, PROJ4_SURFACE, 1.2463576545507855);
}

pub fn a7(s: &mut dyn Session) {
    project_case(s, PROJ5.0, PROJ5.1, 3.1415926535897993);
}

pub fn a8(s: &mut dyn Session) {
    project_case(s, PROJ6.0, PROJ6.1, 0.0074999999675910405);
}
