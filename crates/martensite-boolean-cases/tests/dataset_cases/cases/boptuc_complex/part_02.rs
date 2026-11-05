//! The `boptuc_complex` case bodies, part 2; each statement is one line of its script.
use super::*;

pub fn E6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a157", "a");
    restore(s, "a152", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(10053.1));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn E7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a158", "a");
    restore(s, "b148", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(23561.8));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn E8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a160", "a");
    restore(s, "b160", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(31415.7));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn F7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "a206", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn F8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b199", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(23498.7));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn F9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a206", "a");
    restore(s, "b199", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(25398.9));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn G1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a208", "a");
    restore(s, "b210", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(8997.65));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn G2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "c", [0., 0., 0.], [4., 4., 4.]);
    ttranslate(s, "c", [3., 3., 6.]);
    two_step.bop(s, "b", "c");
    two_step.result(s, "rr", Operation::Fuse);
    restore(s, "A1_rr_7.brep", "A1_rr_7");
    prism(s, "p1", "A1_rr_7", [0., 0., 4.]);
    two_step.bop(s, "rr", "p1");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(96.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["rr", "p1"], Prop::Area),
    );
}

pub fn G3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "c", [0., 0., 0.], [4., 4., 4.]);
    ttranslate(s, "c", [3., 3., 6.]);
    bfuse(s, "rr", "b", "c");
    restore(s, "A1_rr_7.brep", "A1_rr_7");
    prism(s, "p1", "A1_rr_7", [0., 0., -4.]);
    explode(s, "rr", Some(ShapeEnum::Solid));
    two_step.bop(s, "rr_1", "p1");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["rr_1", "p1"], Prop::Area),
    );
}

pub fn G4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "c", [0., 0., 0.], [4., 4., 4.]);
    ttranslate(s, "c", [0., 0., 6.]);
    bfuse(s, "rr", "b", "c");
    restore(s, "C1_rr_8.brep", "C1_rr_8");
    prism(s, "p1", "C1_rr_8", [0., 0., -4.]);
    explode(s, "rr", Some(ShapeEnum::Solid));
    two_step.bop(s, "rr_1", "p1");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["rr_1", "p1"], Prop::Area),
    );
}

pub fn G5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "c", [0., 0., 0.], [4., 4., 4.]);
    ttranslate(s, "c", [0., 0., 6.]);
    bfuse(s, "rr", "b", "c");
    restore(s, "C1_rr_8.brep", "C1_rr_8");
    prism(s, "p1", "C1_rr_8", [0., 0., 4.]);
    explode(s, "rr", Some(ShapeEnum::Solid));
    two_step.bop(s, "rr_1", "p1");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(96.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["rr_1", "p1"], Prop::Area),
    );
}

pub fn G6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "c", [0., 0., 0.], [4., 4., 4.]);
    ttranslate(s, "c", [0., 0., 2.]);
    bfuse(s, "rr", "b", "c");
    restore(s, "E1_rr_7.brep", "E1_rr_7");
    prism(s, "p1", "E1_rr_7", [0., -4., 0.]);
    explode(s, "rr", Some(ShapeEnum::Solid));
    two_step.bop(s, "rr_1", "p1");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(96.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["rr_1", "p1"], Prop::Area),
    );
}

pub fn G7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "c", [0., 0., 0.], [4., 4., 4.]);
    ttranslate(s, "c", [0., 0., 2.]);
    bfuse(s, "rr", "b", "c");
    restore(s, "E1_rr_7.brep", "E1_rr_7");
    prism(s, "p1", "E1_rr_7", [0., 4., 0.]);
    explode(s, "rr", Some(ShapeEnum::Solid));
    two_step.bop(s, "rr_1", "p1");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["rr_1", "p1"], Prop::Area),
    );
}

pub fn G8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh2a", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(250.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn G9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh3", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(400.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn H1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh4", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(300.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn H2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh7", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(300.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn H3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh8", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(304.289));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn H4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh9", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(200.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn H5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh10", "b1");
    restore(s, "shx", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(68.6292));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn H6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh5", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(600.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn H7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "g1", "b1");
    restore(s, "g2", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(4886.92));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn H8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sg1", "b1");
    restore(s, "sg2", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(4886.92));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn H9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sf4", "b1");
    restore(s, "sf5", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(61221.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn I1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sf5", "b1");
    restore(s, "sf3", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn I2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sf5", "b1");
    restore(s, "sf55", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(25132.7));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}
