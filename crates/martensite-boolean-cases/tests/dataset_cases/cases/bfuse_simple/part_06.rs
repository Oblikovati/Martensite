//! The `bfuse_simple` case bodies, part 6; each statement is one line of its script.
use super::*;

pub fn J1(s: &mut dyn Session) {
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
            Command::Origin([8., 4., 0.]),
            Command::Plane([-1., 0., 0.], [0., 1., 0.]),
            Command::Direction(-1., 0.),
            Command::Arc(1., 180.),
            Command::X(2.),
            Command::Arc(1., 180.),
        ],
    );
    mkplane(s, "pl", "prof");
    prism(s, "ob", "pl", [1., 0., 0.]);
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

pub fn J2(s: &mut dyn Session) {
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
            Command::Origin([8., 3., 3.]),
            Command::Plane([-1., 0., 0.], [0., 1., 0.]),
            Command::Direction(0., -1.),
            Command::Arc(1., 180.),
            Command::Y(2.),
            Command::Arc(1., 180.),
        ],
    );
    mkplane(s, "pl", "prof");
    prism(s, "ob", "pl", [1., 0., 0.]);
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

pub fn J3(s: &mut dyn Session) {
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
            Command::Origin([8., 6., 0.]),
            Command::Plane([-1., 0., 0.], [0., 1., 0.]),
            Command::Direction(-1., 0.),
            Command::Arc(1., 180.),
            Command::X(2.),
            Command::Arc(1., 180.),
        ],
    );
    mkplane(s, "pl", "prof");
    prism(s, "ob", "pl", [1., 0., 0.]);
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

pub fn J4(s: &mut dyn Session) {
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
            Command::Origin([8., 7., 3.]),
            Command::Plane([-1., 0., 0.], [0., 1., 0.]),
            Command::Direction(0., -1.),
            Command::Arc(1., 180.),
            Command::Y(2.),
            Command::Arc(1., 180.),
        ],
    );
    mkplane(s, "pl", "prof");
    prism(s, "ob", "pl", [1., 0., 0.]);
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

pub fn J5(s: &mut dyn Session) {
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
            Command::Origin([4., 3., 5.]),
            Command::Plane([0., 0., -1.], [1., 0., 0.]),
            Command::Direction(-1., 0.),
            Command::Arc(1., 180.),
            Command::X(2.),
            Command::Arc(1., 180.),
        ],
    );
    mkplane(s, "pl", "prof");
    prism(s, "ob", "pl", [0., 0., 1.]);
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

pub fn J6(s: &mut dyn Session) {
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
            Command::Origin([3., 6., 5.]),
            Command::Plane([0., 0., -1.], [1., 0., 0.]),
            Command::Direction(0., -1.),
            Command::Arc(1., 180.),
            Command::Y(2.),
            Command::Arc(1., 180.),
        ],
    );
    vertex(s, "v", [3., 6., 5.]);
    mkplane(s, "pl", "prof");
    prism(s, "ob", "pl", [0., 0., 1.]);
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

pub fn J7(s: &mut dyn Session) {
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
            Command::Origin([5., 3., 5.]),
            Command::Plane([0., 0., -1.], [1., 0., 0.]),
            Command::Direction(-1., 0.),
            Command::Arc(1., 180.),
            Command::X(2.),
            Command::Arc(1., 180.),
        ],
    );
    mkplane(s, "pl", "prof");
    prism(s, "ob", "pl", [0., 0., 1.]);
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

pub fn J8(s: &mut dyn Session) {
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
            Command::Origin([8., 4., 5.]),
            Command::Plane([0., 0., -1.], [1., 0., 0.]),
            Command::Direction(0., 1.),
            Command::Arc(1., 180.),
            Command::Y(-2.),
            Command::Arc(1., 180.),
        ],
    );
    mkplane(s, "pl", "prof");
    prism(s, "ob", "pl", [0., 0., 1.]);
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

pub fn J9(s: &mut dyn Session) {
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

pub fn K1(s: &mut dyn Session) {
    make_box(s, "b", [0., 0., 0.], [200., 200., 100.]);
    profile(
        s,
        "wr",
        &[
            Command::Plane([1., 0., 0.], [0., 0., 1.]),
            Command::Origin([50., 0., 0.]),
            Command::First(50., -80.),
            Command::X(50.),
            Command::Y(5.),
            Command::X(10.),
            Command::Y(-25.),
            Command::X(-60.),
        ],
    );
    revol(s, "rv", "wr", [50., 100., 50.], [0., 0., 1.], 360.);
    bfuse(s, "result", "b", "rv");
    checkprops(s, "result", Prop::Area, Expected::Mass(161571.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b", "rv"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b", Prop::Area, "160000");
    kind_is(s, "b", ShapeEnum::Solid);
    printed_mass(s, "rv", Prop::Area, "11781");
    kind_is(s, "rv", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn L2(s: &mut dyn Session) {
    println!("CTS21801");
    println!("fuse");
    println!();
    pcylinder(s, "c1", 20., 100.);
    pcylinder(s, "c2", 20., 100.);
    ttranslate(s, "c2", [0., 0., 50.]);
    bfuse(s, "result", "c1", "c2");
    checkprops(s, "result", Prop::Area, Expected::Mass(21362.8));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["c1", "c2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn L3(s: &mut dyn Session) {
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    copy(s, "b", "c");
    pcylinder(s, "s", 2., 4.);
    ttranslate(s, "s", [5., 5., -2.]);
    bcut(s, "rr", "c", "s");
    explode(s, "rr", Some(ShapeEnum::Solid));
    bfuse(s, "result", "rr_1", "c");
    checkprops(s, "result", Prop::Area, Expected::Mass(600.));
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
