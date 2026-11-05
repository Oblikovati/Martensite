//! The `bfuse_complex` case bodies, part 3; each statement is one line of its script.
use super::*;

pub fn H8(s: &mut dyn Session) {
    restore(s, "CTO902_cts20487a.rle", "a");
    restore(s, "cts20487b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(165253.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn H9(s: &mut dyn Session) {
    restore(s, "CTO902_cts20506a.rle", "a");
    restore(s, "cts20506b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(1.41768e+06));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn I1(s: &mut dyn Session) {
    restore(s, "CTO902_cts20492a.rle", "a");
    restore(s, "CTO902_cts20492b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(40078.6));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn I2(s: &mut dyn Session) {
    restore(s, "CTO902_cts20493a.rle", "a");
    restore(s, "cts20493b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(186203.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn I3(s: &mut dyn Session) {
    restore(s, "CTO902_cts20494a.rle", "a");
    restore(s, "cts20494b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(63361.7));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn I4(s: &mut dyn Session) {
    restore(s, "cts20495a.rle", "a");
    restore(s, "CTO902_cts20495b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(54470.6));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn I5(s: &mut dyn Session) {
    restore(s, "cts20552-part.rle", "a");
    restore(s, "cts20552-tool.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(221130.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn I6(s: &mut dyn Session) {
    restore(s, "cts20551-tool.rle", "a");
    restore(s, "CTO902_cts20551b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(452089.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn I7(s: &mut dyn Session) {
    restore(s, "CTO902_cts20535a.rle", "a");
    restore(s, "CTO902_cts20535b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(81701.3));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn I8(s: &mut dyn Session) {
    restore(s, "CTO902_cts20529a.rle", "a");
    restore(s, "CTO902_cts20529b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(192383.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn I9(s: &mut dyn Session) {
    restore(s, "pro12956a.rle", "a");
    restore(s, "CTO904_pro12956b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(68458.3));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn J1(s: &mut dyn Session) {
    pcylinder(s, "c1", 10., 20.);
    pcylinder(s, "c2", 5., 20.);
    ttranslate(s, "c2", [5., 0., 10.]);
    bfuse(s, "result", "c1", "c2");
    checkprops(s, "result", Prop::Area, Expected::Mass(2199.11));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["c1", "c2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn J2(s: &mut dyn Session) {
    restore(s, "CTO902_cts20482a.rle", "part");
    restore(s, "CTO902_cts20482b.rle", "tool1");
    restore(s, "CTO902_cts20482c.rle", "tool2");
    bfuse(s, "result1", "part", "tool1");
    bfuse(s, "result", "result1", "tool2");
    checkprops(s, "result", Prop::Area, Expected::Mass(189131.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["result1", "tool2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "part", Prop::Area, "175255");
    kind_is(s, "part", ShapeEnum::Solid);
    printed_mass(s, "tool1", Prop::Area, "19435.5");
    kind_is(s, "tool1", ShapeEnum::Solid);
    printed_mass(s, "result1", Prop::Area, "182147");
    kind_is(s, "result1", ShapeEnum::Compound);
    printed_mass(s, "tool2", Prop::Area, "24037");
    kind_is(s, "tool2", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn J3(s: &mut dyn Session) {
    restore(s, "CTO902_cts20484a.rle", "part");
    restore(s, "cts20484-tool1.rle", "tool1");
    restore(s, "cts20484-tool2.rle", "tool2");
    bfuse(s, "result1", "part", "tool1");
    bfuse(s, "result", "result1", "tool2");
    checkprops(s, "result", Prop::Area, Expected::Mass(133085.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["result1", "tool2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "part", Prop::Area, "116552");
    kind_is(s, "part", ShapeEnum::Compound);
    printed_mass(s, "tool1", Prop::Area, "10873.3");
    kind_is(s, "tool1", ShapeEnum::Solid);
    printed_mass(s, "result1", Prop::Area, "123859");
    kind_is(s, "result1", ShapeEnum::Compound);
    printed_mass(s, "tool2", Prop::Area, "15782.2");
    kind_is(s, "tool2", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn J4(s: &mut dyn Session) {
    restore(s, "cts20549a.rle", "a");
    restore(s, "CTO904_cts20549b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(22561.3));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn J5(s: &mut dyn Session) {
    pcylinder(s, "a", 50., 150.);
    pcylinder(s, "b", 50., 150.);
    trotate(s, "b", [0., 0., 75.], [0., 1., 0.], 90.);
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(85663.7));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn J7(s: &mut dyn Session) {
    restore(s, "cts20549a.rle", "a");
    restore(s, "CTO904_cts20549b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(22561.3));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn J8(s: &mut dyn Session) {
    restore(s, "cts20751a.rle", "a");
    restore(s, "CTO904_cts20751b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(52140.6));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn J9(s: &mut dyn Session) {
    restore(s, "CTO904_cts20769a.rle", "a");
    restore(s, "cts20769a.brep", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(92385.1));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn K1(s: &mut dyn Session) {
    restore(s, "CTO904_cts20370-part.rle", "a");
    restore(s, "cts20370-tool.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(261265.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn K2(s: &mut dyn Session) {
    restore(s, "CTO904_cts20578a.rle", "a");
    restore(s, "CTO904_cts20578b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(759758.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn K3(s: &mut dyn Session) {
    restore(s, "CTO904_fra60275a.rle", "s");
    restore(s, "fra60275b.rle", "c");
    tcopy(s, "c", "w");
    mkplane(s, "f", "w");
    prism(s, "pp", "f", [0., 0., 100.]);
    bfuse(s, "result", "s", "pp");
    checkprops(s, "result", Prop::Area, Expected::Mass(358551.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["s", "pp"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "s", Prop::Area, "315233");
    kind_is(s, "s", ShapeEnum::Solid);
    printed_mass(s, "pp", Prop::Area, "73184.6");
    kind_is(s, "pp", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn K5(s: &mut dyn Session) {
    restore(s, "CTO904_buc60116a.rle", "a");
    restore(s, "buc60116b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(224496.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn K6(s: &mut dyn Session) {
    restore(s, "cts20973a.rle", "a");
    restore(s, "cts20973b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(324489.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn L1(s: &mut dyn Session) {
    restore(s, "CTO902_cts20525-part.rle", "a");
    restore(s, "CTO902_cts20525-tool.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(160000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn L2(s: &mut dyn Session) {
    restore(s, "CTO900_cts21453a.rle", "a");
    restore(s, "CTO900_cts21453b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(163507.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn L3(s: &mut dyn Session) {
    restore(s, "CTO902_cts20528-part.rle", "a");
    restore(s, "cts20528-tool.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(112227.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn L4(s: &mut dyn Session) {
    restore(s, "CTO904_pro12859a.rle", "a");
    restore(s, "CTO904_pro12859b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(468899.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn L5(s: &mut dyn Session) {
    restore(s, "CTO904_pro12852a.rle", "a");
    restore(s, "CTO904_pro12852b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(592128.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn L8(s: &mut dyn Session) {
    restore(s, "CTO904_cts21326a.rle", "a");
    restore(s, "cts21326b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(50708.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn L9(s: &mut dyn Session) {
    restore(s, "cts21199_base.rle", "a");
    restore(s, "cts21199_face.rle", "skface");
    prism(s, "b", "skface", [0., 0., -50.]);
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(18033.5));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a", Prop::Area, "13941.9");
    kind_is(s, "a", ShapeEnum::Compound);
    printed_mass(s, "b", Prop::Area, "8796.46");
    kind_is(s, "b", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}
