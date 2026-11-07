//! The classes of IgesDimen: what their initialization keeps and their round through a file.
#![allow(
    unused_mut,
    unused_variables,
    unused_assignments,
    clippy::needless_late_init,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::manual_range_contains,
    clippy::too_many_arguments,
    clippy::collapsible_else_if,
    clippy::needless_return,
    reason = "the members keep the order and the locals of the record they read"
)]
use crate::iges_test_samples::{
    assert_entity_round, sample_iges_dimen_angular_dimension, sample_iges_dimen_basic_dimension,
    sample_iges_dimen_center_line, sample_iges_dimen_curve_dimension,
    sample_iges_dimen_diameter_dimension, sample_iges_dimen_dimension_display_data,
    sample_iges_dimen_dimension_tolerance, sample_iges_dimen_dimension_units,
    sample_iges_dimen_dimensioned_geometry, sample_iges_dimen_flag_note,
    sample_iges_dimen_general_label, sample_iges_dimen_general_note,
    sample_iges_dimen_general_symbol, sample_iges_dimen_leader_arrow,
    sample_iges_dimen_linear_dimension, sample_iges_dimen_new_dimensioned_geometry,
    sample_iges_dimen_new_general_note, sample_iges_dimen_ordinate_dimension,
    sample_iges_dimen_point_dimension, sample_iges_dimen_radius_dimension,
    sample_iges_dimen_section, sample_iges_dimen_sectioned_area, sample_iges_dimen_witness_line,
};
use martensite_tkernel::standard::Failure;

#[test]
fn class_angular_dimension() -> Result<(), Failure> {
    let object = sample_iges_dimen_angular_dimension()?;
    assert_eq!(object.radius(), 5.5);
    assert_entity_round(&object, "IgesDimenAngularDimension")?;
    Ok(())
}

#[test]
fn class_basic_dimension() -> Result<(), Failure> {
    let object = sample_iges_dimen_basic_dimension()?;
    assert_eq!(object.nb_property_values(), 8);
    assert_entity_round(&object, "IgesDimenBasicDimension")?;
    Ok(())
}

#[test]
fn class_center_line() -> Result<(), Failure> {
    let object = sample_iges_dimen_center_line()?;
    assert_eq!(object.datatype(), 2);
    assert_eq!(object.z_displacement(), 2.5);
    assert_entity_round(&object, "IgesDimenCenterLine")?;
    Ok(())
}

#[test]
fn class_curve_dimension() -> Result<(), Failure> {
    let object = sample_iges_dimen_curve_dimension()?;
    assert_entity_round(&object, "IgesDimenCurveDimension")?;
    Ok(())
}

#[test]
fn class_diameter_dimension() -> Result<(), Failure> {
    let object = sample_iges_dimen_diameter_dimension()?;
    assert_entity_round(&object, "IgesDimenDiameterDimension")?;
    Ok(())
}

#[test]
fn class_dimension_display_data() -> Result<(), Failure> {
    let object = sample_iges_dimen_dimension_display_data()?;
    assert_eq!(object.nb_property_values(), 2);
    assert_eq!(object.dimension_type(), 2);
    assert_eq!(object.label_position(), 2);
    assert_eq!(object.character_set(), 2);
    assert_eq!(object.decimal_symbol(), 2);
    assert_eq!(object.witness_line_angle(), 7.5);
    assert_eq!(object.text_alignment(), 2);
    assert_eq!(object.text_level(), 2);
    assert_eq!(object.text_placement(), 2);
    assert_eq!(object.arrow_head_orientation(), 2);
    assert_eq!(object.initial_value(), 12.5);
    assert_entity_round(&object, "IgesDimenDimensionDisplayData")?;
    Ok(())
}

#[test]
fn class_dimension_tolerance() -> Result<(), Failure> {
    let object = sample_iges_dimen_dimension_tolerance()?;
    assert_eq!(object.nb_property_values(), 8);
    assert_eq!(object.secondary_tolerance_flag(), 2);
    assert_eq!(object.tolerance_type(), 2);
    assert_eq!(object.tolerance_placement_flag(), 2);
    assert_eq!(object.upper_tolerance(), 5.5);
    assert_eq!(object.lower_tolerance(), 6.5);
    assert!(object.sign_suppression_flag());
    assert_eq!(object.fraction_flag(), 2);
    assert_eq!(object.precision(), 2);
    assert_entity_round(&object, "IgesDimenDimensionTolerance")?;
    Ok(())
}

#[test]
fn class_dimension_units() -> Result<(), Failure> {
    let object = sample_iges_dimen_dimension_units()?;
    assert_eq!(object.nb_property_values(), 2);
    assert_eq!(object.secondary_dimen_position(), 2);
    assert_eq!(object.units_indicator(), 2);
    assert_eq!(object.character_set(), 2);
    assert_eq!(object.fraction_flag(), 2);
    assert_eq!(object.precision_or_denominator(), 2);
    assert_entity_round(&object, "IgesDimenDimensionUnits")?;
    Ok(())
}

#[test]
fn class_dimensioned_geometry() -> Result<(), Failure> {
    let object = sample_iges_dimen_dimensioned_geometry()?;
    assert_eq!(object.nb_dimensions(), 2);
    assert_entity_round(&object, "IgesDimenDimensionedGeometry")?;
    Ok(())
}

#[test]
fn class_flag_note() -> Result<(), Failure> {
    let object = sample_iges_dimen_flag_note()?;
    assert_eq!(object.angle(), 2.5);
    assert_entity_round(&object, "IgesDimenFlagNote")?;
    Ok(())
}

#[test]
fn class_general_label() -> Result<(), Failure> {
    let object = sample_iges_dimen_general_label()?;
    assert_entity_round(&object, "IgesDimenGeneralLabel")?;
    Ok(())
}

#[test]
fn class_general_note() -> Result<(), Failure> {
    let object = sample_iges_dimen_general_note()?;
    assert_entity_round(&object, "IgesDimenGeneralNote")?;
    Ok(())
}

#[test]
fn class_general_symbol() -> Result<(), Failure> {
    let object = sample_iges_dimen_general_symbol()?;
    assert_entity_round(&object, "IgesDimenGeneralSymbol")?;
    Ok(())
}

#[test]
fn class_leader_arrow() -> Result<(), Failure> {
    let object = sample_iges_dimen_leader_arrow()?;
    assert_eq!(object.arrow_head_height(), 1.5);
    assert_eq!(object.arrow_head_width(), 2.5);
    assert_eq!(object.z_depth(), 3.5);
    assert_entity_round(&object, "IgesDimenLeaderArrow")?;
    Ok(())
}

#[test]
fn class_linear_dimension() -> Result<(), Failure> {
    let object = sample_iges_dimen_linear_dimension()?;
    assert_entity_round(&object, "IgesDimenLinearDimension")?;
    Ok(())
}

#[test]
fn class_new_dimensioned_geometry() -> Result<(), Failure> {
    let object = sample_iges_dimen_new_dimensioned_geometry()?;
    assert_eq!(object.nb_dimensions(), 2);
    assert_eq!(object.dimension_orientation_flag(), 2);
    assert_eq!(object.angle_value(), 4.5);
    assert_entity_round(&object, "IgesDimenNewDimensionedGeometry")?;
    Ok(())
}

#[test]
fn class_new_general_note() -> Result<(), Failure> {
    let object = sample_iges_dimen_new_general_note()?;
    assert_eq!(object.text_width(), 1.5);
    assert_eq!(object.text_height(), 2.5);
    assert_eq!(object.justify_code(), 2);
    assert_eq!(object.area_rotation_angle(), 5.5);
    assert_eq!(object.normal_interline_space(), 7.5);
    assert_entity_round(&object, "IgesDimenNewGeneralNote")?;
    Ok(())
}

#[test]
fn class_ordinate_dimension() -> Result<(), Failure> {
    let object = sample_iges_dimen_ordinate_dimension()?;
    assert!(object.is_line());
    assert_entity_round(&object, "IgesDimenOrdinateDimension")?;
    Ok(())
}

#[test]
fn class_point_dimension() -> Result<(), Failure> {
    let object = sample_iges_dimen_point_dimension()?;
    assert_entity_round(&object, "IgesDimenPointDimension")?;
    Ok(())
}

#[test]
fn class_radius_dimension() -> Result<(), Failure> {
    let object = sample_iges_dimen_radius_dimension()?;
    assert_entity_round(&object, "IgesDimenRadiusDimension")?;
    Ok(())
}

#[test]
fn class_section() -> Result<(), Failure> {
    let object = sample_iges_dimen_section()?;
    assert_eq!(object.datatype(), 2);
    assert_eq!(object.z_displacement(), 2.5);
    assert_entity_round(&object, "IgesDimenSection")?;
    Ok(())
}

#[test]
fn class_sectioned_area() -> Result<(), Failure> {
    let object = sample_iges_dimen_sectioned_area()?;
    assert_eq!(object.pattern(), 2);
    assert_eq!(object.distance(), 4.5);
    assert_eq!(object.angle(), 5.5);
    assert_entity_round(&object, "IgesDimenSectionedArea")?;
    Ok(())
}

#[test]
fn class_witness_line() -> Result<(), Failure> {
    let object = sample_iges_dimen_witness_line()?;
    assert_eq!(object.datatype(), 2);
    assert_eq!(object.z_displacement(), 2.5);
    assert_entity_round(&object, "IgesDimenWitnessLine")?;
    Ok(())
}
