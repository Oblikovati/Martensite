#[test]
fn math_integ_new_kronrod_vs_gauss_polynomial() {
    let mut aFunc: PolynomialFunc = PolynomialFunc;

    let aKronrodResult: IntegResult = kronrod_rule(&mut aFunc, 0.0, 1.0, 7);
    let aGaussResult: IntegResult = gauss(&mut aFunc, 0.0, 1.0, 15);

    assert!(aKronrodResult.is_done());
    assert!(aGaussResult.is_done());

    near(
        aKronrodResult.value.unwrap(),
        aGaussResult.value.unwrap(),
        1.0e-10,
    );
}

#[test]
fn math_integ_new_kronrod_vs_gauss_sine() {
    let mut aFunc: SineFunc = SineFunc;

    let aKronrodResult: IntegResult = kronrod_rule(&mut aFunc, 0.0, std::f64::consts::PI, 7);
    let aGaussResult: IntegResult = gauss(&mut aFunc, 0.0, std::f64::consts::PI, 15);

    assert!(aKronrodResult.is_done());
    assert!(aGaussResult.is_done());

    near(
        aKronrodResult.value.unwrap(),
        aGaussResult.value.unwrap(),
        1.0e-10,
    );
}

#[test]
fn math_integ_new_tanh_sinh_vs_kronrod_polynomial() {
    let mut aFunc: PolynomialFunc = PolynomialFunc;

    let aTanhSinhResult: IntegResult = tanh_sinh(&mut aFunc, 0.0, 1.0, &DoubleExpConfig::default());
    let aKronrodResult: IntegResult = kronrod_rule(&mut aFunc, 0.0, 1.0, 7);

    assert!(aTanhSinhResult.is_done());
    assert!(aKronrodResult.is_done());

    near(
        aTanhSinhResult.value.unwrap(),
        aKronrodResult.value.unwrap(),
        1.0e-6,
    );
}
