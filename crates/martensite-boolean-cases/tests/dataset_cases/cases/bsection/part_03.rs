//! The `bsection` case bodies, part 3; each statement is one line of its script.
use super::*;

pub fn I7(s: &mut dyn Session) {
    restore(s, "GEN510-3681_nofog.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(38.4985));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn I9(s: &mut dyn Session) {
    restore(s, "GEN525_3367_1.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(356.984));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn J1(s: &mut dyn Session) {
    restore(s, "GEN525_3367_1_nofog.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(356.984));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn J2(s: &mut dyn Session) {
    restore(s, "GEN525_3367_2.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(356.984));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn J3(s: &mut dyn Session) {
    restore(s, "GEN525_3367_2_nofog.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(356.984));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn J4(s: &mut dyn Session) {
    restore(s, "GEN525_3403.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(1093.37));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn J5(s: &mut dyn Session) {
    restore(s, "GEN525_3403_nofog.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(1093.37));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn J6(s: &mut dyn Session) {
    restore(s, "GEN539.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(2225.71));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn J7(s: &mut dyn Session) {
    restore(s, "GEN539_nofog.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(2224.79));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn J8(s: &mut dyn Session) {
    restore(s, "GEN585.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(2225.7));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn J9(s: &mut dyn Session) {
    restore(s, "GEN585_nofog.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(2224.79));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn K1(s: &mut dyn Session) {
    restore(s, "GEN594.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(30.8671));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn K2(s: &mut dyn Session) {
    restore(s, "GEN594_nofog.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(30.8671));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn K3(s: &mut dyn Session) {
    restore(s, "GEN605.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(8.41959));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn K4(s: &mut dyn Session) {
    restore(s, "GEN605_nofog.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(8.41959));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn K5(s: &mut dyn Session) {
    restore(s, "GEN664.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(718.013));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn K6(s: &mut dyn Session) {
    restore(s, "GEN664_nofog.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(718.013));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn K7(s: &mut dyn Session) {
    restore(s, "GEN676.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(88.9434));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn K8(s: &mut dyn Session) {
    restore(s, "GEN676_nofog.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(88.9434));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn L3(s: &mut dyn Session) {
    restore(s, "GEN745.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(96.7669));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn L4(s: &mut dyn Session) {
    restore(s, "GEN745_nofog.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(96.7669));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn L5(s: &mut dyn Session) {
    restore(s, "GEN746.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(556.6));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn L6(s: &mut dyn Session) {
    restore(s, "GEN746_nofog.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(556.6));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn L7(s: &mut dyn Session) {
    restore(s, "GEN747.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(72.4212));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn L8(s: &mut dyn Session) {
    restore(s, "GEN747_nofog.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(72.4222));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn L9(s: &mut dyn Session) {
    restore(s, "GEN757.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(683.183));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn M1(s: &mut dyn Session) {
    restore(s, "GEN757_nofog.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(683.183));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn M2(s: &mut dyn Session) {
    restore(s, "GEN758.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(22.2575));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn M4(s: &mut dyn Session) {
    restore(s, "GEN776.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(226.488));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn M5(s: &mut dyn Session) {
    restore(s, "GEN776_nofog.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(226.487));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn M7(s: &mut dyn Session) {
    restore(s, "GEN778_nofog.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(456.707));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn N8(s: &mut dyn Session) {
    restore(s, "CIN900_cts20hlk.rle", "a");
    restore(s, "CIN900_cts20hll.rle", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(119.639));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn N9(s: &mut dyn Session) {
    restore(s, "fra61083a.rle", "a");
    restore(s, "CIN900_fra61hlt.rle", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(10.));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn O1(s: &mut dyn Session) {
    restore(s, "pro12053a.rle", "a");
    restore(s, "CIN900_pro11hlu.rle", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(132.564));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn O2(s: &mut dyn Session) {
    restore(s, "pro14150a.rle", "a");
    restore(s, "pro14150b.rle", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(267.619));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn O3(s: &mut dyn Session) {
    restore(s, "pro14151a.rle", "a");
    restore(s, "CFI_pro14fjq.rle", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(49.9609));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}
