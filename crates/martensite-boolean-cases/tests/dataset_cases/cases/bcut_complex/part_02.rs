//! The `bcut_complex` case bodies, part 2; each statement is one line of its script.
use super::*;

pub fn D9(s: &mut dyn Session) {
    restore(s, "so2", "b1");
    restore(s, "so7", "b2");
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(269.65));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn E1(s: &mut dyn Session) {
    restore(s, "so2", "b1");
    restore(s, "so6", "b2");
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(228.015));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn E2(s: &mut dyn Session) {
    restore(s, "so2", "b1");
    restore(s, "so6", "b2");
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(340.931));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn E3(s: &mut dyn Session) {
    restore(s, "pro5557_revol.brep", "revol1");
    restore(s, "pro5557_bsurf.brep", "bsurf2");
    halfspace(s, "space2", "bsurf2", [0., -0.3, 0.]);
    bcut(s, "result", "revol1", "space2");
    checkprops(s, "result", Prop::Area, Expected::Mass(0.402138));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["revol1", "space2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "revol1", Prop::Area, "0.74513");
    kind_is(s, "revol1", ShapeEnum::Solid);
    printed_mass(s, "space2", Prop::Area, "123.107");
    kind_is(s, "space2", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn E4(s: &mut dyn Session) {
    restore(s, "CTO900_pro6066a.rle", "prot1");
    restore(s, "CTO900_pro6066b.rle", "box2");
    bcut(s, "result", "prot1", "box2");
    checkprops(s, "result", Prop::Area, Expected::Mass(0.264921));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["prot1", "box2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn E6(s: &mut dyn Session) {
    restore(s, "CTO900_ger60043-part.rle", "part");
    restore(s, "CTO900_ger60043-tool.rle", "tool");
    bcut(s, "result", "part", "tool");
    checkprops(s, "result", Prop::Area, Expected::Mass(17135.1));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn E8(s: &mut dyn Session) {
    restore(s, "CTO900_jap60038-part.rle", "part");
    restore(s, "jap60038-tool.rle", "tool");
    bcut(s, "result", "part", "tool");
    checkprops(s, "result", Prop::Area, Expected::Mass(173510.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn E9(s: &mut dyn Session) {
    restore(s, "CTO900_pro6066-part.rle", "part");
    restore(s, "CTO900_pro6066-tool.rle", "tool");
    bcut(s, "result", "part", "tool");
    checkprops(s, "result", Prop::Area, Expected::Mass(570249.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn F1(s: &mut dyn Session) {
    restore(s, "cts17785-part.rle", "part");
    restore(s, "cts17785-tool.rle", "tool");
    bcut(s, "result", "part", "tool");
    checkprops(s, "result", Prop::Area, Expected::Mass(56064.6));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn F3(s: &mut dyn Session) {
    restore(s, "CTO900_cts18140-part.rle", "part");
    restore(s, "CTO900_cts18140-tool.rle", "tool");
    bcut(s, "result", "part", "tool");
    checkprops(s, "result", Prop::Area, Expected::Mass(92062.6));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn F8(s: &mut dyn Session) {
    restore(s, "CTO900_pro8951a.rle", "base");
    restore(s, "pro8951d.rle", "prism");
    bcut(s, "result", "base", "prism");
    checkprops(s, "result", Prop::Area, Expected::Mass(298872.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["base", "prism"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn F9(s: &mut dyn Session) {
    restore(s, "CTO900_fra60656-part.rle", "part");
    restore(s, "CTO900_fra60656-tool.rle", "tool");
    bcut(s, "result", "part", "tool");
    checkprops(s, "result", Prop::Area, Expected::Mass(1.21042e+06));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn G4(s: &mut dyn Session) {
    restore(s, "CTO900_fra50089-part.rle", "part");
    restore(s, "fra50089-tool.rle", "tool");
    bcut(s, "result", "part", "tool");
    checkprops(s, "result", Prop::Area, Expected::Mass(193593.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn G5(s: &mut dyn Session) {
    restore(s, "CTO900_pro8320-part.rle", "part");
    restore(s, "pro8320-tool.rle", "tool");
    bcut(s, "result", "part", "tool");
    checkprops(s, "result", Prop::Area, Expected::Mass(13770.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn G9(s: &mut dyn Session) {
    restore(s, "cts21128c.rle", "c");
    restore(s, "cts21128d.rle", "d");
    bcut(s, "result", "c", "d");
    checkprops(s, "result", Prop::Area, Expected::Mass(2814.01));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["c", "d"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn H2(s: &mut dyn Session) {
    restore(s, "CTO900_cts20212-part.rle", "part");
    restore(s, "CTO900_cts20212-tool.rle", "tool");
    bcut(s, "result", "part", "tool");
    checkprops(s, "result", Prop::Area, Expected::Mass(3038.87));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn H3(s: &mut dyn Session) {
    restore(s, "CTO900_cts20374-part.rle", "part");
    restore(s, "cts20374-tool.rle", "tool");
    bcut(s, "result", "part", "tool");
    checkprops(s, "result", Prop::Area, Expected::Mass(472107.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn H4(s: &mut dyn Session) {
    restore(s, "CTO900_pro12953-part.rle", "part");
    restore(s, "CTO900_pro12953-tool.rle", "tool");
    bcut(s, "result", "part", "tool");
    checkprops(s, "result", Prop::Area, Expected::Mass(78.7758));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn H5(s: &mut dyn Session) {
    restore(s, "CTO900_pro15323c.rle", "part");
    restore(s, "pro15323d.rle", "tool");
    bcut(s, "result", "part", "tool");
    checkprops(s, "result", Prop::Area, Expected::Mass(43877.8));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn H9(s: &mut dyn Session) {
    restore(s, "CTO900_ger50084c.rle", "prism");
    restore(s, "CTO900_ger50084d.rle", "base");
    bcut(s, "result", "prism", "base");
    checkprops(s, "result", Prop::Area, Expected::Mass(1290.02));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["prism", "base"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn I2(s: &mut dyn Session) {
    restore(s, "CTO901_cts20138_base.rle", "base");
    restore(s, "CTO901_cts20138_outil.rle", "outil");
    bcut(s, "result", "base", "outil");
    checkprops(s, "result", Prop::Area, Expected::Mass(31806.5));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["base", "outil"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn I4(s: &mut dyn Session) {
    restore(s, "CTO902_cts20456a.rle", "a");
    restore(s, "cts20456b.rle", "b");
    bfuse(s, "result", "a", "b");
    make_box(s, "bb", [0., 0., 0.], [100., 100., -200.]);
    bcut(s, "result", "result", "bb");
    checkprops(s, "result", Prop::Area, Expected::Mass(1.22443e+06));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["result", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a", Prop::Area, "1.12327e+06");
    kind_is(s, "a", ShapeEnum::Solid);
    printed_mass(s, "b", Prop::Area, "279701");
    kind_is(s, "b", ShapeEnum::Solid);
    printed_mass(s, "result", Prop::Area, "1.22443e+06");
    kind_is(s, "result", ShapeEnum::Compound);
    printed_mass(s, "bb", Prop::Area, "100000");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn I5(s: &mut dyn Session) {
    restore(s, "CTO902_cts20462-part.rle", "part");
    restore(s, "CTO902_cts20462-tool.rle", "tool");
    bcut(s, "result", "part", "tool");
    checkprops(s, "result", Prop::Area, Expected::Mass(25298.7));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn I6(s: &mut dyn Session) {
    restore(s, "CTO902_cts20455-part.rle", "part");
    restore(s, "CTO902_cts20455-tool.rle", "tool");
    bcut(s, "result", "part", "tool");
    checkprops(s, "result", Prop::Area, Expected::Mass(26347.6));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn I7(s: &mut dyn Session) {
    restore(s, "CTO902_cts20459-part.rle", "part");
    restore(s, "CTO902_cts20459-tool.rle", "tool");
    bcut(s, "result", "part", "tool");
    checkprops(s, "result", Prop::Area, Expected::Mass(26347.6));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn I9(s: &mut dyn Session) {
    restore(s, "cts20483-part.rle", "part");
    restore(s, "CTO902_cts20483-tool.rle", "tool");
    bcut(s, "result", "part", "tool");
    checkprops(s, "result", Prop::Area, Expected::Mass(1539.48));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn J1(s: &mut dyn Session) {
    restore(s, "CTO902_cts20503-part.rle", "part");
    restore(s, "CTO902_cts20503-tool.rle", "tool");
    bcut(s, "result", "part", "tool");
    checkprops(s, "result", Prop::Area, Expected::Mass(800270.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn J2(s: &mut dyn Session) {
    restore(s, "CTO902_cts20504-part.rle", "part");
    restore(s, "CTO902_cts20504-tool.rle", "tool");
    bcut(s, "result", "part", "tool");
    checkprops(s, "result", Prop::Area, Expected::Mass(910888.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn J3(s: &mut dyn Session) {
    restore(s, "CTO902_cts20505-part.rle", "part");
    restore(s, "CTO902_cts20505-tool.rle", "tool");
    bcut(s, "result", "part", "tool");
    checkprops(s, "result", Prop::Area, Expected::Mass(922509.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn J4(s: &mut dyn Session) {
    restore(s, "CTO902_cts20498-part.rle", "part");
    restore(s, "CTO902_cts20498-tool.rle", "tool");
    bcut(s, "result", "part", "tool");
    checkprops(s, "result", Prop::Area, Expected::Mass(243558.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn J5(s: &mut dyn Session) {
    restore(s, "CTO902_cts20489-part.rle", "part");
    restore(s, "CTO902_cts20489-tool.rle", "tool");
    bcut(s, "result", "part", "tool");
    checkprops(s, "result", Prop::Area, Expected::Mass(222577.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn J7(s: &mut dyn Session) {
    restore(s, "cts20537-part.rle", "part");
    restore(s, "CTO902_cts20537-tool.rle", "tool");
    bcut(s, "result", "part", "tool");
    checkprops(s, "result", Prop::Area, Expected::Mass(161860.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}
