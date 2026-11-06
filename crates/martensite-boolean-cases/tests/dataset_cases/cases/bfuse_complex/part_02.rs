//! The `bfuse_complex` case bodies, part 2; each statement is one line of its script.
use super::*;

pub fn D3(s: &mut dyn Session) {
    restore(s, "ger60041a.rle", "a");
    fsameparameter(s, "a");
    restore(s, "ger60041b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(17651.7));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a", Prop::Area, "16938.5");
    kind_is(s, "a", ShapeEnum::Compound);
    printed_mass(s, "b", Prop::Area, "1847.26");
    kind_is(s, "b", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn D5(s: &mut dyn Session) {
    restore(s, "CTO900_ksi0014a.rle", "a");
    restore(s, "ksi0014b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(223775.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn D6(s: &mut dyn Session) {
    restore(s, "CTO900_pro6050a.rle", "a");
    restore(s, "CTO900_pro6050b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(1.06905e+06));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn D7(s: &mut dyn Session) {
    restore(s, "CTO900_pro6965a.rle", "a");
    restore(s, "CTO900_pro6965b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(406922.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn D8(s: &mut dyn Session) {
    restore(s, "CTO900_pro11172a.rle", "a");
    restore(s, "CTO900_pro11172b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(681699.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn D9(s: &mut dyn Session) {
    restore(s, "CTO900_pro12559a.rle", "a");
    restore(s, "CTO900_pro12559b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(132237.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn E1(s: &mut dyn Session) {
    restore(s, "CTO900_fra11018a.rle", "a");
    restore(s, "CTO900_fra11018b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(25189.7));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn E5(s: &mut dyn Session) {
    restore(s, "CTO900_pro9476-part.rle", "part");
    restore(s, "CTO900_pro9476-tool.rle", "tool");
    bfuse(s, "result", "part", "tool");
    checkprops(s, "result", Prop::Area, Expected::Mass(1.83166e+06));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn E7(s: &mut dyn Session) {
    restore(s, "cts19304a.rle", "a");
    restore(s, "cts19304b.rle", "b");
    restore(s, "cts19304c.rle", "c");
    mkplane(s, "af", "a");
    mkplane(s, "bf", "b");
    mkplane(s, "cf", "c");
    bfuse(s, "resafbf", "af", "bf");
    bfuse(s, "result", "resafbf", "cf");
    checkprops(s, "result", Prop::Area, Expected::Mass(12677.1));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["resafbf", "cf"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "af", Prop::Area, "3963.63");
    kind_is(s, "af", ShapeEnum::Face);
    printed_mass(s, "bf", Prop::Area, "7496.52");
    kind_is(s, "bf", ShapeEnum::Face);
    printed_mass(s, "resafbf", Prop::Area, "11460.2");
    kind_is(s, "resafbf", ShapeEnum::Compound);
    printed_mass(s, "cf", Prop::Area, "9492.35");
    kind_is(s, "cf", ShapeEnum::Face);
    view_mass_is_nonzero(s, "result");
}

pub fn E9(s: &mut dyn Session) {
    restore(s, "CTO900_pro7934a.rle", "a");
    restore(s, "CTO900_pro7934b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(201978.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn F3(s: &mut dyn Session) {
    restore(s, "CTO900_pro6110a.rle", "a");
    restore(s, "CTO900_pro6110b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(1.7431e+06));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn F4(s: &mut dyn Session) {
    restore(s, "CTO900_pro10494b.rle", "b");
    restore(s, "CTO900_pro10494c.rle", "c");
    bfuse(s, "result", "b", "c");
    checkprops(s, "result", Prop::Area, Expected::Mass(152689.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b", "c"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn F5(s: &mut dyn Session) {
    restore(s, "CTO900_pro10658a.rle", "a");
    restore(s, "pro10658b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkshape_named(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(8231.06));
    checkprops(s, "result", Prop::Volume, Expected::Mass(30472.5));
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Wires(9),
            NbCount::Faces(9),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a", Prop::Area, "4609.64");
    kind_is(s, "a", ShapeEnum::Solid);
    printed_mass(s, "b", Prop::Area, "4027.15");
    kind_is(s, "b", ShapeEnum::Solid);
}

pub fn F9(s: &mut dyn Session) {
    restore(s, "CTO900_cts20124a.rle", "a");
    restore(s, "CTO900_cts20124b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(49957.7));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn G2(s: &mut dyn Session) {
    restore(s, "CTO900_cts21180c.rle", "part");
    restore(s, "CTO900_cts21180d.rle", "tool");
    updatetolerance(s, "tool", true);
    bfuse(s, "result", "part", "tool");
    checkprops(s, "result", Prop::Area, Expected::Mass(671652.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["part", "tool"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "part", Prop::Area, "667590");
    kind_is(s, "part", ShapeEnum::Compound);
    printed_mass(s, "tool", Prop::Area, "16823");
    kind_is(s, "tool", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn G3(s: &mut dyn Session) {
    restore(s, "CTO900_cts20124a.rle", "a");
    restore(s, "CTO900_cts20124b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(49957.7));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn G4(s: &mut dyn Session) {
    restore(s, "CTO901_cts20171_base.rle", "base");
    restore(s, "CTO901_cts20171_outil.rle", "outil");
    bfuse(s, "result", "base", "outil");
    checkprops(s, "result", Prop::Area, Expected::Mass(47886.5));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["base", "outil"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn G5(s: &mut dyn Session) {
    restore(s, "CTO901_cts20204_base.rle", "base");
    restore(s, "CTO901_cts20204_outil.rle", "outil");
    bfuse(s, "result", "base", "outil");
    checkprops(s, "result", Prop::Area, Expected::Mass(541480.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["base", "outil"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn G6(s: &mut dyn Session) {
    restore(s, "CTO901_cts20205-base.rle", "base");
    restore(s, "CTO901_cts20205-outil.rle", "outil");
    bfuse(s, "result", "base", "outil");
    checkprops(s, "result", Prop::Area, Expected::Mass(542196.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["base", "outil"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn G7(s: &mut dyn Session) {
    restore(s, "CTO901_cts20216_base.rle", "base");
    restore(s, "CTO901_cts20216_outil.rle", "outil");
    bfuse(s, "result", "base", "outil");
    checkprops(s, "result", Prop::Area, Expected::Mass(2005.99));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["base", "outil"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn G8(s: &mut dyn Session) {
    restore(s, "CTO901_cts20222_base.rle", "base");
    restore(s, "CTO901_cts20222_outil.rle", "outil");
    bfuse(s, "result", "base", "outil");
    checkprops(s, "result", Prop::Area, Expected::Mass(3342.49));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["base", "outil"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn G9(s: &mut dyn Session) {
    restore(s, "CTO902_cts20452a.rle", "a");
    restore(s, "CTO902_cts20452b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(538530.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn H1(s: &mut dyn Session) {
    restore(s, "CTO902_cts20453b.rle", "b");
    restore(s, "cts20453a.rle", "a");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(1.89512e+06));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn H2(s: &mut dyn Session) {
    restore(s, "CTO902_cts20454a.rle", "a");
    restore(s, "cts20454b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(978396.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn H3(s: &mut dyn Session) {
    restore(s, "CTO902_cts20457a.rle", "a");
    restore(s, "cts20457b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(26347.6));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn H4(s: &mut dyn Session) {
    restore(s, "CTO902_cts20458a.rle", "a");
    restore(s, "CTO902_cts20458b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(94390.1));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn H5(s: &mut dyn Session) {
    restore(s, "CTO902_cts20482a.rle", "a");
    restore(s, "CTO902_cts20482b.rle", "b");
    restore(s, "CTO902_cts20482c.rle", "c");
    bfuse(s, "result", "a", "b");
    bfuse(s, "result", "result", "c");
    checkprops(s, "result", Prop::Area, Expected::Mass(189131.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["result", "c"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a", Prop::Area, "175255");
    kind_is(s, "a", ShapeEnum::Solid);
    printed_mass(s, "b", Prop::Area, "19435.5");
    kind_is(s, "b", ShapeEnum::Solid);
    printed_mass(s, "result", Prop::Area, "189131");
    kind_is(s, "result", ShapeEnum::Compound);
    printed_mass(s, "c", Prop::Area, "24037");
    kind_is(s, "c", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn H6(s: &mut dyn Session) {
    restore(s, "CTO902_cts20484a.rle", "a");
    restore(s, "cts20484-tool2.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(125777.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn H7(s: &mut dyn Session) {
    restore(s, "CTO902_cts20486a.rle", "a");
    restore(s, "cts20486b.rle", "b");
    restore(s, "cts20486c.rle", "c");
    bfuse(s, "result", "a", "b");
    bfuse(s, "result", "a", "c");
    checkprops(s, "result", Prop::Area, Expected::Mass(168515.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "c"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a", Prop::Area, "160000");
    kind_is(s, "a", ShapeEnum::Solid);
    printed_mass(s, "b", Prop::Area, "15079.6");
    kind_is(s, "b", ShapeEnum::Solid);
    printed_mass(s, "c", Prop::Area, "21646.3");
    kind_is(s, "c", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}
