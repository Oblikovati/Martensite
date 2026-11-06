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
#[allow(unused_imports)]
use crate::step_shape::*;

#[test]
fn init_round_trip_step_repr_property_definition_relationship() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_relating_property_definition = Handle::new(StepReprPropertyDefinition::new());
    let a_related_property_definition = Handle::new(StepReprPropertyDefinition::new());
    let object = StepReprPropertyDefinitionRelationship::new();
    object.init(
        &a_name,
        &a_description,
        &a_relating_property_definition,
        &a_related_property_definition,
    );
    assert!(object.name() == a_name);
    assert!(object.description() == a_description);
    assert!(object.relating_property_definition() == a_relating_property_definition);
    assert!(object.related_property_definition() == a_related_property_definition);
}

#[test]
fn runtime_type_step_repr_property_definition_representation() {
    let object = Handle::new(StepReprPropertyDefinitionRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprPropertyDefinitionRepresentation"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_repr_property_definition_representation() {
    let a_definition = StepReprRepresentedDefinition::default();
    let a_used_representation = Handle::new(StepReprRepresentation::new());
    let object = StepReprPropertyDefinitionRepresentation::new();
    object.init(&a_definition, &a_used_representation);
    assert!(object.used_representation() == a_used_representation);
}

#[test]
fn runtime_type_step_repr_quantified_assembly_component_usage() {
    let object = Handle::new(StepReprQuantifiedAssemblyComponentUsage::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprQuantifiedAssemblyComponentUsage"
    );
    assert!(object.is_kind_name("StepReprAssemblyComponentUsage"));
}

#[test]
fn runtime_type_step_repr_real_representation_item() {
    let object = Handle::new(StepReprRealRepresentationItem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprRealRepresentationItem"
    );
    assert!(object.is_kind_name("StepReprRepresentationItem"));
}

#[test]
fn init_round_trip_step_repr_real_representation_item() {
    let the_name = Handle::new(HAsciiString::new("sample"));
    let the_value = 2.5;
    let object = StepReprRealRepresentationItem::new();
    object.init(&the_name, the_value);
    assert!(object.value() == the_value);
}

#[test]
fn runtime_type_step_repr_repr_item_and_length_measure_with_unit() {
    let object = Handle::new(StepReprReprItemAndLengthMeasureWithUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprReprItemAndLengthMeasureWithUnit"
    );
    assert!(object.is_kind_name("StepReprReprItemAndMeasureWithUnit"));
}

#[test]
fn runtime_type_step_repr_repr_item_and_length_measure_with_unit_and_qri() {
    let object = Handle::new(StepReprReprItemAndLengthMeasureWithUnitAndQRI::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprReprItemAndLengthMeasureWithUnitAndQRI"
    );
    assert!(object.is_kind_name("StepReprReprItemAndMeasureWithUnitAndQRI"));
}

#[test]
fn runtime_type_step_repr_repr_item_and_measure_with_unit() {
    let object = Handle::new(StepReprReprItemAndMeasureWithUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprReprItemAndMeasureWithUnit"
    );
    assert!(object.is_kind_name("StepReprRepresentationItem"));
}

#[test]
fn init_round_trip_step_repr_repr_item_and_measure_with_unit() {
    let a_mwu = Handle::new(StepBasicMeasureWithUnit::new());
    let a_ri = Handle::new(StepReprRepresentationItem::new());
    let object = StepReprReprItemAndMeasureWithUnit::new();
    object.init(&a_mwu, &a_ri).unwrap();
    assert!(object.get_measure_with_unit() == a_mwu);
}

#[test]
fn runtime_type_step_repr_repr_item_and_measure_with_unit_and_qri() {
    let object = Handle::new(StepReprReprItemAndMeasureWithUnitAndQRI::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprReprItemAndMeasureWithUnitAndQRI"
    );
    assert!(object.is_kind_name("StepReprReprItemAndMeasureWithUnit"));
}

#[test]
fn init_round_trip_step_repr_repr_item_and_measure_with_unit_and_qri() {
    let a_mwu = Handle::new(StepBasicMeasureWithUnit::new());
    let a_ri = Handle::new(StepReprRepresentationItem::new());
    let a_qri = Handle::new(StepShapeQualifiedRepresentationItem::new());
    let object = StepReprReprItemAndMeasureWithUnitAndQRI::new();
    object.init(&a_mwu, &a_ri, &a_qri).unwrap();
    assert!(object.get_qualified_representation_item() == a_qri);
}

#[test]
fn runtime_type_step_repr_repr_item_and_plane_angle_measure_with_unit() {
    let object = Handle::new(StepReprReprItemAndPlaneAngleMeasureWithUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprReprItemAndPlaneAngleMeasureWithUnit"
    );
    assert!(object.is_kind_name("StepReprReprItemAndMeasureWithUnit"));
}

#[test]
fn runtime_type_step_repr_repr_item_and_plane_angle_measure_with_unit_and_qri() {
    let object = Handle::new(StepReprReprItemAndPlaneAngleMeasureWithUnitAndQRI::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprReprItemAndPlaneAngleMeasureWithUnitAndQRI"
    );
    assert!(object.is_kind_name("StepReprReprItemAndMeasureWithUnitAndQRI"));
}

#[test]
fn runtime_type_step_repr_representation() {
    let object = Handle::new(StepReprRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprRepresentation"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_repr_representation() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_items =
        Handle::new(HArray1::<Handle<StepReprRepresentationItem>>::with_bounds(1, 2).unwrap());
    let a_context_of_items = Handle::new(StepReprRepresentationContext::new());
    let object = StepReprRepresentation::new();
    object.init(&a_name, &a_items, &a_context_of_items);
    assert!(object.name() == a_name);
    assert!(object.items() == a_items);
    assert!(object.context_of_items() == a_context_of_items);
}

#[test]
fn runtime_type_step_repr_representation_context() {
    let object = Handle::new(StepReprRepresentationContext::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprRepresentationContext"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_repr_representation_context() {
    let a_context_identifier = Handle::new(HAsciiString::new("sample"));
    let a_context_type = Handle::new(HAsciiString::new("sample"));
    let object = StepReprRepresentationContext::new();
    object.init(&a_context_identifier, &a_context_type);
    assert!(object.context_identifier() == a_context_identifier);
    assert!(object.context_type() == a_context_type);
}

#[test]
fn runtime_type_step_repr_representation_context_reference() {
    let object = Handle::new(StepReprRepresentationContextReference::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprRepresentationContextReference"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_repr_representation_context_reference() {
    let the_context_identifier = Handle::new(HAsciiString::new("sample"));
    let object = StepReprRepresentationContextReference::new();
    object.init(&the_context_identifier);
    assert!(object.context_identifier() == the_context_identifier);
}

#[test]
fn runtime_type_step_repr_representation_item() {
    let object = Handle::new(StepReprRepresentationItem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprRepresentationItem"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_repr_representation_item() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let object = StepReprRepresentationItem::new();
    object.init(&a_name);
    assert!(object.name() == a_name);
}

#[test]
fn runtime_type_step_repr_representation_map() {
    let object = Handle::new(StepReprRepresentationMap::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprRepresentationMap"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_repr_representation_map() {
    let a_mapping_origin = Handle::new(StepReprRepresentationItem::new());
    let a_mapped_representation = Handle::new(StepReprRepresentation::new());
    let object = StepReprRepresentationMap::new();
    object.init(&a_mapping_origin, &a_mapped_representation);
    assert!(object.mapping_origin() == a_mapping_origin);
    assert!(object.mapped_representation() == a_mapped_representation);
}

#[test]
fn runtime_type_step_repr_representation_reference() {
    let object = Handle::new(StepReprRepresentationReference::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprRepresentationReference"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_repr_representation_reference() {
    let the_id = Handle::new(HAsciiString::new("sample"));
    let the_context_of_items = Handle::new(StepReprRepresentationContextReference::new());
    let object = StepReprRepresentationReference::new();
    object.init(&the_id, &the_context_of_items);
    assert!(object.id() == the_id);
    assert!(object.context_of_items() == the_context_of_items);
}

#[test]
fn runtime_type_step_repr_representation_relationship() {
    let object = Handle::new(StepReprRepresentationRelationship::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprRepresentationRelationship"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_repr_representation_relationship() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_rep1 = Handle::new(StepReprRepresentation::new());
    let a_rep2 = Handle::new(StepReprRepresentation::new());
    let object = StepReprRepresentationRelationship::new();
    object.init(&a_name, &a_description, &a_rep1, &a_rep2);
    assert!(object.name() == a_name);
    assert!(object.description() == a_description);
    assert!(object.rep1() == a_rep1);
    assert!(object.rep2() == a_rep2);
}

#[test]
fn runtime_type_step_repr_representation_relationship_with_transformation() {
    let object = Handle::new(StepReprRepresentationRelationshipWithTransformation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprRepresentationRelationshipWithTransformation"
    );
    assert!(object.is_kind_name("StepReprShapeRepresentationRelationship"));
}

#[test]
fn init_round_trip_step_repr_representation_relationship_with_transformation() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_rep1 = Handle::new(StepReprRepresentation::new());
    let a_rep2 = Handle::new(StepReprRepresentation::new());
    let a_trans = StepReprTransformation::default();
    let object = StepReprRepresentationRelationshipWithTransformation::new();
    object.init(&a_name, &a_description, &a_rep1, &a_rep2, &a_trans);
}

#[test]
fn runtime_type_step_repr_shape_aspect() {
    let object = Handle::new(StepReprShapeAspect::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepReprShapeAspect");
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_repr_shape_aspect() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_of_shape = Handle::new(StepReprProductDefinitionShape::new());
    let a_product_definitional = StepDataLogical::default();
    let object = StepReprShapeAspect::new();
    object.init(&a_name, &a_description, &a_of_shape, a_product_definitional);
    assert!(object.name() == a_name);
    assert!(object.description() == a_description);
    assert!(object.of_shape() == a_of_shape);
    assert!(object.product_definitional() == a_product_definitional);
}

#[test]
fn runtime_type_step_repr_shape_aspect_deriving_relationship() {
    let object = Handle::new(StepReprShapeAspectDerivingRelationship::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprShapeAspectDerivingRelationship"
    );
    assert!(object.is_kind_name("StepReprShapeAspectRelationship"));
}

#[test]
fn runtime_type_step_repr_shape_aspect_relationship() {
    let object = Handle::new(StepReprShapeAspectRelationship::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepReprShapeAspectRelationship"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}
