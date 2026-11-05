//! `bugs_moddata_1`: each statement is one line of its script.
use crate::support::{
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    commands::{make_box, psphere},
    nbshapes::{Counting, NbCount, checknbshapes},
    one_step::{bcommon, bcut, bfuse},
    properties::Prop,
    restore::restore,
    session::Session,
    two_step::TwoStepBoolean,
};
use martensite_tkbo::bop_algo::Operation;

pub use super::bugs_moddata_1_hand_2::*;
pub use super::bugs_moddata_1_hand_9::*;
pub use super::bugs_moddata_1_hand_10::*;

pub fn bug10604_1(s: &mut dyn Session) {
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
    two_step.result(s, "result", Operation::Common);
    checkshape_named(s, "result");
    println!("Finish boolean operation ...");
    checkprops(s, "result", Prop::Area, Expected::Mass(585.072));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(6),
            NbCount::Edges(12),
            NbCount::Wires(5),
            NbCount::Faces(5),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(31),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug10604_2(s: &mut dyn Session) {
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
    two_step.result(s, "result", Operation::Fuse);
    checkshape_named(s, "result");
    println!("Finish boolean operation ...");
    checkprops(s, "result", Prop::Area, Expected::Mass(808.713));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(8),
            NbCount::Edges(15),
            NbCount::Wires(8),
            NbCount::Faces(7),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(41),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug10604_3(s: &mut dyn Session) {
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
    two_step.result(s, "result", Operation::Cut);
    checkshape_named(s, "result");
    println!("Finish boolean operation ...");
    checkprops(s, "result", Prop::Area, Expected::Mass(893.394));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(7),
            NbCount::Edges(15),
            NbCount::Wires(6),
            NbCount::Faces(5),
            NbCount::Shells(1),
            NbCount::Solids(1),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(36),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug10604_4(s: &mut dyn Session) {
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
    two_step.result(s, "result", Operation::Cut21);
    checkshape_named(s, "result");
    println!("Finish boolean operation ...");
    checkprops(s, "result", Prop::Area, Expected::Mass(500.39));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(7),
            NbCount::Edges(12),
            NbCount::Wires(7),
            NbCount::Faces(7),
            NbCount::Shells(2),
            NbCount::Solids(2),
            NbCount::CompSolids(0),
            NbCount::Compounds(1),
            NbCount::Shapes(38),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug183_1(s: &mut dyn Session) {
    println!("========================");
    println!(" OCC183 ");
    println!("(case 1)");
    println!("========================");
    println!();
    restore(s, "f1", "b1");
    checkshape_named(s, "b1");
    restore(s, "f4", "b2");
    checkshape_named(s, "b2");
    bcommon(s, "result", "b1", "b2");
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(315.159));
    checkview(s, &CheckView::of("result"));
}

pub fn bug183_2(s: &mut dyn Session) {
    println!("========================");
    println!(" OCC183 ");
    println!("(case 2)");
    println!("========================");
    println!();
    restore(s, "so1", "b1");
    checkshape_named(s, "b1");
    restore(s, "so4", "b2");
    checkshape_named(s, "b2");
    bfuse(s, "result", "b1", "b2");
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(942.478));
    checkview(s, &CheckView::of("result"));
}

pub fn bug183_3(s: &mut dyn Session) {
    println!("========================");
    println!(" OCC183 ");
    println!("(case 3)");
    println!("========================");
    println!();
    restore(s, "so1", "b1");
    checkshape_named(s, "b1");
    restore(s, "so4", "b2");
    checkshape_named(s, "b2");
    bcut(s, "result", "b1", "b2");
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(863.938));
    checkview(s, &CheckView::of("result"));
}

pub fn bug183_4(s: &mut dyn Session) {
    println!("========================");
    println!(" OCC183 ");
    println!("(case 4)");
    println!("========================");
    println!();
    restore(s, "so1", "b1");
    checkshape_named(s, "b1");
    restore(s, "so2", "b2");
    checkshape_named(s, "b2");
    bcommon(s, "result", "b1", "b2");
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(392.699));
    checkview(s, &CheckView::of("result"));
}

pub fn bug183_5(s: &mut dyn Session) {
    println!("========================");
    println!(" OCC183 ");
    println!("(case 5)");
    println!("========================");
    println!();
    restore(s, "so4", "b1");
    checkshape_named(s, "b1");
    restore(s, "so2", "b2");
    checkshape_named(s, "b2");
    bcommon(s, "result", "b1", "b2");
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(198.863));
    checkview(s, &CheckView::of("result"));
}

pub fn bug217(s: &mut dyn Session) {
    println!("========");
    println!("OCC217");
    println!("========");
    make_box(s, "b", [-5., -5., -5.], [10., 10., 10.]);
    psphere(s, "s", 6.);
    bfuse(s, "result", "s", "b");
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(618.85));
    checkview(s, &CheckView::of("result"));
}
