//! `bugs_modalg_2` case bodies whose
//! inputs are in the public dataset. Every command result is logged, as `dlog on`
//! does, and the case's log gets the parse-rule verdict.
use crate::support::{
    checks::checknbshapes,
    checkshape::{CheckOptions, checkshape},
    checkshape_structural::checkshape_structural,
    commands::explode_type,
    distance::bclassify_default,
    log,
    props::{Prop, checkprops},
    restore::restore_as,
    session::{Session, checkview_display, set, set_exploded},
    text::numbers_after,
    topology::{checksection, compound, dump_shape, shape_type_word, tcopy, vertex, wexplo},
};
use martensite_tkg3d::top_abs::{ShapeEnum, State};
use martensite_tkmath::gp::Point3;

/// `checkshape result r` (structural): its text names `OK` when the shape looks OK.
fn structural_ok(shape: &martensite_tkbrep::topods::Shape, name: &str) -> bool {
    checkshape_structural(shape, name, "r", CheckOptions::default()).contains("OK")
}

/// The shell faces in a compound; nbshapes, area and checkshape.
pub fn bug20465(s: &mut dyn Session) {
    let r = restore_as(s, "OCC20465.brep", "r").shape();
    checkshape(&r);
    let faces = set_exploded(s, "r", explode_type(&r, ShapeEnum::Face));
    let parts: Vec<_> = faces.iter().take(11).collect();
    let result = set(s, "result", compound(&parts));
    checknbshapes(&result, [23, 33, 13, 11, 0, 0, 0, 1, 81]);
    checkprops(&result, Prop::Area, 900.0);
    checkshape(&result);
    checkview_display(s, Some("result"), "bug20465");
}

/// The face must look OK in both checkshape styles.
pub fn bug221(s: &mut dyn Session) {
    let result = restore_as(s, "OCC221.brep", "result").shape();
    if !structural_ok(&result, "result") {
        log::puts("Faulty OCC221 (checkshape) : Checkshape command works wrongly");
    }
    if checkshape(&result).has_faulty() {
        log::puts("Faulty OCC221 (checkshape) : Checkshape command works wrongly");
    }
    checkprops(&result, Prop::Area, 425.537);
    checkview_display(s, Some("result"), "bug221");
}

/// `bug23`, with `TODO ... Faulty shapes in variables faulty_1 to faulty` and `TODO ...
/// Error : The length of result shape is`.
pub fn bug23(s: &mut dyn Session) {
    log::todo("Faulty shapes in variables faulty_1 to faulty");
    log::todo("Error : The length of result shape is");
    let sh = restore_as(s, "OCC23.brep", "sh").shape();
    checkshape(&sh);
    let faces = set_exploded(s, "sh", explode_type(&sh, ShapeEnum::Face));
    let wires = set_exploded(s, "sh_2", explode_type(&faces[1], ShapeEnum::Wire));
    let result = set(s, "result", wires[0].clone());
    checkprops(&result, Prop::Length, 0.0);
    checkshape(&result);
    checksection(&result);
    checkview_display(s, Some("result"), "bug23");
}

/// `tcopy` of the face must not raise.
pub fn bug298(s: &mut dyn Session) {
    let result = restore_as(s, "OCC298.brep", "result").shape();
    checkshape(&result);
    let copy = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| tcopy(&result)));
    match copy {
        Ok(a) => {
            set(s, "a", a);
            log::puts("OCC298 OK: function TCOPY works properly");
        }
        Err(_) => log::puts("Faulty OCC298: function TCOPY works wrongly"),
    }
    checkprops(&result, Prop::Area, 8e+100);
    checkshape(&result);
    checkview_display(s, Some("result"), "bug298");
}

/// The structural checkshape of the face looks OK, and its area.
fn bug302(s: &mut dyn Session, file: &str, case: u32, area: f64, image: &str) {
    let result = restore_as(s, file, "result").shape();
    if !structural_ok(&result, "result") {
        log::puts(&format!(
            "Faulty OCC302 (case {case}): command  checkbrep works wrongly"
        ));
    }
    checkprops(&result, Prop::Area, area);
    checkview_display(s, Some("result"), image);
}

pub fn bug302_1(s: &mut dyn Session) {
    bug302(s, "OCC302a.brep", 1, 271.611, "bug302_1");
}

pub fn bug302_2(s: &mut dyn Session) {
    bug302(s, "OCC302b.brep", 2, 1.93998, "bug302_2");
}

pub fn bug302_3(s: &mut dyn Session) {
    bug302(s, "OCC302c.brep", 3, 23240.8, "bug302_3");
}

/// The solid and its fifth wire are valid.
pub fn bug317(s: &mut dyn Session) {
    let result = restore_as(s, "OCC317.brep", "result").shape();
    if checkshape(&result).has_faulty() {
        log::puts("Faulty OCC317: Source shape is invalid. It was detected by Checkshape command");
    } else {
        let wires = set_exploded(s, "result", explode_type(&result, ShapeEnum::Wire));
        if checkshape(&wires[4]).has_faulty() {
            log::puts("Faulty OCC317: Wire is invalid. It was detected by Checkshape command");
        }
        if !structural_ok(&result, "result") {
            log::puts("Faulty OCC317: Wire is invalid. It was detected by Checkshape command");
        }
    }
    checkprops(&result, Prop::Area, 2104.21);
    checkview_display(s, Some("result"), "bug317");
}

/// The structural checkshape looks OK, and the area.
fn bug327(s: &mut dyn Session, file: &str, case: u32, area: f64, image: &str) {
    let result = restore_as(s, file, "result").shape();
    if structural_ok(&result, "result") {
        log::puts(&format!(" OCC327  case {case}: OK"));
    } else {
        log::puts(&format!(" OCC327  case {case}: Faulty"));
    }
    checkprops(&result, Prop::Area, area);
    checkview_display(s, Some("result"), image);
}

pub fn bug327_1(s: &mut dyn Session) {
    bug327(s, "OCC327a.brep", 1, 1477.93, "bug327_1");
}

pub fn bug327_2(s: &mut dyn Session) {
    bug327(s, "OCC327b.brep", 2, 5674.84, "bug327_2");
}

pub fn bug327_3(s: &mut dyn Session) {
    bug327(s, "OCC327c.brep", 3, 61013.8, "bug327_3");
}

pub fn bug327_4(s: &mut dyn Session) {
    bug327(s, "OCC327d.brep", 4, 7793.26, "bug327_4");
}

/// A face checkshape must call faulty; the TODOs keep the known failure (checkshape
/// finds it valid) and the wrong zero area.
fn known_valid_face(s: &mut dyn Session, file: &str, faulty: &str, image: &str) {
    log::todo(faulty);
    log::todo("Error : The area of result shape is");
    let result = restore_as(s, file, "result").shape();
    if !checkshape(&result).has_faulty() {
        log::puts(faulty);
    }
    checkprops(&result, Prop::Area, 0.0);
    checkview_display(s, Some("result"), image);
}

/// Checkshape accepts a wrong tolerance order (TODO).
pub fn bug330(s: &mut dyn Session) {
    known_valid_face(
        s,
        "OCC330.brep",
        "Faulty OCC330: Checkshape command works wrongly",
        "bug330",
    );
}

/// Checkshape says the face is OK (TODO).
pub fn bug406(s: &mut dyn Session) {
    let faulty = "Faulty OCC406: Command checkshape works wrongly: Source shape looks invalid";
    known_valid_face(s, "OCC406.brep", faulty, "bug406");
}

/// `wexplo` of the face's wire gives its six edges.
pub fn bug356(s: &mut dyn Session) {
    let result = restore_as(s, "OCC356.brep", "result").shape();
    checkshape(&result);
    let wires = set_exploded(s, "result", explode_type(&result, ShapeEnum::Wire));
    let edges = wexplo(&wires[0], Some(&result));
    for (j, edge) in edges.iter().enumerate() {
        set(s, &format!("WEDGE_{}", j + 1), edge.clone());
    }
    // `repeat 10`: count the WEDGE_j that are edges, stopping at the first that is not.
    let nom = edges
        .iter()
        .take(10)
        .take_while(|edge| shape_type_word(edge) == "EDGE")
        .count();
    if nom != 6 {
        log::puts("Faulty OCC356 : Command WEXPLO works WRONGLY");
    }
    checkprops(&result, Prop::Area, 6207.83);
    checkview_display(s, Some("result"), "bug356");
}

/// Checkshape accepts the valid face.
pub fn bug399(s: &mut dyn Session) {
    let result = restore_as(s, "OCC399.brep", "result").shape();
    if checkshape(&result).has_faulty() {
        log::puts(
            "Faulty OCC399: Command checkshape works wrongly: Source shape is invalid but this shape must be OK",
        );
    }
    checkprops(&result, Prop::Area, 4558.53);
    checkview_display(s, Some("result"), "bug399");
}

/// The structural checkshape looks OK.
pub fn bug437(s: &mut dyn Session) {
    let result = restore_as(s, "OCC437.brep", "result").shape();
    if !structural_ok(&result, "result") {
        log::puts(" Faulty OCC437: function CHECKSHAPE works wrongly");
    }
    checkprops(&result, Prop::Area, 0.000201234);
    checkview_display(s, Some("result"), "bug437");
}

/// `bug446` and `bug447`: checkshape of a closed shell.
fn bug446(s: &mut dyn Session, file: &str, case: u32, area: f64, image: &str) {
    let result = restore_as(s, file, "result").shape();
    if checkshape(&result).has_faulty() {
        log::puts(&format!(
            "Faulty OCC446 (case {case}): Source shape is NotClosed"
        ));
    }
    checkprops(&result, Prop::Area, area);
    checkview_display(s, Some("result"), image);
}

/// Case 1 keeps its TODOs: checkshape names faulty_1 to faulty_2.
pub fn bug446_1(s: &mut dyn Session) {
    log::todo("Faulty shapes in variables faulty_1 to faulty_2");
    log::todo("Faulty OCC446");
    bug446(s, "OCC446a.brep", 1, 108.845, "bug446_1");
}

pub fn bug446_2(s: &mut dyn Session) {
    bug446(s, "OCC446b.brep", 2, 24272.8, "bug446_2");
}

pub fn bug446_3(s: &mut dyn Session) {
    bug446(s, "OCC446c.brep", 3, 13694.6, "bug446_3");
}

pub fn bug446_4(s: &mut dyn Session) {
    bug446(s, "OCC446d.brep", 4, 629.347, "bug446_4");
}

/// Restoring the 3D curve must not reset the p-curve range: the dumped ranges of
/// p-curve 1 and 3D curve 1 differ.
pub fn bug465(s: &mut dyn Session) {
    let result = restore_as(s, "OCC465.brep", "result").shape();
    let dum = dump_shape("result", &result);
    log::result(&dum);
    let pcurve = numbers_after(&dum, "PCurve : 1 on surface 1, range : ", 2, "").unwrap();
    let curve = numbers_after(&dum, "Curve 3D : 1, range : ", 2, "").unwrap();
    if pcurve == curve {
        log::puts(
            "Faulty OCC465: it is reset the PCurve range by range of Curve3d after restoring Curve3d",
        );
    }
    checkprops(&result, Prop::Length, 4.21716);
    checkshape(&result);
    checksection(&result);
    checkview_display(s, Some("result"), "bug465");
}

/// The point is not OUT of the solid.
pub fn bug516(s: &mut dyn Session) {
    let p = Point3::new(14.99999878745701, 5.52319672860377e-07, 7.78393041);
    set(s, "p", p);
    set(s, "v", vertex(p));
    let result = restore_as(s, "OCC516.brep", "result").shape();
    if bclassify_default(&result, p) == State::Out {
        log::puts("Faulty : Wrong classification of the 3D-point with the Solid");
    }
    checkprops(&result, Prop::Area, 7752.98);
    checkshape(&result);
    checkview_display(s, Some("result"), "bug516");
}
