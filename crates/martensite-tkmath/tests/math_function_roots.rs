//! Math function roots tests.
#![allow(non_snake_case)]
use martensite_tkmath::math::{Function, FunctionRoots, FunctionWithDerivative};
struct QuadraticWithDerivative;
impl Function for QuadraticWithDerivative {
    fn value(&mut self, x: f64, value: &mut f64) -> bool {
        let _ = x;
        *value = x * x - 4.0;
        true
    }
}
impl FunctionWithDerivative for QuadraticWithDerivative {
    fn derivative(&mut self, x: f64, derivative: &mut f64) -> bool {
        let _ = x;
        *derivative = 2.0 * x;
        true
    }
    fn values(&mut self, x: f64, value: &mut f64, derivative: &mut f64) -> bool {
        let _ = x;
        *value = x * x - 4.0;
        *derivative = 2.0 * x;
        true
    }
}
struct CubicWithDerivative;
impl Function for CubicWithDerivative {
    fn value(&mut self, x: f64, value: &mut f64) -> bool {
        let _ = x;
        *value = x * x * x - 6.0 * x * x + 11.0 * x - 6.0;
        true
    }
}
impl FunctionWithDerivative for CubicWithDerivative {
    fn derivative(&mut self, x: f64, derivative: &mut f64) -> bool {
        let _ = x;
        *derivative = 3.0 * x * x - 12.0 * x + 11.0;
        true
    }
    fn values(&mut self, x: f64, value: &mut f64, derivative: &mut f64) -> bool {
        let _ = x;
        *value = x * x * x - 6.0 * x * x + 11.0 * x - 6.0;
        *derivative = 3.0 * x * x - 12.0 * x + 11.0;
        true
    }
}
struct SineWithDerivative;
impl Function for SineWithDerivative {
    fn value(&mut self, x: f64, value: &mut f64) -> bool {
        let _ = x;
        *value = martensite_cmath::sin(x);
        true
    }
}
impl FunctionWithDerivative for SineWithDerivative {
    fn derivative(&mut self, x: f64, derivative: &mut f64) -> bool {
        let _ = x;
        *derivative = martensite_cmath::cos(x);
        true
    }
    fn values(&mut self, x: f64, value: &mut f64, derivative: &mut f64) -> bool {
        let _ = x;
        *value = martensite_cmath::sin(x);
        *derivative = martensite_cmath::cos(x);
        true
    }
}
struct LinearWithDerivative;
impl Function for LinearWithDerivative {
    fn value(&mut self, x: f64, value: &mut f64) -> bool {
        let _ = x;
        *value = 2.0 * x - 4.0;
        true
    }
}
impl FunctionWithDerivative for LinearWithDerivative {
    fn derivative(&mut self, x: f64, derivative: &mut f64) -> bool {
        let _ = x;
        *derivative = 2.0;
        true
    }
    fn values(&mut self, x: f64, value: &mut f64, derivative: &mut f64) -> bool {
        let _ = x;
        *value = 2.0 * x - 4.0;
        *derivative = 2.0;
        true
    }
}
struct ConstantZeroFunction;
impl Function for ConstantZeroFunction {
    fn value(&mut self, x: f64, value: &mut f64) -> bool {
        let _ = x;
        *value = 0.0;
        true
    }
}
impl FunctionWithDerivative for ConstantZeroFunction {
    fn derivative(&mut self, x: f64, derivative: &mut f64) -> bool {
        let _ = x;
        *derivative = 0.0;
        true
    }
    fn values(&mut self, x: f64, value: &mut f64, derivative: &mut f64) -> bool {
        let _ = x;
        *value = 0.0;
        *derivative = 0.0;
        true
    }
}
struct NoRootsFunction;
impl Function for NoRootsFunction {
    fn value(&mut self, x: f64, value: &mut f64) -> bool {
        let _ = x;
        *value = x * x + 1.0;
        true
    }
}
impl FunctionWithDerivative for NoRootsFunction {
    fn derivative(&mut self, x: f64, derivative: &mut f64) -> bool {
        let _ = x;
        *derivative = 2.0 * x;
        true
    }
    fn values(&mut self, x: f64, value: &mut f64, derivative: &mut f64) -> bool {
        let _ = x;
        *value = x * x + 1.0;
        *derivative = 2.0 * x;
        true
    }
}
struct QuarticWithDerivative;
impl Function for QuarticWithDerivative {
    fn value(&mut self, x: f64, value: &mut f64) -> bool {
        let _ = x;
        *value = (x - 1.0) * (x - 2.0) * (x - 3.0) * (x - 4.0);
        true
    }
}
impl FunctionWithDerivative for QuarticWithDerivative {
    fn derivative(&mut self, x: f64, derivative: &mut f64) -> bool {
        let _ = x;
        *derivative = 4.0 * x * x * x - 30.0 * x * x + 70.0 * x - 50.0;
        true
    }
    fn values(&mut self, x: f64, value: &mut f64, derivative: &mut f64) -> bool {
        let _ = x;
        *value = (x - 1.0) * (x - 2.0) * (x - 3.0) * (x - 4.0);
        *derivative = 4.0 * x * x * x - 30.0 * x * x + 70.0 * x - 50.0;
        true
    }
}
fn near(a: f64, b: f64, tolerance: f64) {
    assert!((a - b).abs() <= tolerance, "{a} != {b}");
}
fn sorted(solver: &FunctionRoots) -> Vec<f64> {
    let mut roots: Vec<f64> = (1..=solver.number_of_solutions().unwrap())
        .map(|i| solver.value(i).unwrap())
        .collect();
    roots.sort_by(f64::total_cmp);
    roots
}
#[test]
fn math_function_roots_quadratic_two_roots() {
    let mut f = QuadraticWithDerivative;
    let s = FunctionRoots::with_options(&mut f, -5.0, 5.0, 20, [1e-10, 1e-10, 0.0], 0.0);
    assert!(s.is_done());
    assert!(!s.is_all_null().unwrap());
    assert_eq!(s.number_of_solutions().unwrap(), 2);
    for (root, expected) in sorted(&s).into_iter().zip([-2.0, 2.0]) {
        near(root, expected, 1e-8);
    }
}
#[test]
fn math_function_roots_cubic_three_roots() {
    let mut f = CubicWithDerivative;
    let s = FunctionRoots::with_options(&mut f, 0.0, 4.0, 30, [1e-10, 1e-10, 0.0], 0.0);
    assert!(s.is_done());
    assert!(!s.is_all_null().unwrap());
    assert_eq!(s.number_of_solutions().unwrap(), 3);
    for (root, expected) in sorted(&s).into_iter().zip([1.0, 2.0, 3.0]) {
        near(root, expected, 1e-8);
    }
}
#[test]
fn math_function_roots_sine_multiple_roots() {
    let mut f = SineWithDerivative;
    let s = FunctionRoots::with_options(
        &mut f,
        0.0,
        2.0 * std::f64::consts::PI,
        50,
        [1e-10, 1e-10, 0.0],
        0.0,
    );
    assert!(s.is_done());
    assert!(!s.is_all_null().unwrap());
    let count: i32 = s.number_of_solutions().unwrap();
    assert!(count >= 2);
    assert!(count <= 3);
    for i in 1..=count {
        let mut value: f64 = 0.0;
        f.value(s.value(i).unwrap(), &mut value);
        near(value, 0.0, 1e-8);
    }
}
#[test]
fn math_function_roots_linear_single_root() {
    let mut f = LinearWithDerivative;
    let s = FunctionRoots::with_options(&mut f, 0.0, 4.0, 10, [1e-10, 1e-10, 0.0], 0.0);
    assert!(s.is_done());
    assert!(!s.is_all_null().unwrap());
    assert_eq!(s.number_of_solutions().unwrap(), 1);
    near(s.value(1).unwrap(), 2.0, 1e-10);
}
#[test]
fn math_function_roots_constant_zero_function() {
    let mut f = ConstantZeroFunction;
    let s = FunctionRoots::with_options(&mut f, -2.0, 2.0, 10, [1e-10, 1e-10, 1e-6], 0.0);
    assert!(s.is_done());
    assert!(s.is_all_null().unwrap());
}
#[test]
fn math_function_roots_no_roots_function() {
    let mut f = NoRootsFunction;
    let s = FunctionRoots::with_options(&mut f, -5.0, 5.0, 20, [1e-10, 1e-10, 0.0], 0.0);
    assert!(s.is_done());
    assert!(!s.is_all_null().unwrap());
    assert_eq!(s.number_of_solutions().unwrap(), 0);
}
#[test]
fn math_function_roots_quartic_four_roots() {
    let mut f = QuarticWithDerivative;
    let s = FunctionRoots::with_options(&mut f, 0.0, 5.0, 40, [1e-10, 1e-10, 0.0], 0.0);
    assert!(s.is_done());
    assert!(!s.is_all_null().unwrap());
    assert_eq!(s.number_of_solutions().unwrap(), 4);
    for (root, expected) in sorted(&s).into_iter().zip([1.0, 2.0, 3.0, 4.0]) {
        near(root, expected, 1e-8);
    }
}
#[test]
fn math_function_roots_custom_tolerances() {
    let mut f = QuadraticWithDerivative;
    for tolerance in [1e-3, 1e-12] {
        let s =
            FunctionRoots::with_options(&mut f, -5.0, 5.0, 20, [tolerance, tolerance, 0.0], 0.0);
        assert!(s.is_done());
        assert_eq!(s.number_of_solutions().unwrap(), 2);
    }
}
#[test]
fn math_function_roots_custom_sample_count() {
    let mut f = CubicWithDerivative;
    let s = FunctionRoots::with_options(&mut f, 0.0, 4.0, 5, [1e-8, 1e-8, 0.0], 0.0);
    assert!(s.is_done());
    assert!(s.number_of_solutions().unwrap() >= 1);
    let s = FunctionRoots::with_options(&mut f, 0.0, 4.0, 100, [1e-10, 1e-10, 0.0], 0.0);
    assert!(s.is_done());
    assert_eq!(s.number_of_solutions().unwrap(), 3);
}
#[test]
fn math_function_roots_state_number_access() {
    let mut f = QuadraticWithDerivative;
    let s = FunctionRoots::with_options(&mut f, -5.0, 5.0, 20, [1e-10, 1e-10, 0.0], 0.0);
    assert!(s.is_done());
    assert_eq!(s.number_of_solutions().unwrap(), 2);
    if s.number_of_solutions().unwrap() >= 1 {
        assert!(s.state_number(1).unwrap() >= 0);
    }
    if s.number_of_solutions().unwrap() >= 2 {
        assert!(s.state_number(2).unwrap() >= 0);
    }
    assert!(s.number_of_solutions().unwrap() >= 0);
}
#[test]
fn math_function_roots_shifted_target() {
    let mut f = QuadraticWithDerivative;
    let s = FunctionRoots::with_options(&mut f, -2.0, 2.0, 20, [1e-10, 1e-10, 0.0], -4.0);
    assert!(s.is_done());
    assert_eq!(s.number_of_solutions().unwrap(), 1);
    near(s.value(1).unwrap(), 0.0, 1e-8);
}
#[test]
fn math_function_roots_not_done_exceptions() {
    let mut f = QuadraticWithDerivative;
    let s = FunctionRoots::with_options(&mut f, -5.0, 5.0, 20, [1e-10, 1e-10, 0.0], 0.0);
    assert!(s.is_done());
    assert!(s.is_all_null().is_ok());
    assert!(s.number_of_solutions().is_ok());
    if s.number_of_solutions().unwrap() > 0 {
        assert!(s.value(1).is_ok());
    }
}
#[test]
fn math_function_roots_narrow_range() {
    let mut f = LinearWithDerivative;
    let s = FunctionRoots::with_options(&mut f, 1.9, 2.1, 10, [1e-10, 1e-10, 0.0], 0.0);
    assert!(s.is_done());
    assert_eq!(s.number_of_solutions().unwrap(), 1);
    near(s.value(1).unwrap(), 2.0, 1e-8);
}
#[test]
fn math_function_roots_root_at_boundary() {
    let mut f = LinearWithDerivative;
    for (lower, upper) in [(2.0, 4.0), (0.0, 2.0)] {
        let s = FunctionRoots::with_options(&mut f, lower, upper, 10, [1e-10, 1e-10, 0.0], 0.0);
        assert!(s.is_done());
        assert_eq!(s.number_of_solutions().unwrap(), 1);
    }
}
#[test]
fn math_function_roots_reversed_range() {
    let mut f = QuadraticWithDerivative;
    let s = FunctionRoots::with_options(&mut f, 5.0, -5.0, 20, [1e-10, 1e-10, 0.0], 0.0);
    assert!(s.is_done());
    assert_eq!(s.number_of_solutions().unwrap(), 2);
}
