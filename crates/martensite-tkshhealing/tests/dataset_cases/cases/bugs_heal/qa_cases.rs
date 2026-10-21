//! The tests/bugs/heal cases built on QABugs commands and on several fixshape runs.
use super::banner;
use crate::support::{
    build::whatis,
    checkshape::checkshape,
    commands::{explode, explode_type},
    log,
    logged_checks::checkreal,
    props::{Prop, checkprops},
    qa::{occ22, occ24},
    qa_projection::{occ7570, occ24008, occ24370, occ26930},
    restore::restore_as,
    session::{Session, checkview_display, set, set_exploded},
    swdraw::{fixshape, statshape},
    text::lindex,
    topology::compound,
};
use martensite_tkg3d::top_abs::ShapeEnum;

/// The `N\t FACE` count of a statshape table (`regexp {([-0-9.+eE]+)\s FACE}`).
fn statshape_faces(text: &str) -> usize {
    let line = text
        .lines()
        .find(|line| line.contains("\t FACE"))
        .expect("regexp {([-0-9.+eE]+)\\s FACE}: no FACE row");
    line.split('\t').next().unwrap().parse().unwrap()
}

/// OCC22: ShapeBuild_ReShape processes locations (5 faces after the split).
pub fn bug22(s: &mut dyn Session) {
    banner(&["=======", "OCC22", "=======", ""]);
    let shape = restore_as(s, "OCC21.brep", "shape").shape();
    let faces = set_exploded(s, "shape", explode_type(&shape, ShapeEnum::Face));
    let divided = set(s, "compoundToBeDivided", compound(&[&faces[0], &faces[1]]));
    let first = occ22(&shape, &divided, false);
    log::result(&first.text);
    let result = first
        .shape
        .map(|result| set(s, "result", result))
        .expect("statshape: Shape unknown : result");
    log::puts("OCC22 OK; (case 1)");
    let nb_faces = statshape_faces(&statshape(&result));
    if nb_faces != 5 {
        log::puts("OCC22 FAULTY: Initial shape was not split");
    }
    let second = occ22(&shape, &divided, true);
    log::result(&second.text);
    if let Some(res) = second.shape {
        set(s, "res", res);
    }
    log::puts("OCC22 OK; (case 2)");
    checkview_display(s, Some("result"), "bug22");
}

/// OCC22919: fixshape of five faces with a missing seam, each then checked.
pub fn bug22919(s: &mut dyn Session) {
    banner(&["========", "OCC22919", "========", ""]);
    let ss = restore_as(s, "bug22919_ss.brep", "ss").shape();
    let parts = set_exploded(s, "ss", explode(&ss));
    for (name, index) in [("ok1", 0), ("ok3", 2), ("ok5", 4), ("ko2", 1), ("ko4", 3)] {
        let fixed = set(s, name, fixshape(&parts[index], &[]));
        log::result(&whatis(name, &fixed));
        checkshape(&fixed);
    }
}

/// OCC24: ShapeProcess context modifications survive the ReShape round trip.
pub fn bug24(s: &mut dyn Session) {
    banner(&["=======", "OCC24", "=======", ""]);
    let shape = restore_as(s, "OCC24.brep", "s").shape();
    checkshape(&shape);
    let faces = set_exploded(s, "s", explode_type(&shape, ShapeEnum::Face));
    let c = set(s, "c", compound(&[&faces[0]]));
    let output = occ24(&shape, &c, "OCC24", "Seq");
    log::result(&output.text);
    let result = output
        .shape
        .map(|result| set(s, "result", result))
        .expect("checkprops: result is not a shape");
    log::puts("OCC24 OK");
    checkprops(&result, Prop::Area, 37.6991);
    checkshape(&result);
    checkview_display(s, Some("result"), "bug24");
}

/// OCC24008: the projection must not raise (`Exception` in its result is an Error).
fn bug24008(s: &mut dyn Session, curve: &str, surface: &str) {
    banner(&["========", "OCC24008", "========", ""]);
    let a = restore_as(s, curve, "a").curve();
    let b = restore_as(s, surface, "b").surface();
    let info = occ24008(&a, &b);
    log::result(&info);
    if info.contains("Exception") {
        log::puts("Error : Exception was caught");
    } else {
        log::puts("OK: Command wrong property");
    }
}

pub fn bug24008_1(s: &mut dyn Session) {
    bug24008(s, "bug24008_curv_6964.draw", "bug24008_surf_6962.draw");
}

pub fn bug24008_2(s: &mut dyn Session) {
    bug24008(s, "bug24008_curv_rbp.draw", "bug24008_surf_rbls.draw");
}

/// OCC24370: ShapeFix_EdgeProjAux finds the end parameters 0 and 1.
pub fn bug24370(s: &mut dyn Session) {
    banner(&["============", "OCC24370", "============", ""]);
    let edge = restore_as(s, "bug24370_edge.brep", "edge").shape();
    let pcurve = restore_as(s, "bug24370_pcurve_draw", "pcurve").curve2d();
    let surface = restore_as(s, "bug24370_surface_draw", "surface").surface();
    let info = occ24370(&edge, &pcurve, &surface, 0.001);
    log::result(&info);
    let first: f64 = lindex(&info, 2).parse().unwrap();
    let last: f64 = lindex(&info, 3).parse().unwrap();
    checkreal("First", first, 0., 1.0e-03, 0.01);
    checkreal("Last", last, 1., 1.0e-03, 0.01);
}

/// OCC26930: the projection of the curve onto the surface is a line.
fn bug26930(s: &mut dyn Session, surface: &str, curve: &str, end: f64) {
    banner(&["========", "OCC26930", "========", ""]);
    let surface = restore_as(s, surface, "s").surface();
    let curve = restore_as(s, curve, "c").curve();
    log::result(&occ26930(&surface, &curve, 0.0, end));
}

pub fn bug26930_1(s: &mut dyn Session) {
    bug26930(s, "bug26930_surf_1.draw", "bug26930_curv_1.draw", 14.3316);
}

pub fn bug26930_2(s: &mut dyn Session) {
    bug26930(
        s,
        "bug26930_surf_2.draw",
        "bug26930_curv_2.draw",
        12.41152967687705,
    );
}

/// OCC7570: dropping small edges with a huge precision must not crash.
pub fn bug7570(s: &mut dyn Session) {
    banner(&["=======", "OCC7570", "=======", ""]);
    let a = restore_as(s, "bug7570_small.brep", "a").shape();
    occ7570(&a);
}
