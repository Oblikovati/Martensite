#[test]
fn math_brent_minimum_reversed_bracket_order() {
    // Test with brackets in reverse order (Cx < Ax)
    let mut aFunc = QuadraticFunction;

    let mut aSolver = BrentMinimum::new(1.0e-10);
    aSolver.perform(&mut aFunc, 4.0, 1.5, 0.0); // Reversed order

    assert!(aSolver.is_done());
    assert_near(aSolver.location().unwrap(), 2.0, 1.0e-8);
}

#[test]
fn math_brent_minimum_high_precision_requirement() {
    // Test with extremely tight tolerance
    let mut aFunc = CosineFunction;
    let mut aSolver = BrentMinimum::with_options(1.0e-15, 200, 1e-12);

    aSolver.perform(&mut aFunc, 2.5, 3.1, 4.0);

    assert!(aSolver.is_done());
    assert_near(aSolver.location().unwrap(), std::f64::consts::PI, 1.0e-8);
    assert!(aSolver.iterations().unwrap() > 10);
}

#[test]
fn math_brent_minimum_very_narrow_bracket() {
    // Test with very narrow initial bracket
    let mut aFunc = QuadraticFunction;

    let mut aSolver = BrentMinimum::new(1.0e-10);
    aSolver.perform(&mut aFunc, 1.99, 2.0, 2.01); // Very narrow around minimum

    assert!(aSolver.is_done());
    assert_near(aSolver.location().unwrap(), 2.0, 1.0e-8);
}
