//! `tests/lowalgos/classifier/bug30354_1` and `_2`: a point of a face, classified by both commands.
use crate::support::{
    lowalgos_commands::{b2dclassifx, b2dclassify, pcurve},
    restore::restore,
    session::{Drawing, Session, ViewStep},
};
use martensite_tkmath::gp::Point2;

fn graded_classification(answer: &str, needle: &str) {
    if answer.contains(needle) {
        println!("OK: good result of classification");
    } else {
        println!("Error: Wrong result of classification");
    }
}

fn classified(s: &mut dyn Session, at: Point2, needle: &str) {
    println!("============");
    println!("0030354: BOP Cut doesn't modify the attached face");
    println!("============");
    println!();
    restore(s, "bug30354_Face_14.brep", "b1");
    s.bind_drawing("pp", Drawing::PlanePoint(at));
    graded_classification(b2dclassifx(s, "b1", at), needle);
    graded_classification(b2dclassify(s, "b1", at), needle);
    s.view(ViewStep::PlaneView);
    pcurve(s, "b1");
    s.view(ViewStep::Fit);
    s.view(ViewStep::Disp(&["pp"]));
}

pub fn bug30354_1(s: &mut dyn Session) {
    let at = Point2::new(0.0026084998890765357, 0.045000000000000012);
    classified(s, at, "IN");
}

pub fn bug30354_2(s: &mut dyn Session) {
    let at = Point2::new(0.0046465389982220402, 0.045000000000000005);
    classified(s, at, "ON");
}
