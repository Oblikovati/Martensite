use super::{BrentMinimum, Function, FunctionSetWithDerivatives, Line};

#[derive(Default)]
struct ScalarSearchReturn {
    iterations: u64,
    accepted: bool,
    declined: bool,
}
struct ObservedScalar<'a, F> {
    function: &'a mut F,
    declined: bool,
}
impl<F: Function> Function for ObservedScalar<'_, F> {
    fn value(&mut self, parameter: f64, value: &mut f64) -> bool {
        let accepted = self.function.value(parameter, value);
        self.declined |= !accepted;
        accepted
    }
}
pub(super) fn observe_minimum<F: FunctionSetWithDerivatives>(
    solver: &mut BrentMinimum,
    line: &mut Line<'_, F>,
    bracket: [f64; 3],
) {
    let actual = observe_scalar(solver, line, bracket);
    line.work.searches += 1;
    line.work.iterations_entered += actual.iterations;
    line.work.accepted += u64::from(actual.accepted);
    line.work.exhausted += u64::from(!actual.accepted && !actual.declined);
}
fn observe_scalar<F: Function>(
    solver: &mut BrentMinimum,
    function: &mut F,
    bracket: [f64; 3],
) -> ScalarSearchReturn {
    let mut actual = ScalarSearchReturn::default();
    let mut observed = ObservedScalar {
        function,
        declined: false,
    };
    solver.perform_with_criterion(&mut observed, bracket, &mut |solver, _| {
        actual.iterations += 1;
        actual.accepted = solver.is_solution_reached();
        actual.accepted
    });
    actual.declined = observed.declined;
    actual
}
#[cfg(test)]
#[path = "observation_tests.rs"]
mod tests;
