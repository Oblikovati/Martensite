//! `checkreal` and `checkmaxtol -ref`: the reference texts stay as the script writes them,
//! because the check's message prints them back.
use super::session::Session;
use martensite_tkernel::storage::format_general;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkshhealing::shape_analysis::ShapeAnalysisShapeTolerance;

/// The bounds of `checkmaxtol -ref`: absolute and relative.
const MAX_TOLERANCE_BOUNDS: (f64, f64) = (0.0001, 0.01);

fn parsed(what: &str, text: &str) -> f64 {
    text.parse()
        .unwrap_or_else(|cause| panic!("{what} {text:?}: expected a number: {cause}"))
}

/// `checkreal name value expected abs rel`; `checkreal("area", "2", "2.0", 1e-3, 0.)`.
pub fn checkreal(name: &str, value: &str, expected: &str, tol_abs: f64, tol_rel: f64) {
    let (actual, wanted) = (parsed(name, value), parsed(name, expected));
    if (actual - wanted).abs() > tol_abs + tol_rel * wanted.abs() {
        println!("Error: {name} = {value} is not equal to expected {expected}");
    } else {
        println!("Check of {name} OK: value = {value}, expected = {expected}");
    }
}

/// The digits a tolerance prints with; a stream writes doubles in full.
const TOLERANCE_DIGITS: usize = 17;
/// The rows of `tolerance name`, in the command's order.
const TOLERANCE_ROWS: [(&str, ShapeEnum); 4] = [
    ("Tolerance MAX=", ShapeEnum::Shape),
    ("FACE    : MAX=", ShapeEnum::Face),
    ("EDGE    : MAX=", ShapeEnum::Edge),
    ("VERTEX  : MAX=", ShapeEnum::Vertex),
];

fn tolerance_text(value: f64) -> String {
    format_general(value, TOLERANCE_DIGITS)
}

/// `tolerance name`: writes its rows and returns the largest of the first, as it prints.
/// Example: `tolerance_report(s, "result")`.
pub fn tolerance_report(s: &dyn Session, name: &str) -> String {
    let shape = s
        .shape(name)
        .unwrap_or_else(|| panic!("tolerance {name}: expected a bound shape"));
    let mut largest = String::new();
    for (title, kind) in TOLERANCE_ROWS {
        let mut tolerance = ShapeAnalysisShapeTolerance::new();
        tolerance.init_tolerance();
        tolerance
            .add_tolerance(shape, kind)
            .unwrap_or_else(|failure| panic!("tolerance {name} {kind:?}: {failure:?}"));
        let [max, avg, min] =
            [1, 0, -1].map(|mode| tolerance_text(tolerance.global_tolerance(mode)));
        println!("{title}{max} AVG={avg} MIN={min}");
        if kind == ShapeEnum::Shape {
            largest = max;
        }
    }
    largest
}

/// `checkmaxtol name -ref reference`; `checkmaxtol_ref(s, "result", "5.21731e-007")`.
pub fn checkmaxtol_ref(s: &dyn Session, name: &str, reference: &str) {
    let (tol_abs, tol_rel) = MAX_TOLERANCE_BOUNDS;
    let value = tolerance_report(s, name);
    checkreal("Max tolerance", &value, reference, tol_abs, tol_rel);
}

#[test]
fn checkreal_keeps_both_texts_and_applies_both_bounds() {
    checkreal("a", "1.5", "1.50", 0., 0.);
    checkreal("b", "1.5", "2", 0.25, 0.25);
}
