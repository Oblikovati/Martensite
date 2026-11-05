//! The `bopfuse_complex` case bodies, part 1; each statement is one line of its script.
use super::*;

pub fn D4(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    restore(s, "BUC60928.brep", "sh");
    explode(s, "sh", None);
    bop.arguments(s, &["sh_1", "sh_2"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "rx");
    cells.bcadd(s, "result", &[("sh_1", 1)], 1, true);
    cells.bcadd(s, "result", &[("sh_2", 1), ("sh_1", 0)], 0, false);
    cells.bcmakecontainers(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Solids(1), NbCount::Shells(2), NbCount::Faces(10)],
    );
    checkprops(s, "result", Prop::Area, Expected::Mass(487710.));
    checkprops(s, "result", Prop::Volume, Expected::Mass(1.37052e+007));
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    part_count(s, "sh", None, 2);
}

pub fn D5(s: &mut dyn Session) {
    let mut bop = BopSession::default();
    let mut cells = CellsSession::default();
    restore(s, "BUC60928.brep", "sh");
    explode(s, "sh", None);
    bop.arguments(s, &["sh_2", "sh_1"], &[]);
    bop.bfillds();
    cells.bcbuild(s, &mut bop, "rx");
    cells.bcadd(s, "result", &[("sh_1", 1)], 1, true);
    cells.bcadd(s, "result", &[("sh_2", 1), ("sh_1", 0)], 0, false);
    cells.bcmakecontainers(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[NbCount::Solids(1), NbCount::Shells(2), NbCount::Faces(10)],
    );
    checkprops(s, "result", Prop::Area, Expected::Mass(487710.));
    checkprops(s, "result", Prop::Volume, Expected::Mass(1.37052e+007));
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    part_count(s, "sh", None, 2);
}

pub fn E3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "buc60996.brep", "sh");
    explode(s, "sh", None);
    two_step.bop(s, "sh_1", "sh_2");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(73561.9));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["sh_1", "sh_2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "sh_1", Prop::Area, "60000");
    kind_is(s, "sh_1", ShapeEnum::Solid);
    printed_mass(s, "sh_2", Prop::Area, "47123.9");
    kind_is(s, "sh_2", ShapeEnum::Solid);
    part_count(s, "sh", None, 2);
    view_mass_is_nonzero(s, "result");
}

pub fn E4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "buc60996.brep", "sh");
    explode(s, "sh", None);
    two_step.bop(s, "sh_2", "sh_1");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(73561.9));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["sh_2", "sh_1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "sh_2", Prop::Area, "47123.9");
    kind_is(s, "sh_2", ShapeEnum::Solid);
    printed_mass(s, "sh_1", Prop::Area, "60000");
    kind_is(s, "sh_1", ShapeEnum::Solid);
    part_count(s, "sh", None, 2);
    view_mass_is_nonzero(s, "result");
}

pub fn E8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a57", "a");
    restore(s, "a102", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(22200.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn E9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b58", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(19521.9));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn F1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b60", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(18750.));
    checkprops(s, "result", Prop::Volume, Expected::Mass(156250.));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Wires(36),
            NbCount::Faces(36),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a", Prop::Area, "15000");
    kind_is(s, "a", ShapeEnum::Compound);
    printed_mass(s, "b", Prop::Area, "15000");
    kind_is(s, "b", ShapeEnum::Compound);
    view_mass_is_nonzero(s, "result");
}

pub fn F2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b62", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(20389.2));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn F3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b64", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(30735.6));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn F4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b66", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(19659.3));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn F5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a90", "a");
    restore(s, "b90", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(24142.1));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn F6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b102", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(18961.2));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn F7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b104", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(22567.7));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn F8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a106", "a");
    restore(s, "a157", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(10933.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn F9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b110", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(16240.9));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn G1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b112", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(32686.4));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn G2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b114", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(21046.7));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn G3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a116", "a");
    restore(s, "b116", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(12566.4));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn G4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a116", "a");
    restore(s, "b118", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(12566.4));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn G5(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a116", "a");
    restore(s, "b124", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(17907.1));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn G6(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a116", "a");
    restore(s, "b126", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(13561.9));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn G7(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "a138", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(15000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn G8(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b130", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(17420.9));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn G9(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b132", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(24401.3));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn H1(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b136", "b");
    fixshape(s, "b", "b");
    settolerance(s, "b", 0.003);
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(16),
            NbCount::Edges(28),
            NbCount::Wires(13),
            NbCount::Faces(13),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkprops(s, "result", Prop::Area, Expected::Mass(20777.6));
    checkprops(s, "result", Prop::Volume, Expected::Mass(173396.));
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a", Prop::Area, "15000");
    kind_is(s, "a", ShapeEnum::Compound);
    printed_mass(s, "b", Prop::Area, "10311.9");
    kind_is(s, "b", ShapeEnum::Compound);
}

pub fn H2(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a138", "a");
    restore(s, "b130", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(14201.9));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn H3(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a138", "a");
    restore(s, "b140", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(19534.6));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}

pub fn H4(s: &mut dyn Session) {
    let mut two_step = TwoStepBoolean::default();
    restore(s, "a102", "a");
    restore(s, "b142", "b");
    two_step.bop(s, "a", "b");
    two_step.result(s, "result", Operation::Fuse);
    checkprops(s, "result", Prop::Area, Expected::Mass(17021.3));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
}
