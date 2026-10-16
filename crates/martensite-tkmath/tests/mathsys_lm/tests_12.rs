#[test]
fn math_sys_lm_step_min_participates_in_stagnation_detection() {
    let mut aFunc: LinearSystem2D = LinearSystem2D;
    let mut aStart: Vector = Vector::filled(1, 2, 0.0).unwrap();
    let mut aConfig: LMConfig = LMConfig::default();
    aConfig.solver.max_iterations = 1;
    aConfig.solver.tolerance = 1.0e-15;
    aConfig.solver.x_tolerance = 1.0e-15;
    aConfig.solver.f_tolerance = 1.0e-15;
    aConfig.solver.relative_tolerance = 0.0;

    let mut aDefaultStep: SystemResult = levenberg_marquardt(&mut aFunc, &aStart, &aConfig);
    assert_eq!(aDefaultStep.status, Status::MaxIterations);

    aConfig.solver.step_min = 10.0;
    let mut aMinimumStep: SystemResult = levenberg_marquardt(&mut aFunc, &aStart, &aConfig);
    assert_eq!(aMinimumStep.status, Status::NotConverged);
}

#[test]
fn math_sys_lm_tiny_accepted_step_with_large_residual_is_not_converged() {
    let mut aFunc: ScalarResidual = ScalarResidual;
    let mut aStart: Vector = Vector::filled(1, 1, 0.0).unwrap();
    let mut aConfig: LMConfig = LMConfig::default();
    aConfig.lambda_init = 1.0e12;
    aConfig.solver.x_tolerance = 1.0e-10;
    aConfig.solver.f_tolerance = 1.0e-12;

    let mut aResult: SystemResult = levenberg_marquardt(&mut aFunc, &aStart, &aConfig);
    assert_eq!(aResult.status, Status::NotConverged);
    assert!(!(aResult.is_root));
    assert!(aResult.residual_norm > 0.5);
}
