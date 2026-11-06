//! The `bopcommon_complex` case bodies, part 1; each statement is one line of its script.
use super::*;

pub fn E3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "BUC60928.brep", "sh");
    explode(s, "sh", None);
    two_step.bop(s, "sh_1", "sh_2");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(30514.4));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["sh_1", "sh_2"], Prop::Area),
    );
}

pub fn E4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "BUC60928.brep", "sh");
    explode(s, "sh", None);
    two_step.bop(s, "sh_2", "sh_1");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(30514.4));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["sh_2", "sh_1"], Prop::Area),
    );
}

pub fn F2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "buc60996.brep", "sh");
    explode(s, "sh", None);
    two_step.bop(s, "sh_1", "sh_2");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(33561.9));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["sh_1", "sh_2"], Prop::Area),
    );
}

pub fn F3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "buc60996.brep", "sh");
    explode(s, "sh", None);
    two_step.bop(s, "sh_2", "sh_1");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(33561.9));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["sh_2", "sh_1"], Prop::Area),
    );
}

pub fn F6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a57", "a");
    restore(s, "a102", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn F7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b58", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(4912.9));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn F8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b60", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(11250.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn F9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b62", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(8410.54));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn G1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b64", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(10245.2));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn G2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b66", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(11123.7));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn G3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a90", "a");
    restore(s, "b90", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(14142.1));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn G4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b102", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(15000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn G5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b104", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(11926.3));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn G6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a106", "a");
    restore(s, "a157", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(4826.73));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn G7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b110", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(5827.66));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn G8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b112", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(15000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn G9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b114", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(11830.1));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn H1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a116", "a");
    restore(s, "b116", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(7853.98));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn H2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a116", "a");
    restore(s, "b118", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(7853.98));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn H3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a116", "a");
    restore(s, "b124", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn H4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a116", "a");
    restore(s, "b126", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(10000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn H5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "a138", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(15079.6));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn H6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b130", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(6126.11));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn H7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b132", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(17146.1));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn H8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b136", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(2884.96));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn H9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a138", "a");
    restore(s, "b130", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn I1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a138", "a");
    restore(s, "b140", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn I2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b142", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(13686.7));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn I3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "a152", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(7853.98));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn I4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "a158", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(7363.11));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn I5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a148", "a");
    restore(s, "b148", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(11781.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn I6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "a160", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(9817.48));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn I7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a152", "a");
    restore(s, "b118", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Common);
    checkprops(s, "result", Prop::Area, Expected::Mass(5890.37));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}
