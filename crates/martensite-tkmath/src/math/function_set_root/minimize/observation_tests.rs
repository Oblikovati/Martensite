use super::*;

struct QuadraticScalar {
    decline: bool,
}
impl Function for QuadraticScalar {
    fn value(&mut self, parameter: f64, output: &mut f64) -> bool {
        *output = (parameter - 1.) * (parameter - 1.);
        !self.decline
    }
}
#[test]
fn actual_scalar_acceptance_is_observed() {
    let mut solver = BrentMinimum::with_options(1e-8, 100, 1e-8);
    let actual = observe_scalar(
        &mut solver,
        &mut QuadraticScalar { decline: false },
        [0., 0.8, 2.],
    );
    assert!(actual.accepted);
    assert!(!actual.declined);
    assert!(actual.iterations > 0);
}
#[test]
fn scalar_callback_decline_cannot_reuse_stale_success() {
    let mut solver = BrentMinimum::with_options(1e-8, 100, 1e-8);
    let first = observe_scalar(
        &mut solver,
        &mut QuadraticScalar { decline: false },
        [0., 0.8, 2.],
    );
    assert!(first.accepted);
    let next = observe_scalar(
        &mut solver,
        &mut QuadraticScalar { decline: true },
        [0., 0.8, 2.],
    );
    assert!(solver.is_done());
    assert!(!next.accepted);
    assert!(next.declined);
    assert_eq!(next.iterations, 0);
}
#[test]
fn zero_scalar_budget_is_not_callback_decline() {
    let mut solver = BrentMinimum::with_options(1e-8, 0, 1e-8);
    let actual = observe_scalar(
        &mut solver,
        &mut QuadraticScalar { decline: false },
        [0., 0.8, 2.],
    );
    assert!(!actual.accepted);
    assert!(!actual.declined);
    assert_eq!(actual.iterations, 0);
}
