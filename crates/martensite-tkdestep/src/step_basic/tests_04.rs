#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;
#[allow(unused_imports)]
use crate::step_repr::*;

#[test]
fn init_round_trip_step_basic_document_usage_constraint() {
    let a_source = Handle::new(StepBasicDocument::new());
    let ase = Handle::new(HAsciiString::new("sample"));
    let asev = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicDocumentUsageConstraint::new();
    object.init(&a_source, &ase, &asev);
    assert!(object.source() == a_source);
    assert!(object.subject_element() == ase);
    assert!(object.subject_element_value() == asev);
}

#[test]
fn runtime_type_step_basic_effectivity() {
    let object = Handle::new(StepBasicEffectivity::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicEffectivity"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_effectivity() {
    let aid = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicEffectivity::new();
    object.init(&aid);
    assert!(object.id() == aid);
}

#[test]
fn runtime_type_step_basic_effectivity_assignment() {
    let object = Handle::new(StepBasicEffectivityAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicEffectivityAssignment"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_effectivity_assignment() {
    let a_assigned_effectivity = Handle::new(StepBasicEffectivity::new());
    let object = StepBasicEffectivityAssignment::new();
    object.init(&a_assigned_effectivity);
    assert!(object.assigned_effectivity() == a_assigned_effectivity);
}

#[test]
fn runtime_type_step_basic_euler_angles() {
    let object = Handle::new(StepBasicEulerAngles::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicEulerAngles"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_euler_angles() {
    let a_angles = Handle::new(HArray1::<f64>::with_bounds(1, 2).unwrap());
    let object = StepBasicEulerAngles::new();
    object.init(&a_angles);
    assert!(object.angles() == a_angles);
}

#[test]
fn runtime_type_step_basic_external_identification_assignment() {
    let object = Handle::new(StepBasicExternalIdentificationAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicExternalIdentificationAssignment"
    );
    assert!(object.is_kind_name("StepBasicIdentificationAssignment"));
}

#[test]
fn init_round_trip_step_basic_external_identification_assignment() {
    let a_identification_assignment_assigned_id = Handle::new(HAsciiString::new("sample"));
    let a_identification_assignment_role = Handle::new(StepBasicIdentificationRole::new());
    let a_source = Handle::new(StepBasicExternalSource::new());
    let object = StepBasicExternalIdentificationAssignment::new();
    object.init(
        &a_identification_assignment_assigned_id,
        &a_identification_assignment_role,
        &a_source,
    );
    assert!(object.source() == a_source);
}

#[test]
fn runtime_type_step_basic_external_source() {
    let object = Handle::new(StepBasicExternalSource::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicExternalSource"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_external_source() {
    let a_source_id = StepBasicSourceItem::default();
    let object = StepBasicExternalSource::new();
    object.init(&a_source_id);
}

#[test]
fn runtime_type_step_basic_externally_defined_item() {
    let object = Handle::new(StepBasicExternallyDefinedItem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicExternallyDefinedItem"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_externally_defined_item() {
    let a_item_id = StepBasicSourceItem::default();
    let a_source = Handle::new(StepBasicExternalSource::new());
    let object = StepBasicExternallyDefinedItem::new();
    object.init(&a_item_id, &a_source);
    assert!(object.source() == a_source);
}

#[test]
fn runtime_type_step_basic_general_property() {
    let object = Handle::new(StepBasicGeneralProperty::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicGeneralProperty"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_general_property() {
    let a_id = Handle::new(HAsciiString::new("sample"));
    let a_name = Handle::new(HAsciiString::new("sample"));
    let has_description = true;
    let a_description = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicGeneralProperty::new();
    object.init(&a_id, &a_name, has_description, &a_description);
    assert!(object.id() == a_id);
    assert!(object.name() == a_name);
    assert!(object.has_description() == has_description);
    assert!(object.description() == a_description);
}

#[test]
fn runtime_type_step_basic_general_property_association() {
    let object = Handle::new(StepBasicGeneralPropertyAssociation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicGeneralPropertyAssociation"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_general_property_association() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_general_property = Handle::new(StepBasicGeneralProperty::new());
    let a_property_definition = Handle::new(StepReprPropertyDefinition::new());
    let object = StepBasicGeneralPropertyAssociation::new();
    object.init(
        &a_name,
        &a_description,
        &a_general_property,
        &a_property_definition,
    );
    assert!(object.name() == a_name);
    assert!(object.description() == a_description);
    assert!(object.general_property() == a_general_property);
    assert!(object.property_definition() == a_property_definition);
}

#[test]
fn runtime_type_step_basic_general_property_relationship() {
    let object = Handle::new(StepBasicGeneralPropertyRelationship::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicGeneralPropertyRelationship"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_general_property_relationship() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let has_description = true;
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_relating_general_property = Handle::new(StepBasicGeneralProperty::new());
    let a_related_general_property = Handle::new(StepBasicGeneralProperty::new());
    let object = StepBasicGeneralPropertyRelationship::new();
    object.init(
        &a_name,
        has_description,
        &a_description,
        &a_relating_general_property,
        &a_related_general_property,
    );
    assert!(object.name() == a_name);
    assert!(object.has_description() == has_description);
    assert!(object.description() == a_description);
    assert!(object.relating_general_property() == a_relating_general_property);
    assert!(object.related_general_property() == a_related_general_property);
}

#[test]
fn runtime_type_step_basic_group() {
    let object = Handle::new(StepBasicGroup::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepBasicGroup");
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_group() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let has_description = true;
    let a_description = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicGroup::new();
    object.init(&a_name, has_description, &a_description);
    assert!(object.name() == a_name);
    assert!(object.has_description() == has_description);
    assert!(object.description() == a_description);
}

#[test]
fn runtime_type_step_basic_group_assignment() {
    let object = Handle::new(StepBasicGroupAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicGroupAssignment"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_group_assignment() {
    let a_assigned_group = Handle::new(StepBasicGroup::new());
    let object = StepBasicGroupAssignment::new();
    object.init(&a_assigned_group);
    assert!(object.assigned_group() == a_assigned_group);
}

#[test]
fn runtime_type_step_basic_group_relationship() {
    let object = Handle::new(StepBasicGroupRelationship::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicGroupRelationship"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_group_relationship() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let has_description = true;
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_relating_group = Handle::new(StepBasicGroup::new());
    let a_related_group = Handle::new(StepBasicGroup::new());
    let object = StepBasicGroupRelationship::new();
    object.init(
        &a_name,
        has_description,
        &a_description,
        &a_relating_group,
        &a_related_group,
    );
    assert!(object.name() == a_name);
    assert!(object.has_description() == has_description);
    assert!(object.description() == a_description);
    assert!(object.relating_group() == a_relating_group);
    assert!(object.related_group() == a_related_group);
}

#[test]
fn runtime_type_step_basic_identification_assignment() {
    let object = Handle::new(StepBasicIdentificationAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicIdentificationAssignment"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_identification_assignment() {
    let a_assigned_id = Handle::new(HAsciiString::new("sample"));
    let a_role = Handle::new(StepBasicIdentificationRole::new());
    let object = StepBasicIdentificationAssignment::new();
    object.init(&a_assigned_id, &a_role);
    assert!(object.assigned_id() == a_assigned_id);
    assert!(object.role() == a_role);
}

#[test]
fn runtime_type_step_basic_identification_role() {
    let object = Handle::new(StepBasicIdentificationRole::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicIdentificationRole"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_identification_role() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let has_description = true;
    let a_description = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicIdentificationRole::new();
    object.init(&a_name, has_description, &a_description);
    assert!(object.name() == a_name);
    assert!(object.has_description() == has_description);
    assert!(object.description() == a_description);
}

#[test]
fn runtime_type_step_basic_length_measure_with_unit() {
    let object = Handle::new(StepBasicLengthMeasureWithUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicLengthMeasureWithUnit"
    );
    assert!(object.is_kind_name("StepBasicMeasureWithUnit"));
}

#[test]
fn runtime_type_step_basic_length_unit() {
    let object = Handle::new(StepBasicLengthUnit::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepBasicLengthUnit");
    assert!(object.is_kind_name("StepBasicNamedUnit"));
}

#[test]
fn runtime_type_step_basic_local_time() {
    let object = Handle::new(StepBasicLocalTime::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepBasicLocalTime");
    assert!(object.is_kind_name("Standard_Transient"));
}
