//! `bugs_moddata_1`: linear scripts in the commands of `support::bugs_commands`; each statement is one
//! line of its script.
use crate::support::{
    bugs_commands::beyond::alone_count,
    bugs_commands::repair::{fsameparameter, updatetolerance},
    checkprops::{Expected, checkprops},
    checkshape::{checkshape_named, checkshape_top},
    checkview::{CheckView, checkview},
    nbshapes::{Counting, NbCount, checknbshapes},
    one_step::bfuse,
    properties::Prop,
    restore::restore,
    section_check::checksection,
    session::Session,
    two_step::TwoStepBoolean,
};
use martensite_tkbo::bop_algo::Operation;

pub fn bug10604_5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("============");
    println!("OCC10604");
    println!("============");
    println!();
    println!("Load first shape ...");
    restore(s, "OCC10604-1.brep", "b1");
    println!("Load second shape ...");
    restore(s, "OCC10604-2.brep", "b2");
    println!("Prepare boolean operation ...");
    two_step.bop(s, "b1", "b2");
    println!("Start boolean operation ...");
    two_step.section(s, "result");
    checkshape_named(s, "result");
    println!("Finish boolean operation ...");
    checkprops(s, "result", Prop::Length, Expected::Mass(88.9912));
    checksection(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(6),
            NbCount::Edges(6),
            NbCount::Wires(0),
            NbCount::Faces(0),
            NbCount::Shells(0),
            NbCount::Solids(0),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(13),
        ],
    );
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    alone_count(s, "result", 0);
}

pub fn bug183_6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("========================");
    println!(" OCC183 ");
    println!(" CTS16184");
    println!("(case 6)");
    println!("========================");
    println!();
    restore(s, "CTO900_cts16184a.rle", "a");
    checkshape_named(s, "a");
    restore(s, "cts16184b.brep", "b");
    checkshape_named(s, "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkshape_top(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(0.0892544));
    checkview(s, &CheckView::of("result"));
}

pub fn bug227_1(s: &mut dyn Session) {
    println!("========");
    println!("OCC227");
    println!("(case 1)");
    println!("========");
    restore(s, "OCC227a.brep", "a");
    fsameparameter(s, "a");
    updatetolerance(s, "a");
    checkshape_named(s, "a");
    restore(s, "OCC227b.brep", "b");
    fsameparameter(s, "b");
    updatetolerance(s, "b");
    checkshape_named(s, "b");
    bfuse(s, "result", "a", "b");
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(1583.92));
    checkview(s, &CheckView::of("result"));
}

pub fn bug227_2(s: &mut dyn Session) {
    println!("========");
    println!("OCC227");
    println!("(case 2)");
    println!("========");
    restore(s, "OCC227a.brep", "a");
    fsameparameter(s, "a");
    updatetolerance(s, "a");
    checkshape_named(s, "a");
    restore(s, "OCC227b.brep", "b");
    fsameparameter(s, "b");
    updatetolerance(s, "b");
    checkshape_named(s, "b");
    bfuse(s, "result", "b", "a");
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(1583.92));
    checkview(s, &CheckView::of("result"));
}
