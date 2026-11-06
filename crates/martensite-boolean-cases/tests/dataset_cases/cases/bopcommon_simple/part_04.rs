//! The `bopcommon_simple` case bodies, part 4; each statement is one line of its script.
use super::*;

pub fn V2(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [1.6, 2. * r, 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 135.);
    operands_checked(s, Operation::Common, Expected::Mass(16.0004));
}

pub fn V3(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [-0.5, -r, 0.], [1.4, 2. * r, 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 60.);
    operands_checked(s, Operation::Common, Expected::Mass(16.1674));
}

pub fn V4(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [1.6, 2. * r, 2.]);
    operands_checked(s, Operation::Common, Expected::Mass(16.0004));
}

pub fn V5(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [2., 2., 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(12.5664));
}

pub fn V6(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [2., 2., 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Common, Expected::Mass(12.5664));
}

pub fn V7(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [2., 1.5, 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(10.9757));
}

pub fn V8(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [-1., -1., 0.], [1. + r, 2., 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 60.);
    operands_checked(s, Operation::Common, Expected::Mass(12.338));
}

pub fn V9(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [1.5, 2., 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(10.9757));
}

pub fn W1(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [2., 2.5, 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(12.5664));
}

pub fn W2(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [2.5, 2., 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(12.5664));
}

pub fn W3(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [1., 2., 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(8.28319));
}

pub fn W4(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [0., -1., 0.], [1., 2., 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(8.28319));
}

pub fn W5(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [2., 1., 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(8.28319));
}

pub fn W6(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-1., -1., 0.], [1. + r, 1. + r, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Common, Expected::Mass(11.1116));
}

pub fn W7(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-1., -1., 0.], [1. + r, 1. + r, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Common, Expected::Mass(11.1116));
}

pub fn W8(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-1., -1., 0.], [1. + r, 1. + r, 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(11.1116));
}

pub fn W9(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [1. + r, 1. + r, 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(11.1116));
}

pub fn X1(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-0.8, -0.8, 0.], [1.8, 1.8, 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(11.7384));
}

pub fn X2(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -0.8, 0.], [1.8, 1.8, 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(11.7384));
}

pub fn X3(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [-1., -r, 0.], [1. + r, 1. + r, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 30.);
    operands_checked(s, Operation::Common, Expected::Mass(12.1096));
}

pub fn X4(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -0.8, 0.], [2., 1.6, 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(11.7384));
}

pub fn X5(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-0.8, -1., 0.], [1.6, 2., 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(11.7384));
}

pub fn X6(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -1., 0.], [2. * r, 2., 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 30.);
    operands_checked(s, Operation::Common, Expected::Mass(12.1096));
}

pub fn X7(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-0.5, -1., 0.], [1.5, 1.5, 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(9.33406));
}

pub fn X8(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-0.5, -1., 0.], [1.5, 1.5, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 30.);
    operands_checked(s, Operation::Common, Expected::Mass(9.33406));
}

pub fn X9(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [1.5, 1.5, 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(9.33406));
}

pub fn Y1(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [1. + r, 2. * r, 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(10.3842));
}

pub fn Y2(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [1. + r, 2. * r, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 135.);
    operands_checked(s, Operation::Common, Expected::Mass(10.3842));
}

pub fn Y3(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [-0.5, -r, 0.], [1.5, 2. * r, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 60.);
    operands_checked(s, Operation::Common, Expected::Mass(10.5189));
}

pub fn Y4(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [2. * r, r + 1., 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(10.3842));
}

pub fn Y5(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -1., 0.], [2. * r, 1. + r, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 30.);
    operands_checked(s, Operation::Common, Expected::Mass(11.8813));
}

pub fn Y6(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-0.8, -1., 0.], [1.6, 1.8, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 90.);
    operands_checked(s, Operation::Common, Expected::Mass(11.3244));
}

pub fn Y7(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-0.8, -1., 0.], [1.6, 1.8, 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(11.3244));
}

pub fn Y8(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1.1, 0.], [1., 2.2, 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(8.28319));
}

pub fn Y9(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [0., -1.1, 0.], [1., 2.2, 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(8.28319));
}

pub fn Z1(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1.1, -1., 0.], [2.2, 1., 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(8.28319));
}

pub fn Z2(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., 1., 0.], [2., 2., 1.]);
    operands_checked(s, Operation::Common, Expected::Empty);
}

pub fn Z3(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [1., -1., 0.], [2., 2., 1.]);
    operands_checked(s, Operation::Common, Expected::Empty);
}

pub fn Z4(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [2. * r, 2. * r, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 135.);
    operands_checked(s, Operation::Common, Expected::Mass(9.65685));
}

pub fn Z5(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [2. * r, 2. * r, 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(9.65685));
}

pub fn Z6(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [1.6, 2. * r, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 135.);
    operands_checked(s, Operation::Common, Expected::Mass(10.2206));
}

pub fn Z7(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [-0.5, -r, 0.], [1.4, 2. * r, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 60.);
    operands_checked(s, Operation::Common, Expected::Mass(10.3712));
}

pub fn Z8(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [1.6, 2. * r, 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(10.2206));
}

pub fn Z9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcylinder(s, "b2", 2., 6.);
    ttranslate(s, "b2", [0., 0., -2.]);
    operands_checked(s, Operation::Common, Expected::Mass(34.8496));
}

pub fn ZA1(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [1.6, 1.6, 2.]);
    operands_checked(s, Operation::Common, Expected::Mass(16.6872));
}

pub fn ZA2(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [-0.5, -r, 0.], [1.45, 1.8, 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 60.);
    operands_checked(s, Operation::Common, Expected::Mass(16.4622));
}

pub fn ZA3(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [1.6, 1.6, 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 135.);
    operands_checked(s, Operation::Common, Expected::Mass(16.6872));
}

pub fn ZA4(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [2. * r, 2. * r, 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 60.);
    operands_checked(s, Operation::Common, Expected::Mass(17.7473));
}

pub fn ZA5(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-0.8, -0.8, 0.], [1.6, 1.6, 2.]);
    operands_checked(s, Operation::Common, Expected::Mass(16.8455));
}

pub fn ZA6(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [0., -0.5, 0.], [2., 1., 2.]);
    operands_checked(s, Operation::Common, Expected::Mass(9.47172));
}

pub fn ZA7(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [0., -0.5, 0.], [2., 1., 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 30.);
    operands_checked(s, Operation::Common, Expected::Mass(9.47172));
}

pub fn ZA8(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [0., 0., 0.], [2., 2., 2.]);
    operands_checked(s, Operation::Common, Expected::Mass(8.71239));
}

pub fn ZA9(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-2., -2., 0.], [2., 2., 2.]);
    operands_checked(s, Operation::Common, Expected::Mass(8.71239));
}

pub fn ZB1(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [0., 0., 0.], [r, r, 2.]);
    operands_checked(s, Operation::Common, Expected::Mass(7.90092));
}

pub fn ZB2(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [0., 0., 0.], [r, r, 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -30.);
    operands_checked(s, Operation::Common, Expected::Mass(7.90092));
}

pub fn ZB3(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [1.6, 1.6, 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(10.7843));
}

pub fn ZB4(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [-0.5, -r, 0.], [1.45, 1.8, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 60.);
    operands_checked(s, Operation::Common, Expected::Mass(10.6153));
}

pub fn ZB5(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [1.6, 1.6, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 135.);
    operands_checked(s, Operation::Common, Expected::Mass(10.7843));
}

pub fn ZB6(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [2. * r, 2. * r, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 60.);
    operands_checked(s, Operation::Common, Expected::Mass(11.6529));
}

pub fn ZB7(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-0.8, -0.8, 0.], [1.6, 1.6, 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(10.9104));
}

pub fn ZB8(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [0., -0.5, 0.], [2., 1., 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(5.69247));
}

pub fn ZB9(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [0., -0.5, 0.], [2., 1., 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 30.);
    operands_checked(s, Operation::Common, Expected::Mass(5.69247));
}

pub fn ZC1(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [0., 0., 0.], [2., 2., 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(5.14159));
}

pub fn ZC2(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-2., -2., 0.], [2., 2., 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(5.14159));
}

pub fn ZC3(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [0., 0., 0.], [r, r, 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(4.64527));
}
