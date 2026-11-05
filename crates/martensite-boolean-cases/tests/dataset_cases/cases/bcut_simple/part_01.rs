//! The `bcut_simple` case bodies, part 1; each statement is one line of its script.
use super::*;

pub fn A1(s: &mut dyn Session) {
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

pub fn A2(s: &mut dyn Session) {
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

pub fn A3(s: &mut dyn Session) {
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

pub fn A4(s: &mut dyn Session) {
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

pub fn A5(s: &mut dyn Session) {
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

pub fn A6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0., 0., 0.], [1., 1., 1.]);
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b1", Prop::Area, "6");
    kind_is(s, "b1", ShapeEnum::Solid);
    printed_mass(s, "b2", Prop::Area, "6");
    kind_is(s, "b2", ShapeEnum::Solid);
}

pub fn A7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0., 0., 0.], [1., 1., 1.]);
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b2", Prop::Area, "6");
    kind_is(s, "b2", ShapeEnum::Solid);
    printed_mass(s, "b1", Prop::Area, "6");
    kind_is(s, "b1", ShapeEnum::Solid);
}

pub fn A8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0., 0., 0.], [1., 1.5, 1.]);
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b1", Prop::Area, "6");
    kind_is(s, "b1", ShapeEnum::Solid);
    printed_mass(s, "b2", Prop::Area, "8");
    kind_is(s, "b2", ShapeEnum::Solid);
}

pub fn A9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0., 0., 0.], [1., 1.5, 1.]);
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(4.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn B1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0., 1., 0.], [1., 0.5, 1.]);
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(6.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn B2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0., 1., 0.], [1., 0.5, 1.]);
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(4.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn B3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [1., 1., 0.], [1., 1., 1.]);
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(6.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn B4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [1., 1., 0.], [1., 1., 1.]);
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(6.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn B5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0., 0., 0.], [0.5, 1., 0.5]);
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(5.5));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn B6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0., 0., 0.], [0.5, 1., 0.5]);
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b2", Prop::Area, "2.5");
    kind_is(s, "b2", ShapeEnum::Solid);
    printed_mass(s, "b1", Prop::Area, "6");
    kind_is(s, "b1", ShapeEnum::Solid);
}

pub fn B7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0., -0.5, 0.], [0.5, 0.5, 1.]);
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(6.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn B8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0., -0.5, 0.], [0.5, 0.5, 1.]);
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(2.5));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn B9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0., -0.5, 0.], [0.5, 1.5, 1.]);
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(4.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn C1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0., -0.5, 0.], [0.5, 1.5, 1.]);
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(2.5));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn C2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0., 0.5, 0.], [1., 1., 1.]);
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(4.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn C3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0., 0.5, 0.], [1., 1., 1.]);
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(4.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn C4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0., 0.25, 0.], [1., 0.5, 1.]);
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(6.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn C5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0., 0.25, 0.], [1., 0.5, 1.]);
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b2", Prop::Area, "4");
    kind_is(s, "b2", ShapeEnum::Solid);
    printed_mass(s, "b1", Prop::Area, "6");
    kind_is(s, "b1", ShapeEnum::Solid);
}

pub fn C6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0., 0., 0.], [0.5, 0.5, 0.5]);
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(6.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn C7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0., 0., 0.], [0.5, 0.5, 0.5]);
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Empty);
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "b2", Prop::Area, "1.5");
    kind_is(s, "b2", ShapeEnum::Solid);
    printed_mass(s, "b1", Prop::Area, "6");
    kind_is(s, "b1", ShapeEnum::Solid);
}

pub fn C8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0., -0.5, 0.], [0.5, 0.5, 0.5]);
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(6.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn C9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0., -0.5, 0.], [0.5, 0.5, 0.5]);
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(1.5));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn D1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0., -0.5, -0.5], [0.5, 0.5, 0.5]);
    bcut(s, "result", "b1", "b2");
    checkprops(s, "result", Prop::Area, Expected::Mass(6.));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b1", "b2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}

pub fn D2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    nurbsconvert(s, "b1", "b1");
    make_box(s, "b2", [0., -0.5, -0.5], [0.5, 0.5, 0.5]);
    bcut(s, "result", "b2", "b1");
    checkprops(s, "result", Prop::Area, Expected::Mass(1.5));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["b2", "b1"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    view_mass_is_nonzero(s, "result");
}
