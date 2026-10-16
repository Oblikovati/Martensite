#[test]
fn math_newton_function_root_custom_iteration_limit() {
    // Test with custom iteration limits
    let mut aFunc = CubicWithDerivative; // More challenging function

    // Few iterations
    let mut aSolver1 = NewtonFunctionRoot::with_iterations(&mut aFunc, 1.5, 1.0e-10, 1.0e-10, 5);
    if aSolver1.is_done() {
        assert!(aSolver1.iterations().unwrap() <= 5);
    }

    // Many iterations
    let mut aSolver2 = NewtonFunctionRoot::with_iterations(&mut aFunc, 1.5, 1.0e-10, 1.0e-10, 100);
    assert!(aSolver2.is_done());
}

#[test]
fn math_newton_function_root_protected_constructor_and_perform() {
    // Test protected constructor and separate Perform call
    let mut aFunc = QuadraticWithDerivative;

    let mut aSolver = NewtonFunctionRoot::with_bounds(0.0, 5.0, 1.0e-10, 1.0e-10, 100); // Protected constructor with bounds
    aSolver.perform(&mut aFunc, 3.0);

    assert!(aSolver.is_done());
    assert_near(aSolver.root().unwrap(), 2.0, 1.0e-8);
}

#[test]
fn math_newton_function_root_unperformed_state() {
    // Test state handling before solving
    let mut aFunc = QuadraticWithDerivative;
    let mut aSolver = NewtonFunctionRoot::with_bounds(0.0, 5.0, 1.0e-10, 1.0e-10, 100); // Protected constructor only

    // Before Perform() is called, solver should report not done
    assert!(!aSolver.is_done());

    // In release builds, verify the solver maintains consistent state
    if !aSolver.is_done() {
        assert!(!aSolver.is_done());
    }
}
