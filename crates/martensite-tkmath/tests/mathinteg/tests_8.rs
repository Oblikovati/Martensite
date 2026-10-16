#[test]
fn math_integ_gauss_composite_gaussian_function() {
    let mut aFunc: GaussianFunc = GaussianFunc;

    let mut aResult: IntegResult = gauss_composite(&mut aFunc, -3.0, 3.0, 50, 15);
    assert!(aResult.is_done());

    near(
        aResult.value.unwrap(),
        (std::f64::consts::PI).sqrt(),
        1.0e-4,
    );
}

#[test]
fn math_integ_gauss_composite_invalid_subintervals() {
    let mut aFunc: SinFunc = SinFunc;
    let mut aResult: IntegResult = gauss_composite(&mut aFunc, 0.0, std::f64::consts::PI, 0, 7);
    assert_eq!(aResult.status, Status::InvalidInput);
}

#[test]
fn math_integ_bounds_reversed_bounds() {
    let mut aFunc: SinFunc = SinFunc;
    let mut aResult: IntegResult = gauss(&mut aFunc, std::f64::consts::PI, 0.0, 15);
    assert!(aResult.is_done());
    near(aResult.value.unwrap(), -2.0, THE_TOLERANCE);
}
