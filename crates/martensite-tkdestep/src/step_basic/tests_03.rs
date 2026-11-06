#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

#[test]
fn init_round_trip_step_basic_date_and_time() {
    let a_date_component = Handle::new(StepBasicDate::new());
    let a_time_component = Handle::new(StepBasicLocalTime::new());
    let object = StepBasicDateAndTime::new();
    object.init(&a_date_component, &a_time_component);
    assert!(object.date_component() == a_date_component);
    assert!(object.time_component() == a_time_component);
}

#[test]
fn runtime_type_step_basic_date_and_time_assignment() {
    let object = Handle::new(StepBasicDateAndTimeAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicDateAndTimeAssignment"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_date_and_time_assignment() {
    let a_assigned_date_and_time = Handle::new(StepBasicDateAndTime::new());
    let a_role = Handle::new(StepBasicDateTimeRole::new());
    let object = StepBasicDateAndTimeAssignment::new();
    object.init(&a_assigned_date_and_time, &a_role);
    assert!(object.assigned_date_and_time() == a_assigned_date_and_time);
    assert!(object.role() == a_role);
}

#[test]
fn runtime_type_step_basic_date_assignment() {
    let object = Handle::new(StepBasicDateAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicDateAssignment"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_date_assignment() {
    let a_assigned_date = Handle::new(StepBasicDate::new());
    let a_role = Handle::new(StepBasicDateRole::new());
    let object = StepBasicDateAssignment::new();
    object.init(&a_assigned_date, &a_role);
    assert!(object.assigned_date() == a_assigned_date);
    assert!(object.role() == a_role);
}

#[test]
fn runtime_type_step_basic_date_role() {
    let object = Handle::new(StepBasicDateRole::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepBasicDateRole");
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_date_role() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicDateRole::new();
    object.init(&a_name);
    assert!(object.name() == a_name);
}

#[test]
fn runtime_type_step_basic_date_time_role() {
    let object = Handle::new(StepBasicDateTimeRole::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicDateTimeRole"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_date_time_role() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicDateTimeRole::new();
    object.init(&a_name);
    assert!(object.name() == a_name);
}

#[test]
fn runtime_type_step_basic_derived_unit() {
    let object = Handle::new(StepBasicDerivedUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicDerivedUnit"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_derived_unit() {
    let elements =
        Handle::new(HArray1::<Handle<StepBasicDerivedUnitElement>>::with_bounds(1, 2).unwrap());
    let object = StepBasicDerivedUnit::new();
    object.init(&elements);
    assert!(object.elements() == elements);
}

#[test]
fn runtime_type_step_basic_derived_unit_element() {
    let object = Handle::new(StepBasicDerivedUnitElement::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicDerivedUnitElement"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_derived_unit_element() {
    let a_unit = Handle::new(StepBasicNamedUnit::new());
    let a_exponent = 2.5;
    let object = StepBasicDerivedUnitElement::new();
    object.init(&a_unit, a_exponent);
    assert!(object.unit() == a_unit);
    assert!(object.exponent() == a_exponent);
}

#[test]
fn runtime_type_step_basic_design_context() {
    let object = Handle::new(StepBasicDesignContext::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicDesignContext"
    );
    assert!(object.is_kind_name("StepBasicProductDefinitionContext"));
}

#[test]
fn runtime_type_step_basic_digital_document() {
    let object = Handle::new(StepBasicDigitalDocument::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicDigitalDocument"
    );
    assert!(object.is_kind_name("StepBasicDocument"));
}

#[test]
fn runtime_type_step_basic_dimensional_exponents() {
    let object = Handle::new(StepBasicDimensionalExponents::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicDimensionalExponents"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_dimensional_exponents() {
    let a_length_exponent = 2.5;
    let a_mass_exponent = 2.5;
    let a_time_exponent = 2.5;
    let a_electric_current_exponent = 2.5;
    let a_thermodynamic_temperature_exponent = 2.5;
    let a_amount_of_substance_exponent = 2.5;
    let a_luminous_intensity_exponent = 2.5;
    let object = StepBasicDimensionalExponents::new();
    object.init(
        a_length_exponent,
        a_mass_exponent,
        a_time_exponent,
        a_electric_current_exponent,
        a_thermodynamic_temperature_exponent,
        a_amount_of_substance_exponent,
        a_luminous_intensity_exponent,
    );
    assert!(object.length_exponent() == a_length_exponent);
    assert!(object.mass_exponent() == a_mass_exponent);
    assert!(object.time_exponent() == a_time_exponent);
    assert!(object.electric_current_exponent() == a_electric_current_exponent);
    assert!(object.thermodynamic_temperature_exponent() == a_thermodynamic_temperature_exponent);
    assert!(object.amount_of_substance_exponent() == a_amount_of_substance_exponent);
    assert!(object.luminous_intensity_exponent() == a_luminous_intensity_exponent);
}

#[test]
fn runtime_type_step_basic_document() {
    let object = Handle::new(StepBasicDocument::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepBasicDocument");
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_document() {
    let a_id = Handle::new(HAsciiString::new("sample"));
    let a_name = Handle::new(HAsciiString::new("sample"));
    let has_description = true;
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_kind = Handle::new(StepBasicDocumentType::new());
    let object = StepBasicDocument::new();
    object.init(&a_id, &a_name, has_description, &a_description, &a_kind);
    assert!(object.id() == a_id);
    assert!(object.name() == a_name);
    assert!(object.has_description() == has_description);
    assert!(object.description() == a_description);
    assert!(object.kind() == a_kind);
}

#[test]
fn runtime_type_step_basic_document_file() {
    let object = Handle::new(StepBasicDocumentFile::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicDocumentFile"
    );
    assert!(object.is_kind_name("StepBasicDocument"));
}

#[test]
fn init_round_trip_step_basic_document_file() {
    let a_document_id = Handle::new(HAsciiString::new("sample"));
    let a_document_name = Handle::new(HAsciiString::new("sample"));
    let has_document_description = true;
    let a_document_description = Handle::new(HAsciiString::new("sample"));
    let a_document_kind = Handle::new(StepBasicDocumentType::new());
    let a_characterized_object_name = Handle::new(HAsciiString::new("sample"));
    let has_characterized_object_description = true;
    let a_characterized_object_description = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicDocumentFile::new();
    object
        .init(
            &a_document_id,
            &a_document_name,
            has_document_description,
            &a_document_description,
            &a_document_kind,
            &a_characterized_object_name,
            has_characterized_object_description,
            &a_characterized_object_description,
        )
        .unwrap();
}

#[test]
fn runtime_type_step_basic_document_product_association() {
    let object = Handle::new(StepBasicDocumentProductAssociation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicDocumentProductAssociation"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_document_product_association() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let has_description = true;
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_relating_document = Handle::new(StepBasicDocument::new());
    let a_related_product = StepBasicProductOrFormationOrDefinition::default();
    let object = StepBasicDocumentProductAssociation::new();
    object.init(
        &a_name,
        has_description,
        &a_description,
        &a_relating_document,
        &a_related_product,
    );
    assert!(object.name() == a_name);
    assert!(object.has_description() == has_description);
    assert!(object.description() == a_description);
    assert!(object.relating_document() == a_relating_document);
}

#[test]
fn runtime_type_step_basic_document_product_equivalence() {
    let object = Handle::new(StepBasicDocumentProductEquivalence::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicDocumentProductEquivalence"
    );
    assert!(object.is_kind_name("StepBasicDocumentProductAssociation"));
}

#[test]
fn runtime_type_step_basic_document_reference() {
    let object = Handle::new(StepBasicDocumentReference::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicDocumentReference"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn runtime_type_step_basic_document_relationship() {
    let object = Handle::new(StepBasicDocumentRelationship::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicDocumentRelationship"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_document_relationship() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_relating = Handle::new(StepBasicDocument::new());
    let a_related = Handle::new(StepBasicDocument::new());
    let object = StepBasicDocumentRelationship::new();
    object.init(&a_name, &a_description, &a_relating, &a_related);
    assert!(object.name() == a_name);
    assert!(object.description() == a_description);
    assert!(object.relating_document() == a_relating);
    assert!(object.related_document() == a_related);
}

#[test]
fn runtime_type_step_basic_document_representation_type() {
    let object = Handle::new(StepBasicDocumentRepresentationType::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicDocumentRepresentationType"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_document_representation_type() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_represented_document = Handle::new(StepBasicDocument::new());
    let object = StepBasicDocumentRepresentationType::new();
    object.init(&a_name, &a_represented_document);
    assert!(object.name() == a_name);
    assert!(object.represented_document() == a_represented_document);
}

#[test]
fn runtime_type_step_basic_document_type() {
    let object = Handle::new(StepBasicDocumentType::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicDocumentType"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_document_type() {
    let apdt = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicDocumentType::new();
    object.init(&apdt);
    assert!(object.product_data_type() == apdt);
}

#[test]
fn runtime_type_step_basic_document_usage_constraint() {
    let object = Handle::new(StepBasicDocumentUsageConstraint::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicDocumentUsageConstraint"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}
