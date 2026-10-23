//! bugs xde regression cases whose inputs
//! are in the public dataset or built by the script. Each body follows its script line by
//! line; the bugs group end only prints `TEST COMPLETED`.
use super::bugs_common::banner;
use crate::support::{
    build::maxtolerance_printed,
    checks::nbshapes,
    checkshape::{CheckOptions, checkshape},
    checkshape_structural::checkshape_structural,
    log,
    logged_checks::checknbshapes_some,
    props::{Prop, checkprops},
    restore::restore_as,
    session::{Session, checkview_display, set},
    sewing::{checkfreebounds, checkmaxtol, sewing},
    swdraw::{dt_shape_convert_rev, fixshape, fixsmall, fixwgaps, statshape},
    text::{lindex, numbers_after},
    topology::{checksection, dump_shape},
};
use martensite_tkbrep::topods::Shape;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

/// `restore file name`, `fixshape result name args...`, `checkshape result`,
/// `checkprops result -s area` and `checkview -display result -2d`.
fn fixed_area(
    s: &mut dyn Session,
    title: &[&str],
    input: (&str, &str),
    args: &[&str],
    area: f64,
    image: &str,
) {
    banner(title);
    let a = restore_as(s, input.0, input.1).shape();
    let result = set(s, "result", fixshape(&a, args));
    checkshape(&result);
    checkprops(&result, Prop::Area, area);
    checkview_display(s, Some("result"), image);
}

const OCC125: [&str; 4] = ["================", "OCC125", "================", ""];

pub fn bug125_1(s: &mut dyn Session) {
    fixed_area(
        s,
        &OCC125,
        ("OCC125-1.brep", "shell1"),
        &["1e-7"],
        282.743,
        "bug125_1",
    );
}

pub fn bug125_2(s: &mut dyn Session) {
    fixed_area(
        s,
        &OCC125,
        ("OCC125-2.brep", "shell2"),
        &["1e-7"],
        282.743,
        "bug125_2",
    );
}

pub fn bug168(s: &mut dyn Session) {
    let title = [
        "========",
        "OCC168",
        "OCC279",
        "========",
        "Fix missing seam don't add seam ",
        "",
    ];
    fixed_area(
        s,
        &title,
        ("OCC168.brep", "a"),
        &["1e-7"],
        992.596,
        "bug168",
    );
}

pub fn bug34(s: &mut dyn Session) {
    let title = ["================", "OCC34", "================", ""];
    fixed_area(
        s,
        &title,
        ("OCC34.brep", "sh"),
        &["1.e-7", "1"],
        116.997,
        "bug34",
    );
}

/// OCC1430: ShapeFixWireframe processes separate wires (`fixsmall` or `fixwgaps`).
fn bug1430(s: &mut dyn Session, file: &str, gaps: bool, image: &str) {
    banner(&[
        "================",
        "OCC1430",
        "ShapeFix_Wireframe does not process separate wires",
        "================",
        "",
    ]);
    let a = restore_as(s, file, "a").shape();
    checkshape(&a);
    let fixed = if gaps {
        fixwgaps(&a, Some(0.1))
    } else {
        fixsmall(&a, Some(10.))
    };
    log::result(&fixed.text);
    let Some(result) = fixed.shape.map(|shape| set(s, "result", shape)) else {
        log::puts("OCC1430: Error");
        return;
    };
    checkshape(&result);
    checkview_display(s, Some("result"), image);
}

pub fn bug1430_1(s: &mut dyn Session) {
    bug1430(s, "OCC1430-1.brep", false, "bug1430_1");
}

pub fn bug1430_2(s: &mut dyn Session) {
    bug1430(s, "OCC1430-2.brep", true, "bug1430_2");
}

/// OCC2: the sixth word of `statshape` is `EDGE`.
pub fn bug2(s: &mut dyn Session) {
    banner(&["================", "OCC2", "================", ""]);
    let result = restore_as(s, "OCC46.brep", "result").shape();
    checkshape(&result);
    let table = statshape(&result);
    log::result(&table);
    if lindex(&table, 5) != "EDGE" {
        log::puts("Error : Output of statshape is incorrect.");
    }
    checkview_display(s, Some("result"), "bug2");
}

/// OCC22576: the healed face keeps its length and section.
pub fn bug22576(s: &mut dyn Session) {
    banner(&[
        "============",
        "OCC22576",
        "============",
        "",
        "Load shape ...",
    ]);
    let res = restore_as(s, "test-fix280-face.brep", "res").shape();
    let result = set(s, "result", fixshape(&res, &[]));
    checkprops(&result, Prop::Length, 1.56058);
    log::result(&format!(" nb alone Vertices : {}\n", checksection(&result)));
    checkshape(&result);
    checkview_display(s, Some("result"), "bug22576");
}

/// The WIRE count of `nbshapes shape` (`regexp {WIRE +: +([-0-9.+eE]+)}`).
fn wire_count(shape: &Shape) -> f64 {
    let text = nbshapes(shape);
    log::result(&text);
    numbers_after(&text, "WIRE", 1, ":").map_or_else(
        || {
            let at = text.find("WIRE").expect("nbshapes prints WIRE");
            text[at + 4..]
                .trim_start()
                .trim_start_matches(':')
                .split_whitespace()
                .next()
                .unwrap()
                .parse()
                .unwrap()
        },
        |values| values[0],
    )
}

/// OCC4968: fixshape adds the missing wires.
pub fn bug4968(s: &mut dyn Session) {
    banner(&["=========", " OCC4968 ", "=========", ""]);
    let r = restore_as(s, "OCC4968.brep", "r").shape();
    let wire_number1 = wire_count(&r);
    let result = set(s, "result", fixshape(&r, &[]));
    let wire_number2 = wire_count(&result);
    checkshape_structural(&result, "result", "r", CheckOptions::default());
    log::puts(&format!("wire_number1 = {wire_number1}"));
    log::puts(&format!("wire_number2 = {wire_number2}"));
    if wire_number2 > wire_number1 {
        log::puts("OK OCC4968: wire number - OK");
    } else {
        log::puts("Faulty OCC4968: wrong wire number");
    }
    checkview_display(s, Some("result"), "bug4968");
}

/// OCC6555: fixshape of a valid box keeps its solid (the TShape address of the dump).
pub fn bug6555(s: &mut dyn Session) {
    banner(&["========", "OCC6555", "========", ""]);
    let mut maker = BRepPrimAPIMakeBox::from_sizes(10., 10., 10.).unwrap();
    let s1 = set(s, "s1", maker.shape().unwrap().clone());
    let report = checkshape(&s1);
    if report.text.split_whitespace().any(|word| word == "Faulty") {
        log::puts("Faulty OCC6555 : checkshape is wrong for s1");
    } else {
        log::puts("checkshape is good for s1");
    }
    let dump1 = dump_shape("s1", &s1);
    log::result(&dump1);
    let result = set(s, "result", fixshape(&s1, &[]));
    let res = checkshape(&result);
    if res.text.split_whitespace().any(|word| word == "Faulty") {
        log::puts("Faulty OCC6555 : checkshape is wrong for res");
    } else {
        log::puts("checkshape is good for res");
    }
    let dump2 = dump_shape("result", &result);
    log::result(&dump2);
    if lindex(&dump1, 29) != lindex(&dump2, 29) {
        log::puts("Faulty OCC6555");
    } else {
        log::puts("OK OCC6555");
    }
    checkview_display(s, Some("result"), "bug6555");
}

/// OCC6675: sewing at 1e-7.
pub fn bug6675(s: &mut dyn Session) {
    banner(&["========", "OCC6675", "========", ""]);
    let a = restore_as(s, "OCC6675.brep", "a").shape();
    let result = sewing(1.00000e-07, &[&a])
        .map(|shape| set(s, "result", shape))
        .expect("checkshape: result is not a shape (sewing built nothing)");
    checkshape(&result);
    checkmaxtol(&result, "result", 1.0021647610996499e-005);
    let mut shells = [None; 9];
    shells[4] = Some(1);
    checknbshapes_some(&result, shells);
    checkfreebounds(&result, "result", 0);
    checkview_display(s, Some("result"), "bug6675");
}

/// OCC810: fixshape must not raise.
pub fn bug810(s: &mut dyn Session) {
    banner(&[
        "========================",
        " OCC810 ",
        "========================",
        "",
    ]);
    let a = restore_as(s, "OCC810.rle", "a").shape();
    set(s, "result", fixshape(&a, &["1.e-7", "0.1"]));
    checkview_display(s, Some("result"), "bug810");
}

/// OCC945: fixshape at precision 29.9 must not raise.
pub fn bug945(s: &mut dyn Session) {
    banner(&["=========", " OCC945 ", "=========", ""]);
    let a = restore_as(s, "OCC945.brep", "a").shape();
    let result = set(s, "result", fixshape(&a, &["29.9"]));
    checkshape(&result);
    checkview_display(s, Some("result"), "bug945");
}

/// OCC966 (`TODO OCC12345 ALL: Faulty OCC966 : Tolerance of shape is more than 0.000176`).
pub fn bug966(s: &mut dyn Session) {
    log::todo("Faulty OCC966 : Tolerance of shape is more than 0.000176");
    banner(&["=========", " OCC966 ", "=========", ""]);
    let a = restore_as(s, "OCC966.brep", "a").shape();
    let converted = dt_shape_convert_rev(&a, false, true);
    let result = set(s, "result", converted.shape);
    log::result(&converted.text);
    let [face, edge, vertex] = maxtolerance_printed("result", &result);
    if face > 0.000176 || edge > 0.000176 || vertex > 0.000176 {
        log::puts("Faulty OCC966 : Tolerance of shape is more than 0.000176");
    }
    checkview_display(s, Some("result"), "bug966");
}
