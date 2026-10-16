#[test]
fn math_root_multiple_non_finite_sample_output_is_numerical_error() {
    let mut aFunc = make_scalar(|_: f64, theY: &mut f64| {
        *theY = f64::NAN;
        true
    });
    assert_eq!(
        find_all_roots(&mut aFunc, 0.0, 1.0, &MultipleConfig::default()).status,
        Status::NumericalError
    );
}

#[test]
fn math_root_multiple_callback_refusal_is_callback_error() {
    let mut aFunc = make_scalar(|_: f64, _: &mut f64| false);
    assert_eq!(
        find_all_roots(&mut aFunc, 0.0, 1.0, &MultipleConfig::default()).status,
        Status::CallbackError
    );
    assert_eq!(
        find_all_roots_with_intervals(&mut aFunc, 0.0, 1.0, 20, [1e-10; 3]).status,
        Status::CallbackError
    );
}

#[test]
fn math_root_multiple_derivative_callback_refusal_is_callback_error() {
    let mut aFunc = RefusingDerivativeFunc;

    let mut aConfig = MultipleConfig::default();
    aConfig.nb_samples = 10;
    aConfig.f_tolerance = 1.0e-12;
    assert_eq!(
        find_all_roots_with_derivative(&mut aFunc, 0.0, 1.0, &aConfig).status,
        Status::CallbackError
    );
}
