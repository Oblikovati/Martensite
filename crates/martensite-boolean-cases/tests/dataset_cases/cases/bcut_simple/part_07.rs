//! The `bcut_simple` case bodies, part 7; each statement is one line of its script.
use super::*;

pub fn K2(s: &mut dyn Session) {
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
            Command::Origin([0., 0., -10.]),
            Command::First(50., -75.),
            Command::X(75.),
            Command::Y(-100.),
            Command::X(-75.),
        ],
    );
    prism(s, "pr3", "p3", [0., 0., 30.]);
    bcut(s, "result", "po1", "pr3");
    checkprops(s, "result", Prop::Area, Expected::Mass(97000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["po1", "pr3"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "pr1", Prop::Area, "88000");
    kind_is(s, "pr1", ShapeEnum::Solid);
    printed_mass(s, "pr2", Prop::Area, "25500");
    kind_is(s, "pr2", ShapeEnum::Solid);
    printed_mass(s, "po1", Prop::Area, "95000");
    kind_is(s, "po1", ShapeEnum::Compound);
    printed_mass(s, "pr3", Prop::Area, "25500");
    kind_is(s, "pr3", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn K3(s: &mut dyn Session) {
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
            Command::Origin([0., 0., -10.]),
            Command::First(50., 75.),
            Command::Y(100.),
            Command::X(75.),
            Command::Y(-100.),
        ],
    );
    prism(s, "pr3", "p3", [0., 0., 30.]);
    bcut(s, "result", "po1", "pr3");
    checkprops(s, "result", Prop::Area, Expected::Mass(97000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["po1", "pr3"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "pr1", Prop::Area, "88000");
    kind_is(s, "pr1", ShapeEnum::Solid);
    printed_mass(s, "pr2", Prop::Area, "25500");
    kind_is(s, "pr2", ShapeEnum::Solid);
    printed_mass(s, "po1", Prop::Area, "95000");
    kind_is(s, "po1", ShapeEnum::Compound);
    printed_mass(s, "pr3", Prop::Area, "25500");
    kind_is(s, "pr3", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn K4(s: &mut dyn Session) {
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
            Command::Origin([0., 0., -10.]),
            Command::First(50., 75.),
            Command::X(75.),
            Command::Y(100.),
            Command::X(-75.),
        ],
    );
    prism(s, "pr3", "p3", [0., 0., 30.]);
    bcut(s, "result", "po1", "pr3");
    checkprops(s, "result", Prop::Area, Expected::Mass(97000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["po1", "pr3"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "pr1", Prop::Area, "88000");
    kind_is(s, "pr1", ShapeEnum::Solid);
    printed_mass(s, "pr2", Prop::Area, "25500");
    kind_is(s, "pr2", ShapeEnum::Solid);
    printed_mass(s, "po1", Prop::Area, "95000");
    kind_is(s, "po1", ShapeEnum::Compound);
    printed_mass(s, "pr3", Prop::Area, "25500");
    kind_is(s, "pr3", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn K5(s: &mut dyn Session) {
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
            Command::Origin([0., 0., -10.]),
            Command::First(50., -75.),
            Command::Y(-100.),
            Command::X(75.),
            Command::Y(100.),
        ],
    );
    prism(s, "pr3", "p3", [0., 0., 30.]);
    bcut(s, "result", "po1", "pr3");
    checkprops(s, "result", Prop::Area, Expected::Mass(97000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["po1", "pr3"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "pr1", Prop::Area, "88000");
    kind_is(s, "pr1", ShapeEnum::Solid);
    printed_mass(s, "pr2", Prop::Area, "25500");
    kind_is(s, "pr2", ShapeEnum::Solid);
    printed_mass(s, "po1", Prop::Area, "95000");
    kind_is(s, "po1", ShapeEnum::Compound);
    printed_mass(s, "pr3", Prop::Area, "25500");
    kind_is(s, "pr3", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn K6(s: &mut dyn Session) {
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
            Command::Origin([0., 0., -10.]),
            Command::First(50., -75.),
            Command::X(75.),
            Command::Y(-100.),
            Command::X(-75.),
        ],
    );
    prism(s, "pr3", "p3", [0., 0., 30.]);
    bcut(s, "result", "po1", "pr3");
    checkprops(s, "result", Prop::Area, Expected::Mass(97000.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["po1", "pr3"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "pr1", Prop::Area, "88000");
    kind_is(s, "pr1", ShapeEnum::Solid);
    printed_mass(s, "pr2", Prop::Area, "25500");
    kind_is(s, "pr2", ShapeEnum::Solid);
    printed_mass(s, "po1", Prop::Area, "95000");
    kind_is(s, "po1", ShapeEnum::Compound);
    printed_mass(s, "pr3", Prop::Area, "25500");
    kind_is(s, "pr3", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn K7(s: &mut dyn Session) {
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
            Command::Origin([0., 0., 20.]),
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
            Command::Origin([0., 0., 25.]),
            Command::First(50., 75.),
            Command::X(75.),
            Command::Y(100.),
            Command::X(-75.),
        ],
    );
    prism(s, "pr4", "p4", [0., 0., -5.]);
    bcut(s, "result", "po2", "pr4");
    checkprops(s, "result", Prop::Area, Expected::Mass(145250.));
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
    printed_mass(s, "pr4", Prop::Area, "16750");
    kind_is(s, "pr4", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn K8(s: &mut dyn Session) {
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
            Command::First(50., -125.),
            Command::X(75.),
            Command::Y(-50.),
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
    kind_is(s, "pr1", ShapeEnum::Solid);
    printed_mass(s, "pr2", Prop::Area, "25500");
    kind_is(s, "pr2", ShapeEnum::Solid);
    printed_mass(s, "po1", Prop::Area, "95000");
    kind_is(s, "po1", ShapeEnum::Compound);
    printed_mass(s, "pr3", Prop::Area, "15000");
    kind_is(s, "pr3", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}
