//! `bugs_modalg_5` case bodies whose
//! inputs are in the public dataset.
use crate::support::{
    checks::nbshapes,
    checkshape::checkshape,
    distance::{bclassify_default, distmini},
    log,
    logged_checks::checknbshapes,
    props::{Prop, checkprops_equal, props_logged},
    restore::restore_as,
    session::{Session, ViewStep, set},
    topology::tcopy,
};
use martensite_tkg3d::top_abs::State;
use martensite_tkmath::gp::Point3;

/// Checkshape of both faces.
pub fn bug24035(s: &mut dyn Session) {
    log::required("Faulty shapes in variables faulty_1 to faulty_2");
    let face1 = restore_as(s, "bug24035_face1", "face1").shape();
    let face2 = restore_as(s, "bug24035_face2", "face2").shape();
    checkshape(&face1);
    checkshape(&face2);
}

/// The classification of the point must not hang (cpulimit).
pub fn bug24242(s: &mut dyn Session) {
    let b1 = restore_as(s, "bug24242_sx", "b1").shape();
    let p = set(s, "p", Point3::new(14.3213918, 20., 20.));
    bclassify_default(&b1, p);
}

/// `tcopy` keeps the length (`checkprops -equal` compares lprops).
pub fn bug24397_1(s: &mut dyn Session) {
    let result = restore_as(s, "case_1_wire3.brep", "result").shape();
    let result1 = set(s, "result1", tcopy(&result));
    checkprops_equal(&result1, &result);
}

/// The copy's shape counts
/// differ from the original's.
pub fn bug24397_2(s: &mut dyn Session) {
    log::todo("Error :  is WRONG because number of");
    let b = restore_as(s, "OCC485b.brep", "b").shape();
    let b1 = set(s, "b1", tcopy(&b));
    let reference = nbshapes(&b);
    log::result(&reference);
    checknbshapes(&b1, [None; 9], Some(&reference));
}

/// Distmini between the extruded surface and the line is 0.
pub fn bug25270(s: &mut dyn Session) {
    let f = restore_as(s, "bug25270_face.brep", "f").shape();
    let e = restore_as(s, "bug25270_edge.brep", "e").shape();
    let ddist = distmini(s, "result", &f, &e, None).unwrap().value;
    if ddist > 1.0e-7 {
        log::puts(&format!(
            "Error: Extrema between extruded surface and line is incorrect {ddist}"
        ));
    }
}

/// The restored point is OUT of the solid; smallview, fit and the screenshot.
pub fn bug25555(s: &mut dyn Session) {
    let p = restore_as(s, "bug25555_p", "p").point();
    let z = restore_as(s, "bug25555_z", "z").shape();
    if bclassify_default(&z, p) != State::Out {
        log::puts("Error : Wrong result of classification of a point relative to solid");
    }
    s.step(ViewStep::SmallView("AXON"));
    s.step(ViewStep::Fit);
    s.step(ViewStep::Xwd("bug25555"));
}

/// 0027063: `lprops` of an edge without curves must not raise.
pub fn bug27063(s: &mut dyn Session) {
    let result = restore_as(s, "bug27063_emptyedge.brep", "result").shape();
    log::puts("Check that lprops does not crash on empty edge");
    props_logged(Prop::Length, &result, None);
}
