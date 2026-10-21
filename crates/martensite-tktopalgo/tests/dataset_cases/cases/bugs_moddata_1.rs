//! `bugs_moddata_1` case bodies whose
//! inputs are in the public dataset.
use crate::support::{
    checks::nbshapes,
    checkshape::checkshape,
    commands::{dump_curve, mkcurve, mksurface},
    distance::distmini,
    geometry::{cvalue, extrema_curve_surface, intersect_curve_surface, line},
    log,
    logged_checks::{checkreal, nbshapes_count},
    props::{Prop, checkprops},
    qa::buc60652,
    restore::{Restored, restore_as},
    session::{Session, checkview_display, set},
    text::{llength, numbers_after},
    topology::{SAMEPARAMETER_TOLERANCE, compound, sameparameter_in_place, tcopy, tmirror, vertex},
};

/// `fsameparameter result` (forced, default 1e-7), its area.
pub fn buc60637(s: &mut dyn Session) {
    let result = restore_as(s, "buc60637a.brep", "result").shape();
    sameparameter_in_place(&result, SAMEPARAMETER_TOLERANCE, true);
    checkprops(&result, Prop::Area, 144.866);
    checkview_display(s, Some("result"), "buc60637");
}

/// Checkshape, `BUC60652 result` (OuterWire as `w`) and the area.
fn buc60652_case(s: &mut dyn Session, file: &str, area: f64, image: &str) {
    let result = restore_as(s, file, "result").shape();
    checkshape(&result);
    set(s, "w", buc60652(&result));
    checkprops(&result, Prop::Area, area);
    checkview_display(s, Some("result"), image);
}

pub fn buc60652_1(s: &mut dyn Session) {
    buc60652_case(s, "buc60652a.brep", 19614.8, "buc60652_1");
}

pub fn buc60652_2(s: &mut dyn Session) {
    buc60652_case(s, "buc60652b.brep", 29961.0, "buc60652_2");
}

pub fn buc60652_3(s: &mut dyn Session) {
    buc60652_case(s, "buc60652c.brep", 77815.1, "buc60652_3");
}

pub fn buc60652_4(s: &mut dyn Session) {
    buc60652_case(s, "shading_104.brep", 119134.0, "buc60652_4");
}

/// `fsameparameter result`, checkshape.
pub fn buc60667(s: &mut dyn Session) {
    let result = restore_as(s, "buc60667a.brep", "result").shape();
    sameparameter_in_place(&result, SAMEPARAMETER_TOLERANCE, true);
    checkshape(&result);
    checkview_display(s, Some("result"), "buc60667");
}

/// Checkshape of the shape and of its tcopy.
pub fn buc60755(s: &mut dyn Session) {
    log::todo("Faulty shapes in variables faulty_1 to faulty_");
    let a = restore_as(s, "buc60755a.brep", "a").shape();
    checkshape(&a);
    let result = set(s, "result", tcopy(&a));
    checkshape(&result);
    checkprops(&result, Prop::Area, 623.832);
    checkview_display(s, Some("result"), "buc60755");
}

/// Checkshape.
pub fn buc60769(s: &mut dyn Session) {
    let result = restore_as(s, "buc60769a.brep", "result").shape();
    checkshape(&result);
    checkview_display(s, Some("result"), "buc60769");
}

/// Distmini edge/face and distmini of the edge's start vertex/face agree (0.1).
pub fn bug107(s: &mut dyn Session) {
    let ee = restore_as(s, "OCC107-1.brep", "ee").shape();
    checkshape(&ee);
    let ff = restore_as(s, "OCC107-2.brep", "ff").shape();
    checkshape(&ff);
    let d1 = distmini(s, "dd1", &ee, &ff, None).unwrap().value;
    let cc = set(s, "cc", mkcurve(&ee));
    let point = cvalue(&cc, 0.0);
    let vv = set(s, "vv", vertex(point));
    let d2 = distmini(s, "dd2", &vv, &ff, None).unwrap().value;
    if (d1 - d2).abs() > 0.1 {
        log::puts("Faulty :WRONG computation of distances.");
    } else {
        log::puts("All is OK");
    }
}

/// The compound of both shapes and its area.
pub fn bug126(s: &mut dyn Session) {
    let sh1 = restore_as(s, "OCC126-1.brep", "sh1").shape();
    checkshape(&sh1);
    let sh2 = restore_as(s, "OCC126-2.brep", "sh2").shape();
    checkshape(&sh2);
    let result = set(s, "result", compound(&[&sh1, &sh2]));
    checkprops(&result, Prop::Area, 560.774);
    checkview_display(s, Some("result"), "bug126");
}

/// The line intersects the surface (GeomAPI_IntCS) in 2 points (or one).
pub fn bug130(s: &mut dyn Session) {
    let sh = restore_as(s, "OCC130.brep", "sh").shape();
    checkshape(&sh);
    let l = set(s, "l", line([-120., -100., 400.], [0., 0., 1.]));
    let surf = set(s, "surf", mksurface(&sh));
    match intersect_curve_surface(s, "res", &l, &surf) {
        Ok(results) => {
            log::puts("OCC130 OK : intersection found between a specific surface and a line");
            // `repeat 10`: the res_j that are points, stopping at the first that is not.
            let nom = (1..=10)
                .take_while(|j| {
                    let name = format!("res_{j}");
                    results
                        .iter()
                        .any(|(n, v)| *n == name && matches!(v, Restored::Point(_)))
                })
                .count();
            if nom != 2 && nom != 0 {
                log::puts(&format!(
                    "Faulty OCC130: Intersection was made WRONGLY: {nom} points"
                ));
            } else {
                log::puts(&format!(
                    "OCC130 OK : Intersection command works properly: {nom} points"
                ));
            }
        }
        Err(_) => {
            log::puts("Faulty OCC130 : NO intersection found between a specific surface and a line")
        }
    }
}

/// Checkshape reports the misoriented wire.
pub fn bug161(s: &mut dyn Session) {
    log::required("Faulty shapes in variables faulty_1 to faulty_");
    let result = restore_as(s, "OCC161.brep", "result").shape();
    checkshape(&result);
    checkview_display(s, Some("result"), "bug161");
}

/// One curve/surface extremum between the line and the surface, at the expected
/// maximum (35.6687907545308) or minimum (0) distance.
pub fn bug163(s: &mut dyn Session) {
    let res = restore_as(s, "OCC130.brep", "res").shape();
    checkshape(&res);
    let l = set(s, "l", line([-120., -100., 400.], [0., 0., 1.]));
    let surface = set(s, "s", mksurface(&res));
    let (che, segments) = extrema_curve_surface(s, &l, &surface);
    if llength(&che) != 1 {
        log::puts("Error: Invalid extrema number");
    }
    let info = dump_curve("ext_1", &segments[0]);
    log::result(&info);
    let ext_length = numbers_after(&info, "Parameters : 0 ", 1, "").unwrap()[0];
    let mut status = false;
    if ext_length > 35.0 && ext_length < 36.0 {
        checkreal("Max distance:", ext_length, 35.6687907545308, 0.01, 0.01);
        status = true;
    }
    if ext_length > 0.0 && ext_length < 1.0 {
        checkreal("Min distance:", ext_length, 0.0, 0.01, 0.01);
        status = true;
    }
    if !status {
        log::puts("Error: wrong extrema point");
    }
}

/// `tmirror` keeps the number of sub-shapes; the area.
pub fn bug175(s: &mut dyn Session) {
    let result = restore_as(s, "OCC175.brep", "result").shape();
    checkshape(&result);
    let col1 = nbshapes_count(&nbshapes(&result), "SHAPE");
    let result = set(s, "result", tmirror(&result, [0., 0., 0.], [0., 1., 0.]));
    let col2 = nbshapes_count(&nbshapes(&result), "SHAPE");
    if col1 != col2 {
        log::puts("Error : Command mirror does NOT work properly. ");
    }
    checkprops(&result, Prop::Area, 13091.3);
    checkview_display(s, Some("result"), "bug175");
}

/// Distmini between the two lines is at most 7e-13.
pub fn bug20404(s: &mut dyn Session) {
    let e1 = restore_as(s, "OCC20404-line1.brep", "e1").shape();
    let e2 = restore_as(s, "OCC20404-line2.brep", "e2").shape();
    let d = distmini(s, "d", &e1, &e2, None).unwrap().value;
    log::puts(&format!("Distance={d}"));
    if d > 7.0e-13 {
        log::puts("Faulty OCC20404");
    } else {
        log::puts("OK OCC20404");
    }
}
