//! The `bopcut_complex` case bodies, part 2; each statement is one line of its script.
use super::*;

pub fn J1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a138", "a");
    restore(s, "b140", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(15079.6));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn J2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b142", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(5124.72));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn J3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "a152", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(22854.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn J4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "a158", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(13527.4));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn J5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a148", "a");
    restore(s, "b148", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(23561.9));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn J6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "a160", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(13036.5));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn J7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a152", "a");
    restore(s, "b118", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(5890.6));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn J8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a152", "a");
    restore(s, "a157", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(10053.1));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn J9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a157", "a");
    restore(s, "a152", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(4869.47));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn K1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a158", "a");
    restore(s, "b148", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(23561.8));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn K2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a160", "a");
    restore(s, "b160", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(31415.7));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn K9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b193", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(10000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn L1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "a206", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(19007.8));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn L2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b199", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(18083.6));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn L3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a206", "a");
    restore(s, "b199", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(15190.4));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn L4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a208", "a");
    restore(s, "b208", "b");
    two_step.bop(s, "b", "a");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(5142.93));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b", "a"], Prop::Area),
    );
}

pub fn L6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a349", "a");
    restore(s, "b349", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(79.0703));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn L9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "CTO904_pro13075a.rle", "a");
    restore(s, "CTO904_pro13075b.rle", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(3053.84));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn M3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "CTO904_cts20339-part.rle", "a");
    restore(s, "cts20339-tool.rle", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(407561.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn M4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "cts20975a.rle", "a");
    restore(s, "CTO904_cts20975b.rle", "b");
    bopaddpcs(s, "a");
    btolx(s, "a");
    bopaddpcs(s, "b");
    btolx(s, "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(5609.09));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a", Prop::Area, "5552");
    kind_is(s, "a", ShapeEnum::Solid);
    printed_mass(s, "b", Prop::Area, "3915.96");
    kind_is(s, "b", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn M6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "Jyls3", "a");
    explode(s, "a", Some(ShapeEnum::Shell));
    copy(s, "a_1", "a");
    restore(s, "Jyls4", "b");
    explode(s, "b", Some(ShapeEnum::Shell));
    copy(s, "b_1", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(11372.4));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn M7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "CTO909_bad_cut_outil.brep", "a");
    restore(s, "CTO909_bad_cut_part.brep", "b");
    two_step.bop(s, "b", "a");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(163.988));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b", "a"], Prop::Area),
    );
}

pub fn N3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "m2_o", "m2_o");
    restore(s, "m2_t", "m2_t");
    two_step.bop(s, "m2_o", "m2_t");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(266.995));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["m2_o", "m2_t"], Prop::Area),
    );
}

pub fn N5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "c", [0., 0., 0.], [4., 4., 4.]);
    ttranslate(s, "c", [3., 3., 6.]);
    two_step.bop(s, "b", "c");
    two_step.result(s, "rr", Operation::Fuse);
    restore(s, "A1_rr_7.brep", "A1_rr_7");
    prism(s, "p1", "A1_rr_7", [0., 0., 4.]);
    two_step.bop(s, "rr", "p1");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(600.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["rr", "p1"], Prop::Area),
    );
}

pub fn N6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "c", [0., 0., 0.], [4., 4., 4.]);
    ttranslate(s, "c", [3., 3., 6.]);
    bfuse(s, "rr", "b", "c");
    restore(s, "A1_rr_7.brep", "A1_rr_7");
    prism(s, "p1", "A1_rr_7", [0., 0., -4.]);
    explode(s, "rr", Some(ShapeEnum::Solid));
    two_step.bop(s, "rr_1", "p1");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(664.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["rr_1", "p1"], Prop::Area),
    );
}

pub fn N7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "c", [0., 0., 0.], [4., 4., 4.]);
    ttranslate(s, "c", [0., 0., 6.]);
    bfuse(s, "rr", "b", "c");
    restore(s, "C1_rr_8.brep", "C1_rr_8");
    prism(s, "p1", "C1_rr_8", [0., 0., -4.]);
    explode(s, "rr", Some(ShapeEnum::Solid));
    two_step.bop(s, "rr_1", "p1");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(600.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["rr_1", "p1"], Prop::Area),
    );
}

pub fn N8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "c", [0., 0., 0.], [4., 4., 4.]);
    ttranslate(s, "c", [0., 0., 6.]);
    bfuse(s, "rr", "b", "c");
    restore(s, "C1_rr_8.brep", "C1_rr_8");
    prism(s, "p1", "C1_rr_8", [0., 0., 4.]);
    explode(s, "rr", Some(ShapeEnum::Solid));
    two_step.bop(s, "rr_1", "p1");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(600.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["rr_1", "p1"], Prop::Area),
    );
}

pub fn N9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "c", [0., 0., 0.], [4., 4., 4.]);
    ttranslate(s, "c", [0., 0., 2.]);
    bfuse(s, "rr", "b", "c");
    restore(s, "E1_rr_7.brep", "E1_rr_7");
    prism(s, "p1", "E1_rr_7", [0., -4., 0.]);
    explode(s, "rr", Some(ShapeEnum::Solid));
    two_step.bop(s, "rr_1", "p1");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(600.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["rr_1", "p1"], Prop::Area),
    );
}

pub fn O1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "c", [0., 0., 0.], [4., 4., 4.]);
    ttranslate(s, "c", [0., 0., 2.]);
    bfuse(s, "rr", "b", "c");
    restore(s, "E1_rr_7.brep", "E1_rr_7");
    prism(s, "p1", "E1_rr_7", [0., 4., 0.]);
    explode(s, "rr", Some(ShapeEnum::Solid));
    two_step.bop(s, "rr_1", "p1");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(632.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["rr_1", "p1"], Prop::Area),
    );
}

pub fn O2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh2a", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(250.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}
