#[test]
fn math_integ_double_exp_new_tanh_sinh_polynomial() {
    let mut aFunc: PolynomialFunc = PolynomialFunc;

    let aResult: IntegResult = tanh_sinh(&mut aFunc, 0.0, 1.0, &DoubleExpConfig::default());

    assert!(aResult.is_done());
    near(aResult.value.unwrap(), 1.0 / 3.0, 1.0e-6);
}

#[test]
fn math_integ_double_exp_new_tanh_sinh_sine() {
    let mut aFunc: SineFunc = SineFunc;

    let aResult: IntegResult = tanh_sinh(
        &mut aFunc,
        0.0,
        std::f64::consts::PI,
        &DoubleExpConfig::default(),
    );

    assert!(aResult.is_done());
    near(aResult.value.unwrap(), 2.0, 1.0e-6);
}

#[test]
fn math_integ_double_exp_new_tanh_sinh_sqrt_singularity() {
    let mut aFunc: SqrtSingularityFunc = SqrtSingularityFunc;

    let mut aConfig: DoubleExpConfig = DoubleExpConfig::default();
    aConfig.nb_levels = 8;

    let aResult: IntegResult = tanh_sinh(&mut aFunc, 0.0, 1.0, &aConfig);

    assert!(aResult.is_done());
    near(aResult.value.unwrap(), 2.0, 1.0e-4);
}
