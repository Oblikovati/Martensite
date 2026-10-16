#[test]
fn math_sys_comparison_newton_non_finite_callback_is_numerical_error() {
    let mut aSystem: FailingSystem = FailingSystem;
    let mut aStart: Vector = Vector::filled(1, 2, 0.0).unwrap();
    let mut aResult: SystemResult = newton_uniform(&mut aSystem, &aStart, 1.0e-12, 1.0e-12, 100);
    assert_eq!(aResult.status, Status::NumericalError);
    assert!(!(aResult.is_done()));
}

#[test]
fn math_sys_comparison_newton_callback_false_is_callback_error() {
    let mut aSystem: CallbackFailingSystem = CallbackFailingSystem;
    let mut aStart: Vector = Vector::filled(1, 2, 0.0).unwrap();
    let mut aResult: SystemResult = newton_uniform(&mut aSystem, &aStart, 1.0e-12, 1.0e-12, 100);
    assert_eq!(aResult.status, Status::CallbackError);
}
