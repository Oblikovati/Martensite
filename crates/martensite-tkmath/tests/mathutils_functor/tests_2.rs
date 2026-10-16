#[test]
fn math_utils_functor_scalar_shifted_value() {
    let mut aF = Polynomial::new(&[0.0, 0.0, 1.0]);
    let mut aShifted = make_shifted(aF, 5.0);

    let mut aValue: f64 = 0.0;
    assert!(aShifted.value(2.0, &mut aValue));
    near(aValue, 9.0, THE_TOLERANCE);
}

#[test]
fn math_utils_functor_scalar_negated_value() {
    let mut aF = Polynomial::new(&[0.0, 0.0, 1.0]);
    let mut aNegated = make_negated(aF);

    let mut aValue: f64 = 0.0;
    assert!(aNegated.value(3.0, &mut aValue));
    near(aValue, -9.0, THE_TOLERANCE);
}

#[test]
fn math_utils_functor_scalar_constant_value() {
    let mut aConst = Constant::new(42.0);

    let mut aValue: f64 = 0.0;
    let mut aDeriv: f64 = 0.0;
    assert!(aConst.value(123.456, &mut aValue));
    near(aValue, 42.0, THE_TOLERANCE);

    assert!(aConst.values(789.0, &mut aValue, &mut aDeriv));
    near(aValue, 42.0, THE_TOLERANCE);
    near(aDeriv, 0.0, THE_TOLERANCE);
}

#[test]
fn math_utils_functor_scalar_linear_value() {
    let mut aLinear = Linear::new(2.0, 3.0);

    let mut aValue: f64 = 0.0;
    let mut aDeriv: f64 = 0.0;
    assert!(aLinear.value(5.0, &mut aValue));
    near(aValue, 13.0, THE_TOLERANCE);

    assert!(aLinear.values(5.0, &mut aValue, &mut aDeriv));
    near(aValue, 13.0, THE_TOLERANCE);
    near(aDeriv, 2.0, THE_TOLERANCE);
}

#[test]
fn math_utils_functor_scalar_sine_value() {
    let mut aSine = Sine::new(1.0, 1.0, 0.0, 0.0);

    let mut aValue: f64 = 0.0;
    assert!(aSine.value(0.0, &mut aValue));
    near(aValue, 0.0, THE_TOLERANCE);

    assert!(aSine.value(std::f64::consts::PI / 2.0, &mut aValue));
    near(aValue, 1.0, THE_TOLERANCE);
}

#[test]
fn math_utils_functor_scalar_sine_values() {
    let mut aSine = Sine::new(2.0, 1.0, 0.0, 0.0);

    let mut aValue: f64 = 0.0;
    let mut aDeriv: f64 = 0.0;
    assert!(aSine.values(0.0, &mut aValue, &mut aDeriv));
    near(aValue, 0.0, THE_TOLERANCE);
    near(aDeriv, 2.0, THE_TOLERANCE);
}

#[test]
fn math_utils_functor_scalar_cosine_value() {
    let mut aCosine = Cosine::new(1.0, 1.0, 0.0, 0.0);

    let mut aValue: f64 = 0.0;
    assert!(aCosine.value(0.0, &mut aValue));
    near(aValue, 1.0, THE_TOLERANCE);

    assert!(aCosine.value(std::f64::consts::PI, &mut aValue));
    near(aValue, -1.0, THE_TOLERANCE);
}
