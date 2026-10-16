#[test]
fn math_root_bisection_newton_cos_minus_x() {
    let mut aFunc: CosMinusXFunc = CosMinusXFunc::default();
    let aResult: ScalarResult = bisection_newton(&mut aFunc, 0.0, 1.0, &Config::default());
    assert!(aResult.is_done());
    let aFx: f64 = (aResult.root.unwrap()).cos() - aResult.root.unwrap();
    near(aFx, 0.0, THE_TOLERANCE);
}

#[test]
fn math_root_bisection_newton_log_linear_equation() {
    let mut aFunc: LogLinearFunc = LogLinearFunc::default();
    let mut aConfig: Config = Config::default();
    aConfig.x_tolerance = 1.0e-14;
    aConfig.f_tolerance = 1.0e-14;

    let aResult: ScalarResult = bisection_newton(&mut aFunc, 0.25, 4.0, &aConfig);
    assert_eq!(aResult.status, Status::Ok);
    assert!(aResult.root.is_some());
    assert!(aResult.value.is_some());
    near(aResult.root.unwrap(), 2.1234734225812275, 2.0e-13);
    near(aResult.value.unwrap(), 0.0, 1.0e-13);
}

#[test]
fn math_root_bisection_newton_negative_quintic_root() {
    let mut aFunc: QuinticFunc = QuinticFunc::default();
    let mut aConfig: Config = Config::default();
    aConfig.x_tolerance = 1.0e-14;
    aConfig.f_tolerance = 1.0e-14;

    let aResult: ScalarResult = bisection_newton(&mut aFunc, -2.0, -0.5, &aConfig);
    assert_eq!(aResult.status, Status::Ok);
    assert!(aResult.root.is_some());
    assert!(aResult.value.is_some());
    near(aResult.root.unwrap(), -1.1329975658850653, 2.0e-13);
    near(aResult.value.unwrap(), 0.0, 1.0e-13);
}

#[test]
fn math_root_bisection_newton_faster_than_pure_bisection() {
    let mut aFunc: SqrtTwoFunc = SqrtTwoFunc::default();
    let mut aConfig: Config = Config::default();
    aConfig.max_iterations = 100;

    let aBisec: ScalarResult = bisection(&mut aFunc, 1.0, 2.0, &aConfig);
    let aHybrid: ScalarResult = bisection_newton(&mut aFunc, 1.0, 2.0, &aConfig);

    assert!(aBisec.is_done());
    assert!(aHybrid.is_done());

    assert!(aHybrid.iterations <= aBisec.iterations);
}

#[test]
fn math_root_bisection_newton_stalled_newton_step_with_large_residual_is_not_converged() {
    let mut aFunc = Combined(|x: f64, out: &mut f64, df: &mut f64| {
        *out = x - 0.25;
        *df = 1e300;
        true
    });
    let aConfig = Config {
        x_tolerance: 1e-12,
        f_tolerance: 0.0,
        max_iterations: 1,
        ..Default::default()
    };
    let aResult = bisection_newton(&mut aFunc, 0.0, 2.0, &aConfig);
    assert!(!aResult.is_done());
    assert!(aResult.value.is_some());
    assert!(aResult.value.unwrap().abs() > aConfig.f_tolerance);
}

#[test]
fn math_root_convergence_newton_iteration_count() {
    let mut aFunc: SqrtTwoFunc = SqrtTwoFunc::default();
    let aResult: ScalarResult = newton(&mut aFunc, 1.0, &Config::default());
    assert!(aResult.is_done());

    assert!(aResult.iterations < 15);
}

#[test]
fn math_root_convergence_brent_iteration_count() {
    let mut aFunc: SqrtTwoFunc = SqrtTwoFunc::default();
    let aResult: ScalarResult = brent(&mut aFunc, 1.0, 2.0, &Config::default());
    assert!(aResult.is_done());

    assert!(aResult.iterations < 50);
}

#[test]
fn math_root_bool_conversion_successful_result_is_true() {
    let mut aFunc: SqrtTwoFunc = SqrtTwoFunc::default();
    let aResult: ScalarResult = newton(&mut aFunc, 1.0, &Config::default());
    assert!(bool::from(&aResult));
}
