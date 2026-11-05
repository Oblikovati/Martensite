//! The `bcommon_simple` case bodies, part 2; each statement is one line of its script.
use super::*;

pub fn C8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    let r = 2_f64.sqrt();
    make_box(s, "b2", [0., 0., 0.], [r, r / 2., 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    bcommon(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(4.41421));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b1", Prop::Area, "6");
    kind_is(s, "b1", ShapeEnum::Solid);
    printed_mass(s, "b2", Prop::Area, "6.24264");
    kind_is(s, "b2", ShapeEnum::Solid);
    rotation_is(s, "b2", 45.0);
    view_mass_is_nonzero(s, "result");
}

pub fn C9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [0., 0., 0.], [r, r, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    bcommon(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(2.91421));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b1", Prop::Area, "6");
    kind_is(s, "b1", ShapeEnum::Solid);
    printed_mass(s, "b2", Prop::Area, "3.82843");
    kind_is(s, "b2", ShapeEnum::Solid);
    rotation_is(s, "b2", 45.0);
    view_mass_is_nonzero(s, "result");
}

pub fn D1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    let r = 2_f64.sqrt();
    make_box(s, "b2", [0., 0., 0.], [r, 0.25, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    bcommon(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(3.61764));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b1", Prop::Area, "6");
    kind_is(s, "b1", ShapeEnum::Solid);
    printed_mass(s, "b2", Prop::Area, "4.03553");
    kind_is(s, "b2", ShapeEnum::Solid);
    rotation_is(s, "b2", 45.0);
    view_mass_is_nonzero(s, "result");
}

pub fn D2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    let r = 31_f64.sqrt();
    make_box(s, "b2", [0., 0., 0.], [r / 4., 0.25, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 34.73);
    bcommon(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(3.65032));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b1", Prop::Area, "6");
    kind_is(s, "b1", ShapeEnum::Solid);
    printed_mass(s, "b2", Prop::Area, "3.97985");
    kind_is(s, "b2", ShapeEnum::Solid);
    rotation_is(s, "b2", 34.73);
    view_mass_is_nonzero(s, "result");
}

pub fn D3(s: &mut dyn Session) {
    psphere(s, "s", 1.);
    make_box(s, "b", [0., 0., 0.], [1., 1., 1.]);
    bcommon(s, "result", "s", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(3.92699));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["s", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn D4(s: &mut dyn Session) {
    psphere(s, "s", 1.);
    make_box(s, "b", [0., 0., 0.], [1., 1., 1.]);
    bcommon(s, "result", "b", "s");
    checkprops(s, "result", Prop::Area, Expected::Mass(3.92699));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b", "s"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn D5(s: &mut dyn Session) {
    psphere(s, "s", 1.);
    trotate(s, "s", [0., 0., 0.], [0., 0., 1.], -90.);
    trotate(s, "s", [0., 0., 0.], [0., 1., 0.], -45.);
    make_box(s, "b", [0., 0., 0.], [1., 1., 1.]);
    bcommon(s, "result", "s", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(3.92699));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["s", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn D6(s: &mut dyn Session) {
    psphere(s, "s", 1.);
    trotate(s, "s", [0., 0., 0.], [0., 0., 1.], -90.);
    trotate(s, "s", [0., 0., 0.], [0., 1., 0.], -45.);
    make_box(s, "b", [0., 0., 0.], [1., 1., 1.]);
    bcommon(s, "result", "b", "s");
    checkprops(s, "result", Prop::Area, Expected::Mass(3.92699));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b", "s"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn D7(s: &mut dyn Session) {
    psphere(s, "s", 1.);
    make_box(s, "b", [0., 0., 0.], [1., 1., 1.]);
    trotate(s, "b", [0., 0., 1.], [0., 1., 0.], 90.);
    bcommon(s, "result", "s", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(3.92699));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["s", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn D8(s: &mut dyn Session) {
    psphere(s, "s", 1.);
    make_box(s, "b", [0., 0., 0.], [1., 1., 1.]);
    trotate(s, "b", [0., 0., 1.], [0., 1., 0.], 90.);
    bcommon(s, "result", "b", "s");
    checkprops(s, "result", Prop::Area, Expected::Mass(3.92699));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b", "s"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn D9(s: &mut dyn Session) {
    vertex(s, "v1", [0., 0., 0.]);
    vertex(s, "v2", [1., 0., 0.]);
    vertex(s, "v3", [1., 1., 0.]);
    vertex(s, "v4", [0., 1., 0.]);
    edge(s, "e1", "v1", "v2");
    edge(s, "e2", "v2", "v3");
    edge(s, "e3", "v3", "v4");
    edge(s, "e4", "v4", "v1");
    wire(s, "wa", &["e1", "e2", "e3", "e4"]);
    mkplane(s, "fa", "wa");
    treverse(s, "fa");
    prism(s, "ba", "fa", [0., 0., 1.]);
    vertex(s, "v5", [0., 0., 1.]);
    vertex(s, "v6", [1., 0., 1.]);
    vertex(s, "v7", [1., 1., 1.]);
    vertex(s, "v8", [0., 1., 1.]);
    edge(s, "e5", "v5", "v6");
    edge(s, "e6", "v6", "v7");
    edge(s, "e7", "v7", "v8");
    edge(s, "e8", "v8", "v5");
    wire(s, "wb", &["e5", "e6", "e7", "e8"]);
    mkplane(s, "fb", "wb");
    prism(s, "bb", "fb", [0., 0., -1.]);
    bcommon(s, "result", "ba", "bb");
    checkprops(s, "result", Prop::Area, Expected::Mass(6.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "6");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "6");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn E1(s: &mut dyn Session) {
    vertex(s, "v1", [0., 0., 0.]);
    vertex(s, "v2", [1., 0., 0.]);
    vertex(s, "v3", [1., 1., 0.]);
    vertex(s, "v4", [0., 1., 0.]);
    edge(s, "e1", "v1", "v2");
    edge(s, "e2", "v2", "v3");
    edge(s, "e3", "v3", "v4");
    edge(s, "e4", "v4", "v1");
    wire(s, "wa", &["e1", "e2", "e3", "e4"]);
    mkplane(s, "fa", "wa");
    prism(s, "ba", "fa", [0., 0., 1.]);
    vertex(s, "v5", [0., 0., 0.]);
    vertex(s, "v6", [1., 0., 0.]);
    vertex(s, "v7", [1., 0.5, 0.]);
    vertex(s, "v8", [0., 0.5, 0.]);
    edge(s, "e5", "v5", "v6");
    edge(s, "e6", "v6", "v7");
    edge(s, "e7", "v7", "v8");
    edge(s, "e8", "v8", "v5");
    wire(s, "wb", &["e5", "e6", "e7", "e8"]);
    mkplane(s, "fb", "wb");
    prism(s, "bb", "fb", [0., 0., 1.]);
    bcommon(s, "result", "ba", "bb");
    checkprops(s, "result", Prop::Area, Expected::Mass(4.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "6");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "4");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn E2(s: &mut dyn Session) {
    vertex(s, "v1", [0., 0., 0.]);
    vertex(s, "v2", [1., 0., 0.]);
    vertex(s, "v3", [1., 1., 0.]);
    vertex(s, "v4", [0., 1., 0.]);
    edge(s, "e1", "v1", "v2");
    edge(s, "e2", "v2", "v3");
    edge(s, "e3", "v3", "v4");
    edge(s, "e4", "v4", "v1");
    wire(s, "wa", &["e1", "e2", "e3", "e4"]);
    mkplane(s, "fa", "wa");
    prism(s, "ba", "fa", [0., 0., 1.]);
    vertex(s, "v5", [0., 0., 1.]);
    vertex(s, "v6", [1., 0., 1.]);
    vertex(s, "v7", [1., 0.5, 1.]);
    vertex(s, "v8", [0., 0.5, 1.]);
    edge(s, "e5", "v5", "v6");
    edge(s, "e6", "v6", "v7");
    edge(s, "e7", "v7", "v8");
    edge(s, "e8", "v8", "v5");
    wire(s, "wb", &["e5", "e6", "e7", "e8"]);
    mkplane(s, "fb", "wb");
    prism(s, "bb", "fb", [0., 0., -1.]);
    bcommon(s, "result", "ba", "bb");
    checkprops(s, "result", Prop::Area, Expected::Mass(4.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "6");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "4");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn E3(s: &mut dyn Session) {
    vertex(s, "v1", [0., 0., 0.]);
    vertex(s, "v2", [2., 0., 0.]);
    vertex(s, "v3", [2., 2., 0.]);
    vertex(s, "v4", [0., 2., 0.]);
    edge(s, "e1", "v1", "v2");
    edge(s, "e2", "v2", "v3");
    edge(s, "e3", "v3", "v4");
    edge(s, "e4", "v4", "v1");
    wire(s, "wa", &["e1", "e2", "e3", "e4"]);
    mkplane(s, "fa", "wa");
    prism(s, "ba", "fa", [0., 0., 2.]);
    vertex(s, "v5", [0., 0., 0.]);
    vertex(s, "v6", [1., 0., 0.]);
    vertex(s, "v7", [1., 1., 0.]);
    vertex(s, "v8", [0., 1., 0.]);
    edge(s, "e5", "v5", "v6");
    edge(s, "e6", "v6", "v7");
    edge(s, "e7", "v7", "v8");
    edge(s, "e8", "v8", "v5");
    wire(s, "wb", &["e5", "e6", "e7", "e8"]);
    mkplane(s, "fb", "wb");
    prism(s, "bb", "fb", [0., 0., 1.]);
    bcommon(s, "result", "ba", "bb");
    checkprops(s, "result", Prop::Area, Expected::Mass(6.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "24");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "6");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn E4(s: &mut dyn Session) {
    vertex(s, "v1", [0., 0., 0.]);
    vertex(s, "v2", [2., 0., 0.]);
    vertex(s, "v3", [2., 2., 0.]);
    vertex(s, "v4", [0., 2., 0.]);
    edge(s, "e1", "v1", "v2");
    edge(s, "e2", "v2", "v3");
    edge(s, "e3", "v3", "v4");
    edge(s, "e4", "v4", "v1");
    wire(s, "wa", &["e1", "e2", "e3", "e4"]);
    mkplane(s, "fa", "wa");
    prism(s, "ba", "fa", [0., 0., 2.]);
    vertex(s, "v5", [0., 0., 1.]);
    vertex(s, "v6", [1., 0., 1.]);
    vertex(s, "v7", [1., 1., 1.]);
    vertex(s, "v8", [0., 1., 1.]);
    edge(s, "e5", "v5", "v6");
    edge(s, "e6", "v6", "v7");
    edge(s, "e7", "v7", "v8");
    edge(s, "e8", "v8", "v5");
    wire(s, "wb", &["e5", "e6", "e7", "e8"]);
    mkplane(s, "fb", "wb");
    prism(s, "bb", "fb", [0., 0., -1.]);
    bcommon(s, "result", "ba", "bb");
    checkprops(s, "result", Prop::Area, Expected::Mass(6.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "24");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "6");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn E5(s: &mut dyn Session) {
    vertex(s, "v1", [0., 0., 0.]);
    vertex(s, "v2", [2., 0., 0.]);
    vertex(s, "v3", [2., 2., 0.]);
    vertex(s, "v4", [0., 2., 0.]);
    edge(s, "e1", "v1", "v2");
    edge(s, "e2", "v2", "v3");
    edge(s, "e3", "v3", "v4");
    edge(s, "e4", "v4", "v1");
    wire(s, "wa", &["e1", "e2", "e3", "e4"]);
    mkplane(s, "fa", "wa");
    prism(s, "ba", "fa", [0., 0., 2.]);
    vertex(s, "v5", [0., 0., 0.]);
    vertex(s, "v6", [0., 1., 0.]);
    vertex(s, "v7", [1., 1., 0.]);
    vertex(s, "v8", [1., 0., 0.]);
    edge(s, "e5", "v5", "v6");
    edge(s, "e6", "v6", "v7");
    edge(s, "e7", "v7", "v8");
    edge(s, "e8", "v8", "v5");
    wire(s, "wb", &["e5", "e6", "e7", "e8"]);
    mkplane(s, "fb", "wb");
    prism(s, "bb", "fb", [0., 0., 1.]);
    bcommon(s, "result", "ba", "bb");
    checkprops(s, "result", Prop::Area, Expected::Mass(6.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "24");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "6");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn E6(s: &mut dyn Session) {
    vertex(s, "v1", [2., 2., 0.]);
    vertex(s, "v2", [4., 2., 0.]);
    vertex(s, "v3", [4., 4., 0.]);
    vertex(s, "v4", [2., 4., 0.]);
    edge(s, "e1", "v1", "v2");
    edge(s, "e2", "v2", "v3");
    edge(s, "e3", "v3", "v4");
    edge(s, "e4", "v4", "v1");
    wire(s, "wa", &["e1", "e2", "e3", "e4"]);
    mkplane(s, "fa", "wa");
    prism(s, "ba", "fa", [0., 0., 4.]);
    vertex(s, "v5", [2., 0., 0.]);
    vertex(s, "v6", [3., 0., 0.]);
    vertex(s, "v7", [3., 3., 0.]);
    vertex(s, "v8", [2., 3., 0.]);
    edge(s, "e5", "v5", "v6");
    edge(s, "e6", "v6", "v7");
    edge(s, "e7", "v7", "v8");
    edge(s, "e8", "v8", "v5");
    wire(s, "wb", &["e5", "e6", "e7", "e8"]);
    mkplane(s, "fb", "wb");
    prism(s, "bb", "fb", [0., 0., 4.]);
    bcommon(s, "result", "ba", "bb");
    checkprops(s, "result", Prop::Area, Expected::Mass(18.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "40");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "38");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}
