//! The `bfuse_simple` case bodies, part 3; each statement is one line of its script.
use super::*;

pub fn E8(s: &mut dyn Session) {
    make_box(s, "ba", [3., 3., 0.], [5., 7., 4.]);
    vertex(s, "v1", [3., 2., 1.]);
    vertex(s, "v2", [4., 2., 1.]);
    vertex(s, "v3", [4., 3., 1.]);
    vertex(s, "v4", [3., 3., 1.]);
    edge(s, "ea", "v1", "v2");
    edge(s, "eb", "v2", "v3");
    edge(s, "ec", "v3", "v4");
    edge(s, "ed", "v4", "v1");
    wire(s, "w", &["ea", "eb", "ec", "ed"]);
    mkplane(s, "f", "w");
    prism(s, "bb", "f", [0., 0., -1.]);
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

pub fn E9(s: &mut dyn Session) {
    make_box(s, "ba", [3., 3., 0.], [5., 7., 4.]);
    vertex(s, "v1", [3., 3., 0.]);
    vertex(s, "v2", [4., 3., 0.]);
    vertex(s, "v3", [4., 3., 1.]);
    vertex(s, "v4", [3., 3., 1.]);
    edge(s, "ea", "v1", "v2");
    edge(s, "eb", "v2", "v3");
    edge(s, "ec", "v3", "v4");
    edge(s, "ed", "v4", "v1");
    wire(s, "w", &["ea", "eb", "ec", "ed"]);
    mkplane(s, "f", "w");
    prism(s, "bb", "f", [0., -1., 0.]);
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

pub fn F1(s: &mut dyn Session) {
    make_box(s, "ba", [3., 3., 0.], [5., 7., 4.]);
    vertex(s, "v1", [4., 2., 0.]);
    vertex(s, "v2", [4., 3., 0.]);
    vertex(s, "v3", [4., 3., 1.]);
    vertex(s, "v4", [4., 2., 1.]);
    edge(s, "ea", "v1", "v2");
    edge(s, "eb", "v2", "v3");
    edge(s, "ec", "v3", "v4");
    edge(s, "ed", "v4", "v1");
    wire(s, "w", &["ea", "eb", "ec", "ed"]);
    mkplane(s, "f", "w");
    prism(s, "bb", "f", [-1., 0., 0.]);
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

pub fn F2(s: &mut dyn Session) {
    make_box(s, "ba", [3., 3., 0.], [5., 7., 4.]);
    vertex(s, "v1", [4., 3., 0.]);
    vertex(s, "v2", [4., 2., 0.]);
    vertex(s, "v3", [3., 2., 0.]);
    vertex(s, "v4", [3., 3., 0.]);
    edge(s, "ea", "v1", "v2");
    edge(s, "eb", "v2", "v3");
    edge(s, "ec", "v3", "v4");
    edge(s, "ed", "v4", "v1");
    wire(s, "w", &["ea", "eb", "ec", "ed"]);
    mkplane(s, "f", "w");
    prism(s, "bb", "f", [0., 0., 1.]);
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

pub fn F3(s: &mut dyn Session) {
    make_box(s, "ba", [3., 3., 0.], [5., 7., 4.]);
    vertex(s, "v1", [8., 3., 0.]);
    vertex(s, "v2", [9., 3., 0.]);
    vertex(s, "v3", [9., 4., 0.]);
    vertex(s, "v4", [8., 4., 0.]);
    edge(s, "ea", "v1", "v2");
    edge(s, "eb", "v2", "v3");
    edge(s, "ec", "v3", "v4");
    edge(s, "ed", "v4", "v1");
    wire(s, "w", &["ea", "eb", "ec", "ed"]);
    mkplane(s, "f", "w");
    prism(s, "bb", "f", [0., 0., 1.]);
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

pub fn F4(s: &mut dyn Session) {
    make_box(s, "ba", [3., 3., 0.], [5., 7., 4.]);
    vertex(s, "v1", [8., 3., 0.]);
    vertex(s, "v2", [9., 3., 0.]);
    vertex(s, "v3", [9., 3., 1.]);
    vertex(s, "v4", [8., 3., 1.]);
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

pub fn F5(s: &mut dyn Session) {
    make_box(s, "ba", [3., 3., 0.], [5., 7., 4.]);
    vertex(s, "v1", [8., 3., 0.]);
    vertex(s, "v2", [8., 4., 0.]);
    vertex(s, "v3", [8., 4., 1.]);
    vertex(s, "v4", [8., 3., 1.]);
    edge(s, "ea", "v1", "v2");
    edge(s, "eb", "v2", "v3");
    edge(s, "ec", "v3", "v4");
    edge(s, "ed", "v4", "v1");
    wire(s, "w", &["ea", "eb", "ec", "ed"]);
    mkplane(s, "f", "w");
    prism(s, "bb", "f", [1., 0., 0.]);
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

pub fn F6(s: &mut dyn Session) {
    make_box(s, "ba", [3., 3., 0.], [5., 7., 4.]);
    vertex(s, "v1", [8., 3., 1.]);
    vertex(s, "v2", [9., 3., 1.]);
    vertex(s, "v3", [9., 4., 1.]);
    vertex(s, "v4", [8., 4., 1.]);
    edge(s, "ea", "v1", "v2");
    edge(s, "eb", "v2", "v3");
    edge(s, "ec", "v3", "v4");
    edge(s, "ed", "v4", "v1");
    wire(s, "w", &["ea", "eb", "ec", "ed"]);
    mkplane(s, "f", "w");
    prism(s, "bb", "f", [0., 0., -1.]);
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

pub fn F7(s: &mut dyn Session) {
    make_box(s, "ba", [3., 3., 0.], [5., 7., 4.]);
    vertex(s, "v1", [8., 4., 0.]);
    vertex(s, "v2", [9., 4., 0.]);
    vertex(s, "v3", [9., 4., 1.]);
    vertex(s, "v4", [8., 4., 1.]);
    edge(s, "ea", "v1", "v2");
    edge(s, "eb", "v2", "v3");
    edge(s, "ec", "v3", "v4");
    edge(s, "ed", "v4", "v1");
    wire(s, "w", &["ea", "eb", "ec", "ed"]);
    mkplane(s, "f", "w");
    prism(s, "bb", "f", [0., -1., 0.]);
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

pub fn F8(s: &mut dyn Session) {
    make_box(s, "ba", [3., 3., 0.], [5., 7., 4.]);
    vertex(s, "v1", [9., 3., 0.]);
    vertex(s, "v2", [9., 4., 0.]);
    vertex(s, "v3", [9., 4., 1.]);
    vertex(s, "v4", [9., 3., 1.]);
    edge(s, "ea", "v1", "v2");
    edge(s, "eb", "v2", "v3");
    edge(s, "ec", "v3", "v4");
    edge(s, "ed", "v4", "v1");
    wire(s, "w", &["ea", "eb", "ec", "ed"]);
    mkplane(s, "f", "w");
    prism(s, "bb", "f", [-1., 0., 0.]);
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

pub fn F9(s: &mut dyn Session) {
    make_box(s, "ba", [3., 3., 0.], [5., 7., 4.]);
    vertex(s, "v1", [3., 3., 4.]);
    vertex(s, "v2", [4., 3., 4.]);
    vertex(s, "v3", [4., 4., 4.]);
    vertex(s, "v4", [3., 4., 4.]);
    edge(s, "ea", "v1", "v2");
    edge(s, "eb", "v2", "v3");
    edge(s, "ec", "v3", "v4");
    edge(s, "ed", "v4", "v1");
    wire(s, "w", &["ea", "eb", "ec", "ed"]);
    mkplane(s, "f", "w");
    prism(s, "bb", "f", [0., 0., 1.]);
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

pub fn G1(s: &mut dyn Session) {
    make_box(s, "ba", [3., 3., 0.], [5., 7., 4.]);
    vertex(s, "v1", [3., 3., 4.]);
    vertex(s, "v2", [4., 3., 4.]);
    vertex(s, "v3", [4., 3., 5.]);
    vertex(s, "v4", [3., 3., 5.]);
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

pub fn G2(s: &mut dyn Session) {
    make_box(s, "ba", [3., 3., 0.], [5., 7., 4.]);
    vertex(s, "v1", [3., 3., 4.]);
    vertex(s, "v2", [3., 4., 4.]);
    vertex(s, "v3", [3., 4., 5.]);
    vertex(s, "v4", [3., 3., 5.]);
    edge(s, "ea", "v1", "v2");
    edge(s, "eb", "v2", "v3");
    edge(s, "ec", "v3", "v4");
    edge(s, "ed", "v4", "v1");
    wire(s, "w", &["ea", "eb", "ec", "ed"]);
    mkplane(s, "f", "w");
    prism(s, "bb", "f", [1., 0., 0.]);
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

pub fn G3(s: &mut dyn Session) {
    make_box(s, "ba", [3., 3., 0.], [5., 7., 4.]);
    vertex(s, "v1", [3., 3., 5.]);
    vertex(s, "v2", [4., 3., 5.]);
    vertex(s, "v3", [4., 4., 5.]);
    vertex(s, "v4", [3., 4., 5.]);
    edge(s, "ea", "v1", "v2");
    edge(s, "eb", "v2", "v3");
    edge(s, "ec", "v3", "v4");
    edge(s, "ed", "v4", "v1");
    wire(s, "w", &["ea", "eb", "ec", "ed"]);
    mkplane(s, "f", "w");
    prism(s, "bb", "f", [0., 0., -1.]);
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

pub fn G4(s: &mut dyn Session) {
    make_box(s, "ba", [3., 3., 0.], [5., 7., 4.]);
    vertex(s, "v1", [3., 4., 4.]);
    vertex(s, "v2", [4., 4., 4.]);
    vertex(s, "v3", [4., 4., 5.]);
    vertex(s, "v4", [3., 4., 5.]);
    edge(s, "ea", "v1", "v2");
    edge(s, "eb", "v2", "v3");
    edge(s, "ec", "v3", "v4");
    edge(s, "ed", "v4", "v1");
    wire(s, "w", &["ea", "eb", "ec", "ed"]);
    mkplane(s, "f", "w");
    prism(s, "bb", "f", [0., -1., 0.]);
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

pub fn G5(s: &mut dyn Session) {
    make_box(s, "ba", [3., 3., 0.], [5., 7., 4.]);
    vertex(s, "v1", [4., 3., 4.]);
    vertex(s, "v2", [4., 4., 4.]);
    vertex(s, "v3", [4., 4., 5.]);
    vertex(s, "v4", [4., 3., 5.]);
    edge(s, "ea", "v1", "v2");
    edge(s, "eb", "v2", "v3");
    edge(s, "ec", "v3", "v4");
    edge(s, "ed", "v4", "v1");
    wire(s, "w", &["ea", "eb", "ec", "ed"]);
    mkplane(s, "f", "w");
    prism(s, "bb", "f", [-1., 0., 0.]);
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
