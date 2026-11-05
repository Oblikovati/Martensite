//! The `bopcut_simple` case bodies, part 6; each statement is one line of its script.
use super::*;

pub fn ZI8(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    psphere(s, "b2", 6.);
    ttranslate(s, "b2", [0., 0., 8.]);
    operands_checked(s, Operation::Cut, Expected::Mass(196.529));
}

pub fn ZI9(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    psphere(s, "b2", 6.);
    ttranslate(s, "b2", [0., 0., 8.]);
    trotate(s, "b2", [0., 0., 8.], [0., 0., 1.], 90.);
    operands_checked(s, Operation::Cut, Expected::Mass(196.529));
}

pub fn ZJ1(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    psphere(s, "b2", 6.);
    ttranslate(s, "b2", [0., 0., 8.]);
    trotate(s, "b2", [0., 0., 8.], [0., 0., 1.], 180.);
    operands_checked(s, Operation::Cut, Expected::Mass(196.529));
}

pub fn ZJ2(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    psphere(s, "b2", 6.);
    ttranslate(s, "b2", [0., 0., 8.]);
    trotate(s, "b2", [0., 0., 8.], [0., 0., 1.], 270.);
    operands_checked(s, Operation::Cut, Expected::Mass(196.529));
}

pub fn ZJ3(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    psphere(s, "b2", 6.);
    ttranslate(s, "b2", [0., 0., 8.]);
    trotate(s, "b2", [0., 0., 8.], [0., 1., 0.], 90.);
    operands_checked(s, Operation::Cut, Expected::Mass(196.529));
}

pub fn ZJ4(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    pcone(s, "b2", 2., 1., 4.);
    ttranslate(s, "b2", [0., 0., 8.]);
    operands_checked(s, Operation::Cut, Expected::Mass(301.593));
}

pub fn ZJ5(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    pcone(s, "b2", 1., 2., 4.);
    ttranslate(s, "b2", [0., 0., 4.]);
    operands_checked(s, Operation::Cut, Expected::Mass(331.027));
}

pub fn ZJ6(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    pcone(s, "b2", 2., 1., 8.);
    operands_checked(s, Operation::Cut, Expected::Mass(361.87));
}

pub fn ZJ7(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    pcone(s, "b2", 2., 1., 9.);
    ttranslate(s, "b2", [0., 0., -1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(360.295));
}

pub fn ZJ8(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    pcone(s, "b2", 2., 1., 4.);
    ttranslate(s, "b2", [0., 0., 2.]);
    operands_checked(s, Operation::Cut, Expected::Mass(356.16));
}

pub fn ZJ9(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    pcone(s, "b2", 2., 1., 10.);
    ttranslate(s, "b2", [0., 0., -1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(362.225));
}

pub fn ZK1(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    pcone(s, "b2", 4., 2., 4.);
    ttranslate(s, "b2", [0., 0., 8.]);
    operands_checked(s, Operation::Cut, Expected::Mass(301.593));
}

pub fn ZK2(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    pcone(s, "b2", 4., 2., 4.);
    ttranslate(s, "b2", [0., 0., 8.]);
    trotate(s, "b2", [0., 0., 8.], [0., 0., 1.], 90.);
    operands_checked(s, Operation::Cut, Expected::Mass(301.593));
}

pub fn ZK3(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    pcone(s, "b2", 4., 2., 4.);
    ttranslate(s, "b2", [0., 0., 8.]);
    trotate(s, "b2", [0., 0., 8.], [0., 0., 1.], 180.);
    operands_checked(s, Operation::Cut, Expected::Mass(301.593));
}

pub fn ZK4(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    pcone(s, "b2", 4., 2., 4.);
    ttranslate(s, "b2", [0., 0., 8.]);
    trotate(s, "b2", [0., 0., 8.], [0., 0., 1.], 270.);
    operands_checked(s, Operation::Cut, Expected::Mass(301.593));
}

pub fn ZK5(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    pcone(s, "b2", 2., 1., 10.);
    ttranslate(s, "b2", [0., 0., -1.]);
    trotate(s, "b2", [0., 0., 4.], [0., 1., 0.], 90.);
    operands_checked(s, Operation::Cut, Expected::Mass(358.727));
}

pub fn ZK6(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    pcone(s, "b2", 2., 1., 10.);
    ttranslate(s, "b2", [0., 0., -1.]);
    trotate(s, "b2", [0., 0., 4.], [1., 0., 0.], 90.);
    operands_checked(s, Operation::Cut, Expected::Mass(358.826));
}

pub fn ZK7(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    pcone(s, "b2", 6., 1., 10.);
    ttranslate(s, "b2", [0., 0., -1.]);
    trotate(s, "b2", [0., 0., 4.], [0., 1., 0.], 90.);
    operands_checked(s, Operation::Cut, Expected::Mass(308.057));
}

pub fn ZK8(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    pcone(s, "b2", 6., 1., 10.);
    ttranslate(s, "b2", [0., 0., -1.]);
    trotate(s, "b2", [0., 0., 4.], [1., 0., 0.], 90.);
    operands_checked(s, Operation::Cut, Expected::Mass(308.068));
}

pub fn ZK9(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    pcone(s, "b2", 1., 6., 8.);
    ttranslate(s, "b2", [0., 0., -1.]);
    trotate(s, "b2", [0., 0., 4.], [0., 1., 0.], 90.);
    operands_checked(s, Operation::Cut, Expected::Mass(355.69));
}

pub fn ZL1(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    pcone(s, "b2", 1., 6., 8.);
    ttranslate(s, "b2", [0., 0., -1.]);
    trotate(s, "b2", [0., 0., 4.], [0., 1., 0.], -90.);
    operands_checked(s, Operation::Cut, Expected::Mass(355.82));
}

pub fn ZL2(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    ptorus(s, "b2", 4., 1.);
    ttranslate(s, "b2", [0., 0., 4.]);
    operands_checked(s, Operation::Cut, Expected::Mass(317.718));
}

pub fn ZL3(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    ptorus(s, "b2", 4., 1.);
    ttranslate(s, "b2", [0., 0., 4.]);
    trotate(s, "b2", [0., 0., 4.], [0., 0., 1.], 90.);
    operands_checked(s, Operation::Cut, Expected::Mass(317.718));
}

pub fn ZL4(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    ptorus(s, "b2", 4., 1.);
    ttranslate(s, "b2", [0., 0., 4.]);
    trotate(s, "b2", [0., 0., 4.], [0., 0., 1.], 180.);
    operands_checked(s, Operation::Cut, Expected::Mass(317.718));
}

pub fn ZL5(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    ptorus(s, "b2", 4., 1.);
    ttranslate(s, "b2", [0., 0., 4.]);
    trotate(s, "b2", [0., 0., 4.], [0., 0., 1.], 270.);
    operands_checked(s, Operation::Cut, Expected::Mass(317.718));
}

pub fn ZL6(s: &mut dyn Session) {
    pcone(s, "b1", 8., 4., 8.);
    pcone(s, "b2", 4., 2., 4.);
    ttranslate(s, "b2", [0., 0., 8.]);
    operands_checked(s, Operation::Cut, Expected::Mass(588.519));
}

pub fn ZL7(s: &mut dyn Session) {
    pcone(s, "b1", 8., 4., 8.);
    pcone(s, "b2", 4., 2., 4.);
    ttranslate(s, "b2", [0., 0., 8.]);
    trotate(s, "b2", [0., 0., 2.], [0., 0., 1.], 90.);
    operands_checked(s, Operation::Cut, Expected::Mass(588.519));
}

pub fn ZL8(s: &mut dyn Session) {
    pcone(s, "b1", 8., 4., 8.);
    pcone(s, "b2", 4., 2., 4.);
    ttranslate(s, "b2", [0., 0., 8.]);
    trotate(s, "b2", [0., 0., 2.], [0., 0., 1.], 180.);
    operands_checked(s, Operation::Cut, Expected::Mass(588.519));
}

pub fn ZL9(s: &mut dyn Session) {
    pcone(s, "b1", 8., 4., 8.);
    pcone(s, "b2", 4., 2., 4.);
    ttranslate(s, "b2", [0., 0., 8.]);
    trotate(s, "b2", [0., 0., 2.], [0., 0., 1.], 270.);
    operands_checked(s, Operation::Cut, Expected::Mass(588.519));
}

pub fn ZM1(s: &mut dyn Session) {
    pcone(s, "b1", 8., 4., 8.);
    pcone(s, "b2", 2., 1., 2.);
    ttranslate(s, "b2", [0., 0., 8.]);
    operands_checked(s, Operation::Cut, Expected::Mass(588.519));
}

pub fn ZM2(s: &mut dyn Session) {
    pcone(s, "b1", 8., 4., 8.);
    pcone(s, "b2", 2., 1., 2.);
    ttranslate(s, "b2", [0., 0., 6.]);
    operands_checked(s, Operation::Cut, Expected::Mass(619.018));
}

pub fn ZM3(s: &mut dyn Session) {
    pcone(s, "b1", 8., 4., 8.);
    pcone(s, "b2", 2., 1., 8.);
    operands_checked(s, Operation::Cut, Expected::Mass(648.796));
}

pub fn ZM4(s: &mut dyn Session) {
    pcone(s, "b1", 8., 4., 8.);
    pcone(s, "b2", 2., 1., 9.);
    ttranslate(s, "b2", [0., 0., -1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(647.221));
}

pub fn ZM5(s: &mut dyn Session) {
    pcone(s, "b1", 8., 4., 8.);
    pcone(s, "b2", 2., 1., 4.);
    ttranslate(s, "b2", [0., 0., 2.]);
    operands_checked(s, Operation::Cut, Expected::Mass(643.086));
}

pub fn ZM6(s: &mut dyn Session) {
    pcone(s, "b1", 8., 4., 8.);
    pcone(s, "b2", 2., 1., 10.);
    ttranslate(s, "b2", [0., 0., -1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(649.15));
}

pub fn ZM7(s: &mut dyn Session) {
    pcone(s, "b1", 8., 4., 8.);
    pcone(s, "b2", 4., 8., 4.);
    ttranslate(s, "b2", [0., 0., -4.]);
    operands_checked(s, Operation::Cut, Expected::Mass(588.519));
}

pub fn ZM8(s: &mut dyn Session) {
    pcone(s, "b1", 8., 4., 8.);
    pcone(s, "b2", 4., 8., 4.);
    ttranslate(s, "b2", [0., 0., -4.]);
    trotate(s, "b2", [0., 0., -4.], [0., 0., 1.], 90.);
    operands_checked(s, Operation::Cut, Expected::Mass(588.519));
}

pub fn ZM9(s: &mut dyn Session) {
    pcone(s, "b1", 8., 4., 8.);
    pcone(s, "b2", 4., 8., 4.);
    ttranslate(s, "b2", [0., 0., -4.]);
    trotate(s, "b2", [0., 0., -4.], [0., 0., 1.], 180.);
    operands_checked(s, Operation::Cut, Expected::Mass(588.519));
}

pub fn ZN1(s: &mut dyn Session) {
    pcone(s, "b1", 8., 4., 8.);
    pcone(s, "b2", 4., 8., 4.);
    ttranslate(s, "b2", [0., 0., -4.]);
    trotate(s, "b2", [0., 0., -4.], [0., 0., 1.], 270.);
    operands_checked(s, Operation::Cut, Expected::Mass(588.519));
}

pub fn ZN2(s: &mut dyn Session) {
    pcone(s, "b1", 8., 4., 8.);
    pcone(s, "b2", 8., 4., 4.);
    ttranslate(s, "b2", [4., 0., 0.]);
    operands_checked(s, Operation::Cut, Expected::Mass(523.976));
}

pub fn ZN3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcylinder(s, "b2", 1., 6.);
    ttranslate(s, "b2", [2., 2., -2.]);
    operands_checked(s, Operation::Cut, Expected::Mass(114.85));
}

pub fn ZN4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcylinder(s, "b2", 1., 2.);
    ttranslate(s, "b2", [2., 2., 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(114.85));
}

pub fn ZN5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcylinder(s, "b2", 2., 4.);
    ttranslate(s, "b2", [4., 2., 0.]);
    operands_checked(s, Operation::Cut, Expected::Mass(92.5664));
}

pub fn ZN6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcylinder(s, "b2", 2., 4.);
    ttranslate(s, "b2", [2., 0., 0.]);
    operands_checked(s, Operation::Cut, Expected::Mass(92.5664));
}

pub fn ZN7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcylinder(s, "b2", 2., 4.);
    ttranslate(s, "b2", [0., 2., 0.]);
    operands_checked(s, Operation::Cut, Expected::Mass(92.5664));
}

pub fn ZN8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcylinder(s, "b2", 1., 4.);
    ttranslate(s, "b2", [4., 2., 0.]);
    operands_checked(s, Operation::Cut, Expected::Mass(97.4248));
}

pub fn ZN9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcylinder(s, "b2", 1., 4.);
    ttranslate(s, "b2", [2., 0., 0.]);
    operands_checked(s, Operation::Cut, Expected::Mass(97.4248));
}

pub fn ZO1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcylinder(s, "b2", 1., 4.);
    ttranslate(s, "b2", [0., 2., 0.]);
    operands_checked(s, Operation::Cut, Expected::Mass(97.4248));
}

pub fn ZO2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcylinder(s, "b2", 1., 4.);
    ttranslate(s, "b2", [4., 3., 0.]);
    operands_checked(s, Operation::Cut, Expected::Mass(97.4248));
}

pub fn ZO3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcylinder(s, "b2", 1., 4.);
    ttranslate(s, "b2", [3., 0., 0.]);
    operands_checked(s, Operation::Cut, Expected::Mass(97.4248));
}

pub fn ZO4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcylinder(s, "b2", 1., 4.);
    ttranslate(s, "b2", [0., 3., 0.]);
    operands_checked(s, Operation::Cut, Expected::Mass(97.4248));
}

pub fn ZO5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcylinder(s, "b2", 1., 4.);
    ttranslate(s, "b2", [1., 0., 0.]);
    operands_checked(s, Operation::Cut, Expected::Mass(97.4248));
}

pub fn ZO6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcylinder(s, "b2", 4., 4.);
    ttranslate(s, "b2", [-2., 2., 0.]);
    operands_checked(s, Operation::Cut, Expected::Mass(70.4308));
}

pub fn ZO7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcylinder(s, "b2", 4., 4.);
    ttranslate(s, "b2", [-2., 2., 0.]);
    trotate(s, "b2", [-2., 2., 0.], [0., 0., 1.], 30.);
    operands_checked(s, Operation::Cut, Expected::Mass(70.4308));
}

pub fn ZO8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcylinder(s, "b2", 4., 4.);
    ttranslate(s, "b2", [-2., 2., 0.]);
    trotate(s, "b2", [-2., 2., 0.], [0., 0., 1.], 90.);
    operands_checked(s, Operation::Cut, Expected::Mass(70.4308));
}

pub fn ZO9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcylinder(s, "b2", 1., 4.);
    ttranslate(s, "b2", [3., 1., 4.]);
    operands_checked(s, Operation::Cut, Expected::Mass(96.));
}

pub fn ZP1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcylinder(s, "b2", 1., 4.);
    ttranslate(s, "b2", [1., 1., 4.]);
    operands_checked(s, Operation::Cut, Expected::Mass(96.));
}

pub fn ZP2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcylinder(s, "b2", 2., 6.);
    ttranslate(s, "b2", [0., 0., -2.]);
    operands_checked(s, Operation::Cut, Expected::Mass(86.2832));
}
