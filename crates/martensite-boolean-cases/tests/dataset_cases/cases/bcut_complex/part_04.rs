//! The `bcut_complex` case bodies, part 4; each statement is one line of its script.
use super::*;

pub fn Q1(s: &mut dyn Session) {
    restore(s, "buc60290a.rle", "sol1");
    restore(s, "buc60290b.rle", "sol2");
    bcut(s, "result", "sol2", "sol1");
    checkprops(s, "result", Prop::Area, Expected::Mass(1.06741e+06));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["sol2", "sol1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn Q2(s: &mut dyn Session) {
    println!("PRO16389");
    println!("Cut");
    println!();
    restore(s, "pro16389a.rle", "base");
    restore(s, "pro16389b.rle", "prism");
    bcut(s, "result", "base", "prism");
    checkprops(s, "result", Prop::Area, Expected::Mass(312467.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["base", "prism"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn Q3(s: &mut dyn Session) {
    println!("CTS20196");
    println!();
    restore(s, "CFE900_cts20gan.rle", "base");
    restore(s, "cts20196_face.rle", "cont");
    prism(s, "prism", "cont", [0., 20., 0.]);
    bcut(s, "result", "base", "prism");
    checkprops(s, "result", Prop::Area, Expected::Mass(250288.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["base", "prism"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn Q8(s: &mut dyn Session) {
    println!("PRO16389");
    println!("Cut");
    println!();
    restore(s, "pro16389a.rle", "base");
    restore(s, "pro16389b.rle", "prism");
    bcut(s, "result", "base", "prism");
    checkprops(s, "result", Prop::Area, Expected::Mass(312467.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["base", "prism"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn Q9(s: &mut dyn Session) {
    restore(s, "CTO907_topo001.brep", "m");
    explode(s, "m", None);
    compound(s, &["m_2", "m_3", "m_4", "m_5"], "c1");
    bcut(s, "result", "m_1", "c1");
    checkprops(s, "result", Prop::Area, Expected::Mass(57618.4));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["m_1", "c1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "m_1", Prop::Area, "60000");
    kind_is(s, "m_1", ShapeEnum::Solid);
    printed_mass(s, "c1", Prop::Area, "52409.2");
    kind_is(s, "c1", ShapeEnum::Compound);
    part_count(s, "m", None, 5);
    view_mass_is_nonzero(s, "result");
}
