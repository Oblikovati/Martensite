//! The `bopfuse_simple` case bodies, part 3; each statement is one line of its script.
use super::*;

pub fn N9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.5, 0.], [1.5, 1.5, -1.]);
    trotate(s, "b2", [0., 0.5, 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Fuse, Expected::Mass(15.));
}

pub fn O1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.5, 0.], [0.8, 0.8, -1.]);
    trotate(s, "b2", [0., 0.5, 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Fuse, Expected::Mass(9.25177));
}

pub fn O2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.25, 0.], [0.8, 0.8, -1.]);
    trotate(s, "b2", [0., 0.25, 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Fuse, Expected::Mass(9.43383));
}

pub fn O3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.25, 0.], [0.5, 0.5, -1.]);
    trotate(s, "b2", [0., 0.25, 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Fuse, Expected::Mass(8.02145));
}

pub fn O4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 2_f64.sqrt() * 0.75;
    make_box(s, "b2", [0., 0.625, 0.], [r, r, -1.]);
    trotate(s, "b2", [0., 0.625, 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Fuse, Expected::Mass(11.0395));
}

pub fn O5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.25, 0., 0.], [0.5, 1.7, -1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Fuse, Expected::Mass(10.9358));
}

pub fn O6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.25, 0., 0.], [0.5, 1.7, -1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -30.);
    operands_checked(s, Operation::Fuse, Expected::Mass(11.0896));
}

pub fn O7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.25, -0.25, 0.], [0.25, 2., -1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -30.);
    operands_checked(s, Operation::Fuse, Expected::Mass(11.067));
}

pub fn O8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.25, 0., 0.], [0.5, 1.3, -1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Fuse, Expected::Mass(9.76188));
}

pub fn O9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., -0.25, 0.], [0.5, 1.55, -1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -35.);
    operands_checked(s, Operation::Fuse, Expected::Mass(10.5853));
}

pub fn P1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.25, 0.5], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0., 0.25, 0.], [0., 0., 1.], 230.);
    operands_checked(s, Operation::Fuse, Expected::Mass(7.95708));
}

pub fn P2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.1, 0.4, 0.5], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0.1, 0.4, 0.], [0., 0., 1.], 245.);
    operands_checked(s, Operation::Fuse, Expected::Mass(7.39351));
}

pub fn P3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.25, -0.25, 0.5], [0.5, 0.5, 1.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(7.875));
}

pub fn P4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, 0.5, 0.5], [0.5, 0.5, 1.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(7.));
}

pub fn P5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, 0.5, 0.5], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0.25, 0.25, 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Fuse, Expected::Mass(7.37868));
}

pub fn P6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, 0.5, 0.5], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0.25, 0.25, 0.], [0., 0., 1.], 90.);
    operands_checked(s, Operation::Fuse, Expected::Mass(8.));
}

pub fn P7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, 0.25, 0.5], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0.25, 0.25, 0.], [0., 0., 1.], 125.);
    operands_checked(s, Operation::Fuse, Expected::Mass(7.73041));
}

pub fn P8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, -0.25, 0.5], [0.5, 1.5, 1.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(9.));
}

pub fn P9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.9, -0.25, 0.5], [0.5, 1.7, 1.]);
    trotate(s, "b2", [0.9, -0.25, 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Fuse, Expected::Mass(9.31449));
}

pub fn Q1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [1.25, 0., 0.5], [0.25, 1.7, 1.]);
    trotate(s, "b2", [1.1, 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Fuse, Expected::Mass(9.31226));
}

pub fn Q2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.25, -0.25, 0.5], [0.5, 1.7, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -30.);
    operands_checked(s, Operation::Fuse, Expected::Mass(9.5406));
}

pub fn Q3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.25, -0.25, 0.5], [0.5, 1.55, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Fuse, Expected::Mass(8.93786));
}

pub fn Q4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.5, -0.25, 0.5], [1., 1., 1.]);
    trotate(s, "b2", [0.5, -0.25, 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Fuse, Expected::Mass(8.69365));
}

pub fn Q5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.15, 0.5], [0.8, 0.9, 1.]);
    trotate(s, "b2", [0., 0.25, 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Fuse, Expected::Mass(8.28688));
}

pub fn Q6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.5, -0.25, 0.5], [0.5, 1., 1.]);
    trotate(s, "b2", [0.5, -0.25, 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Fuse, Expected::Mass(7.90013));
}

pub fn Q7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.25, -0.25, 0.25], [0.25, 1., 0.5]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -30.);
    operands_checked(s, Operation::Fuse, Expected::Mass(6.57476));
}

pub fn Q8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., -0.25, 0.25], [0.5, 1.7, 0.5]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -35.);
    operands_checked(s, Operation::Fuse, Expected::Mass(7.1901));
}

pub fn Q9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [3., 3., 3.]);
    make_box(s, "b2", [0., 0., 0.], [1., 1., 1.]);
    trotate(s, "b2", [0., 0., 0.], [1., 0., 0.], -30.);
    ttranslate(s, "b2", [1., 1., 3.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(56.6906));
}

pub fn R1(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [2., 2., 2.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(24.));
}

pub fn R2(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [2., 2., 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Fuse, Expected::Mass(24.));
}

pub fn R3(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [2., 1.5, 2.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(21.9531));
}

pub fn R4(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [-1., -1., 0.], [1. + r, 2., 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 60.);
    operands_checked(s, Operation::Fuse, Expected::Mass(23.2038));
}

pub fn R5(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [1.5, 2., 2.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(21.9531));
}

pub fn R6(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [2., 2.5, 2.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(28.));
}

pub fn R7(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [2.5, 2., 2.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(28.));
}

pub fn R8(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [1., 2., 2.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(21.4248));
}

pub fn R9(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [0., -1., 0.], [1., 2., 2.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(21.4248));
}

pub fn S1(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [2., 1., 2.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(21.4248));
}

pub fn S2(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-1., -1., 0.], [1. + r, 1. + r, 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Fuse, Expected::Mass(21.2532));
}

pub fn S3(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-1., -1., 0.], [1. + r, 1. + r, 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Fuse, Expected::Mass(21.2532));
}

pub fn S4(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-1., -1., 0.], [1. + r, 1. + r, 2.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(21.2532));
}

pub fn S5(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [1. + r, 1. + r, 2.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(21.2532));
}

pub fn S6(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-0.8, -0.8, 0.], [1.8, 1.8, 2.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(21.882));
}

pub fn S7(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -0.8, 0.], [1.8, 1.8, 2.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(21.882));
}

pub fn S8(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [-1., -r, 0.], [1. + r, 1. + r, 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 30.);
    operands_checked(s, Operation::Fuse, Expected::Mass(22.4434));
}

pub fn S9(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -0.8, 0.], [2., 1.6, 2.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(21.802));
}

pub fn T1(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-0.8, -1., 0.], [1.6, 2., 2.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(21.802));
}

pub fn T2(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -1., 0.], [2. * r, 2., 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 30.);
    operands_checked(s, Operation::Fuse, Expected::Mass(22.4075));
}

pub fn T3(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-0.5, -1., 0.], [1.5, 1.5, 2.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(20.6654));
}

pub fn T4(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-0.5, -1., 0.], [1.5, 1.5, 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 30.);
    operands_checked(s, Operation::Fuse, Expected::Mass(20.6654));
}

pub fn T5(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [1.5, 1.5, 2.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(20.6654));
}

pub fn T6(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [1. + r, 2. * r, 2.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(19.9656));
}

pub fn T7(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [1. + r, 2. * r, 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 135.);
    operands_checked(s, Operation::Fuse, Expected::Mass(19.9656));
}

pub fn T8(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [-0.5, -r, 0.], [1.5, 2. * r, 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 60.);
    operands_checked(s, Operation::Fuse, Expected::Mass(20.6285));
}

pub fn T9(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [2. * r, r + 1., 2.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(19.9656));
}

pub fn U1(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -1., 0.], [2. * r, 1. + r, 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 30.);
    operands_checked(s, Operation::Fuse, Expected::Mass(21.6831));
}

pub fn U2(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-0.8, -1., 0.], [1.6, 1.8, 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 90.);
    operands_checked(s, Operation::Fuse, Expected::Mass(20.863));
}

pub fn U3(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-0.8, -1., 0.], [1.6, 1.8, 2.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(20.863));
}

pub fn U4(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1.1, 0.], [1., 2.2, 2.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(22.6248));
}

pub fn U5(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [0., -1.1, 0.], [1., 2.2, 2.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(22.6248));
}

pub fn U6(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1.1, -1., 0.], [2.2, 1., 2.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(22.6248));
}

pub fn U7(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., 1., 0.], [2., 2., 2.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(42.8496));
}

pub fn U8(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [1., -1., 0.], [2., 2., 2.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(42.8496));
}

pub fn U9(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [2. * r, 2. * r, 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 135.);
    operands_checked(s, Operation::Fuse, Expected::Mass(18.8496));
}

pub fn V1(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [2. * r, 2. * r, 2.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(18.8496));
}
