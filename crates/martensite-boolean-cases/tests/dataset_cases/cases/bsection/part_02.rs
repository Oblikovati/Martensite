//! The `bsection` case bodies, part 2; each statement is one line of its script.
use super::*;

pub fn E3(s: &mut dyn Session) {
    restore(s, "a44", "a");
    restore(s, "b44", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(4.91577));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn E4(s: &mut dyn Session) {
    restore(s, "CIN001_a45.rle", "a");
    restore(s, "CIN001_b45.rle", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(0.374089));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn E5(s: &mut dyn Session) {
    restore(s, "a46", "a");
    restore(s, "b46", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(0.278859));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn E6(s: &mut dyn Session) {
    restore(s, "a47", "a");
    restore(s, "b27", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(7.94247));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn E7(s: &mut dyn Session) {
    restore(s, "a48", "a");
    restore(s, "b48", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(71.2757));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn E8(s: &mut dyn Session) {
    restore(s, "a49", "a");
    restore(s, "b49", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(122.155));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn E9(s: &mut dyn Session) {
    restore(s, "a60", "a");
    restore(s, "b60_bsect", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(1.64804));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn F1(s: &mut dyn Session) {
    restore(s, "a51", "a");
    restore(s, "b51", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(9.94056));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn F2(s: &mut dyn Session) {
    restore(s, "a52", "a");
    restore(s, "b52", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(164.908));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn F3(s: &mut dyn Session) {
    restore(s, "a53", "a");
    restore(s, "a47", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(15.2808));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn F4(s: &mut dyn Session) {
    restore(s, "a54", "a");
    restore(s, "b54", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(3872.37));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn F5(s: &mut dyn Session) {
    restore(s, "a55", "a");
    restore(s, "b55", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(15.7507));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn F6(s: &mut dyn Session) {
    restore(s, "a57_bsect", "a");
    restore(s, "b57_bsect", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(117.749));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn F7(s: &mut dyn Session) {
    restore(s, "a58_bsect", "a");
    restore(s, "b58_bsect", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(3.46962));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn F8(s: &mut dyn Session) {
    restore(s, "a59", "a");
    restore(s, "b59", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(23.7344));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn F9(s: &mut dyn Session) {
    restore(s, "CIN001_a61.rle", "a");
    restore(s, "b61", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(773.793));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn G1(s: &mut dyn Session) {
    restore(s, "a62", "a");
    restore(s, "b62_bsect", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(1.17784));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn G2(s: &mut dyn Session) {
    restore(s, "a64", "a");
    restore(s, "b64_bsect", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(1.58211));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn G3(s: &mut dyn Session) {
    restore(s, "a65", "a");
    restore(s, "b65", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(0.885026));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn G4(s: &mut dyn Session) {
    restore(s, "a66", "a");
    restore(s, "b66_bsect", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(20.2081));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn G5(s: &mut dyn Session) {
    restore(s, "a67", "a");
    restore(s, "b67", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(5.75983));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn G6(s: &mut dyn Session) {
    restore(s, "a68", "a");
    restore(s, "b68", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(10.448));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn G7(s: &mut dyn Session) {
    restore(s, "a69", "a");
    restore(s, "b69", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(0.00170879));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn G8(s: &mut dyn Session) {
    restore(s, "a71", "a");
    restore(s, "b71", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(10.8828));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn G9(s: &mut dyn Session) {
    restore(s, "a72", "a");
    restore(s, "b72", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(0.5));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn H1(s: &mut dyn Session) {
    restore(s, "a73", "a");
    restore(s, "b73", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(13.1087));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn H2(s: &mut dyn Session) {
    restore(s, "a74", "a");
    restore(s, "b74", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(94.0179));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn H3(s: &mut dyn Session) {
    restore(s, "BUGDATA_2293.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(285.489));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn H4(s: &mut dyn Session) {
    restore(s, "BUGDATA_2293_nofog.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(285.489));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn H5(s: &mut dyn Session) {
    restore(s, "BUGDATA_2381.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(691.399));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn H6(s: &mut dyn Session) {
    restore(s, "BUGDATA_2381_nofog.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(691.399));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn H9(s: &mut dyn Session) {
    restore(s, "GEN465.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(49.9609));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn I1(s: &mut dyn Session) {
    restore(s, "GEN465_nofog.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(49.9609));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn I2(s: &mut dyn Session) {
    restore(s, "GEN510-3454.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(8.10428));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn I3(s: &mut dyn Session) {
    restore(s, "GEN510-3454_nofog.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(8.14819));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}

pub fn I6(s: &mut dyn Session) {
    restore(s, "GEN510-3681.rle", "a");
    explode(s, "a", None);
    bsection(s, "result", "a_1", "a_2");
    checkprops(s, "result", Prop::Length, Expected::Mass(38.4985));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a_1", "a_2"], Prop::Length),
    );
}
