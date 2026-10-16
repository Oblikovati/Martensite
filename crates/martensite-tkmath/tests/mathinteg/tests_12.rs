#[test]
fn math_integ_kronrod_auto_counts_exploratory_and_adaptive_evaluations() {
    #[derive(Default)]
    struct CountingKink {
        NbCalls: usize,
    }
    impl Function for CountingKink {
        fn value(&mut self, theX: f64, theF: &mut f64) -> bool {
            let _ = (&theX, &theF);
            self.NbCalls += 1;
            *theF = (theX - 0.3141592653589793).abs();
            true
        }
    }

    let mut aFunc: CountingKink = CountingKink::default();
    let mut aResult: IntegResult = kronrod_auto(&mut aFunc, 0.0, 1.0, 1.0e-30, 7);

    assert_eq!(aResult.number_of_points, aFunc.NbCalls);
    assert!(aResult.number_of_points > 15);
}

#[test]
fn math_integ_kronrod_applies_quadpack_roundoff_floor() {
    let mut aFunc: ConstantFunc = ConstantFunc;
    let mut aResult: IntegResult = kronrod_rule(&mut aFunc, 0.0, 2.0, 7);

    assert!(aResult.is_done());
    assert!(aResult.absolute_error.is_some());
    assert!(
        aResult.absolute_error.unwrap() >= 50.0 * f64::EPSILON * (aResult.value.unwrap()).abs()
    );
}

#[test]
fn math_integ_kronrod_unrepresentable_midpoint_is_not_converged() {
    let mut aFunc: ConstantFunc = ConstantFunc;
    let aLower: f64 = 4503599627370496.0;
    let anUpper: f64 = aLower.next_up();
    let mut aConfig: KronrodConfig = KronrodConfig::default();
    aConfig.integration.tolerance = 1.0e-30;
    aConfig.integration.max_iterations = 2;

    let mut aResult: IntegResult = kronrod(&mut aFunc, aLower, anUpper, &aConfig);

    assert_eq!(aResult.status, Status::NotConverged);
    assert_eq!(aResult.iterations, 1);
}
