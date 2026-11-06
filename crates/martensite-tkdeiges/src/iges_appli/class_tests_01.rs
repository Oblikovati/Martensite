//! The classes of IgesAppli: what their initialization keeps and their round through a file.
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
    assert_entity_round, sample_iges_appli_drilled_hole, sample_iges_appli_element_results,
    sample_iges_appli_finite_element, sample_iges_appli_flow, sample_iges_appli_flow_line_spec,
    sample_iges_appli_level_function, sample_iges_appli_level_to_pwb_layer_map,
    sample_iges_appli_line_widening, sample_iges_appli_nodal_constraint,
    sample_iges_appli_nodal_displ_and_rot, sample_iges_appli_nodal_results, sample_iges_appli_node,
    sample_iges_appli_part_number, sample_iges_appli_pin_number, sample_iges_appli_piping_flow,
    sample_iges_appli_pwb_artwork_stackup, sample_iges_appli_pwb_drilled_hole,
    sample_iges_appli_reference_designator, sample_iges_appli_region_restriction,
};
use martensite_tkernel::standard::Failure;

#[test]
fn class_drilled_hole() -> Result<(), Failure> {
    let object = sample_iges_appli_drilled_hole()?;
    assert_eq!(object.nb_property_values(), 5);
    assert_eq!(object.drill_dia_size(), 2.5);
    assert_eq!(object.finish_dia_size(), 3.5);
    assert_eq!(object.nb_lower_layer(), 2);
    assert_eq!(object.nb_higher_layer(), 2);
    assert_entity_round(&object, "IgesAppliDrilledHole")?;
    Ok(())
}

#[test]
fn class_element_results() -> Result<(), Failure> {
    let object = sample_iges_appli_element_results()?;
    assert_eq!(object.sub_case_number(), 2);
    assert_eq!(object.time(), 3.5);
    assert_eq!(object.nb_result_values(), 1);
    assert_eq!(object.result_report_flag(), 2);
    assert_entity_round(&object, "IgesAppliElementResults")?;
    Ok(())
}

#[test]
fn class_finite_element() -> Result<(), Failure> {
    let object = sample_iges_appli_finite_element()?;
    assert_eq!(object.topology(), 2);
    assert_entity_round(&object, "IgesAppliFiniteElement")?;
    Ok(())
}

#[test]
fn class_flow() -> Result<(), Failure> {
    let object = sample_iges_appli_flow()?;
    assert_eq!(object.nb_context_flags(), 2);
    assert_eq!(object.type_of_flow(), 2);
    assert_eq!(object.function_flag(), 2);
    assert_entity_round(&object, "IgesAppliFlow")?;
    Ok(())
}

#[test]
fn class_flow_line_spec() -> Result<(), Failure> {
    let object = sample_iges_appli_flow_line_spec()?;
    assert_entity_round(&object, "IgesAppliFlowLineSpec")?;
    Ok(())
}

#[test]
fn class_level_function() -> Result<(), Failure> {
    let object = sample_iges_appli_level_function()?;
    assert_eq!(object.nb_property_values(), 2);
    assert_eq!(object.func_description_code(), 2);
    assert_entity_round(&object, "IgesAppliLevelFunction")?;
    Ok(())
}

#[test]
fn class_level_to_pwb_layer_map() -> Result<(), Failure> {
    let object = sample_iges_appli_level_to_pwb_layer_map()?;
    assert_eq!(object.nb_property_values(), 2);
    assert_entity_round(&object, "IgesAppliLevelToPWBLayerMap")?;
    Ok(())
}

#[test]
fn class_line_widening() -> Result<(), Failure> {
    let object = sample_iges_appli_line_widening()?;
    assert_eq!(object.nb_property_values(), 5);
    assert_eq!(object.width_of_metalization(), 2.5);
    assert_eq!(object.cornering_code(), 2);
    assert_eq!(object.extension_flag(), 2);
    assert_eq!(object.justification_flag(), 2);
    assert_eq!(object.extension_value(), 6.5);
    assert_entity_round(&object, "IgesAppliLineWidening")?;
    Ok(())
}

#[test]
fn class_nodal_constraint() -> Result<(), Failure> {
    let object = sample_iges_appli_nodal_constraint()?;
    assert_eq!(object.type_(), 2);
    assert_entity_round(&object, "IgesAppliNodalConstraint")?;
    Ok(())
}

#[test]
fn class_nodal_displ_and_rot() -> Result<(), Failure> {
    let object = sample_iges_appli_nodal_displ_and_rot()?;
    assert_entity_round(&object, "IgesAppliNodalDisplAndRot")?;
    Ok(())
}

#[test]
fn class_nodal_results() -> Result<(), Failure> {
    let object = sample_iges_appli_nodal_results()?;
    assert_eq!(object.sub_case_number(), 2);
    assert_eq!(object.time(), 3.5);
    assert_entity_round(&object, "IgesAppliNodalResults")?;
    Ok(())
}

#[test]
fn class_node() -> Result<(), Failure> {
    let object = sample_iges_appli_node()?;
    assert_entity_round(&object, "IgesAppliNode")?;
    Ok(())
}

#[test]
fn class_pwb_artwork_stackup() -> Result<(), Failure> {
    let object = sample_iges_appli_pwb_artwork_stackup()?;
    assert_eq!(object.nb_property_values(), 2);
    assert_entity_round(&object, "IgesAppliPWBArtworkStackup")?;
    Ok(())
}

#[test]
fn class_pwb_drilled_hole() -> Result<(), Failure> {
    let object = sample_iges_appli_pwb_drilled_hole()?;
    assert_eq!(object.nb_property_values(), 3);
    assert_eq!(object.drill_diameter_size(), 2.5);
    assert_eq!(object.finish_diameter_size(), 3.5);
    assert_eq!(object.function_code(), 2);
    assert_entity_round(&object, "IgesAppliPWBDrilledHole")?;
    Ok(())
}

#[test]
fn class_part_number() -> Result<(), Failure> {
    let object = sample_iges_appli_part_number()?;
    assert_eq!(object.nb_property_values(), 2);
    assert_entity_round(&object, "IgesAppliPartNumber")?;
    Ok(())
}

#[test]
fn class_pin_number() -> Result<(), Failure> {
    let object = sample_iges_appli_pin_number()?;
    assert_eq!(object.nb_property_values(), 2);
    assert_entity_round(&object, "IgesAppliPinNumber")?;
    Ok(())
}

#[test]
fn class_piping_flow() -> Result<(), Failure> {
    let object = sample_iges_appli_piping_flow()?;
    assert_eq!(object.nb_context_flags(), 2);
    assert_eq!(object.type_of_flow(), 2);
    assert_entity_round(&object, "IgesAppliPipingFlow")?;
    Ok(())
}

#[test]
fn class_reference_designator() -> Result<(), Failure> {
    let object = sample_iges_appli_reference_designator()?;
    assert_eq!(object.nb_property_values(), 2);
    assert_entity_round(&object, "IgesAppliReferenceDesignator")?;
    Ok(())
}

#[test]
fn class_region_restriction() -> Result<(), Failure> {
    let object = sample_iges_appli_region_restriction()?;
    assert_eq!(object.nb_property_values(), 3);
    assert_eq!(object.electrical_vias_restriction(), 2);
    assert_eq!(object.electrical_component_restriction(), 2);
    assert_eq!(object.electrical_ckt_restriction(), 2);
    assert_entity_round(&object, "IgesAppliRegionRestriction")?;
    Ok(())
}
