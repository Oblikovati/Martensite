//! The `bopcommon_simple` case bodies, part 3; each statement is one line of its script.
use super::*;

pub fn N9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.5, 0.], [1.5, 1.5, -1.]);
    trotate(s, "b2", [0., 0.5, 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Common, Expected::Empty);
}

pub fn O1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.5, 0.], [0.8, 0.8, -1.]);
    trotate(s, "b2", [0., 0.5, 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Common, Expected::Empty);
}

pub fn O2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.25, 0.], [0.8, 0.8, -1.]);
    trotate(s, "b2", [0., 0.25, 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Common, Expected::Empty);
}

pub fn O3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.25, 0.], [0.5, 0.5, -1.]);
    trotate(s, "b2", [0., 0.25, 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Common, Expected::Empty);
}

pub fn O4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 2_f64.sqrt() * 0.75;
    make_box(s, "b2", [0., 0.625, 0.], [r, r, -1.]);
    trotate(s, "b2", [0., 0.625, 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Common, Expected::Empty);
}

pub fn O5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.25, 0., 0.], [0.5, 1.7, -1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Common, Expected::Empty);
}

pub fn O6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.25, 0., 0.], [0.5, 1.7, -1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -30.);
    operands_checked(s, Operation::Common, Expected::Empty);
}

pub fn O7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.25, -0.25, 0.], [0.25, 2., -1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -30.);
    operands_checked(s, Operation::Common, Expected::Empty);
}

pub fn O8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.25, 0., 0.], [0.5, 1.3, -1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Common, Expected::Empty);
}

pub fn O9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., -0.25, 0.], [0.5, 1.55, -1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -35.);
    operands_checked(s, Operation::Common, Expected::Empty);
}

pub fn P1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.25, 0.5], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0., 0.25, 0.], [0., 0., 1.], 230.);
    operands_checked(s, Operation::Common, Expected::Mass(0.542919));
}

pub fn P2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.1, 0.4, 0.5], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0.1, 0.4, 0.], [0., 0., 1.], 245.);
    operands_checked(s, Operation::Common, Expected::Mass(1.10649));
}

pub fn P3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.25, -0.25, 0.5], [0.5, 0.5, 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(0.625));
}

pub fn P4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, 0.5, 0.5], [0.5, 0.5, 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(1.5));
}

pub fn P5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, 0.5, 0.5], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0.25, 0.25, 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Common, Expected::Mass(1.12132));
}

pub fn P6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, 0.5, 0.5], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0.25, 0.25, 0.], [0., 0., 1.], 90.);
    operands_checked(s, Operation::Common, Expected::Empty);
}

pub fn P7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, 0.25, 0.5], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0.25, 0.25, 0.], [0., 0., 1.], 125.);
    operands_checked(s, Operation::Common, Expected::Mass(0.769594));
}

pub fn P8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, -0.25, 0.5], [0.5, 1.5, 1.]);
    operands_checked(s, Operation::Common, Expected::Mass(2.5));
}

pub fn P9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.9, -0.25, 0.5], [0.5, 1.7, 1.]);
    trotate(s, "b2", [0.9, -0.25, 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Common, Expected::Mass(2.78551));
}

pub fn Q1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [1.25, 0., 0.5], [0.25, 1.7, 1.]);
    trotate(s, "b2", [1.1, 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Common, Expected::Mass(1.43774));
}

pub fn Q2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.25, -0.25, 0.5], [0.5, 1.7, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -30.);
    operands_checked(s, Operation::Common, Expected::Mass(2.5594));
}

pub fn Q3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.25, -0.25, 0.5], [0.5, 1.55, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Common, Expected::Mass(2.71214));
}

pub fn Q4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.5, -0.25, 0.5], [1., 1., 1.]);
    trotate(s, "b2", [0.5, -0.25, 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Common, Expected::Mass(3.30635));
}

pub fn Q5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.15, 0.5], [0.8, 0.9, 1.]);
    trotate(s, "b2", [0., 0.25, 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Common, Expected::Mass(2.55312));
}

pub fn Q6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.5, -0.25, 0.5], [0.5, 1., 1.]);
    trotate(s, "b2", [0.5, -0.25, 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Common, Expected::Mass(2.09987));
}

pub fn Q7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.25, -0.25, 0.25], [0.25, 1., 0.5]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -30.);
    operands_checked(s, Operation::Common, Expected::Mass(1.17524));
}

pub fn Q8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., -0.25, 0.25], [0.5, 1.7, 0.5]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -35.);
    operands_checked(s, Operation::Common, Expected::Mass(2.7099));
}

pub fn Q9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [3., 3., 3.]);
    make_box(s, "b2", [0., 0., 0.], [1., 1., 1.]);
    trotate(s, "b2", [0., 0., 0.], [1., 0., 0.], -30.);
    ttranslate(s, "b2", [1., 1., 3.]);
    operands_checked(s, Operation::Common, Expected::Mass(3.3094));
}

pub fn R1(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [2., 2., 2.]);
    operands_checked(s, Operation::Common, Expected::Mass(18.8496));
}

pub fn R2(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [2., 2., 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Common, Expected::Mass(18.8496));
}

pub fn R3(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [2., 1.5, 2.]);
    operands_checked(s, Operation::Common, Expected::Mass(16.8965));
}

pub fn R4(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [-1., -1., 0.], [1. + r, 2., 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 60.);
    operands_checked(s, Operation::Common, Expected::Mass(18.574));
}

pub fn R5(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [1.5, 2., 2.]);
    operands_checked(s, Operation::Common, Expected::Mass(16.8965));
}

pub fn R6(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [2., 2.5, 2.]);
    operands_checked(s, Operation::Common, Expected::Mass(18.8496));
}

pub fn R7(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [2.5, 2., 2.]);
    operands_checked(s, Operation::Common, Expected::Mass(18.8496));
}

pub fn R8(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [1., 2., 2.]);
    operands_checked(s, Operation::Common, Expected::Mass(13.4248));
}

pub fn R9(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [0., -1., 0.], [1., 2., 2.]);
    operands_checked(s, Operation::Common, Expected::Mass(13.4248));
}

pub fn S1(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [2., 1., 2.]);
    operands_checked(s, Operation::Common, Expected::Mass(13.4248));
}

pub fn S2(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-1., -1., 0.], [1. + r, 1. + r, 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Common, Expected::Mass(17.0816));
}

pub fn S3(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-1., -1., 0.], [1. + r, 1. + r, 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Common, Expected::Mass(17.0816));
}

pub fn S4(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-1., -1., 0.], [1. + r, 1. + r, 2.]);
    operands_checked(s, Operation::Common, Expected::Mass(17.0816));
}

pub fn S5(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [1. + r, 1. + r, 2.]);
    operands_checked(s, Operation::Common, Expected::Mass(17.0816));
}

pub fn S6(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-0.8, -0.8, 0.], [1.8, 1.8, 2.]);
    operands_checked(s, Operation::Common, Expected::Mass(17.8475));
}

pub fn S7(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -0.8, 0.], [1.8, 1.8, 2.]);
    operands_checked(s, Operation::Common, Expected::Mass(17.8475));
}

pub fn S8(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [-1., -r, 0.], [1. + r, 1. + r, 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 30.);
    operands_checked(s, Operation::Common, Expected::Mass(18.2984));
}

pub fn S9(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -0.8, 0.], [2., 1.6, 2.]);
    operands_checked(s, Operation::Common, Expected::Mass(17.8475));
}

pub fn T1(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-0.8, -1., 0.], [1.6, 2., 2.]);
    operands_checked(s, Operation::Common, Expected::Mass(17.8475));
}

pub fn T2(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -1., 0.], [2. * r, 2., 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 30.);
    operands_checked(s, Operation::Common, Expected::Mass(18.2984));
}

pub fn T3(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-0.5, -1., 0.], [1.5, 1.5, 2.]);
    operands_checked(s, Operation::Common, Expected::Mass(14.6841));
}

pub fn T4(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-0.5, -1., 0.], [1.5, 1.5, 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 30.);
    operands_checked(s, Operation::Common, Expected::Mass(14.6841));
}

pub fn T5(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1., 0.], [1.5, 1.5, 2.]);
    operands_checked(s, Operation::Common, Expected::Mass(14.6841));
}

pub fn T6(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [1. + r, 2. * r, 2.]);
    operands_checked(s, Operation::Common, Expected::Mass(16.1977));
}

pub fn T7(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [1. + r, 2. * r, 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 135.);
    operands_checked(s, Operation::Common, Expected::Mass(16.1977));
}

pub fn T8(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [-0.5, -r, 0.], [1.5, 2. * r, 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 60.);
    operands_checked(s, Operation::Common, Expected::Mass(16.3454));
}

pub fn T9(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [2. * r, r + 1., 2.]);
    operands_checked(s, Operation::Common, Expected::Mass(16.1977));
}

pub fn U1(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 3_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -1., 0.], [2. * r, 1. + r, 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 30.);
    operands_checked(s, Operation::Common, Expected::Mass(18.0229));
}

pub fn U2(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-0.8, -1., 0.], [1.6, 1.8, 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 90.);
    operands_checked(s, Operation::Common, Expected::Mass(17.3465));
}

pub fn U3(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-0.8, -1., 0.], [1.6, 1.8, 2.]);
    operands_checked(s, Operation::Common, Expected::Mass(17.3465));
}

pub fn U4(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., -1.1, 0.], [1., 2.2, 2.]);
    operands_checked(s, Operation::Common, Expected::Mass(13.4248));
}

pub fn U5(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [0., -1.1, 0.], [1., 2.2, 2.]);
    operands_checked(s, Operation::Common, Expected::Mass(13.4248));
}

pub fn U6(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1.1, -1., 0.], [2.2, 1., 2.]);
    operands_checked(s, Operation::Common, Expected::Mass(13.4248));
}

pub fn U7(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [-1., 1., 0.], [2., 2., 2.]);
    operands_checked(s, Operation::Common, Expected::Empty);
}

pub fn U8(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    make_box(s, "b2", [1., -1., 0.], [2., 2., 2.]);
    operands_checked(s, Operation::Common, Expected::Empty);
}

pub fn U9(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [2. * r, 2. * r, 2.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 135.);
    operands_checked(s, Operation::Common, Expected::Mass(15.3137));
}

pub fn V1(s: &mut dyn Session) {
    pcylinder(s, "b1", 1., 2.);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [-r, -r, 0.], [2. * r, 2. * r, 2.]);
    operands_checked(s, Operation::Common, Expected::Mass(15.3137));
}
