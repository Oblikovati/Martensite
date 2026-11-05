//! `tests/lowalgos/intss/bug29807_i1001` to `i4005`: a face pair of the same cone and cylinder, in
//! place, turned or converted, whose reached tolerance and two curves the scripts check.
use crate::support::{
    bopcurves::{BopCurvesOptions, bopcurves_bound},
    checkview::screenshot,
    commands::{explode, trotate, ttranslate},
    construction::nurbsconvert,
    lowalgos_commands::removeloc,
    restore::restore,
    session::{Session, ViewStep},
    tolerance_check::checkreal,
};
use martensite_tkg3d::top_abs::ShapeEnum;

/// The point of the scripts' rotations, and their two axes.
const AT: [f64; 3] = [
    23.85857157145715500000,
    12.00000000000000000000,
    5.50000000000000000000,
];
const FIRST_AXIS: [f64; 3] = [7., -7.14142842854285, 0.];
const SECOND_AXIS: [f64; 3] = [7.1414284285428495, 7.0000000000000009, -0.];

/// What the script does with the tolerance it reads back.
enum Reached {
    /// `checkreal Tolerance $Toler expected 0.0 0.01`.
    Equal(&'static str),
    /// `if { ${Toler} > cap } { puts "Error: bad tolerance of result" }`.
    AtMost(f64),
}

fn banner() {
    println!("========");
    println!("0029807: Impossible to cut cone from prism");
    println!("========");
    println!();
}

/// The first turn of the `b` scripts and the `removeloc` after it.
fn turned(s: &mut dyn Session, name: &str, degrees: f64) {
    trotate(s, name, AT, FIRST_AXIS, degrees);
    removeloc(s, name);
}

/// The two turns and the shift of the later scripts.
fn turned_twice(s: &mut dyn Session, name: &str, degrees: [f64; 2], shift: f64) {
    trotate(s, name, AT, FIRST_AXIS, degrees[0]);
    trotate(s, name, AT, SECOND_AXIS, degrees[1]);
    ttranslate(s, name, [0., 0., shift]);
}

/// From `smallview` to the last capture: the same in every one of these scripts.
fn checked(s: &mut dyn Session, faces: [&str; 2], reached: Reached) {
    s.view(ViewStep::SmallView);
    s.view(ViewStep::Donly(&faces));
    s.view(ViewStep::Fit);
    let log = bopcurves_bound(s, faces[0], faces[1], BopCurvesOptions::TWO_D);
    let (toler, nb_curv) = log.reached_and_count();
    match reached {
        Reached::Equal(expected) => checkreal("Tolerance", toler, expected, 0.0, 0.01),
        Reached::AtMost(cap) => {
            if log.reached() > cap {
                println!("Error: bad tolerance of result");
            }
        }
    }
    if nb_curv != 2 {
        println!("Error: Please check NbCurves for intersector");
    }
    screenshot(s, "photo_3d");
    s.view(ViewStep::PlaneView);
    s.view(ViewStep::Fit);
    screenshot(s, "photo_2d");
}

fn explode_two(s: &mut dyn Session, first: &str, second: &str) {
    explode(s, first, Some(ShapeEnum::Face));
    explode(s, second, Some(ShapeEnum::Face));
}

pub fn bug29807_i1001(s: &mut dyn Session) {
    banner();
    restore(s, "bug29807-obj.brep", "b1");
    restore(s, "bug29807-tool.brep", "b2");
    explode_two(s, "b1", "b2");
    let expected = Reached::Equal("0.00015253053837904724");
    checked(s, ["b1_5", "b2_2"], expected);
}

pub fn bug29807_i1002(s: &mut dyn Session) {
    banner();
    restore(s, "bug29807-obj.brep", "b1");
    restore(s, "bug29807-tool.brep", "b2");
    turned(s, "b2", 5.);
    explode_two(s, "b1", "b2");
    checked(s, ["b1_5", "b2_2"], Reached::AtMost(0.002));
}

pub fn bug29807_i1003(s: &mut dyn Session) {
    banner();
    restore(s, "bug29807-obj.brep", "b1");
    restore(s, "bug29807-tool.brep", "b2");
    turned(s, "b2", -5.);
    explode_two(s, "b1", "b2");
    checked(s, ["b1_5", "b2_2"], Reached::AtMost(5.1e-5));
}

pub fn bug29807_i1004(s: &mut dyn Session) {
    banner();
    restore(s, "bug29807-obj.brep", "b1");
    restore(s, "bug29807-tool.brep", "b2");
    turned_twice(s, "b2", [-5., 20.], 0.3);
    explode_two(s, "b1", "b2");
    checked(s, ["b1_5", "b2_2"], Reached::AtMost(0.000113));
}

pub fn bug29807_i1005(s: &mut dyn Session) {
    banner();
    restore(s, "bug29807-obj.brep", "b1");
    restore(s, "bug29807-tool.brep", "b2");
    turned_twice(s, "b2", [5., -38.], 2.);
    explode_two(s, "b1", "b2");
    checked(s, ["b1_5", "b2_2"], Reached::AtMost(8e-7));
}

pub fn bug29807_i1006(s: &mut dyn Session) {
    banner();
    restore(s, "bug25542_cylinder.brep", "b1");
    restore(s, "bug25542_cone.brep", "b2");
    explode_two(s, "b1", "b2");
    let expected = Reached::Equal("0.0016015772839744358");
    checked(s, ["b1_1", "b2_1"], expected);
}

fn nurbs_of(s: &mut dyn Session, face: &str) {
    nurbsconvert(s, "f1", face);
}

pub fn bug29807_i2001(s: &mut dyn Session) {
    banner();
    restore(s, "bug29807-obj.brep", "b1");
    restore(s, "bug29807-tool.brep", "b2");
    explode_two(s, "b1", "b2");
    nurbs_of(s, "b1_5");
    let expected = Reached::Equal("2.693336906196208e-008");
    checked(s, ["f1", "b2_2"], expected);
}

pub fn bug29807_i2002(s: &mut dyn Session) {
    banner();
    restore(s, "bug29807-obj.brep", "b1");
    restore(s, "bug29807-tool.brep", "b2");
    turned(s, "b2", 5.);
    explode_two(s, "b1", "b2");
    nurbs_of(s, "b1_5");
    let expected = Reached::Equal("2.9119212307774807e-007");
    checked(s, ["f1", "b2_2"], expected);
}

pub fn bug29807_i2003(s: &mut dyn Session) {
    banner();
    restore(s, "bug29807-obj.brep", "b1");
    restore(s, "bug29807-tool.brep", "b2");
    turned(s, "b2", -5.);
    explode_two(s, "b1", "b2");
    nurbs_of(s, "b1_5");
    let expected = Reached::Equal("2.412545178522274e-008");
    checked(s, ["f1", "b2_2"], expected);
}

pub fn bug29807_i2004(s: &mut dyn Session) {
    banner();
    restore(s, "bug29807-obj.brep", "b1");
    restore(s, "bug29807-tool.brep", "b2");
    turned_twice(s, "b2", [-5., 20.], 0.3);
    explode_two(s, "b1", "b2");
    nurbs_of(s, "b1_5");
    let expected = Reached::Equal("2.6039506888710934e-007");
    checked(s, ["f1", "b2_2"], expected);
}

pub fn bug29807_i2005(s: &mut dyn Session) {
    banner();
    restore(s, "bug29807-obj.brep", "b1");
    restore(s, "bug29807-tool.brep", "b2");
    turned_twice(s, "b2", [5., -38.], 2.);
    explode_two(s, "b1", "b2");
    nurbs_of(s, "b1_5");
    let expected = Reached::Equal("6.4774617011651419e-006");
    checked(s, ["f1", "b2_2"], expected);
}

pub fn bug29807_i2006(s: &mut dyn Session) {
    banner();
    restore(s, "bug25542_cylinder.brep", "b1");
    restore(s, "bug25542_cone.brep", "b2");
    explode_two(s, "b1", "b2");
    nurbs_of(s, "b1_1");
    let expected = Reached::Equal("6.4791886898294872e-006");
    checked(s, ["f1", "b2_1"], expected);
}

pub fn bug29807_i3001(s: &mut dyn Session) {
    banner();
    restore(s, "bug29807-obj.brep", "b1");
    restore(s, "bug29807-cone.brep", "f2");
    explode(s, "b1", Some(ShapeEnum::Face));
    let expected = Reached::Equal("0.00015253053837762444");
    checked(s, ["b1_5", "f2"], expected);
}

pub fn bug29807_i3002(s: &mut dyn Session) {
    banner();
    restore(s, "bug29807-obj.brep", "b1");
    restore(s, "bug29807-cone.brep", "f2");
    turned(s, "f2", 5.);
    explode(s, "b1", Some(ShapeEnum::Face));
    checked(s, ["b1_5", "f2"], Reached::AtMost(0.0004));
}

pub fn bug29807_i3003(s: &mut dyn Session) {
    banner();
    restore(s, "bug29807-obj.brep", "b1");
    restore(s, "bug29807-cone.brep", "f2");
    turned(s, "f2", -5.);
    explode(s, "b1", Some(ShapeEnum::Face));
    let expected = Reached::Equal("5.4742962483090032e-005");
    checked(s, ["b1_5", "f2"], expected);
}

pub fn bug29807_i3004(s: &mut dyn Session) {
    banner();
    restore(s, "bug29807-obj.brep", "b1");
    restore(s, "bug29807-cone.brep", "f2");
    turned_twice(s, "f2", [-5., 20.], 0.3);
    explode(s, "b1", Some(ShapeEnum::Face));
    checked(s, ["b1_5", "f2"], Reached::AtMost(0.000113));
}

pub fn bug29807_i3005(s: &mut dyn Session) {
    banner();
    restore(s, "bug29807-obj.brep", "b1");
    restore(s, "bug29807-cone.brep", "f2");
    turned_twice(s, "f2", [5., -38.], 2.);
    explode(s, "b1", Some(ShapeEnum::Face));
    checked(s, ["b1_5", "f2"], Reached::AtMost(8e-7));
}

pub fn bug29807_i4001(s: &mut dyn Session) {
    banner();
    restore(s, "bug29807-obj.brep", "b1");
    restore(s, "bug29807-cone.brep", "f2");
    explode(s, "b1", Some(ShapeEnum::Face));
    nurbs_of(s, "b1_5");
    let expected = Reached::Equal("2.6933365231971514e-008");
    checked(s, ["f1", "f2"], expected);
}

pub fn bug29807_i4002(s: &mut dyn Session) {
    banner();
    restore(s, "bug29807-obj.brep", "b1");
    restore(s, "bug29807-cone.brep", "f2");
    turned(s, "f2", 5.);
    explode(s, "b1", Some(ShapeEnum::Face));
    nurbs_of(s, "b1_5");
    let expected = Reached::Equal("2.9119209602049977e-007");
    checked(s, ["f1", "f2"], expected);
}

pub fn bug29807_i4003(s: &mut dyn Session) {
    banner();
    restore(s, "bug29807-obj.brep", "b1");
    restore(s, "bug29807-cone.brep", "f2");
    turned(s, "f2", -5.);
    explode(s, "b1", Some(ShapeEnum::Face));
    nurbs_of(s, "b1_5");
    let expected = Reached::Equal("2.4125458576041928e-008");
    checked(s, ["f1", "f2"], expected);
}

pub fn bug29807_i4004(s: &mut dyn Session) {
    banner();
    restore(s, "bug29807-obj.brep", "b1");
    restore(s, "bug29807-cone.brep", "f2");
    turned_twice(s, "f2", [-5., 20.], 0.3);
    explode(s, "b1", Some(ShapeEnum::Face));
    nurbs_of(s, "b1_5");
    let expected = Reached::Equal("2.6068687218615023e-007");
    checked(s, ["f1", "f2"], expected);
}

pub fn bug29807_i4005(s: &mut dyn Session) {
    banner();
    restore(s, "bug29807-obj.brep", "b1");
    restore(s, "bug29807-cone.brep", "f2");
    turned_twice(s, "f2", [5., -38.], 2.);
    explode(s, "b1", Some(ShapeEnum::Face));
    nurbs_of(s, "b1_5");
    let expected = Reached::Equal("6.4774618953705733e-006");
    checked(s, ["f1", "f2"], expected);
}
