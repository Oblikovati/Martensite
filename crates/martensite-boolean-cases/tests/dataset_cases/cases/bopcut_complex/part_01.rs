//! The `bopcut_complex` case bodies, part 1; each statement is one line of its script.
use super::*;

pub fn E5(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    restore(s, "BUC60928.brep", "sh");
    explode(s, "sh", None);
    bop.arguments(s, &["sh_1", "sh_2"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "rx");
    cells.bcadd(s, "result", &[("sh_1", 1), ("sh_2", 0)], 0, false);
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Solids(3), NbCount::Shells(3), NbCount::Faces(14)],
    );
    checkprops(s, "result", Prop::Area, Expected::Mass(190481.));
    checkprops(s, "result", Prop::Volume, Expected::Mass(1.23285e+006));
    checkview(s, &CheckView::of("result"));
}

pub fn E6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "BUC60928.brep", "sh");
    explode(s, "sh", None);
    two_step.bop(s, "sh_2", "sh_1");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(358258.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["sh_2", "sh_1"], Prop::Area),
    );
}

pub fn F4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "buc60996.brep", "sh");
    explode(s, "sh", None);
    two_step.bop(s, "sh_1", "sh_2");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(57854.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["sh_1", "sh_2"], Prop::Area),
    );
}

pub fn F5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "buc60996.brep", "sh");
    explode(s, "sh", None);
    two_step.bop(s, "sh_2", "sh_1");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(33561.9));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["sh_2", "sh_1"], Prop::Area),
    );
}

pub fn F8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b11", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(17478.5));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn F9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b13", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(18354.2));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn G1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b15", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(16226.2));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn G2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b17", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(14227.1));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn G3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b19", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(15874.1));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn G4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b21", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(8143.4));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn G5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a57", "a");
    restore(s, "a102", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(7199.97));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn G6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b58", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(15294.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn G7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b60", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(15000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn G8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b62", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(19602.9));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn G9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b64", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(17745.2));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn H1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b66", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(16123.7));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn H2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a90", "a");
    restore(s, "b90", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(19142.1));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn H3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b102", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn H4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b104", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(9359.94));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn H5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a106", "a");
    restore(s, "b106", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(9519.3));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn H6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a106", "a");
    restore(s, "a157", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(9519.3));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn H7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b110", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(16073.9));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn H8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b112", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(2.32262e-10));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn H9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b114", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(11830.1));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn I1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a116", "a");
    restore(s, "b116", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(7853.98));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn I2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a116", "a");
    restore(s, "b118", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(7853.98));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn I3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a116", "a");
    restore(s, "b124", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(11781.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn I4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a116", "a");
    restore(s, "b126", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(11781.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn I5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "a138", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(25053.1));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn I6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b130", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(18298.7));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn I7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b132", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(21073.1));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn I8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b136", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(15256.6));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn I9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a138", "a");
    restore(s, "b130", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Cut);
    checkprops(s, "result", Prop::Area, Expected::Mass(15079.6));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}
