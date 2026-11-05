//! The `bopsection` case bodies, part 2; each statement is one line of its script.
use super::*;

pub fn F2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sg1", "b1");
    restore(s, "sg2", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(189.813));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Length),
    );
}

pub fn F3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sf4", "b1");
    restore(s, "sf5", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(200.543));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Length),
    );
}

pub fn F4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sf5", "b1");
    restore(s, "sf3", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(160.543));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Length),
    );
}

pub fn F5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "sf5", "b1");
    restore(s, "sf55", "b2");
    two_step.bop(s, "b1", "b2");
    two_step.section(s, "result");
    checkprops(s, "result", Prop::Length, Expected::Mass(1376.64));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Length),
    );
}
