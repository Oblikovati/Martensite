//! `tests/perf/modalg`: operations timed, with the checks their scripts make afterwards.
use crate::support::{
    bopcurves::{BopCurvesOptions, bopcurves_bound},
    checkprops::{Expected, checkprops, checkprops_equal},
    checkshape::{checkshape, checkshape_named},
    checkview::{CheckView, checkview},
    commands::{compound, explode, make_box, pcone, pcylinder, prism, ttranslate},
    construction::{copy, tcopy},
    lowalgos_commands::{Curve, cylinder},
    nbshapes::{Counting, NbCount, checknbshapes, checknbshapes_like},
    one_step::{bcut, bfuse, bsection},
    perf_commands::{Chrono, bcut_parallel, bezier_as_spline, xdistef},
    properties::Prop,
    restore::restore,
    section_check::checksection,
    session::Session,
    surface_commands::{mkedge, mkface, mkface_bounded, plane},
    tolerance_check::checkreal,
};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::top_abs::ShapeEnum;

pub fn bug452_1(s: &mut dyn Session) {
    println!("========");
    println!("OCC452");
    println!("(case 1)");
    println!("========");
    println!();
    pcone(s, "pc", 10., 0., 20.);
    explode(s, "pc", Some(ShapeEnum::Face));
    prism(s, "pcy", "pc_2", [0., 0., 10.]);
    let mut chrono = Chrono::started();
    bcut(s, "result", "pc", "pcy");
    chrono.stop();
    chrono.counter("bcut");
    checkprops(s, "result", Prop::Area, Expected::Mass(254.16));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

/// The script's `checkshape` of an input, and the branch on the word it reads back.
fn source_validity(s: &dyn Session, name: &str, number: usize) {
    let shape = s
        .shape(name)
        .unwrap_or_else(|| panic!("checkshape {name}: expected a bound shape"));
    if checkshape(shape).has_faulty() {
        println!(
            "Faulty OCC452 (shape {number}): Source shape is invalid. It was detected by Checkshape command"
        );
    } else {
        println!("OCC452 OK (shape {number}): Source shape is valid");
    }
}

pub fn bug452_4(s: &mut dyn Session) {
    println!("========");
    println!("PRO7934");
    println!("OCC452");
    println!("(case 4)");
    println!("========");
    println!();
    restore(s, "CTO900_pro7934a.rle", "a");
    source_validity(s, "a", 7);
    restore(s, "CTO900_pro7934b.rle", "b");
    source_validity(s, "b", 8);
    let mut chrono = Chrono::started();
    bfuse(s, "result", "a", "b");
    chrono.stop();
    chrono.counter("bfuse");
    checkprops(s, "result", Prop::Area, Expected::Mass(201978.));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug25788(s: &mut dyn Session) {
    println!("=========");
    println!("OCC25788");
    println!("=========");
    println!();
    make_box(s, "b1", [0.; 3], [100., 100., 1.]);
    let n = 40;
    let mut holes = Vec::new();
    for i in 1..n {
        for j in 1..n {
            let name = format!("p_{i}_{j}");
            pcylinder(s, &name, 0.5, 1.);
            let at = [
                f64::from(i) * 100. / f64::from(n),
                f64::from(j) * 100. / f64::from(n),
                0.,
            ];
            ttranslate(s, &name, at);
            holes.push(name);
        }
    }
    let names: Vec<&str> = holes.iter().map(String::as_str).collect();
    compound(s, &names, "b2");
    let mut chrono = Chrono::started();
    bcut_parallel(s, "r", "b1", "b2");
    chrono.stop();
    chrono.counter("bcut");
}

pub fn bug26447(s: &mut dyn Session) {
    println!("============");
    println!("OCC26447");
    println!("============");
    println!();
    let c1 = cylinder(s, "c1", ([0.; 3], [1., 0., 0.], [0., -1., 0.]), 100.);
    let c2 = cylinder(s, "c2", ([0.; 3], [0., 1., 0.], [1., 0., 0.]), 100.);
    mkface(s, "f1", &c1);
    mkface(s, "f2", &c2);
    let mut chrono = Chrono::started();
    for _ in 1..=1000 {
        bopcurves_bound(s, "f1", "f2", BopCurvesOptions::PLAIN);
    }
    chrono.stop();
    chrono.counter("bopcurves");
}

fn bug26310_pair(s: &mut dyn Session, first: &str, second: &str) {
    let log = bopcurves_bound(s, first, second, BopCurvesOptions::TWO_D);
    let (toler, nb_curv) = log.reached_and_count();
    checkreal("ToleranceReached", toler, "3.05e-07", 0.0, 0.01);
    if nb_curv != 4 {
        println!("Error: Number of curves is bad!");
    }
}

pub fn bug26310_1(s: &mut dyn Session) {
    println!("========");
    println!("OCC26310");
    println!("========");
    println!();
    restore(s, "OCC26310-b1.brep", "b1");
    restore(s, "OCC26310-b2.brep", "b2");
    explode(s, "b1", Some(ShapeEnum::Face));
    explode(s, "b2", Some(ShapeEnum::Face));
    bug26310_pair(s, "b1_1", "b2_1");
    bug26310_pair(s, "b2_1", "b1_1");
}

/// `bug83_1` and `bug83_2` up to the timed section: the operands, and the plane both construct.
fn bug83_inputs(s: &mut dyn Session) {
    restore(s, "BUC60912_sec_slow.brep", "c");
    explode(s, "c", None);
    checkshape_named(s, "c_1");
    checkshape_named(s, "c_2");
    copy(s, "c_1", "sh");
    copy(s, "c_2", "pr");
    let f = plane([0.; 3], [1., 0., 0.]);
    mkface_bounded(s, "f", &f, (-11., 11., -11., 11.));
}

fn bug83_checks(s: &mut dyn Session) {
    checkprops(s, "result", Prop::Length, Expected::Mass(42.879));
    checkshape_named(s, "result");
    checksection(s, "result");
    checkview(s, &CheckView::of("result"));
}

fn bug83_banner() {
    println!("============");
    println!("BUC60912");
    println!("OCC83");
    println!("============");
    println!();
}

pub fn bug83_1(s: &mut dyn Session) {
    bug83_banner();
    println!("1 case: perform section with plane");
    bug83_inputs(s);
    let mut chrono = Chrono::started();
    bsection(s, "result", "f", "pr");
    chrono.stop();
    chrono.counter("bsection");
    bug83_checks(s);
}

pub fn bug83_2(s: &mut dyn Session) {
    bug83_banner();
    println!("2 case: perform section with planar BSpline surface");
    bug83_inputs(s);
    println!("Info: perform section with planar BSpline surface");
    let mut chrono = Chrono::started();
    bsection(s, "result", "sh", "pr");
    chrono.stop();
    chrono.counter("bsection");
    bug83_checks(s);
}

/// `convert`, the script's `repeat` of `insertknot`, `mkedge`, `prism`, `explode`, and `xdistef`.
fn bug28030_workload(s: &mut dyn Session, names: [&str; 3], knots: (i32, f64), label: &str) {
    let curve = bezier_as_spline(knots.0, knots.1);
    let geometry: Curve = Handle::clone(&curve).cast();
    // Beyond the script: an earlier test of this case counted the whole knot workload.
    assert_eq!(curve.nb_knots(), knots.0 + 2, "{}: knots", names[0]);
    let [edge, face, third] = names;
    mkedge(s, edge, &geometry);
    prism(s, face, edge, [0., 0., 1.]);
    explode(s, face, Some(ShapeEnum::Edge));
    let mut chrono = Chrono::started();
    xdistef(s, third, face);
    chrono.stop();
    chrono.counter(label);
}

pub fn bug28030(s: &mut dyn Session) {
    println!("========");
    println!("OCC28030");
    println!("========");
    println!();
    bug28030_workload(s, ["e1", "p1", "p1_3"], (98, 0.01), "xdistef_1");
    bug28030_workload(s, ["e2", "p2", "p2_3"], (1000, 0.00098), "xdistef_2");
}

pub fn bug26542(s: &mut dyn Session) {
    println!("============");
    println!("0026452: Infinite loop in Edge/Edge intersection");
    println!("============");
    println!();
    restore(s, "bug26452_edges.brep", "e");
    explode(s, "e", None);
    let mut chrono = Chrono::started();
    bfuse(s, "r1", "e_1", "e_2");
    chrono.stop();
    chrono.counter("COUNTER1");
    tcopy(s, "e_1", "e1");
    tcopy(s, "e_2", "e2");
    let mut chrono = Chrono::started();
    bfuse(s, "r2", "e1", "e2");
    chrono.stop();
    chrono.counter("COUNTER2");
    checkprops(s, "r1", Prop::Length, Expected::Mass(78.3329));
    let expected = [NbCount::Vertices(3), NbCount::Edges(2)];
    checknbshapes(s, "r1", Counting::Located, &expected);
    checkprops_equal(s, "r2", "r1");
    checknbshapes_like(s, "r2", "r1");
}
