//! `bugs_moddata_1` case bodies whose
//! inputs are in the public dataset.
use crate::support::{
    checks::{checklength2d, checknbshapes, checkreal},
    commands::{dump_curve, length3d, project, segment},
    restore::restore_as,
    session::{Session, checkview_display, set},
};
use martensite_tkernel::{precision, standard::Handle};
use martensite_tkg3d::geom::GeomCurve;

/// The projection and its checklength.
pub fn buc60965(s: &mut dyn Session) {
    let c = restore_as(s, "buc60965-c.draw", "c").curve();
    let surface = restore_as(s, "buc60965-s.draw", "s").surface();
    let result = project(&c, &surface, precision::confusion()).expect("Projection Failed");
    checklength2d(&set(s, "result", result), 3.141968717040271);
    checkview_display(s, Some("result"), "buc60965");
}

/// `catch { project result c s }` must not fail
/// (a failure prints `Faulty`).
fn projection_succeeds(
    s: &mut dyn Session,
    surface: (&str, &str),
    curve: (&str, &str),
    image: &str,
) {
    let surf = restore_as(s, surface.0, surface.1).surface();
    let c = restore_as(s, curve.0, curve.1).curve();
    let result = project(&c, &surf, precision::confusion());
    set(s, "result", result.expect("Faulty: the projection failed"));
    checkview_display(s, Some("result"), image);
}

pub fn bug12635_1(s: &mut dyn Session) {
    let inputs = (("OCC12635_s1.draw", "s1"), ("OCC12635_c1.draw", "c1"));
    projection_succeeds(s, inputs.0, inputs.1, "bug12635_1");
}

pub fn bug12635_2(s: &mut dyn Session) {
    let inputs = (("OCC12635_s6.draw", "s6"), ("OCC12635_c18.draw", "c18"));
    projection_succeeds(s, inputs.0, inputs.1, "bug12635_2");
}

pub fn bug12884(s: &mut dyn Session) {
    let inputs = (("OCC12884_s2.draw", "s2"), ("OCC12884_c1.draw", "c1"));
    projection_succeeds(s, inputs.0, inputs.1, "bug12884");
}

pub fn bug12888(s: &mut dyn Session) {
    let inputs = (("OCC12888_s1.draw", "s1"), ("OCC12888_c2.draw", "c2"));
    projection_succeeds(s, inputs.0, inputs.1, "bug12888");
}

/// `segment result -4.76283758432072e-05 1.06458532027221` outside any catch.
pub fn bug141(s: &mut dyn Session) {
    let result = restore_as(s, "OCC141.draw", "result").curve();
    segment(&result, -4.76283758432072e-05, 1.06458532027221).expect("segment of OCC141");
    checkview_display(s, Some("result"), "bug141");
}

/// The seventh word list element of `dump res`: the periodicity word of a B-spline dump.
fn dump_word_6(curve: &Handle<dyn GeomCurve>) -> String {
    let dump = dump_curve("res", curve);
    let words: Vec<&str> = dump.split_whitespace().collect();
    assert!(words.len() >= 7, "Bad dump list of res: {dump}");
    words[6].to_string()
}

/// The periodic B-spline is periodic before `segment res t1 t2` over its own
/// bounds and no longer periodic after it, with the same length (`checkreal`, 0.001).
pub fn bug14782(s: &mut dyn Session) {
    let res = restore_as(s, "OCC14782.draw", "res").curve();
    let (t1, t2) = {
        let curve = res.get().unwrap();
        (curve.first_parameter(), curve.last_parameter())
    };
    assert_eq!(dump_word_6(&res), "periodic", "1. Faulty OCC14782");
    let ll1 = length3d(&res, precision::confusion());
    segment(&res, t1, t2).expect("segment res t1 t2");
    assert_ne!(dump_word_6(&res), "periodic", "2. Faulty OCC14782");
    let ll2 = length3d(&res, precision::confusion());
    checkreal("length1", ll1, ll2, 0., 0.001);
}

/// The restored VRML-converted shape is one bare face.
pub fn bug19777(s: &mut dyn Session) {
    let result = restore_as(s, "OCC19777.brep", "result").shape();
    checknbshapes(&result, [0, 0, 0, 1, 0, 0, 0, 0, 1]);
    checkview_display(s, Some("result"), "bug19777");
}
