//! The `bfuse_simple` case bodies, part 4; each statement is one line of its script.
use super::*;

pub fn G6(s: &mut dyn Session) {
    vertex(s, "vp1", [3., 3., 0.]);
    vertex(s, "vp2", [8., 3., 0.]);
    vertex(s, "vp3", [8., 9., 0.]);
    vertex(s, "vp4", [3., 9., 0.]);
    edge(s, "eap", "vp1", "vp2");
    edge(s, "ebp", "vp2", "vp3");
    edge(s, "ecp", "vp3", "vp4");
    edge(s, "edp", "vp4", "vp1");
    wire(s, "wp", &["eap", "ebp", "ecp", "edp"]);
    mkplane(s, "fp", "wp");
    prism(s, "ba", "fp", [0., 0., 4.]);
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
    checkprops(s, "result", Prop::Area, Expected::Mass(152.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "148");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "6");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn G7(s: &mut dyn Session) {
    vertex(s, "vp1", [3., 3., 0.]);
    vertex(s, "vp2", [8., 3., 0.]);
    vertex(s, "vp3", [8., 9., 0.]);
    vertex(s, "vp4", [3., 9., 0.]);
    edge(s, "eap", "vp1", "vp2");
    edge(s, "ebp", "vp2", "vp3");
    edge(s, "ecp", "vp3", "vp4");
    edge(s, "edp", "vp4", "vp1");
    wire(s, "wp", &["eap", "ebp", "ecp", "edp"]);
    mkplane(s, "fp", "wp");
    prism(s, "ba", "fp", [0., 0., 4.]);
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
    checkprops(s, "result", Prop::Area, Expected::Mass(152.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "148");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "6");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn G8(s: &mut dyn Session) {
    vertex(s, "vp1", [3., 3., 0.]);
    vertex(s, "vp2", [8., 3., 0.]);
    vertex(s, "vp3", [8., 9., 0.]);
    vertex(s, "vp4", [3., 9., 0.]);
    edge(s, "eap", "vp1", "vp2");
    edge(s, "ebp", "vp2", "vp3");
    edge(s, "ecp", "vp3", "vp4");
    edge(s, "edp", "vp4", "vp1");
    wire(s, "wp", &["eap", "ebp", "ecp", "edp"]);
    mkplane(s, "fp", "wp");
    prism(s, "ba", "fp", [0., 0., 4.]);
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
    checkprops(s, "result", Prop::Area, Expected::Mass(152.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "148");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "6");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn G9(s: &mut dyn Session) {
    vertex(s, "vp1", [3., 3., 0.]);
    vertex(s, "vp2", [8., 3., 0.]);
    vertex(s, "vp3", [8., 9., 0.]);
    vertex(s, "vp4", [3., 9., 0.]);
    edge(s, "eap", "vp1", "vp2");
    edge(s, "ebp", "vp2", "vp3");
    edge(s, "ecp", "vp3", "vp4");
    edge(s, "edp", "vp4", "vp1");
    wire(s, "wp", &["eap", "ebp", "ecp", "edp"]);
    mkplane(s, "fp", "wp");
    prism(s, "ba", "fp", [0., 0., 4.]);
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
    checkprops(s, "result", Prop::Area, Expected::Mass(152.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "148");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "6");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn H1(s: &mut dyn Session) {
    vertex(s, "vp1", [3., 3., 0.]);
    vertex(s, "vp2", [8., 3., 0.]);
    vertex(s, "vp3", [8., 9., 0.]);
    vertex(s, "vp4", [3., 9., 0.]);
    edge(s, "eap", "vp1", "vp2");
    edge(s, "ebp", "vp2", "vp3");
    edge(s, "ecp", "vp3", "vp4");
    edge(s, "edp", "vp4", "vp1");
    wire(s, "wp", &["eap", "ebp", "ecp", "edp"]);
    mkplane(s, "fp", "wp");
    prism(s, "ba", "fp", [0., 0., 4.]);
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
    checkprops(s, "result", Prop::Area, Expected::Mass(152.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "148");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "6");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn H2(s: &mut dyn Session) {
    vertex(s, "vp1", [3., 3., 0.]);
    vertex(s, "vp2", [8., 3., 0.]);
    vertex(s, "vp3", [8., 9., 0.]);
    vertex(s, "vp4", [3., 9., 0.]);
    edge(s, "eap", "vp1", "vp2");
    edge(s, "ebp", "vp2", "vp3");
    edge(s, "ecp", "vp3", "vp4");
    edge(s, "edp", "vp4", "vp1");
    wire(s, "wp", &["eap", "ebp", "ecp", "edp"]);
    mkplane(s, "fp", "wp");
    prism(s, "ba", "fp", [0., 0., 4.]);
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
    checkprops(s, "result", Prop::Area, Expected::Mass(152.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "148");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "6");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn H3(s: &mut dyn Session) {
    vertex(s, "vp1", [3., 3., 0.]);
    vertex(s, "vp2", [8., 3., 0.]);
    vertex(s, "vp3", [8., 9., 0.]);
    vertex(s, "vp4", [3., 9., 0.]);
    edge(s, "eap", "vp1", "vp2");
    edge(s, "ebp", "vp2", "vp3");
    edge(s, "ecp", "vp3", "vp4");
    edge(s, "edp", "vp4", "vp1");
    wire(s, "wp", &["eap", "ebp", "ecp", "edp"]);
    mkplane(s, "fp", "wp");
    prism(s, "ba", "fp", [0., 0., 4.]);
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
    checkprops(s, "result", Prop::Area, Expected::Mass(152.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "148");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "6");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn H4(s: &mut dyn Session) {
    vertex(s, "vp1", [3., 3., 0.]);
    vertex(s, "vp2", [8., 3., 0.]);
    vertex(s, "vp3", [8., 9., 0.]);
    vertex(s, "vp4", [3., 9., 0.]);
    edge(s, "eap", "vp1", "vp2");
    edge(s, "ebp", "vp2", "vp3");
    edge(s, "ecp", "vp3", "vp4");
    edge(s, "edp", "vp4", "vp1");
    wire(s, "wp", &["eap", "ebp", "ecp", "edp"]);
    mkplane(s, "fp", "wp");
    prism(s, "ba", "fp", [0., 0., 4.]);
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
    checkprops(s, "result", Prop::Area, Expected::Mass(152.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "148");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "6");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn H5(s: &mut dyn Session) {
    vertex(s, "vp1", [3., 3., 0.]);
    vertex(s, "vp2", [8., 3., 0.]);
    vertex(s, "vp3", [8., 9., 0.]);
    vertex(s, "vp4", [3., 9., 0.]);
    edge(s, "eap", "vp1", "vp2");
    edge(s, "ebp", "vp2", "vp3");
    edge(s, "ecp", "vp3", "vp4");
    edge(s, "edp", "vp4", "vp1");
    wire(s, "wp", &["eap", "ebp", "ecp", "edp"]);
    mkplane(s, "fp", "wp");
    prism(s, "ba", "fp", [0., 0., 4.]);
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
    checkprops(s, "result", Prop::Area, Expected::Mass(152.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "148");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "6");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn H6(s: &mut dyn Session) {
    vertex(s, "vp1", [3., 3., 0.]);
    vertex(s, "vp2", [8., 3., 0.]);
    vertex(s, "vp3", [8., 9., 0.]);
    vertex(s, "vp4", [3., 9., 0.]);
    edge(s, "eap", "vp1", "vp2");
    edge(s, "ebp", "vp2", "vp3");
    edge(s, "ecp", "vp3", "vp4");
    edge(s, "edp", "vp4", "vp1");
    wire(s, "wp", &["eap", "ebp", "ecp", "edp"]);
    mkplane(s, "fp", "wp");
    prism(s, "ba", "fp", [0., 0., 4.]);
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
    checkprops(s, "result", Prop::Area, Expected::Mass(152.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "148");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "6");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn H7(s: &mut dyn Session) {
    vertex(s, "vp1", [3., 3., 0.]);
    vertex(s, "vp2", [8., 3., 0.]);
    vertex(s, "vp3", [8., 9., 0.]);
    vertex(s, "vp4", [3., 9., 0.]);
    edge(s, "eap", "vp1", "vp2");
    edge(s, "ebp", "vp2", "vp3");
    edge(s, "ecp", "vp3", "vp4");
    edge(s, "edp", "vp4", "vp1");
    wire(s, "wp", &["eap", "ebp", "ecp", "edp"]);
    mkplane(s, "fp", "wp");
    prism(s, "ba", "fp", [0., 0., 4.]);
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
    checkprops(s, "result", Prop::Area, Expected::Mass(152.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "bb"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "148");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "bb", Prop::Area, "6");
    kind_is(s, "bb", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}
