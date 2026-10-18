//! Undefined values and derivatives retain domain-error ancestry.
use martensite_tkernel::standard::Failure;
use martensite_tkg2d::geom2d::errors::{
    undefined_derivative, undefined_derivative_with_stack, undefined_value,
    undefined_value_with_stack,
};
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dLine};
use martensite_tkmath::gp::{Direction2, Point2};

fn assert_domain_error_kind(failure: &Failure, name: &str) {
    assert_eq!(failure.exception_type(), name);
    assert!(failure.is_kind(name));
    assert!(failure.is_kind("Standard_DomainError"));
    assert!(failure.is_kind("Standard_Failure"));
    assert!(!failure.is_kind("Standard_RangeError"));
    assert!(!failure.is_kind("Standard_ConstructionError"));
}

#[test]
fn undefined_derivative_is_caught_by_its_domain_error_base() {
    let constructed = undefined_derivative("m");
    let stacked = undefined_derivative_with_stack("m", "trace");
    for failure in [&constructed, &stacked, &constructed.clone()] {
        assert_domain_error_kind(failure, "Geom2d_UndefinedDerivative");
    }
    assert_eq!(stacked.message(), "m");
    assert_eq!(stacked.stack_string(), "trace");
}

#[test]
fn undefined_value_is_caught_by_its_domain_error_base() {
    let constructed = undefined_value("m");
    let stacked = undefined_value_with_stack("m", "trace");
    for failure in [&constructed, &stacked, &constructed.clone()] {
        assert_domain_error_kind(failure, "Geom2d_UndefinedValue");
    }
    assert_eq!(stacked.message(), "m");
    assert_eq!(stacked.stack_string(), "trace");
}

/// Geom2dLine::DN raises Geom2dUndefinedDerivative for N < 1; a domain-error handler catches it.
#[test]
fn raised_line_derivative_failure_carries_the_domain_error_base() {
    let direction = Direction2::new(1.0, 0.0).unwrap();
    let line = Geom2dLine::from_point(Point2::new(0.0, 0.0), direction);
    let raised = line.dn(0.0, 0).unwrap_err();
    assert_domain_error_kind(&raised, "Geom2d_UndefinedDerivative");
}
