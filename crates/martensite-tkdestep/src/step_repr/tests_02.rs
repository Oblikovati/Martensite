#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_basic::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

#[test]
fn runtime_type_step_repr_feature_for_datum_target_relationship() {
    let object = Handle::new(StepReprFeatureForDatumTargetRelationship::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprFeatureForDatumTargetRelationship"
    );
    assert!(object.is_kind_name("StepReprShapeAspectRelationship"));
}

#[test]
fn runtime_type_step_repr_functionally_defined_transformation() {
    let object = Handle::new(StepReprFunctionallyDefinedTransformation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprFunctionallyDefinedTransformation"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_repr_functionally_defined_transformation() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_description = Handle::new(HAsciiString::new("sample"));
    let object = StepReprFunctionallyDefinedTransformation::new();
    object.init(&a_name, &a_description);
    assert!(object.name() == a_name);
    assert!(object.description() == a_description);
}

#[test]
fn runtime_type_step_repr_geometric_alignment() {
    let object = Handle::new(StepReprGeometricAlignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprGeometricAlignment"
    );
    assert!(object.is_kind_name("StepReprDerivedShapeAspect"));
}

#[test]
fn runtime_type_step_repr_global_uncertainty_assigned_context() {
    let object = Handle::new(StepReprGlobalUncertaintyAssignedContext::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprGlobalUncertaintyAssignedContext"
    );
    assert!(object.is_kind_name("StepReprRepresentationContext"));
}

#[test]
fn init_round_trip_step_repr_global_uncertainty_assigned_context() {
    let a_context_identifier = Handle::new(HAsciiString::new("sample"));
    let a_context_type = Handle::new(HAsciiString::new("sample"));
    let a_uncertainty = Handle::new(
        HArray1::<Handle<StepBasicUncertaintyMeasureWithUnit>>::with_bounds(1, 2).unwrap(),
    );
    let object = StepReprGlobalUncertaintyAssignedContext::new();
    object.init(&a_context_identifier, &a_context_type, &a_uncertainty);
    assert!(object.uncertainty() == a_uncertainty);
}

#[test]
fn runtime_type_step_repr_global_unit_assigned_context() {
    let object = Handle::new(StepReprGlobalUnitAssignedContext::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprGlobalUnitAssignedContext"
    );
    assert!(object.is_kind_name("StepReprRepresentationContext"));
}

#[test]
fn init_round_trip_step_repr_global_unit_assigned_context() {
    let a_context_identifier = Handle::new(HAsciiString::new("sample"));
    let a_context_type = Handle::new(HAsciiString::new("sample"));
    let a_units = Handle::new(HArray1::<Handle<StepBasicNamedUnit>>::with_bounds(1, 2).unwrap());
    let object = StepReprGlobalUnitAssignedContext::new();
    object.init(&a_context_identifier, &a_context_type, &a_units);
    assert!(object.units() == a_units);
}

#[test]
fn runtime_type_step_repr_integer_representation_item() {
    let object = Handle::new(StepReprIntegerRepresentationItem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprIntegerRepresentationItem"
    );
    assert!(object.is_kind_name("StepReprRepresentationItem"));
}

#[test]
fn init_round_trip_step_repr_integer_representation_item() {
    let the_name = Handle::new(HAsciiString::new("sample"));
    let the_value = 7;
    let object = StepReprIntegerRepresentationItem::new();
    object.init(&the_name, the_value);
    assert!(object.value() == the_value);
}

#[test]
fn runtime_type_step_repr_item_defined_transformation() {
    let object = Handle::new(StepReprItemDefinedTransformation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprItemDefinedTransformation"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_repr_item_defined_transformation() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_transform_item1 = Handle::new(StepReprRepresentationItem::new());
    let a_transform_item2 = Handle::new(StepReprRepresentationItem::new());
    let object = StepReprItemDefinedTransformation::new();
    object.init(
        &a_name,
        &a_description,
        &a_transform_item1,
        &a_transform_item2,
    );
    assert!(object.name() == a_name);
    assert!(object.description() == a_description);
    assert!(object.transform_item1() == a_transform_item1);
    assert!(object.transform_item2() == a_transform_item2);
}

#[test]
fn runtime_type_step_repr_make_from_usage_option() {
    let object = Handle::new(StepReprMakeFromUsageOption::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprMakeFromUsageOption"
    );
    assert!(object.is_kind_name("StepReprProductDefinitionUsage"));
}

#[test]
fn runtime_type_step_repr_mapped_item() {
    let object = Handle::new(StepReprMappedItem::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepReprMappedItem");
    assert!(object.is_kind_name("StepReprRepresentationItem"));
}

#[test]
fn init_round_trip_step_repr_mapped_item() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_mapping_source = Handle::new(StepReprRepresentationMap::new());
    let a_mapping_target = Handle::new(StepReprRepresentationItem::new());
    let object = StepReprMappedItem::new();
    object.init(&a_name, &a_mapping_source, &a_mapping_target);
    assert!(object.mapping_source() == a_mapping_source);
    assert!(object.mapping_target() == a_mapping_target);
}

#[test]
fn runtime_type_step_repr_material_designation() {
    let object = Handle::new(StepReprMaterialDesignation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprMaterialDesignation"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_repr_material_designation() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_of_definition = StepReprCharacterizedDefinition::default();
    let object = StepReprMaterialDesignation::new();
    object.init(&a_name, &a_of_definition);
    assert!(object.name() == a_name);
}

#[test]
fn runtime_type_step_repr_material_property() {
    let object = Handle::new(StepReprMaterialProperty::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprMaterialProperty"
    );
    assert!(object.is_kind_name("StepReprPropertyDefinition"));
}

#[test]
fn runtime_type_step_repr_material_property_representation() {
    let object = Handle::new(StepReprMaterialPropertyRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprMaterialPropertyRepresentation"
    );
    assert!(object.is_kind_name("StepReprPropertyDefinitionRepresentation"));
}

#[test]
fn init_round_trip_step_repr_material_property_representation() {
    let a_property_definition_representation_definition = StepReprRepresentedDefinition::default();
    let a_property_definition_representation_used_representation =
        Handle::new(StepReprRepresentation::new());
    let a_dependent_environment = Handle::new(StepReprDataEnvironment::new());
    let object = StepReprMaterialPropertyRepresentation::new();
    object.init(
        &a_property_definition_representation_definition,
        &a_property_definition_representation_used_representation,
        &a_dependent_environment,
    );
    assert!(object.dependent_environment() == a_dependent_environment);
}

#[test]
fn runtime_type_step_repr_measure_representation_item() {
    let object = Handle::new(StepReprMeasureRepresentationItem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprMeasureRepresentationItem"
    );
    assert!(object.is_kind_name("StepReprRepresentationItem"));
}

#[test]
fn init_round_trip_step_repr_measure_representation_item() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_value_component = Handle::new(StepBasicMeasureValueMember::new());
    let a_unit_component = StepBasicUnit::default();
    let object = StepReprMeasureRepresentationItem::new();
    object
        .init(&a_name, &a_value_component, &a_unit_component)
        .unwrap();
}

#[test]
fn runtime_type_step_repr_mechanical_design_and_draughting_relationship() {
    let object = Handle::new(StepReprMechanicalDesignAndDraughtingRelationship::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprMechanicalDesignAndDraughtingRelationship"
    );
    assert!(object.is_kind_name("StepReprRepresentationRelationship"));
}

#[test]
fn runtime_type_step_repr_next_assembly_usage_occurrence() {
    let object = Handle::new(StepReprNextAssemblyUsageOccurrence::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprNextAssemblyUsageOccurrence"
    );
    assert!(object.is_kind_name("StepReprAssemblyComponentUsage"));
}

#[test]
fn runtime_type_step_repr_parallel_offset() {
    let object = Handle::new(StepReprParallelOffset::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprParallelOffset"
    );
    assert!(object.is_kind_name("StepReprDerivedShapeAspect"));
}

#[test]
fn runtime_type_step_repr_parametric_representation_context() {
    let object = Handle::new(StepReprParametricRepresentationContext::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprParametricRepresentationContext"
    );
    assert!(object.is_kind_name("StepReprRepresentationContext"));
}

#[test]
fn runtime_type_step_repr_perpendicular_to() {
    let object = Handle::new(StepReprPerpendicularTo::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprPerpendicularTo"
    );
    assert!(object.is_kind_name("StepReprDerivedShapeAspect"));
}

#[test]
fn runtime_type_step_repr_product_concept() {
    let object = Handle::new(StepReprProductConcept::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprProductConcept"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_repr_product_concept() {
    let a_id = Handle::new(HAsciiString::new("sample"));
    let a_name = Handle::new(HAsciiString::new("sample"));
    let has_description = true;
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_market_context = Handle::new(StepBasicProductConceptContext::new());
    let object = StepReprProductConcept::new();
    object.init(
        &a_id,
        &a_name,
        has_description,
        &a_description,
        &a_market_context,
    );
    assert!(object.id() == a_id);
    assert!(object.name() == a_name);
    assert!(object.has_description() == has_description);
    assert!(object.description() == a_description);
    assert!(object.market_context() == a_market_context);
}

#[test]
fn runtime_type_step_repr_product_definition_shape() {
    let object = Handle::new(StepReprProductDefinitionShape::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprProductDefinitionShape"
    );
    assert!(object.is_kind_name("StepReprPropertyDefinition"));
}

#[test]
fn runtime_type_step_repr_product_definition_usage() {
    let object = Handle::new(StepReprProductDefinitionUsage::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprProductDefinitionUsage"
    );
    assert!(object.is_kind_name("StepBasicProductDefinitionRelationship"));
}

#[test]
fn runtime_type_step_repr_promissory_usage_occurrence() {
    let object = Handle::new(StepReprPromissoryUsageOccurrence::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprPromissoryUsageOccurrence"
    );
    assert!(object.is_kind_name("StepReprAssemblyComponentUsage"));
}

#[test]
fn runtime_type_step_repr_property_definition() {
    let object = Handle::new(StepReprPropertyDefinition::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprPropertyDefinition"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_repr_property_definition() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let has_description = true;
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_definition = StepReprCharacterizedDefinition::default();
    let object = StepReprPropertyDefinition::new();
    object.init(&a_name, has_description, &a_description, &a_definition);
    assert!(object.name() == a_name);
    assert!(object.has_description() == has_description);
    assert!(object.description() == a_description);
}

#[test]
fn runtime_type_step_repr_property_definition_relationship() {
    let object = Handle::new(StepReprPropertyDefinitionRelationship::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprPropertyDefinitionRelationship"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}
