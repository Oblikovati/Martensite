#[test]
fn math_integ_gauss_constant_function() {
    let mut aFunc: ConstantFunc = ConstantFunc;

    let mut aResult: IntegResult = gauss(&mut aFunc, 0.0, 2.0, 3);
    assert!(aResult.is_done());
    near(aResult.value.unwrap(), 10.0, THE_TOLERANCE);
}

#[test]
fn math_integ_gauss_linear_function() {
    let mut aFunc: LinearFunc = LinearFunc;

    let mut aResult: IntegResult = gauss(&mut aFunc, 0.0, 3.0, 3);
    assert!(aResult.is_done());
    near(aResult.value.unwrap(), 12.0, THE_TOLERANCE);
}

#[test]
fn math_integ_gauss_quadratic_function() {
    let mut aFunc: QuadraticFunc = QuadraticFunc;

    let mut aResult: IntegResult = gauss(&mut aFunc, 0.0, 1.0, 3);
    assert!(aResult.is_done());
    near(aResult.value.unwrap(), 1.0 / 3.0, THE_TOLERANCE);
}
