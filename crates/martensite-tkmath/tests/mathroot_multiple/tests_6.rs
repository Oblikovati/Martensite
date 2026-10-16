#[test]
fn math_root_multiple_find_all_roots_with_derivative_negative_extrema_asymmetric_roots() {
    let mut aFunc = make_scalar_with_derivative(|theX: f64, theY: &mut f64, theDY: &mut f64| {
        *theY = -(theX).sin();
        *theDY = -(theX).cos();
        true
    });

    let mut aConfig = MultipleConfig::default();
    aConfig.offset = -0.5;
    aConfig.f_tolerance = 1.0e-12;

    let mut aResult =
        find_all_roots_with_derivative(&mut aFunc, 0.0, 2.0 * std::f64::consts::PI, &aConfig);
    assert!(aResult.is_done());
    assert_eq!(aResult.nb_roots(), 2);
    if aResult.nb_roots() == 2 {
        near(aResult.roots[0], std::f64::consts::PI / 6.0, THE_TOLERANCE);
        near(
            aResult.roots[1],
            5.0 * std::f64::consts::PI / 6.0,
            THE_TOLERANCE,
        );
    }
}

#[test]
fn math_root_multiple_find_all_roots_exact_zero_at_sample() {
    let mut aFunc = Polynomial::new(&[0.0, -1.0, 1.0]);

    let mut aResult = find_all_roots_samples(&mut aFunc, 0.0, 2.0, 10);
    assert!(aResult.is_done());
    assert!(aResult.nb_roots() >= 2);

    let mut isZeroFound: bool = false;
    let mut isOneFound: bool = false;
    for i in 0..aResult.nb_roots() {
        if (aResult.roots[i]).abs() < THE_TOLERANCE {
            isZeroFound = true;
        }
        if (aResult.roots[i] - 1.0).abs() < THE_TOLERANCE {
            isOneFound = true;
        }
    }
    assert!(isZeroFound);
    assert!(isOneFound);
}

#[test]
fn math_root_multiple_find_all_roots_exact_zero_with_sign_change() {
    let mut aFunc = make_scalar(|theX: f64, theY: &mut f64| {
        *theY = theX;
        true
    });

    let mut aResult = find_all_roots_samples(&mut aFunc, -1.0, 1.0, 10);
    assert!(aResult.is_done());
    assert!(aResult.nb_roots() >= 1);

    let mut isFound: bool = false;
    for i in 0..aResult.nb_roots() {
        if (aResult.roots[i]).abs() < THE_TOLERANCE {
            isFound = true;
            break;
        }
    }
    assert!(isFound);
}
