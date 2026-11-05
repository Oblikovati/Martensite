//! The `bcommon_simple` case bodies, part 4; each statement is one line of its script.
use super::*;

pub fn F9(s: &mut dyn Session) {
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
    vertex(s, "v6", [r, 0., 1.]);
    vertex(s, "v7", [0., 0., 1.]);
    vertex(s, "v8", [0., 0., 0.]);
    edge(s, "e5", "v5", "v6");
    edge(s, "e6", "v6", "v7");
    edge(s, "e7", "v7", "v8");
    edge(s, "e8", "v8", "v5");
    wire(s, "wb", &["e5", "e6", "e7", "e8"]);
    mkplane(s, "fb", "wb");
    prism(s, "bb", "fb", [0., r / 2., 0.]);
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

pub fn G1(s: &mut dyn Session) {
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
    vertex(s, "v5", [0., r / 2., 0.]);
    vertex(s, "v6", [0., r / 2., 1.]);
    vertex(s, "v7", [0., 0., 1.]);
    vertex(s, "v8", [0., 0., 0.]);
    edge(s, "e5", "v5", "v6");
    edge(s, "e6", "v6", "v7");
    edge(s, "e7", "v7", "v8");
    edge(s, "e8", "v8", "v5");
    wire(s, "wb", &["e5", "e6", "e7", "e8"]);
    mkplane(s, "fb", "wb");
    prism(s, "bb", "fb", [r, 0., 0.]);
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

pub fn G2(s: &mut dyn Session) {
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
    vertex(s, "v6", [1., 1., 0.]);
    vertex(s, "v7", [1., 1., 1.]);
    vertex(s, "v8", [0., 0., 1.]);
    edge(s, "e5", "v5", "v6");
    edge(s, "e6", "v6", "v7");
    edge(s, "e7", "v7", "v8");
    edge(s, "e8", "v8", "v5");
    wire(s, "wb", &["e5", "e6", "e7", "e8"]);
    mkplane(s, "fb", "wb");
    prism(s, "bb", "fb", [-0.5, 0.5, 0.]);
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

pub fn G3(s: &mut dyn Session) {
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
    vertex(s, "v6", [1., 1., 0.]);
    vertex(s, "v7", [1., 1., 1.]);
    vertex(s, "v8", [0., 0., 1.]);
    edge(s, "e5", "v5", "v6");
    edge(s, "e6", "v6", "v7");
    edge(s, "e7", "v7", "v8");
    edge(s, "e8", "v8", "v5");
    wire(s, "wb", &["e5", "e6", "e7", "e8"]);
    mkplane(s, "fb", "wb");
    prism(s, "bb", "fb", [-1., 1., 0.]);
    bcommon(s, "result", "ba", "bb");
    checkprops(s, "result", Prop::Area, Expected::Mass(4.41421));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "6");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "9.65685");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn G4(s: &mut dyn Session) {
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
    vertex(s, "v5", [0., 0., 0.]);
    vertex(s, "v6", [1.5, 0., 0.]);
    vertex(s, "v7", [1.5, r / 2., 0.]);
    vertex(s, "v8", [0., r / 2., 0.]);
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
    printed_mass(s, "bb", Prop::Area, "6.53553");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn G5(s: &mut dyn Session) {
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
    vertex(s, "v5", [0., 0., 0.]);
    vertex(s, "v6", [1.5, 0., 0.]);
    vertex(s, "v7", [1.5, r / 2., 0.]);
    vertex(s, "v8", [0., r / 2., 0.]);
    edge(s, "e5", "v5", "v6");
    edge(s, "e6", "v6", "v7");
    edge(s, "e7", "v7", "v8");
    edge(s, "e8", "v8", "v5");
    wire(s, "wb", &["e8", "e7", "e6", "e5"]);
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
    printed_mass(s, "bb", Prop::Area, "6.53553");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn G6(s: &mut dyn Session) {
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
    vertex(s, "v5", [0., 0., 1.]);
    vertex(s, "v6", [1.5, 0., 1.]);
    vertex(s, "v7", [1.5, r / 2., 1.]);
    vertex(s, "v8", [0., r / 2., 1.]);
    edge(s, "e5", "v5", "v6");
    edge(s, "e6", "v6", "v7");
    edge(s, "e7", "v7", "v8");
    edge(s, "e8", "v8", "v5");
    wire(s, "wb", &["e5", "e6", "e7", "e8"]);
    mkplane(s, "fb", "wb");
    prism(s, "bb", "fb", [0., 0., -1.]);
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
    printed_mass(s, "bb", Prop::Area, "6.53553");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn G7(s: &mut dyn Session) {
    let r = 5_f64.sqrt();
    let k = 1. / r;
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
    vertex(s, "v6", [r / 2., 0., 0.]);
    vertex(s, "v7", [r / 2., k, 0.]);
    vertex(s, "v8", [0., k, 0.]);
    edge(s, "e5", "v5", "v6");
    edge(s, "e6", "v6", "v7");
    edge(s, "e7", "v7", "v8");
    edge(s, "e8", "v8", "v5");
    wire(s, "wb", &["e5", "e6", "e7", "e8"]);
    mkplane(s, "fb", "wb");
    prism(s, "bb", "fb", [0., 0., 1.]);
    trotate(s, "bb", [0., 0., 0.], [0., 0., 1.], 45.);
    bcommon(s, "result", "ba", "bb");
    checkprops(s, "result", Prop::Area, Expected::Mass(3.55724));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "6");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "4.1305");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn G8(s: &mut dyn Session) {
    let r = 5_f64.sqrt();
    let k = 1. / r;
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
    vertex(s, "v6", [r / 2., 0., 1.]);
    vertex(s, "v7", [r / 2., k, 1.]);
    vertex(s, "v8", [0., k, 1.]);
    edge(s, "e5", "v5", "v6");
    edge(s, "e6", "v6", "v7");
    edge(s, "e7", "v7", "v8");
    edge(s, "e8", "v8", "v5");
    wire(s, "wb", &["e5", "e6", "e7", "e8"]);
    mkplane(s, "fb", "wb");
    prism(s, "bb", "fb", [0., 0., -1.]);
    trotate(s, "bb", [0., 0., 0.], [0., 0., 1.], 45.);
    bcommon(s, "result", "ba", "bb");
    checkprops(s, "result", Prop::Area, Expected::Mass(3.55724));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "6");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "4.1305");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn G9(s: &mut dyn Session) {
    let r = 2_f64.sqrt() / 2.;
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
    vertex(s, "v5", [0., 0.5, 0.]);
    vertex(s, "v6", [r, 0.5, 0.]);
    vertex(s, "v7", [r, r, 0.]);
    vertex(s, "v8", [0., r, 0.]);
    edge(s, "e5", "v5", "v6");
    edge(s, "e6", "v6", "v7");
    edge(s, "e7", "v7", "v8");
    edge(s, "e8", "v8", "v5");
    wire(s, "wb", &["e5", "e6", "e7", "e8"]);
    mkplane(s, "fb", "wb");
    prism(s, "bb", "fb", [0., 0., 1.]);
    trotate(s, "bb", [0., 0.5, 0.], [0., 0., 1.], -45.);
    bcommon(s, "result", "ba", "bb");
    checkprops(s, "result", Prop::Area, Expected::Mass(2.12132));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "6");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "2.12132");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn H1(s: &mut dyn Session) {
    let r = 2_f64.sqrt() / 2.;
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
    vertex(s, "v5", [0., 0.5, 0.]);
    vertex(s, "v6", [r, 0.5, 0.]);
    vertex(s, "v7", [r, r, 0.]);
    vertex(s, "v8", [0., r, 0.]);
    edge(s, "e5", "v5", "v6");
    edge(s, "e6", "v6", "v7");
    edge(s, "e7", "v7", "v8");
    edge(s, "e8", "v8", "v5");
    wire(s, "wb", &["e5", "e6", "e7", "e8"]);
    mkplane(s, "fb", "wb");
    prism(s, "bb", "fb", [0., 0., 1.5]);
    trotate(s, "bb", [0., 0.5, 0.], [0., 0., 1.], -45.);
    bcommon(s, "result", "ba", "bb");
    checkprops(s, "result", Prop::Area, Expected::Mass(2.12132));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "6");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "3.03553");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}
