//! tests/bugs/modalg_5 case bodies at 3d097a0328e71b826377d4814ab05ec3c3d23871 owned by
//! TKShHealing whose inputs are in the public dataset or built by the script.
use super::bugs_common::{banner, tolerance_max};
use crate::support::{
    geom_create::{circle, line, trim_curve},
    log,
    logged_checks::checknbshapes_some,
    props::{Prop, checkprops},
    restore::{Restored, restore_as},
    session::{Session, ViewStep, set},
    sewing::{checkfreebounds, checkmaxtol, sewing_with},
    swdraw::settolerance,
    topology::{mkedge, mkplane, sameparameter_in_place, vertex, wire},
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::storage::format_general;
use martensite_tkmath::gp::Point3;
use std::f64::consts::PI;

/// tests/bugs/modalg_5/bug24203.
/// `sameparameter e1` (in place, 1e-7) keeps the edge tolerance at most 0.20.
pub fn bug24203(s: &mut dyn Session) {
    banner(&["============", "OCC24203", "============", ""]);
    let e1 = restore_as(s, "bug24203_notspedge.brep", "e1").shape();
    sameparameter_in_place(&e1, 1.0e-7, false);
    let max_tol_1 = tolerance_max(&e1);
    log::puts(&format!("MaxTolerance = {max_tol_1}"));
    if max_tol_1 > 0.20 {
        log::puts("Faulty OCC24203");
    } else {
        log::puts("OCC24203 OK");
    }
}

/// tests/bugs/modalg_5/bug24347.
/// The minimal distance between the face (tolerance 4e-6) and the vertex, printed by
/// `dump d1_val` at precision 15, is exactly 2.72742879238428e-14.
pub fn bug24347(s: &mut dyn Session) {
    banner(&["============", "OCC24347", "============", ""]);
    let checkvalue = 2.72742879238428e-14;
    let f = restore_as(s, "bug24347_face.brep", "f").shape();
    let v = set(
        s,
        "v",
        vertex(Point3::new(34.265509314637, 0.1, 0.73361964922719)),
    );
    log::result(&settolerance(&f, 4.0e-6));
    let d1 = crate::support::distance::distmini(s, "d1", &f, &v, None)
        .expect("dump: d1_val is not a variable (no distance found)");
    let dump = format!(
        "\n\n*********** Dump of d1_val *************\n{}",
        format_general(d1.value, 15)
    );
    log::result(&dump);
    let distvalue: f64 = dump.split_whitespace().last().unwrap().parse().unwrap();
    if distvalue != checkvalue {
        log::puts("Error: distance is incorrect");
    } else {
        log::puts("OK: distance is correct");
    }
}

/// One `circle c ...`, `trim c c u1 u2`, `mkedge e1 c`, `line l 0 0 0 0 1 0`,
/// `trim l l 0 2`, `mkedge e2 l`, `wire w e1 e2`, `mkplane plane w` block.
fn half_disc(s: &mut dyn Session, circle_words: &[f64], range: (f64, f64), name: &str) -> Shape {
    let c = trim_curve(&circle(circle_words), range.0, range.1);
    s.set("c", &Restored::Curve(c.clone()));
    let e1 = set(s, "e1", mkedge(&c));
    let l = trim_curve(&line([0., 0., 0., 0., 1., 0.]), 0., 2.);
    s.set("l", &Restored::Curve(l.clone()));
    let e2 = set(s, "e2", mkedge(&l));
    let w = set(s, "w", wire(&[&e1, &e2]).unwrap_or_else(|e| panic!("{e}")));
    set(s, name, mkplane(&w).unwrap_or_else(|e| panic!("{e}")))
}

/// `checknbshapes name -vertex v -edge e -wire w -face f -shell 1 -solid 0 -compsolid 0
/// -compound 0 -shape n`.
fn counts(shape: &Shape, [v, e, w, f, n]: [usize; 5]) {
    checknbshapes_some(
        shape,
        [
            Some(v),
            Some(e),
            Some(w),
            Some(f),
            Some(1),
            Some(0),
            Some(0),
            Some(0),
            Some(n),
        ],
    );
}

/// tests/bugs/modalg_5/bug24817.
/// Non-manifold sewing (`+n`) of two and three half discs.
pub fn bug24817(s: &mut dyn Session) {
    banner(&["========", "CR24817", "========", ""]);
    let plane1 = half_disc(s, &[0., 1., 0., 1.], (0.5 * PI, 1.5 * PI), "plane1");
    let plane2 = half_disc(s, &[0., 1., 0., 1.], (1.5 * PI, 2.5 * PI), "plane2");
    let plane3 = half_disc(
        s,
        &[0., 1., 0., 1., 0., 0., 1.],
        (1.5 * PI, 2.5 * PI),
        "plane3",
    );
    let sr1 = sewing_with(1.0e-6, &[&plane1, &plane2], &["+n"])
        .map(|shape| set(s, "sr1", shape))
        .expect("checknbshapes: sr1 is not a shape");
    let sr2 = sewing_with(1.0e-6, &[&plane1, &plane2, &plane3], &["+n"])
        .map(|shape| set(s, "sr2", shape))
        .expect("checknbshapes: sr2 is not a shape");
    counts(&sr1, [2, 3, 2, 2, 10]);
    checkprops(&sr1, Prop::Area, 3.14159);
    s.step(ViewStep::Clear);
    s.step(ViewStep::SmallView("AXON"));
    s.step(ViewStep::Donly(&["sr1"]));
    s.step(ViewStep::Fit);
    s.step(ViewStep::Xwd("bug24817_1"));
    counts(&sr2, [2, 4, 3, 3, 13]);
    checkprops(&sr2, Prop::Area, 4.71239);
    s.step(ViewStep::Donly(&["sr2"]));
    s.step(ViewStep::Fit);
    s.step(ViewStep::Xwd("bug24817_2"));
    checkmaxtol(&sr1, "sr1", 1.0000000015308085e-007);
    checkfreebounds(&sr1, "sr1", 2);
    checkmaxtol(&sr2, "sr2", 1.0000000015308085e-007);
    checkfreebounds(&sr2, "sr2", 0);
}
