#[test]
fn math_root_brent_inverse_quadratic_interpolation_converges_on_asymmetric_cubic() {
    let mut aFunc: CubicFunc = CubicFunc::default();
    let mut aConfig: Config = Config::default();
    aConfig.x_tolerance = 1.0e-14;
    aConfig.f_tolerance = 0.0;
    aConfig.max_iterations = 100;

    let aResult: ScalarResult = brent(&mut aFunc, 1.0, 2.0, &aConfig);
    assert!(aResult.is_done());
    assert!(aResult.root.is_some());
    assert!(aResult.iterations < 12);
    near(aResult.root.unwrap(), 1.5213797068045676, 1.0e-14);
}

#[test]
fn math_root_brent_zero_function_tolerance_converges_positionally() {
    let mut aFunc: SqrtTwoFunc = SqrtTwoFunc::default();
    let mut aConfig: Config = Config::default();
    aConfig.x_tolerance = 1.0e-14;
    aConfig.f_tolerance = 0.0;
    aConfig.max_iterations = 100;

    let aResult: ScalarResult = brent(&mut aFunc, 1.0, 2.0, &aConfig);
    assert!(aResult.is_done());
    assert!(aResult.root.is_some());
    near(aResult.root.unwrap(), (2.0_f64).sqrt(), 2.0e-14);
}

#[test]
fn math_root_brent_callback_refusal_during_iteration_is_propagated() {
    let mut aFunc = FailingFunc::default();
    let aResult = brent(&mut aFunc, 1.0, 2.0, &Config::default());
    assert_eq!(aResult.status, Status::CallbackError);
    assert_eq!(aResult.iterations, 1);
    assert!(aResult.root.is_none());
    assert!(aResult.value.is_none());
}

#[test]
fn math_root_bisection_sqrt_two() {
    let mut aFunc: SqrtTwoFunc = SqrtTwoFunc::default();
    let aResult: ScalarResult = bisection(&mut aFunc, 1.0, 2.0, &Config::default());
    assert!(aResult.is_done());
    near(aResult.root.unwrap(), (2.0_f64).sqrt(), THE_TOLERANCE);
}

#[test]
fn math_root_bisection_cos_minus_x() {
    let mut aFunc: CosMinusXFunc = CosMinusXFunc::default();
    let aResult: ScalarResult = bisection(&mut aFunc, 0.0, 1.0, &Config::default());
    assert!(aResult.is_done());
    let aFx: f64 = (aResult.root.unwrap()).cos() - aResult.root.unwrap();
    near(aFx, 0.0, THE_TOLERANCE);
}

#[test]
fn math_root_bisection_sin_pi() {
    let mut aFunc: SinFunc = SinFunc::default();
    let aResult: ScalarResult = bisection(&mut aFunc, 2.0, 4.0, &Config::default());
    assert!(aResult.is_done());
    near(aResult.root.unwrap(), std::f64::consts::PI, THE_TOLERANCE);
}

#[test]
fn math_root_bisection_invalid_bracket() {
    let mut aFunc: SqrtTwoFunc = SqrtTwoFunc::default();

    let aResult: ScalarResult = bisection(&mut aFunc, 2.0, 3.0, &Config::default());
    assert_eq!(aResult.status, Status::InvalidInput);
}

#[test]
fn math_root_bisection_newton_sqrt_two() {
    let mut aFunc: SqrtTwoFunc = SqrtTwoFunc::default();
    let aResult: ScalarResult = bisection_newton(&mut aFunc, 1.0, 2.0, &Config::default());
    assert!(aResult.is_done());
    near(aResult.root.unwrap(), (2.0_f64).sqrt(), THE_TOLERANCE);
}
