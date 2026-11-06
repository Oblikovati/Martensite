//! The area check of the plate: the mass as the report prints it, six significant digits.
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::storage::format_general;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_gprop::BRepGProp;

/// The surface area within one percent of `expected`; `check_area(&shape, 2.92409)`.
pub fn check_area(shape: &Shape, expected: f64) {
    let mut properties = GProps::default();
    BRepGProp::surface_properties_eps(shape, &mut properties, 1.0e-4, false).unwrap();
    let text = format_general(properties.mass(), 6);
    let actual: f64 = text.parse().unwrap_or_else(|error| {
        panic!("area rendered as {text:?}: expected numeric text: {error}")
    });
    assert_ne!(actual, 0., "checkprops: expected nonzero area");
    assert!(
        ((expected - actual) / expected).abs() <= 0.01,
        "checkprops: area {actual}, expected {expected} within 1 percent"
    );
}
