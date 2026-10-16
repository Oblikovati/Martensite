#[test]
fn math_root_trig_linear_sin_zero_constant() {
    let aResult: TrigResult = trigonometric_linear(1.0, 0.0, 0.0, TAU);
    assert!(aResult.is_done());
    assert_eq!(aResult.nb_roots, 2);
    verify_roots(&aResult, [0.0, 0.0, 0.0, 1.0, 0.0], 1e-10);
}

#[test]
fn math_root_trig_linear_sin_half_value() {
    let aResult: TrigResult = trigonometric_linear(1.0, -0.5, 0.0, TAU);
    assert!(aResult.is_done());
    assert_eq!(aResult.nb_roots, 2);
    verify_roots(&aResult, [0.0, 0.0, 0.0, 1.0, -0.5], 1e-10);
}

#[test]
fn math_root_trig_linear_sin_negative_half() {
    let aResult: TrigResult = trigonometric_linear(1.0, 0.5, 0.0, TAU);
    assert!(aResult.is_done());
    assert_eq!(aResult.nb_roots, 2);
    verify_roots(&aResult, [0.0, 0.0, 0.0, 1.0, 0.5], 1e-10);
}
