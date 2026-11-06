//! The `bopcut_simple` case bodies, part 1; each statement is one line of its script.
use super::*;

pub fn A1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0., 0.], [1., 1., 1.]);
    operands_checked(s, Operation::Cut, Expected::Empty);
}

pub fn A2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0., 0.], [1., 1.5, 1.]);
    operands_checked(s, Operation::Cut, Expected::Empty);
}

pub fn A3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 1., 0.], [1., 0.5, 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(6.));
}

pub fn A4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [1., 1., 0.], [1., 1., 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(6.));
}

pub fn A5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0., 0.], [0.5, 1., 0.5]);
    operands_checked(s, Operation::Cut, Expected::Mass(5.5));
}

pub fn A6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., -0.5, 0.], [0.5, 0.5, 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(6.));
}

pub fn A7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., -0.5, 0.], [0.5, 1.5, 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(4.));
}

pub fn A8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.5, 0.], [1., 1., 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(4.));
}

pub fn A9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.25, 0.], [1., 0.5, 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(6.));
}

pub fn B1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0., 0.], [0.5, 0.5, 0.5]);
    operands_checked(s, Operation::Cut, Expected::Mass(6.));
}

pub fn B2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., -0.5, 0.], [0.5, 0.5, 0.5]);
    operands_checked(s, Operation::Cut, Expected::Mass(6.));
}

pub fn B3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., -0.5, -0.5], [0.5, 0.5, 0.5]);
    operands_checked(s, Operation::Cut, Expected::Mass(6.));
}

pub fn B4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.5, -0.5, -0.5], [0.5, 0.5, 0.5]);
    operands_checked(s, Operation::Cut, Expected::Mass(6.));
}

pub fn B5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0., 0.], [1.5, 0.5, 0.5]);
    operands_checked(s, Operation::Cut, Expected::Mass(5.5));
}

pub fn B6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., -0.5, 0.], [1.5, 0.5, 0.5]);
    operands_checked(s, Operation::Cut, Expected::Mass(6.));
}

pub fn B7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, 0., 0.], [0.5, 0.5, 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(6.5));
}

pub fn B8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, -0.5, 0.], [0.5, 0.5, 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(6.));
}

pub fn B9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, 0., 0.], [0.5, 1.5, 1.]);
    operands_checked(s, Operation::Cut, Expected::Mass(6.));
}

pub fn C1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.5, 0., 0.], [1., 1., 0.5]);
    operands_checked(s, Operation::Cut, Expected::Mass(5.5));
}

pub fn C2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.5, 0., -0.5], [1., 1., 0.5]);
    operands_checked(s, Operation::Cut, Expected::Mass(6.));
}

pub fn C3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 2_f64.sqrt();
    make_box(s, "b2", [0., 0., 0.], [r, r / 2., 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Cut, Expected::Mass(4.41421));
}

pub fn C4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [0., 0., 0.], [r, r, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Cut, Expected::Mass(5.91421));
}

pub fn C5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 2_f64.sqrt();
    make_box(s, "b2", [0., 0., 0.], [r, 0.25, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Cut, Expected::Mass(7.03921));
}

pub fn C6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 31_f64.sqrt();
    make_box(s, "b2", [0., 0., 0.], [r / 4., 0.25, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 34.73);
    operands_checked(s, Operation::Cut, Expected::Mass(7.21677));
}

pub fn C7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 2_f64.sqrt();
    make_box(s, "b2", [0., 0., 0.], [1.5, r / 2., 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Cut, Expected::Mass(4.41421));
}

pub fn C8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let a30 = martensite_cmath::atan2(1_f64, 2.) * 180. / PI;
    let r = 5_f64.sqrt();
    make_box(s, "b2", [0., 0.5, 0.], [r / 2., 1. / r, 1.]);
    trotate(s, "b2", [0., 0.5, 0.], [0., 0., 1.], a30);
    operands_checked(s, Operation::Cut, Expected::Mass(5.11803));
}

pub fn C9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0., 0.], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Cut, Expected::Mass(6.04289));
}

pub fn D1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let a30 = martensite_cmath::atan2(1_f64, 2.) * 180. / PI;
    let r = 5_f64.sqrt();
    make_box(s, "b2", [0., 0.5, 0.], [1. / r, r / 2., 1.]);
    trotate(s, "b2", [0., 0.5, 0.], [0., 0., 1.], a30 - 90.);
    operands_checked(s, Operation::Cut, Expected::Mass(7.05967));
}

pub fn D2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.5, 0., 0.], [0.5, 1.7, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -30.);
    operands_checked(s, Operation::Cut, Expected::Mass(5.));
}

pub fn D3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0., 0.], [1.5, 0.25, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Cut, Expected::Mass(7.03921));
}

pub fn D4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 31_f64.sqrt();
    make_box(s, "b2", [0., 0., 0.], [r / 4., 0.5, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 34.73);
    operands_checked(s, Operation::Cut, Expected::Mass(5.46836));
}

pub fn D5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0., 0.], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 30.);
    operands_checked(s, Operation::Cut, Expected::Mass(6.27831));
}

pub fn D6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0., 0.], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 60.);
    operands_checked(s, Operation::Cut, Expected::Mass(6.06699));
}

pub fn D7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [1., 1., 0.], [1., 1., 1.]);
    trotate(s, "b2", [1., 1., 1.], [0., 0., 1.], 30.);
    operands_checked(s, Operation::Cut, Expected::Mass(6.));
}

pub fn D8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.25, 0., 0.], [0.25, 1.7, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -30.);
    operands_checked(s, Operation::Cut, Expected::Mass(6.51036));
}

pub fn D9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.5, 0., 0.], [0.5, 1.7, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], -50.);
    operands_checked(s, Operation::Cut, Expected::Mass(5.42883));
}

pub fn E1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 30_f64.sqrt();
    make_box(s, "b2", [0., 0., 0.], [r / 4., 0.25, 1.]);
    trotate(s, "b2", [0., 0., 0.], [0., 0., 1.], 40.);
    operands_checked(s, Operation::Cut, Expected::Mass(7.24544));
}

pub fn E2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, 0., 0.], [0.5, 0.5, 0.5]);
    operands_checked(s, Operation::Cut, Expected::Mass(6.5));
}

pub fn E3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, -0.5, 0.], [0.5, 0.5, 0.5]);
    operands_checked(s, Operation::Cut, Expected::Mass(6.));
}

pub fn E4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, -0.5, -0.5], [0.5, 0.5, 0.5]);
    operands_checked(s, Operation::Cut, Expected::Mass(6.));
}

pub fn E5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, 0., 0.], [0.5, 1.5, 0.5]);
    operands_checked(s, Operation::Cut, Expected::Mass(6.5));
}

pub fn E6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, 0., 0.], [1.5, 0.5, 0.5]);
    operands_checked(s, Operation::Cut, Expected::Mass(6.));
}

pub fn E7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, -0.5, 0.], [1.5, 0.5, 0.5]);
    operands_checked(s, Operation::Cut, Expected::Mass(6.));
}

pub fn E8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, -0.5, -0.5], [1.5, 0.5, 0.5]);
    operands_checked(s, Operation::Cut, Expected::Mass(6.));
}

pub fn E9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, 0., 0.], [1.5, 1.5, 0.5]);
    operands_checked(s, Operation::Cut, Expected::Mass(5.25));
}

pub fn F1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.25, 0., 0.], [1.5, 0.5, 0.5]);
    operands_checked(s, Operation::Cut, Expected::Mass(5.5));
}

pub fn F2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.25, -0.5, 0.], [1.5, 0.5, 0.5]);
    operands_checked(s, Operation::Cut, Expected::Mass(6.));
}

pub fn F3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [-0.25, -0.5, -0.5], [1.5, 0.5, 0.5]);
    operands_checked(s, Operation::Cut, Expected::Mass(6.));
}

pub fn F4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0.25, 0.25, 0.], [1.5, 0.5, 0.5]);
    operands_checked(s, Operation::Cut, Expected::Mass(6.75));
}

pub fn F5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [0., 0.5, 0.], [r, r, 1.]);
    trotate(s, "b2", [0., 0.5, 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Cut, Expected::Mass(7.82843));
}

pub fn F6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 2_f64.sqrt() * 0.75;
    make_box(s, "b2", [0., 0.5, 0.], [r, r, 1.]);
    trotate(s, "b2", [0., 0.5, 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Cut, Expected::Mass(3.91421));
}

pub fn F7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let a30 = martensite_cmath::atan2(1_f64, 2.) * 180. / PI;
    let r = 5_f64.sqrt();
    make_box(s, "b2", [0., 0.5, 0.], [1.25, 1. / r, 1.]);
    trotate(s, "b2", [0., 0.5, 0.], [0., 0., 1.], a30);
    operands_checked(s, Operation::Cut, Expected::Mass(5.11803));
}

pub fn F8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 2_f64.sqrt();
    make_box(s, "b2", [-0.25, -0.25, 0.], [2., r / 2., 1.]);
    trotate(s, "b2", [-0.25, -0.25, 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Cut, Expected::Mass(4.41421));
}

pub fn F9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [0., 0.5, 0.], [r, r / 2., 1.]);
    trotate(s, "b2", [0., 0.5, 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Cut, Expected::Mass(5.45711));
}

pub fn G1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let a30 = martensite_cmath::atan2(1_f64, 2.) * 180. / PI;
    let r = 5_f64.sqrt();
    make_box(s, "b2", [0., 0.5, 0.], [2. / r, 1.25, 1.]);
    trotate(s, "b2", [0., 0.5, 0.], [0., 0., 1.], a30 - 90.);
    operands_checked(s, Operation::Cut, Expected::Mass(4.55205));
}

pub fn G2(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let a30 = martensite_cmath::atan2(1_f64, 2.) * 180. / PI;
    let r = 5_f64.sqrt() / 4.;
    make_box(s, "b2", [0.25, 0., 0.], [1.5 * r, r, 1.]);
    trotate(s, "b2", [0.25, 0., 0.], [0., 0., 1.], a30);
    operands_checked(s, Operation::Cut, Expected::Mass(7.85758));
}

pub fn G3(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 2_f64.sqrt() / 2.;
    make_box(s, "b2", [1. - r, 0., 0.], [2. * r, 1.5 * r, 1.]);
    trotate(s, "b2", [1., 0., 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Cut, Expected::Mass(1.95711));
}

pub fn G4(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 2_f64.sqrt();
    make_box(s, "b2", [-0.25, -0.25, 0.], [2., r / 4., 1.]);
    trotate(s, "b2", [-0.25, -0.25, 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Cut, Expected::Mass(6.37132));
}

pub fn G5(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let a30 = martensite_cmath::atan2(1_f64, 2.) * 180. / PI;
    let r = 5_f64.sqrt();
    make_box(s, "b2", [0., 0.25, 0.], [1.3, 3. / r / 2., 1.]);
    trotate(s, "b2", [0., 0.25, 0.], [0., 0., 1.], a30);
    operands_checked(s, Operation::Cut, Expected::Mass(4.11803));
}

pub fn G6(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let a30 = martensite_cmath::atan2(1_f64, 2.) * 180. / PI;
    let r = 5_f64.sqrt();
    make_box(s, "b2", [0., 0.45, 0.], [1. / r, r / 2., 1.]);
    trotate(s, "b2", [0., 0.45, 0.], [0., 0., 1.], a30 - 90.);
    operands_checked(s, Operation::Cut, Expected::Mass(7.05967));
}

pub fn G7(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.25, 0.], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0., 0.25, 0.], [0., 0., 1.], 45.);
    operands_checked(s, Operation::Cut, Expected::Mass(6.04289));
}

pub fn G8(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.25, 0.], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0., 0.25, 0.], [0., 0., 1.], -30.);
    operands_checked(s, Operation::Cut, Expected::Mass(7.5));
}

pub fn G9(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    make_box(s, "b2", [0., 0.25, 0.], [0.5, 0.5, 1.]);
    trotate(s, "b2", [0., 0.25, 0.], [0., 0., 1.], 240.);
    operands_checked(s, Operation::Cut, Expected::Mass(5.70873));
}

pub fn H1(s: &mut dyn Session) {
    make_box(s, "b1", [0., 0., 0.], [1., 1., 1.]);
    let r = 2_f64.sqrt() * 0.875;
    make_box(s, "b2", [0., 0.75, 0.], [r, r, 1.]);
    trotate(s, "b2", [0., 0.75, 0.], [0., 0., 1.], -45.);
    operands_checked(s, Operation::Cut, Expected::Mass(4.03921));
}
