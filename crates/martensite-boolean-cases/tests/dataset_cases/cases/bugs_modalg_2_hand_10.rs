//! `bugs_modalg_2` bodies written by hand, second file.
use crate::support::{
    bugs_commands::{
        inspect::{distmini, dump_real},
        repair::halfspace_of,
    },
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    commands::{explode, make_box},
    construction::{edge, mkplane, vertex, wire},
    nbshapes::{Counting, NbCount, checknbshapes},
    one_step::bcut,
    properties::Prop,
    restore::restore,
    section_check::checksection,
    session::Session,
    two_step::TwoStepBoolean,
};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkg3d::top_abs::ShapeEnum;

/// The script's square, its half-space at `z_point`, the box at `z_box` cut by it, then the checks.
fn box_under_halfspace(s: &mut dyn Session, z_point: f64, z_box: f64, area: f64) {
    for (name, at) in [
        ("v1", [250., 250., 0.]),
        ("v2", [-250., 250., 0.]),
        ("v3", [-250., -250., 0.]),
        ("v4", [250., -250., 0.]),
    ] {
        vertex(s, name, at);
    }
    edge(s, "e1", "v1", "v2");
    edge(s, "e2", "v2", "v3");
    edge(s, "e3", "v3", "v4");
    edge(s, "e4", "v4", "v1");
    wire(s, "w1", &["e1", "e2", "e3", "e4"]);
    mkplane(s, "f", "w1");
    halfspace_of(s, "hs", "f", [0., 0., z_point]);
    make_box(s, "b", [0., 0., z_box], [150., 200., 200.]);
    bcut(s, "result", "b", "hs");
    checkprops(s, "result", Prop::Area, Expected::Mass(area));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug297_1(s: &mut dyn Session) {
    println!("========");
    println!("OCC297");
    println!("========");
    box_under_halfspace(s, -100., -30., 179000.);
}

pub fn bug297_2(s: &mut dyn Session) {
    println!("========");
    println!("OCC297");
    println!("========");
    box_under_halfspace(s, 100., -30., 81000.);
}

pub fn bug297_3(s: &mut dyn Session) {
    println!("========");
    println!("OCC297");
    println!("========");
    box_under_halfspace(s, -100., -80., 144000.);
}

pub fn bug297_4(s: &mut dyn Session) {
    println!("========");
    println!("OCC297");
    println!("========");
    box_under_halfspace(s, 100., -80., 116000.);
}

pub fn bug238_1(s: &mut dyn Session) {
    println!("TODO OCC12345 ALL: Faulty : Result shape is WRONG !!!");
    println!("TODO OCC12345 ALL: Error : The length of result shape is");
    println!("========================");
    println!(" OCC238 ");
    println!("========================");
    println!(" ");
    restore(s, "OCC238a.brep", "a");
    println!("a");
    restore(s, "case_2_edge1.brep", "b");
    println!("b");
    let mut two_step = TwoStepBoolean::default();
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    explode(s, "result", Some(ShapeEnum::Vertex));
    explode(s, "a", Some(ShapeEnum::Edge));
    let distance = distmini(s, "di", "result_1", "a_3");
    dump_real("di_val", distance);
    if distance > 1.0e-5 {
        println!("Faulty : Result shape is WRONG !!!");
    } else {
        println!(" Result shape is COMPLETE");
    }
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Vertices(14), NbCount::Edges(7)],
    );
    checkprops(s, "result", Prop::Length, Expected::Mass(0.));
    checkshape_named(s, "result");
    checksection(s, "result");
    checkview(s, &CheckView::of("result"));
}
