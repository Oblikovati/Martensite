//! Closed-form jets at the existing exact inputs, independently enclosed by MPFR (#201).
use super::{checkPointsEqual, checkVectorsEqual, standard_frame};
use martensite_tkernel::precision::confusion;
use martensite_tkmath::{elclib, gp::{Circle3, Ellipse3, Hyperbola3, Point3, Vector3}};
#[path = "elementary_reference_words.rs"]
mod words;

fn reference_point(words: [u64; 3]) -> Point3 {
    let [x, y, z] = words.map(f64::from_bits);
    Point3::new(x, y, z)
}

fn reference_vector(words: [u64; 3]) -> Vector3 {
    let [x, y, z] = words.map(f64::from_bits);
    Vector3::new(x, y, z)
}

fn check_reference_jet<C>(curve: &C, parameter: f64, expected: [[u64; 3]; 4])
where C: elclib::ThirdDerivativeCurve<Point = Point3, Vector = Vector3> {
    let reference = reference_point(expected[0]);
    checkPointsEqual(elclib::value(parameter, curve), reference, confusion());
    let (point, first, second, third) = elclib::d3(parameter, curve);
    checkPointsEqual(point, reference, confusion());
    for (index, actual) in [first, second, third].into_iter().enumerate() {
        let reference = reference_vector(expected[index + 1]);
        checkVectorsEqual(actual, reference, confusion());
        let order = i32::try_from(index + 1).unwrap();
        checkVectorsEqual(elclib::dn(parameter, curve, order), reference, confusion());
    }
}

fn original_quarter_pi_input() -> f64 {
    let parameter = std::f64::consts::PI / 4.0;
    assert_eq!(parameter.to_bits(), 0x3fe921fb54442d18);
    parameter
}

#[test]
fn circle_jet_matches_independent_exact_quarter_pi_input() {
    let frame = standard_frame(Point3::new(0.0, 0.0, 0.0));
    let circle = Circle3::new(frame, 2.0).unwrap();
    check_reference_jet(&circle, original_quarter_pi_input(), words::CIRCLE_REFERENCE_JET);
}

#[test]
fn ellipse_jet_matches_independent_exact_quarter_pi_input() {
    let frame = standard_frame(Point3::new(0.0, 0.0, 0.0));
    let ellipse = Ellipse3::new(frame, 3.0, 2.0).unwrap();
    check_reference_jet(&ellipse, original_quarter_pi_input(), words::ELLIPSE_REFERENCE_JET);
}

#[test]
fn hyperbola_jet_matches_independent_exact_half_input() {
    let frame = standard_frame(Point3::new(0.0, 0.0, 0.0));
    let hyperbola = Hyperbola3::new(frame, 3.0, 2.0).unwrap();
    check_reference_jet(&hyperbola, 0.5, words::HYPERBOLA_REFERENCE_JET);
}
