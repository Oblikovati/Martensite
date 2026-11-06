//! The `bopcut_simple` case bodies, part 4; each statement is one line of its script.
use super::*;

pub fn V2(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [1.6, 2. * r, 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 135.);
    operands_checked(s, Operation::Cut, Expected::Mass(23.4218));
}

pub fn V3(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [-0.5, -r, 0.], [1.4, 2. * r, 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 60.);
    operands_checked(s, Operation::Cut, Expected::Mass(21.0975));
}

pub fn V4(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [1.6, 2. * r, 2.]);
    operands_checked(s, Operation::Cut, Expected::Mass(23.4218));
}

pub fn V5(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [2., 2., 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(12.5664));
}

pub fn V6(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [2., 2., 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Cut, Expected::Mass(12.5664));
}

pub fn V7(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [2., 1.5, 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(16.3928));
}

pub fn V8(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [-1., -1., 0.], [1. + r, 2., 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 60.);
    operands_checked(s, Operation::Cut, Expected::Mass(14.6136));
}

pub fn V9(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [1.5, 2., 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(16.3928));
}

pub fn W1(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [2., 2.5, 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(12.5664));
}

pub fn W2(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [2.5, 2., 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(12.5664));
}

pub fn W3(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [1., 2., 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(17.708));
}

pub fn W4(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [0., -1., 0.], [1., 2., 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(17.708));
}

pub fn W5(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [2., 1., 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(17.708));
}

pub fn W6(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-1., -1., 0.], [1. + r, 1. + r, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Cut, Expected::Mass(18.5364));
}

pub fn W7(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-1., -1., 0.], [1. + r, 1. + r, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Cut, Expected::Mass(18.5364));
}

pub fn W8(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-1., -1., 0.], [1. + r, 1. + r, 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(18.5364));
}

pub fn W9(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [1. + r, 1. + r, 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(18.5364));
}

pub fn X1(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-0.8, -0.8, 0.], [1.8, 1.8, 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(17.5404));
}

pub fn X2(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -0.8, 0.], [1.8, 1.8, 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(17.5404));
}

pub fn X3(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [-1., -r, 0.], [1. + r, 1. + r, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 30.);
    operands_checked(s, Operation::Cut, Expected::Mass(16.6608));
}

pub fn X4(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -0.8, 0.], [2., 1.6, 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(17.5404));
}

pub fn X5(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-0.8, -1., 0.], [1.6, 2., 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(17.5404));
}

pub fn X6(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -1., 0.], [2. * r, 2., 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 30.);
    operands_checked(s, Operation::Cut, Expected::Mass(16.6608));
}

pub fn X7(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-0.5, -1., 0.], [1.5, 1.5, 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(18.9636));
}

pub fn X8(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-0.5, -1., 0.], [1.5, 1.5, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 30.);
    operands_checked(s, Operation::Cut, Expected::Mass(18.9636));
}

pub fn X9(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [1.5, 1.5, 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(18.9636));
}

pub fn Y1(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [1. + r, 2. * r, 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(21.5214));
}

pub fn Y2(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [1. + r, 2. * r, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 135.);
    operands_checked(s, Operation::Cut, Expected::Mass(21.5214));
}

pub fn Y3(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [-0.5, -r, 0.], [1.5, 2. * r, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 60.);
    operands_checked(s, Operation::Cut, Expected::Mass(20.4872));
}

pub fn Y4(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [2. * r, r + 1., 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(21.5214));
}

pub fn Y5(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -1., 0.], [2. * r, 1. + r, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 30.);
    operands_checked(s, Operation::Cut, Expected::Mass(18.708));
}

pub fn Y6(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-0.8, -1., 0.], [1.6, 1.8, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 90.);
    operands_checked(s, Operation::Cut, Expected::Mass(20.0274));
}

pub fn Y7(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-0.8, -1., 0.], [1.6, 1.8, 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(20.0274));
}

pub fn Y8(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1.1, 0.], [1., 2.2, 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(17.708));
}

pub fn Y9(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [0., -1.1, 0.], [1., 2.2, 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(17.708));
}

pub fn Z1(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1.1, -1., 0.], [2.2, 1., 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(17.708));
}

pub fn Z2(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., 1., 0.], [2., 2., 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(18.8496));
}

pub fn Z3(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [1., -1., 0.], [2., 2., 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(18.8496));
}

pub fn Z4(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [2. * r, 2. * r, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 135.);
    operands_checked(s, Operation::Cut, Expected::Mass(24.5064));
}

pub fn Z5(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [2. * r, 2. * r, 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(24.5064));
}

pub fn Z6(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [1.6, 2. * r, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 135.);
    operands_checked(s, Operation::Cut, Expected::Mass(23.3561));
}

pub fn Z7(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [-0.5, -r, 0.], [1.4, 2. * r, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 60.);
    operands_checked(s, Operation::Cut, Expected::Mass(22.261));
}

pub fn Z8(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [1.6, 2. * r, 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(23.3561));
}

pub fn Z9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcylinder(s, "b2", 2., 6.);
    ttranslate(s, "b2", [0., 0., -2.]);
    operands_checked(s, Operation::Cut, Expected::Mass(86.2832));
}

pub fn ZA1(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [1.6, 1.6, 2.]);
    operands_checked(s, Operation::Cut, Expected::Mass(20.6804));
}

pub fn ZA2(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [-0.5, -r, 0.], [1.45, 1.8, 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 60.);
    operands_checked(s, Operation::Cut, Expected::Mass(18.6723));
}

pub fn ZA3(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [1.6, 1.6, 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 135.);
    operands_checked(s, Operation::Cut, Expected::Mass(20.6804));
}

pub fn ZA4(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [2. * r, 2. * r, 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 60.);
    operands_checked(s, Operation::Cut, Expected::Mass(17.1023));
}

pub fn ZA5(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-0.8, -0.8, 0.], [1.6, 1.6, 2.]);
    operands_checked(s, Operation::Cut, Expected::Mass(21.204));
}

pub fn ZA6(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [0., -0.5, 0.], [2., 1., 2.]);
    operands_checked(s, Operation::Cut, Expected::Mass(20.306));
}

pub fn ZA7(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [0., -0.5, 0.], [2., 1., 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 30.);
    operands_checked(s, Operation::Cut, Expected::Mass(20.306));
}

pub fn ZA8(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [0., 0., 0.], [2., 2., 2.]);
    operands_checked(s, Operation::Cut, Expected::Mass(18.1372));
}

pub fn ZA9(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-2., -2., 0.], [2., 2., 2.]);
    operands_checked(s, Operation::Cut, Expected::Mass(18.1372));
}

pub fn ZB1(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [0., 0., 0.], [r, r, 2.]);
    operands_checked(s, Operation::Cut, Expected::Mass(21.8768));
}

pub fn ZB2(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [0., 0., 0.], [r, r, 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -30.);
    operands_checked(s, Operation::Cut, Expected::Mass(21.8768));
}

pub fn ZB3(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [1.6, 1.6, 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(22.2057));
}

pub fn ZB4(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [-0.5, -r, 0.], [1.45, 1.8, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 60.);
    operands_checked(s, Operation::Cut, Expected::Mass(21.1451));
}

pub fn ZB5(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [1.6, 1.6, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 135.);
    operands_checked(s, Operation::Cut, Expected::Mass(22.2057));
}

pub fn ZB6(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [2. * r, 2. * r, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 60.);
    operands_checked(s, Operation::Cut, Expected::Mass(20.7552));
}

pub fn ZB7(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-0.8, -0.8, 0.], [1.6, 1.6, 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(22.5144));
}

pub fn ZB8(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [0., -0.5, 0.], [2., 1., 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(20.5344));
}

pub fn ZB9(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [0., -0.5, 0.], [2., 1., 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 30.);
    operands_checked(s, Operation::Cut, Expected::Mass(20.5344));
}

pub fn ZC1(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [0., 0., 0.], [2., 2., 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(19.2788));
}

pub fn ZC2(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-2., -2., 0.], [2., 2., 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(19.2788));
}

pub fn ZC3(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [0., 0., 0.], [r, r, 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(21.058));
}
