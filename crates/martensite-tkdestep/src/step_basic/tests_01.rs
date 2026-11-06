#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

#[test]
fn runtime_type_step_basic_action() {
    let object = Handle::new(StepBasicAction::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepBasicAction");
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_action() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let has_description = true;
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_chosen_method = Handle::new(StepBasicActionMethod::new());
    let object = StepBasicAction::new();
    object.init(&a_name, has_description, &a_description, &a_chosen_method);
    assert!(object.name() == a_name);
    assert!(object.has_description() == has_description);
    assert!(object.description() == a_description);
    assert!(object.chosen_method() == a_chosen_method);
}

#[test]
fn runtime_type_step_basic_action_assignment() {
    let object = Handle::new(StepBasicActionAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicActionAssignment"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_action_assignment() {
    let a_assigned_action = Handle::new(StepBasicAction::new());
    let object = StepBasicActionAssignment::new();
    object.init(&a_assigned_action);
    assert!(object.assigned_action() == a_assigned_action);
}

#[test]
fn runtime_type_step_basic_action_method() {
    let object = Handle::new(StepBasicActionMethod::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicActionMethod"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_action_method() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let has_description = true;
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_consequence = Handle::new(HAsciiString::new("sample"));
    let a_purpose = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicActionMethod::new();
    object.init(
        &a_name,
        has_description,
        &a_description,
        &a_consequence,
        &a_purpose,
    );
    assert!(object.name() == a_name);
    assert!(object.has_description() == has_description);
    assert!(object.description() == a_description);
    assert!(object.consequence() == a_consequence);
    assert!(object.purpose() == a_purpose);
}

#[test]
fn runtime_type_step_basic_action_request_assignment() {
    let object = Handle::new(StepBasicActionRequestAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicActionRequestAssignment"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_action_request_assignment() {
    let a_assigned_action_request = Handle::new(StepBasicVersionedActionRequest::new());
    let object = StepBasicActionRequestAssignment::new();
    object.init(&a_assigned_action_request);
    assert!(object.assigned_action_request() == a_assigned_action_request);
}

#[test]
fn runtime_type_step_basic_action_request_solution() {
    let object = Handle::new(StepBasicActionRequestSolution::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicActionRequestSolution"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_action_request_solution() {
    let a_method = Handle::new(StepBasicActionMethod::new());
    let a_request = Handle::new(StepBasicVersionedActionRequest::new());
    let object = StepBasicActionRequestSolution::new();
    object.init(&a_method, &a_request);
    assert!(object.method() == a_method);
    assert!(object.request() == a_request);
}

#[test]
fn runtime_type_step_basic_address() {
    let object = Handle::new(StepBasicAddress::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepBasicAddress");
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_address() {
    let has_ainternal_location = true;
    let a_internal_location = Handle::new(HAsciiString::new("sample"));
    let has_astreet_number = true;
    let a_street_number = Handle::new(HAsciiString::new("sample"));
    let has_astreet = true;
    let a_street = Handle::new(HAsciiString::new("sample"));
    let has_apostal_box = true;
    let a_postal_box = Handle::new(HAsciiString::new("sample"));
    let has_atown = true;
    let a_town = Handle::new(HAsciiString::new("sample"));
    let has_aregion = true;
    let a_region = Handle::new(HAsciiString::new("sample"));
    let has_apostal_code = true;
    let a_postal_code = Handle::new(HAsciiString::new("sample"));
    let has_acountry = true;
    let a_country = Handle::new(HAsciiString::new("sample"));
    let has_afacsimile_number = true;
    let a_facsimile_number = Handle::new(HAsciiString::new("sample"));
    let has_atelephone_number = true;
    let a_telephone_number = Handle::new(HAsciiString::new("sample"));
    let has_aelectronic_mail_address = true;
    let a_electronic_mail_address = Handle::new(HAsciiString::new("sample"));
    let has_atelex_number = true;
    let a_telex_number = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicAddress::new();
    object.init(
        has_ainternal_location,
        &a_internal_location,
        has_astreet_number,
        &a_street_number,
        has_astreet,
        &a_street,
        has_apostal_box,
        &a_postal_box,
        has_atown,
        &a_town,
        has_aregion,
        &a_region,
        has_apostal_code,
        &a_postal_code,
        has_acountry,
        &a_country,
        has_afacsimile_number,
        &a_facsimile_number,
        has_atelephone_number,
        &a_telephone_number,
        has_aelectronic_mail_address,
        &a_electronic_mail_address,
        has_atelex_number,
        &a_telex_number,
    );
    assert!(object.has_internal_location() == has_ainternal_location);
    assert!(object.internal_location() == a_internal_location);
    assert!(object.has_street_number() == has_astreet_number);
    assert!(object.street_number() == a_street_number);
    assert!(object.has_street() == has_astreet);
    assert!(object.street() == a_street);
    assert!(object.has_postal_box() == has_apostal_box);
    assert!(object.postal_box() == a_postal_box);
    assert!(object.has_town() == has_atown);
    assert!(object.town() == a_town);
    assert!(object.has_region() == has_aregion);
    assert!(object.region() == a_region);
    assert!(object.has_postal_code() == has_apostal_code);
    assert!(object.postal_code() == a_postal_code);
    assert!(object.has_country() == has_acountry);
    assert!(object.country() == a_country);
    assert!(object.has_facsimile_number() == has_afacsimile_number);
    assert!(object.facsimile_number() == a_facsimile_number);
    assert!(object.has_telephone_number() == has_atelephone_number);
    assert!(object.telephone_number() == a_telephone_number);
    assert!(object.has_electronic_mail_address() == has_aelectronic_mail_address);
    assert!(object.electronic_mail_address() == a_electronic_mail_address);
    assert!(object.has_telex_number() == has_atelex_number);
    assert!(object.telex_number() == a_telex_number);
}

#[test]
fn runtime_type_step_basic_application_context() {
    let object = Handle::new(StepBasicApplicationContext::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicApplicationContext"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_application_context() {
    let a_application = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicApplicationContext::new();
    object.init(&a_application);
    assert!(object.application() == a_application);
}

#[test]
fn runtime_type_step_basic_application_context_element() {
    let object = Handle::new(StepBasicApplicationContextElement::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicApplicationContextElement"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_application_context_element() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_frame_of_reference = Handle::new(StepBasicApplicationContext::new());
    let object = StepBasicApplicationContextElement::new();
    object.init(&a_name, &a_frame_of_reference);
    assert!(object.name() == a_name);
    assert!(object.frame_of_reference() == a_frame_of_reference);
}

#[test]
fn runtime_type_step_basic_application_protocol_definition() {
    let object = Handle::new(StepBasicApplicationProtocolDefinition::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicApplicationProtocolDefinition"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_application_protocol_definition() {
    let a_status = Handle::new(HAsciiString::new("sample"));
    let a_application_interpreted_model_schema_name = Handle::new(HAsciiString::new("sample"));
    let a_application_protocol_year = 7;
    let a_application = Handle::new(StepBasicApplicationContext::new());
    let object = StepBasicApplicationProtocolDefinition::new();
    object.init(
        &a_status,
        &a_application_interpreted_model_schema_name,
        a_application_protocol_year,
        &a_application,
    );
    assert!(object.status() == a_status);
    assert!(
        object.application_interpreted_model_schema_name()
            == a_application_interpreted_model_schema_name
    );
    assert!(object.application_protocol_year() == a_application_protocol_year);
    assert!(object.application() == a_application);
}

#[test]
fn runtime_type_step_basic_approval() {
    let object = Handle::new(StepBasicApproval::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepBasicApproval");
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_approval() {
    let a_status = Handle::new(StepBasicApprovalStatus::new());
    let a_level = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicApproval::new();
    object.init(&a_status, &a_level);
    assert!(object.status() == a_status);
    assert!(object.level() == a_level);
}

#[test]
fn runtime_type_step_basic_approval_assignment() {
    let object = Handle::new(StepBasicApprovalAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicApprovalAssignment"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_approval_assignment() {
    let a_assigned_approval = Handle::new(StepBasicApproval::new());
    let object = StepBasicApprovalAssignment::new();
    object.init(&a_assigned_approval);
    assert!(object.assigned_approval() == a_assigned_approval);
}

#[test]
fn runtime_type_step_basic_approval_date_time() {
    let object = Handle::new(StepBasicApprovalDateTime::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicApprovalDateTime"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_approval_date_time() {
    let a_date_time = StepBasicDateTimeSelect::default();
    let a_dated_approval = Handle::new(StepBasicApproval::new());
    let object = StepBasicApprovalDateTime::new();
    object.init(&a_date_time, &a_dated_approval);
    assert!(object.dated_approval() == a_dated_approval);
}

#[test]
fn runtime_type_step_basic_approval_person_organization() {
    let object = Handle::new(StepBasicApprovalPersonOrganization::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicApprovalPersonOrganization"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_approval_person_organization() {
    let a_person_organization = StepBasicPersonOrganizationSelect::default();
    let a_authorized_approval = Handle::new(StepBasicApproval::new());
    let a_role = Handle::new(StepBasicApprovalRole::new());
    let object = StepBasicApprovalPersonOrganization::new();
    object.init(&a_person_organization, &a_authorized_approval, &a_role);
    assert!(object.authorized_approval() == a_authorized_approval);
    assert!(object.role() == a_role);
}

#[test]
fn runtime_type_step_basic_approval_relationship() {
    let object = Handle::new(StepBasicApprovalRelationship::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicApprovalRelationship"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}
