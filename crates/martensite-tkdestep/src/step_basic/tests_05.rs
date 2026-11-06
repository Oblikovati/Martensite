#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

#[test]
fn init_round_trip_step_basic_local_time() {
    let a_hour_component = 7;
    let has_aminute_component = true;
    let a_minute_component = 7;
    let has_asecond_component = true;
    let a_second_component = 2.5;
    let a_zone = Handle::new(StepBasicCoordinatedUniversalTimeOffset::new());
    let object = StepBasicLocalTime::new();
    object.init(
        a_hour_component,
        has_aminute_component,
        a_minute_component,
        has_asecond_component,
        a_second_component,
        &a_zone,
    );
    assert!(object.hour_component() == a_hour_component);
    assert!(object.has_minute_component() == has_aminute_component);
    assert!(object.minute_component() == a_minute_component);
    assert!(object.has_second_component() == has_asecond_component);
    assert!(object.second_component() == a_second_component);
    assert!(object.zone() == a_zone);
}

#[test]
fn runtime_type_step_basic_mass_measure_with_unit() {
    let object = Handle::new(StepBasicMassMeasureWithUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicMassMeasureWithUnit"
    );
    assert!(object.is_kind_name("StepBasicMeasureWithUnit"));
}

#[test]
fn runtime_type_step_basic_mass_unit() {
    let object = Handle::new(StepBasicMassUnit::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepBasicMassUnit");
    assert!(object.is_kind_name("StepBasicNamedUnit"));
}

#[test]
fn runtime_type_step_basic_measure_with_unit() {
    let object = Handle::new(StepBasicMeasureWithUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicMeasureWithUnit"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_measure_with_unit() {
    let a_value_component = Handle::new(StepBasicMeasureValueMember::new());
    let a_unit_component = StepBasicUnit::default();
    let object = StepBasicMeasureWithUnit::new();
    object.init(&a_value_component, &a_unit_component);
    assert!(object.value_component_member() == a_value_component);
}

#[test]
fn runtime_type_step_basic_mechanical_context() {
    let object = Handle::new(StepBasicMechanicalContext::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicMechanicalContext"
    );
    assert!(object.is_kind_name("StepBasicProductContext"));
}

#[test]
fn runtime_type_step_basic_name_assignment() {
    let object = Handle::new(StepBasicNameAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicNameAssignment"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_name_assignment() {
    let a_assigned_name = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicNameAssignment::new();
    object.init(&a_assigned_name);
    assert!(object.assigned_name() == a_assigned_name);
}

#[test]
fn runtime_type_step_basic_object_role() {
    let object = Handle::new(StepBasicObjectRole::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepBasicObjectRole");
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_object_role() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let has_description = true;
    let a_description = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicObjectRole::new();
    object.init(&a_name, has_description, &a_description);
    assert!(object.name() == a_name);
    assert!(object.has_description() == has_description);
    assert!(object.description() == a_description);
}

#[test]
fn runtime_type_step_basic_ordinal_date() {
    let object = Handle::new(StepBasicOrdinalDate::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicOrdinalDate"
    );
    assert!(object.is_kind_name("StepBasicDate"));
}

#[test]
fn init_round_trip_step_basic_ordinal_date() {
    let a_year_component = 7;
    let a_day_component = 7;
    let object = StepBasicOrdinalDate::new();
    object.init(a_year_component, a_day_component);
    assert!(object.day_component() == a_day_component);
}

#[test]
fn runtime_type_step_basic_organization() {
    let object = Handle::new(StepBasicOrganization::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicOrganization"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_organization() {
    let has_aid = true;
    let a_id = Handle::new(HAsciiString::new("sample"));
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_description = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicOrganization::new();
    object.init(has_aid, &a_id, &a_name, &a_description);
    assert!(object.has_id() == has_aid);
    assert!(object.id() == a_id);
    assert!(object.name() == a_name);
    assert!(object.description() == a_description);
}

#[test]
fn runtime_type_step_basic_organization_assignment() {
    let object = Handle::new(StepBasicOrganizationAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicOrganizationAssignment"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_organization_assignment() {
    let a_assigned_organization = Handle::new(StepBasicOrganization::new());
    let a_role = Handle::new(StepBasicOrganizationRole::new());
    let object = StepBasicOrganizationAssignment::new();
    object.init(&a_assigned_organization, &a_role);
    assert!(object.assigned_organization() == a_assigned_organization);
    assert!(object.role() == a_role);
}

#[test]
fn runtime_type_step_basic_organization_role() {
    let object = Handle::new(StepBasicOrganizationRole::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicOrganizationRole"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_organization_role() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicOrganizationRole::new();
    object.init(&a_name);
    assert!(object.name() == a_name);
}

#[test]
fn runtime_type_step_basic_organizational_address() {
    let object = Handle::new(StepBasicOrganizationalAddress::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicOrganizationalAddress"
    );
    assert!(object.is_kind_name("StepBasicAddress"));
}

#[test]
fn init_round_trip_step_basic_organizational_address() {
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
    let a_organizations =
        Handle::new(HArray1::<Handle<StepBasicOrganization>>::with_bounds(1, 2).unwrap());
    let a_description = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicOrganizationalAddress::new();
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
        &a_organizations,
        &a_description,
    );
    assert!(object.organizations() == a_organizations);
    assert!(object.description() == a_description);
}

#[test]
fn runtime_type_step_basic_person() {
    let object = Handle::new(StepBasicPerson::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepBasicPerson");
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_person() {
    let a_id = Handle::new(HAsciiString::new("sample"));
    let has_alast_name = true;
    let a_last_name = Handle::new(HAsciiString::new("sample"));
    let has_afirst_name = true;
    let a_first_name = Handle::new(HAsciiString::new("sample"));
    let has_amiddle_names = true;
    let a_middle_names = Handle::new(HArray1::<Handle<HAsciiString>>::with_bounds(1, 2).unwrap());
    let has_aprefix_titles = true;
    let a_prefix_titles = Handle::new(HArray1::<Handle<HAsciiString>>::with_bounds(1, 2).unwrap());
    let has_asuffix_titles = true;
    let a_suffix_titles = Handle::new(HArray1::<Handle<HAsciiString>>::with_bounds(1, 2).unwrap());
    let object = StepBasicPerson::new();
    object.init(
        &a_id,
        has_alast_name,
        &a_last_name,
        has_afirst_name,
        &a_first_name,
        has_amiddle_names,
        &a_middle_names,
        has_aprefix_titles,
        &a_prefix_titles,
        has_asuffix_titles,
        &a_suffix_titles,
    );
    assert!(object.id() == a_id);
    assert!(object.has_last_name() == has_alast_name);
    assert!(object.last_name() == a_last_name);
    assert!(object.has_first_name() == has_afirst_name);
    assert!(object.first_name() == a_first_name);
    assert!(object.has_middle_names() == has_amiddle_names);
    assert!(object.middle_names() == a_middle_names);
    assert!(object.has_prefix_titles() == has_aprefix_titles);
    assert!(object.prefix_titles() == a_prefix_titles);
    assert!(object.has_suffix_titles() == has_asuffix_titles);
    assert!(object.suffix_titles() == a_suffix_titles);
}

#[test]
fn runtime_type_step_basic_person_and_organization() {
    let object = Handle::new(StepBasicPersonAndOrganization::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicPersonAndOrganization"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_person_and_organization() {
    let a_the_person = Handle::new(StepBasicPerson::new());
    let a_the_organization = Handle::new(StepBasicOrganization::new());
    let object = StepBasicPersonAndOrganization::new();
    object.init(&a_the_person, &a_the_organization);
    assert!(object.the_person() == a_the_person);
    assert!(object.the_organization() == a_the_organization);
}

#[test]
fn runtime_type_step_basic_person_and_organization_assignment() {
    let object = Handle::new(StepBasicPersonAndOrganizationAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicPersonAndOrganizationAssignment"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_person_and_organization_assignment() {
    let a_assigned_person_and_organization = Handle::new(StepBasicPersonAndOrganization::new());
    let a_role = Handle::new(StepBasicPersonAndOrganizationRole::new());
    let object = StepBasicPersonAndOrganizationAssignment::new();
    object.init(&a_assigned_person_and_organization, &a_role);
    assert!(object.assigned_person_and_organization() == a_assigned_person_and_organization);
    assert!(object.role() == a_role);
}

#[test]
fn runtime_type_step_basic_person_and_organization_role() {
    let object = Handle::new(StepBasicPersonAndOrganizationRole::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicPersonAndOrganizationRole"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_person_and_organization_role() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicPersonAndOrganizationRole::new();
    object.init(&a_name);
    assert!(object.name() == a_name);
}
