#[test]
fn math_integ_gauss_composite_rational_oscillation_on_finite_interval() {
    let mut aFunc: RationalOscillationFunc = RationalOscillationFunc;
    let mut aResult: IntegResult = gauss_composite(&mut aFunc, 0.2, 3.4, 64, 15);

    assert!(aResult.is_done());
    assert!(aResult.value.is_some());
    near(aResult.value.unwrap(), -0.023002601281013002, 2.0e-12);
}

#[test]
fn math_integ_kronrod_log_cosine_on_asymmetric_interval() {
    let mut aFunc: LogCosineFunc = LogCosineFunc;
    let mut aConfig: KronrodConfig = KronrodConfig::new(1.0e-12, 100);
    let mut aResult: IntegResult = kronrod(&mut aFunc, -0.7, 2.4, &aConfig);

    assert!(aResult.is_done());
    assert!(aResult.value.is_some());
    near(aResult.value.unwrap(), 1.4805767078645715, 2.0e-11);
}

#[test]
fn math_integ_double_exp_reversed_square_root_log_interval() {
    let mut aFunc: SquareRootLogFunc = SquareRootLogFunc;
    let mut aConfig: DoubleExpConfig = DoubleExpConfig::new(1.0e-11, 100);
    aConfig.nb_levels = 12;
    let mut aResult: IntegResult = tanh_sinh(&mut aFunc, 2.0, 0.0, &aConfig);

    assert!(aResult.is_done());
    assert!(aResult.value.is_some());
    near(aResult.value.unwrap(), -1.426_347_068_120_945, 2.0e-9);
}
