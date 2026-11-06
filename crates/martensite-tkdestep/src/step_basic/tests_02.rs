#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

#[test]
fn init_round_trip_step_basic_approval_relationship() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_relating_approval = Handle::new(StepBasicApproval::new());
    let a_related_approval = Handle::new(StepBasicApproval::new());
    let object = StepBasicApprovalRelationship::new();
    object.init(
        &a_name,
        &a_description,
        &a_relating_approval,
        &a_related_approval,
    );
    assert!(object.name() == a_name);
    assert!(object.description() == a_description);
    assert!(object.relating_approval() == a_relating_approval);
    assert!(object.related_approval() == a_related_approval);
}

#[test]
fn runtime_type_step_basic_approval_role() {
    let object = Handle::new(StepBasicApprovalRole::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicApprovalRole"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_approval_role() {
    let a_role = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicApprovalRole::new();
    object.init(&a_role);
    assert!(object.role() == a_role);
}

#[test]
fn runtime_type_step_basic_approval_status() {
    let object = Handle::new(StepBasicApprovalStatus::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicApprovalStatus"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_approval_status() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicApprovalStatus::new();
    object.init(&a_name);
    assert!(object.name() == a_name);
}

#[test]
fn runtime_type_step_basic_area_unit() {
    let object = Handle::new(StepBasicAreaUnit::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepBasicAreaUnit");
    assert!(object.is_kind_name("StepBasicNamedUnit"));
}

#[test]
fn runtime_type_step_basic_calendar_date() {
    let object = Handle::new(StepBasicCalendarDate::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicCalendarDate"
    );
    assert!(object.is_kind_name("StepBasicDate"));
}

#[test]
fn init_round_trip_step_basic_calendar_date() {
    let a_year_component = 7;
    let a_day_component = 7;
    let a_month_component = 7;
    let object = StepBasicCalendarDate::new();
    object.init(a_year_component, a_day_component, a_month_component);
    assert!(object.day_component() == a_day_component);
    assert!(object.month_component() == a_month_component);
}

#[test]
fn runtime_type_step_basic_certification() {
    let object = Handle::new(StepBasicCertification::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicCertification"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_certification() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_purpose = Handle::new(HAsciiString::new("sample"));
    let a_kind = Handle::new(StepBasicCertificationType::new());
    let object = StepBasicCertification::new();
    object.init(&a_name, &a_purpose, &a_kind);
    assert!(object.name() == a_name);
    assert!(object.purpose() == a_purpose);
    assert!(object.kind() == a_kind);
}

#[test]
fn runtime_type_step_basic_certification_assignment() {
    let object = Handle::new(StepBasicCertificationAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicCertificationAssignment"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_certification_assignment() {
    let a_assigned_certification = Handle::new(StepBasicCertification::new());
    let object = StepBasicCertificationAssignment::new();
    object.init(&a_assigned_certification);
    assert!(object.assigned_certification() == a_assigned_certification);
}

#[test]
fn runtime_type_step_basic_certification_type() {
    let object = Handle::new(StepBasicCertificationType::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicCertificationType"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_certification_type() {
    let a_description = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicCertificationType::new();
    object.init(&a_description);
    assert!(object.description() == a_description);
}

#[test]
fn runtime_type_step_basic_characterized_object() {
    let object = Handle::new(StepBasicCharacterizedObject::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicCharacterizedObject"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_characterized_object() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let has_description = true;
    let a_description = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicCharacterizedObject::new();
    object.init(&a_name, has_description, &a_description);
    assert!(object.name() == a_name);
    assert!(object.has_description() == has_description);
    assert!(object.description() == a_description);
}

#[test]
fn runtime_type_step_basic_contract() {
    let object = Handle::new(StepBasicContract::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepBasicContract");
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_contract() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_purpose = Handle::new(HAsciiString::new("sample"));
    let a_kind = Handle::new(StepBasicContractType::new());
    let object = StepBasicContract::new();
    object.init(&a_name, &a_purpose, &a_kind);
    assert!(object.name() == a_name);
    assert!(object.purpose() == a_purpose);
    assert!(object.kind() == a_kind);
}

#[test]
fn runtime_type_step_basic_contract_assignment() {
    let object = Handle::new(StepBasicContractAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicContractAssignment"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_contract_assignment() {
    let a_assigned_contract = Handle::new(StepBasicContract::new());
    let object = StepBasicContractAssignment::new();
    object.init(&a_assigned_contract);
    assert!(object.assigned_contract() == a_assigned_contract);
}

#[test]
fn runtime_type_step_basic_contract_type() {
    let object = Handle::new(StepBasicContractType::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicContractType"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_contract_type() {
    let a_description = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicContractType::new();
    object.init(&a_description);
    assert!(object.description() == a_description);
}

#[test]
fn runtime_type_step_basic_conversion_based_unit() {
    let object = Handle::new(StepBasicConversionBasedUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicConversionBasedUnit"
    );
    assert!(object.is_kind_name("StepBasicNamedUnit"));
}

#[test]
fn runtime_type_step_basic_conversion_based_unit_and_area_unit() {
    let object = Handle::new(StepBasicConversionBasedUnitAndAreaUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicConversionBasedUnitAndAreaUnit"
    );
    assert!(object.is_kind_name("StepBasicConversionBasedUnit"));
}

#[test]
fn runtime_type_step_basic_conversion_based_unit_and_length_unit() {
    let object = Handle::new(StepBasicConversionBasedUnitAndLengthUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicConversionBasedUnitAndLengthUnit"
    );
    assert!(object.is_kind_name("StepBasicConversionBasedUnit"));
}

#[test]
fn runtime_type_step_basic_conversion_based_unit_and_mass_unit() {
    let object = Handle::new(StepBasicConversionBasedUnitAndMassUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicConversionBasedUnitAndMassUnit"
    );
    assert!(object.is_kind_name("StepBasicConversionBasedUnit"));
}

#[test]
fn runtime_type_step_basic_conversion_based_unit_and_plane_angle_unit() {
    let object = Handle::new(StepBasicConversionBasedUnitAndPlaneAngleUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicConversionBasedUnitAndPlaneAngleUnit"
    );
    assert!(object.is_kind_name("StepBasicConversionBasedUnit"));
}

#[test]
fn runtime_type_step_basic_conversion_based_unit_and_ratio_unit() {
    let object = Handle::new(StepBasicConversionBasedUnitAndRatioUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicConversionBasedUnitAndRatioUnit"
    );
    assert!(object.is_kind_name("StepBasicConversionBasedUnit"));
}

#[test]
fn runtime_type_step_basic_conversion_based_unit_and_solid_angle_unit() {
    let object = Handle::new(StepBasicConversionBasedUnitAndSolidAngleUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicConversionBasedUnitAndSolidAngleUnit"
    );
    assert!(object.is_kind_name("StepBasicConversionBasedUnit"));
}

#[test]
fn runtime_type_step_basic_conversion_based_unit_and_time_unit() {
    let object = Handle::new(StepBasicConversionBasedUnitAndTimeUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicConversionBasedUnitAndTimeUnit"
    );
    assert!(object.is_kind_name("StepBasicConversionBasedUnit"));
}

#[test]
fn runtime_type_step_basic_conversion_based_unit_and_volume_unit() {
    let object = Handle::new(StepBasicConversionBasedUnitAndVolumeUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicConversionBasedUnitAndVolumeUnit"
    );
    assert!(object.is_kind_name("StepBasicConversionBasedUnit"));
}

#[test]
fn runtime_type_step_basic_coordinated_universal_time_offset() {
    let object = Handle::new(StepBasicCoordinatedUniversalTimeOffset::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicCoordinatedUniversalTimeOffset"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_coordinated_universal_time_offset() {
    let a_hour_offset = 7;
    let has_aminute_offset = true;
    let a_minute_offset = 7;
    let a_sense = StepBasicAheadOrBehind::default();
    let object = StepBasicCoordinatedUniversalTimeOffset::new();
    object.init(a_hour_offset, has_aminute_offset, a_minute_offset, a_sense);
    assert!(object.hour_offset() == a_hour_offset);
    assert!(object.has_minute_offset() == has_aminute_offset);
    assert!(object.minute_offset() == a_minute_offset);
    assert!(object.sense() == a_sense);
}

#[test]
fn runtime_type_step_basic_date() {
    let object = Handle::new(StepBasicDate::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepBasicDate");
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_date() {
    let a_year_component = 7;
    let object = StepBasicDate::new();
    object.init(a_year_component);
    assert!(object.year_component() == a_year_component);
}

#[test]
fn runtime_type_step_basic_date_and_time() {
    let object = Handle::new(StepBasicDateAndTime::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicDateAndTime"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}
