//! Math integration tests.
#![allow(non_snake_case)]
use martensite_tkmath::math::{
    Function, GaussSingleIntegration as Gauss, KronrodSingleIntegration as Kronrod,
};
use std::f64::consts::PI;
fn near(a: f64, b: f64, tolerance: f64) {
    assert!((a - b).abs() <= tolerance, "{a} != {b}");
}
struct ConstantFunction(f64);
impl Function for ConstantFunction {
    fn value(&mut self, x: f64, value: &mut f64) -> bool {
        let _ = x;
        *value = self.0;
        true
    }
}
struct LinearFunction(f64, f64);
impl Function for LinearFunction {
    fn value(&mut self, x: f64, value: &mut f64) -> bool {
        let _ = x;
        *value = self.0 * x + self.1;
        true
    }
}
struct QuadraticFunction(f64, f64, f64);
impl Function for QuadraticFunction {
    fn value(&mut self, x: f64, value: &mut f64) -> bool {
        let _ = x;
        *value = self.0 * x * x + self.1 * x + self.2;
        true
    }
}
struct PowerFunction(i32);
impl Function for PowerFunction {
    fn value(&mut self, x: f64, value: &mut f64) -> bool {
        let _ = x;
        *value = martensite_cmath::pow(x, f64::from(self.0));
        true
    }
}
struct SineFunction;
impl Function for SineFunction {
    fn value(&mut self, x: f64, value: &mut f64) -> bool {
        let _ = x;
        *value = martensite_cmath::sin(x);
        true
    }
}
struct ExponentialFunction;
impl Function for ExponentialFunction {
    fn value(&mut self, x: f64, value: &mut f64) -> bool {
        let _ = x;
        *value = martensite_cmath::exp(x);
        true
    }
}
#[test]
fn math_integration_gauss_constant_function() {
    let mut f = ConstantFunction(5.0);
    let s = Gauss::new(&mut f, 0.0, 2.0, 4);
    assert!(s.is_done());
    near(s.value().unwrap(), 5.0 * (2.0 - 0.0), 1e-12);
}
#[test]
fn math_integration_gauss_linear_function() {
    let mut f = LinearFunction(2.0, 3.0);
    let s = Gauss::new(&mut f, 1.0, 4.0, 2);
    assert!(s.is_done());
    near(s.value().unwrap(), 24.0, 1e-12);
}
#[test]
fn math_integration_gauss_quadratic_function() {
    let mut f = QuadraticFunction(1.0, -2.0, 1.0);
    let s = Gauss::new(&mut f, 0.0, 2.0, 3);
    assert!(s.is_done());
    near(s.value().unwrap(), 2.0 / 3.0, 1e-12);
}
#[test]
fn math_integration_gauss_sine_function() {
    let mut f = SineFunction;
    let s = Gauss::new(&mut f, 0.0, PI, 10);
    assert!(s.is_done());
    near(s.value().unwrap(), 2.0, 1e-10);
}
#[test]
fn math_integration_gauss_exponential_function() {
    let mut f = ExponentialFunction;
    let s = Gauss::new(&mut f, 0.0, 1.0, 15);
    assert!(s.is_done());
    near(s.value().unwrap(), martensite_cmath::exp(1.0_f64) - 1.0, 1e-8);
}
#[test]
fn math_integration_gauss_different_orders() {
    let mut f = QuadraticFunction(1.0, 0.0, 0.0);
    for order in [2, 3, 5, 10, 20] {
        let s = Gauss::new(&mut f, 0.0, 1.0, order);
        assert!(s.is_done());
        near(
            s.value().unwrap(),
            1.0 / 3.0,
            if order >= 3 { 1e-12 } else { 1e-6 },
        );
    }
}
#[test]
fn math_integration_gauss_with_tolerance() {
    let mut f = QuadraticFunction(1.0, 0.0, 0.0);
    let tolerance: f64 = 1e-10;
    let s = Gauss::with_tolerance(&mut f, 0.0, 1.0, 5, tolerance);
    assert!(s.is_done());
    near(s.value().unwrap(), 1.0 / 3.0, tolerance * 10.0);
}
#[test]
fn math_integration_gauss_negative_interval() {
    let mut f = LinearFunction(1.0, 0.0);
    let s = Gauss::new(&mut f, -2.0, 2.0, 3);
    assert!(s.is_done());
    near(s.value().unwrap(), 0.0, 1e-12);
}
#[test]
fn math_integration_kronrod_constant_function() {
    let mut f = ConstantFunction(3.0);
    let s = Kronrod::new(&mut f, 1.0, 5.0, 15);
    assert!(s.is_done());
    near(s.value().unwrap(), 3.0 * (5.0 - 1.0), 1e-12);
}
#[test]
fn math_integration_kronrod_quadratic_function() {
    let mut f = QuadraticFunction(2.0, -1.0, 3.0);
    let s = Kronrod::new(&mut f, 0.0, 1.0, 15);
    assert!(s.is_done());
    near(s.value().unwrap(), 19.0 / 6.0, 1e-10);
}
#[test]
fn math_integration_kronrod_sine_function() {
    let mut f = SineFunction;
    let s = Kronrod::new(&mut f, 0.0, PI / 2.0, 21);
    assert!(s.is_done());
    near(s.value().unwrap(), 1.0, 1e-12);
}
#[test]
fn math_integration_gauss_vs_kronrod_comparison() {
    let mut f = PowerFunction(4);
    let gauss = Gauss::new(&mut f, 0.0, 2.0, 10);
    assert!(gauss.is_done());
    let kronrod = Kronrod::new(&mut f, 0.0, 2.0, 21);
    assert!(kronrod.is_done());
    near(gauss.value().unwrap(), 32.0 / 5.0, 1e-10);
    near(kronrod.value().unwrap(), 32.0 / 5.0, 1e-10);
    near(gauss.value().unwrap(), kronrod.value().unwrap(), 1e-8);
}
#[test]
fn math_integration_default_constructor_and_perform() {
    let _unused = Gauss::default();
    let mut f = LinearFunction(1.0, 2.0);
    let s = Gauss::new(&mut f, 0.0, 3.0, 5);
    assert!(s.is_done());
    near(s.value().unwrap(), 10.5, 1e-12);
}
#[test]
fn math_integration_zero_length_interval() {
    let mut f = ConstantFunction(1.0);
    let s = Gauss::new(&mut f, 5.0, 5.0, 5);
    if s.is_done() {
        near(s.value().unwrap(), 0.0, 1e-7);
    }
}
#[test]
fn math_integration_reverse_interval() {
    let mut f = LinearFunction(1.0, 0.0);
    let s = Gauss::new(&mut f, 2.0, 0.0, 5);
    if s.is_done() {
        near(s.value().unwrap(), -2.0, 1e-12);
    }
}
#[test]
fn math_integration_high_order_integration() {
    let mut f = PowerFunction(10);
    let s = Gauss::new(&mut f, 0.0, 1.0, 30);
    assert!(s.is_done());
    near(s.value().unwrap(), 1.0 / 11.0, 1e-12);
}
