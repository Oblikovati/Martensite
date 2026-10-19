#[test]
fn math_integ_special_sqrt_function() {
    let mut aFunc: SqrtFunc = SqrtFunc;

    let mut aResult: IntegResult = gauss_composite(&mut aFunc, 0.001, 1.0, 20, 15);
    assert!(aResult.is_done());

    let aExpected: f64 = (2.0 / 3.0) * (1.0 - martensite_cmath::pow(0.001_f64, 1.5));
    near(aResult.value.unwrap(), aExpected, 1.0e-6);
}

#[test]
fn math_integ_bool_conversion_successful_result_is_true() {
    let mut aFunc: SinFunc = SinFunc;
    let mut aResult: IntegResult = gauss(&mut aFunc, 0.0, std::f64::consts::PI, 15);
    assert!(bool::from(&aResult));
}

#[test]
fn math_integ_bool_conversion_invalid_input_is_false() {
    let mut aFunc: SinFunc = SinFunc;
    let mut aResult: IntegResult = gauss(&mut aFunc, 0.0, std::f64::consts::PI, 0);
    assert!(!(bool::from(&aResult)));
}
