//! `tests/bugs/heal`: same-domain unification of a seam, of shared solids and of a mirrored fuse.
use crate::support::{
    beyond_script::same_masses,
    bop_session::BopSession,
    checkprops::checkprops_equal,
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    commands::{compound, explode, make_box, pcylinder, prism},
    construction::{copy, mkplane, vertex},
    heal_commands::{UnifyOptions, bopcheck_is_ok, checkshape_all, unifysamedom},
    lowalgos_commands::{cylinder_from_normal, tmirror},
    nbshapes::{Counting, NbCount, checknbshapes, checknbshapes_like_counting},
    session::Session,
    surface_commands::{mkface_bounded, polyline},
    two_step::TwoStepBoolean,
};
use martensite_tkbo::bop_algo::Operation;
use std::f64::consts::PI;

pub fn bug29502(s: &mut dyn Session) {
    println!("========");
    println!("OCC29502");
    println!("========");
    println!();
    let c = cylinder_from_normal(s, "c", [0.; 3], [0., 0., 1.], 10.);
    mkface_bounded(s, "f", &c, (0., 2. * PI, -10., 10.));
    vertex(s, "v", [10., 0., 0.]);
    let mut bop = BopSession::default();
    bop.arguments(s, &["f"], &["v"]);
    bop.bfillds();
    bop.bsplit(s, "r");
    let split = [
        NbCount::Vertices(3),
        NbCount::Edges(4),
        NbCount::Wires(1),
        NbCount::Faces(1),
    ];
    checknbshapes(s, "r", Counting::Located, &split);
    unifysamedom(s, "result", "r", &[], UnifyOptions::default());
    checkshape_named(s, "result");
    checkprops_equal(s, "result", "f");
    checknbshapes_like_counting(s, "result", "f", Counting::Located);
    // Beyond the script: an earlier test of this case compared the area too, not only the length.
    same_masses(s, "result", "f");
}

fn invalid_after_unification(s: &mut dyn Session, name: &str) {
    if !bopcheck_is_ok(s, name) {
        println!("Error: invalid shape after faces unification");
    }
}

/// `unifysamedom first r`, `unifysamedom second r +i` and what the scripts check of both.
fn unified_both_ways(s: &mut dyn Session, names: [&str; 2], reference: &str) {
    unifysamedom(s, names[0], "r", &[], UnifyOptions::default());
    let internal = UnifyOptions {
        allow_internal: true,
        ..UnifyOptions::default()
    };
    unifysamedom(s, names[1], "r", &[], internal);
    checkshape_all(s, &names);
    checknbshapes_like_counting(s, names[0], reference, Counting::Typed);
    checknbshapes_like_counting(s, names[1], reference, Counting::Typed);
    invalid_after_unification(s, names[0]);
    invalid_after_unification(s, names[1]);
}

/// From `checkshape r` on, the same in `bug33171_1` and `bug33171_2`.
fn bug33171_checks(s: &mut dyn Session, invalid: &str) {
    checkshape_named(s, "r");
    if !bopcheck_is_ok(s, "r") {
        println!("{invalid}");
    }
    unified_both_ways(s, ["ru1", "ru2"], "r");
    let shells = explode(s, "r", None);
    let names: Vec<&str> = shells.iter().map(String::as_str).collect();
    compound(s, &names, "shs");
    unified_both_ways(s, ["shsu1", "shsu2"], "shs");
    checkview(s, &CheckView::of("ru2"));
}

fn bug33171_banner() {
    println!("========================");
    println!(" \t0033171: Modeling Algorithms - Invalid result of faces unification");
    println!("========================");
    println!();
}

pub fn bug33171_1(s: &mut dyn Session) {
    bug33171_banner();
    let square = [
        [0., 0., 0.],
        [10., 0., 0.],
        [10., 10., 0.],
        [0., 10., 0.],
        [0., 0., 0.],
    ];
    polyline(s, "p", &square);
    mkplane(s, "f", "p");
    prism(s, "s", "f", [0., 0., 5.]);
    let first = [
        [3., 10., 0.],
        [3., 7., 0.],
        [6., 7., 0.],
        [6., 3., 0.],
        [10., 3., 0.],
    ];
    polyline(s, "p1", &first);
    polyline(s, "p2", &[[6., 7., 0.], [10., 7., 0.]]);
    polyline(s, "p3", &[[8., 7., 0.], [8., 10., 0.]]);
    polyline(s, "p4", &[[0., 5., 0.], [10., 5., 0.]]);
    prism(s, "sh1", "p1", [0., 0., 5.]);
    prism(s, "sh2", "p2", [0., 0., 5.]);
    prism(s, "sh3", "p3", [0., 0., 5.]);
    prism(s, "sh4", "p4", [0., 0., 5.]);
    let mut bop = BopSession::default();
    bop.arguments(s, &["s"], &["sh1", "sh2", "sh3", "sh4"]);
    bop.bfillds();
    bop.bsplit(s, "r");
    bug33171_checks(s, "Error: invalid shape after split");
}

pub fn bug33171_2(s: &mut dyn Session) {
    bug33171_banner();
    make_box(s, "b1", [0.; 3], [10., 10., 5.]);
    make_box(s, "b2", [10., 0., 0.], [5., 5., 5.]);
    let mut bop = BopSession::default();
    bop.arguments(s, &["b1", "b2"], &[]);
    bop.bfillds();
    bop.bbuild(s, "r");
    bug33171_checks(s, "Error: invalid shape after fuse");
}

pub fn bug33421(s: &mut dyn Session) {
    println!("TODO CR33439 ALL: Error :  is WRONG because number of EDGE entities in shape");
    println!("TODO CR33439 ALL: Error :  is WRONG because number of WIRE entities in shape");
    println!("TODO CR33439 ALL: Error :  is WRONG because number of FACE entities in shape");
    println!("=========================================");
    println!("0033421: Modeling Algorithms - ShapeUpgrade_UnifySameDomain fails");
    println!("=========================================");
    println!();
    pcylinder(s, "c1", 10., 10.);
    copy(s, "c1", "c2");
    tmirror(s, "c2", [0., 0., 10.], [0., 0., 1.]);
    let mut two_step = TwoStepBoolean::default();
    two_step.bop(s, "c1", "c2");
    two_step.result(s, "c3", Operation::Fuse);
    unifysamedom(s, "result", "c3", &[], UnifyOptions::default());
    let expected = [
        NbCount::Vertices(3),
        NbCount::Edges(3),
        NbCount::Wires(3),
        NbCount::Faces(3),
        NbCount::Shells(1),
        NbCount::Solids(1),
    ];
    checknbshapes(s, "result", Counting::Typed, &expected);
    checkview(s, &CheckView::of("result"));
}
