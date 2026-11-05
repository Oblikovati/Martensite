//! The `bcommon_simple` case bodies, part 3; each statement is one line of its script.
use super::*;

pub fn E7(s: &mut dyn Session) {
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
    vertex(s, "v5", [2., 0., 4.]);
    vertex(s, "v6", [3., 0., 4.]);
    vertex(s, "v7", [3., 3., 4.]);
    vertex(s, "v8", [2., 3., 4.]);
    edge(s, "e5", "v5", "v6");
    edge(s, "e6", "v6", "v7");
    edge(s, "e7", "v7", "v8");
    edge(s, "e8", "v8", "v5");
    wire(s, "wb", &["e5", "e6", "e7", "e8"]);
    mkplane(s, "fb", "wb");
    prism(s, "bb", "fb", [0., 0., -4.]);
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

pub fn E8(s: &mut dyn Session) {
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
    wire(s, "wb", &["e8", "e7", "e6", "e5"]);
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

pub fn E9(s: &mut dyn Session) {
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
    vertex(s, "v6", [2., 3., 0.]);
    vertex(s, "v7", [2., 3., 4.]);
    vertex(s, "v8", [2., 0., 4.]);
    edge(s, "e5", "v5", "v6");
    edge(s, "e6", "v6", "v7");
    edge(s, "e7", "v7", "v8");
    edge(s, "e8", "v8", "v5");
    wire(s, "wb", &["e5", "e6", "e7", "e8"]);
    mkplane(s, "fb", "wb");
    prism(s, "bb", "fb", [1., 0., 0.]);
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

pub fn F1(s: &mut dyn Session) {
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
    vertex(s, "v5", [2., 1., 0.]);
    vertex(s, "v6", [3., 1., 0.]);
    vertex(s, "v7", [3., 1., 4.]);
    vertex(s, "v8", [2., 1., 4.]);
    edge(s, "e5", "v5", "v6");
    edge(s, "e6", "v6", "v7");
    edge(s, "e7", "v7", "v8");
    edge(s, "e8", "v8", "v5");
    wire(s, "wb", &["e5", "e6", "e7", "e8"]);
    mkplane(s, "fb", "wb");
    prism(s, "bb", "fb", [0., 2., 0.]);
    bcommon(s, "result", "ba", "bb");
    checkprops(s, "result", Prop::Area, Expected::Mass(18.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "40");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "28");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn F2(s: &mut dyn Session) {
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
    vertex(s, "v5", [2., 3., 0.]);
    vertex(s, "v6", [3., 3., 0.]);
    vertex(s, "v7", [3., 3., 4.]);
    vertex(s, "v8", [2., 3., 4.]);
    edge(s, "e5", "v5", "v6");
    edge(s, "e6", "v6", "v7");
    edge(s, "e7", "v7", "v8");
    edge(s, "e8", "v8", "v5");
    wire(s, "wb", &["e5", "e6", "e7", "e8"]);
    mkplane(s, "fb", "wb");
    prism(s, "bb", "fb", [0., -2., 0.]);
    bcommon(s, "result", "ba", "bb");
    checkprops(s, "result", Prop::Area, Expected::Mass(18.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "40");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "28");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn F3(s: &mut dyn Session) {
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
    vertex(s, "v5", [3., 1., 0.]);
    vertex(s, "v6", [3., 3., 0.]);
    vertex(s, "v7", [3., 3., 4.]);
    vertex(s, "v8", [3., 1., 4.]);
    edge(s, "e5", "v5", "v6");
    edge(s, "e6", "v6", "v7");
    edge(s, "e7", "v7", "v8");
    edge(s, "e8", "v8", "v5");
    wire(s, "wb", &["e5", "e6", "e7", "e8"]);
    mkplane(s, "fb", "wb");
    prism(s, "bb", "fb", [-1., 0., 0.]);
    bcommon(s, "result", "ba", "bb");
    checkprops(s, "result", Prop::Area, Expected::Mass(18.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "40");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "28");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn F4(s: &mut dyn Session) {
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
    vertex(s, "v5", [2., 2., 0.]);
    vertex(s, "v6", [3., 2., 0.]);
    vertex(s, "v7", [3., 4., 0.]);
    vertex(s, "v8", [2., 4., 0.]);
    edge(s, "e5", "v5", "v6");
    edge(s, "e6", "v6", "v7");
    edge(s, "e7", "v7", "v8");
    edge(s, "e8", "v8", "v5");
    wire(s, "wb", &["e5", "e6", "e7", "e8"]);
    mkplane(s, "fb", "wb");
    prism(s, "bb", "fb", [0., 0., 2.]);
    bcommon(s, "result", "ba", "bb");
    checkprops(s, "result", Prop::Area, Expected::Mass(16.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "40");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "16");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn F5(s: &mut dyn Session) {
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
    vertex(s, "v5", [2., 2., 0.]);
    vertex(s, "v6", [3., 2., 0.]);
    vertex(s, "v7", [3., 2., 2.]);
    vertex(s, "v8", [2., 2., 2.]);
    edge(s, "e5", "v5", "v6");
    edge(s, "e6", "v6", "v7");
    edge(s, "e7", "v7", "v8");
    edge(s, "e8", "v8", "v5");
    wire(s, "wb", &["e5", "e6", "e7", "e8"]);
    mkplane(s, "fb", "wb");
    prism(s, "bb", "fb", [0., 2., 0.]);
    bcommon(s, "result", "ba", "bb");
    checkprops(s, "result", Prop::Area, Expected::Mass(16.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "40");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "16");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn F6(s: &mut dyn Session) {
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
    vertex(s, "v5", [2., 3., 0.]);
    vertex(s, "v6", [3., 3., 0.]);
    vertex(s, "v7", [3., 3., 2.]);
    vertex(s, "v8", [2., 3., 2.]);
    edge(s, "e5", "v5", "v6");
    edge(s, "e6", "v6", "v7");
    edge(s, "e7", "v7", "v8");
    edge(s, "e8", "v8", "v5");
    wire(s, "wb", &["e5", "e6", "e7", "e8"]);
    mkplane(s, "fb", "wb");
    prism(s, "bb", "fb", [0., -4., 0.]);
    bcommon(s, "result", "ba", "bb");
    checkprops(s, "result", Prop::Area, Expected::Mass(10.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "40");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "28");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn F7(s: &mut dyn Session) {
    let r = 2_f64.sqrt();
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
    vertex(s, "v5", [r, 0., 0.]);
    vertex(s, "v6", [r, 1., 0.]);
    vertex(s, "v7", [0., 1., 0.]);
    vertex(s, "v8", [0., 0., 0.]);
    edge(s, "e5", "v5", "v6");
    edge(s, "e6", "v6", "v7");
    edge(s, "e7", "v7", "v8");
    edge(s, "e8", "v8", "v5");
    wire(s, "wb", &["e5", "e6", "e7", "e8"]);
    mkplane(s, "fb", "wb");
    prism(s, "bb", "fb", [0., 0., 1.]);
    trotate(s, "bb", [0., 0., 0.], [0., 0., 1.], 45.);
    bcommon(s, "result", "ba", "bb");
    checkprops(s, "result", Prop::Area, Expected::Mass(4.41421));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "6");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "7.65685");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn F8(s: &mut dyn Session) {
    let r = 2_f64.sqrt();
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
    vertex(s, "v5", [r, 0., 0.]);
    vertex(s, "v6", [r, r / 2., 0.]);
    vertex(s, "v7", [0., r / 2., 0.]);
    vertex(s, "v8", [0., 0., 0.]);
    edge(s, "e5", "v5", "v6");
    edge(s, "e6", "v6", "v7");
    edge(s, "e7", "v7", "v8");
    edge(s, "e8", "v8", "v5");
    wire(s, "wb", &["e5", "e6", "e7", "e8"]);
    mkplane(s, "fb", "wb");
    prism(s, "bb", "fb", [0., 0., 1.]);
    trotate(s, "bb", [0., 0., 0.], [0., 0., 1.], 45.);
    bcommon(s, "result", "ba", "bb");
    checkprops(s, "result", Prop::Area, Expected::Mass(4.41421));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "6");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "6.24264");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}
