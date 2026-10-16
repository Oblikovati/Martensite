#[test]
fn math_root_trig_degenerate_constant_non_zero() {
    let aResult: TrigResult = trigonometric([0.0, 0.0, 0.0, 0.0, 1.0], 0.0, TAU, 1.5e-12);
    assert!(aResult.is_done());
    assert_eq!(aResult.nb_roots, 0);
}

#[test]
fn math_root_trig_degenerate_near_zero_coefficients() {
    let aResult: TrigResult = trigonometric([1.0e-14, 1.0e-14, 1.0, 0.0, -0.5], 0.0, TAU, 1.5e-12);
    assert!(aResult.is_done());

    assert_eq!(aResult.nb_roots, 2);
    verify_roots(&aResult, [1.0e-14, 1.0e-14, 1.0, 0.0, -0.5], 1.0e-6);
}

#[test]
fn math_root_trig_special_case_pi_root() {
    let aResult: TrigResult = trigonometric([1.0, 0.0, -1.0, 0.0, 0.0], 0.0, TAU, 1.5e-12);
    assert!(aResult.is_done());
    assert!(aResult.nb_roots >= 2);
    verify_roots(&aResult, [1.0, 0.0, -1.0, 0.0, 0.0], 1e-10);
}
