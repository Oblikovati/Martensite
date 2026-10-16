#[test]
fn math_opt_1_d_brent_narrow_interval() {
    let mut aFunc: ParabolaFunc = ParabolaFunc::default();
    let aResult: ScalarResult = brent(&mut aFunc, 2.9, 3.1, &Config::default());
    assert!(aResult.is_done());
    near(aResult.root.unwrap(), 3.0, THE_TOLERANCE);
}

#[test]
fn math_opt_1_d_brent_wide_interval() {
    let mut aFunc: ParabolaFunc = ParabolaFunc::default();
    let aResult: ScalarResult = brent(&mut aFunc, -100.0, 100.0, &Config::default());
    assert!(aResult.is_done());
    near(aResult.root.unwrap(), 3.0, THE_TOLERANCE);
}

#[test]
fn math_opt_1_d_brent_custom_tolerance() {
    let mut aFunc: ParabolaFunc = ParabolaFunc::default();
    let mut aConfig: Config = Config::default();
    aConfig.x_tolerance = 1.0e-10;
    aConfig.max_iterations = 200;

    let aResult: ScalarResult = brent(&mut aFunc, 0.0, 10.0, &aConfig);
    assert!(aResult.is_done());
    near(aResult.root.unwrap(), 3.0, 1.0e-9);
}

#[test]
fn math_opt_1_d_brent_non_smooth_function() {
    let mut aFunc: AbsShiftedFunc = AbsShiftedFunc::default();
    let aResult: ScalarResult = brent(&mut aFunc, 0.0, 5.0, &Config::default());
    assert!(aResult.is_done());
    near(aResult.root.unwrap(), 2.0, 1.0e-6);
    near(aResult.value.unwrap(), 1.0, 1.0e-6);
}

#[test]
fn math_opt_1_d_golden_simple_parabola() {
    let mut aFunc: ParabolaFunc = ParabolaFunc::default();
    let aResult: ScalarResult = golden(&mut aFunc, 0.0, 10.0, &Config::default());
    assert!(aResult.is_done());
    near(aResult.root.unwrap(), 3.0, THE_TOLERANCE);
    near(aResult.value.unwrap(), 1.0, THE_TOLERANCE);
}

#[test]
fn math_opt_1_d_golden_cosine_minimum() {
    let mut aFunc: CosFunc = CosFunc::default();
    let aResult: ScalarResult = golden(&mut aFunc, 2.0, 4.0, &Config::default());
    assert!(aResult.is_done());
    near(aResult.root.unwrap(), std::f64::consts::PI, THE_TOLERANCE);
    near(aResult.value.unwrap(), -1.0, THE_TOLERANCE);
}
