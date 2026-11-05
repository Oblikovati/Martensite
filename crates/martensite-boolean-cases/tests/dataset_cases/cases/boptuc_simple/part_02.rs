//! The `boptuc_simple` case bodies, part 2; each statement is one line of its script.
use super::*;

pub fn H2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let a30 = martensite_cmath::atan2(1_f64, 2.) * 180. / PI;
    let r = 5_f64.sqrt() * 0.5;
    make_box(s, "b2", [0., 0.25, 0.], [r, r, 1.]);
    trotate(s, "b2", [0., 0.25, 0.], [0., 0., 1.], a30);
    operands_checked(s, Operation::Cut21, Expected::Mass(6.23084));
}

pub fn H3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 2_f64.sqrt() * 0.5;
    make_box(s, "b2", [0.5, 0., 0.], [1., r, 1.]);
    trotate(s, "b2", [0.5, 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Cut21, Expected::Mass(2.17157));
}

pub fn H4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 2_f64.sqrt() * 0.5;
    make_box(s, "b2", [0.5, 0., 0.], [1.5, r, 1.]);
    trotate(s, "b2", [0.5, 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Cut21, Expected::Mass(4.16421));
}

pub fn H5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let a30 = martensite_cmath::atan2(1_f64, 2.) * 180. / PI;
    let r = 5_f64.sqrt();
    make_box(s, "b2", [-0.1, 0., 0.], [1.5, 1. / r, 1.]);
    trotate(s, "b2", [1., 0., 0.], [0., 0., 1.], a30);
    operands_checked(s, Operation::Cut21, Expected::Mass(5.11803));
}

pub fn H6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 2_f64.sqrt();
    make_box(s, "b2", [0., -0.25, 0.], [r, 0.5, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Cut21, Expected::Mass(3.66421));
}

pub fn H7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.25, 0.], [0.1, 0.5, 1.]);
    trotate(s, "b2", [0., 0.25, 0.], [0., 0., 1.], 240.);
    operands_checked(s, Operation::Cut21, Expected::Mass(0.769504));
}

pub fn H8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let a30 = martensite_cmath::atan2(1_f64, 2.) * 180. / PI;
    let r = 5_f64.sqrt() * 0.5;
    make_box(s, "b2", [0., 0.25, 0.], [r, 0.5, 1.]);
    trotate(s, "b2", [0., 0.25, 0.], [0., 0., 1.], a30);
    operands_checked(s, Operation::Cut21, Expected::Mass(2.68576));
}

pub fn H9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, 0.25, 0.], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0.25, 0.25, 0.], [0., 0., 1.], 30.);
    operands_checked(s, Operation::Cut21, Expected::Empty);
}

pub fn I1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, 0.25, 0.], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0.25, 0.25, 0.], [0., 0., 1.], 120.);
    operands_checked(s, Operation::Cut21, Expected::Mass(2.14434));
}

pub fn I2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.25, 0.], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0., 0.25, 0.], [0., 0., 1.], 35.);
    operands_checked(s, Operation::Cut21, Expected::Mass(1.63554));
}

pub fn I3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.25, 0.], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0., 0.25, 0.], [0., 0., 1.], 115.);
    operands_checked(s, Operation::Cut21, Expected::Mass(2.5));
}

pub fn I4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.25, 0.], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0., 0.25, 0.], [0., 0., 1.], 230.);
    operands_checked(s, Operation::Cut21, Expected::Mass(2.58452));
}

pub fn I5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.25, 0.], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0., 0.25, 0.], [0., 0., 1.], 250.);
    operands_checked(s, Operation::Cut21, Expected::Mass(2.45473));
}

pub fn I6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.5, 0.], [1.5, 1.5, 1.]);
    trotate(s, "b2", [0., 0.5, 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Cut21, Expected::Mass(9.58579));
}

pub fn I7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.5, 0.], [0.8, 0.8, 1.]);
    trotate(s, "b2", [0., 0.5, 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Cut21, Expected::Mass(1.3204));
}

pub fn I8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.25, 0.], [0.8, 0.8, 1.]);
    trotate(s, "b2", [0., 0.25, 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Cut21, Expected::Mass(2.39241));
}

pub fn I9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.25, 0.], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0., 0.25, 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Cut21, Expected::Mass(0.521447));
}

pub fn J1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 2_f64.sqrt() * 0.75;
    make_box(s, "b2", [0., 0.625, 0.], [r, r, 1.]);
    trotate(s, "b2", [0., 0.625, 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Cut21, Expected::Mass(5.19853));
}

pub fn J2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.25, 0., 0.], [0.5, 1.7, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Cut21, Expected::Mass(4.52157));
}

pub fn J3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.25, 0., 0.], [0.5, 1.7, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -30.);
    operands_checked(s, Operation::Cut21, Expected::Mass(4.72361));
}

pub fn J4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.25, -0.25, 0.], [0.25, 2., 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -30.);
    operands_checked(s, Operation::Cut21, Expected::Mass(4.12361));
}

pub fn J5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.25, 0., 0.], [0.5, 1.3, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Cut21, Expected::Mass(2.79619));
}

pub fn J6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., -0.25, 0.], [0.5, 1.55, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -35.);
    operands_checked(s, Operation::Cut21, Expected::Mass(3.56678));
}

pub fn J7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.1, 0.25, 0.], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0., 0.25, 0.], [0., 0., 1.], 230.);
    operands_checked(s, Operation::Cut21, Expected::Mass(2.55897));
}

pub fn J8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.1, 0.4, 0.], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0.1, 0.4, 0.], [0., 0., 1.], 245.);
    operands_checked(s, Operation::Cut21, Expected::Mass(1.89718));
}

pub fn J9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.25, -0.25, 0.], [0.5, 0.5, 1.]);
    operands_checked(s, Operation::Cut21, Expected::Mass(2.375));
}

pub fn K1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, 0.25, 0.], [0.5, 0.5, 1.]);
    operands_checked(s, Operation::Cut21, Expected::Empty);
}

pub fn K2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, 0.25, 0.], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0.25, 0.25, 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Cut21, Expected::Mass(0.521447));
}

pub fn K3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, 0.25, 0.], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0.25, 0.25, 0.], [0., 0., 1.], 90.);
    operands_checked(s, Operation::Cut21, Expected::Mass(1.75));
}

pub fn K4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, 0.25, 0.], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0.25, 0.25, 0.], [0., 0., 1.], 125.);
    operands_checked(s, Operation::Cut21, Expected::Mass(2.15801));
}

pub fn K5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, -0.25, 0.], [0.5, 1.5, 1.]);
    operands_checked(s, Operation::Cut21, Expected::Mass(3.5));
}

pub fn K6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.9, -0.25, 0.], [0.5, 1.7, 1.]);
    trotate(s, "b2", [0.9, -0.25, 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Cut21, Expected::Mass(4.5216));
}

pub fn K7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [1.25, 0., 0.], [0.25, 1.7, 1.]);
    trotate(s, "b2", [1.1, 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Cut21, Expected::Mass(3.65013));
}

pub fn K8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.25, -0.25, 0.], [0.5, 1.7, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -30.);
    operands_checked(s, Operation::Cut21, Expected::Mass(4.72361));
}

pub fn K9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.25, -0.25, 0.], [0.5, 1.55, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Cut21, Expected::Mass(3.54619));
}

pub fn L1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.5, -0.25, 0.], [1., 1., 1.]);
    trotate(s, "b2", [0.5, -0.25, 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Cut21, Expected::Mass(4.35051));
}

pub fn L2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.15, 0.], [0.8, 0.9, 1.]);
    trotate(s, "b2", [0., 0.25, 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Cut21, Expected::Mass(3.18454));
}

pub fn L3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.5, -0.25, 0.], [0.5, 1., 1.]);
    trotate(s, "b2", [0.5, -0.25, 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Cut21, Expected::Mass(2.41789));
}

pub fn L4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 2_f64.sqrt();
    make_box(s, "b2", [0., 0., 0.], [r, r / 2., 0.5]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Cut21, Expected::Mass(3.41421));
}

pub fn L5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [0., 0., 0.], [r, r, 0.5]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Cut21, Expected::Mass(1.70711));
}

pub fn L6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 2_f64.sqrt();
    make_box(s, "b2", [0., 0., 0.], [r, 0.25, 0.5]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Cut21, Expected::Mass(0.978553));
}

pub fn L7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 31_f64.sqrt();
    make_box(s, "b2", [0., 0., 0.], [r / 4., 0.25, 0.5]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 34.73);
    operands_checked(s, Operation::Cut21, Expected::Mass(0.816945));
}

pub fn L8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 2_f64.sqrt();
    make_box(s, "b2", [0., 0., 0.], [1.5, r / 2., 0.5]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Cut21, Expected::Mass(3.62132));
}

pub fn L9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let a30 = martensite_cmath::atan2(1_f64, 2.) * 180. / PI;
    let r = 5_f64.sqrt();
    make_box(s, "b2", [0., 0.5, 0.], [r / 2., 1. / r, 0.5]);
    trotate(s, "b2", [0., 0.5, 0.], [0., 0., 1.], a30);
    operands_checked(s, Operation::Cut21, Expected::Mass(2.25623));
}

pub fn M1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0., 0.], [0.5, 0.5, 0.5]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Cut21, Expected::Mass(1.10355));
}

pub fn M2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let a30 = martensite_cmath::atan2(1_f64, 2.) * 180. / PI;
    let r = 5_f64.sqrt();
    make_box(s, "b2", [0., 0.5, 0.], [1. / r, r / 2., 0.5]);
    trotate(s, "b2", [0., 0.5, 0.], [0., 0., 1.], a30 - 90.);
    operands_checked(s, Operation::Cut21, Expected::Mass(0.68541));
}

pub fn M3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.5, 0., 0.], [0.5, 1.7, 0.5]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -30.);
    operands_checked(s, Operation::Cut21, Expected::Mass(3.53397));
}

pub fn M4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0., 0.], [1.5, 0.25, 0.5]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Cut21, Expected::Mass(1.10723));
}

pub fn M5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 31_f64.sqrt();
    make_box(s, "b2", [0., 0., 0.], [r / 4., 0.5, 0.5]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 34.73);
    operands_checked(s, Operation::Cut21, Expected::Mass(1.92137));
}

pub fn M6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0., 0.], [0.5, 0.5, 0.5]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 30.);
    operands_checked(s, Operation::Cut21, Expected::Mass(0.82735));
}

pub fn M7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0., 0.], [0.5, 0.5, 0.5]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 60.);
    operands_checked(s, Operation::Cut21, Expected::Mass(1.25));
}

pub fn M8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [1., 1., 0.], [1., 1., 0.5]);
    trotate(s, "b2", [1., 1., 1.], [0., 0., 1.], 30.);
    operands_checked(s, Operation::Cut21, Expected::Mass(4.));
}

pub fn M9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.25, 0., 0.], [0.25, 1.7, 0.5]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -30.);
    operands_checked(s, Operation::Cut21, Expected::Mass(1.8953));
}

pub fn N1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.5, 0., 0.], [0.5, 1.7, 0.5]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -50.);
    operands_checked(s, Operation::Cut21, Expected::Mass(2.65985));
}

pub fn N2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 30_f64.sqrt();
    make_box(s, "b2", [0., 0., 0.], [r / 4., 0.25, 0.5]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 40.);
    operands_checked(s, Operation::Cut21, Expected::Mass(0.755878));
}

pub fn N3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, 0.25, 0.], [0.5, 0.5, -1.]);
    trotate(s, "b2", [0.25, 0.25, 0.], [0., 0., 1.], 30.);
    operands_checked(s, Operation::Cut21, Expected::Mass(2.5));
}

pub fn N4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, 0.25, 0.], [0.5, 0.5, -1.]);
    trotate(s, "b2", [0.25, 0.25, 0.], [0., 0., 1.], 120.);
    operands_checked(s, Operation::Cut21, Expected::Mass(2.5));
}

pub fn N5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.25, 0.], [0.5, 0.5, -1.]);
    trotate(s, "b2", [0., 0.25, 0.], [0., 0., 1.], 35.);
    operands_checked(s, Operation::Cut21, Expected::Mass(2.5));
}

pub fn N6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.25, 0.], [0.5, 0.5, -1.]);
    trotate(s, "b2", [0., 0.25, 0.], [0., 0., 1.], 115.);
    operands_checked(s, Operation::Cut21, Expected::Mass(2.5));
}

pub fn N7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.25, 0.], [0.5, 0.5, -1.]);
    trotate(s, "b2", [0., 0.25, 0.], [0., 0., 1.], 230.);
    operands_checked(s, Operation::Cut21, Expected::Mass(2.5));
}

pub fn N8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.25, 0.], [0.5, 0.5, -1.]);
    trotate(s, "b2", [0., 0.25, 0.], [0., 0., 1.], 250.);
    operands_checked(s, Operation::Cut21, Expected::Mass(2.5));
}
