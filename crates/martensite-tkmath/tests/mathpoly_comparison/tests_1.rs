#[test]
fn math_poly_comparison_quadratic_negative_roots() {
    let aA: f64 = 1.0;
    let aB: f64 = 5.0;
    let aC: f64 = 6.0;

    let anOldSolver = DirectPolynomialRoots::quadratic(aA, aB, aC);
    let aNewResult = quadratic(aA, aB, aC);

    assert!(anOldSolver.is_done());
    assert!(aNewResult.is_done());

    compare_roots(
        old_roots(&anOldSolver),
        new_roots(&aNewResult),
        THE_TOLERANCE,
    );
}

#[test]
fn math_poly_comparison_quadratic_large_coefficients() {
    let aA: f64 = 1000.0;
    let aB: f64 = -3000.0;
    let aC: f64 = 2000.0;

    let anOldSolver = DirectPolynomialRoots::quadratic(aA, aB, aC);
    let aNewResult = quadratic(aA, aB, aC);

    assert!(anOldSolver.is_done());
    assert!(aNewResult.is_done());

    compare_roots(
        old_roots(&anOldSolver),
        new_roots(&aNewResult),
        THE_TOLERANCE,
    );
}

#[test]
fn math_poly_comparison_cubic_three_distinct_roots() {
    let aA: f64 = 1.0;
    let aB: f64 = -6.0;
    let aC: f64 = 11.0;
    let aD: f64 = -6.0;

    let anOldSolver = DirectPolynomialRoots::cubic(aA, aB, aC, aD);
    let aNewResult = cubic(aA, aB, aC, aD);

    assert!(anOldSolver.is_done());
    assert!(aNewResult.is_done());

    compare_roots(
        old_roots(&anOldSolver),
        new_roots(&aNewResult),
        THE_TOLERANCE,
    );
}
