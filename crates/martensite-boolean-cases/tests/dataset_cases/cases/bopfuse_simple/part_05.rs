//! The `bopfuse_simple` case bodies, part 5; each statement is one line of its script.
use super::*;

pub fn ZC4(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [0., 0., 0.], [r, r, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -30.);
    operands_checked(s, Operation::Fuse, Expected::Mass(19.1684));
}

pub fn ZC5(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [0., 0., -0.2], [1.4, 1.4, 1.4]);
    trotate(s, "b2", [0., 0., -0.2], [-1., 1., 0.], -45.);
    operands_checked(s, Operation::Fuse, Expected::Mass(19.7221));
}

pub fn ZC6(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -2., 0.5], [2., 2., 1.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(26.5664));
}

pub fn ZC7(s: &mut dyn Session) {
    pcylinder(s, "b2", 1., 4.);
    make_box(s, "b1", [-1., 0., 1.], [2., 2., 2.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(41.9911));
}

pub fn ZC8(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    pcylinder(s, "b2", 0.5, 1.);
    ttranslate(s, "b2", [0., 0., 2.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(21.9911));
}

pub fn ZC9(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    pcylinder(s, "b2", 0.5, 1.);
    ttranslate(s, "b2", [0., 0., 1.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(18.8496));
}

pub fn ZD1(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    pcylinder(s, "b2", 0.5, 1.);
    ttranslate(s, "b2", [0., 0., 0.5]);
    operands_checked(s, Operation::Fuse, Expected::Mass(18.8496));
}

pub fn ZD2(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    pcylinder(s, "b2", 0.5, 2.);
    operands_checked(s, Operation::Fuse, Expected::Mass(18.8496));
}

pub fn ZD3(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    pcylinder(s, "b2", 0.5, 3.);
    ttranslate(s, "b2", [0., 0., -1.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(21.9911));
}

pub fn ZD4(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    pcylinder(s, "b2", 1., 2.);
    ttranslate(s, "b2", [0., 0., 2.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(31.4159));
}

pub fn ZD5(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    pcylinder(s, "b2", 1., 2.);
    ttranslate(s, "b2", [0., 0., 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 90.);
    operands_checked(s, Operation::Fuse, Expected::Mass(31.4159));
}

pub fn ZD6(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    pcylinder(s, "b2", 1., 2.);
    ttranslate(s, "b2", [0., 0., 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 180.);
    operands_checked(s, Operation::Fuse, Expected::Mass(31.4159));
}

pub fn ZD7(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    pcylinder(s, "b2", 1., 2.);
    ttranslate(s, "b2", [0., 0., 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 270.);
    operands_checked(s, Operation::Fuse, Expected::Mass(31.4159));
}

pub fn ZD8(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 4.);
    pcylinder(s, "b2", 1., 4.);
    trotate(s, "b2", [0., 0., 2.], [1., 0., 0.], 90.);
    operands_checked(s, Operation::Fuse, Expected::Mass(46.8319));
}

pub fn ZD9(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 4.);
    pcylinder(s, "b2", 1., 4.);
    trotate(s, "b2", [0., 0., 2.], [1., 0., 0.], 90.);
    trotate(s, "b2", [0., 0., 2.], [0., 1., 0.], 60.);
    operands_checked(s, Operation::Fuse, Expected::Mass(46.8319));
}

pub fn ZE1(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 4.);
    pcylinder(s, "b2", 1., 4.);
    trotate(s, "b2", [0., 0., 2.], [1., 0., 0.], 90.);
    trotate(s, "b2", [0., 0., 2.], [0., 1., 0.], 180.);
    operands_checked(s, Operation::Fuse, Expected::Mass(46.8319));
}

pub fn ZE2(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 4.);
    pcylinder(s, "b2", 1., 4.);
    trotate(s, "b2", [0., 0., 2.], [1., 0., 0.], 90.);
    trotate(s, "b2", [0., 0., 2.], [0., 1., 0.], 60.);
    trotate(s, "b2", [0., 0., 2.], [0., 0., 1.], 120.);
    operands_checked(s, Operation::Fuse, Expected::Mass(46.8319));
}

pub fn ZE3(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 4.);
    pcylinder(s, "b2", 0.5, 4.);
    trotate(s, "b2", [0., 0., 2.], [1., 0., 0.], 90.);
    operands_checked(s, Operation::Fuse, Expected::Mass(38.0586));
}

pub fn ZE4(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 4.);
    pcylinder(s, "b2", 0.5, 4.);
    trotate(s, "b2", [0., 0., 2.], [1., 0., 0.], 90.);
    trotate(s, "b2", [0., 0., 2.], [0., 1., 0.], 90.);
    operands_checked(s, Operation::Fuse, Expected::Mass(38.0584));
}

pub fn ZE5(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 4.);
    pcylinder(s, "b2", 0.5, 4.);
    trotate(s, "b2", [0., 0., 2.], [1., 0., 0.], 90.);
    trotate(s, "b2", [0., 0., 2.], [0., 1., 0.], 180.);
    operands_checked(s, Operation::Fuse, Expected::Mass(38.058));
}

pub fn ZE6(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 4.);
    pcylinder(s, "b2", 0.5, 4.);
    trotate(s, "b2", [0., 0., 2.], [1., 0., 0.], 90.);
    trotate(s, "b2", [0., 0., 2.], [0., 1., 0.], 270.);
    operands_checked(s, Operation::Fuse, Expected::Mass(38.0572));
}

pub fn ZE7(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 4.);
    pcylinder(s, "b2", 0.5, 4.);
    trotate(s, "b2", [0., 0., 2.], [1., 0., 0.], 90.);
    ttranslate(s, "b2", [0.5, 0., 0.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(38.8306));
}

pub fn ZE8(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 4.);
    pcylinder(s, "b2", 0.5, 4.);
    trotate(s, "b2", [0., 0., 2.], [1., 0., 0.], 90.);
    trotate(s, "b2", [0., 0., 2.], [0., 1., 0.], 90.);
    ttranslate(s, "b2", [0.5, 0., 0.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(38.8304));
}

pub fn ZE9(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 4.);
    pcylinder(s, "b2", 0.5, 4.);
    trotate(s, "b2", [0., 0., 2.], [1., 0., 0.], 90.);
    trotate(s, "b2", [0., 0., 2.], [0., 1., 0.], 180.);
    ttranslate(s, "b2", [0.5, 0., 0.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(38.8309));
}

pub fn ZF1(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 4.);
    pcylinder(s, "b2", 0.5, 4.);
    trotate(s, "b2", [0., 0., 2.], [1., 0., 0.], 90.);
    trotate(s, "b2", [0., 0., 2.], [0., 1., 0.], 270.);
    ttranslate(s, "b2", [0.5, 0., 0.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(38.8307));
}

pub fn ZF2(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 4.);
    pcylinder(s, "b2", 1., 4.);
    ttranslate(s, "b2", [1., 0., 0.]);
    trotate(s, "b2", [1., 0., 0.], [0., 0., 1.], -120.);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 60.);
    operands_checked(s, Operation::Fuse, Expected::Mass(43.62));
}

pub fn ZF3(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 4.);
    pcylinder(s, "b2", 1., 4.);
    ttranslate(s, "b2", [1., 0., 0.]);
    trotate(s, "b2", [1., 0., 0.], [0., 0., 1.], 120.);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 60.);
    operands_checked(s, Operation::Fuse, Expected::Mass(43.62));
}

pub fn ZF4(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 4.);
    pcylinder(s, "b2", 1., 4.);
    ttranslate(s, "b2", [1., 0., 0.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(43.62));
}

pub fn ZF5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcone(s, "b2", 1., 0.5, 2.);
    ttranslate(s, "b2", [2., 2., 4.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(103.359));
}

pub fn ZF6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcone(s, "b2", 1., 0.5, 2.);
    ttranslate(s, "b2", [2., 2., 2.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(96.));
}

pub fn ZF7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcone(s, "b2", 1., 0.5, 4.);
    ttranslate(s, "b2", [2., 2., 0.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(96.));
}

pub fn ZF8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcone(s, "b2", 1., 0.5, 5.);
    ttranslate(s, "b2", [2., 2., -1.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(102.596));
}

pub fn ZF9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcone(s, "b2", 1., 0.5, 2.);
    ttranslate(s, "b2", [2., 2., 1.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(96.));
}

pub fn ZG1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcone(s, "b2", 3., 2., 4.);
    ttranslate(s, "b2", [4., 2., 0.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(137.238));
}

pub fn ZG2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcone(s, "b2", 3., 2., 4.);
    ttranslate(s, "b2", [2., 0., 0.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(137.238));
}

pub fn ZG3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcone(s, "b2", 3., 2., 4.);
    ttranslate(s, "b2", [0., 2., 0.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(137.238));
}

pub fn ZG4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcone(s, "b2", 3., 2., 4.);
    ttranslate(s, "b2", [2., 0., 0.]);
    trotate(s, "b2", [2., 0., 0.], [0., 0., 1.], 30.);
    operands_checked(s, Operation::Fuse, Expected::Mass(137.238));
}

pub fn ZG5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcone(s, "b2", 2., 1., 4.);
    ttranslate(s, "b2", [4., 2., 0.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(111.284));
}

pub fn ZG6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcone(s, "b2", 2., 1., 4.);
    ttranslate(s, "b2", [2., 0., 0.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(111.284));
}

pub fn ZG7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcone(s, "b2", 2., 1., 4.);
    ttranslate(s, "b2", [0., 2., 0.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(111.284));
}

pub fn ZG8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcone(s, "b2", 1., 0.5, 2.);
    ttranslate(s, "b2", [3., 1., 4.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(103.359));
}

pub fn ZG9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcone(s, "b2", 1., 0.5, 2.);
    ttranslate(s, "b2", [1., 1., 4.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(103.359));
}

pub fn ZH1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcone(s, "b2", 5., 4., 4.);
    ttranslate(s, "b2", [6., 2., 0.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(273.296));
}

pub fn ZH2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcone(s, "b2", 5., 4., 4.);
    ttranslate(s, "b2", [-2., 2., 0.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(273.296));
}

pub fn ZH3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcone(s, "b2", 5., 4., 4.);
    ttranslate(s, "b2", [-2., 2., 0.]);
    trotate(s, "b2", [-2., 2., 0.], [0., 0., 1.], 30.);
    operands_checked(s, Operation::Fuse, Expected::Mass(273.296));
}

pub fn ZH4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [4., 4., 4.]);
    pcone(s, "b2", 5., 3.5, 4.);
    ttranslate(s, "b2", [-2., 2., 0.]);
    trotate(s, "b2", [-2., 2., 0.], [0., 0., 1.], 30.);
    operands_checked(s, Operation::Fuse, Expected::Mass(262.691));
}

pub fn ZH5(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    psphere(s, "b2", 4.);
    ttranslate(s, "b2", [0., 0., 8.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(351.858));
}

pub fn ZH6(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    psphere(s, "b2", 4.);
    ttranslate(s, "b2", [0., 0., 8.]);
    trotate(s, "b2", [0., 0., 8.], [0., 0., 1.], 90.);
    operands_checked(s, Operation::Fuse, Expected::Mass(351.858));
}

pub fn ZH7(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    psphere(s, "b2", 4.);
    ttranslate(s, "b2", [0., 0., 8.]);
    trotate(s, "b2", [0., 0., 8.], [0., 0., 1.], 180.);
    operands_checked(s, Operation::Fuse, Expected::Mass(351.858));
}

pub fn ZH8(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    psphere(s, "b2", 4.);
    ttranslate(s, "b2", [0., 0., 8.]);
    trotate(s, "b2", [0., 0., 8.], [0., 0., 1.], 270.);
    operands_checked(s, Operation::Fuse, Expected::Mass(351.858));
}

pub fn ZH9(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    psphere(s, "b2", 4.);
    ttranslate(s, "b2", [0., 0., 8.]);
    trotate(s, "b2", [0., 0., 8.], [1., 0., 0.], 90.);
    operands_checked(s, Operation::Fuse, Expected::Mass(351.858));
}

pub fn ZI1(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    psphere(s, "b2", 4.);
    ttranslate(s, "b2", [0., 0., 8.]);
    trotate(s, "b2", [0., 0., 8.], [1., 0., 0.], 90.);
    trotate(s, "b2", [0., 0., 8.], [0., 0., 1.], 90.);
    operands_checked(s, Operation::Fuse, Expected::Mass(351.858));
}

pub fn ZI2(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    psphere(s, "b2", 4.);
    ttranslate(s, "b2", [0., 0., 8.]);
    trotate(s, "b2", [0., 0., 8.], [1., 0., 0.], 90.);
    trotate(s, "b2", [0., 0., 8.], [0., 0., 1.], 180.);
    operands_checked(s, Operation::Fuse, Expected::Mass(351.858));
}

pub fn ZI3(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    psphere(s, "b2", 4.);
    ttranslate(s, "b2", [0., 0., 8.]);
    trotate(s, "b2", [0., 0., 8.], [1., 0., 0.], 90.);
    trotate(s, "b2", [0., 0., 8.], [0., 0., 1.], 270.);
    operands_checked(s, Operation::Fuse, Expected::Mass(351.858));
}

pub fn ZI4(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    psphere(s, "b2", 2.);
    ttranslate(s, "b2", [0., 0., 8.]);
    trotate(s, "b2", [0., 0., 8.], [1., 0., 0.], 90.);
    operands_checked(s, Operation::Fuse, Expected::Mass(314.159));
}

pub fn ZI5(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    psphere(s, "b2", 2.);
    ttranslate(s, "b2", [0., 0., 8.]);
    trotate(s, "b2", [0., 0., 8.], [1., 0., 0.], 90.);
    trotate(s, "b2", [0., 0., 8.], [0., 1., 0.], 90.);
    operands_checked(s, Operation::Fuse, Expected::Mass(314.159));
}

pub fn ZI6(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    psphere(s, "b2", 2.);
    ttranslate(s, "b2", [0., 0., 8.]);
    trotate(s, "b2", [0., 0., 8.], [1., 0., 0.], 90.);
    trotate(s, "b2", [0., 0., 8.], [0., 1., 0.], 180.);
    operands_checked(s, Operation::Fuse, Expected::Mass(314.159));
}

pub fn ZI7(s: &mut dyn Session) {
    pcylinder(s, "b1", 4., 8.);
    psphere(s, "b2", 2.);
    ttranslate(s, "b2", [0., 0., 8.]);
    trotate(s, "b2", [0., 0., 8.], [1., 0., 0.], 90.);
    trotate(s, "b2", [0., 0., 8.], [0., 1., 0.], 270.);
    operands_checked(s, Operation::Fuse, Expected::Mass(314.159));
}
