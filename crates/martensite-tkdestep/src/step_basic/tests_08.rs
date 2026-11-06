#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

#[test]
fn init_round_trip_step_basic_si_unit_and_time_unit() {
    let has_aprefix = true;
    let a_prefix = StepBasicSiPrefix::default();
    let a_name = StepBasicSiUnitName::default();
    let object = StepBasicSiUnitAndTimeUnit::new();
    object.init(has_aprefix, a_prefix, a_name);
}

#[test]
fn runtime_type_step_basic_si_unit_and_volume_unit() {
    let object = Handle::new(StepBasicSiUnitAndVolumeUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicSiUnitAndVolumeUnit"
    );
    assert!(object.is_kind_name("StepBasicSiUnit"));
}

#[test]
fn runtime_type_step_basic_solid_angle_measure_with_unit() {
    let object = Handle::new(StepBasicSolidAngleMeasureWithUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicSolidAngleMeasureWithUnit"
    );
    assert!(object.is_kind_name("StepBasicMeasureWithUnit"));
}

#[test]
fn runtime_type_step_basic_solid_angle_unit() {
    let object = Handle::new(StepBasicSolidAngleUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicSolidAngleUnit"
    );
    assert!(object.is_kind_name("StepBasicNamedUnit"));
}

#[test]
fn runtime_type_step_basic_thermodynamic_temperature_unit() {
    let object = Handle::new(StepBasicThermodynamicTemperatureUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicThermodynamicTemperatureUnit"
    );
    assert!(object.is_kind_name("StepBasicNamedUnit"));
}

#[test]
fn runtime_type_step_basic_time_measure_with_unit() {
    let object = Handle::new(StepBasicTimeMeasureWithUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicTimeMeasureWithUnit"
    );
    assert!(object.is_kind_name("StepBasicMeasureWithUnit"));
}

#[test]
fn runtime_type_step_basic_time_unit() {
    let object = Handle::new(StepBasicTimeUnit::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepBasicTimeUnit");
    assert!(object.is_kind_name("StepBasicNamedUnit"));
}

#[test]
fn runtime_type_step_basic_uncertainty_measure_with_unit() {
    let object = Handle::new(StepBasicUncertaintyMeasureWithUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicUncertaintyMeasureWithUnit"
    );
    assert!(object.is_kind_name("StepBasicMeasureWithUnit"));
}

#[test]
fn init_round_trip_step_basic_uncertainty_measure_with_unit() {
    let a_value_component = Handle::new(StepBasicMeasureValueMember::new());
    let a_unit_component = StepBasicUnit::default();
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_description = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicUncertaintyMeasureWithUnit::new();
    object.init(
        &a_value_component,
        &a_unit_component,
        &a_name,
        &a_description,
    );
    assert!(object.name() == a_name);
    assert!(object.description() == a_description);
}

#[test]
fn runtime_type_step_basic_versioned_action_request() {
    let object = Handle::new(StepBasicVersionedActionRequest::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicVersionedActionRequest"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_basic_versioned_action_request() {
    let a_id = Handle::new(HAsciiString::new("sample"));
    let a_version = Handle::new(HAsciiString::new("sample"));
    let a_purpose = Handle::new(HAsciiString::new("sample"));
    let has_description = true;
    let a_description = Handle::new(HAsciiString::new("sample"));
    let object = StepBasicVersionedActionRequest::new();
    object.init(
        &a_id,
        &a_version,
        &a_purpose,
        has_description,
        &a_description,
    );
    assert!(object.id() == a_id);
    assert!(object.version() == a_version);
    assert!(object.purpose() == a_purpose);
    assert!(object.has_description() == has_description);
    assert!(object.description() == a_description);
}

#[test]
fn runtime_type_step_basic_volume_unit() {
    let object = Handle::new(StepBasicVolumeUnit::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepBasicVolumeUnit");
    assert!(object.is_kind_name("StepBasicNamedUnit"));
}

#[test]
fn runtime_type_step_basic_week_of_year_and_day_date() {
    let object = Handle::new(StepBasicWeekOfYearAndDayDate::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepBasicWeekOfYearAndDayDate"
    );
    assert!(object.is_kind_name("StepBasicDate"));
}

#[test]
fn init_round_trip_step_basic_week_of_year_and_day_date() {
    let a_year_component = 7;
    let a_week_component = 7;
    let has_aday_component = true;
    let a_day_component = 7;
    let object = StepBasicWeekOfYearAndDayDate::new();
    object.init(
        a_year_component,
        a_week_component,
        has_aday_component,
        a_day_component,
    );
    assert!(object.week_component() == a_week_component);
    assert!(object.has_day_component() == has_aday_component);
    assert!(object.day_component() == a_day_component);
}
