//! The classes of IgesGraph: what their initialization keeps and their round through a file.
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
    assert_entity_round, sample_iges_graph_color, sample_iges_graph_definition_level,
    sample_iges_graph_drawing_size, sample_iges_graph_drawing_units, sample_iges_graph_high_light,
    sample_iges_graph_intercharacter_spacing, sample_iges_graph_line_font_def_pattern,
    sample_iges_graph_line_font_def_template, sample_iges_graph_line_font_predefined,
    sample_iges_graph_nominal_size, sample_iges_graph_pick,
    sample_iges_graph_text_display_template, sample_iges_graph_text_font_def,
    sample_iges_graph_uniform_rect_grid,
};
use martensite_tkernel::standard::Failure;

#[test]
fn class_color() -> Result<(), Failure> {
    let object = sample_iges_graph_color()?;
    assert_entity_round(&object, "IgesGraphColor")?;
    Ok(())
}

#[test]
fn class_definition_level() -> Result<(), Failure> {
    let object = sample_iges_graph_definition_level()?;
    assert_entity_round(&object, "IgesGraphDefinitionLevel")?;
    Ok(())
}

#[test]
fn class_drawing_size() -> Result<(), Failure> {
    let object = sample_iges_graph_drawing_size()?;
    assert_eq!(object.nb_property_values(), 2);
    assert_eq!(object.x_size(), 2.5);
    assert_eq!(object.y_size(), 3.5);
    assert_entity_round(&object, "IgesGraphDrawingSize")?;
    Ok(())
}

#[test]
fn class_drawing_units() -> Result<(), Failure> {
    let object = sample_iges_graph_drawing_units()?;
    assert_eq!(object.nb_property_values(), 2);
    assert_eq!(object.flag(), 2);
    assert_entity_round(&object, "IgesGraphDrawingUnits")?;
    Ok(())
}

#[test]
fn class_high_light() -> Result<(), Failure> {
    let object = sample_iges_graph_high_light()?;
    assert_eq!(object.nb_property_values(), 1);
    assert_eq!(object.high_light_status(), 2);
    assert_entity_round(&object, "IgesGraphHighLight")?;
    Ok(())
}

#[test]
fn class_intercharacter_spacing() -> Result<(), Failure> {
    let object = sample_iges_graph_intercharacter_spacing()?;
    assert_eq!(object.nb_property_values(), 1);
    assert_eq!(object.i_space(), 2.5);
    assert_entity_round(&object, "IgesGraphIntercharacterSpacing")?;
    Ok(())
}

#[test]
fn class_line_font_def_pattern() -> Result<(), Failure> {
    let object = sample_iges_graph_line_font_def_pattern()?;
    assert_entity_round(&object, "IgesGraphLineFontDefPattern")?;
    Ok(())
}

#[test]
fn class_line_font_def_template() -> Result<(), Failure> {
    let object = sample_iges_graph_line_font_def_template()?;
    assert_eq!(object.orientation(), 2);
    assert_eq!(object.distance(), 3.5);
    assert_eq!(object.scale(), 4.5);
    assert_entity_round(&object, "IgesGraphLineFontDefTemplate")?;
    Ok(())
}

#[test]
fn class_line_font_predefined() -> Result<(), Failure> {
    let object = sample_iges_graph_line_font_predefined()?;
    assert_eq!(object.nb_property_values(), 1);
    assert_eq!(object.line_font_pattern_code(), 2);
    assert_entity_round(&object, "IgesGraphLineFontPredefined")?;
    Ok(())
}

#[test]
fn class_nominal_size() -> Result<(), Failure> {
    let object = sample_iges_graph_nominal_size()?;
    assert_eq!(object.nb_property_values(), 2);
    assert_eq!(object.nominal_size_value(), 2.5);
    assert_entity_round(&object, "IgesGraphNominalSize")?;
    Ok(())
}

#[test]
fn class_pick() -> Result<(), Failure> {
    let object = sample_iges_graph_pick()?;
    assert_eq!(object.nb_property_values(), 1);
    assert_eq!(object.pick_flag(), 2);
    assert_entity_round(&object, "IgesGraphPick")?;
    Ok(())
}

#[test]
fn class_text_display_template() -> Result<(), Failure> {
    let object = sample_iges_graph_text_display_template()?;
    assert_eq!(object.box_width(), 1.5);
    assert_eq!(object.box_height(), 2.5);
    assert_eq!(object.font_code(), 2);
    assert_eq!(object.slant_angle(), 5.5);
    assert_eq!(object.rotation_angle(), 6.5);
    assert_eq!(object.mirror_flag(), 2);
    assert_eq!(object.rotate_flag(), 2);
    assert_entity_round(&object, "IgesGraphTextDisplayTemplate")?;
    Ok(())
}

#[test]
fn class_text_font_def() -> Result<(), Failure> {
    let object = sample_iges_graph_text_font_def()?;
    assert_eq!(object.font_code(), 2);
    assert_eq!(object.superseded_font_code(), 2);
    assert_eq!(object.scale(), 2);
    assert_entity_round(&object, "IgesGraphTextFontDef")?;
    Ok(())
}

#[test]
fn class_uniform_rect_grid() -> Result<(), Failure> {
    let object = sample_iges_graph_uniform_rect_grid()?;
    assert_eq!(object.nb_property_values(), 9);
    assert_eq!(object.nb_points_x(), 2);
    assert_eq!(object.nb_points_y(), 2);
    assert_entity_round(&object, "IgesGraphUniformRectGrid")?;
    Ok(())
}
