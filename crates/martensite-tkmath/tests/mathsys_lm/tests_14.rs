#[test]
fn math_sys_lm_derivative_callback_false_is_callback_error() {
    let mut aFunc: DerivativeCallbackFailure = DerivativeCallbackFailure;
    let mut aStart: Vector = Vector::filled(1, 1, 0.0).unwrap();
    let mut aResult: SystemResult = levenberg_marquardt(&mut aFunc, &aStart, &LMConfig::default());
    assert_eq!(aResult.status, Status::CallbackError);
}

#[test]
fn math_sys_lm_bounds_may_use_different_lower_indices() {
    let mut aFunc: LinearSystem2D = LinearSystem2D;
    let mut aStart: Vector = Vector::filled(4, 5, 0.0).unwrap();
    let mut aLower: Vector = Vector::filled(-3, -2, -10.0).unwrap();
    let mut anUpper: Vector = Vector::filled(8, 9, 10.0).unwrap();
    let mut aResult: SystemResult =
        levenberg_marquardt_bounded(&mut aFunc, &aStart, &aLower, &anUpper, &LMConfig::default());
    assert!(aResult.is_done());
    near(
        *aResult.solution.as_ref().unwrap().at(0).unwrap(),
        2.0,
        THE_TOLERANCE,
    );
    near(
        *aResult.solution.as_ref().unwrap().at(1).unwrap(),
        1.0,
        THE_TOLERANCE,
    );
}
