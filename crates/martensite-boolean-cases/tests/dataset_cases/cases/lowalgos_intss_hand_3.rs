//! `lowalgos_intss` scripts built on `bopcurves`: what it answers, and the curves it binds.
use crate::support::{
    bop_session::BopSession,
    bopcurves::{BopCurvesOptions, BoundCurves, bopcurves_bound},
    checkprops::{Expected, checkprops},
    checkview::{CheckView, checkview, screenshot},
    commands::{add, explode, orientation, pcone, prism, psphere, trotate},
    construction::copy,
    construction::wire,
    lowalgos_commands::{
        banner, checklength, curve_length, curve_range, donly_curves, mkplane_only, mksurface,
        name_refs, plane, qa_check_bends, sphere, tmirror,
    },
    nbshapes::{Counting, NbCount, checknbshapes},
    one_step::{bcommon, bsection},
    properties::Prop,
    restore::restore,
    section_check::checksection_expecting,
    session::{Session, ViewStep},
    surface_commands::{circle_along, mkedge, mkface},
    surface_intersection::{edge_of, xdistcs},
    tolerance_check::{checkmaxtol_ref, checkreal},
};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use std::f64::consts::PI;

pub fn bug21494_1(s: &mut dyn Session) {
    banner(8, "OCC21494: Intersection between cone and sphere fails");
    pcone(s, "bc", 15., 0., 10.);
    psphere(s, "bs", 10.);
    explode(s, "bc", Some(ShapeEnum::Face));
    explode(s, "bs", Some(ShapeEnum::Face));
    let log = bopcurves_bound(s, "bc_1", "bs_1", BopCurvesOptions::TWO_D);
    let (_, nb_curv) = log.reached_and_count();
    if !format!("{} point(s) found", log.point_names.len()).contains("1 point(s) found") {
        println!("Error: Cone apex and Pole of sphere are excluded from the intersection result");
    }
    if nb_curv != 1 {
        println!("Error: Please check NbCurves for intersector");
    }
    if log.reached() > 2.0e-7 {
        println!("Error: Big tolerance value");
    }
    s.view(ViewStep::SmallView);
    donly_curves(s, &log, &name_refs(&log.point_names));
    s.view(ViewStep::Fit);
    s.view(ViewStep::Disp(&["bc", "bs"]));
    screenshot(s, "photo_3d");
    // Beyond the script: an earlier test of this case saw both curves on the faces.
    assert_eq!(
        log.plane_names,
        ["c2d1_1", "c2d2_1"],
        "bug21494_1: plane curves"
    );
}

pub fn bug23948_1(s: &mut dyn Session) {
    banner(9, "OCC23948");
    restore(s, "bug22766_f1", "f1");
    restore(s, "bug22766_f2", "f2");
    let log = bopcurves_bound(s, "f1", "f2", BopCurvesOptions::TWO_D);
    let (toler, nb_curv) = log.reached_and_count();
    if nb_curv != 2 {
        println!("Error: 2 curve(s) expected, but {nb_curv} found.");
    }
    checkreal("TolReached", toler, "1.0e-6", 0.0, 0.1);
    let mut bop = BopSession::default();
    for line in &log.lines {
        edge_of(s, "ee", line.curve());
        bop.add_object(s, "ee");
    }
    if nb_curv > 1 {
        bop.bfillds();
        bop.bbuild(s, "rs");
        checksection_expecting(s, "rs", 2);
        checkmaxtol_ref(s, "rs", "1.0e-6");
        checkprops(s, "rs", Prop::Length, Expected::Mass(111.803));
    }
    s.view(ViewStep::SmallView);
    donly_curves(s, &log, &[]);
    s.view(ViewStep::Fit);
    s.view(ViewStep::Disp(&["f1", "f2"]));
    screenshot(s, "photo");
}

pub fn bug23948_2(s: &mut dyn Session) {
    banner(9, "OCC23948");
    restore(s, "bug22766_f1", "f1");
    restore(s, "bug22766_f2", "f2");
    let (s1, s2) = (mksurface(s, "s1", "f1"), mksurface(s, "s2", "f2"));
    let log = bopcurves_bound(s, "f1", "f2", BopCurvesOptions::PLAIN);
    let (_, nb_curv) = log.reached_and_count();
    for index in 0..nb_curv {
        let curve = log.curve_at(index);
        let range = curve_range(&curve);
        if range[1] - range[0] < 1.0e-9 {
            println!("Error: Wrong curve's range!");
        }
        xdistcs(&log.names[index], &curve, &s1, [0., 1.], 10, 2.0e-7);
        xdistcs(&log.names[index], &curve, &s2, [0., 1.], 10, 2.0e-7);
    }
    s.view(ViewStep::SmallView);
    donly_curves(s, &log, &[]);
    s.view(ViewStep::Fit);
    s.view(ViewStep::Disp(&["s1", "s2"]));
    screenshot(s, "photo");
}

/// The body of the script's loop, after its `restore`.
fn bug24676_pair(s: &mut dyn Session, name: &str) {
    let log = bopcurves_bound(s, name, "b2", BopCurvesOptions::PLAIN);
    if log.reached_and_count().1 != 1 {
        println!("Error: Number of curves is wrong");
    }
}

pub fn bug24676(s: &mut dyn Session) {
    banner(8, "0024676: Wrong result done by intersection algorithm");
    restore(s, "bug24676_b2.brep", "b2");
    restore(s, "bug24676_b1_1.brep", "b1_1");
    bug24676_pair(s, "b1_1");
    restore(s, "bug24676_b1_2.brep", "b1_2");
    bug24676_pair(s, "b1_2");
    restore(s, "bug24676_b1_3.brep", "b1_3");
    bug24676_pair(s, "b1_3");
    restore(s, "bug24676_b1_4.brep", "b1_4");
    bug24676_pair(s, "b1_4");
    restore(s, "bug24676_b1_5.brep", "b1_5");
    bug24676_pair(s, "b1_5");
    restore(s, "bug24676_b1_6.brep", "b1_6");
    bug24676_pair(s, "b1_6");
}

pub fn bug25818(s: &mut dyn Session) {
    banner(12, "OCC25818");
    plane(s, "pn1", [0., 0., 0.], [0., 0., 1.], [1., 0., 0.]);
    plane(s, "pn2", [0., 0., 0.], [1., 0., 0.], [0., 1., 0.]);
    let surfaces = (
        plane(s, "pn1", [0., 0., 0.], [0., 0., 1.], [1., 0., 0.]),
        plane(s, "pn2", [0., 0., 0.], [1., 0., 0.], [0., 1., 0.]),
    );
    mkface(s, "f1", &surfaces.0);
    mkface(s, "f2", &surfaces.1);
    bopcurves_bound(s, "f1", "f2", BopCurvesOptions::PLAIN);
    s.view(ViewStep::SmallView);
    s.view(ViewStep::Fit);
    screenshot(s, "photo");
}

/// `bug25820_2` and `bug25820_3` after the second solid: only its face differs.
fn bug25820(s: &mut dyn Session, face: &str) {
    explode(s, "b2", Some(ShapeEnum::Face));
    copy(s, face, "b2");
    s.view(ViewStep::SmallView);
    s.view(ViewStep::Clear);
    s.view(ViewStep::Disp(&["b1"]));
    s.view(ViewStep::Disp(&["b2"]));
    s.view(ViewStep::Fit);
    let log = bopcurves_bound(s, "b1", "b2", BopCurvesOptions::PLAIN);
    let (toler, nb_curv) = log.reached_and_count();
    if log.reached() > 1.0e-7 {
        println!("ERROR: OCC25820 is reproduced. Tolerance is to large ({toler}).");
    }
    if nb_curv != 1 {
        println!("ERROR: OCC25820 is reproduced. Too many curves were found.");
    }
    screenshot(s, "photo");
}

fn bug25820_first(s: &mut dyn Session) {
    banner(8, "OCC25820");
    restore(s, "OCC25820-Revolution_5.brep", "b1");
    explode(s, "b1", Some(ShapeEnum::Face));
    copy(s, "b1_4", "b1");
}

pub fn bug25820_2(s: &mut dyn Session) {
    bug25820_first(s);
    restore(s, "OCC25820-Translation_2.brep", "b2");
    bug25820(s, "b2_4");
}

pub fn bug25820_3(s: &mut dyn Session) {
    bug25820_first(s);
    restore(s, "OCC25820-Translation_3.brep", "b2");
    bug25820(s, "b2_3");
}

/// The branch of `bug25890` for more than one curve: the sum of lengths, and no pair in common.
fn bug25890_overlaps(s: &mut dyn Session, log: &BoundCurves, expected: &str) {
    let mut sum = 0.0;
    for first in 0..log.lines.len() {
        let printed = curve_length(&log.names[first], &log.curve_at(first), 1.0e-4);
        sum += printed.parse::<f64>().unwrap();
        for second in first + 1..log.lines.len() {
            println!(" Check c_{} and c_{}", first + 1, second + 1);
            edge_of(s, "e1", &log.curve_at(first));
            edge_of(s, "e2", &log.curve_at(second));
            bcommon(s, "rr", "e1", "e2");
            let empty = [NbCount::Compounds(1), NbCount::Shapes(1)];
            checknbshapes(s, "rr", Counting::Typed, &empty);
        }
    }
    checkreal("Length", &sum.to_string(), expected, 1.0e-7, 0.0);
}

pub fn bug25890(s: &mut dyn Session) {
    banner(12, "OCC25890");
    let exp_len = "5.036361740755879";
    restore(s, "bug25890_f1.brep", "f1");
    restore(s, "bug25890_f2.brep", "f2");
    let log = bopcurves_bound(s, "f1", "f2", BopCurvesOptions::TWO_D);
    let (toler, nb_curv) = log.reached_and_count();
    checkreal("TolReached", toler, "1.5361108163989659e-007", 0.0, 0.1);
    if nb_curv != 1 {
        println!("Error : NbCurv is bad");
        bug25890_overlaps(s, &log, exp_len);
    } else {
        checklength("c_1", &log.curve_at(0), exp_len, 1.0e-4);
    }
}

pub fn bug25952_1(s: &mut dyn Session) {
    banner(8, "0025952: Wrong intersection curve");
    println!("1"); // What `pload QAcommands` returns.
    println!("TODO OCC25952 ALL: Error: The curve c_1 is possible to have a bend at");
    restore(s, "bug25952_shape.brep", "q");
    explode(s, "q", None);
    copy(s, "q_1", "b1");
    copy(s, "q_2", "b2");
    let log = bopcurves_bound(s, "b1", "b2", BopCurvesOptions::TWO_D);
    let (toler, nb_curv) = log.reached_and_count();
    checkreal("Tolerance", toler, "1.0e-7", 1.0e-7, 0.0);
    if nb_curv != 1 {
        println!("Error: Please check NbCurves for intersector");
    } else {
        let curve = log.curve_at(0);
        checklength("c_1", &curve, "0.00068663591416249451", 1.0e-4);
        let scan = qa_check_bends("c_1", &curve);
        // Beyond the script: an earlier test of this case held the one bend and where it is.
        assert_eq!(scan.flagged, ["0.47100000000000036"], "QACheckBends c_1");
        assert_eq!(scan.samples, 1000, "QACheckBends c_1");
        assert_eq!(curve_range(&curve), [0., 1.], "bounds c_1");
    }
    s.view(ViewStep::SmallView);
    donly_curves(s, &log, &[]);
    s.view(ViewStep::Fit);
    s.view(ViewStep::Disp(&["b1", "b2"]));
    screenshot(s, "photo");
}

pub fn bug26008(s: &mut dyn Session) {
    banner(16, "OCC26008");
    let first = sphere(s, "ss1", [78.4160443343, 8.94293392454, 70.9553808628], 6.5);
    let second = sphere(s, "ss2", [82.5593917073, 4.52613090823, 76.3299967], 3.5);
    mkface(s, "f1", &first);
    mkface(s, "f2", &second);
    bopcurves_bound(s, "f1", "f2", BopCurvesOptions::TWO_D);
    s.view(ViewStep::PlaneView);
    s.view(ViewStep::Fit);
    s.view(ViewStep::Xwd("photo"));
}

/// The construction every `bug26509` script starts with.
fn bug26509_faces(s: &mut dyn Session) {
    banner(
        8,
        "0026509: Wrong intersection curve(s) obtained for pair of faces.",
    );
    let down = [0., -1., 0.];
    let (c1, c2) = (
        circle_along([0.; 3], down, 75.),
        circle_along([0.; 3], down, 65.),
    );
    mkedge(s, "c1", &c1);
    mkedge(s, "c2", &c2);
    wire(s, "c1", &["c1"]);
    wire(s, "c2", &["c2"]);
    orientation(s, "c2", Orientation::Reversed);
    mkplane_only(s, "f1", "c1");
    add(s, "c2", "f1");
    prism(s, "p1", "f1", [0., 1000., 0.]);
    trotate(s, "p1", [0.; 3], [0., 0., 1.], 180.);
    let c3 = circle_along([50., 500., -2000.], [0., 0., 1.], 50.);
    mkedge(s, "c3", &c3);
    wire(s, "c3", &["c3"]);
    mkplane_only(s, "f2", "c3");
    prism(s, "p2", "f2", [0., 0., 4000.]);
    explode(s, "p1", Some(ShapeEnum::Face));
    explode(s, "p2", Some(ShapeEnum::Face));
    trotate(s, "p2_1", [0., 500., 0.], [1., 0., 0.], -77.);
    trotate(s, "p2_1", [0.; 3], [0., 0., 1.], 180.);
}

/// The checks every `bug26509` script ends with.
fn bug26509_checks(s: &mut dyn Session, expected_curves: usize) {
    let log = bopcurves_bound(s, "p1_2", "p2_1", BopCurvesOptions::TWO_D);
    let (toler, nb_curv) = log.reached_and_count();
    checkreal("Tolerance", toler, "3.6430300265667996e-07", 0.0, 0.1);
    if nb_curv != expected_curves {
        println!("Error: Please check NbCurves for intersector");
    }
    println!("Check whether the intersection result is closed in 3D");
    bsection(s, "result", "p1_2", "p2_1");
    checksection_expecting(s, "result", 0);
    checkmaxtol_ref(s, "result", "4.0e-7");
    checkprops(s, "result", Prop::Length, Expected::Mass(1881.92));
    checkview(s, &CheckView::of("result"));
}

const P1_AXIS: [f64; 3] = [1.22464679914735e-16, 1., 0.];

fn turned_second(s: &mut dyn Session) {
    trotate(
        s,
        "p2_1",
        [-49.9999999999998, 1448.74012957047, -449.90210868773],
        [1.19325918102424e-16, 0.974370064785235, -0.224951054343865],
        -1.26610367277949897868 * 180. / PI,
    );
}

pub fn bug26509_1(s: &mut dyn Session) {
    bug26509_faces(s);
    bug26509_checks(s, 3);
}

pub fn bug26509_2(s: &mut dyn Session) {
    bug26509_faces(s);
    trotate(
        s,
        "p1_2",
        [0.; 3],
        P1_AXIS,
        -1.57079632679489655800 * 180. / PI,
    );
    bug26509_checks(s, 1);
}

pub fn bug26509_3(s: &mut dyn Session) {
    bug26509_faces(s);
    trotate(
        s,
        "p1_2",
        [0.; 3],
        P1_AXIS,
        1.57079632679489655800 * 180. / PI,
    );
    bug26509_checks(s, 2);
}

pub fn bug26509_4(s: &mut dyn Session) {
    bug26509_faces(s);
    trotate(
        s,
        "p1_2",
        [0.; 3],
        P1_AXIS,
        1.57079632679489655800 * 180. / PI,
    );
    tmirror(s, "p1_2", [0., -500., 0.], [0., 1., 0.]);
    bug26509_checks(s, 2);
}

pub fn bug26509_5(s: &mut dyn Session) {
    bug26509_faces(s);
    turned_second(s);
    bug26509_checks(s, 2);
}

pub fn bug26509_6(s: &mut dyn Session) {
    bug26509_faces(s);
    trotate(
        s,
        "p1_2",
        [0.; 3],
        P1_AXIS,
        -1.57079632679489655800 * 180. / PI,
    );
    turned_second(s);
    bug26509_checks(s, 1);
}
