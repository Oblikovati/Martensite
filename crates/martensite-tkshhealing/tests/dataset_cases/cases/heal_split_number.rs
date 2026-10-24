//! heal split number regression cases that build their inputs: a sphere band or a
//! planar triangle, exploded into faces, then `DT_SplitByNumber result face n [n2]`
//! (ShapeUpgradeShapeDivideArea splitting by number) and `checknbshapes result -t -face
//! k`; the grid ends run `puts [checkshape result]` and end shows `result`
//! with `checkview -display result -2d`.
use crate::support::{
    build::{checknbshapes_t, polyline, psphere},
    checkshape::checkshape,
    commands::explode_type,
    log,
    restore::Restored,
    session::{Session, ViewStep, checkview_display, set, set_exploded},
    swdraw::dt_split_by_number,
    topology::{mkplane, trotate},
};
use martensite_tkbrep::topods::Shape;
use martensite_tkg3d::top_abs::ShapeEnum;

/// `explode name f`: the face variables `name_1`, ... and the names the command returns.
fn explode_faces(s: &mut dyn Session, name: &str, shape: &Shape) -> Vec<Shape> {
    let faces = set_exploded(s, name, explode_type(shape, ShapeEnum::Face));
    let names: Vec<String> = (1..=faces.len()).map(|i| format!("{name}_{i}")).collect();
    log::result(&names.join(" "));
    faces
}

/// `DT_SplitByNumber result face parts [number2]`, `checknbshapes result -t -face faces`,
/// then the grid end (split_two_numbers shares these bodies).
fn split_and_check(
    s: &mut dyn Session,
    face: &Shape,
    parts: i32,
    number2: Option<i32>,
    faces: usize,
    image: &str,
) {
    let result = set(s, "result", dt_split_by_number(face, parts, number2));
    checknbshapes_t(&result, "result", &[("FACE", faces)], None, "");
    checkshape(&result);
    checkview_display(s, Some("result"), image);
}

/// `psphere a 10 0 90`, `fit`, `explode a f`, then the split of `a_1`.
pub(crate) fn hemisphere(
    s: &mut dyn Session,
    parts: i32,
    number2: Option<i32>,
    faces: usize,
    image: &str,
) {
    let a = set(s, "a", psphere(10., Some((0., 90.)), None));
    s.step(ViewStep::Fit);
    let a_faces = explode_faces(s, "a", &a);
    split_and_check(s, &a_faces[0], parts, number2, faces, image);
}

/// `psphere a 10 0 90 90`, `trotate a 0 0 0 0 0 1 -90`, `explode a f`, then the split of
/// `a_1`.
fn quarter_band(s: &mut dyn Session, parts: i32, faces: usize, image: &str) {
    let a = psphere(10., Some((0., 90.)), Some(90.));
    s.set("a", &Restored::Shape(a.clone()));
    let a = set(s, "a", trotate(&a, [0., 0., 0.], [0., 0., 1.], -90., false));
    let a_faces = explode_faces(s, "a", &a);
    split_and_check(s, &a_faces[0], parts, None, faces, image);
}

/// `psphere a 10`, `fit`, `explode a f`, then the split of `a_1`.
fn sphere(s: &mut dyn Session, parts: i32, faces: usize, image: &str) {
    let a = set(s, "a", psphere(10., None, None));
    s.step(ViewStep::Fit);
    let a_faces = explode_faces(s, "a", &a);
    split_and_check(s, &a_faces[0], parts, None, faces, image);
}

/// `polyline a 0 0 0 10 0 0 2 5 0 0 0 0`, `mkplane a a`, then the split of `a`.
fn triangle(s: &mut dyn Session, parts: i32, faces: usize, image: &str) {
    let wire = set(
        s,
        "a",
        polyline(&[[0., 0., 0.], [10., 0., 0.], [2., 5., 0.], [0., 0., 0.]]),
    );
    let a = set(
        s,
        "a",
        mkplane(&wire).unwrap_or_else(|error| panic!("{error}")),
    );
    split_and_check(s, &a, parts, None, faces, image);
}

/// split number — A1.
pub fn a1(s: &mut dyn Session) {
    hemisphere(s, 10, None, 10, "A1");
}

/// split number — A2.
pub fn a2(s: &mut dyn Session) {
    hemisphere(s, 100, None, 102, "A2");
}

/// split number — A5.
pub fn a5(s: &mut dyn Session) {
    quarter_band(s, 10, 8, "A5");
}

/// split number — A6.
pub fn a6(s: &mut dyn Session) {
    quarter_band(s, 100, 96, "A6");
}

/// split number — A7.
pub fn a7(s: &mut dyn Session) {
    sphere(s, 10, 10, "A7");
}

/// split number — A8.
pub fn a8(s: &mut dyn Session) {
    sphere(s, 100, 136, "A8");
}

/// split number — C1.
pub fn c1(s: &mut dyn Session) {
    triangle(s, 10, 14, "C1");
}

/// split number — C2.
pub fn c2(s: &mut dyn Session) {
    triangle(s, 100, 118, "C2");
}
