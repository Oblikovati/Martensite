#[test]
fn math_root_newton_bounded_sqrt_two_with_bounds() {
    let mut aFunc: SqrtTwoFunc = SqrtTwoFunc::default();
    let aResult: ScalarResult = newton_bounded(&mut aFunc, 1.5, 1.0, 2.0, &Config::default());
    assert!(aResult.is_done());
    near(aResult.root.unwrap(), (2.0_f64).sqrt(), THE_TOLERANCE);
}

#[test]
fn math_root_newton_bounded_sin_with_bounds() {
    let mut aFunc: SinFunc = SinFunc::default();
    let aResult: ScalarResult = newton_bounded(&mut aFunc, 3.0, 2.0, 4.0, &Config::default());
    assert!(aResult.is_done());
    near(aResult.root.unwrap(), std::f64::consts::PI, THE_TOLERANCE);
}

#[test]
fn math_root_secant_sqrt_two() {
    let mut aFunc: SqrtTwoFunc = SqrtTwoFunc::default();
    let aResult: ScalarResult = secant(&mut aFunc, 1.0, 2.0, &Config::default());
    assert!(aResult.is_done());
    near(aResult.root.unwrap(), (2.0_f64).sqrt(), THE_TOLERANCE);
}

#[test]
fn math_root_secant_cos_minus_x() {
    let mut aFunc: CosMinusXFunc = CosMinusXFunc::default();
    let aResult: ScalarResult = secant(&mut aFunc, 0.0, 1.0, &Config::default());
    assert!(aResult.is_done());
    let aFx: f64 = martensite_cmath::cos(aResult.root.unwrap()) - aResult.root.unwrap();
    near(aFx, 0.0, THE_TOLERANCE);
}

#[test]
fn math_root_secant_exp_function() {
    let mut aFunc: ExpMinusThreeFunc = ExpMinusThreeFunc::default();
    let aResult: ScalarResult = secant(&mut aFunc, 0.0, 2.0, &Config::default());
    assert!(aResult.is_done());
    near(aResult.root.unwrap(), martensite_cmath::log(3.0_f64), THE_TOLERANCE);
}

#[test]
fn math_root_brent_sqrt_two() {
    let mut aFunc: SqrtTwoFunc = SqrtTwoFunc::default();
    let aResult: ScalarResult = brent(&mut aFunc, 1.0, 2.0, &Config::default());
    assert!(aResult.is_done());
    near(aResult.root.unwrap(), (2.0_f64).sqrt(), THE_TOLERANCE);
}

#[test]
fn math_root_brent_cos_minus_x() {
    let mut aFunc: CosMinusXFunc = CosMinusXFunc::default();
    let aResult: ScalarResult = brent(&mut aFunc, 0.0, 1.0, &Config::default());
    assert!(aResult.is_done());
    let aFx: f64 = martensite_cmath::cos(aResult.root.unwrap()) - aResult.root.unwrap();
    near(aFx, 0.0, THE_TOLERANCE);
}

#[test]
fn math_root_brent_cos_minus_x_high_accuracy() {
    let mut aFunc: CosMinusXFunc = CosMinusXFunc::default();
    let mut aConfig: Config = Config::default();
    aConfig.x_tolerance = 1.0e-14;
    aConfig.f_tolerance = 1.0e-14;
    let aResult: ScalarResult = brent(&mut aFunc, 0.0, 1.0, &aConfig);

    assert!(aResult.is_done());
    assert!(aResult.root.is_some());
    near(aResult.root.unwrap(), 0.73908513321516045, 1.0e-13);
}
