//! The `bfuse_simple` case bodies, part 5; each statement is one line of its script.
use super::*;

pub fn H8(s: &mut dyn Session) {
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

pub fn H9(s: &mut dyn Session) {
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

pub fn I1(s: &mut dyn Session) {
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

pub fn I2(s: &mut dyn Session) {
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

pub fn I3(s: &mut dyn Session) {
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

pub fn I4(s: &mut dyn Session) {
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

pub fn I5(s: &mut dyn Session) {
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

pub fn I6(s: &mut dyn Session) {
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
    prism(s, "ba", "fp", [0., 0., 5.]);
    profile_wire(
        s,
        "prof",
        &[
            Command::Origin([4., 3., 0.]),
            Command::Plane([0., 1., 0.], [1., 0., 0.]),
            Command::Direction(-1., 0.),
            Command::Arc(1., 180.),
            Command::X(2.),
            Command::Arc(1., 180.),
        ],
    );
    mkplane(s, "pl", "prof");
    prism(s, "ob", "pl", [0., -1., 0.]);
    bfuse(s, "result", "ba", "ob");
    checkprops(s, "result", Prop::Area, Expected::Mass(180.283));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "ob"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "170");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "ob", Prop::Area, "24.5664");
    kind_is(s, "ob", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn I7(s: &mut dyn Session) {
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
    prism(s, "ba", "fp", [0., 0., 5.]);
    profile_wire(
        s,
        "prof",
        &[
            Command::Origin([3., 3., 3.]),
            Command::Plane([0., 1., 0.], [1., 0., 0.]),
            Command::Direction(0., -1.),
            Command::Arc(1., 180.),
            Command::Y(2.),
            Command::Arc(1., 180.),
        ],
    );
    mkplane(s, "pl", "prof");
    prism(s, "ob", "pl", [0., -1., 0.]);
    bfuse(s, "result", "ba", "ob");
    checkprops(s, "result", Prop::Area, Expected::Mass(180.283));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "ob"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "170");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "ob", Prop::Area, "24.5664");
    kind_is(s, "ob", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn I8(s: &mut dyn Session) {
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
    prism(s, "ba", "fp", [0., 0., 5.]);
    profile_wire(
        s,
        "prof",
        &[
            Command::Origin([5., 3., 0.]),
            Command::Plane([0., 1., 0.], [1., 0., 0.]),
            Command::Direction(-1., 0.),
            Command::Arc(1., 180.),
            Command::X(2.),
            Command::Arc(1., 180.),
        ],
    );
    mkplane(s, "pl", "prof");
    prism(s, "ob", "pl", [0., -1., 0.]);
    bfuse(s, "result", "ba", "ob");
    checkprops(s, "result", Prop::Area, Expected::Mass(180.283));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "ob"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "170");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "ob", Prop::Area, "24.5664");
    kind_is(s, "ob", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn I9(s: &mut dyn Session) {
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
    prism(s, "ba", "fp", [0., 0., 5.]);
    profile_wire(
        s,
        "prof",
        &[
            Command::Origin([6., 3., 3.]),
            Command::Plane([0., 1., 0.], [1., 0., 0.]),
            Command::Direction(0., -1.),
            Command::Arc(1., 180.),
            Command::Y(2.),
            Command::Arc(1., 180.),
        ],
    );
    mkplane(s, "pl", "prof");
    prism(s, "ob", "pl", [0., -1., 0.]);
    bfuse(s, "result", "ba", "ob");
    checkprops(s, "result", Prop::Area, Expected::Mass(180.283));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ba", "ob"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ba", Prop::Area, "170");
    kind_is(s, "ba", ShapeEnum::Solid);
    printed_mass(s, "ob", Prop::Area, "24.5664");
    kind_is(s, "ob", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}
