use super::*;

#[test]
fn dimension_parameter_defaults_leave_each_existing_attribute_unselected() {
    let params = DimensionParams::default();
    assert!(params.flyout.is_none());
    assert!(params.style.arrow_length.is_none());
    assert!(params.style.arrow.is_none());
    assert!(params.style.label.is_none());
}
