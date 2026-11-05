//! The `bcut_simple` case bodies, part 5; each statement is one line of its script.
use super::*;

pub fn I8(s: &mut dyn Session) {
    profile(
        s,
        "p1",
        &[
            Command::Plane([0., 0., -1.], [1., 0., 0.]),
            Command::Origin([0., 0., 40.]),
            Command::First(0., -50.),
            Command::Y(-100.),
            Command::X(150.),
            Command::Y(100.),
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
            Command::Y(-100.),
            Command::X(75.),
            Command::Y(100.),
        ],
    );
    prism(s, "pr3", "p3", [0., 0., -30.]);
    bcut(s, "result", "po1", "pr3");
    checkprops(s, "result", Prop::Area, Expected::Mass(52000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["po1", "pr3"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "pr1", Prop::Area, "50000");
    printed_mass(s, "pr2", Prop::Area, "25500");
    printed_mass(s, "po1", Prop::Area, "53000");
    printed_mass(s, "pr3", Prop::Area, "25500");
    view_mass_is_nonzero(s, "result");
}

pub fn I9(s: &mut dyn Session) {
    profile(
        s,
        "p1",
        &[
            Command::Plane([0., 0., -1.], [1., 0., 0.]),
            Command::Origin([0., 0., 40.]),
            Command::First(0., -50.),
            Command::Y(-100.),
            Command::X(150.),
            Command::Y(100.),
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
            Command::Origin([0., 0., 50.]),
            Command::First(50., 75.),
            Command::X(75.),
            Command::Y(100.),
            Command::X(-75.),
        ],
    );
    prism(s, "pr3", "p3", [0., 0., -30.]);
    bcut(s, "result", "po1", "pr3");
    checkprops(s, "result", Prop::Area, Expected::Mass(52000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["po1", "pr3"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "pr1", Prop::Area, "50000");
    printed_mass(s, "pr2", Prop::Area, "25500");
    printed_mass(s, "po1", Prop::Area, "53000");
    printed_mass(s, "pr3", Prop::Area, "25500");
    view_mass_is_nonzero(s, "result");
}

pub fn J1(s: &mut dyn Session) {
    profile(
        s,
        "p1",
        &[
            Command::Plane([0., 0., -1.], [1., 0., 0.]),
            Command::Origin([0., 0., 40.]),
            Command::First(0., -50.),
            Command::Y(-100.),
            Command::X(150.),
            Command::Y(100.),
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
            Command::Origin([0., 0., 50.]),
            Command::First(50., 75.),
            Command::Y(100.),
            Command::X(75.),
            Command::Y(-100.),
        ],
    );
    prism(s, "pr3", "p3", [0., 0., -3.]);
    bcut(s, "result", "po1", "pr3");
    checkprops(s, "result", Prop::Area, Expected::Mass(53000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["po1", "pr3"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "pr1", Prop::Area, "50000");
    printed_mass(s, "pr2", Prop::Area, "25500");
    printed_mass(s, "po1", Prop::Area, "53000");
    printed_mass(s, "pr3", Prop::Area, "16050");
    view_mass_is_nonzero(s, "result");
}

pub fn J2(s: &mut dyn Session) {
    profile(
        s,
        "p1",
        &[
            Command::Origin([0., 0., 40.]),
            Command::X(175.),
            Command::Y(250.),
            Command::X(-175.),
        ],
    );
    prism(s, "pr1", "p1", [0., 0., -40.]);
    profile(
        s,
        "p2",
        &[
            Command::Origin([0., 0., 50.]),
            Command::First(25., 25.),
            Command::Y(75.),
            Command::X(50.),
            Command::Y(-75.),
        ],
    );
    prism(s, "pr2", "p2", [0., 0., -30.]);
    profile(
        s,
        "p3",
        &[
            Command::Origin([0., 0., 50.]),
            Command::First(100., 150.),
            Command::Y(75.),
            Command::X(50.),
            Command::Y(-75.),
        ],
    );
    prism(s, "pr3", "p3", [0., 0., -30.]);
    bcut(s, "po1", "pr1", "pr2");
    bcut(s, "po2", "po1", "pr3");
    profile(
        s,
        "p4",
        &[
            Command::Plane([0., 0., -1.], [1., 0., 0.]),
            Command::Origin([0., 0., 50.]),
            Command::First(50., -75.),
            Command::X(75.),
            Command::Y(-100.),
            Command::X(-75.),
        ],
    );
    prism(s, "pr4", "p4", [0., 0., -30.]);
    bcut(s, "result", "po2", "pr4");
    checkprops(s, "result", Prop::Area, Expected::Mass(134500.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["po2", "pr4"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "pr1", Prop::Area, "121500");
    printed_mass(s, "pr2", Prop::Area, "15000");
    printed_mass(s, "po1", Prop::Area, "126500");
    printed_mass(s, "pr3", Prop::Area, "15000");
    printed_mass(s, "po2", Prop::Area, "131500");
    printed_mass(s, "pr4", Prop::Area, "25500");
    view_mass_is_nonzero(s, "result");
}

pub fn J3(s: &mut dyn Session) {
    profile(
        s,
        "p1",
        &[
            Command::Origin([0., 0., 40.]),
            Command::X(175.),
            Command::Y(250.),
            Command::X(-175.),
        ],
    );
    prism(s, "pr1", "p1", [0., 0., -40.]);
    profile(
        s,
        "p2",
        &[
            Command::Origin([0., 0., 50.]),
            Command::First(25., 25.),
            Command::Y(75.),
            Command::X(50.),
            Command::Y(-75.),
        ],
    );
    prism(s, "pr2", "p2", [0., 0., -30.]);
    profile(
        s,
        "p3",
        &[
            Command::Plane([0., 0., -1.], [1., 0., 0.]),
            Command::Origin([0., 0., 50.]),
            Command::First(100., -150.),
            Command::Y(-75.),
            Command::X(50.),
            Command::Y(75.),
        ],
    );
    prism(s, "pr3", "p3", [0., 0., -30.]);
    bcut(s, "po1", "pr1", "pr2");
    bcut(s, "po2", "po1", "pr3");
    profile(
        s,
        "p4",
        &[
            Command::Plane([0., 0., -1.], [1., 0., 0.]),
            Command::Origin([0., 0., 50.]),
            Command::First(50., -75.),
            Command::X(75.),
            Command::Y(-100.),
            Command::X(-75.),
        ],
    );
    prism(s, "pr4", "p4", [0., 0., -30.]);
    ttranslate(s, "pr4", [-10., 0., 0.]);
    bcut(s, "result", "po2", "pr4");
    checkprops(s, "result", Prop::Area, Expected::Mass(134500.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["po2", "pr4"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "pr1", Prop::Area, "121500");
    printed_mass(s, "pr2", Prop::Area, "15000");
    printed_mass(s, "po1", Prop::Area, "126500");
    printed_mass(s, "pr3", Prop::Area, "15000");
    printed_mass(s, "po2", Prop::Area, "131500");
    printed_mass(s, "pr4", Prop::Area, "25500");
    view_mass_is_nonzero(s, "result");
}

pub fn J4(s: &mut dyn Session) {
    profile(
        s,
        "p1",
        &[
            Command::Origin([0., 0., 40.]),
            Command::X(175.),
            Command::Y(250.),
            Command::X(-175.),
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
            Command::Y(-75.),
            Command::X(50.),
            Command::Y(75.),
        ],
    );
    prism(s, "pr2", "p2", [0., 0., -30.]);
    profile(
        s,
        "p3",
        &[
            Command::Plane([0., 0., -1.], [1., 0., 0.]),
            Command::Origin([0., 0., 50.]),
            Command::First(100., -150.),
            Command::Y(-75.),
            Command::X(50.),
            Command::Y(75.),
        ],
    );
    prism(s, "pr3", "p3", [0., 0., -30.]);
    bcut(s, "po1", "pr1", "pr2");
    bcut(s, "po2", "po1", "pr3");
    profile(
        s,
        "p4",
        &[
            Command::Plane([0., 0., -1.], [1., 0., 0.]),
            Command::Origin([0., 0., 50.]),
            Command::First(50., -75.),
            Command::X(75.),
            Command::Y(-100.),
            Command::X(-75.),
        ],
    );
    prism(s, "pr4", "p4", [0., 0., -30.]);
    bcut(s, "result", "po2", "pr4");
    checkprops(s, "result", Prop::Area, Expected::Mass(134500.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["po2", "pr4"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "pr1", Prop::Area, "121500");
    kind_is(s, "pr1", ShapeEnum::Solid);
    printed_mass(s, "pr2", Prop::Area, "15000");
    kind_is(s, "pr2", ShapeEnum::Solid);
    printed_mass(s, "po1", Prop::Area, "126500");
    kind_is(s, "po1", ShapeEnum::Compound);
    printed_mass(s, "pr3", Prop::Area, "15000");
    kind_is(s, "pr3", ShapeEnum::Solid);
    printed_mass(s, "po2", Prop::Area, "131500");
    kind_is(s, "po2", ShapeEnum::Compound);
    printed_mass(s, "pr4", Prop::Area, "25500");
    kind_is(s, "pr4", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}
