//! `bugs_modalg_2`: linear scripts in the commands of `support::bugs_commands`; each statement is one
//! line of its script.
use crate::support::{
    beyond_script::{all_counts, printed_mass},
    bugs_commands::beyond::{alone_count, child_kinds, is_oriented},
    bugs_commands::repair::breducetolerance,
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    commands::explode,
    nbshapes::{Counting, NbCount, checknbshapes},
    one_step::{bcommon, bcut, bfuse},
    properties::Prop,
    restore::restore,
    section_check::checksection,
    session::Session,
    two_step::TwoStepBoolean,
};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkg3d::top_abs::ShapeEnum;

pub fn bug239_1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("TODO OCC12345 ALL: Error :  is WRONG because number of");
    println!("TODO OCC12345 ALL: Error : The length of result shape is");
    println!("========================");
    println!(" OCC239 ");
    println!("========================");
    println!(" ");
    restore(s, "OCC238a.brep", "a");
    restore(s, "case_2_wire1.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Vertices(18), NbCount::Edges(10)],
    );
    checkprops(s, "result", Prop::Length, Expected::Mass(0.));
    checkshape_named(s, "result");
    checksection(s, "result");
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    is_oriented(s, "a", ShapeEnum::Solid, Orientation::Forward);
    is_oriented(s, "b", ShapeEnum::Wire, Orientation::Reversed);
    all_counts(s, "result", [18, 11, 7, 0, 0, 0, 0, 1, 37]);
    printed_mass(s, "result", Prop::Length, "742.56");
    alone_count(s, "result", 14);
}

pub fn bug239_2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("========================");
    println!(" OCC239 ");
    println!("========================");
    println!(" ");
    restore(s, "OCC238a.brep", "a");
    restore(s, "case_2_wire1.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Vertices(18), NbCount::Edges(11)],
    );
    checkprops(s, "result", Prop::Length, Expected::Mass(2434.21));
    checkshape_named(s, "result");
    checksection(s, "result");
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    is_oriented(s, "a", ShapeEnum::Solid, Orientation::Forward);
    is_oriented(s, "b", ShapeEnum::Wire, Orientation::Reversed);
}

pub fn bug244(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("========================");
    println!(" OCC244 ");
    println!("========================");
    println!(" ");
    restore(s, "case_5_solid.brep", "a");
    restore(s, "case_4_wire3.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Vertices(12), NbCount::Edges(11)],
    );
    checkprops(s, "result", Prop::Length, Expected::Mass(1808.02));
    checkshape_named(s, "result");
    checksection(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug245_1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("========================");
    println!(" OCC245 ");
    println!("========================");
    println!(" ");
    restore(s, "case_5_shell.brep", "a");
    restore(s, "case_4_wire3.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Vertices(12), NbCount::Edges(11)],
    );
    checkprops(s, "result", Prop::Length, Expected::Mass(1808.02));
    checkshape_named(s, "result");
    checksection(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug245_2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("========================");
    println!(" OCC245 ");
    println!("========================");
    println!(" ");
    restore(s, "case_5_shell.brep", "a");
    restore(s, "case_4_wire3.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Vertices(3), NbCount::Edges(2)],
    );
    checkprops(s, "result", Prop::Length, Expected::Mass(272.04));
    checkshape_named(s, "result");
    checksection(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug249_1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("TODO OCC12345 ALL: Error : The command is not valid. The length is");
    println!("TODO OCC12345 ALL: Error :  is WRONG because number of");
    println!("========================");
    println!(" OCC249 ");
    println!("========================");
    println!(" ");
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_edge3.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
    checkprops(s, "result", Prop::Length, Expected::Mass(0.));
    checkshape_named(s, "result");
    checksection(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug249_2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("TODO OCC12345 ALL: Error : The length of result shape is");
    println!("TODO OCC12345 ALL: Error :  is WRONG because number of");
    println!("========================");
    println!(" OCC249 ");
    println!("========================");
    println!(" ");
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_edge3.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Vertices(4), NbCount::Edges(2)],
    );
    checkprops(s, "result", Prop::Length, Expected::Mass(0.));
    checkshape_named(s, "result");
    checksection(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug249_3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("TODO OCC12345 ALL: Error : The length of result shape is");
    println!("TODO OCC12345 ALL: Error :  is WRONG because number of");
    println!("========================");
    println!(" OCC249 ");
    println!("========================");
    println!(" ");
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_edge3.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Vertices(4), NbCount::Edges(2)],
    );
    checkprops(s, "result", Prop::Length, Expected::Mass(0.));
    checkshape_named(s, "result");
    checksection(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug249_4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("TODO OCC12345 ALL: Error : The length of result shape is");
    println!("TODO OCC12345 ALL: Error :  is WRONG because number of");
    println!("========================");
    println!(" OCC249 ");
    println!("========================");
    println!(" ");
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_edge3.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Vertices(6), NbCount::Edges(5)],
    );
    checkprops(s, "result", Prop::Length, Expected::Mass(0.));
    checkshape_named(s, "result");
    checksection(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug250_1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("TODO OCC12345 ALL: Error : The command is not valid. The length is");
    println!("TODO OCC12345 ALL: Error :  is WRONG because number of");
    println!("========================");
    println!(" OCC250 ");
    println!("========================");
    println!(" ");
    restore(s, "case_9_wire_a.brep", "a");
    restore(s, "case_9_wire2.brep", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Vertices(2), NbCount::Edges(1)],
    );
    checkprops(s, "result", Prop::Length, Expected::Mass(0.));
    checkshape_named(s, "result");
    checksection(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug334(s: &mut dyn Session) {
    println!("============");
    println!("BUC60928");
    println!("OCC334");
    println!("============");
    restore(s, "BUC60928.brep", "a");
    explode(s, "a", None);
    checkshape_named(s, "a_1");
    checkshape_named(s, "a_2");
    bcut(s, "result", "a_2", "a_1");
    checkprops(s, "result", Prop::Area, Expected::Mass(358258.));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    child_kinds(s, "a", &[ShapeEnum::Solid, ShapeEnum::Shell]);
}

pub fn bug338(s: &mut dyn Session) {
    println!("========");
    println!("BUC60996");
    println!("OCC338");
    println!("========");
    println!();
    restore(s, "buc60996.brep", "b");
    explode(s, "b", None);
    checkshape_named(s, "b_1");
    checkshape_named(s, "b_2");
    bfuse(s, "f1", "b_1", "b_2");
    checkshape_named(s, "f1");
    bfuse(s, "result", "b_2", "b_1");
    checkprops(s, "result", Prop::Area, Expected::Mass(73561.9));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    child_kinds(s, "b", &[ShapeEnum::Solid, ShapeEnum::Solid]);
}

pub fn bug410_4(s: &mut dyn Session) {
    println!("========================");
    println!(" OCC410 ");
    println!("(case 4)");
    println!("========================");
    println!();
    restore(s, "f7", "b1");
    checkshape_named(s, "b1");
    restore(s, "f3", "b2");
    checkshape_named(s, "b2");
    bcut(s, "result", "b2", "b1");
    println!(" OCC410 OK: function CUT works properly");
    checkprops(s, "result", Prop::Area, Expected::Mass(93.6289));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug485(s: &mut dyn Session) {
    println!("=========");
    println!(" OCC485 ");
    println!("=========");
    println!();
    restore(s, "OCC485a.brep", "a_1");
    checkshape_named(s, "a_1");
    restore(s, "OCC485a.brep", "a_2");
    checkshape_named(s, "a_2");
    bfuse(s, "result", "a_1", "a_2");
    println!("OCC485 OK: function FUSE works without Draw hangs up ");
    checkprops(s, "result", Prop::Area, Expected::Mass(9291.88));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug488(s: &mut dyn Session) {
    println!("=========");
    println!(" OCC488 ");
    println!("=========");
    println!();
    restore(s, "OCC488a.brep", "a_1");
    checkshape_named(s, "a_1");
    restore(s, "OCC488b.brep", "a_2");
    checkshape_named(s, "a_2");
    bcommon(s, "result", "a_1", "a_2");
    println!("OCC488 OK: function COMMON works properly ");
    checkprops(s, "result", Prop::Area, Expected::Mass(92466.8));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug490(s: &mut dyn Session) {
    println!("=========");
    println!(" OCC490 ");
    println!("=========");
    println!();
    restore(s, "OCC490a.brep", "a_1");
    checkshape_named(s, "a_1");
    restore(s, "OCC490b.brep", "a_2");
    checkshape_named(s, "a_2");
    bfuse(s, "result", "a_1", "a_2");
    println!("OCC490 : function FUSE works without hangs up ");
    checkprops(s, "result", Prop::Area, Expected::Mass(2744.08));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug497_1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("=========");
    println!(" OCC497 ");
    println!("(case 1)");
    println!("=========");
    println!();
    restore(s, "OCC497a.brep", "a_1");
    checkshape_named(s, "a_1");
    restore(s, "OCC497b.brep", "a_2");
    checkshape_named(s, "a_2");
    explode(s, "a_2", Some(ShapeEnum::Solid));
    two_step.bop(s, "a_2_1", "a_2_2");
    two_step.result(s, "r1", Operation::Fuse);
    bcut(s, "result", "a_1", "r1");
    println!("OCC497 : function CUT works without hangs up ");
    checkprops(s, "result", Prop::Area, Expected::Mass(646.969));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug497_2(s: &mut dyn Session) {
    println!("=========");
    println!(" OCC497 ");
    println!("(case 2)");
    println!("=========");
    println!();
    restore(s, "OCC497c.brep", "a_1");
    checkshape_named(s, "a_1");
    restore(s, "OCC497d.brep", "a_2");
    checkshape_named(s, "a_2");
    bcut(s, "result", "a_1", "a_2");
    println!("OCC497 : function CUT works without hangs up ");
    checkprops(s, "result", Prop::Area, Expected::Mass(2225.65));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug497_3(s: &mut dyn Session) {
    println!("=========");
    println!(" OCC497 ");
    println!("(case 3)");
    println!("=========");
    println!();
    restore(s, "OCC497e.brep", "a_1");
    checkshape_named(s, "a_1");
    restore(s, "OCC497f.brep", "a_2");
    checkshape_named(s, "a_2");
    breducetolerance(s, "a_2");
    bcut(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Area, Expected::Mass(2392.19));
    checkprops(s, "result", Prop::Volume, Expected::Mass(7520.02));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Shells(1), NbCount::Solids(1)],
    );
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug497_5(s: &mut dyn Session) {
    println!("=========");
    println!(" OCC497 ");
    println!("(case 5)");
    println!("=========");
    println!();
    restore(s, "OCC497i.brep", "a_1");
    checkshape_named(s, "a_1");
    restore(s, "OCC497j.brep", "a_2");
    checkshape_named(s, "a_2");
    bfuse(s, "result", "a_1", "a_2");
    println!("OCC497 : function FUSE works without hangs up ");
    checkprops(s, "result", Prop::Area, Expected::Mass(3233.5));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}

pub fn bug528(s: &mut dyn Session) {
    println!("========================");
    println!("OCC528");
    println!("========================");
    println!();
    restore(s, "OCC528_1.brep", "c1");
    checkshape_named(s, "c1");
    restore(s, "OCC528_2.brep", "c2");
    checkshape_named(s, "c2");
    bcut(s, "result", "c1", "c2");
    println!("OCC528 OK:function BCUT works properly");
    checkprops(s, "result", Prop::Area, Expected::Mass(124.092));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}
