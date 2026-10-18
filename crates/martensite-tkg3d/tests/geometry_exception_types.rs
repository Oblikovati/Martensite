//! retained most-derived exception names survive construction and copying through a base value.
use martensite_tkernel::standard::Failure;
use martensite_tkg2d::geom2d::errors as plane_errors;
use martensite_tkg3d::geom::errors as space_errors;

type GeometryExceptionFactory = fn(&str) -> Failure;

#[test]
fn geometry_exception_names_preserve_base_dispatch() {
    let factories: [GeometryExceptionFactory; 4] = [
        plane_errors::undefined_derivative,
        plane_errors::undefined_value,
        space_errors::undefined_derivative,
        space_errors::undefined_value,
    ];
    let observed: String = factories.into_iter().map(exception_type_receipt).collect();
    assert_eq!(
        observed,
        include_str!("fixtures/geometry-failures.txt")
    );
}

/// Undefined geometry failures preserve domain ancestry across base dispatch.
#[test]
fn geom_exceptions_are_caught_by_their_domain_error_base() {
    let failures = [
        (
            "Geom_UndefinedDerivative",
            space_errors::undefined_derivative("m"),
        ),
        (
            "Geom_UndefinedDerivative",
            space_errors::undefined_derivative_with_stack("m", "trace"),
        ),
        ("Geom_UndefinedValue", space_errors::undefined_value("m")),
        (
            "Geom_UndefinedValue",
            space_errors::undefined_value_with_stack("m", "trace"),
        ),
    ];
    for (name, failure) in &failures {
        assert_eq!(failure.exception_type(), *name);
        assert!(failure.is_kind(name));
        assert!(failure.is_kind("Standard_DomainError"));
        assert!(failure.is_kind("Standard_Failure"));
        assert!(!failure.is_kind("Standard_RangeError"));
        assert!(failure.clone().is_kind("Standard_DomainError"));
        assert_eq!(failure.message(), "m");
    }
    assert_eq!(failures[1].1.stack_string(), "trace");
    assert_eq!(failures[3].1.stack_string(), "trace");
}

fn exception_type_receipt(factory: GeometryExceptionFactory) -> String {
    let original = factory("geometry probe");
    let copied = original.clone();
    format!(
        "{} {}\n",
        original.exception_type(),
        copied.exception_type()
    )
}
