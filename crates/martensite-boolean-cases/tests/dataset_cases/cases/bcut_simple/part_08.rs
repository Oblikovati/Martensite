//! The `bcut_simple` case bodies, part 8; each statement is one line of its script.
use super::*;

pub fn K9(s: &mut dyn Session) {
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
            Command::First(25., -125.),
            Command::X(50.),
            Command::Y(-50.),
            Command::X(-50.),
        ],
    );
    prism(s, "pr3", "p3", [0., 0., -30.]);
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
    printed_mass(s, "pr3", Prop::Area, "11000");
    kind_is(s, "pr3", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn L1(s: &mut dyn Session) {
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
            Command::First(50., 25.),
            Command::X(25.),
            Command::Y(-280.),
            Command::X(-25.),
        ],
    );
    prism(s, "pr3", "p3", [0., 0., -30.]);
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
    printed_mass(s, "pr3", Prop::Area, "32300");
    kind_is(s, "pr3", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn L2(s: &mut dyn Session) {
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
            Command::Origin([0., 0., 20.]),
            Command::First(50., 255.),
            Command::Y(-280.),
            Command::X(25.),
            Command::Y(280.),
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
    printed_mass(s, "pr3", Prop::Area, "32300");
    kind_is(s, "pr3", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn L3(s: &mut dyn Session) {
    profile(s, "f1", &[Command::Arc(60., 360.)]);
    prism(s, "p1", "f1", [0., 0., 20.]);
    profile(
        s,
        "f2",
        &[
            Command::Origin([0., 0., 20.]),
            Command::First(10., -20.),
            Command::Arc(40., 360.),
        ],
    );
    prism(s, "p2", "f2", [0., 0., -6.]);
    profile(
        s,
        "f3",
        &[
            Command::Plane([0., 0., -1.], [1., 0., 0.]),
            Command::Origin([0., 0., 23.]),
            Command::First(50., -10.),
            Command::Arc(-30., 360.),
        ],
    );
    prism(s, "p3", "f3", [0., 0., -9.]);
    bcut(s, "r1", "p1", "p2");
    bcut(s, "result", "r1", "p3");
    checkprops(s, "result", Prop::Area, Expected::Mass(30153.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["r1", "p3"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn L4(s: &mut dyn Session) {
    profile(s, "f1", &[Command::Arc(60., 360.)]);
    prism(s, "p1", "f1", [0., 0., 20.]);
    profile(
        s,
        "f2",
        &[
            Command::Origin([0., 0., 20.]),
            Command::First(10., -20.),
            Command::Arc(40., 360.),
        ],
    );
    prism(s, "p2", "f2", [0., 0., -6.]);
    profile(
        s,
        "f3",
        &[
            Command::Plane([0., 0., -1.], [1., 0., 0.]),
            Command::Origin([0., 0., 14.]),
            Command::First(50., -10.),
            Command::Arc(-30., 360.),
        ],
    );
    prism(s, "p3", "f3", [0., 0., 9.]);
    bcut(s, "r1", "p1", "p2");
    bcut(s, "result", "r1", "p3");
    checkprops(s, "result", Prop::Area, Expected::Mass(30153.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["r1", "p3"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn L5(s: &mut dyn Session) {
    profile(s, "f1", &[Command::Arc(60., 360.)]);
    prism(s, "p1", "f1", [0., 0., 20.]);
    profile(
        s,
        "f2",
        &[
            Command::Origin([0., 0., 20.]),
            Command::First(10., -20.),
            Command::Arc(40., 360.),
        ],
    );
    prism(s, "p2", "f2", [0., 0., -6.]);
    profile(
        s,
        "f3",
        &[
            Command::Origin([0., 0., 23.]),
            Command::First(50., 10.),
            Command::Arc(30., 360.),
        ],
    );
    prism(s, "p3", "f3", [0., 0., -9.]);
    bcut(s, "r1", "p1", "p2");
    bcut(s, "result", "r1", "p3");
    checkprops(s, "result", Prop::Area, Expected::Mass(30153.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["r1", "p3"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn L6(s: &mut dyn Session) {
    profile(s, "f1", &[Command::Arc(60., 360.)]);
    prism(s, "p1", "f1", [0., 0., 20.]);
    profile(
        s,
        "f2",
        &[
            Command::Origin([0., 0., 20.]),
            Command::First(10., -20.),
            Command::Arc(40., 360.),
        ],
    );
    prism(s, "p2", "f2", [0., 0., -6.]);
    profile(
        s,
        "f3",
        &[
            Command::Origin([0., 0., 14.]),
            Command::First(50., 10.),
            Command::Arc(30., 360.),
        ],
    );
    prism(s, "p3", "f3", [0., 0., 9.]);
    bcut(s, "r1", "p1", "p2");
    bcut(s, "result", "r1", "p3");
    checkprops(s, "result", Prop::Area, Expected::Mass(30153.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["r1", "p3"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn L8(s: &mut dyn Session) {
    println!("CTS21801");
    println!("cut");
    println!();
    pcylinder(s, "c1", 20., 100.);
    pcylinder(s, "c2", 20., 100.);
    ttranslate(s, "c2", [0., 0., 50.]);
    bcut(s, "result", "c1", "c2");
    checkprops(s, "result", Prop::Area, Expected::Mass(8796.46));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["c1", "c2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn M2(s: &mut dyn Session) {
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    copy(s, "b", "c");
    pcylinder(s, "s", 2., 4.);
    ttranslate(s, "s", [5., 5., -2.]);
    bcut(s, "rr", "c", "s");
    explode(s, "rr", Some(ShapeEnum::Solid));
    bcut(s, "result", "rr_1", "c");
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["c", "rr_1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "c", Prop::Area, "600");
    kind_is(s, "c", ShapeEnum::Solid);
    printed_mass(s, "s", Prop::Area, "75.3982");
    kind_is(s, "s", ShapeEnum::Solid);
    printed_mass(s, "rr_1", Prop::Area, "625.133");
    kind_is(s, "rr_1", ShapeEnum::Solid);
    part_count(s, "rr", Some(ShapeEnum::Solid), 1);
}

pub fn M3(s: &mut dyn Session) {
    make_box(s, "b", [0., 0., 0.], [10., 10., 10.]);
    copy(s, "b", "c");
    pcylinder(s, "s", 2., 4.);
    ttranslate(s, "s", [5., 5., -2.]);
    bcut(s, "rr", "c", "s");
    explode(s, "rr", Some(ShapeEnum::Solid));
    bcut(s, "result", "c", "rr_1");
    checkprops(s, "result", Prop::Area, Expected::Mass(50.2655));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["c", "rr_1"], Prop::Area),
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
