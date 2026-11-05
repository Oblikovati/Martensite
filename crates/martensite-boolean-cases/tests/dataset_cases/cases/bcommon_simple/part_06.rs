//! The `bcommon_simple` case bodies, part 6; each statement is one line of its script.
use super::*;

pub fn I5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [3., 3., 3.]);
    make_box(s, "b2", [0., 0., 0.], [1., 1., 1.]);
    ttranslate(s, "b2", [0., 3., 0.]);
    bcommon(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b1", Prop::Area, "54");
    kind_is(s, "b1", ShapeEnum::Solid);
    printed_mass(s, "b2", Prop::Area, "6");
    kind_is(s, "b2", ShapeEnum::Solid);
}

pub fn I6(s: &mut dyn Session) {
    make_box(s, "ba", [3., 3., 0.], [5., 7., 4.]);
    vertex(s, "v1", [7., 0., 2.]);
    vertex(s, "v2", [8., 0., 2.]);
    vertex(s, "v3", [8., 0., 3.]);
    vertex(s, "v4", [7., 0., 3.]);
    edge(s, "ea", "v1", "v2");
    edge(s, "eb", "v2", "v3");
    edge(s, "ec", "v3", "v4");
    edge(s, "ed", "v4", "v1");
    wire(s, "w", &["ea", "eb", "ec", "ed"]);
    mkplane(s, "f", "w");
    prism(s, "bb", "f", [0., 6., 0.]);
    bcommon(s, "result", "ba", "bb");
    checkprops(s, "result", Prop::Area, Expected::Mass(14.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "166");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "26");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn I7(s: &mut dyn Session) {
    make_box(s, "ba", [3., 3., 0.], [5., 7., 4.]);
    vertex(s, "v1", [7., 0., 2.]);
    vertex(s, "v2", [8., 0., 2.]);
    vertex(s, "v3", [8., 6., 2.]);
    vertex(s, "v4", [7., 6., 2.]);
    edge(s, "ea", "v1", "v2");
    edge(s, "eb", "v2", "v3");
    edge(s, "ec", "v3", "v4");
    edge(s, "ed", "v4", "v1");
    wire(s, "w", &["ea", "eb", "ec", "ed"]);
    mkplane(s, "f", "w");
    prism(s, "bb", "f", [0., 0., 1.]);
    bcommon(s, "result", "ba", "bb");
    checkprops(s, "result", Prop::Area, Expected::Mass(14.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "166");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "26");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn I8(s: &mut dyn Session) {
    make_box(s, "ba", [3., 3., 0.], [5., 7., 4.]);
    vertex(s, "v1", [8., 0., 2.]);
    vertex(s, "v2", [8., 6., 2.]);
    vertex(s, "v3", [8., 6., 3.]);
    vertex(s, "v4", [8., 0., 3.]);
    edge(s, "ea", "v1", "v2");
    edge(s, "eb", "v2", "v3");
    edge(s, "ec", "v3", "v4");
    edge(s, "ed", "v4", "v1");
    wire(s, "w", &["ea", "eb", "ec", "ed"]);
    mkplane(s, "f", "w");
    prism(s, "bb", "f", [-1., 0., 0.]);
    bcommon(s, "result", "ba", "bb");
    checkprops(s, "result", Prop::Area, Expected::Mass(14.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "166");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "26");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn I9(s: &mut dyn Session) {
    pcylinder(s, "cyla", 100., 400.);
    let circ_curve = circle_along([150., 0., 200.], [1., 0., 0.], 100.);
    mkedge(s, "e", &circ_curve);
    wire(s, "w", &["e"]);
    crate::support::topology_commands::mkplane(s, "f", "w", true);
    prism(s, "cylb", "f", [-300., 0., 0.]);
    bcommon(s, "result", "cyla", "cylb");
    checkprops(s, "result", Prop::Area, Expected::Mass(160000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["cyla", "cylb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "cyla", Prop::Area, "314159");
    kind_is(s, "cyla", ShapeEnum::Solid);
    printed_mass(s, "cylb", Prop::Area, "251327");
    kind_is(s, "cylb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn J1(s: &mut dyn Session) {
    println!("CTS21801");
    println!("common");
    println!();
    pcylinder(s, "c1", 20., 100.);
    pcylinder(s, "c2", 20., 100.);
    ttranslate(s, "c2", [0., 0., 50.]);
    bcommon(s, "result", "c1", "c2");
    checkprops(s, "result", Prop::Area, Expected::Mass(8796.46));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["c1", "c2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "c1", Prop::Area, "15079.6");
    kind_is(s, "c1", ShapeEnum::Solid);
    printed_mass(s, "c2", Prop::Area, "15079.6");
    kind_is(s, "c2", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn J2(s: &mut dyn Session) {
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    copy(s, "b", "c");
    pcylinder(s, "s", 2., 4.);
    ttranslate(s, "s", [5., 5., -2.]);
    bcut(s, "rr", "c", "s");
    explode(s, "rr", Some(ShapeEnum::Solid));
    bcommon(s, "result", "rr_1", "c");
    checkprops(s, "result", Prop::Area, Expected::Mass(625.133));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["rr_1", "c"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "c", Prop::Area, "600");
    kind_is(s, "c", ShapeEnum::Solid);
    printed_mass(s, "s", Prop::Area, "75.3982");
    kind_is(s, "s", ShapeEnum::Solid);
    printed_mass(s, "rr_1", Prop::Area, "625.133");
    kind_is(s, "rr_1", ShapeEnum::Solid);
    part_count(s, "rr", Some(ShapeEnum::Solid), 1);
    view_mass_is_nonzero(s, "result");
}
