#[test]
fn math_opt_1_d_bool_conversion_successful_result_is_true() {
    let mut aFunc: ParabolaFunc = ParabolaFunc::default();
    let aResult: ScalarResult = brent(&mut aFunc, 0.0, 10.0, &Config::default());
    assert!(bool::from(&aResult));
}

#[test]
fn math_opt_1_d_iteration_brent_converges_quickly() {
    let mut aFunc: ParabolaFunc = ParabolaFunc::default();
    let aResult: ScalarResult = brent(&mut aFunc, 0.0, 10.0, &Config::default());
    assert!(aResult.is_done());
    assert!(aResult.iterations < 50);
}

#[test]
fn math_opt_1_d_iteration_golden_converges() {
    let mut aFunc: ParabolaFunc = ParabolaFunc::default();
    let aResult: ScalarResult = golden(&mut aFunc, 0.0, 10.0, &Config::default());
    assert!(aResult.is_done());
    assert!(aResult.iterations < 100);
}
