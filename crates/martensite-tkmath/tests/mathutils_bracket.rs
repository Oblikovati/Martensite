//! Mathutils bracket tests.
#![allow(non_snake_case)]
use martensite_tkmath::math::{MultipleVarFunction, MultipleVarFunctionWithGradient, Vector};
use martensite_tkmath::math_utils::*;
fn quadratic(x: f64, out: &mut f64) -> bool {
    *out = (x - 2.0) * (x - 2.0);
    true
}
fn cached_quadratic(x: f64, out: &mut f64) -> bool {
    if x.abs() < 1e-15 || (x - 1.0).abs() < 1e-15 {
        return false;
    }
    quadratic(x, out)
}
fn near(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() <= tol, "{a} != {b}");
}
struct DecreasingLinear;
impl MultipleVarFunction for DecreasingLinear {
    fn variables(&self) -> i32 {
        1
    }
    fn value(&mut self, x: &Vector<'_>, out: &mut f64) -> bool {
        *out = -x.at(0).unwrap();
        true
    }
}
impl MultipleVarFunctionWithGradient for DecreasingLinear {
    fn gradient(&mut self, _: &Vector<'_>, out: &mut Vector<'_>) -> bool {
        *out.change_at(0).unwrap() = -1.0;
        true
    }
    fn values(&mut self, x: &Vector<'_>, value: &mut f64, g: &mut Vector<'_>) -> bool {
        self.value(x, value) && self.gradient(x, g)
    }
}
fn distant(x: &Vector<'_>, out: &mut f64) -> bool {
    let dx: f64 = x.at(0).unwrap() - 25.0;
    *out = dx * dx;
    true
}
fn distant_failing(x: &Vector<'_>, out: &mut f64) -> bool {
    if *x.at(0).unwrap() > 3.0 {
        return false;
    }
    distant(x, out)
}

#[test]
fn math_utils_bracket_bracket_minimum_with_limits_succeeds() {
    let options = MinBracketOptions {
        max_iterations: 50,
        use_limits: true,
        left_limit: 0.0,
        right_limit: 5.0,
        ..Default::default()
    };
    let result = bracket_minimum(&mut quadratic, 0.0, 1.0, &options);
    assert!(result.is_valid);
    assert!(result.a >= options.left_limit);
    assert!(result.c <= options.right_limit);
    assert!(result.fb < result.fa);
    assert!(result.fb < result.fc);
}

#[test]
fn math_utils_bracket_bracket_minimum_with_restrictive_limits_fails() {
    let options = MinBracketOptions {
        max_iterations: 50,
        use_limits: true,
        left_limit: 0.0,
        right_limit: 1.0,
        ..Default::default()
    };
    assert!(!bracket_minimum(&mut quadratic, 0.0, 0.5, &options).is_valid);
}

#[test]
fn math_utils_bracket_bracket_minimum_uses_precomputed_endpoint_values() {
    let options = MinBracketOptions {
        max_iterations: 50,
        has_fa: true,
        has_fb: true,
        fa: 4.0,
        fb: 1.0,
        ..Default::default()
    };
    let result = bracket_minimum(&mut cached_quadratic, 0.0, 1.0, &options);
    assert!(result.is_valid);
    assert!(result.fb < result.fa);
    assert!(result.fb < result.fc);
}

#[test]
fn math_utils_core_norm_and_dot_product_handle_scale_extremes() {
    let mut large = Vector::with_bounds(1, 2).unwrap();
    large[1] = 3e300;
    large[2] = 4e300;
    double_eq(norm(&large), 5e300);
    let mut small = Vector::with_bounds(1, 2).unwrap();
    small[1] = 3e-300;
    small[2] = 4e-300;
    double_eq(norm(&small), 5e-300);
    let mut a = Vector::with_bounds(1, 3).unwrap();
    let b = Vector::filled(-2, 0, 1.0).unwrap();
    a[1] = 1e16;
    a[2] = 1.0;
    a[3] = -1e16;
    assert_eq!(dot_product(&a, &b), 1.0);
}

#[test]
fn math_utils_core_rejects_non_finite_values_and_invalid_configuration() {
    assert!(!is_equal(f64::INFINITY, f64::INFINITY, ZERO_TOL));
    assert!(!is_within_tolerance(f64::NAN, 0.0, 1.0, 1.0));
    assert!(!is_within_tolerance(-f64::MAX, f64::MAX, 0.0, 2.0));
    let config = Config {
        relative_tolerance: f64::NAN,
        ..Default::default()
    };
    assert!(!config.is_valid());
    assert!(!is_converged(1.0, 1.0, 0.0, &config));
    let reversed = Domain1D::new(2.0, -2.0);
    assert!(!reversed.is_valid());
    assert!(!reversed.contains(0.0, 0.0));
}

#[test]
fn math_utils_convergence_combines_absolute_and_relative_tolerance() {
    let config = Config {
        x_tolerance: 1e-12,
        relative_tolerance: 1e-8,
        ..Default::default()
    };
    assert!(is_converged(1e12, 1e12 + 1.0, 1.0, &config));
    assert!(!is_converged(0.0, 1e-6, 1.0, &config));
}

#[test]
fn math_utils_deriv_adaptive_gradient_uses_representable_per_variable_step() {
    let mut f = make_vector(1, |x: &Vector<'_>, out: &mut f64| {
        *out = *x.at(0).unwrap();
        true
    });
    let mut x = Vector::filled(5, 5, 1e300).unwrap();
    let mut gradient = Vector::with_bounds(-2, -2).unwrap();
    assert!(numerical_gradient_adaptive(
        &mut f,
        &mut x,
        &mut gradient,
        1e-8
    ));
    near(gradient[-2], 1.0, 1e-8);
    assert_eq!(x[5], 1e300);
    assert!(!numerical_gradient_adaptive(
        &mut f,
        &mut x,
        &mut gradient,
        0.0
    ));
}

#[test]
fn math_utils_deriv_callback_failure_restores_input() {
    let mut f = make_vector(1, |_: &Vector<'_>, _: &mut f64| false);
    let mut x = Vector::filled(3, 3, 7.0).unwrap();
    let mut gradient = Vector::with_bounds(3, 3).unwrap();
    assert!(!numerical_gradient_adaptive(
        &mut f,
        &mut x,
        &mut gradient,
        1e-8
    ));
    assert_eq!(x[3], 7.0);
}

#[test]
fn math_utils_bracket_reports_invalid_and_callback_failures() {
    assert_eq!(
        bracket_root(&mut quadratic, f64::NAN, 1.0, 50).status,
        Status::InvalidInput
    );
    let failure = bracket_root(&mut |_: f64, _: &mut f64| false, -1.0, 1.0, 50);
    assert!(!failure.is_valid);
    assert_eq!(failure.status, Status::CallbackError);
    assert_eq!(
        bracket_root(
            &mut |_: f64, out: &mut f64| {
                *out = f64::NAN;
                true
            },
            -1.0,
            1.0,
            50
        )
        .status,
        Status::NumericalError
    );
}

#[test]
fn math_utils_bracket_supports_arbitrary_finite_limits() {
    let options = MinBracketOptions {
        use_limits: true,
        left_limit: -7.0,
        right_limit: 11.0,
        ..Default::default()
    };
    let result = bracket_minimum(&mut quadratic, 9.0, -3.0, &options);
    assert!(result.is_valid);
    assert!(result.a < result.b);
    assert!(result.b < result.c);
    assert!(result.a >= options.left_limit);
    assert!(result.c <= options.right_limit);
}

#[test]
fn math_utils_line_search_wolfe_exhaustion_never_claims_success() {
    let x = Vector::filled(4, 4, 0.0).unwrap();
    let direction = Vector::filled(-1, -1, 1.0).unwrap();
    let gradient = Vector::filled(8, 8, -1.0).unwrap();
    let result = wolfe_search(
        &mut DecreasingLinear,
        &x,
        &direction,
        &gradient,
        0.0,
        [1.0, 1e-4, 0.9],
        2,
    );
    assert!(!result.is_valid);
    assert_eq!(result.status, Status::MaxIterations);
    assert_eq!(result.f_new, -result.alpha);
}

#[test]
fn math_utils_line_search_rejects_invalid_config_and_propagates_callback_failure() {
    let x = Vector::filled(1, 1, 0.0).unwrap();
    let direction = Vector::filled(1, 1, 1.0).unwrap();
    let gradient = Vector::filled(1, 1, -1.0).unwrap();
    assert_eq!(
        wolfe_search(
            &mut DecreasingLinear,
            &x,
            &direction,
            &gradient,
            0.0,
            [-1.0, 1e-4, 0.9],
            20
        )
        .status,
        Status::InvalidInput
    );
    let mut failing = make_vector_with_gradient(
        1,
        |_: &Vector<'_>, _: &mut f64| false,
        |_: &Vector<'_>, _: &mut Vector<'_>| true,
    );
    assert_eq!(
        wolfe_search(
            &mut failing,
            &x,
            &direction,
            &gradient,
            0.0,
            [1.0, 1e-4, 0.9],
            20
        )
        .status,
        Status::CallbackError
    );
}

#[test]
fn math_utils_line_search_exact_search_finds_minimum_beyond_alpha_ten() {
    let mut f = make_vector(1, distant);
    let x = Vector::filled(0, 0, 0.0).unwrap();
    let direction = Vector::filled(0, 0, 1.0).unwrap();
    let result = exact_line_search(&mut f, &x, &direction, 100.0, 1e-10, 100);
    assert!(result.is_valid);
    assert_eq!(result.status, Status::Ok);
    near(result.alpha, 25.0, 1e-6);
    near(result.f_new, 0.0, 1e-12);
}

#[test]
fn math_utils_line_search_exact_search_propagates_callback_failure_during_bracketing() {
    let mut f = make_vector(1, distant_failing);
    let x = Vector::filled(0, 0, 0.0).unwrap();
    let direction = Vector::filled(0, 0, 1.0).unwrap();
    let result = exact_line_search(&mut f, &x, &direction, 10.0, 1e-10, 100);
    assert!(!result.is_valid);
    assert_eq!(result.status, Status::CallbackError);
    assert!(result.alpha >= 0.0);
    assert!(result.alpha <= 3.0);
    assert_eq!(result.f_new, (result.alpha - 25.0) * (result.alpha - 25.0));
}

#[test]
fn math_utils_line_search_four_argument_call_retains_alpha_limit_meaning() {
    let mut f = make_vector(1, distant);
    let x = Vector::filled(0, 0, 0.0).unwrap();
    let direction = Vector::filled(0, 0, 1.0).unwrap();
    // Rust requires the default tolerance and iteration limit explicitly.
    let result = exact_line_search(&mut f, &x, &direction, 10.0, 1e-6, 100);
    assert!(result.is_valid);
    assert!(result.alpha.abs() <= 10.0);
    assert!(result.f_new > 0.0);
}

#[test]
fn math_utils_poly_duplicate_policy_is_scale_relative_and_rejects_non_finite_roots() {
    assert_eq!(remove_duplicate_roots(&mut [1e12, 1e12 + 1.0], 1e-10), 1);
    assert_eq!(remove_duplicate_roots(&mut [0.0, 1e-9], 1e-10), 2);
    assert_eq!(remove_duplicate_roots(&mut [0.0, f64::INFINITY], 1e-10), 0);
    assert_eq!(remove_duplicate_roots(&mut [f64::NAN, 1.0], 1e-10), 0);
}

// Doubles are equal within four units in the last place.
fn double_eq(a: f64, b: f64) {
    assert!(!a.is_nan() && !b.is_nan());
    assert!(ordered_bits(a).abs_diff(ordered_bits(b)) <= 4, "{a} != {b}");
}
fn ordered_bits(value: f64) -> u64 {
    let bits: u64 = value.to_bits();
    if bits >> 63 != 0 {
        (!bits).wrapping_add(1)
    } else {
        bits | (1 << 63)
    }
}
