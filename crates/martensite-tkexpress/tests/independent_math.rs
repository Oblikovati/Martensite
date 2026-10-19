//! Exact polynomial identities and closed-form function values independent of snapshots.
use martensite_tkernel::{standard::Handle, tcollection::AsciiString};
use martensite_tkexpress::{
    expr::{ExprHandle, ExprNamedUnknown, handles::one_based},
    expr_intrp::ExprIntrpGenExp,
};

fn parsed_expression(text: &str) -> ExprHandle {
    let interpreter = ExprIntrpGenExp::create();
    interpreter.set_scanner_output(Box::new(std::io::sink()));
    interpreter.process(&AsciiString::new(text));
    assert!(interpreter.is_done(), "expected valid expression {text:?}");
    interpreter.expression().unwrap()
}

fn evaluate_at(expression: &ExprHandle, variable: &Handle<ExprNamedUnknown>, x: f64) -> f64 {
    expression
        .evaluate(&one_based(vec![variable.clone()]), &one_based(vec![x]))
        .unwrap()
}

fn polynomial_derivative_values(x: f64) -> [f64; 6] {
    [
        x * x * x * x + 3. * x * x - 2. * x + 7.,
        4. * x * x * x + 6. * x - 2.,
        12. * x * x + 6.,
        24. * x,
        24.,
        0.,
    ]
}

#[test]
fn polynomial_evaluation_and_five_derivatives_have_exact_dyadic_values() {
    let expression = parsed_expression("x^4+3*x^2-2*x+7");
    let variable = ExprNamedUnknown::new(&AsciiString::new("x"));
    for numerator in -12..=12 {
        let x = f64::from(numerator) / 4.;
        let expected = polynomial_derivative_values(x);
        assert_eq!(evaluate_at(&expression, &variable, x), expected[0]);
        for (degree, value) in expected.iter().enumerate().skip(1) {
            let derivative = expression.n_derivative(&variable, degree as i32).unwrap();
            assert_eq!(
                evaluate_at(&derivative, &variable, x),
                *value,
                "degree {degree}, x={x}"
            );
        }
    }
}

const ELEMENTARY_CASES: &[(&str, f64, f64, f64)] = &[
    ("Sin(x)", 0., 0., 1.),
    ("Cos(x)", 0., 1., 0.),
    ("Exp(x)", 0., 1., 1.),
    ("Ln(x)", 1., 0., 1.),
    ("Sqrt(x)", 4., 2., 0.25),
    ("ATan(x)", 0., 0., 1.),
    ("Sinh(x)", 0., 0., 1.),
    ("Cosh(x)", 0., 1., 0.),
];

#[test]
fn elementary_function_values_and_first_derivatives_have_closed_forms() {
    let variable = ExprNamedUnknown::new(&AsciiString::new("x"));
    for &(text, x, value, slope) in ELEMENTARY_CASES {
        let expression = parsed_expression(text);
        assert_eq!(evaluate_at(&expression, &variable, x), value, "{text}");
        let derivative = expression.derivative(&variable).unwrap();
        assert_eq!(evaluate_at(&derivative, &variable, x), slope, "{text}");
    }
}

#[test]
fn sums_and_products_have_independent_integer_results() {
    for (text, expected) in [
        ("Sum[x^2, x, 1, 1, 4]", 30.),
        ("Prod[x+1, x, 1, 1, 3]", 24.),
        ("Sum[x, x, 0, 0.5, 3]", 1.5),
    ] {
        assert_eq!(
            parsed_expression(text).evaluate_numeric().unwrap(),
            expected
        );
    }
}

#[test]
fn interpreter_assignments_are_isolated_between_instances() {
    let assigned = ExprIntrpGenExp::create();
    let unassigned = ExprIntrpGenExp::create();
    assigned.process(&AsciiString::new("x <- 7"));
    assigned.process(&AsciiString::new("x+1"));
    unassigned.process(&AsciiString::new("x+1"));
    let simplified = assigned.expression().unwrap().simplified().unwrap();
    assert_eq!(simplified.evaluate_numeric().unwrap(), 8.);
    let failure = unassigned
        .expression()
        .unwrap()
        .evaluate_numeric()
        .unwrap_err();
    assert_eq!(failure.exception_type(), "Expr_NotEvaluable");
}
