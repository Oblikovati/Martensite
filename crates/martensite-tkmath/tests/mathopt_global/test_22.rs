#[test]
fn math_opt_glob_opt_pso_hybrid_accepts_single_iteration_budget() {
    let mut aFunc: SphereFunc = SphereFunc;
    let mut aLower: Vector = Vector::filled(0, 1 - 1, -1.0).unwrap();
    let mut anUpper: Vector = Vector::filled(0, 1 - 1, 1.0).unwrap();
    let mut aConfig: GlobalConfig = GlobalConfig::new(GlobalStrategy::PSOHybrid, 1);
    aConfig.population = 4;
    aConfig.polish_budget_per_dim = 0;

    let aResult: VectorResult = global_minimum(&mut aFunc, &aLower, &anUpper, &aConfig);

    assert_ne!(aResult.status, Status::InvalidInput);
}
