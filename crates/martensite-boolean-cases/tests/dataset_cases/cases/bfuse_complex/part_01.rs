//! The `bfuse_complex` case bodies, part 1; each statement is one line of its script.
use super::*;

pub fn A1(s: &mut dyn Session) {
    restore(s, "CIN900_cts20hlk.rle", "a");
    restore(s, "CIN900_cts20hll.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(43552.5));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a", Prop::Area, "5956.63");
    kind_is(s, "a", ShapeEnum::Solid);
    printed_mass(s, "b", Prop::Area, "39293.5");
    kind_is(s, "b", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn A2(s: &mut dyn Session) {
    restore(s, "fra61083a.rle", "a");
    restore(s, "CIN900_fra61hlt.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(309.521));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a", Prop::Area, "299.521");
    kind_is(s, "a", ShapeEnum::Solid);
    printed_mass(s, "b", Prop::Area, "14.5");
    kind_is(s, "b", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn A3(s: &mut dyn Session) {
    restore(s, "CIN900_cts21hlo.rle", "a");
    restore(s, "CIN900_cts21hlp.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(164274.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a", Prop::Area, "128468");
    kind_is(s, "a", ShapeEnum::Compound);
    printed_mass(s, "b", Prop::Area, "78254.6");
    kind_is(s, "b", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn A4(s: &mut dyn Session) {
    restore(s, "f2", "b1");
    restore(s, "f6", "b2");
    bfuse(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(190.027));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn A5(s: &mut dyn Session) {
    restore(s, "f7", "b1");
    restore(s, "f3", "b2");
    bfuse(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(172.169));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn A6(s: &mut dyn Session) {
    restore(s, "f2", "b1");
    restore(s, "f5", "b2");
    bfuse(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(178.896));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn A7(s: &mut dyn Session) {
    restore(s, "f4", "b1");
    restore(s, "f5", "b2");
    bfuse(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(314.159));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn A8(s: &mut dyn Session) {
    restore(s, "f1", "b1");
    restore(s, "f5", "b2");
    bfuse(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(628.319));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn A9(s: &mut dyn Session) {
    restore(s, "f1", "b1");
    restore(s, "f4", "b2");
    bfuse(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(628.319));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn B1(s: &mut dyn Session) {
    restore(s, "f2", "b1");
    restore(s, "f3", "b2");
    bfuse(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(200.713));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b1", Prop::Area, "157.08");
    kind_is(s, "b1", ShapeEnum::Face);
    printed_mass(s, "b2", Prop::Area, "157.08");
    kind_is(s, "b2", ShapeEnum::Face);
    view_mass_is_nonzero(s, "result");
}

pub fn B2(s: &mut dyn Session) {
    restore(s, "f3", "b1");
    restore(s, "f4", "b2");
    bfuse(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(392.699));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b1", Prop::Area, "157.08");
    kind_is(s, "b1", ShapeEnum::Face);
    printed_mass(s, "b2", Prop::Area, "314.159");
    kind_is(s, "b2", ShapeEnum::Face);
    view_mass_is_nonzero(s, "result");
}

pub fn B3(s: &mut dyn Session) {
    restore(s, "f3", "b1");
    restore(s, "f5", "b2");
    bfuse(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(157.08));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b1", Prop::Area, "157.08");
    kind_is(s, "b1", ShapeEnum::Face);
    printed_mass(s, "b2", Prop::Area, "78.5398");
    kind_is(s, "b2", ShapeEnum::Face);
    view_mass_is_nonzero(s, "result");
}

pub fn B4(s: &mut dyn Session) {
    restore(s, "f1", "b1");
    restore(s, "f41", "b2");
    bfuse(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(628.319));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b1", Prop::Area, "628.319");
    kind_is(s, "b1", ShapeEnum::Face);
    printed_mass(s, "b2", Prop::Area, "314.159");
    kind_is(s, "b2", ShapeEnum::Face);
    view_mass_is_nonzero(s, "result");
}

pub fn B5(s: &mut dyn Session) {
    restore(s, "so1", "b1");
    restore(s, "so4", "b2");
    bfuse(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(942.478));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b1", Prop::Area, "942.478");
    kind_is(s, "b1", ShapeEnum::Solid);
    printed_mass(s, "b2", Prop::Area, "549.779");
    kind_is(s, "b2", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn B6(s: &mut dyn Session) {
    restore(s, "so1", "b1");
    restore(s, "so2", "b2");
    bfuse(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(942.478));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b1", Prop::Area, "942.478");
    kind_is(s, "b1", ShapeEnum::Solid);
    printed_mass(s, "b2", Prop::Area, "392.699");
    kind_is(s, "b2", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn B7(s: &mut dyn Session) {
    restore(s, "so2", "b1");
    restore(s, "so3", "b2");
    bfuse(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(458.149));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b1", Prop::Area, "392.699");
    kind_is(s, "b1", ShapeEnum::Solid);
    printed_mass(s, "b2", Prop::Area, "392.699");
    kind_is(s, "b2", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn B8(s: &mut dyn Session) {
    restore(s, "so4", "b1");
    restore(s, "so2", "b2");
    bfuse(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(743.615));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b1", Prop::Area, "549.779");
    kind_is(s, "b1", ShapeEnum::Solid);
    printed_mass(s, "b2", Prop::Area, "392.699");
    kind_is(s, "b2", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn B9(s: &mut dyn Session) {
    restore(s, "so5", "b1");
    restore(s, "so2", "b2");
    bfuse(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(430.878));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b1", Prop::Area, "198.863");
    kind_is(s, "b1", ShapeEnum::Solid);
    printed_mass(s, "b2", Prop::Area, "392.699");
    kind_is(s, "b2", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn C1(s: &mut dyn Session) {
    restore(s, "so3", "b1");
    restore(s, "so7", "b2");
    bfuse(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(445.059));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn C2(s: &mut dyn Session) {
    restore(s, "so2", "b1");
    restore(s, "so7", "b2");
    bfuse(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(466.035));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn C3(s: &mut dyn Session) {
    restore(s, "so2", "b1");
    restore(s, "so6", "b2");
    bfuse(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(469.85));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn C4(s: &mut dyn Session) {
    restore(s, "box.brep", "b");
    restore(s, "c2.brep", "c");
    bfuse(s, "result", "b", "c");
    checkprops(s, "result", Prop::Area, Expected::Mass(131766.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b", "c"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn C5(s: &mut dyn Session) {
    restore(s, "c2.brep", "c");
    restore(s, "box.brep", "b");
    bfuse(s, "result", "c", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(131766.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["c", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn C6(s: &mut dyn Session) {
    restore(s, "CTO900_pro7637_prism_oblong.rle", "p1");
    tscale(s, "p1", [0., 0., 0.], 1. * hooks::SCALE);
    restore(s, "CTO900_pro7637_box.rle", "p2");
    tscale(s, "p2", [0., 0., 0.], 1. * hooks::SCALE);
    bfuse(s, "result", "p2", "p1");
    checkprops(s, "result", Prop::Area, Expected::Mass(1.85425e+09));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["p2", "p1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn C7(s: &mut dyn Session) {
    restore(s, "CTO900_pro7637c_prism_oblong_dsg.rle", "p1");
    tscale(s, "p1", [0., 0., 0.], 1. * hooks::SCALE);
    restore(s, "CTO900_pro7637c_box_dsg.rle", "p2");
    tscale(s, "p2", [0., 0., 0.], 1. * hooks::SCALE);
    bfuse(s, "result", "p2", "p1");
    checkprops(s, "result", Prop::Area, Expected::Mass(1.85425e+09));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["p2", "p1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn C8(s: &mut dyn Session) {
    restore(s, "CTO900_cts40185_top.rle", "p1");
    tscale(s, "p1", [0., 0., 0.], 1. * hooks::SCALE);
    restore(s, "cts40185_ailette.rle", "p2");
    tscale(s, "p2", [0., 0., 0.], 1. * hooks::SCALE);
    bfuse(s, "result", "p2", "p1");
    checkprops(s, "result", Prop::Area, Expected::Mass(74.7286));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["p2", "p1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn C9(s: &mut dyn Session) {
    restore(s, "CTO900_cts16184a.rle", "a");
    restore(s, "cts16184b.brep", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(0.0892544));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn D2(s: &mut dyn Session) {
    restore(s, "ger60039a.brep", "a");
    fsameparameter(s, "a");
    restore(s, "CTO900_ger60039b.rle", "b");
    bfuse(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(18154.6));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a", Prop::Area, "16938.5");
    kind_is(s, "a", ShapeEnum::Compound);
    printed_mass(s, "b", Prop::Area, "5653.2");
    kind_is(s, "b", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}
