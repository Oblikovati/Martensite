#[test]
fn math_root_trig_mixed_sin_times_cos() {
    let aResult: TrigResult = trigonometric([0.0, 0.5, 0.0, 1.0, 0.0], 0.0, TAU, 1.5e-12);
    assert!(aResult.is_done());
    assert_eq!(aResult.nb_roots, 2);
    verify_roots(&aResult, [0.0, 0.5, 0.0, 1.0, 0.0], 1e-10);
}

#[test]
fn math_root_trig_general_four_roots() {
    let aResult: TrigResult = trigonometric([1.0, 0.0, 1.0, 0.0, -0.5], 0.0, TAU, 1.5e-12);
    assert!(aResult.is_done());

    assert!(aResult.nb_roots >= 2);
    verify_roots(&aResult, [1.0, 0.0, 1.0, 0.0, -0.5], 1e-10);
}

#[test]
fn math_root_trig_general_all_coefficients() {
    let aResult: TrigResult = trigonometric([1.0, 0.5, 0.3, 0.4, -0.2], 0.0, TAU, 1.5e-12);
    assert!(aResult.is_done());

    verify_roots(&aResult, [1.0, 0.5, 0.3, 0.4, -0.2], 1.0e-8);
}
