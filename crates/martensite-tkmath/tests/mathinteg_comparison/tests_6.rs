#[test]
fn math_integ_comparison_polynomial_exactness_order7() {
    let mut aFunc: Poly7Func = Poly7Func;

    let aResult: IntegResult = gauss(&mut aFunc, 0.0, 1.0, 7_usize);

    assert!(aResult.is_done());
    near(aResult.value.unwrap(), 1.0 / 8.0, THE_TOLERANCE);
}

#[test]
fn math_integ_comparison_large_interval() {
    let mut anOldFunc: ExpFuncOld = ExpFuncOld;
    let mut aNewFunc: ExpFuncNew = ExpFuncNew;

    let anOrder: i32 = 21;

    let anOldInteg: GaussSingleIntegration =
        GaussSingleIntegration::new(&mut anOldFunc, 0.0, 10.0, anOrder);

    let aNewResult: IntegResult = gauss(&mut aNewFunc, 0.0, 10.0, anOrder as usize);

    assert!(anOldInteg.is_done());
    assert!(aNewResult.is_done());

    near(
        anOldInteg.value().unwrap(),
        aNewResult.value.unwrap(),
        1.0e-6,
    );
    near(
        aNewResult.value.unwrap(),
        martensite_cmath::exp(10.0_f64) - 1.0,
        1.0e-6,
    );
}

#[test]
fn math_integ_comparison_small_interval() {
    let mut anOldFunc: SinFuncOld = SinFuncOld;
    let mut aNewFunc: SinFuncNew = SinFuncNew;

    let anOrder: i32 = 15;

    let anOldInteg: GaussSingleIntegration =
        GaussSingleIntegration::new(&mut anOldFunc, 0.0, 0.001, anOrder);

    let aNewResult: IntegResult = gauss(&mut aNewFunc, 0.0, 0.001, anOrder as usize);

    assert!(anOldInteg.is_done());
    assert!(aNewResult.is_done());

    near(
        anOldInteg.value().unwrap(),
        aNewResult.value.unwrap(),
        THE_TOLERANCE,
    );

    near(aNewResult.value.unwrap(), 0.0000005, 1.0e-12);
}
