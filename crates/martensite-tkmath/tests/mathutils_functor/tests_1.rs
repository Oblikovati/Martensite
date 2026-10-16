#[test]
fn math_utils_functor_scalar_rational_division_by_zero() {
    let mut aRat = Rational::new(&[0.0, 1.0], &[-1.0, 1.0]);

    let mut aValue: f64 = 0.0;
    assert!(!(aRat.value(1.0, &mut aValue)));
}

#[test]
fn math_utils_functor_scalar_composite_value() {
    let mut aOuter = Polynomial::new(&[0.0, 0.0, 1.0]);
    let mut aInner = Polynomial::new(&[1.0, 1.0]);
    let mut aComposite = make_composite(aOuter, aInner);

    let mut aValue: f64 = 0.0;
    assert!(aComposite.value(0.0, &mut aValue));
    near(aValue, 1.0, THE_TOLERANCE);

    assert!(aComposite.value(2.0, &mut aValue));
    near(aValue, 9.0, THE_TOLERANCE);
}

#[test]
fn math_utils_functor_scalar_sum_value() {
    let mut aF = Polynomial::new(&[0.0, 0.0, 1.0]);
    let mut aG = Polynomial::new(&[1.0, 2.0]);
    let mut aSum = make_sum(aF, aG);

    let mut aValue: f64 = 0.0;
    assert!(aSum.value(1.0, &mut aValue));
    near(aValue, 4.0, THE_TOLERANCE);

    assert!(aSum.value(2.0, &mut aValue));
    near(aValue, 9.0, THE_TOLERANCE);
}

#[test]
fn math_utils_functor_scalar_difference_value() {
    let mut aF = Polynomial::new(&[0.0, 0.0, 1.0]);
    let mut aG = Polynomial::new(&[0.0, 1.0]);
    let mut aDiff = make_difference(aF, aG);

    let mut aValue: f64 = 0.0;
    assert!(aDiff.value(3.0, &mut aValue));
    near(aValue, 6.0, THE_TOLERANCE);
}

#[test]
fn math_utils_functor_scalar_product_value() {
    let mut aF = Polynomial::new(&[0.0, 1.0]);
    let mut aG = Polynomial::new(&[1.0, 1.0]);
    let mut aProd = make_product(aF, aG);

    let mut aValue: f64 = 0.0;
    assert!(aProd.value(2.0, &mut aValue));
    near(aValue, 6.0, THE_TOLERANCE);
}

#[test]
fn math_utils_functor_scalar_quotient_value() {
    let mut aF = Polynomial::new(&[0.0, 0.0, 1.0]);
    let mut aG = Polynomial::new(&[0.0, 1.0]);
    let mut aQuot = make_quotient(aF, aG);

    let mut aValue: f64 = 0.0;
    assert!(aQuot.value(5.0, &mut aValue));
    near(aValue, 5.0, THE_TOLERANCE);
}

#[test]
fn math_utils_functor_scalar_scaled_value() {
    let mut aF = Polynomial::new(&[0.0, 0.0, 1.0]);
    let mut aScaled = make_scaled(aF, 3.0);

    let mut aValue: f64 = 0.0;
    assert!(aScaled.value(2.0, &mut aValue));
    near(aValue, 12.0, THE_TOLERANCE);
}
