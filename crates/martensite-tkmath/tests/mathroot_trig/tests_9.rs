#[test]
fn math_root_trig_bounds_multiple_circles() {
    let aResult: TrigResult = trigonometric_linear(1.0, 0.0, 0.0, 4.0 * PI);
    assert!(aResult.is_done());

    assert_eq!(aResult.nb_roots, 2);
}

#[test]
fn math_root_trig_cosine_only_returns_one_period() {
    let aResult: TrigResult = trigonometric([0.0, 0.0, 1.0, 0.0, 0.0], 0.0, 4.0 * PI, 1.5e-12);
    assert!(aResult.is_done());
    assert_eq!(aResult.nb_roots, 2);
}

#[test]
fn math_root_trig_degenerate_all_zero() {
    let aResult: TrigResult = trigonometric([0.0, 0.0, 0.0, 0.0, 0.0], 0.0, TAU, 1.5e-12);
    assert!(aResult.infinite_roots);
}
