#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

#[test]
fn init_round_trip_step_basic_product_definition_reference() {
    let the_source = Handle::new(StepBasicExternalSource::new());
    let the_product_id = Handle::new(HAsciiString::new("sample"));
    let the_product_definition_formation_id = Handle::new(HAsciiString::new("sample"));
    let the_product_definition_id = Handle::new(HAsciiString::new("sample"));
    let the_id_owning_organization_name = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicProductDefinitionReference::new();
    object.init(
        &the_source,
        &the_product_id,
        &the_product_definition_formation_id,
        &the_product_definition_id,
        &the_id_owning_organization_name,
    );
    assert!(object.source() == the_source);
    assert!(object.product_id() == the_product_id);
    assert!(object.product_definition_formation_id() == the_product_definition_formation_id);
    assert!(object.product_definition_id() == the_product_definition_id);
    assert!(object.id_owning_organization_name() == the_id_owning_organization_name);
}

#[test]
fn runtime_type_step_basic_product_definition_reference_with_local_representation() {
    let object = Handle::new(StepBasicProductDefinitionReferenceWithLocalRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicProductDefinitionReferenceWithLocalRepresentation"
    );
    assert!(object.is_kind_name("StepBasicProductDefinition"));
}

#[test]
fn init_round_trip_step_basic_product_definition_reference_with_local_representation() {
    let the_source = Handle::new(StepBasicExternalSource::new());
    let the_id = Handle::new(HAsciiString::new("sample"));
    let the_description = Handle::new(HAsciiString::new("sample"));
    let the_formation = Handle::new(StepBasicProductDefinitionFormation::new());
    let the_frame_of_reference = Handle::new(StepBasicProductDefinitionContext::new());
    let object = StepBasicProductDefinitionReferenceWithLocalRepresentation::new();
    object.init(
        &the_source,
        &the_id,
        &the_description,
        &the_formation,
        &the_frame_of_reference,
    );
    assert!(object.source() == the_source);
}

#[test]
fn runtime_type_step_basic_product_definition_relationship() {
    let object = Handle::new(StepBasicProductDefinitionRelationship::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicProductDefinitionRelationship"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_product_definition_relationship() {
    let a_id = Handle::new(HAsciiString::new("sample"));
    let a_name = Handle::new(HAsciiString::new("sample"));
    let has_description = true;
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_relating_product_definition = Handle::new(StepBasicProductDefinition::new());
    let a_related_product_definition = Handle::new(StepBasicProductDefinition::new());
    let object = StepBasicProductDefinitionRelationship::new();
    object
        .init(
            &a_id,
            &a_name,
            has_description,
            &a_description,
            &a_relating_product_definition,
            &a_related_product_definition,
        )
        .unwrap();
    assert!(object.id() == a_id);
    assert!(object.name() == a_name);
    assert!(object.has_description() == has_description);
    assert!(object.description() == a_description);
}

#[test]
fn runtime_type_step_basic_product_definition_with_associated_documents() {
    let object = Handle::new(StepBasicProductDefinitionWithAssociatedDocuments::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicProductDefinitionWithAssociatedDocuments"
    );
    assert!(object.is_kind_name("StepBasicProductDefinition"));
}

#[test]
fn init_round_trip_step_basic_product_definition_with_associated_documents() {
    let a_id = Handle::new(HAsciiString::new("sample"));
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_formation = Handle::new(StepBasicProductDefinitionFormation::new());
    let a_frame = Handle::new(StepBasicProductDefinitionContext::new());
    let a_doc_ids = Handle::new(HArray1::<Handle<StepBasicDocument>>::with_bounds(1, 2).unwrap());
    let object = StepBasicProductDefinitionWithAssociatedDocuments::new();
    object.init(&a_id, &a_description, &a_formation, &a_frame, &a_doc_ids);
    assert!(object.doc_ids() == a_doc_ids);
}

#[test]
fn runtime_type_step_basic_product_related_product_category() {
    let object = Handle::new(StepBasicProductRelatedProductCategory::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicProductRelatedProductCategory"
    );
    assert!(object.is_kind_name("StepBasicProductCategory"));
}

#[test]
fn init_round_trip_step_basic_product_related_product_category() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let has_adescription = true;
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_products = Handle::new(HArray1::<Handle<StepBasicProduct>>::with_bounds(1, 2).unwrap());
    let object = StepBasicProductRelatedProductCategory::new();
    object.init(&a_name, has_adescription, &a_description, &a_products);
    assert!(object.products() == a_products);
}

#[test]
fn runtime_type_step_basic_product_type() {
    let object = Handle::new(StepBasicProductType::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicProductType"
    );
    assert!(object.is_kind_name("StepBasicProductRelatedProductCategory"));
}

#[test]
fn runtime_type_step_basic_ratio_measure_with_unit() {
    let object = Handle::new(StepBasicRatioMeasureWithUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicRatioMeasureWithUnit"
    );
    assert!(object.is_kind_name("StepBasicMeasureWithUnit"));
}

#[test]
fn runtime_type_step_basic_ratio_unit() {
    let object = Handle::new(StepBasicRatioUnit::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepBasicRatioUnit");
    assert!(object.is_kind_name("StepBasicNamedUnit"));
}

#[test]
fn runtime_type_step_basic_role_association() {
    let object = Handle::new(StepBasicRoleAssociation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicRoleAssociation"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_role_association() {
    let a_role = Handle::new(StepBasicObjectRole::new());
    let a_item_with_role = StepBasicRoleSelect::default();
    let object = StepBasicRoleAssociation::new();
    object.init(&a_role, &a_item_with_role);
    assert!(object.role() == a_role);
}

#[test]
fn runtime_type_step_basic_security_classification() {
    let object = Handle::new(StepBasicSecurityClassification::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicSecurityClassification"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_security_classification() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_purpose = Handle::new(HAsciiString::new("sample"));
    let a_security_level = Handle::new(StepBasicSecurityClassificationLevel::new());
    let object = StepBasicSecurityClassification::new();
    object.init(&a_name, &a_purpose, &a_security_level);
    assert!(object.name() == a_name);
    assert!(object.purpose() == a_purpose);
    assert!(object.security_level() == a_security_level);
}

#[test]
fn runtime_type_step_basic_security_classification_assignment() {
    let object = Handle::new(StepBasicSecurityClassificationAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicSecurityClassificationAssignment"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_security_classification_assignment() {
    let a_assigned_security_classification = Handle::new(StepBasicSecurityClassification::new());
    let object = StepBasicSecurityClassificationAssignment::new();
    object.init(&a_assigned_security_classification);
    assert!(object.assigned_security_classification() == a_assigned_security_classification);
}

#[test]
fn runtime_type_step_basic_security_classification_level() {
    let object = Handle::new(StepBasicSecurityClassificationLevel::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicSecurityClassificationLevel"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_security_classification_level() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicSecurityClassificationLevel::new();
    object.init(&a_name);
    assert!(object.name() == a_name);
}

#[test]
fn runtime_type_step_basic_si_unit_and_area_unit() {
    let object = Handle::new(StepBasicSiUnitAndAreaUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicSiUnitAndAreaUnit"
    );
    assert!(object.is_kind_name("StepBasicSiUnit"));
}

#[test]
fn runtime_type_step_basic_si_unit_and_length_unit() {
    let object = Handle::new(StepBasicSiUnitAndLengthUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicSiUnitAndLengthUnit"
    );
    assert!(object.is_kind_name("StepBasicSiUnit"));
}

#[test]
fn init_round_trip_step_basic_si_unit_and_length_unit() {
    let has_aprefix = true;
    let a_prefix = StepBasicSiPrefix::default();
    let a_name = StepBasicSiUnitName::default();
    let object = StepBasicSiUnitAndLengthUnit::new();
    object.init(has_aprefix, a_prefix, a_name);
}

#[test]
fn runtime_type_step_basic_si_unit_and_mass_unit() {
    let object = Handle::new(StepBasicSiUnitAndMassUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicSiUnitAndMassUnit"
    );
    assert!(object.is_kind_name("StepBasicSiUnit"));
}

#[test]
fn init_round_trip_step_basic_si_unit_and_mass_unit() {
    let has_aprefix = true;
    let a_prefix = StepBasicSiPrefix::default();
    let a_name = StepBasicSiUnitName::default();
    let object = StepBasicSiUnitAndMassUnit::new();
    object.init(has_aprefix, a_prefix, a_name);
}

#[test]
fn runtime_type_step_basic_si_unit_and_plane_angle_unit() {
    let object = Handle::new(StepBasicSiUnitAndPlaneAngleUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicSiUnitAndPlaneAngleUnit"
    );
    assert!(object.is_kind_name("StepBasicSiUnit"));
}

#[test]
fn init_round_trip_step_basic_si_unit_and_plane_angle_unit() {
    let has_aprefix = true;
    let a_prefix = StepBasicSiPrefix::default();
    let a_name = StepBasicSiUnitName::default();
    let object = StepBasicSiUnitAndPlaneAngleUnit::new();
    object.init(has_aprefix, a_prefix, a_name);
}

#[test]
fn runtime_type_step_basic_si_unit_and_ratio_unit() {
    let object = Handle::new(StepBasicSiUnitAndRatioUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicSiUnitAndRatioUnit"
    );
    assert!(object.is_kind_name("StepBasicSiUnit"));
}

#[test]
fn init_round_trip_step_basic_si_unit_and_ratio_unit() {
    let has_aprefix = true;
    let a_prefix = StepBasicSiPrefix::default();
    let a_name = StepBasicSiUnitName::default();
    let object = StepBasicSiUnitAndRatioUnit::new();
    object.init(has_aprefix, a_prefix, a_name);
}

#[test]
fn runtime_type_step_basic_si_unit_and_solid_angle_unit() {
    let object = Handle::new(StepBasicSiUnitAndSolidAngleUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicSiUnitAndSolidAngleUnit"
    );
    assert!(object.is_kind_name("StepBasicSiUnit"));
}

#[test]
fn init_round_trip_step_basic_si_unit_and_solid_angle_unit() {
    let has_aprefix = true;
    let a_prefix = StepBasicSiPrefix::default();
    let a_name = StepBasicSiUnitName::default();
    let object = StepBasicSiUnitAndSolidAngleUnit::new();
    object.init(has_aprefix, a_prefix, a_name);
}

#[test]
fn runtime_type_step_basic_si_unit_and_thermodynamic_temperature_unit() {
    let object = Handle::new(StepBasicSiUnitAndThermodynamicTemperatureUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicSiUnitAndThermodynamicTemperatureUnit"
    );
    assert!(object.is_kind_name("StepBasicSiUnit"));
}

#[test]
fn init_round_trip_step_basic_si_unit_and_thermodynamic_temperature_unit() {
    let has_aprefix = true;
    let a_prefix = StepBasicSiPrefix::default();
    let a_name = StepBasicSiUnitName::default();
    let object = StepBasicSiUnitAndThermodynamicTemperatureUnit::new();
    object.init(has_aprefix, a_prefix, a_name);
}

#[test]
fn runtime_type_step_basic_si_unit_and_time_unit() {
    let object = Handle::new(StepBasicSiUnitAndTimeUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicSiUnitAndTimeUnit"
    );
    assert!(object.is_kind_name("StepBasicSiUnit"));
}
