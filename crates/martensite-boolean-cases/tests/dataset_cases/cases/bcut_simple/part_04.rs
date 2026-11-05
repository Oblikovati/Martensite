//! The `bcut_simple` case bodies, part 4; each statement is one line of its script.
use super::*;

pub fn H9(s: &mut dyn Session) {
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

pub fn I1(s: &mut dyn Session) {
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

pub fn I2(s: &mut dyn Session) {
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

pub fn I3(s: &mut dyn Session) {
    profile(
        s,
        "p1",
        &[
            Command::Origin([0., 0., 40.]),
            Command::First(0., 50.),
            Command::X(150.),
            Command::Y(100.),
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

pub fn I4(s: &mut dyn Session) {
    profile(
        s,
        "p1",
        &[
            Command::Origin([0., 0., 40.]),
            Command::First(0., 50.),
            Command::X(150.),
            Command::Y(100.),
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

pub fn I5(s: &mut dyn Session) {
    profile(
        s,
        "p1",
        &[
            Command::Origin([0., 0., 40.]),
            Command::First(0., 50.),
            Command::X(150.),
            Command::Y(100.),
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

pub fn I6(s: &mut dyn Session) {
    profile(
        s,
        "p1",
        &[
            Command::Origin([0., 0., 40.]),
            Command::First(0., 50.),
            Command::X(150.),
            Command::Y(100.),
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

pub fn I7(s: &mut dyn Session) {
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
            Command::X(75.),
            Command::Y(-100.),
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
