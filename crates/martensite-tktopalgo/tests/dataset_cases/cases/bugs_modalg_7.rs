//! `bugs_modalg_7` case bodies whose
//! inputs are in the public dataset.
use crate::support::{
    checkshape::checkshape,
    commands::{explode, explode_type, pcurve},
    distance::distmini,
    log,
    logged_checks::{checknbshapes_some, checkreal},
    printed::g17,
    props::{Prop, checkprops_equal, props_logged},
    qa::{buc60652, occ32744},
    restore::restore_as,
    session::{Session, ViewStep, checkview_display, set, set_exploded},
    text::lindex,
    topology::{sameparameter_in_place, validrange},
};
use martensite_tkg3d::top_abs::ShapeEnum;

const ONE_VERTEX: [Option<usize>; 9] = [Some(1), Some(0), None, None, None, None, None, None, None];

/// Checkshape.
pub fn bug22288(s: &mut dyn Session) {
    log::todo("Faulty shapes in variables");
    let result = restore_as(s, "bug22288_b1_m.brep", "result").shape();
    checkshape(&result);
}

/// Checkshape, then `pcurve f` (every edge has a p-curve); the view of f.
pub fn bug23667(s: &mut dyn Session) {
    let f = restore_as(s, "bug23667_f_ref.brep", "f").shape();
    checkshape(&f);
    for (i, curve) in pcurve(&f).into_iter().enumerate() {
        set(s, &format!("f_{}", i + 1), curve);
    }
    checkview_display(s, Some("f"), "bug23667");
}

/// The printed area and centre of `sprops` do not depend on the
/// NaturalRestriction flag.
pub fn bug24490(s: &mut dyn Session) {
    let ff = restore_as(s, "bug24490_face_naturalrestriction_flag.brep", "ff").shape();
    let fn_ = restore_as(s, "bug24490_face_naturalrestriction_noflag.brep", "fn").shape();
    let words = |shape| {
        let props = props_logged(Prop::Area, shape, None);
        let c = props.centre;
        let text = format!(
            "Mass : {} Center of gravity : X = {} Y = {} Z = {}",
            props.printed_mass(),
            martensite_tkernel::storage::format_general(c.x(), 6),
            martensite_tkernel::storage::format_general(c.y(), 6),
            martensite_tkernel::storage::format_general(c.z(), 6)
        );
        [2, 9, 12, 15].map(|i| lindex(&text, i).parse::<f64>().unwrap())
    };
    let (a, b) = (words(&ff), words(&fn_));
    let names = [
        "Areas",
        "Center of gravity (X)",
        "Center of gravity (Y)",
        "Center of gravity (Z)",
    ];
    for i in 0..4 {
        if a[i] != b[i] {
            log::puts(&format!(
                "ERROR: OCC24490 is reproduced. {} of identical shapes are different.",
                names[i]
            ));
        }
    }
}

/// Distmini of the two sub-shapes is 0; smallview, fit and the screenshot.
pub fn bug27087(s: &mut dyn Session) {
    s.step(ViewStep::SmallView("AXON"));
    let a = restore_as(s, "bug27087.brep", "a").shape();
    let parts = set_exploded(s, "a", explode(&a));
    if distmini(s, "r", &parts[0], &parts[1], None).unwrap().value > 0.0001 {
        log::puts("Error: too big deviation, should be 0!");
    }
    s.step(ViewStep::Fit);
    s.step(ViewStep::Xwd("bug27087"));
}

/// Checkshape accepts a gap covered by the vertex tolerance.
pub fn bug28499(s: &mut dyn Session) {
    let f = restore_as(s, "bug28499.brep", "f").shape();
    checkshape(&f);
}

/// The distance between the cylinder and the line is one vertex.
pub fn bug29655(s: &mut dyn Session) {
    let shape = restore_as(s, "bug29655.brep", "s").shape();
    let parts = set_exploded(s, "s", explode(&shape));
    match distmini(s, "res", &parts[0], &parts[1], None) {
        Some(d) if !d.solutions.is_empty() => checknbshapes_some(&d.solutions[0], ONE_VERTEX),
        _ => log::puts("Error : result of distmini is wrong"),
    }
}

/// `validrange e` gives the expected range (1e-7).
pub fn bug30090(s: &mut dyn Session) {
    let e = restore_as(s, "bug30090_edge.brep", "e").shape();
    match validrange(&e) {
        Some((first, last)) => {
            // The command prints both with `%.17g`; the regexp reads them back.
            let text = format!("{} {}", g17(first), g17(last));
            log::result(&text);
            let first: f64 = lindex(&text, 0).parse().unwrap();
            let last: f64 = lindex(&text, 1).parse().unwrap();
            checkreal("First", first, 0.043210875526017684, 1.0e-7, 0.0);
            checkreal("Last", last, 0.98374162670821774, 1.0e-7, 0.0);
        }
        None => {
            log::result("edge has no valid range");
            log::puts("Error: valid range is not found");
        }
    }
}

/// 0031029: `sameparameter f 1.e-5` in place, then checkshape.
pub fn bug31029(s: &mut dyn Session) {
    let f = restore_as(s, "bug31029.brep", "f").shape();
    sameparameter_in_place(&f, 1.0e-5, false);
    checkshape(&f);
}

/// `BRepTools::OuterWire` (the `BUC60652` command sets `w`) is the face's third wire.
pub fn bug31172(s: &mut dyn Session) {
    let face = restore_as(s, "bug31172.brep", "face").shape();
    let w = set(s, "w", buc60652(&face));
    let wires = set_exploded(s, "face", explode_type(&face, ShapeEnum::Wire));
    checkprops_equal(&w, &wires[2]);
}

/// 0032744: GCPnts_UniformDeflection on the edge ends within the cpulimit.
pub fn bug32744(s: &mut dyn Session) {
    let shape = restore_as(s, "bug32744.brep", "s").shape();
    occ32744(&shape);
}

/// Distmini between the two edges, both ways, gives one vertex solution.
pub fn bug32934(s: &mut dyn Session) {
    let edges = restore_as(s, "bug32934.brep", "edges").shape();
    let parts = set_exploded(s, "edges", explode_type(&edges, ShapeEnum::Edge));
    for (name, a, b) in [("di", &parts[0], &parts[1]), ("dii", &parts[1], &parts[0])] {
        let d = distmini(s, name, a, b, None).unwrap();
        if d.solutions.len() > 1 {
            log::puts("Error : result of distmini is wrong");
        } else {
            checknbshapes_some(&d.solutions[0], ONE_VERTEX);
        }
    }
}
