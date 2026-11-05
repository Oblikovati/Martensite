//! The `bcut_complex` case bodies, part 1; each statement is one line of its script.
use super::*;

pub fn A2(s: &mut dyn Session) {
    restore(s, "CIN900_cts21hlq.rle", "a");
    restore(s, "CIN900_cts21hlr.rle", "b");
    bcut(s, "result", "a", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(330507.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn A3(s: &mut dyn Session) {
    restore(s, "f2", "b1");
    restore(s, "f6", "b2");
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(111.487));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn A4(s: &mut dyn Session) {
    restore(s, "f2", "b1");
    restore(s, "f6", "b2");
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(32.9469));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn A5(s: &mut dyn Session) {
    restore(s, "f7", "b1");
    restore(s, "f3", "b2");
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(15.0891));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn A6(s: &mut dyn Session) {
    restore(s, "f7", "b1");
    restore(s, "f3", "b2");
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(93.6289));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn A7(s: &mut dyn Session) {
    restore(s, "f2", "b1");
    restore(s, "f5", "b2");
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(100.356));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn A8(s: &mut dyn Session) {
    restore(s, "f2", "b1");
    restore(s, "f5", "b2");
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(21.8166));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn A9(s: &mut dyn Session) {
    restore(s, "f4", "b1");
    restore(s, "f5", "b2");
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(235.619));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn B1(s: &mut dyn Session) {
    restore(s, "f4", "b1");
    restore(s, "f5", "b2");
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
}

pub fn B2(s: &mut dyn Session) {
    restore(s, "f1", "b1");
    restore(s, "f5", "b2");
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(549.779));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn B3(s: &mut dyn Session) {
    restore(s, "f1", "b1");
    restore(s, "f5", "b2");
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
}

pub fn B4(s: &mut dyn Session) {
    restore(s, "f1", "b1");
    restore(s, "f4", "b2");
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(314.159));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn B5(s: &mut dyn Session) {
    restore(s, "f1", "b1");
    restore(s, "f4", "b2");
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
}

pub fn B6(s: &mut dyn Session) {
    restore(s, "f2", "b1");
    restore(s, "f3", "b2");
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(43.6332));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn B7(s: &mut dyn Session) {
    restore(s, "f2", "b1");
    restore(s, "f3", "b2");
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(43.6332));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn B8(s: &mut dyn Session) {
    restore(s, "f3", "b1");
    restore(s, "f4", "b2");
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(78.5398));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn B9(s: &mut dyn Session) {
    restore(s, "f3", "b1");
    restore(s, "f4", "b2");
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(235.619));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn C1(s: &mut dyn Session) {
    restore(s, "f3", "b1");
    restore(s, "f5", "b2");
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(78.5398));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn C2(s: &mut dyn Session) {
    restore(s, "f3", "b1");
    restore(s, "f5", "b2");
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
}

pub fn C3(s: &mut dyn Session) {
    restore(s, "f1", "b1");
    restore(s, "f41", "b2");
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(314.159));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn C4(s: &mut dyn Session) {
    restore(s, "f1", "b1");
    restore(s, "f41", "b2");
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
}

pub fn C5(s: &mut dyn Session) {
    restore(s, "so1", "b1");
    restore(s, "so4", "b2");
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(863.938));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn C6(s: &mut dyn Session) {
    restore(s, "so1", "b1");
    restore(s, "so4", "b2");
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
}

pub fn C7(s: &mut dyn Session) {
    restore(s, "so1", "b1");
    restore(s, "so2", "b2");
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(863.938));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn C8(s: &mut dyn Session) {
    restore(s, "so1", "b1");
    restore(s, "so2", "b2");
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
}

pub fn C9(s: &mut dyn Session) {
    restore(s, "so2", "b1");
    restore(s, "so3", "b2");
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(222.529));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn D1(s: &mut dyn Session) {
    restore(s, "so2", "b1");
    restore(s, "so3", "b2");
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(222.529));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn D2(s: &mut dyn Session) {
    restore(s, "so4", "b1");
    restore(s, "so2", "b2");
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(473.753));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn D3(s: &mut dyn Session) {
    restore(s, "so4", "b1");
    restore(s, "so2", "b2");
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(311.646));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn D4(s: &mut dyn Session) {
    restore(s, "so5", "b1");
    restore(s, "so2", "b2");
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(99.5976));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn D5(s: &mut dyn Session) {
    restore(s, "so5", "b1");
    restore(s, "so2", "b2");
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(378.518));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn D6(s: &mut dyn Session) {
    restore(s, "so3", "b1");
    restore(s, "so7", "b2");
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(209.44));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn D7(s: &mut dyn Session) {
    restore(s, "so3", "b1");
    restore(s, "so7", "b2");
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(209.44));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn D8(s: &mut dyn Session) {
    restore(s, "so2", "b1");
    restore(s, "so7", "b2");
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(282.913));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}
