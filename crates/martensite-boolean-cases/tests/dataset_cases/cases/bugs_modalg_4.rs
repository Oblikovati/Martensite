//! `bugs_modalg_4`: each statement is one line of its script.
use crate::support::{
    checkprops::{Expected, checkprops},
    checkshape::{checkshape_named, checkshape_top},
    checkview::{CheckView, checkview},
    commands::{make_box, prism, psphere, trotate, ttranslate},
    construction::tcopy,
    nbshapes::{Counting, NbCount, checknbshapes},
    one_step::{bcommon, bcut},
    properties::Prop,
    restore::restore,
    session::Session,
    two_step::TwoStepBoolean,
    volume_commands::compound,
};
use martensite_tkbo::bop_algo::Operation;

pub use super::bugs_modalg_4_hand_2::*;

pub fn bug693(s: &mut dyn Session) {
    println!("================");
    println!("OCC693");
    println!("================");
    println!();
    make_box(s, "h1", [234., 52., 0.], [41., 98., 40.]);
    make_box(s, "v1", [241., 136., 20.], [31., 31., 15.]);
    make_box(s, "h3", [215., 120., 20.], [78., 15., 15.]);
    compound(s, &["h1"], "c1");
    compound(s, &["v1"], "c2");
    tcopy(s, "c1", "cc1");
    tcopy(s, "c2", "cc2");
    bcommon(s, "int1", "cc1", "cc2");
    checkshape_top(s, "int1");
    tcopy(s, "c1", "cc1");
    tcopy(s, "c2", "cc2");
    bcut(s, "cut1", "cc1", "cc2");
    checkshape_top(s, "cut1");
    tcopy(s, "c1", "cc1");
    tcopy(s, "c2", "cc2");
    bcut(s, "cut2", "cc2", "cc1");
    checkshape_top(s, "cut2");
    compound(s, &["int1", "cut1", "cut2"], "comp3");
    checkshape_top(s, "comp3");
    tcopy(s, "comp3", "ccomp3");
    tcopy(s, "h3", "ch3");
    bcommon(s, "int2", "ccomp3", "ch3");
    checkshape_top(s, "int2");
    tcopy(s, "comp3", "ccomp3");
    tcopy(s, "h3", "ch3");
    bcut(s, "cut3", "ccomp3", "ch3");
    checkshape_top(s, "cut3");
    tcopy(s, "comp3", "ccomp3");
    tcopy(s, "h3", "ch3");
    bcut(s, "cut4", "ch3", "ccomp3");
    checkshape_top(s, "cut4");
    compound(s, &["int2", "cut3", "cut4"], "result");
    checkshape_top(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(33196.));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug7626_1(s: &mut dyn Session) {
    println!("============");
    println!("OCC7626");
    println!("============");
    println!();
    psphere(s, "s", 10.);
    make_box(s, "b", [10., 0., 0.], [20., 20., 20.]);
    trotate(s, "b", [10., 0., 0.], [0., 0., 1.], 45.);
    bcut(s, "result", "s", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(1271.7));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(4),
            NbCount::Edges(7),
            NbCount::Wires(4),
            NbCount::Faces(3),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(21),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug7626_2(s: &mut dyn Session) {
    println!("============");
    println!("OCC7626");
    println!("============");
    println!();
    psphere(s, "s", 10.);
    make_box(s, "b", [10., 0., 0.], [20., 20., 20.]);
    trotate(s, "b", [10., 0., 0.], [0., 0., 1.], 45.);
    bcommon(s, "result", "s", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(199.095));
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(2),
            NbCount::Edges(3),
            NbCount::Wires(3),
            NbCount::Faces(3),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(14),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug763(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("========");
    println!("OCC763");
    println!("========");
    println!();
    restore(s, "OCC763_c1.brep", "c1");
    checkshape_named(s, "c1");
    restore(s, "OCC763_c2.brep", "c2");
    checkshape_named(s, "c2");
    two_step.bop(s, "c1", "c2");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(23189.4));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug779(s: &mut dyn Session) {
    println!("========");
    println!("OCC779");
    println!("========");
    println!();
    restore(s, "OCC779_s1.brep", "s1");
    checkshape_named(s, "s1");
    restore(s, "OCC779_s2.brep", "s2");
    checkshape_named(s, "s2");
    ttranslate(s, "s1", [0., 0., -30.]);
    prism(s, "s1p", "s1", [0., 0., 70.]);
    checkshape_named(s, "s1p");
    bcommon(s, "result", "s1p", "s2");
    checkprops(s, "result", Prop::Area, Expected::Mass(64191.2));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}
