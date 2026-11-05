//! `bcommon_complex`: each statement is one line of its script.
use crate::support::{
    beyond_script::view_mass_is_nonzero,
    checkprops::{Expected, checkprops},
    checkview::{CheckView, checkview},
    one_step::bcommon,
    properties::Prop,
    restore::restore,
    session::Session,
};

pub fn A1(s: &mut dyn Session) {
    restore(s, "f2", "b1");
    restore(s, "f6", "b2");
    bcommon(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(45.5929));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn A2(s: &mut dyn Session) {
    restore(s, "f7", "b1");
    restore(s, "f3", "b2");
    bcommon(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(63.4508));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn A3(s: &mut dyn Session) {
    restore(s, "f2", "b1");
    restore(s, "f5", "b2");
    bcommon(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(56.7232));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn A4(s: &mut dyn Session) {
    restore(s, "f4", "b1");
    restore(s, "f5", "b2");
    bcommon(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(78.5398));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn A5(s: &mut dyn Session) {
    restore(s, "f1", "b1");
    restore(s, "f5", "b2");
    bcommon(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(78.5398));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn A6(s: &mut dyn Session) {
    restore(s, "f1", "b1");
    restore(s, "f4", "b2");
    bcommon(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(314.159));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn A7(s: &mut dyn Session) {
    restore(s, "f2", "b1");
    restore(s, "f3", "b2");
    bcommon(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(113.446));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn A8(s: &mut dyn Session) {
    restore(s, "f3", "b1");
    restore(s, "f4", "b2");
    bcommon(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(78.5398));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn A9(s: &mut dyn Session) {
    restore(s, "f3", "b1");
    restore(s, "f5", "b2");
    bcommon(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(78.5398));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn B1(s: &mut dyn Session) {
    restore(s, "f1", "b1");
    restore(s, "f41", "b2");
    bcommon(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(314.159));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn B2(s: &mut dyn Session) {
    restore(s, "so1", "b1");
    restore(s, "so4", "b2");
    bcommon(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(549.779));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn B3(s: &mut dyn Session) {
    restore(s, "so1", "b1");
    restore(s, "so2", "b2");
    bcommon(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(392.699));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn B4(s: &mut dyn Session) {
    restore(s, "so2", "b1");
    restore(s, "so3", "b2");
    bcommon(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(327.249));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn B5(s: &mut dyn Session) {
    restore(s, "so4", "b1");
    restore(s, "so2", "b2");
    bcommon(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(198.863));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn B6(s: &mut dyn Session) {
    restore(s, "so5", "b1");
    restore(s, "so2", "b2");
    bcommon(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(160.684));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn B7(s: &mut dyn Session) {
    restore(s, "so3", "b1");
    restore(s, "so7", "b2");
    bcommon(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(340.339));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn B8(s: &mut dyn Session) {
    restore(s, "so2", "b1");
    restore(s, "so7", "b2");
    bcommon(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(319.364));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn B9(s: &mut dyn Session) {
    restore(s, "so2", "b1");
    restore(s, "so6", "b2");
    bcommon(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(315.548));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn C2(s: &mut dyn Session) {
    restore(s, "CTO900_cts21453a.rle", "a");
    restore(s, "CTO900_cts21453b.rle", "b");
    bcommon(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(16681.4));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn C4(s: &mut dyn Session) {
    restore(s, "CTO902_cts20460a.rle", "a");
    restore(s, "CTO902_cts20460b.rle", "b");
    bcommon(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(25468.7));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn C5(s: &mut dyn Session) {
    restore(s, "CTO902_cts20339a.rle", "a");
    restore(s, "CTO902_cts20339b.rle", "b");
    bcommon(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(155403.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn C6(s: &mut dyn Session) {
    restore(s, "CTO902_cts20527-tool.rle", "a");
    restore(s, "CTO904_cts20527b.rle", "b");
    bcommon(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(23309.7));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn C7(s: &mut dyn Session) {
    restore(s, "buc60290a.rle", "sol1");
    restore(s, "buc60290b.rle", "sol2");
    bcommon(s, "result", "sol2", "sol1");
    checkprops(s, "result", Prop::Area, Expected::Mass(662100.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["sol2", "sol1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}
