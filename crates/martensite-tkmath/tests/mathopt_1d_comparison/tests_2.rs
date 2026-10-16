#[test]
fn math_opt_1_d_comparison_narrow_interval_parabola() {
    let mut anOldFunc: ParabolaFunc = ParabolaFunc::default();
    let mut aNewFunc: ParabolaFunc = ParabolaFunc::default();

    let mut anOldSolver = BrentMinimum::with_options(THE_TOLERANCE, 100, 1e-12);
    anOldSolver.perform(&mut anOldFunc, 2.9, 3.0, 3.1);

    let aNewResult: ScalarResult = brent(&mut aNewFunc, 2.9, 3.1, &Config::default());

    assert!(anOldSolver.is_done());
    assert!(aNewResult.is_done());

    near(
        anOldSolver.location().unwrap(),
        aNewResult.root.unwrap(),
        THE_TOLERANCE,
    );
    near(aNewResult.root.unwrap(), 3.0, THE_TOLERANCE);
}

#[test]
fn math_opt_1_d_comparison_golden_parabola() {
    let mut aNewFunc: ParabolaFunc = ParabolaFunc::default();

    let aBrentResult: ScalarResult = brent(&mut aNewFunc, 0.0, 10.0, &Config::default());
    let aGoldenResult: ScalarResult = golden(&mut aNewFunc, 0.0, 10.0, &Config::default());

    assert!(aBrentResult.is_done());
    assert!(aGoldenResult.is_done());

    near(
        aBrentResult.root.unwrap(),
        aGoldenResult.root.unwrap(),
        THE_TOLERANCE,
    );
    near(
        aBrentResult.value.unwrap(),
        aGoldenResult.value.unwrap(),
        THE_TOLERANCE,
    );
    near(aGoldenResult.root.unwrap(), 3.0, THE_TOLERANCE);
}

#[test]
fn math_opt_1_d_comparison_golden_cosh_like() {
    let mut aNewFunc: CoshLikeFunc = CoshLikeFunc::default();

    let aBrentResult: ScalarResult = brent(&mut aNewFunc, -5.0, 5.0, &Config::default());
    let aGoldenResult: ScalarResult = golden(&mut aNewFunc, -5.0, 5.0, &Config::default());

    assert!(aBrentResult.is_done());
    assert!(aGoldenResult.is_done());

    near(
        aBrentResult.root.unwrap(),
        aGoldenResult.root.unwrap(),
        THE_TOLERANCE,
    );
    near(
        aBrentResult.value.unwrap(),
        aGoldenResult.value.unwrap(),
        THE_TOLERANCE,
    );
    near(aGoldenResult.root.unwrap(), 0.0, THE_TOLERANCE);
}
