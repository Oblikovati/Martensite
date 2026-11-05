//! The `bcommon_simple` case bodies, part 5; each statement is one line of its script.
use super::*;

pub fn H2(s: &mut dyn Session) {
    let r = 2_f64.sqrt() * 0.75;
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
    checkprops(s, "result", Prop::Area, Expected::Mass(3.93566));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "6");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "4.43198");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn H3(s: &mut dyn Session) {
    let a30 = martensite_cmath::atan2(1_f64, 2.) * 180. / std::f64::consts::PI;
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
    vertex(s, "v5", [0., 0.5, 0.]);
    vertex(s, "v6", [1.25, 0.5, 0.]);
    vertex(s, "v7", [1.25, k, 0.]);
    vertex(s, "v8", [0., k, 0.]);
    edge(s, "e5", "v5", "v6");
    edge(s, "e6", "v6", "v7");
    edge(s, "e7", "v7", "v8");
    edge(s, "e8", "v8", "v5");
    wire(s, "wb", &["e5", "e6", "e7", "e8"]);
    mkplane(s, "fb", "wb");
    prism(s, "bb", "fb", [0., 0., 1.]);
    trotate(s, "bb", [0., 0.5, 0.], [0., 0., 1.], a30);
    bcommon(s, "result", "ba", "bb");
    checkprops(s, "result", Prop::Area, Expected::Mass(2.43812));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "6");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "2.73754");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn H4(s: &mut dyn Session) {
    let r = 2_f64.sqrt();
    let k = r / 2.;
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
    vertex(s, "v3", [-0.25, -0.25, 0.]);
    vertex(s, "v4", [2., -0.25, 0.]);
    edge(s, "eb", "v3", "v4");
    prism(s, "fb", "eb", [0., k - 0.25, 0.]);
    prism(s, "bb", "fb", [0., 0., 1.]);
    vertex(s, "v5", [-0.25, -0.25, 0.]);
    vertex(s, "v6", [2., -0.25, 0.]);
    vertex(s, "v7", [2., k, 0.]);
    vertex(s, "v8", [-0.25, k, 0.]);
    edge(s, "e5", "v5", "v6");
    edge(s, "e6", "v6", "v7");
    edge(s, "e7", "v7", "v8");
    edge(s, "e8", "v8", "v5");
    wire(s, "wb", &["e5", "e6", "e7", "e8"]);
    mkplane(s, "fb", "wb");
    prism(s, "bb", "fb", [0., 0., 1.]);
    trotate(s, "bb", [-0.25, -0.25, 0.], [0., 0., 1.], 45.);
    bcommon(s, "result", "ba", "bb");
    checkprops(s, "result", Prop::Area, Expected::Mass(4.41421));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "6");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "10.7212");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn H5(s: &mut dyn Session) {
    let r = 2_f64.sqrt();
    let k = r / 2.;
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
    vertex(s, "v3", [-0.25, -0.25, 0.]);
    vertex(s, "v4", [2., -0.25, 0.]);
    edge(s, "eb", "v3", "v4");
    prism(s, "fb", "eb", [0., k - 0.25, 0.]);
    prism(s, "bb", "fb", [0., 0., 1.]);
    vertex(s, "v5", [-0.25, -0.25, 0.]);
    vertex(s, "v6", [2., -0.25, 0.]);
    vertex(s, "v7", [2., k, 0.]);
    vertex(s, "v8", [-0.25, k, 0.]);
    edge(s, "e5", "v5", "v6");
    edge(s, "e6", "v6", "v7");
    edge(s, "e7", "v7", "v8");
    edge(s, "e8", "v8", "v5");
    wire(s, "wb", &["e5", "e6", "e7", "e8"]);
    mkplane(s, "fb", "wb");
    prism(s, "bb", "fb", [0., 0., 1.5]);
    trotate(s, "bb", [-0.25, -0.25, 0.], [0., 0., 1.], 45.);
    bcommon(s, "result", "ba", "bb");
    checkprops(s, "result", Prop::Area, Expected::Mass(4.41421));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "6");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "13.9283");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn H6(s: &mut dyn Session) {
    vertex(s, "v1", [-1., -1., 0.]);
    vertex(s, "v2", [1., -1., 0.]);
    vertex(s, "v3", [1., 1., 0.]);
    vertex(s, "v4", [-1., 1., 0.]);
    edge(s, "e1", "v1", "v2");
    edge(s, "e2", "v2", "v3");
    edge(s, "e3", "v3", "v4");
    edge(s, "e4", "v4", "v1");
    wire(s, "wa", &["e1", "e2", "e3", "e4"]);
    mkplane(s, "fa", "wa");
    prism(s, "ba", "fa", [0., 0., 2.]);
    let c_curve = circle([0., 0., 0.], 1.);
    mkedge(s, "eb", &c_curve);
    wire(s, "wb", &["eb"]);
    mkplane(s, "fb", "wb");
    prism(s, "cylb", "fb", [0., 0., 2.]);
    bcommon(s, "result", "ba", "cylb");
    checkprops(s, "result", Prop::Area, Expected::Mass(18.8496));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "cylb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "24");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "cylb", Prop::Area, "18.8496");
    kind_is(s, "cylb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn H7(s: &mut dyn Session) {
    vertex(s, "v1", [-1., -1., 0.]);
    vertex(s, "v2", [1., -1., 0.]);
    vertex(s, "v3", [1., 1., 0.]);
    vertex(s, "v4", [-1., 1., 0.]);
    edge(s, "e1", "v1", "v2");
    edge(s, "e2", "v2", "v3");
    edge(s, "e3", "v3", "v4");
    edge(s, "e4", "v4", "v1");
    wire(s, "wa", &["e1", "e2", "e3", "e4"]);
    mkplane(s, "fa", "wa");
    prism(s, "ba", "fa", [0., 0., 1.]);
    let c_curve = circle([0., 0., 0.], 1.);
    mkedge(s, "eb", &c_curve);
    wire(s, "wb", &["eb"]);
    mkplane(s, "fb", "wb");
    prism(s, "cylb", "fb", [0., 0., 2.]);
    bcommon(s, "result", "ba", "cylb");
    checkprops(s, "result", Prop::Area, Expected::Mass(12.5664));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "cylb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "16");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "cylb", Prop::Area, "18.8496");
    kind_is(s, "cylb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn H8(s: &mut dyn Session) {
    vertex(s, "v1", [-1., -1., 0.]);
    vertex(s, "v2", [2., -1., 0.]);
    vertex(s, "v3", [2., 0., 0.]);
    vertex(s, "v4", [-1., 0., 0.]);
    edge(s, "e1", "v1", "v2");
    edge(s, "e2", "v2", "v3");
    edge(s, "e3", "v3", "v4");
    edge(s, "e4", "v4", "v1");
    wire(s, "wa", &["e1", "e2", "e3", "e4"]);
    mkplane(s, "fa", "wa");
    prism(s, "ba", "fa", [0., 0., 1.]);
    let c_curve = circle([0., 0., 0.], 1.);
    mkedge(s, "eb", &c_curve);
    wire(s, "wb", &["eb"]);
    mkplane(s, "fb", "wb");
    prism(s, "cylb", "fb", [0., 0., 2.]);
    bcommon(s, "result", "ba", "cylb");
    checkprops(s, "result", Prop::Area, Expected::Mass(8.28319));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "cylb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "14");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "cylb", Prop::Area, "18.8496");
    kind_is(s, "cylb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn H9(s: &mut dyn Session) {
    let r = 3_f64.sqrt() / 2.;
    vertex(s, "v1", [-1., -1., 0.]);
    vertex(s, "v2", [1. + r, -1., 0.]);
    vertex(s, "v3", [1. + r, 2., 0.]);
    vertex(s, "v4", [-1., 2., 0.]);
    edge(s, "e1", "v1", "v2");
    edge(s, "e2", "v2", "v3");
    edge(s, "e3", "v3", "v4");
    edge(s, "e4", "v4", "v1");
    wire(s, "wa", &["e1", "e2", "e3", "e4"]);
    mkplane(s, "fa", "wa");
    prism(s, "ba", "fa", [0., 0., 1.]);
    trotate(s, "ba", [0., 0., 0.], [0., 0., 1.], 60.);
    let c_curve = circle([0., 0., 0.], 1.);
    mkedge(s, "eb", &c_curve);
    wire(s, "wb", &["eb"]);
    mkplane(s, "fb", "wb");
    prism(s, "cylb", "fb", [0., 0., 2.]);
    bcommon(s, "result", "ba", "cylb");
    checkprops(s, "result", Prop::Area, Expected::Mass(12.5664));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "cylb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "28.9282");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "cylb", Prop::Area, "18.8496");
    kind_is(s, "cylb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn I1(s: &mut dyn Session) {
    vertex(s, "v1", [-1., -1., 0.]);
    vertex(s, "v2", [1.5, -1., 0.]);
    vertex(s, "v3", [1.5, 2., 0.]);
    vertex(s, "v4", [-1., 2., 0.]);
    edge(s, "e1", "v1", "v2");
    edge(s, "e2", "v2", "v3");
    edge(s, "e3", "v3", "v4");
    edge(s, "e4", "v4", "v1");
    wire(s, "wa", &["e1", "e2", "e3", "e4"]);
    mkplane(s, "fa", "wa");
    prism(s, "ba", "fa", [0., 0., 1.]);
    vertex(s, "v3", [0., 0., 0.]);
    vertex(s, "v4", [1., 0., 0.]);
    vertex(s, "v5", [1., 0., 2.]);
    vertex(s, "v6", [0., 0., 2.]);
    edge(s, "eb", "v3", "v4");
    edge(s, "ec", "v4", "v5");
    edge(s, "ed", "v5", "v6");
    edge(s, "ee", "v6", "v3");
    wire(s, "wb", &["eb", "ec", "ed", "ee"]);
    mkplane(s, "pb", "wb");
    revol(s, "cylb", "pb", [0., 0., 0.], [0., 0., 1.], 360.);
    bcommon(s, "result", "ba", "cylb");
    checkprops(s, "result", Prop::Area, Expected::Mass(12.5664));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "cylb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "26");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "cylb", Prop::Area, "18.8496");
    kind_is(s, "cylb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn I2(s: &mut dyn Session) {
    let ca_curve = circle([0., 0., 0.], 1.);
    mkedge(s, "ea", &ca_curve);
    wire(s, "wa", &["ea"]);
    mkplane(s, "fa", "wa");
    prism(s, "cyla", "fa", [0., 0., 2.]);
    let cb_curve = circle([0., 0., 0.], 0.5);
    mkedge(s, "eb", &cb_curve);
    wire(s, "wb", &["eb"]);
    mkplane(s, "fb", "wb");
    prism(s, "cylb", "fb", [0., 0., 1.]);
    ttranslate(s, "cylb", [0., 0., 2.]);
    bcommon(s, "result", "cyla", "cylb");
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["cyla", "cylb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "cyla", Prop::Area, "18.8496");
    kind_is(s, "cyla", ShapeEnum::Solid);
    printed_mass(s, "cylb", Prop::Area, "4.71239");
    kind_is(s, "cylb", ShapeEnum::Solid);
}

pub fn I3(s: &mut dyn Session) {
    let ca_curve = circle([0., 0., 0.], 1.);
    mkedge(s, "ea", &ca_curve);
    wire(s, "wa", &["ea"]);
    mkplane(s, "fa", "wa");
    prism(s, "cyla", "fa", [0., 0., 2.]);
    vertex(s, "v3", [0., 0., 0.]);
    vertex(s, "v4", [0.5, 0., 0.]);
    vertex(s, "v5", [0.5, 0., 1.]);
    vertex(s, "v6", [0., 0., 1.]);
    edge(s, "eb", "v3", "v4");
    edge(s, "ec", "v4", "v5");
    edge(s, "ed", "v5", "v6");
    edge(s, "ee", "v6", "v3");
    wire(s, "wb", &["eb", "ec", "ed", "ee"]);
    mkplane(s, "pb", "wb");
    revol(s, "cylb", "pb", [0., 0., 0.], [0., 0., 1.], 360.);
    ttranslate(s, "cylb", [0., 0., 1.]);
    bcommon(s, "result", "cyla", "cylb");
    checkprops(s, "result", Prop::Area, Expected::Mass(4.71239));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["cyla", "cylb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "cyla", Prop::Area, "18.8496");
    kind_is(s, "cyla", ShapeEnum::Solid);
    printed_mass(s, "cylb", Prop::Area, "4.71239");
    kind_is(s, "cylb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn I4(s: &mut dyn Session) {
    let ca_curve = circle([0., 0., 0.], 1.);
    mkedge(s, "ea", &ca_curve);
    wire(s, "wa", &["ea"]);
    mkplane(s, "fa", "wa");
    prism(s, "cyla", "fa", [0., 0., 2.]);
    vertex(s, "v3", [0., 0., 0.]);
    vertex(s, "v4", [0.5, 0., 0.]);
    vertex(s, "v5", [0.5, 0., 1.]);
    vertex(s, "v6", [0., 0., 1.]);
    edge(s, "eb", "v3", "v4");
    edge(s, "ec", "v4", "v5");
    edge(s, "ed", "v5", "v6");
    edge(s, "ee", "v6", "v3");
    wire(s, "wb", &["eb", "ec", "ed", "ee"]);
    mkplane(s, "pb", "wb");
    revol(s, "cylb", "pb", [0., 0., 0.], [0., 0., 1.], 360.);
    ttranslate(s, "cylb", [0., 0., 0.5]);
    bcommon(s, "result", "cyla", "cylb");
    checkprops(s, "result", Prop::Area, Expected::Mass(4.71239));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["cyla", "cylb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "cyla", Prop::Area, "18.8496");
    kind_is(s, "cyla", ShapeEnum::Solid);
    printed_mass(s, "cylb", Prop::Area, "4.71239");
    kind_is(s, "cylb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}
