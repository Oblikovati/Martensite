#[test]
fn math_integ_new_error_estimation_tanh_sinh() {
    let mut aFunc: SineFunc = SineFunc;

    let aResult: IntegResult = tanh_sinh(
        &mut aFunc,
        0.0,
        std::f64::consts::PI,
        &DoubleExpConfig::default(),
    );

    assert!(aResult.is_done());
}

#[test]
fn math_integ_kronrod_new_infinite_interval() {
    let mut aFunc: InfiniteGaussianFunc = InfiniteGaussianFunc;

    let aResult: IntegResult = sinh_sinh(&mut aFunc, &DoubleExpConfig::default());

    assert!(aResult.is_done());
    near(
        aResult.value.unwrap(),
        (std::f64::consts::PI).sqrt(),
        1.0e-4,
    );
}

#[test]
fn math_integ_kronrod_new_semi_infinite_interval() {
    let mut aFunc: ExponentialDecayFunc = ExponentialDecayFunc;

    let aResult: IntegResult = exp_sinh(&mut aFunc, 0.0, &DoubleExpConfig::default());

    assert!(aResult.is_done());
    near(aResult.value.unwrap(), 1.0, 1.0e-4);
}
