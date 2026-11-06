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
fn init_round_trip_step_repr_shape_aspect_relationship() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let has_description = true;
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_relating_shape_aspect = Handle::new(StepReprShapeAspect::new());
    let a_related_shape_aspect = Handle::new(StepReprShapeAspect::new());
    let object = StepReprShapeAspectRelationship::new();
    object.init(
        &a_name,
        has_description,
        &a_description,
        &a_relating_shape_aspect,
        &a_related_shape_aspect,
    );
    assert!(object.name() == a_name);
    assert!(object.has_description() == has_description);
    assert!(object.description() == a_description);
    assert!(object.relating_shape_aspect() == a_relating_shape_aspect);
    assert!(object.related_shape_aspect() == a_related_shape_aspect);
}

#[test]
fn runtime_type_step_repr_shape_aspect_transition() {
    let object = Handle::new(StepReprShapeAspectTransition::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprShapeAspectTransition"
    );
    assert!(object.is_kind_name("StepReprShapeAspectRelationship"));
}

#[test]
fn runtime_type_step_repr_shape_representation_relationship() {
    let object = Handle::new(StepReprShapeRepresentationRelationship::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprShapeRepresentationRelationship"
    );
    assert!(object.is_kind_name("StepReprRepresentationRelationship"));
}

#[test]
fn runtime_type_step_repr_shape_representation_relationship_with_transformation() {
    let object = Handle::new(StepReprShapeRepresentationRelationshipWithTransformation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprShapeRepresentationRelationshipWithTransformation"
    );
    assert!(object.is_kind_name("StepReprRepresentationRelationshipWithTransformation"));
}

#[test]
fn runtime_type_step_repr_specified_higher_usage_occurrence() {
    let object = Handle::new(StepReprSpecifiedHigherUsageOccurrence::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprSpecifiedHigherUsageOccurrence"
    );
    assert!(object.is_kind_name("StepReprAssemblyComponentUsage"));
}

#[test]
fn init_round_trip_step_repr_specified_higher_usage_occurrence() {
    let a_product_definition_relationship_id = Handle::new(HAsciiString::new("sample"));
    let a_product_definition_relationship_name = Handle::new(HAsciiString::new("sample"));
    let has_product_definition_relationship_description = true;
    let a_product_definition_relationship_description = Handle::new(HAsciiString::new("sample"));
    let a_product_definition_relationship_relating_product_definition =
        Handle::new(StepBasicProductDefinition::new());
    let a_product_definition_relationship_related_product_definition =
        Handle::new(StepBasicProductDefinition::new());
    let has_assembly_component_usage_reference_designator = true;
    let a_assembly_component_usage_reference_designator = Handle::new(HAsciiString::new("sample"));
    let a_upper_usage = Handle::new(StepReprAssemblyComponentUsage::new());
    let a_next_usage = Handle::new(StepReprNextAssemblyUsageOccurrence::new());
    let object = StepReprSpecifiedHigherUsageOccurrence::new();
    object
        .init(
            &a_product_definition_relationship_id,
            &a_product_definition_relationship_name,
            has_product_definition_relationship_description,
            &a_product_definition_relationship_description,
            &a_product_definition_relationship_relating_product_definition,
            &a_product_definition_relationship_related_product_definition,
            has_assembly_component_usage_reference_designator,
            &a_assembly_component_usage_reference_designator,
            &a_upper_usage,
            &a_next_usage,
        )
        .unwrap();
    assert!(object.upper_usage() == a_upper_usage);
    assert!(object.next_usage() == a_next_usage);
}

#[test]
fn runtime_type_step_repr_structural_response_property() {
    let object = Handle::new(StepReprStructuralResponseProperty::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprStructuralResponseProperty"
    );
    assert!(object.is_kind_name("StepReprPropertyDefinition"));
}

#[test]
fn runtime_type_step_repr_structural_response_property_definition_representation() {
    let object = Handle::new(StepReprStructuralResponsePropertyDefinitionRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprStructuralResponsePropertyDefinitionRepresentation"
    );
    assert!(object.is_kind_name("StepReprPropertyDefinitionRepresentation"));
}

#[test]
fn runtime_type_step_repr_supplied_part_relationship() {
    let object = Handle::new(StepReprSuppliedPartRelationship::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprSuppliedPartRelationship"
    );
    assert!(object.is_kind_name("StepBasicProductDefinitionRelationship"));
}

#[test]
fn runtime_type_step_repr_tangent() {
    let object = Handle::new(StepReprTangent::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepReprTangent");
    assert!(object.is_kind_name("StepReprDerivedShapeAspect"));
}

#[test]
fn runtime_type_step_repr_value_range() {
    let object = Handle::new(StepReprValueRange::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepReprValueRange");
    assert!(object.is_kind_name("StepReprCompoundRepresentationItem"));
}

#[test]
fn runtime_type_step_repr_value_representation_item() {
    let object = Handle::new(StepReprValueRepresentationItem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprValueRepresentationItem"
    );
    assert!(object.is_kind_name("StepReprRepresentationItem"));
}

#[test]
fn init_round_trip_step_repr_value_representation_item() {
    let the_name = Handle::new(HAsciiString::new("sample"));
    let the_value_component_member = Handle::new(StepBasicMeasureValueMember::new());
    let object = StepReprValueRepresentationItem::new();
    object.init(&the_name, &the_value_component_member);
    assert!(object.value_component_member() == the_value_component_member);
}
