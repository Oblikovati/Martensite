//! The `bcut_simple` case bodies, part 3; each statement is one line of its script.
use super::*;

pub fn G3(s: &mut dyn Session) {
    psphere(s, "s", 1.);
    trotate(s, "s", [0., 0., 0.], [0., 0., 1.], -90.);
    trotate(s, "s", [0., 0., 0.], [0., 1., 0.], -45.);
    make_box(s, "b", [0., 0., 0.], [1., 1., 1.]);
    bcut(s, "result", "b", "s");
    checkprops(s, "result", Prop::Area, Expected::Mass(5.2146));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b", "s"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn G4(s: &mut dyn Session) {
    psphere(s, "s", 1.);
    make_box(s, "b", [0., 0., 0.], [1., 1., 1.]);
    trotate(s, "b", [0., 0., 1.], [0., 1., 0.], 90.);
    bcut(s, "result", "s", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(13.3517));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["s", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn G5(s: &mut dyn Session) {
    psphere(s, "s", 1.);
    make_box(s, "b", [0., 0., 0.], [1., 1., 1.]);
    trotate(s, "b", [0., 0., 1.], [0., 1., 0.], 90.);
    bcut(s, "result", "b", "s");
    checkprops(s, "result", Prop::Area, Expected::Mass(5.2146));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b", "s"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn G7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [3., 3., 3.]);
    make_box(s, "b2", [0., 0., 0.], [1., 1., 1.]);
    ttranslate(s, "b2", [0., 3., 0.]);
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(54.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn G8(s: &mut dyn Session) {
    make_box(s, "mb", [-0.5, -0.5, -0.5], [1., 1., 1.]);
    explode(s, "mb", Some(ShapeEnum::Face));
    prism_semi_infinite(s, "pryz", "mb_1", [1., 0., 0.]);
    make_box(s, "ab", [0., -1., -1.], [2., 2., 2.]);
    bcut(s, "result", "ab", "pryz");
    checkprops(s, "result", Prop::Area, Expected::Mass(30.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["ab", "pryz"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "ab", Prop::Area, "24");
    kind_is(s, "ab", ShapeEnum::Solid);
    printed_mass(s, "pryz", Prop::Area, "8e+100");
    kind_is(s, "pryz", ShapeEnum::Solid);
    part_count(s, "mb", Some(ShapeEnum::Face), 6);
    view_mass_is_nonzero(s, "result");
}

pub fn G9(s: &mut dyn Session) {
    pcylinder(s, "cyl", 9., 3.);
    pcone(s, "kone", 7., 6., 4.);
    bfuse(s, "body", "cyl", "kone");
    pcylinder(s, "pcyl", 1., 9.);
    ttranslate(s, "pcyl", [5., 0., -2.]);
    bcut(s, "result", "body", "pcyl");
    checkprops(s, "result", Prop::Area, Expected::Mass(727.481));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["body", "pcyl"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn H1(s: &mut dyn Session) {
    vertex(s, "v1", [0., 0., 0.]);
    vertex(s, "v2", [1., 0., 0.]);
    vertex(s, "v3", [1., 3., 0.]);
    vertex(s, "v4", [2., 3., 0.]);
    vertex(s, "v5", [2., 0., 0.]);
    vertex(s, "v6", [3., 0., 0.]);
    vertex(s, "v7", [3., 5., 0.]);
    vertex(s, "v8", [0., 5., 0.]);
    edge(s, "e1", "v1", "v2");
    edge(s, "e2", "v2", "v3");
    edge(s, "e3", "v3", "v4");
    edge(s, "e4", "v4", "v5");
    edge(s, "e5", "v5", "v6");
    edge(s, "e6", "v6", "v7");
    edge(s, "e7", "v7", "v8");
    edge(s, "e8", "v8", "v1");
    wire(s, "w", &["e1", "e2", "e3", "e4", "e5", "e6", "e7", "e8"]);
    mkplane(s, "p", "w");
    prism(s, "sol", "p", [0., 0., 2.]);
    make_box(s, "b", [-1., 2., 1.], [5., 1., 3.]);
    bcut(s, "result", "sol", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(68.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["sol", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn H2(s: &mut dyn Session) {
    vertex(s, "v1", [0., 0., 0.]);
    vertex(s, "v2", [1., 0., 0.]);
    vertex(s, "v3", [1., 3., 0.]);
    vertex(s, "v4", [2., 3., 0.]);
    vertex(s, "v5", [2., 0., 0.]);
    vertex(s, "v6", [3., 0., 0.]);
    vertex(s, "v7", [3., 5., 0.]);
    vertex(s, "v8", [0., 5., 0.]);
    edge(s, "e1", "v1", "v2");
    edge(s, "e2", "v2", "v3");
    edge(s, "e3", "v3", "v4");
    edge(s, "e4", "v4", "v5");
    edge(s, "e5", "v5", "v6");
    edge(s, "e6", "v6", "v7");
    edge(s, "e7", "v7", "v8");
    edge(s, "e8", "v8", "v1");
    wire(s, "w", &["e1", "e2", "e3", "e4", "e5", "e6", "e7", "e8"]);
    mkplane(s, "p", "w");
    prism(s, "sol", "p", [0., 0., 2.]);
    make_box(s, "b", [-1., 2., 1.], [5., 1., 3.]);
    bcut(s, "result", "sol", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(68.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["sol", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn H3(s: &mut dyn Session) {
    pcylinder(s, "cyl", 9., 3.);
    pcone(s, "kone", 7., 6., 4.);
    bfuse(s, "body", "cyl", "kone");
    pcylinder(s, "pcyl", 1., 9.);
    ttranslate(s, "pcyl", [5., 0., -2.]);
    bcut(s, "result", "body", "pcyl");
    checkprops(s, "result", Prop::Area, Expected::Mass(727.481));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["body", "pcyl"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn H4(s: &mut dyn Session) {
    profile(
        s,
        "p1",
        &[
            Command::Origin([0., 0., 40.]),
            Command::X(150.),
            Command::Y(200.),
            Command::X(-150.),
        ],
    );
    prism(s, "pr1", "p1", [0., 0., -40.]);
    profile(
        s,
        "p2",
        &[
            Command::Origin([0., 0., 50.]),
            Command::First(25., 25.),
            Command::Y(100.),
            Command::X(75.),
            Command::Y(-100.),
        ],
    );
    prism(s, "pr2", "p2", [0., 0., -30.]);
    bcut(s, "po1", "pr1", "pr2");
    profile(
        s,
        "p3",
        &[
            Command::Plane([0., 0., -1.], [1., 0., 0.]),
            Command::Origin([0., 0., 50.]),
            Command::First(50., -75.),
            Command::X(75.),
            Command::Y(-100.),
            Command::X(-75.),
        ],
    );
    prism(s, "pr3", "p3", [0., 0., -30.]);
    bcut(s, "result", "po1", "pr3");
    checkprops(s, "result", Prop::Area, Expected::Mass(98000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["po1", "pr3"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "pr1", Prop::Area, "88000");
    printed_mass(s, "pr2", Prop::Area, "25500");
    printed_mass(s, "po1", Prop::Area, "95000");
    printed_mass(s, "pr3", Prop::Area, "25500");
    view_mass_is_nonzero(s, "result");
}

pub fn H5(s: &mut dyn Session) {
    profile(
        s,
        "p1",
        &[
            Command::Origin([0., 0., 40.]),
            Command::X(150.),
            Command::Y(200.),
            Command::X(-150.),
        ],
    );
    prism(s, "pr1", "p1", [0., 0., -40.]);
    profile(
        s,
        "p2",
        &[
            Command::Origin([0., 0., 50.]),
            Command::First(25., 25.),
            Command::Y(100.),
            Command::X(75.),
            Command::Y(-100.),
        ],
    );
    prism(s, "pr2", "p2", [0., 0., -30.]);
    bcut(s, "po1", "pr1", "pr2");
    profile(
        s,
        "p3",
        &[
            Command::Plane([0., 0., -1.], [1., 0., 0.]),
            Command::Origin([0., 0., 50.]),
            Command::First(50., -75.),
            Command::Y(-100.),
            Command::X(75.),
            Command::Y(100.),
        ],
    );
    prism(s, "pr3", "p3", [0., 0., -30.]);
    bcut(s, "result", "po1", "pr3");
    checkprops(s, "result", Prop::Area, Expected::Mass(98000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["po1", "pr3"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "pr1", Prop::Area, "88000");
    printed_mass(s, "pr2", Prop::Area, "25500");
    printed_mass(s, "po1", Prop::Area, "95000");
    printed_mass(s, "pr3", Prop::Area, "25500");
    view_mass_is_nonzero(s, "result");
}

pub fn H6(s: &mut dyn Session) {
    profile(
        s,
        "p1",
        &[
            Command::Origin([0., 0., 40.]),
            Command::X(150.),
            Command::Y(200.),
            Command::X(-150.),
        ],
    );
    prism(s, "pr1", "p1", [0., 0., -40.]);
    profile(
        s,
        "p2",
        &[
            Command::Origin([0., 0., 50.]),
            Command::First(25., 25.),
            Command::Y(100.),
            Command::X(75.),
            Command::Y(-100.),
        ],
    );
    prism(s, "pr2", "p2", [0., 0., -30.]);
    bcut(s, "po1", "pr1", "pr2");
    profile(
        s,
        "p3",
        &[
            Command::Origin([0., 0., 50.]),
            Command::First(50., 75.),
            Command::X(75.),
            Command::Y(100.),
            Command::X(-75.),
        ],
    );
    prism(s, "pr3", "p3", [0., 0., -30.]);
    bcut(s, "result", "po1", "pr3");
    checkprops(s, "result", Prop::Area, Expected::Mass(98000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["po1", "pr3"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "pr1", Prop::Area, "88000");
    printed_mass(s, "pr2", Prop::Area, "25500");
    printed_mass(s, "po1", Prop::Area, "95000");
    printed_mass(s, "pr3", Prop::Area, "25500");
    view_mass_is_nonzero(s, "result");
}

pub fn H7(s: &mut dyn Session) {
    profile(
        s,
        "p1",
        &[
            Command::Origin([0., 0., 40.]),
            Command::X(150.),
            Command::Y(200.),
            Command::X(-150.),
        ],
    );
    prism(s, "pr1", "p1", [0., 0., -40.]);
    profile(
        s,
        "p2",
        &[
            Command::Origin([0., 0., 50.]),
            Command::First(25., 25.),
            Command::Y(100.),
            Command::X(75.),
            Command::Y(-100.),
        ],
    );
    prism(s, "pr2", "p2", [0., 0., -30.]);
    bcut(s, "po1", "pr1", "pr2");
    profile(
        s,
        "p3",
        &[
            Command::Origin([0., 0., 50.]),
            Command::First(50., 75.),
            Command::Y(100.),
            Command::X(75.),
            Command::Y(-100.),
        ],
    );
    prism(s, "pr3", "p3", [0., 0., -30.]);
    bcut(s, "result", "po1", "pr3");
    checkprops(s, "result", Prop::Area, Expected::Mass(98000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["po1", "pr3"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "pr1", Prop::Area, "88000");
    printed_mass(s, "pr2", Prop::Area, "25500");
    printed_mass(s, "po1", Prop::Area, "95000");
    printed_mass(s, "pr3", Prop::Area, "25500");
    view_mass_is_nonzero(s, "result");
}

pub fn H8(s: &mut dyn Session) {
    profile(
        s,
        "p1",
        &[
            Command::Plane([0., 0., -1.], [1., 0., 0.]),
            Command::Origin([0., 0., 40.]),
            Command::Y(-200.),
            Command::X(150.),
            Command::Y(200.),
        ],
    );
    prism(s, "pr1", "p1", [0., 0., -40.]);
    profile(
        s,
        "p2",
        &[
            Command::Plane([0., 0., -1.], [1., 0., 0.]),
            Command::Origin([0., 0., 50.]),
            Command::First(25., -25.),
            Command::X(75.),
            Command::Y(-100.),
            Command::X(-75.),
        ],
    );
    prism(s, "pr2", "p2", [0., 0., -30.]);
    bcut(s, "po1", "pr1", "pr2");
    profile(
        s,
        "p3",
        &[
            Command::Plane([0., 0., -1.], [1., 0., 0.]),
            Command::Origin([0., 0., 50.]),
            Command::First(50., -75.),
            Command::X(75.),
            Command::Y(-100.),
            Command::X(-75.),
        ],
    );
    prism(s, "pr3", "p3", [0., 0., -30.]);
    bcut(s, "result", "po1", "pr3");
    checkprops(s, "result", Prop::Area, Expected::Mass(98000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["po1", "pr3"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "pr1", Prop::Area, "88000");
    printed_mass(s, "pr2", Prop::Area, "25500");
    printed_mass(s, "po1", Prop::Area, "95000");
    printed_mass(s, "pr3", Prop::Area, "25500");
    view_mass_is_nonzero(s, "result");
}
