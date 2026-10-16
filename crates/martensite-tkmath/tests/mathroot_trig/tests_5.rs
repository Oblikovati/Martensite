#[test]
fn math_root_trig_quadratic_cos_two_solutions() {
    let aResult: TrigResult = trigonometric([1.0, 0.0, 0.0, 0.0, -1.0], 0.0, TAU, 1.5e-12);
    assert!(aResult.is_done());
    assert!(aResult.nb_roots >= 2);
    verify_roots(&aResult, [1.0, 0.0, 0.0, 0.0, -1.0], 1e-10);
}

#[test]
fn math_root_trig_quadratic_cos_four_solutions() {
    let aResult: TrigResult = trigonometric([1.0, 0.0, 0.0, 0.0, -0.25], 0.0, TAU, 1.5e-12);
    assert!(aResult.is_done());
    assert_eq!(aResult.nb_roots, 4);
    verify_roots(&aResult, [1.0, 0.0, 0.0, 0.0, -0.25], 1e-10);
}

#[test]
fn math_root_trig_cos_sin_product_ae_zero() {
    let aResult: TrigResult = trigonometric([0.0, 1.0, 0.0, 0.0, 0.0], 0.0, TAU, 1.5e-12);
    assert!(aResult.is_done());
    assert_eq!(aResult.nb_roots, 4);
    verify_roots(&aResult, [0.0, 1.0, 0.0, 0.0, 0.0], 1e-10);
}
