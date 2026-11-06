//! The `bcut_simple` case bodies, part 2; each statement is one line of its script.
use super::*;

pub fn D3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [-0.5, -0.5, -0.5], [0.5, 0.5, 0.5]);
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(6.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn D4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [-0.5, -0.5, -0.5], [0.5, 0.5, 0.5]);
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(1.5));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn D5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0., 0., 0.], [1.5, 0.5, 0.5]);
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(5.5));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn D6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0., 0., 0.], [1.5, 0.5, 0.5]);
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(1.5));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn D7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0., -0.5, 0.], [1.5, 0.5, 0.5]);
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(6.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn D8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0., -0.5, 0.], [1.5, 0.5, 0.5]);
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(3.5));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn D9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0.25, 0., 0.], [0.5, 0.5, 1.]);
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(6.5));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn E1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0.25, 0., 0.], [0.5, 0.5, 1.]);
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b2", Prop::Area, "2.5");
    kind_is(s, "b2", ShapeEnum::Solid);
    printed_mass(s, "b1", Prop::Area, "6");
    kind_is(s, "b1", ShapeEnum::Solid);
}

pub fn E2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0.25, -0.5, 0.], [0.5, 0.5, 1.]);
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(6.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn E3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0.25, -0.5, 0.], [0.5, 0.5, 1.]);
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(2.5));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn E4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0.25, 0., 0.], [0.5, 1.5, 1.]);
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(6.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn E5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0.25, 0., 0.], [0.5, 1.5, 1.]);
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(2.5));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn E6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0.5, 0., 0.], [1., 1., 0.5]);
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(5.5));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn E7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0.5, 0., 0.], [1., 1., 0.5]);
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(2.5));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn E8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0.5, 0., -0.5], [1., 1., 0.5]);
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(6.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn E9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0.5, 0., -0.5], [1., 1., 0.5]);
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(4.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn F1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    let r = 2_f64.sqrt();
    make_box(s, "b2", [0., 0., 0.], [r, r / 2., 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(4.41421));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b1", Prop::Area, "6");
    kind_is(s, "b1", ShapeEnum::Solid);
    printed_mass(s, "b2", Prop::Area, "6.24264");
    kind_is(s, "b2", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn F2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    let r = 2_f64.sqrt();
    make_box(s, "b2", [0., 0., 0.], [r, r / 2., 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(5.82843));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b2", Prop::Area, "6.24264");
    kind_is(s, "b2", ShapeEnum::Solid);
    printed_mass(s, "b1", Prop::Area, "6");
    kind_is(s, "b1", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn F3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [0., 0., 0.], [r, r, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(5.91421));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b1", Prop::Area, "6");
    kind_is(s, "b1", ShapeEnum::Solid);
    printed_mass(s, "b2", Prop::Area, "3.82843");
    kind_is(s, "b2", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn F4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [0., 0., 0.], [r, r, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(2.91421));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b2", Prop::Area, "3.82843");
    kind_is(s, "b2", ShapeEnum::Solid);
    printed_mass(s, "b1", Prop::Area, "6");
    kind_is(s, "b1", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn F5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    let r = 2_f64.sqrt();
    make_box(s, "b2", [0., 0., 0.], [r, 0.25, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(7.03921));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b1", Prop::Area, "6");
    kind_is(s, "b1", ShapeEnum::Solid);
    printed_mass(s, "b2", Prop::Area, "4.03553");
    kind_is(s, "b2", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn F6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    let r = 2_f64.sqrt();
    make_box(s, "b2", [0., 0., 0.], [r, 0.25, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(1.83211));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b2", Prop::Area, "4.03553");
    kind_is(s, "b2", ShapeEnum::Solid);
    printed_mass(s, "b1", Prop::Area, "6");
    kind_is(s, "b1", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn F7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    let r = 31_f64.sqrt();
    make_box(s, "b2", [0., 0., 0.], [r / 4., 0.25, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 34.73);
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(7.21677));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b1", Prop::Area, "6");
    kind_is(s, "b1", ShapeEnum::Solid);
    printed_mass(s, "b2", Prop::Area, "3.97985");
    kind_is(s, "b2", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn F8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    let r = 31_f64.sqrt();
    make_box(s, "b2", [0., 0., 0.], [r / 4., 0.25, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 34.73);
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(1.54631));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b2", Prop::Area, "3.97985");
    kind_is(s, "b2", ShapeEnum::Solid);
    printed_mass(s, "b1", Prop::Area, "6");
    kind_is(s, "b1", ShapeEnum::Solid);
    view_mass_is_nonzero(s, "result");
}

pub fn F9(s: &mut dyn Session) {
    psphere(s, "s", 1.);
    make_box(s, "b", [0., 0., 0.], [1., 1., 1.]);
    bcut(s, "result", "s", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(13.3518));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["s", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn G1(s: &mut dyn Session) {
    psphere(s, "s", 1.);
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

pub fn G2(s: &mut dyn Session) {
    psphere(s, "s", 1.);
    trotate(s, "s", [0., 0., 0.], [0., 0., 1.], -90.);
    trotate(s, "s", [0., 0., 0.], [0., 1., 0.], -45.);
    make_box(s, "b", [0., 0., 0.], [1., 1., 1.]);
    bcut(s, "result", "s", "b");
    checkprops(s, "result", Prop::Area, Expected::Mass(13.3517));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["s", "b"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}
