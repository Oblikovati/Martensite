#[test]
fn math_root_trig_cde_no_solution() {
    let aResult: TrigResult = trigonometric_cde(1.0, 1.0, -2.0, 0.0, TAU);
    assert!(aResult.is_done());
    assert_eq!(aResult.nb_roots, 0);
}

#[test]
fn math_root_trig_cde_tangent_root_at_lower_bound() {
    let aLower: f64 = -0.989;
    let aC: f64 = martensite_cmath::cos(aLower);
    let aD: f64 = martensite_cmath::sin(aLower);
    let aResult: TrigResult = trigonometric_cde(aC, aD, -1.0, aLower, aLower + 0.1);

    assert_eq!(aResult.status, Status::Ok);
    assert_eq!(aResult.nb_roots, 1);
    near(aResult.roots[0], aLower, THE_TOL);
    verify_roots(&aResult, [0.0, 0.0, aC, aD, -1.0], 1e-10);
}

#[test]
fn math_root_trig_cde_infinite_bounds_use_canonical_period() {
    let aResult: TrigResult = trigonometric_cde(1.0, 1.0, 0.0, -2e100, 2e100);

    assert_eq!(aResult.status, Status::Ok);
    assert_eq!(aResult.nb_roots, 2);
    assert!(aResult.roots[0] >= 0.0);
    assert!(aResult.roots[1] <= TAU);
    verify_roots(&aResult, [0.0, 0.0, 1.0, 1.0, 0.0], 1e-10);
}
