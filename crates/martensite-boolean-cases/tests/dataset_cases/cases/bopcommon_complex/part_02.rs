//! The `bopcommon_complex` case bodies, part 2; each statement is one line of its script.
use super::*;

pub fn I8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a152", "a");
    restore(s, "a157", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(5340.71));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn I9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a157", "a");
    restore(s, "a152", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(5340.71));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn J1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a158", "a");
    restore(s, "b148", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Empty);
}

pub fn J2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a160", "a");
    restore(s, "b160", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(4208.94));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn J9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "a206", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(14007.8));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn K1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b199", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(7797.56));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn K2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a201", "a");
    restore(s, "b201", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(10424.2));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn K3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a201", "a");
    restore(s, "a206", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(8253.56));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn K4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a206", "a");
    restore(s, "b199", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(5545.23));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn K5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "buc60172a.rle", "a");
    restore(s, "buc60172b.rle", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(90486.2));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn K6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "c", [0., 0., 0.], [4., 4., 4.]);
    ttranslate(s, "c", [3., 3., 6.]);
    two_step.bop(s, "b", "c");
    two_step.result(s, "rr", Operation::Fuse);
    restore(s, "A1_rr_7.brep", "A1_rr_7");
    prism(s, "p1", "A1_rr_7", [0., 0., 4.]);
    two_step.bop(s, "rr", "p1");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["rr", "p1"], Prop::Area),
    );
}

pub fn K7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "c", [0., 0., 0.], [4., 4., 4.]);
    ttranslate(s, "c", [3., 3., 6.]);
    bfuse(s, "rr", "b", "c");
    restore(s, "A1_rr_7.brep", "A1_rr_7");
    prism(s, "p1", "A1_rr_7", [0., 0., -4.]);
    explode(s, "rr", Some(ShapeEnum::Solid));
    two_step.bop(s, "rr_1", "p1");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(96.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["rr_1", "p1"], Prop::Area),
    );
}

pub fn K8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "c", [0., 0., 0.], [4., 4., 4.]);
    ttranslate(s, "c", [0., 0., 6.]);
    bfuse(s, "rr", "b", "c");
    restore(s, "C1_rr_8.brep", "C1_rr_8");
    prism(s, "p1", "C1_rr_8", [0., 0., -4.]);
    explode(s, "rr", Some(ShapeEnum::Solid));
    two_step.bop(s, "rr_1", "p1");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(96.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["rr_1", "p1"], Prop::Area),
    );
}

pub fn K9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "c", [0., 0., 0.], [4., 4., 4.]);
    ttranslate(s, "c", [0., 0., 6.]);
    bfuse(s, "rr", "b", "c");
    restore(s, "C1_rr_8.brep", "C1_rr_8");
    prism(s, "p1", "C1_rr_8", [0., 0., 4.]);
    explode(s, "rr", Some(ShapeEnum::Solid));
    two_step.bop(s, "rr_1", "p1");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["rr_1", "p1"], Prop::Area),
    );
}

pub fn L1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "c", [0., 0., 0.], [4., 4., 4.]);
    ttranslate(s, "c", [0., 0., 2.]);
    bfuse(s, "rr", "b", "c");
    restore(s, "E1_rr_7.brep", "E1_rr_7");
    prism(s, "p1", "E1_rr_7", [0., -4., 0.]);
    explode(s, "rr", Some(ShapeEnum::Solid));
    two_step.bop(s, "rr_1", "p1");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["rr_1", "p1"], Prop::Area),
    );
}

pub fn L2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "c", [0., 0., 0.], [4., 4., 4.]);
    ttranslate(s, "c", [0., 0., 2.]);
    bfuse(s, "rr", "b", "c");
    restore(s, "E1_rr_7.brep", "E1_rr_7");
    prism(s, "p1", "E1_rr_7", [0., 4., 0.]);
    explode(s, "rr", Some(ShapeEnum::Solid));
    two_step.bop(s, "rr_1", "p1");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(96.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["rr_1", "p1"], Prop::Area),
    );
}

pub fn L3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh2a", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(350.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn L4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh3", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn L5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh4", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn L6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh7", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(100.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn L7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh8", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(95.7107));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn L8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh9", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(400.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn L9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh10", "b1");
    restore(s, "shx", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(331.371));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn M1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh5", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn M2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "g1", "b1");
    restore(s, "g2", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(2094.4));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn M3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sg1", "b1");
    restore(s, "sg2", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(2094.4));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn M4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sf4", "b1");
    restore(s, "sf5", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(1610.86));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn M5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sf5", "b1");
    restore(s, "sf3", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(1610.86));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn M6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sf5", "b1");
    restore(s, "sf55", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(37699.1));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}
