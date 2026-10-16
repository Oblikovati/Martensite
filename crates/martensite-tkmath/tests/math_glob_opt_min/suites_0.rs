// Declarations of the global minimizer suite; see the parent module.
#[test]
fn math_glob_opt_min_quadratic_function_optimization() {
    let mut aFunc = QuadraticFunction;

    let mut aLowerBorder = Vector::with_bounds(1, 2).unwrap();
    aLowerBorder[1] = -2.0;
    aLowerBorder[2] = -1.0;

    let mut aUpperBorder = Vector::with_bounds(1, 2).unwrap();
    aUpperBorder[1] = 3.0;
    aUpperBorder[2] = 4.0;

    let mut aSolver = GlobalOptimum::new(
        GlobalObjective::Value(&mut aFunc),
        &aLowerBorder,
        &aUpperBorder,
    )
    .unwrap();
    aSolver.perform(true).unwrap();

    assert!(aSolver.is_done());
    assert!(aSolver.number_of_extrema() > 0);

    let mut aSol = Vector::with_bounds(1, 2).unwrap();
    aSolver.points(1, &mut aSol).unwrap();

    near(aSol[1], 1.0, 1.0e-1);
    near(aSol[2], 2.0, 1.0e-1);
    near(aSolver.minimum(), 0.0, 1.0e-2);
}
#[test]
fn math_glob_opt_min_multi_modal_function_optimization() {
    let mut aFunc = MultiModalFunction;

    let mut aLowerBorder = Vector::with_bounds(1, 2).unwrap();
    aLowerBorder[1] = -5.0;
    aLowerBorder[2] = -5.0;

    let mut aUpperBorder = Vector::with_bounds(1, 2).unwrap();
    aUpperBorder[1] = 5.0;
    aUpperBorder[2] = 5.0;

    let mut aSolver = GlobalOptimum::new(
        GlobalObjective::Value(&mut aFunc),
        &aLowerBorder,
        &aUpperBorder,
    )
    .unwrap();
    aSolver.perform(false).unwrap();

    assert!(aSolver.is_done());
    assert!(aSolver.number_of_extrema() >= 1);

    let mut aSol = Vector::with_bounds(1, 2).unwrap();
    aSolver.points(1, &mut aSol).unwrap();
    assert!(aSol[1] >= -5.0 && aSol[1] <= 5.0);
    assert!(aSol[2] >= -5.0 && aSol[2] <= 5.0);
}
#[test]
fn math_glob_opt_min_one_dimensional_optimization() {
    let mut aFunc = MultiModal1DFunction;

    let mut aLowerBorder = Vector::with_bounds(1, 1).unwrap();
    aLowerBorder[1] = 0.0;

    let mut aUpperBorder = Vector::with_bounds(1, 1).unwrap();
    aUpperBorder[1] = 2.0 * PI;

    let mut aSolver = GlobalOptimum::new(
        GlobalObjective::Value(&mut aFunc),
        &aLowerBorder,
        &aUpperBorder,
    )
    .unwrap();
    aSolver.perform(false).unwrap();

    assert!(aSolver.is_done());
    assert!(aSolver.number_of_extrema() > 0);

    let mut aSol = Vector::with_bounds(1, 1).unwrap();
    aSolver.points(1, &mut aSol).unwrap();
    assert!(aSol[1] >= 0.0 && aSol[1] <= 2.0 * PI);
}
#[test]
fn math_glob_opt_min_single_solution_search() {
    let mut aFunc = QuadraticFunction;

    let mut aLowerBorder = Vector::with_bounds(1, 2).unwrap();
    aLowerBorder[1] = -2.0;
    aLowerBorder[2] = -1.0;

    let mut aUpperBorder = Vector::with_bounds(1, 2).unwrap();
    aUpperBorder[1] = 3.0;
    aUpperBorder[2] = 4.0;

    let mut aSolver = GlobalOptimum::new(
        GlobalObjective::Value(&mut aFunc),
        &aLowerBorder,
        &aUpperBorder,
    )
    .unwrap();
    aSolver.perform(true).unwrap();

    assert!(aSolver.is_done());
    assert_eq!(aSolver.number_of_extrema(), 1);
}
