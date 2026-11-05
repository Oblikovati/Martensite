//! `bugs_modalg_1` bodies written by hand: the scripts with variables and read-back values.
use crate::support::{
    bugs_commands::{
        geometry::{circle_framed, mkcurve, mkedge_range},
        inspect::{checksection_count, computetolerance_put},
        repair::halfspace_of,
    },
    checkprops::{Expected, checkprops},
    checkshape::{checkshape, checkshape_named},
    checkview::{CheckView, checkview},
    commands::{bound_shape, compound, explode, make_box, prism},
    construction::tcopy,
    named_shapes::renamevar,
    nbshapes::{Counting, NbCount, checknbshapes, nbshapes},
    one_step::{bcommon, bcut, bfuse, bsection},
    properties::{Prop, props},
    restore::restore,
    section_check::checksection,
    session::Session,
    surface_commands::{mkface, plane},
};
use martensite_tkg3d::top_abs::ShapeEnum;

fn view_result(s: &mut dyn Session) {
    checkview(s, &CheckView::of("result"));
}

/// The script's guard on both inputs, then `operation` on them when both are valid.
fn on_valid_inputs(s: &mut dyn Session, operation: fn(&mut dyn Session, &str, &str, &str)) {
    restore(s, "buc60409a.brep", "c");
    restore(s, "buc60409b.brep", "s");
    if checkshape(&bound_shape(s, "c")).text.contains("Faulty") {
        println!(
            "Faulty BUC60409 (shape 1): Source shape is invalid. It was detected by Checkshape command"
        );
        println!("Body of the script was NOT executed");
        renamevar(s, "c", "result");
        return;
    }
    println!("BUC60409 OK (shape 1): Source shape is valid");
    if checkshape(&bound_shape(s, "s")).text.contains("Faulty") {
        println!(
            "Faulty BUC60409 (shape 2): Source shape is invalid. It was detected by Checkshape command"
        );
        println!("Body of the script was NOT executed");
        renamevar(s, "s", "result");
        return;
    }
    println!("BUC60409 OK (shape 2): Source shape is valid");
    operation(s, "result", "c", "s");
}

fn tail(s: &mut dyn Session, area: f64) {
    checkprops(s, "result", Prop::Area, Expected::Mass(area));
    checkshape_named(s, "result");
    view_result(s);
}

pub fn buc60409_1(s: &mut dyn Session) {
    println!("==============");
    println!("   BUC60409   ");
    println!("     FUSE     ");
    println!("==============");
    on_valid_inputs(s, bfuse);
    tail(s, 39.0894);
}

pub fn buc60409_2(s: &mut dyn Session) {
    println!("=============");
    println!("   BUC60409  ");
    println!("     CUT     ");
    println!("=============");
    on_valid_inputs(s, bcut);
    tail(s, 31.8476);
}

pub fn buc60409_3(s: &mut dyn Session) {
    println!("=================");
    println!("     BUC60409    ");
    println!("      COMMON     ");
    println!("=================");
    on_valid_inputs(s, bcommon);
    tail(s, 4.89279);
}

pub fn buc60532_2(s: &mut dyn Session) {
    println!("==========");
    println!("BUC60532");
    println!("==========");
    restore(s, "buc60532a.brep", "p");
    checkshape_named(s, "p");
    restore(s, "buc60532b.brep", "t1");
    checkshape_named(s, "t1");
    restore(s, "buc60532c.brep", "t2");
    checkshape_named(s, "t2");
    halfspace_of(s, "h1", "t1", [20., 20., 0.]);
    halfspace_of(s, "h2", "t2", [20., 20., 0.]);
    bcut(s, "res1", "p", "h1");
    explode(s, "res1", Some(ShapeEnum::Edge));
    for index in 1..=5 {
        mkcurve(s, &format!("res1_{index}"));
    }
    checkshape_named(s, "res1");
    computetolerance_put(s, "res1");
    bcut(s, "res2", "res1", "h2");
    tcopy(s, "res2", "res");
    checkshape_named(s, "res2");
    explode(s, "res2", Some(ShapeEnum::Edge));
    for index in 1..=4 {
        mkcurve(s, &format!("res2_{index}"));
    }
    explode(s, "res2", Some(ShapeEnum::Face));
    computetolerance_put(s, "res2");
    renamevar(s, "res2", "result");
    tail(s, 78.8852);
}

pub fn buc60668(s: &mut dyn Session) {
    println!("====================");
    println!("BUC60668");
    println!("Three boxes must appear on picture");
    println!("====================");
    make_box(s, "b", [100., 0., 0.], [10., 10., 10.]);
    make_box(s, "b1", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "b2", [50., 50., 50.], [10., 10., 10.]);
    make_box(s, "a", [-10., -10., -10.], [200., 200., 200.]);
    compound(s, &["b", "b1", "b2"], "c");
    let whole = props(Prop::Volume, &bound_shape(s, "c"), None);
    bcommon(s, "result", "c", "a");
    let common = props(Prop::Volume, &bound_shape(s, "result"), None);
    if whole != common {
        println!(" Faulty : Common is incorrect");
    } else {
        println!(" BUC60668 OK : Common was made properly");
    }
    tail(s, 1800.);
}

/// `if {[lindex [checksection result] 4] != 2} {puts ...}`: the fifth word is the count of vertices.
fn two_alone_vertices(s: &mut dyn Session) {
    if checksection_count(s, "result") != 2 {
        println!("Error: section is incomplete");
    }
}

pub fn buc60682(s: &mut dyn Session) {
    println!("==================");
    println!("BUC60682");
    println!("==================");
    restore(s, "buc60682a.brep", "shell1");
    restore(s, "buc60682b.brep", "shell2");
    bsection(s, "result", "shell1", "shell2");
    checkshape_named(s, "result");
    two_alone_vertices(s);
    checkprops(s, "result", Prop::Length, Expected::Mass(5.76248));
    checknbshapes(s, "result", Counting::Located, &[NbCount::Edges(1)]);
    view_result(s);
}

pub fn buc60744(s: &mut dyn Session) {
    println!("========================");
    println!("BUC60744");
    println!("========================");
    println!();
    restore(s, "buc60744a.brep", "a");
    restore(s, "buc60744b.brep", "b");
    bsection(s, "result", "a", "b");
    checkshape_named(s, "result");
    two_alone_vertices(s);
    checkprops(s, "result", Prop::Length, Expected::Mass(228.386));
    view_result(s);
}

pub fn buc60708(s: &mut dyn Session) {
    println!("===========");
    println!("BUC60708");
    println!("===========");
    restore(s, "buc60708a.brep", "a");
    checkshape_named(s, "a");
    restore(s, "buc60708b.brep", "b");
    checkshape_named(s, "b");
    bcommon(s, "result", "a", "b");
    let faces = explode(s, "result", Some(ShapeEnum::Face));
    if faces.last().map(String::as_str) != Some("result_4") {
        println!(" Faulty :Common operation was made  wrongly");
    } else {
        println!(" Common operation was made successfully");
    }
    tail(s, 13.4871);
}

pub fn buc60788_1(s: &mut dyn Session) {
    println!("============");
    println!("BUC60788");
    println!("case 1");
    println!("============");
    restore(s, "buc60788a.brep", "S1");
    checkshape_named(s, "S1");
    restore(s, "buc60788b.brep", "S2");
    checkshape_named(s, "S2");
    bsection(s, "result", "S1", "S2");
    let counts = nbshapes(s, "result", Counting::Located);
    if counts[0] != 4 && counts[4] != 3 {
        println!("Faulty :  result (res) is incorrect");
    } else {
        println!("BUC60788 - OK");
    }
    checkprops(s, "result", Prop::Length, Expected::Mass(750.));
    checkshape_named(s, "result");
    checksection(s, "result");
    view_result(s);
}

/// The script's reading of the counts, then its last checks; `verb` names the operation.
fn counts_nonzero_tail(s: &mut dyn Session, verb: &str, length: f64) {
    let counts = nbshapes(s, "result", Counting::Located);
    if counts[0] != 0 && counts[1] != 0 {
        println!(" BUC60789 OK: {verb} operation was made properly");
    } else {
        println!("Faulty BUC60789: {verb} operation was made wrongly");
    }
    checkprops(s, "result", Prop::Length, Expected::Mass(length));
    checkshape_named(s, "result");
    checksection(s, "result");
    view_result(s);
}

pub fn buc60789_1(s: &mut dyn Session) {
    println!("========");
    println!("BUC60789");
    println!("========");
    println!();
    restore(s, "buc60787b.brep", "S1");
    checkshape_named(s, "S1");
    restore(s, "buc60789b.brep", "S2");
    checkshape_named(s, "S2");
    bcommon(s, "result", "S1", "S2");
    counts_nonzero_tail(s, "COMMON", 78.3594);
}

pub fn buc60789_2(s: &mut dyn Session) {
    println!("========");
    println!("BUC60789");
    println!("========");
    println!();
    restore(s, "buc60787b.brep", "S1");
    checkshape_named(s, "S1");
    restore(s, "buc60789d.brep", "S2");
    checkshape_named(s, "S2");
    bcut(s, "result", "S2", "S1");
    counts_nonzero_tail(s, "CUT", 288.856);
}

/// The half circle's prism and its section with the plane through `at`.
fn prism_section(s: &mut dyn Session, at: [f64; 3]) {
    let half = std::f64::consts::PI / 2.;
    let circle = circle_framed([0., 0., 0.], [0., 0., 1.], [1., 0., 0.], 20.);
    mkedge_range(s, "f", &circle, -half, half);
    prism(s, "f", "f", [50., 0., 30.]);
    let surface = plane(at, [1., 0., 4.]);
    mkface(s, "pl", &surface);
    bsection(s, "result", "f", "pl");
}

fn all_nine(c: [usize; 9]) -> [NbCount; 9] {
    [
        NbCount::Vertices(c[0]),
        NbCount::Edges(c[1]),
        NbCount::Wires(c[2]),
        NbCount::Faces(c[3]),
        NbCount::Shells(c[4]),
        NbCount::Solids(c[5]),
        NbCount::CompSolids(c[6]),
        NbCount::Compounds(c[7]),
        NbCount::Shapes(c[8]),
    ]
}

fn section_tail(s: &mut dyn Session, counts: [usize; 9], length: f64) {
    checknbshapes(s, "result", Counting::Located, &all_nine(counts));
    checkprops(s, "result", Prop::Length, Expected::Mass(length));
    checkshape_named(s, "result");
    checksection(s, "result");
    view_result(s);
}

pub fn bug17194_1(s: &mut dyn Session) {
    println!("============");
    println!("OCC17194");
    println!("============");
    println!();
    prism_section(s, [10., 0., 0.]);
    section_tail(s, [4, 2, 0, 0, 0, 0, 0, 1, 7], 15.8064);
}

pub fn bug17194_2(s: &mut dyn Session) {
    println!("============");
    println!("OCC17194");
    println!("============");
    println!();
    prism_section(s, [10., 0., 10.]);
    section_tail(s, [2, 1, 0, 0, 0, 0, 0, 1, 4], 54.6122);
}
