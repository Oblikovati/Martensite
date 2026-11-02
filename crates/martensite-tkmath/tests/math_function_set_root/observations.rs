use super::*;
use martensite_tkmath::math::RootSearchTermination as Exit;

fn linear_solver(limit: i32) -> FunctionSetRoot {
    let tolerance = Vector::filled(1, 2, 1e-8).unwrap();
    FunctionSetRoot::new(&LinearSystem2D, &tolerance, limit).unwrap()
}
#[test]
fn accepted_local_solution_retains_owned_work() {
    let mut solver = linear_solver(100);
    assert_eq!(solver.search_observation().termination, Exit::NotRun);
    let start = Vector::filled(1, 2, 0.).unwrap();
    solver.perform(&mut LinearSystem2D, &start, false).unwrap();
    let receipt = solver.search_observation();
    assert_eq!(receipt.termination, Exit::Accepted);
    assert!(receipt.iterations_entered > 0);
    assert_eq!(receipt.iteration_limit, 100);
    invalidate_solver(&mut solver);
    assert_eq!(receipt.termination, Exit::Accepted);
}
fn invalidate_solver(solver: &mut FunctionSetRoot) {
    let invalid = Vector::filled(1, 1, 0.).unwrap();
    assert!(
        solver
            .perform(&mut LinearSystem2D, &invalid, false)
            .is_err()
    );
    assert_eq!(solver.search_observation().termination, Exit::InvalidInput);
}
#[test]
fn zero_budget_is_observed_without_accepting_absence() {
    let mut solver = linear_solver(0);
    let start = Vector::filled(1, 2, 0.).unwrap();
    solver.perform(&mut LinearSystem2D, &start, false).unwrap();
    let receipt = solver.search_observation();
    assert_eq!(receipt.termination, Exit::IterationLimit);
    assert_eq!(receipt.iterations_entered, 0);
    assert!(!solver.is_done());
}
#[test]
fn exact_start_needs_no_outer_iteration() {
    let mut solver = linear_solver(0);
    let mut start = Vector::filled(1, 2, 1.).unwrap();
    start[1] = 2.;
    solver.perform(&mut LinearSystem2D, &start, false).unwrap();
    assert_eq!(solver.search_observation().termination, Exit::Accepted);
    assert_eq!(solver.search_observation().iterations_entered, 0);
}
#[test]
fn divergence_preserves_actual_stop_reason() {
    let mut solver = linear_solver(100);
    let start = Vector::filled(1, 2, 10.).unwrap();
    let lower = Vector::filled(1, 2, 0.).unwrap();
    let upper = Vector::filled(1, 2, 1.).unwrap();
    solver
        .perform_bounded(&mut LinearSystem2D, &start, &lower, &upper, true)
        .unwrap();
    assert_eq!(solver.search_observation().termination, Exit::Divergence);
    assert_eq!(solver.search_observation().iterations_entered, 0);
    assert!(!solver.is_done());
}
struct DecliningEvaluation;
impl FunctionSet for DecliningEvaluation {
    fn variables(&self) -> i32 {
        2
    }
    fn equations(&self) -> i32 {
        2
    }
    fn value(&mut self, _: &Vector<'_>, _: &mut Vector<'_>) -> bool {
        false
    }
}
impl FunctionSetWithDerivatives for DecliningEvaluation {
    fn derivatives(&mut self, _: &Vector<'_>, _: &mut Matrix<'_>) -> bool {
        false
    }
    fn values(&mut self, _: &Vector<'_>, _: &mut Vector<'_>, _: &mut Matrix<'_>) -> bool {
        false
    }
}
#[test]
fn callback_decline_is_not_iteration_exhaustion() {
    let mut solver = linear_solver(100);
    let start = Vector::filled(1, 2, 0.).unwrap();
    solver
        .perform(&mut DecliningEvaluation, &start, false)
        .unwrap();
    assert_eq!(
        solver.search_observation().termination,
        Exit::EvaluationDeclined
    );
    assert_eq!(solver.search_observation().iterations_entered, 0);
    assert!(!solver.is_done());
}
struct StationaryResidual;
impl FunctionSet for StationaryResidual {
    fn variables(&self) -> i32 {
        2
    }
    fn equations(&self) -> i32 {
        2
    }
    fn value(&mut self, _: &Vector<'_>, output: &mut Vector<'_>) -> bool {
        output[1] = 1.;
        output[2] = 1.;
        true
    }
}
impl FunctionSetWithDerivatives for StationaryResidual {
    fn derivatives(&mut self, _: &Vector<'_>, output: &mut Matrix<'_>) -> bool {
        for row in 1..=2 {
            for col in 1..=2 {
                output[(row, col)] = 0.;
            }
        }
        true
    }
    fn values(
        &mut self,
        input: &Vector<'_>,
        output: &mut Vector<'_>,
        jacobian: &mut Matrix<'_>,
    ) -> bool {
        self.value(input, output);
        self.derivatives(input, jacobian)
    }
}
#[test]
fn stationary_nonroot_keeps_legacy_result_and_explicit_decline() {
    let mut solver = linear_solver(100);
    let start = Vector::filled(1, 2, 0.).unwrap();
    solver
        .perform(&mut StationaryResidual, &start, false)
        .unwrap();
    assert!(solver.is_done());
    assert_eq!(solver.search_observation().termination, Exit::Stationary);
    assert_eq!(solver.search_observation().iterations_entered, 0);
    assert_eq!(solver.function_set_errors().unwrap().length(), 2);
}
struct DecliningRefresh;
impl FunctionSet for DecliningRefresh {
    fn variables(&self) -> i32 {
        2
    }
    fn equations(&self) -> i32 {
        2
    }
    fn value(&mut self, _: &Vector<'_>, _: &mut Vector<'_>) -> bool {
        false
    }
}
impl FunctionSetWithDerivatives for DecliningRefresh {
    fn derivatives(&mut self, input: &Vector<'_>, output: &mut Matrix<'_>) -> bool {
        LinearSystem2D.derivatives(input, output)
    }
    fn values(
        &mut self,
        input: &Vector<'_>,
        output: &mut Vector<'_>,
        jacobian: &mut Matrix<'_>,
    ) -> bool {
        LinearSystem2D.values(input, output, jacobian)
    }
}
#[test]
fn refresh_callback_decline_is_retained_without_changing_result() {
    let mut solver = linear_solver(100);
    let start = Vector::filled(1, 2, 0.).unwrap();
    solver
        .perform(&mut DecliningRefresh, &start, false)
        .unwrap();
    assert!(solver.is_done());
    assert_eq!(
        solver.search_observation().termination,
        Exit::EvaluationDeclined
    );
    assert!(solver.search_observation().iterations_entered > 0);
}
