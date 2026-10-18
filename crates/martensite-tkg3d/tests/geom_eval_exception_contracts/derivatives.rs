//! The exact derivative-order failures precede parameter evaluation and preserve state.
use super::{
    failures::record_failure,
    fixtures::{curve_fixtures, rational_trigonometric_patch, surface_fixtures},
};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::geom::{GeomCurve, GeomSurface};

fn derivative_failure(
    failures: &mut Vec<String>,
    result: Result<(), Failure>,
    expected: (&str, &str),
) {
    if let Some(failure) = record_failure(failures, result, expected) {
        assert!(failure.is_kind("Standard_DomainError"));
        assert!(failure.is_kind("Standard_Failure"));
    }
}

#[test]
fn all_curve_invalid_derivative_orders_preserve_exact_failure_and_state() {
    let mut failures = Vec::new();
    for (class, curve) in curve_fixtures() {
        check_curve_orders(curve.get().unwrap(), class, &mut failures);
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

fn curve_order_message(class: &str) -> String {
    let reason = if class == "GeomEval_TBezierCurve" {
        "derivative order must be >= 1"
    } else {
        "N must be >= 1"
    };
    format!("{class}::EvalDN: {reason}")
}

fn check_curve_orders(curve: &dyn GeomCurve, class: &str, failures: &mut Vec<String>) {
    let before = curve.eval_d1(0.25).unwrap();
    let message = curve_order_message(class);
    for order in [-3, -1, 0] {
        for parameter in [-3., 0.25, f64::NAN, f64::INFINITY] {
            curve_failure_pair(curve, parameter, order, &message, failures);
            assert_eq!(curve.eval_d1(0.25).unwrap(), before);
        }
    }
}

fn curve_failure_pair(
    curve: &dyn GeomCurve,
    parameter: f64,
    order: i32,
    message: &str,
    failures: &mut Vec<String>,
) {
    for result in [curve.eval_dn(parameter, order), curve.dn(parameter, order)] {
        derivative_failure(
            failures,
            result.map(|_| ()),
            ("Standard_RangeError", message),
        );
    }
}

#[test]
fn all_surface_invalid_derivative_orders_preserve_exact_failure_and_state() {
    let mut failures = Vec::new();
    for (class, surface) in surface_fixtures() {
        check_surface_orders(surface.get().unwrap(), class, &mut failures);
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

fn surface_order_contract(class: &str) -> (&'static str, String) {
    if class == "GeomEval_HyperboloidSurface" {
        return ("Geom_UndefinedDerivative", String::new());
    }
    let name = if matches!(
        class,
        "GeomEval_TBezierSurface" | "GeomEval_AHTBezierSurface"
    ) {
        "Standard_RangeError"
    } else {
        "Geom_UndefinedDerivative"
    };
    (name, format!("{class}::EvalDN: invalid derivative order"))
}

fn check_surface_orders(surface: &dyn GeomSurface, class: &str, failures: &mut Vec<String>) {
    let before = surface.eval_d1(0.25, 0.5).unwrap();
    let (kind, message) = surface_order_contract(class);
    for orders in [(0, 0), (-1, 0), (0, -1), (-1, 2), (2, -1)] {
        for parameter in [-3., 0.25, f64::NAN, f64::INFINITY] {
            surface_failure_pair(surface, parameter, orders, (kind, &message), failures);
            assert_eq!(surface.eval_d1(0.25, 0.5).unwrap(), before);
        }
    }
}

fn surface_failure_pair(
    surface: &dyn GeomSurface,
    parameter: f64,
    (nu, nv): (i32, i32),
    expected: (&str, &str),
    failures: &mut Vec<String>,
) {
    for result in [
        surface.eval_dn(parameter, parameter, nu, nv),
        surface.dn(parameter, parameter, nu, nv),
    ] {
        derivative_failure(failures, result.map(|_| ()), expected);
    }
}

#[test]
fn rational_trigonometric_high_derivatives_keep_refusal_and_state() {
    let surface = rational_trigonometric_patch();
    let mut failures = Vec::new();
    for orders in [(4, 0), (0, 4), (2, 2), (3, 2)] {
        check_rational_high_orders(surface.get().unwrap(), orders, &mut failures);
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

fn check_rational_high_orders(
    surface: &dyn GeomSurface,
    orders: (i32, i32),
    failures: &mut Vec<String>,
) {
    let before = surface.eval_d1(0.25, 0.5).unwrap();
    let message =
        "GeomEval_TBezierSurface::EvalDN: rational derivatives of order > 3 not implemented";
    for parameter in [-3., 0.25, f64::NAN, f64::INFINITY] {
        surface_failure_pair(
            surface,
            parameter,
            orders,
            ("Geom_UndefinedDerivative", message),
            failures,
        );
        assert_eq!(surface.eval_d1(0.25, 0.5).unwrap(), before);
    }
}
