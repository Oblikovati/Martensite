//! The `bopfuse_simple` case bodies, part 1; each statement is one line of its script.
use super::*;

pub fn A1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0., 0.], [1., 1., 1.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(6.));
}

pub fn A2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0., 0.], [1., 1.5, 1.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(8.));
}

pub fn A3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 1., 0.], [1., 0.5, 1.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(8.));
}

pub fn A4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [1., 1., 0.], [1., 1., 1.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(12.));
}

pub fn A5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0., 0.], [0.5, 1., 0.5]);
    operands_checked(s, Operation::Fuse, Expected::Mass(6.));
}

pub fn A6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., -0.5, 0.], [0.5, 0.5, 1.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(7.5));
}

pub fn A7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., -0.5, 0.], [0.5, 1.5, 1.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(7.5));
}

pub fn A8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.5, 0.], [1., 1., 1.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(8.));
}

pub fn A9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.25, 0.], [1., 0.5, 1.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(6.));
}

pub fn B1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0., 0.], [0.5, 0.5, 0.5]);
    operands_checked(s, Operation::Fuse, Expected::Mass(6.));
}

pub fn B2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., -0.5, 0.], [0.5, 0.5, 0.5]);
    operands_checked(s, Operation::Fuse, Expected::Mass(7.));
}

pub fn B3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., -0.5, -0.5], [0.5, 0.5, 0.5]);
    operands_checked(s, Operation::Fuse, Expected::Mass(7.5));
}

pub fn B4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.5, -0.5, -0.5], [0.5, 0.5, 0.5]);
    operands_checked(s, Operation::Fuse, Expected::Mass(7.5));
}

pub fn B5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0., 0.], [1.5, 0.5, 0.5]);
    operands_checked(s, Operation::Fuse, Expected::Mass(7.));
}

pub fn B6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., -0.5, 0.], [1.5, 0.5, 0.5]);
    operands_checked(s, Operation::Fuse, Expected::Mass(8.5));
}

pub fn B7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, 0., 0.], [0.5, 0.5, 1.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(6.));
}

pub fn B8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, -0.5, 0.], [0.5, 0.5, 1.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(7.5));
}

pub fn B9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, 0., 0.], [0.5, 1.5, 1.]);
    operands_checked(s, Operation::Fuse, Expected::Mass(7.5));
}

pub fn C1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.5, 0., 0.], [1., 1., 0.5]);
    operands_checked(s, Operation::Fuse, Expected::Mass(7.5));
}

pub fn C2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.5, 0., -0.5], [1., 1., 0.5]);
    operands_checked(s, Operation::Fuse, Expected::Mass(9.));
}

pub fn C3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 2_f64.sqrt();
    make_box(s, "b2", [0., 0., 0.], [r, r / 2., 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Fuse, Expected::Mass(7.82843));
}

pub fn C4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [0., 0., 0.], [r, r, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Fuse, Expected::Mass(6.91421));
}

pub fn C5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 2_f64.sqrt();
    make_box(s, "b2", [0., 0., 0.], [r, 0.25, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Fuse, Expected::Mass(6.41789));
}

pub fn C6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 31_f64.sqrt();
    make_box(s, "b2", [0., 0., 0.], [r / 4., 0.25, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 34.73);
    operands_checked(s, Operation::Fuse, Expected::Mass(6.32953));
}

pub fn C7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 2_f64.sqrt();
    make_box(s, "b2", [0., 0., 0.], [1.5, r / 2., 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Fuse, Expected::Mass(8.12132));
}

pub fn C8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let a30 = martensite_cmath::atan2(1_f64, 2.) * 180. / PI;
    let r = 5_f64.sqrt();
    make_box(s, "b2", [0., 0.5, 0.], [r / 2., 1. / r, 1.]);
    trotate(s, "b2", [0., 0.5, 0.], [0., 0., 1.], a30);
    operands_checked(s, Operation::Fuse, Expected::Mass(7.01246));
}

pub fn C9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0., 0.], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Fuse, Expected::Mass(6.54289));
}

pub fn D1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let a30 = martensite_cmath::atan2(1_f64, 2.) * 180. / PI;
    let r = 5_f64.sqrt();
    make_box(s, "b2", [0., 0.5, 0.], [1. / r, r / 2., 1.]);
    trotate(s, "b2", [0., 0.5, 0.], [0., 0., 1.], a30 - 90.);
    operands_checked(s, Operation::Fuse, Expected::Mass(6.27082));
}

pub fn D2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.5, 0., 0.], [0.5, 1.7, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -30.);
    operands_checked(s, Operation::Fuse, Expected::Mass(8.7906));
}

pub fn D3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0., 0.], [1.5, 0.25, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Fuse, Expected::Mass(6.63236));
}

pub fn D4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 31_f64.sqrt();
    make_box(s, "b2", [0., 0., 0.], [r / 4., 0.5, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 34.73);
    operands_checked(s, Operation::Fuse, Expected::Mass(6.83585));
}

pub fn D5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0., 0.], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 30.);
    operands_checked(s, Operation::Fuse, Expected::Mass(6.35566));
}

pub fn D6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0., 0.], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 60.);
    operands_checked(s, Operation::Fuse, Expected::Mass(6.98964));
}

pub fn D7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [1., 1., 0.], [1., 1., 1.]);
    trotate(s, "b2", [1., 1., 1.], [0., 0., 1.], 30.);
    operands_checked(s, Operation::Fuse, Expected::Mass(12.));
}

pub fn D8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.25, 0., 0.], [0.25, 1.7, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -30.);
    operands_checked(s, Operation::Fuse, Expected::Mass(7.79626));
}

pub fn D9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.5, 0., 0.], [0.5, 1.7, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -50.);
    operands_checked(s, Operation::Fuse, Expected::Mass(7.83743));
}

pub fn E1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 30_f64.sqrt();
    make_box(s, "b2", [0., 0., 0.], [r / 4., 0.25, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 40.);
    operands_checked(s, Operation::Fuse, Expected::Mass(6.30133));
}

pub fn E2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, 0., 0.], [0.5, 0.5, 0.5]);
    operands_checked(s, Operation::Fuse, Expected::Mass(6.));
}

pub fn E3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, -0.5, 0.], [0.5, 0.5, 0.5]);
    operands_checked(s, Operation::Fuse, Expected::Mass(7.));
}

pub fn E4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, -0.5, -0.5], [0.5, 0.5, 0.5]);
    operands_checked(s, Operation::Fuse, Expected::Mass(7.5));
}

pub fn E5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, 0., 0.], [0.5, 1.5, 0.5]);
    operands_checked(s, Operation::Fuse, Expected::Mass(7.));
}

pub fn E6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, 0., 0.], [1.5, 0.5, 0.5]);
    operands_checked(s, Operation::Fuse, Expected::Mass(7.5));
}

pub fn E7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, -0.5, 0.], [1.5, 0.5, 0.5]);
    operands_checked(s, Operation::Fuse, Expected::Mass(8.75));
}

pub fn E8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, -0.5, -0.5], [1.5, 0.5, 0.5]);
    operands_checked(s, Operation::Fuse, Expected::Mass(9.5));
}

pub fn E9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, 0., 0.], [1.5, 1.5, 0.5]);
    operands_checked(s, Operation::Fuse, Expected::Mass(10.25));
}

pub fn F1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.25, 0., 0.], [1.5, 0.5, 0.5]);
    operands_checked(s, Operation::Fuse, Expected::Mass(7.));
}

pub fn F2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.25, -0.5, 0.], [1.5, 0.5, 0.5]);
    operands_checked(s, Operation::Fuse, Expected::Mass(8.5));
}

pub fn F3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.25, -0.5, -0.5], [1.5, 0.5, 0.5]);
    operands_checked(s, Operation::Fuse, Expected::Mass(9.5));
}

pub fn F4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, 0.25, 0.], [1.5, 0.5, 0.5]);
    operands_checked(s, Operation::Fuse, Expected::Mass(7.5));
}

pub fn F5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [0., 0.5, 0.], [r, r, 1.]);
    trotate(s, "b2", [0., 0.5, 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Fuse, Expected::Mass(6.));
}

pub fn F6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 2_f64.sqrt() * 0.75;
    make_box(s, "b2", [0., 0.5, 0.], [r, r, 1.]);
    trotate(s, "b2", [0., 0.5, 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Fuse, Expected::Mass(7.57843));
}

pub fn F7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let a30 = martensite_cmath::atan2(1_f64, 2.) * 180. / PI;
    let r = 5_f64.sqrt();
    make_box(s, "b2", [0., 0.5, 0.], [1.25, 1. / r, 1.]);
    trotate(s, "b2", [0., 0.5, 0.], [0., 0., 1.], a30);
    operands_checked(s, Operation::Fuse, Expected::Mass(7.39443));
}

pub fn F8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 2_f64.sqrt();
    make_box(s, "b2", [-0.25, -0.25, 0.], [2., r / 2., 1.]);
    trotate(s, "b2", [-0.25, -0.25, 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Fuse, Expected::Mass(9.82843));
}

pub fn F9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [0., 0.5, 0.], [r, r / 2., 1.]);
    trotate(s, "b2", [0., 0.5, 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Fuse, Expected::Mass(6.66421));
}

pub fn G1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let a30 = martensite_cmath::atan2(1_f64, 2.) * 180. / PI;
    let r = 5_f64.sqrt();
    make_box(s, "b2", [0., 0.5, 0.], [2. / r, 1.25, 1.]);
    trotate(s, "b2", [0., 0.5, 0.], [0., 0., 1.], a30 - 90.);
    operands_checked(s, Operation::Fuse, Expected::Mass(7.72287));
}

pub fn G2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let a30 = martensite_cmath::atan2(1_f64, 2.) * 180. / PI;
    let r = 5_f64.sqrt() / 4.;
    make_box(s, "b2", [0.25, 0., 0.], [1.5 * r, r, 1.]);
    trotate(s, "b2", [0.25, 0., 0.], [0., 0., 1.], a30);
    operands_checked(s, Operation::Fuse, Expected::Mass(6.));
}

pub fn G3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [1. - r, 0., 0.], [2. * r, 1.5 * r, 1.]);
    trotate(s, "b2", [1., 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Fuse, Expected::Mass(8.49264));
}

pub fn G4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 2_f64.sqrt();
    make_box(s, "b2", [-0.25, -0.25, 0.], [2., r / 4., 1.]);
    trotate(s, "b2", [-0.25, -0.25, 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Fuse, Expected::Mass(8.25));
}

pub fn G5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let a30 = martensite_cmath::atan2(1_f64, 2.) * 180. / PI;
    let r = 5_f64.sqrt();
    make_box(s, "b2", [0., 0.25, 0.], [1.3, 3. / r / 2., 1.]);
    trotate(s, "b2", [0., 0.25, 0.], [0., 0., 1.], a30);
    operands_checked(s, Operation::Fuse, Expected::Mass(7.56774));
}

pub fn G6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let a30 = martensite_cmath::atan2(1_f64, 2.) * 180. / PI;
    let r = 5_f64.sqrt();
    make_box(s, "b2", [0., 0.45, 0.], [1. / r, r / 2., 1.]);
    trotate(s, "b2", [0., 0.45, 0.], [0., 0., 1.], a30 - 90.);
    operands_checked(s, Operation::Fuse, Expected::Mass(6.27082));
}

pub fn G7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.25, 0.], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0., 0.25, 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Fuse, Expected::Mass(6.54289));
}

pub fn G8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.25, 0.], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0., 0.25, 0.], [0., 0., 1.], -30.);
    operands_checked(s, Operation::Fuse, Expected::Mass(6.));
}

pub fn G9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.25, 0.], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0., 0.25, 0.], [0., 0., 1.], 240.);
    operands_checked(s, Operation::Fuse, Expected::Mass(7.20873));
}

pub fn H1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 2_f64.sqrt() * 0.875;
    make_box(s, "b2", [0., 0.75, 0.], [r, r, 1.]);
    trotate(s, "b2", [0., 0.75, 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Fuse, Expected::Mass(9.22303));
}
