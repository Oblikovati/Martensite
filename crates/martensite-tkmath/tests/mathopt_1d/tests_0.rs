#[test]
fn math_opt_1_d_brent_simple_parabola() {
    let mut aFunc: ParabolaFunc = ParabolaFunc::default();
    let aResult: ScalarResult = brent(&mut aFunc, 0.0, 10.0, &Config::default());
    assert!(aResult.is_done());
    near(aResult.root.unwrap(), 3.0, THE_TOLERANCE);
    near(aResult.value.unwrap(), 1.0, THE_TOLERANCE);
}

#[test]
fn math_opt_1_d_brent_parabola_with_negative_minimum() {
    let mut aFunc: ParabolaFunc2 = ParabolaFunc2;
    let aResult: ScalarResult = brent(&mut aFunc, 0.0, 5.0, &Config::default());
    assert!(aResult.is_done());
    near(aResult.root.unwrap(), 2.0, THE_TOLERANCE);
    near(aResult.value.unwrap(), -1.0, THE_TOLERANCE);
}

#[test]
fn math_opt_1_d_brent_cosine_minimum() {
    let mut aFunc: CosFunc = CosFunc::default();
    let aResult: ScalarResult = brent(&mut aFunc, 2.0, 4.0, &Config::default());
    assert!(aResult.is_done());
    near(aResult.root.unwrap(), std::f64::consts::PI, THE_TOLERANCE);
    near(aResult.value.unwrap(), -1.0, THE_TOLERANCE);
}

#[test]
fn math_opt_1_d_brent_quartic_minimum_right() {
    let mut aFunc: QuarticFunc = QuarticFunc::default();

    let aResult: ScalarResult = brent(&mut aFunc, 0.0, 2.0, &Config::default());
    assert!(aResult.is_done());
    near(aResult.root.unwrap(), 1.0, THE_TOLERANCE);
    near(aResult.value.unwrap(), -1.0, THE_TOLERANCE);
}

#[test]
fn math_opt_1_d_brent_quartic_minimum_left() {
    let mut aFunc: QuarticFunc = QuarticFunc::default();

    let aResult: ScalarResult = brent(&mut aFunc, -2.0, 0.0, &Config::default());
    assert!(aResult.is_done());
    near(aResult.root.unwrap(), -1.0, THE_TOLERANCE);
    near(aResult.value.unwrap(), -1.0, THE_TOLERANCE);
}

#[test]
fn math_opt_1_d_brent_cosh_like_minimum() {
    let mut aFunc: CoshLikeFunc = CoshLikeFunc::default();
    let aResult: ScalarResult = brent(&mut aFunc, -5.0, 5.0, &Config::default());
    assert!(aResult.is_done());
    near(aResult.root.unwrap(), 0.0, THE_TOLERANCE);
    near(aResult.value.unwrap(), 2.0, THE_TOLERANCE);
}
