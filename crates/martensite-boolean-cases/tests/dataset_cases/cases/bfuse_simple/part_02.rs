//! The `bfuse_simple` case bodies, part 2; each statement is one line of its script.
use super::*;

pub fn C7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0.5, 0., -0.5], [1., 1., 0.5]);
    bfuse(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(9.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b2", Prop::Area, "4");
    kind_is(s, "b2", ShapeEnum::Solid);
    printed_mass(s, "b1", Prop::Area, "6");
    kind_is(s, "b1", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn C8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    let r = 2_f64.sqrt();
    make_box(s, "b2", [0., 0., 0.], [r, r / 2., 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    bfuse(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(7.82843));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b2", Prop::Area, "6.24264");
    kind_is(s, "b2", ShapeEnum::Solid);
    printed_mass(s, "b1", Prop::Area, "6");
    kind_is(s, "b1", ShapeEnum::Solid);
    rotation_is(s, "b2", 45.0);
    view_mass_is_nonzero(s, "result");
}

pub fn C9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [0., 0., 0.], [r, r, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    bfuse(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(6.91421));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b2", Prop::Area, "3.82843");
    kind_is(s, "b2", ShapeEnum::Solid);
    printed_mass(s, "b1", Prop::Area, "6");
    kind_is(s, "b1", ShapeEnum::Solid);
    rotation_is(s, "b2", 45.0);
    view_mass_is_nonzero(s, "result");
}

pub fn D1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    let r = 2_f64.sqrt();
    make_box(s, "b2", [0., 0., 0.], [r, 0.25, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    bfuse(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(6.41789));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b2", Prop::Area, "4.03553");
    kind_is(s, "b2", ShapeEnum::Solid);
    printed_mass(s, "b1", Prop::Area, "6");
    kind_is(s, "b1", ShapeEnum::Solid);
    rotation_is(s, "b2", 45.0);
    view_mass_is_nonzero(s, "result");
}

pub fn D2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    let r = 31_f64.sqrt();
    make_box(s, "b2", [0., 0., 0.], [r / 4., 0.25, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 34.73);
    bfuse(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(6.32953));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b2", Prop::Area, "3.97985");
    kind_is(s, "b2", ShapeEnum::Solid);
    printed_mass(s, "b1", Prop::Area, "6");
    kind_is(s, "b1", ShapeEnum::Solid);
    rotation_is(s, "b2", 34.73);
    view_mass_is_nonzero(s, "result");
}

pub fn D3(s: &mut dyn Session) {
    psphere(s, "s", 1.);
    make_box(s, "b", [0., 0., 0.], [1., 1., 1.]);
    bfuse(s, "result", "s", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(14.6394));
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
    bfuse(s, "result", "b", "s");
    checkprops(s, "result", Prop::Area, Expected::Mass(14.6394));
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
    bfuse(s, "result", "s", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(14.6393));
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
    bfuse(s, "result", "b", "s");
    checkprops(s, "result", Prop::Area, Expected::Mass(14.6393));
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
    bfuse(s, "result", "s", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(14.6393));
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
    bfuse(s, "result", "b", "s");
    checkprops(s, "result", Prop::Area, Expected::Mass(14.6393));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b", "s"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn D9(s: &mut dyn Session) {
    profile(
        s,
        "f1",
        &[
            Command::X(100.),
            Command::Y(100.),
            Command::X(-200.),
            Command::Y(-200.),
            Command::X(100.),
        ],
    );
    profile(
        s,
        "f2",
        &[Command::X(-100.), Command::Y(100.), Command::X(100.)],
    );
    ttranslate(s, "f2", [0., 0., 100.]);
    prism(s, "p1", "f1", [0., 0., 100.]);
    prism(s, "p2", "f2", [0., 0., 100.]);
    bfuse(s, "result", "p1", "p2");
    checkprops(s, "result", Prop::Area, Expected::Mass(180000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["p1", "p2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "p1", Prop::Area, "140000");
    kind_is(s, "p1", ShapeEnum::Solid);
    printed_mass(s, "p2", Prop::Area, "60000");
    kind_is(s, "p2", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn E1(s: &mut dyn Session) {
    profile(
        s,
        "f1",
        &[
            Command::Arc(50. * hooks::SCALE, 180.),
            Command::X(-100. * hooks::SCALE),
            Command::Arc(50. * hooks::SCALE, 180.),
        ],
    );
    prism(s, "p1", "f1", [0., 0., 30. * hooks::SCALE]);
    profile(
        s,
        "f2",
        &[
            Command::X(300. * hooks::SCALE),
            Command::Y(200. * hooks::SCALE),
            Command::X(-300. * hooks::SCALE),
        ],
    );
    ttranslate(s, "f2", [-200. * hooks::SCALE, -50. * hooks::SCALE, 0.]);
    prism(s, "p2", "f2", [0., 0., -50. * hooks::SCALE]);
    bfuse(s, "result", "p2", "p1");
    checkprops(s, "result", Prop::Area, Expected::Mass(1.85425e+09));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["p2", "p1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "p2", Prop::Area, "1.7e+09");
    kind_is(s, "p2", ShapeEnum::Solid);
    printed_mass(s, "p1", Prop::Area, "5.11327e+08");
    kind_is(s, "p1", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn E2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [3., 3., 3.]);
    make_box(s, "b2", [0., 0., 0.], [1., 1., 1.]);
    ttranslate(s, "b2", [0., 3., 0.]);
    bfuse(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(58.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn E3(s: &mut dyn Session) {
    vertex(s, "v1", [0., 0., 0.]);
    vertex(s, "v2", [9., 0., 0.]);
    vertex(s, "v3", [9., 0., 3.]);
    vertex(s, "v4", [6.25, 0., 3.]);
    vertex(s, "v5", [6., 0., 4.]);
    vertex(s, "v6", [0., 0., 4.]);
    edge(s, "ea", "v1", "v2");
    edge(s, "eb", "v2", "v3");
    edge(s, "ec", "v3", "v4");
    edge(s, "ed", "v4", "v5");
    edge(s, "ee", "v5", "v6");
    edge(s, "ef", "v6", "v1");
    wire(s, "wa", &["ea", "eb", "ec", "ed", "ee", "ef"]);
    mkplane(s, "pa", "wa");
    revol(s, "cyla", "pa", [0., 0., 0.], [0., 0., 1.], 360.);
    pcylinder(s, "cylb", 1., 9.);
    ttranslate(s, "cylb", [5., 0., -2.]);
    bfuse(s, "result", "cyla", "cylb");
    checkprops(s, "result", Prop::Area, Expected::Mass(740.048));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["cyla", "cylb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "cyla", Prop::Area, "708.632");
    kind_is(s, "cyla", ShapeEnum::Solid);
    printed_mass(s, "cylb", Prop::Area, "62.8319");
    kind_is(s, "cylb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn E4(s: &mut dyn Session) {
    pcylinder(s, "cyl", 3., 5.);
    vertex(s, "v1", [0., 3., 2.]);
    vertex(s, "v2", [0., 4., 2.]);
    vertex(s, "v3", [0., 4., 3.]);
    vertex(s, "v4", [0., 3., 3.]);
    edge(s, "e1", "v1", "v2");
    edge(s, "e2", "v2", "v3");
    edge(s, "e3", "v3", "v4");
    edge(s, "e4", "v4", "v1");
    wire(s, "w", &["e1", "e2", "e3", "e4"]);
    mkplane(s, "f", "w");
    revol(s, "ring", "f", [0., 0., 0.], [0., 0., 1.], 269.);
    bfuse(s, "result", "cyl", "ring");
    checkprops(s, "result", Prop::Area, Expected::Mass(190.356));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["cyl", "ring"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "cyl", Prop::Area, "150.796");
    kind_is(s, "cyl", ShapeEnum::Solid);
    printed_mass(s, "ring", Prop::Area, "67.7291");
    kind_is(s, "ring", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn E5(s: &mut dyn Session) {
    make_box(s, "ba", [3., 3., 0.], [5., 7., 4.]);
    vertex(s, "v1", [3., 2., 0.]);
    vertex(s, "v2", [4., 2., 0.]);
    vertex(s, "v3", [4., 3., 0.]);
    vertex(s, "v4", [3., 3., 0.]);
    edge(s, "ea", "v1", "v2");
    edge(s, "eb", "v2", "v3");
    edge(s, "ec", "v3", "v4");
    edge(s, "ed", "v4", "v1");
    wire(s, "w", &["ea", "eb", "ec", "ed"]);
    mkplane(s, "f", "w");
    prism(s, "bb", "f", [0., 0., 1.]);
    bfuse(s, "result", "ba", "bb");
    normals(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(170.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "166");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "6");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn E6(s: &mut dyn Session) {
    make_box(s, "ba", [3., 3., 0.], [5., 7., 4.]);
    vertex(s, "v1", [3., 2., 0.]);
    vertex(s, "v2", [4., 2., 0.]);
    vertex(s, "v3", [4., 2., 1.]);
    vertex(s, "v4", [3., 2., 1.]);
    edge(s, "ea", "v1", "v2");
    edge(s, "eb", "v2", "v3");
    edge(s, "ec", "v3", "v4");
    edge(s, "ed", "v4", "v1");
    wire(s, "w", &["ea", "eb", "ec", "ed"]);
    mkplane(s, "f", "w");
    prism(s, "bb", "f", [0., 1., 0.]);
    bfuse(s, "result", "ba", "bb");
    checkprops(s, "result", Prop::Area, Expected::Mass(170.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "166");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "6");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn E7(s: &mut dyn Session) {
    make_box(s, "ba", [3., 3., 0.], [5., 7., 4.]);
    vertex(s, "v1", [3., 2., 0.]);
    vertex(s, "v2", [3., 3., 0.]);
    vertex(s, "v3", [3., 3., 1.]);
    vertex(s, "v4", [3., 2., 1.]);
    edge(s, "ea", "v1", "v2");
    edge(s, "eb", "v2", "v3");
    edge(s, "ec", "v3", "v4");
    edge(s, "ed", "v4", "v1");
    wire(s, "w", &["ea", "eb", "ec", "ed"]);
    mkplane(s, "f", "w");
    prism(s, "bb", "f", [1., 0., 0.]);
    bfuse(s, "result", "ba", "bb");
    normals(s, "result");
    checkprops(s, "result", Prop::Area, Expected::Mass(170.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "166");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "6");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}
