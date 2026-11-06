//! The `boptuc_complex` case bodies, part 1; each statement is one line of its script.
use super::*;

pub fn A7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b11", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(1280.22));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn A8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b13", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(2518.43));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn A9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b15", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(4785.54));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn B1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b17", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(9241.17));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn B2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b19", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(13297.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn B3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a57", "a");
    restore(s, "a102", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(15000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn B4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b58", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(9140.77));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn B6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b62", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(9196.86));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn B7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b64", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(23235.6));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn B8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b66", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(14659.3));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn B9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a90", "a");
    restore(s, "b90", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(19142.1));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn C1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b102", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(23961.2));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn C2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b104", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(25134.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn C3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a106", "a");
    restore(s, "a157", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(6240.45));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn C4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b110", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(5994.67));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn C5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b112", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(47686.4));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn C6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b114", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(21046.7));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn C7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a116", "a");
    restore(s, "b116", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(12566.4));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn C8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a116", "a");
    restore(s, "b118", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(4712.39));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn C9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a116", "a");
    restore(s, "b124", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(6126.11));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn D1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a116", "a");
    restore(s, "b126", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(11781.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn D2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "a138", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn D3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b130", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(6347.92));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn D4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b132", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(19918.1));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn D6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a138", "a");
    restore(s, "b130", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(11060.3));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn D7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a138", "a");
    restore(s, "b140", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(4454.93));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn D8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b142", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(25583.2));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn D9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "a152", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn E1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "a158", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(25034.4));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn E2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a148", "a");
    restore(s, "b148", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(23561.9));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn E3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "a160", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(33379.4));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn E4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a152", "a");
    restore(s, "b118", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(10602.8));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn E5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a152", "a");
    restore(s, "a157", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut21);
    checkprops(s, "result", Prop::Area, Expected::Mass(4869.47));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}
