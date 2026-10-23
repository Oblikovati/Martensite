//! perf modalg regression cases owned by
//! TKShHealing whose inputs are in the public dataset: the timed command runs between
//! `dchrono h restart` and `dchrono h stop counter text`, which prints its CPU time.
use crate::support::{
    build::checknbshapes_t,
    checkshape::checkshape,
    chrono::Dchrono,
    commands::{explode, mkcurve, mksurface},
    log,
    props::Prop,
    props_full::{checkprops_equal_skip, checkprops_skip},
    qa_projection::occ24008,
    restore::{Restored, restore_as},
    session::{Session, set, set_exploded},
    swdraw::unifysamedom,
};
use martensite_tkbrep::topods::Shape;

/// tests/perf/modalg/bug27569.
/// Projecting the edge curve on the face surface (`OCC24008`) is timed.
pub fn bug27569(s: &mut dyn Session) {
    for line in ["============", "OCC27569", "============", ""] {
        log::puts(line);
    }
    let a_s = restore_as(s, "bug27569.brep", "aS").shape();
    let parts = set_exploded(s, "aS", explode(&a_s));
    let c = mkcurve(&parts[0]);
    s.set("c", &Restored::Curve(c.clone()));
    let surface = mksurface(&parts[1]);
    s.set("s", &Restored::Surface(surface.clone()));
    let chrono = Dchrono::restart();
    log::result(&occ24008(&c, &surface));
    chrono.stop_counter("OCC24008");
}

/// OCC29502: `unifysamedom result cf [-f] +b` is timed, then checkshape result; returns
/// `cf` and `result`.
fn bug29502(s: &mut dyn Session, file: &str, options: &[&str]) -> (Shape, Shape) {
    for line in ["============", "OCC29502", "============", ""] {
        log::puts(line);
    }
    let cf = restore_as(s, file, "cf").shape();
    let chrono = Dchrono::restart();
    let result = set(s, "result", unifysamedom(&cf, &[], options));
    chrono.stop_counter("unifysamedom");
    checkshape(&result);
    (cf, result)
}

/// tests/perf/modalg/bug29502_1.
/// The unified faces keep the length of the input.
pub fn bug29502_1(s: &mut dyn Session) {
    let (cf, result) = bug29502(s, "bug29502_faces.brep", &["+b"]);
    checkprops_equal_skip(&result, "result", &cf, "cf");
    let counts = [("VERTEX", 146), ("EDGE", 217), ("WIRE", 72), ("FACE", 72)];
    checknbshapes_t(&result, "result", &counts, None, "");
}

/// tests/perf/modalg/bug29502_2.
/// The unified edges (`-f`: no face unification) keep the length of the input.
pub fn bug29502_2(s: &mut dyn Session) {
    let (cf, result) = bug29502(s, "bug29502_split_faces.brep", &["-f", "+b"]);
    checkprops_equal_skip(&result, "result", &cf, "cf");
    let counts = [("VERTEX", 365), ("EDGE", 652), ("WIRE", 288), ("FACE", 288)];
    checknbshapes_t(&result, "result", &counts, None, "");
}

/// tests/perf/modalg/bug29502_3.
/// The unified split faces have the length 2900 and the area 14400.
pub fn bug29502_3(s: &mut dyn Session) {
    let (_, result) = bug29502(s, "bug29502_split_faces.brep", &["+b"]);
    checkprops_skip(&result, &[(Prop::Length, 2900.), (Prop::Area, 14400.)]);
    let counts = [("VERTEX", 146), ("EDGE", 217), ("WIRE", 72), ("FACE", 72)];
    checknbshapes_t(&result, "result", &counts, None, "");
}
