//! The `bsection` case bodies, part 1; each statement is one line of its script.
use super::*;

pub fn A1(s: &mut dyn Session) {
    restore(s, "CIN001_a63.rle", "a");
    restore(s, "CIN001_b63.rle", "b");
    ttranslate(s, "a", [0., -0.0025400016456842422, 0.]);
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(225.961));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a", Prop::Area, "12893.8");
    kind_is(s, "a", ShapeEnum::Solid);
    printed_mass(s, "b", Prop::Area, "21376");
    kind_is(s, "b", ShapeEnum::Solid);
}

pub fn A2(s: &mut dyn Session) {
    restore(s, "CIN001_a10.rle", "a");
    restore(s, "CIN001_b10.rle", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(25.4961));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn A3(s: &mut dyn Session) {
    restore(s, "a2", "a");
    restore(s, "b2_bsect", "b");
    bsection_no_pcurves(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(25.1327));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b", "-n2d"], Prop::Length),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a", Prop::Area, "301.593");
    kind_is(s, "a", ShapeEnum::Solid);
    printed_mass(s, "b", Prop::Area, "201.062");
    kind_is(s, "b", ShapeEnum::Solid);
}

pub fn A4(s: &mut dyn Session) {
    restore(s, "a3", "a");
    restore(s, "b3", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(10.));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn A5(s: &mut dyn Session) {
    restore(s, "a4", "a");
    restore(s, "b4", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(125.664));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn A6(s: &mut dyn Session) {
    restore(s, "CIN001_a5.rle", "a");
    restore(s, "CIN001_b5.rle", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(15.8345));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn A7(s: &mut dyn Session) {
    restore(s, "a7", "a");
    restore(s, "b7", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(1.75));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn A8(s: &mut dyn Session) {
    restore(s, "CIN001_a8.rle", "a");
    restore(s, "b8", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(43.3411));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn B1(s: &mut dyn Session) {
    restore(s, "a11_bsect", "a");
    restore(s, "b11_bsect", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(1.21315));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn B2(s: &mut dyn Session) {
    restore(s, "a12", "a");
    restore(s, "b12", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(140.043));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn B3(s: &mut dyn Session) {
    restore(s, "a13", "a");
    restore(s, "b13_bsect", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(7.47641));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn B4(s: &mut dyn Session) {
    restore(s, "a14", "a");
    restore(s, "b14", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(10.6749));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn B6(s: &mut dyn Session) {
    println!("TODO #22911 ALL: Error : The command is not valid. The length is");
    println!("TODO #22911 ALL: Error : The length of result shape is");
    restore(s, "a16", "a");
    restore(s, "b16", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(76.0946));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "a", Prop::Area, "7109.2");
    kind_is(s, "a", ShapeEnum::Face);
    printed_mass(s, "b", Prop::Area, "625.135");
    kind_is(s, "b", ShapeEnum::Face);
}

pub fn B7(s: &mut dyn Session) {
    restore(s, "CIN001_a17.rle", "a");
    restore(s, "b17_bsect", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(10.));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn B8(s: &mut dyn Session) {
    restore(s, "a18", "a");
    restore(s, "b18", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(53.6115));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn B9(s: &mut dyn Session) {
    restore(s, "a19", "a");
    restore(s, "b19_bsect", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(0.392404));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn C1(s: &mut dyn Session) {
    restore(s, "a30", "a");
    restore(s, "b30", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(12.5664));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn C2(s: &mut dyn Session) {
    restore(s, "a23", "a");
    restore(s, "b23", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(51.7591));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn C4(s: &mut dyn Session) {
    restore(s, "a25", "a");
    restore(s, "b25", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(1.51691));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn C5(s: &mut dyn Session) {
    restore(s, "a26", "a");
    restore(s, "CIN001_b26.rle", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(251.672));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn C6(s: &mut dyn Session) {
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

pub fn C7(s: &mut dyn Session) {
    restore(s, "a28", "a");
    restore(s, "b28", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(176.127));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn C8(s: &mut dyn Session) {
    restore(s, "a29", "a");
    restore(s, "b29", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(6.23599));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn C9(s: &mut dyn Session) {
    restore(s, "a40", "a");
    restore(s, "b40", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(50.8667));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn D1(s: &mut dyn Session) {
    restore(s, "a31", "a");
    restore(s, "b31", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(17041.4));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn D2(s: &mut dyn Session) {
    restore(s, "a32", "a");
    restore(s, "b32", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(12.4984));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn D3(s: &mut dyn Session) {
    restore(s, "a33", "a");
    restore(s, "b33", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(2.00019));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn D4(s: &mut dyn Session) {
    restore(s, "a34", "a");
    restore(s, "b34", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(45.4729));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn D5(s: &mut dyn Session) {
    restore(s, "a35", "a");
    restore(s, "b35", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(15.0786));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn D6(s: &mut dyn Session) {
    restore(s, "a37", "a");
    restore(s, "b37", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(6.28319));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn D7(s: &mut dyn Session) {
    restore(s, "a38", "a");
    restore(s, "b38", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(8.24621));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn D8(s: &mut dyn Session) {
    restore(s, "a49", "a");
    restore(s, "b50", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(254.014));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn D9(s: &mut dyn Session) {
    restore(s, "a41", "a");
    restore(s, "b41", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(19.9279));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn E1(s: &mut dyn Session) {
    restore(s, "a42", "a");
    restore(s, "b42", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(2.99998));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}

pub fn E2(s: &mut dyn Session) {
    restore(s, "a43", "a");
    restore(s, "b43", "b");
    bsection(s, "result", "a", "b");
    checkprops(s, "result", Prop::Length, Expected::Mass(29.2833));
    checksection(s, "result");
    checkview(
        s,
        &CheckView::of("result").otherwise(&["a", "b"], Prop::Length),
    );
}
