//! Canonical source property measurements share one printed-mass acceptance driver.
#[path = "removefeatures_cases/mass_text.rs"]
pub(crate) mod mass_text;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::storage::format_general;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_gprop::BRepGProp;

fn measured_property(shape: &Shape, linear: bool) -> (GProps, Option<f64>) {
    let mut properties = GProps::default();
    let error = if linear {
        BRepGProp::linear_properties(shape, &mut properties, false, false).unwrap();
        None
    } else {
        Some(BRepGProp::surface_properties_eps(shape, &mut properties, 1.0e-4, false).unwrap())
    };
    (properties, error)
}

fn relative_property_error(actual: f64, expected: f64, linear: bool) -> f64 {
    if linear {
        return (expected - actual).abs() / expected;
    }
    ((expected - actual) / expected).abs()
}

/// Preserve the original printed area or length predicate; `check_property(&shape, 117.6, true)`.
/// A linear request keeps lprops defaults; a surface request keeps adaptive epsilon1e-4.
#[allow(dead_code)] // Observed-only original hosts retain the unchanged compatibility entry.
pub fn check_property(shape: &Shape, expected: f64, linear: bool) {
    check_property_observed(shape, expected, linear, |_| {});
}

/// Observe the actual single measurement before grading, returning the checked printed Mass.
/// Example: `check_property_observed(&shape, 52., false, |properties| publish_axes(properties));`.
pub fn check_property_observed(
    shape: &Shape,
    expected: f64,
    linear: bool,
    observe: impl FnOnce(&GProps),
) -> f64 {
    let actual = measured_property_observed(shape, linear, observe);
    assert_printed_property(actual, expected, linear);
    actual
}

fn assert_printed_property(actual: f64, expected: f64, linear: bool) {
    let unit = if linear { "length" } else { "area" };
    assert_ne!(actual, 0., "checkprops: expected nonzero {unit}");
    assert!(
        relative_property_error(actual, expected, linear) <= 0.01,
        "checkprops: {unit} {actual}, expected {expected} within 1 percent"
    );
}

fn measured_property_observed(shape: &Shape, linear: bool, observe: impl FnOnce(&GProps)) -> f64 {
    // Existing callbacks intentionally discard the diagnostic without changing their timing.
    measured_property_error_observed(shape, linear, |properties, _| observe(properties))
}

/// Measure and publish the same properties without adding a caller predicate.
/// Example: `measured_property_error_observed(shape, true, |properties, _| print_axes(properties))`.
pub(crate) fn measured_property_error_observed(
    shape: &Shape,
    linear: bool,
    observe: impl FnOnce(&GProps, Option<f64>),
) -> f64 {
    let (properties, error) = measured_property(shape, linear);
    observe(&properties, error);
    // The source publishes axes before checkprops parses its six-significant-digit Mass.
    mass_text::draw_mass(properties.mass())
}

/// Retain original soft diagnostics and subsequent commands; `check_property_soft_observed(shape, 37.721, true, observe)`.
#[allow(dead_code)] // Only original known-problem callers select the soft diagnostic entry.
pub fn check_property_soft_observed(
    shape: &Shape,
    expected: f64,
    linear: bool,
    observe: impl FnOnce(&GProps),
) -> f64 {
    check_property_soft_error_observed(shape, expected, linear, |properties, _| {
        observe(properties);
    })
}

/// Observe the same measurement's diagnostic before grading; linear requests have no error estimate.
/// Example: `check_property_soft_error_observed(shape, 52., false, |properties, error| print_error(error));`.
#[allow(dead_code)] // Only callers printing the original integration diagnostic select this entry.
pub fn check_property_soft_error_observed(
    shape: &Shape,
    expected: f64,
    linear: bool,
    observe: impl FnOnce(&GProps, Option<f64>),
) -> f64 {
    let actual = measured_property_error_observed(shape, linear, observe);
    report_soft_property(actual, expected, linear);
    actual
}

fn report_soft_property(actual: f64, expected: f64, linear: bool) {
    let unit = if linear { "length" } else { "area" };
    report_soft_property_named(actual, expected, linear, unit);
}

/// Grade an already printed volume without remeasuring; `report_soft_volume(24., 24.)`.
#[allow(dead_code)] // Linear/surface-only source hosts do not select the additive volume entry.
pub fn report_soft_volume(actual: f64, expected: f64) {
    // Volume shares the source surface absolute-relative rule, while retaining its own label.
    report_soft_property_named(actual, expected, false, "volume");
}

fn report_soft_property_named(actual: f64, expected: f64, linear: bool, unit: &str) {
    if actual == 0. {
        println!("Error : The command is not valid. The {unit} is 0.");
    }
    let mismatch = if expected == 0. {
        actual != 0.
    } else {
        relative_property_error(actual, expected, linear) > 0.01
    };
    if mismatch {
        println!(
            "Error : The {unit} of result shape is {}, expected {expected}",
            format_general(actual, 6)
        );
    }
}

/// Retain the distinct printed-empty surface contract and same measurement callback.
/// Example: `check_empty_surface_soft_error_observed(shape, |properties, error| publish(properties, error));`.
#[allow(dead_code)] // Nonempty-only original callers retain their existing property entry points.
pub fn check_empty_surface_soft_error_observed(
    shape: &Shape,
    observe: impl FnOnce(&GProps, Option<f64>),
) -> f64 {
    let actual = measured_property_error_observed(shape, false, observe);
    if let Some(diagnostic) = empty_surface_diagnostic(actual) {
        println!("{diagnostic}");
    }
    actual
}

/// Preserve the exact empty predicate without inventing a small-mass threshold.
/// Example: `empty_surface_diagnostic(0.).is_none()`; a nonzero printed mass yields the source Error.
#[allow(dead_code)] // Scalar controls and explicit empty originals use this additive predicate.
pub(crate) fn empty_surface_diagnostic(actual: f64) -> Option<String> {
    if actual == 0. {
        return None;
    }
    Some(format!(
        "Error : The command is not valid. The area is {}",
        format_general(actual, 6)
    ))
}
