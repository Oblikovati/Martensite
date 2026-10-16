#[test]
fn math_opt_glob_opt_multi_start_evaluates_and_solves_inside_bounds() {
    let mut aFunc: OutsideMinimumFunc = OutsideMinimumFunc::default();
    let mut aLower: Vector = Vector::filled(0, 1 - 1, 0.0).unwrap();
    let mut anUpper: Vector = Vector::filled(0, 1 - 1, 1.0).unwrap();
    let mut aConfig: GlobalConfig = GlobalConfig::new(GlobalStrategy::MultiStart, 100);
    aConfig.starts = 10;

    let aResult: VectorResult = multi_start(&mut aFunc, &aLower, &anUpper, &aConfig);

    assert!(aResult.is_done());
    assert!(aResult.solution.is_some());
    assert_eq!(aFunc.NbOutsideCalls, 0);
    near(
        *aResult.solution.as_ref().unwrap().at(0).unwrap(),
        0.25,
        1.0e-4,
    );
    near(aResult.value.unwrap(), 0.0, 1.0e-8);
}
