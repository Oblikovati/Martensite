//! The `bopcut_complex` case bodies, part 3; each statement is one line of its script.
use super::*;

pub fn O3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh3", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(600.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn O4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh4", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(600.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn O5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh7", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(500.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn O6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh8", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(504.289));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn O7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh9", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(200.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn O8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh10", "b1");
    restore(s, "shx", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(68.6292));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn O9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sh2", "b1");
    restore(s, "sh5", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(600.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn P1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "g1", "b1");
    restore(s, "g2", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(4886.92));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn P2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sg1", "b1");
    restore(s, "sg2", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(4886.92));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn P3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sf4", "b1");
    restore(s, "sf5", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn P4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sf5", "b1");
    restore(s, "sf3", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(61221.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}

pub fn P5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sf5", "b1");
    restore(s, "sf55", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(25132.7));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
}
