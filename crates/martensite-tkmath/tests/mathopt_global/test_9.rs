#[test]
fn math_opt_glob_opt_differential_evolution_preserves_all_rejected_callback_status() {
    let mut aFunc: RejectAllFunc = RejectAllFunc;
    let mut aLower: Vector = Vector::filled(0, 1 - 1, -1.0).unwrap();
    let mut anUpper: Vector = Vector::filled(0, 1 - 1, 1.0).unwrap();
    let mut aConfig: GlobalConfig = GlobalConfig::new(GlobalStrategy::DifferentialEvolution, 10);
    aConfig.population = 4;

    assert_eq!(
        differential_evolution(&mut aFunc, &aLower, &anUpper, &aConfig).status,
        Status::CallbackError
    );
}
