use super::*;
use martensite_tkmath::gp::{Axis3, Plane3, Point3, xoy};

#[test]
fn unchanged_dimension_style_preserves_seeded_layout_and_visibility() {
    let aspect = Prs3dDimensionAspect::new().unwrap();
    aspect.set_arrow_orientation(Orientation::External);
    aspect.set_text_horizontal_position(HPos::Right);
    aspect.set_text_vertical_position(VPos::Above);
    aspect.make_text_shaded(true);
    aspect.make_units_displayed(true);
    DimensionStyle::default().apply(&aspect);
    assert_eq!(aspect.arrow_orientation(), Orientation::External);
    assert_eq!(aspect.text_horizontal_position(), HPos::Right);
    assert_eq!(aspect.text_vertical_position(), VPos::Above);
    assert!(aspect.is_text_shaded() && aspect.is_units_displayed());
}

#[test]
fn explicit_dimension_style_changes_only_the_requested_parts() {
    let aspect = Prs3dDimensionAspect::new().unwrap();
    let style = DimensionStyle {
        arrow: Some(Orientation::Internal),
        label: Some(HPos::Left),
        label_vertical: Some(VPos::Below),
        text_shaded: Some(true),
        show_units: Some(true),
        ..Default::default()
    };
    style.apply(&aspect);
    assert_eq!(aspect.arrow_orientation(), Orientation::Internal);
    assert_eq!(aspect.text_horizontal_position(), HPos::Left);
    assert_eq!(aspect.text_vertical_position(), VPos::Below);
    assert!(aspect.is_text_shaded() && aspect.is_units_displayed());
    assert!(!aspect.is_text_3d() && !aspect.is_arrows_3d());
}

#[test]
fn explicit_dimension_style_updates_arrow_geometry_and_text_height() {
    let aspect = Prs3dDimensionAspect::new().unwrap();
    let style = DimensionStyle {
        arrow_length: Some(0.75),
        zoomable_arrow: Some(false),
        text_height: Some(15.0),
        ..Default::default()
    };
    style.apply(&aspect);
    let arrow = aspect.arrow_aspect();
    assert_eq!(arrow.get().unwrap().length().to_bits(), 0.75_f64.to_bits());
    assert!(!arrow.get().unwrap().is_zoomable());
    assert_eq!(
        aspect.text_aspect().get().unwrap().height().to_bits(),
        15_f64.to_bits()
    );
}

#[test]
fn unset_dimension_value_changes_preserve_a_seeded_custom_number() {
    let dimension = PrsDimLengthDimension::new();
    let value = dimension.get().unwrap();
    value.set_custom_value(19.5);
    value.set_flyout(-7.25);
    DimensionParams::default().apply_value(value);
    assert_eq!(value.get_value().to_bits(), 19.5_f64.to_bits());
    assert_eq!(value.get_flyout().to_bits(), (-7.25_f64).to_bits());
}

#[test]
fn explicit_custom_number_follows_the_requested_auto_value_reset() {
    let dimension = PrsDimLengthDimension::new();
    let value = dimension.get().unwrap();
    value.set_custom_value(19.5);
    let params = DimensionParams {
        auto_value: true,
        value: Some(12.75),
        flyout: Some(4.5),
        ..Default::default()
    };
    params.apply_value(value);
    assert_eq!(value.get_value().to_bits(), 12.75_f64.to_bits());
    assert_eq!(value.get_flyout().to_bits(), 4.5_f64.to_bits());
}

#[test]
fn custom_text_is_applied_after_the_computed_and_numeric_requests() {
    let first = Point3::new(0., 0., 0.);
    let second = Point3::new(3., 4., 0.);
    let plane = Plane3::from_position(Axis3::from_axis2(xoy()));
    let dimension = PrsDimLengthDimension::from_points(&first, &second, &plane).unwrap();
    let value = dimension.get().unwrap();
    let params = DimensionParams {
        auto_value: true,
        value: Some(12.75),
        text_value: Some("owned label"),
        ..Default::default()
    };
    params.apply_value(value);
    assert_eq!(
        value.get_custom_value(),
        ExtendedString::from_utf8("owned label")
    );
    assert_eq!(value.get_value().to_bits(), 5_f64.to_bits());
}

fn black_3d_dimension() -> Handle<PrsDimLengthDimension> {
    let aspect = Prs3dDimensionAspect::new().unwrap();
    DimensionStyle {
        text_3d: Some(true),
        color: Some(martensite_tkernel::quantity::Color::named(
            martensite_tkernel::quantity::NamedColor::BLACK,
        )),
        ..Default::default()
    }
    .apply(&aspect);
    assert!(aspect.is_text_3d());
    let dimension = PrsDimLengthDimension::new();
    dimension.set_dimension_aspect(&Handle::new(aspect));
    dimension
}
fn requested_arrow_angle(angle: f64) -> DimensionParams {
    DimensionParams {
        arrow_angle: Some(angle),
        ..Default::default()
    }
}
#[test]
fn explicit_three_dimensional_text_color_and_arrow_angle_preserve_native_values() {
    let dimension = black_3d_dimension();
    #[allow(clippy::approx_constant)] // The regression sets this exact arrow angle.
    let angle = 3.14 * 12. / 180.;
    requested_arrow_angle(angle)
        .apply_arrow_angle(dimension.get().unwrap())
        .unwrap();
    assert_eq!(
        dimension
            .dimension_aspect()
            .arrow_aspect()
            .angle()
            .to_bits(),
        angle.to_bits()
    );
    assert!(
        requested_arrow_angle(0.)
            .apply_arrow_angle(dimension.get().unwrap())
            .is_err()
    );
}
#[test]
fn default_arrow_angle_keeps_seeded_angle_without_mutation() {
    let dimension = black_3d_dimension();
    dimension
        .dimension_aspect()
        .arrow_aspect()
        .set_angle(0.7)
        .unwrap();
    DimensionParams::default()
        .apply_arrow_angle(dimension.get().unwrap())
        .unwrap();
    assert_eq!(
        dimension
            .dimension_aspect()
            .arrow_aspect()
            .angle()
            .to_bits(),
        0.7_f64.to_bits()
    );
}
