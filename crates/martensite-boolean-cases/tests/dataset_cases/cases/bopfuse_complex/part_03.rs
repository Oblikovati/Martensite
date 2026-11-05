//! The `bopfuse_complex` case bodies, part 3; each statement is one line of its script.
use super::*;

pub fn M4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    println!("PRO15203");
    println!("Fuse");
    println!();
    restore(s, "pro15203a.rle", "c1");
    restore(s, "pro15203d.rle", "c2");
    explode(s, "c1", Some(ShapeEnum::Face));
    copy(s, "c1_1", "c1");
    explode(s, "c2", Some(ShapeEnum::Face));
    copy(s, "c2_1", "c2");
    two_step.bop(s, "c1", "c2");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(570.047));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["c1", "c2"], Prop::Area),
    );
}

pub fn M5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "c", [0., 0., 0.], [4., 4., 4.]);
    ttranslate(s, "c", [3., 3., 6.]);
    two_step.bop(s, "b", "c");
    two_step.result(s, "rr", Operation::Fuse);
    restore(s, "A1_rr_7.brep", "A1_rr_7");
    prism(s, "p1", "A1_rr_7", [0., 0., 4.]);
    two_step.bop(s, "rr", "p1");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(664.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["rr", "p1"], Prop::Area),
    );
}

pub fn M6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "c", [0., 0., 0.], [4., 4., 4.]);
    ttranslate(s, "c", [3., 3., 6.]);
    bfuse(s, "rr", "b", "c");
    restore(s, "A1_rr_7.brep", "A1_rr_7");
    prism(s, "p1", "A1_rr_7", [0., 0., -4.]);
    explode(s, "rr", Some(ShapeEnum::Solid));
    two_step.bop(s, "rr_1", "p1");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(600.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["rr_1", "p1"], Prop::Area),
    );
}

pub fn M7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "c", [0., 0., 0.], [4., 4., 4.]);
    ttranslate(s, "c", [0., 0., 6.]);
    bfuse(s, "rr", "b", "c");
    restore(s, "C1_rr_8.brep", "C1_rr_8");
    prism(s, "p1", "C1_rr_8", [0., 0., -4.]);
    explode(s, "rr", Some(ShapeEnum::Solid));
    two_step.bop(s, "rr_1", "p1");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(600.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["rr_1", "p1"], Prop::Area),
    );
}

pub fn M8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "c", [0., 0., 0.], [4., 4., 4.]);
    ttranslate(s, "c", [0., 0., 6.]);
    bfuse(s, "rr", "b", "c");
    restore(s, "C1_rr_8.brep", "C1_rr_8");
    prism(s, "p1", "C1_rr_8", [0., 0., 4.]);
    explode(s, "rr", Some(ShapeEnum::Solid));
    two_step.bop(s, "rr_1", "p1");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(664.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["rr_1", "p1"], Prop::Area),
    );
}

pub fn M9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "c", [0., 0., 0.], [4., 4., 4.]);
    ttranslate(s, "c", [0., 0., 2.]);
    bfuse(s, "rr", "b", "c");
    restore(s, "E1_rr_7.brep", "E1_rr_7");
    prism(s, "p1", "E1_rr_7", [0., -4., 0.]);
    explode(s, "rr", Some(ShapeEnum::Solid));
    two_step.bop(s, "rr_1", "p1");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(664.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["rr_1", "p1"], Prop::Area),
    );
}

pub fn N1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    make_box(s, "c", [0., 0., 0.], [4., 4., 4.]);
    ttranslate(s, "c", [0., 0., 2.]);
    bfuse(s, "rr", "b", "c");
    restore(s, "E1_rr_7.brep", "E1_rr_7");
    prism(s, "p1", "E1_rr_7", [0., 4., 0.]);
    explode(s, "rr", Some(ShapeEnum::Solid));
    two_step.bop(s, "rr_1", "p1");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(600.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["rr_1", "p1"], Prop::Area),
    );
}

pub fn N2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh2a", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(1200.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn N3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh3", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(1000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn N4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh4", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(900.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn N5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh9", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(1200.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn N6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh10", "b1");
    restore(s, "shx", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(800.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn N7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh5", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(1200.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn N8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "af", "b1");
    restore(s, "bf", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Fuse);
    restore(s, "cf", "b3");
    two_step.bop(s, "result", "b3");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(12677.1));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["result", "b3"], Prop::Area),
    );
}

pub fn N9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "g1", "b1");
    restore(s, "g2", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(11868.2));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn O1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sg1", "b1");
    restore(s, "sg2", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(13962.6));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn O2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sf4", "b1");
    restore(s, "sf5", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(62831.9));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn O3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sf5", "b1");
    restore(s, "sf3", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(62831.9));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn O4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sf5", "b1");
    restore(s, "sf55", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(125664.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}
