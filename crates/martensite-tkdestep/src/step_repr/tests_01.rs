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
fn runtime_type_step_repr_all_around_shape_aspect() {
    let object = Handle::new(StepReprAllAroundShapeAspect::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprAllAroundShapeAspect"
    );
    assert!(object.is_kind_name("StepReprContinuosShapeAspect"));
}

#[test]
fn runtime_type_step_repr_apex() {
    let object = Handle::new(StepReprApex::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepReprApex");
    assert!(object.is_kind_name("StepReprDerivedShapeAspect"));
}

#[test]
fn runtime_type_step_repr_assembly_component_usage() {
    let object = Handle::new(StepReprAssemblyComponentUsage::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprAssemblyComponentUsage"
    );
    assert!(object.is_kind_name("StepReprProductDefinitionUsage"));
}

#[test]
fn init_round_trip_step_repr_assembly_component_usage() {
    let a_product_definition_relationship_id = Handle::new(HAsciiString::new("sample"));
    let a_product_definition_relationship_name = Handle::new(HAsciiString::new("sample"));
    let has_product_definition_relationship_description = true;
    let a_product_definition_relationship_description = Handle::new(HAsciiString::new("sample"));
    let a_product_definition_relationship_relating_product_definition =
        Handle::new(StepBasicProductDefinition::new());
    let a_product_definition_relationship_related_product_definition =
        Handle::new(StepBasicProductDefinition::new());
    let has_reference_designator = true;
    let a_reference_designator = Handle::new(HAsciiString::new("sample"));
    let object = StepReprAssemblyComponentUsage::new();
    object
        .init(
            &a_product_definition_relationship_id,
            &a_product_definition_relationship_name,
            has_product_definition_relationship_description,
            &a_product_definition_relationship_description,
            &a_product_definition_relationship_relating_product_definition,
            &a_product_definition_relationship_related_product_definition,
            has_reference_designator,
            &a_reference_designator,
        )
        .unwrap();
    assert!(object.has_reference_designator() == has_reference_designator);
    assert!(object.reference_designator() == a_reference_designator);
}

#[test]
fn runtime_type_step_repr_assembly_component_usage_substitute() {
    let object = Handle::new(StepReprAssemblyComponentUsageSubstitute::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprAssemblyComponentUsageSubstitute"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_repr_assembly_component_usage_substitute() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_def = Handle::new(HAsciiString::new("sample"));
    let a_base = Handle::new(StepReprAssemblyComponentUsage::new());
    let a_subs = Handle::new(StepReprAssemblyComponentUsage::new());
    let object = StepReprAssemblyComponentUsageSubstitute::new();
    object.init(&a_name, &a_def, &a_base, &a_subs);
    assert!(object.name() == a_name);
    assert!(object.definition() == a_def);
    assert!(object.base() == a_base);
    assert!(object.substitute() == a_subs);
}

#[test]
fn runtime_type_step_repr_between_shape_aspect() {
    let object = Handle::new(StepReprBetweenShapeAspect::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprBetweenShapeAspect"
    );
    assert!(object.is_kind_name("StepReprContinuosShapeAspect"));
}

#[test]
fn runtime_type_step_repr_boolean_representation_item() {
    let object = Handle::new(StepReprBooleanRepresentationItem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprBooleanRepresentationItem"
    );
    assert!(object.is_kind_name("StepReprRepresentationItem"));
}

#[test]
fn init_round_trip_step_repr_boolean_representation_item() {
    let the_name = Handle::new(HAsciiString::new("sample"));
    let the_value = true;
    let object = StepReprBooleanRepresentationItem::new();
    object.init(&the_name, the_value);
    assert!(object.value() == the_value);
}

#[test]
fn runtime_type_step_repr_centre_of_symmetry() {
    let object = Handle::new(StepReprCentreOfSymmetry::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprCentreOfSymmetry"
    );
    assert!(object.is_kind_name("StepReprDerivedShapeAspect"));
}

#[test]
fn runtime_type_step_repr_characterized_representation() {
    let object = Handle::new(StepReprCharacterizedRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprCharacterizedRepresentation"
    );
    assert!(object.is_kind_name("StepReprRepresentation"));
}

#[test]
fn init_round_trip_step_repr_characterized_representation() {
    let the_name = Handle::new(HAsciiString::new("sample"));
    let the_description = Handle::new(HAsciiString::new("sample"));
    let the_items =
        Handle::new(HArray1::<Handle<StepReprRepresentationItem>>::with_bounds(1, 2).unwrap());
    let the_context_of_items = Handle::new(StepReprRepresentationContext::new());
    let object = StepReprCharacterizedRepresentation::new();
    object.init(
        &the_name,
        &the_description,
        &the_items,
        &the_context_of_items,
    );
    assert!(object.description() == the_description);
}

#[test]
fn runtime_type_step_repr_comp_group_sh_asp_and_comp_sh_asp_and_datum_feat_and_sh_asp() {
    let object = Handle::new(StepReprCompGroupShAspAndCompShAspAndDatumFeatAndShAsp::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprCompGroupShAspAndCompShAspAndDatumFeatAndShAsp"
    );
    assert!(object.is_kind_name("StepReprCompShAspAndDatumFeatAndShAsp"));
}

#[test]
fn runtime_type_step_repr_comp_sh_asp_and_datum_feat_and_sh_asp() {
    let object = Handle::new(StepReprCompShAspAndDatumFeatAndShAsp::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprCompShAspAndDatumFeatAndShAsp"
    );
    assert!(object.is_kind_name("StepReprShapeAspect"));
}

#[test]
fn runtime_type_step_repr_composite_group_shape_aspect() {
    let object = Handle::new(StepReprCompositeGroupShapeAspect::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprCompositeGroupShapeAspect"
    );
    assert!(object.is_kind_name("StepReprCompositeShapeAspect"));
}

#[test]
fn runtime_type_step_repr_composite_shape_aspect() {
    let object = Handle::new(StepReprCompositeShapeAspect::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprCompositeShapeAspect"
    );
    assert!(object.is_kind_name("StepReprShapeAspect"));
}

#[test]
fn runtime_type_step_repr_compound_representation_item() {
    let object = Handle::new(StepReprCompoundRepresentationItem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprCompoundRepresentationItem"
    );
    assert!(object.is_kind_name("StepReprRepresentationItem"));
}

#[test]
fn init_round_trip_step_repr_compound_representation_item() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let item_element =
        Handle::new(HArray1::<Handle<StepReprRepresentationItem>>::with_bounds(1, 2).unwrap());
    let object = StepReprCompoundRepresentationItem::new();
    object.init(&a_name, &item_element);
    assert!(object.item_element() == item_element);
}

#[test]
fn runtime_type_step_repr_configuration_design() {
    let object = Handle::new(StepReprConfigurationDesign::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprConfigurationDesign"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_repr_configuration_design() {
    let a_configuration = Handle::new(StepReprConfigurationItem::new());
    let a_design = StepReprConfigurationDesignItem::default();
    let object = StepReprConfigurationDesign::new();
    object.init(&a_configuration, &a_design);
    assert!(object.configuration() == a_configuration);
}

#[test]
fn runtime_type_step_repr_configuration_effectivity() {
    let object = Handle::new(StepReprConfigurationEffectivity::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprConfigurationEffectivity"
    );
    assert!(object.is_kind_name("StepBasicProductDefinitionEffectivity"));
}

#[test]
fn init_round_trip_step_repr_configuration_effectivity() {
    let a_effectivity_id = Handle::new(HAsciiString::new("sample"));
    let a_product_definition_effectivity_usage =
        Handle::new(StepBasicProductDefinitionRelationship::new());
    let a_configuration = Handle::new(StepReprConfigurationDesign::new());
    let object = StepReprConfigurationEffectivity::new();
    object.init(
        &a_effectivity_id,
        &a_product_definition_effectivity_usage,
        &a_configuration,
    );
    assert!(object.configuration() == a_configuration);
}

#[test]
fn runtime_type_step_repr_configuration_item() {
    let object = Handle::new(StepReprConfigurationItem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprConfigurationItem"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_repr_configuration_item() {
    let a_id = Handle::new(HAsciiString::new("sample"));
    let a_name = Handle::new(HAsciiString::new("sample"));
    let has_description = true;
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_item_concept = Handle::new(StepReprProductConcept::new());
    let has_purpose = true;
    let a_purpose = Handle::new(HAsciiString::new("sample"));
    let object = StepReprConfigurationItem::new();
    object.init(
        &a_id,
        &a_name,
        has_description,
        &a_description,
        &a_item_concept,
        has_purpose,
        &a_purpose,
    );
    assert!(object.id() == a_id);
    assert!(object.name() == a_name);
    assert!(object.has_description() == has_description);
    assert!(object.description() == a_description);
    assert!(object.item_concept() == a_item_concept);
    assert!(object.has_purpose() == has_purpose);
    assert!(object.purpose() == a_purpose);
}

#[test]
fn runtime_type_step_repr_constructive_geometry_representation() {
    let object = Handle::new(StepReprConstructiveGeometryRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprConstructiveGeometryRepresentation"
    );
    assert!(object.is_kind_name("StepReprRepresentation"));
}

#[test]
fn runtime_type_step_repr_constructive_geometry_representation_relationship() {
    let object = Handle::new(StepReprConstructiveGeometryRepresentationRelationship::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprConstructiveGeometryRepresentationRelationship"
    );
    assert!(object.is_kind_name("StepReprRepresentationRelationship"));
}

#[test]
fn runtime_type_step_repr_continuos_shape_aspect() {
    let object = Handle::new(StepReprContinuosShapeAspect::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprContinuosShapeAspect"
    );
    assert!(object.is_kind_name("StepReprCompositeShapeAspect"));
}

#[test]
fn runtime_type_step_repr_data_environment() {
    let object = Handle::new(StepReprDataEnvironment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprDataEnvironment"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_repr_data_environment() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_elements = Handle::new(
        HArray1::<Handle<StepReprPropertyDefinitionRepresentation>>::with_bounds(1, 2).unwrap(),
    );
    let object = StepReprDataEnvironment::new();
    object.init(&a_name, &a_description, &a_elements);
    assert!(object.name() == a_name);
    assert!(object.description() == a_description);
    assert!(object.elements() == a_elements);
}

#[test]
fn runtime_type_step_repr_definitional_representation() {
    let object = Handle::new(StepReprDefinitionalRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprDefinitionalRepresentation"
    );
    assert!(object.is_kind_name("StepReprRepresentation"));
}

#[test]
fn runtime_type_step_repr_derived_shape_aspect() {
    let object = Handle::new(StepReprDerivedShapeAspect::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprDerivedShapeAspect"
    );
    assert!(object.is_kind_name("StepReprShapeAspect"));
}

#[test]
fn runtime_type_step_repr_descriptive_representation_item() {
    let object = Handle::new(StepReprDescriptiveRepresentationItem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprDescriptiveRepresentationItem"
    );
    assert!(object.is_kind_name("StepReprRepresentationItem"));
}

#[test]
fn init_round_trip_step_repr_descriptive_representation_item() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_description = Handle::new(HAsciiString::new("sample"));
    let object = StepReprDescriptiveRepresentationItem::new();
    object.init(&a_name, &a_description);
    assert!(object.description() == a_description);
}

#[test]
fn runtime_type_step_repr_extension() {
    let object = Handle::new(StepReprExtension::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepReprExtension");
    assert!(object.is_kind_name("StepReprDerivedShapeAspect"));
}

#[test]
fn runtime_type_step_repr_externally_defined_representation() {
    let object = Handle::new(StepReprExternallyDefinedRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprExternallyDefinedRepresentation"
    );
    assert!(object.is_kind_name("StepReprRepresentation"));
}
