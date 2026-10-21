//! `bugs_moddata_2` cases built on the diagnostic commands.
use crate::support::{
    checkshape::checkshape,
    distance::distmini,
    geometry::{cvalue, plane},
    log,
    props::{Prop, checkprops},
    qa::{occ299, occ367, occ8169, pipe_occ9},
    restore::restore_as,
    session::{Session, checkview_display, set},
    text::llength,
    topology::{checksection, mkedge, mkface, vertex},
};
use martensite_tkmath::gp::Point3;

/// The point is not classified IN.
pub fn bug299(s: &mut dyn Session) {
    log::todo("OCC299: Error 2");
    let pr = restore_as(s, "OCC298.brep", "pr").shape();
    checkshape(&pr);
    let p = set(s, "p", Point3::new(0.86427836, -1., -1.));
    let info_result = occ299(&pr, p);
    if llength(&info_result) < 4 {
        log::puts("OCC299: Error 1");
    } else if info_result.contains("IN") {
        log::puts("OCC299: OK");
    } else {
        log::puts("OCC299: Error 2");
    }
}

/// The uniform abscissa points along the wire reach the expected point (1%);
/// the length and checksection.
pub fn bug367(s: &mut dyn Session) {
    let result = restore_as(s, "OCC105.brep", "result").shape();
    checkshape(&result);
    occ367(&result, 0.5, [-307.47165394, -340.18073533, 0.0], 0.01);
    checkprops(&result, Prop::Length, 1760.13);
    checksection(&result);
    checkview_display(s, Some("result"), "bug367");
}

/// `regexp {The point is (.*) shape} text full status`.
fn point_status(text: &str) -> Option<&str> {
    let start = text.find("The point is ")? + "The point is ".len();
    let end = text.rfind(" shape")?;
    (end >= start).then(|| &text[start..end])
}

/// The point is IN the solid.
pub fn bug372(s: &mut dyn Session) {
    let b1 = restore_as(s, "OCC372.brep", "b1").shape();
    checkshape(&b1);
    let p1 = set(
        s,
        "p1",
        Point3::new(6311.4862583184, -2841.3092756034, 16.461053497188),
    );
    let result = occ299(&b1, p1);
    if llength(&result) < 4 {
        log::puts("OCC372: ERROR 1");
    } else {
        let status = point_status(&result).unwrap_or("");
        log::puts(&format!("status = {status}"));
        if status == "IN" {
            log::puts("OCC372: OK");
        } else {
            log::puts("Faulty : OCC372");
        }
    }
}

/// The restored point is IN the solid.
pub fn bug454(s: &mut dyn Session) {
    let b1 = restore_as(s, "OCC454_b1.brep", "b1").shape();
    checkshape(&b1);
    let p1 = restore_as(s, "OCC454_p1.draw", "p1").point();
    let info_result = occ299(&b1, p1);
    if llength(&info_result) < 4 {
        log::puts("OCC454: ERROR 1");
    } else if info_result.contains("IN") {
        log::puts("OCC454: OK");
    } else {
        log::puts("OCC454: ERROR 2");
    }
}

/// The 2D intersection of the edges projected on the plane has no segment whose
/// first point differs on the two curves (the command prints no `Faulty`).
pub fn bug8169(s: &mut dyn Session) {
    let e1 = restore_as(s, "OCC8169_1.brep", "e1").shape();
    let e2 = restore_as(s, "OCC8169_2.brep", "e2").shape();
    let p = set(s, "p", plane([0., 0., 500.], [0., 0., 1.], [1., 0., 0.]));
    log::result(&crate::support::commands::dump_surface("p", &p));
    let result = set(s, "result", mkface(&p));
    if occ8169(&e1, &e2, &result).contains("Faulty") {
        log::puts("Faulty OCC6768");
    } else {
        log::puts("OK OCC6768");
    }
    checkview_display(s, Some("result"), "bug8169");
}

/// The pipe of the path and the two rails; the
/// mean distance between the rails and the pipe's iso-curves 0 and 1 exceeds the radius
/// (bug9_1) or the pipe tolerance (bug9_2).
fn bug9(s: &mut dyn Session, tolerance: Option<f64>, image: &str) {
    log::todo("Faulty OCC9");
    let d = restore_as(s, "OCC9-path.draw", "d").curve();
    let b = restore_as(s, "OCC9-cur1.draw", "b").curve();
    let c = restore_as(s, "OCC9-cur2.draw", "c").curve();
    let p0 = cvalue(&b, 0.0);
    let v = set(s, "V", vertex(p0));
    let e = set(s, "E", mkedge(&d));
    let check_dist = distmini(s, "dm", &v, &e, None).unwrap().value;
    let res = pipe_occ9(&d, [&b, &c], check_dist, tolerance).unwrap_or_else(|e| panic!("{e}"));
    let res = set(s, "res", res);
    let surface = res.get().unwrap();
    let (mut n, mut sum) = (1, 0.0);
    let mut i = -470;
    while i < 240 {
        n += 1;
        let u = f64::from(i);
        let dist = cvalue(&b, u).distance(&surface.d0(0.0, u).unwrap());
        sum += dist;
        log::puts(&format!("1-st Dist at {i} : {dist}"));
        let dist = cvalue(&c, u).distance(&surface.d0(1.0, u).unwrap());
        sum += dist;
        log::puts(&format!("2-nd Dist at {i} : {dist}"));
        i += 25;
    }
    let avg_dist = sum / f64::from(2 * n);
    // bug9_1 compares with `[dval dm_val]`, bug9_2 with `${tolerance}`.
    let check_dist = tolerance.unwrap_or(check_dist);
    log::puts(&format!("\n Avg dist on {n} intervals: {avg_dist}"));
    log::puts(&format!("\n check_dist = {check_dist}"));
    if avg_dist > check_dist {
        log::puts("\n Faulty OCC9");
    } else {
        log::puts("\n OCC9: OK");
    }
    checkview_display(s, None, image);
}

pub fn bug9_1(s: &mut dyn Session) {
    bug9(s, None, "bug9_1");
}

pub fn bug9_2(s: &mut dyn Session) {
    bug9(s, Some(0.0001), "bug9_2");
}
