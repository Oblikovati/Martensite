#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

step_entity! {
    /// Representation of STEP entity Contract
    StepBasicContract, StepBasicContractState, "StepBasicContract", StandardTransient, {
        the_name: Handle<HAsciiString>,
        the_purpose: Handle<HAsciiString>,
        the_kind: Handle<StepBasicContractType>,
    }
}
impl StepBasicContract {
    step_methods! {
        get name = the_name: Handle<HAsciiString>;
        set set_name = the_name: Handle<HAsciiString>;
        get purpose = the_purpose: Handle<HAsciiString>;
        set set_purpose = the_purpose: Handle<HAsciiString>;
        get kind = the_kind: Handle<StepBasicContractType>;
        set set_kind = the_kind: Handle<StepBasicContractType>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_purpose: &Handle<HAsciiString>,
        a_kind: &Handle<StepBasicContractType>,
    ) {
        let mut state = self.state.write();
        state.the_name = a_name.clone();
        state.the_purpose = a_purpose.clone();
        state.the_kind = a_kind.clone();
    }
}

step_entity! {
    /// Representation of STEP entity ContractAssignment
    StepBasicContractAssignment, StepBasicContractAssignmentState, "StepBasicContractAssignment", StandardTransient, {
        the_assigned_contract: Handle<StepBasicContract>,
    }
}
impl StepBasicContractAssignment {
    step_methods! {
        set init = the_assigned_contract: Handle<StepBasicContract>;
        get assigned_contract = the_assigned_contract: Handle<StepBasicContract>;
        set set_assigned_contract = the_assigned_contract: Handle<StepBasicContract>;
    }
}

step_entity! {
    /// Representation of STEP entity ContractType
    StepBasicContractType, StepBasicContractTypeState, "StepBasicContractType", StandardTransient, {
        the_description: Handle<HAsciiString>,
    }
}
impl StepBasicContractType {
    step_methods! {
        set init = the_description: Handle<HAsciiString>;
        get description = the_description: Handle<HAsciiString>;
        set set_description = the_description: Handle<HAsciiString>;
    }
}

step_entity! {
    /// Conversion based unit.
    StepBasicConversionBasedUnit, StepBasicConversionBasedUnitState, "StepBasicConversionBasedUnit", StepBasicNamedUnit, {
        name: Handle<HAsciiString>,
        conversion_factor: Handle<StandardTransient>,
    }
}
impl StepBasicConversionBasedUnit {
    step_methods! {
        set set_name = name: Handle<HAsciiString>;
        get name = name: Handle<HAsciiString>;
        set set_conversion_factor = conversion_factor: Handle<StandardTransient>;
        get conversion_factor = conversion_factor: Handle<StandardTransient>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_dimensions: &Handle<StepBasicDimensionalExponents>,
        a_name: &Handle<HAsciiString>,
        a_conversion_factor: &Handle<StandardTransient>,
    ) {
        let mut state = self.state.write();
        state.name = a_name.clone();
        state.conversion_factor = a_conversion_factor.clone();
        drop(state);
        self.parent.init(a_dimensions);
    }
}

step_entity! {
    /// Conversion based unit and area unit.
    StepBasicConversionBasedUnitAndAreaUnit, StepBasicConversionBasedUnitAndAreaUnitState, "StepBasicConversionBasedUnitAndAreaUnit", StepBasicConversionBasedUnit, {
        area_unit: Handle<StepBasicAreaUnit>,
    }
}
impl StepBasicConversionBasedUnitAndAreaUnit {
    step_methods! {
        set set_area_unit = area_unit: Handle<StepBasicAreaUnit>;
        get area_unit = area_unit: Handle<StepBasicAreaUnit>;
    }
}

step_entity! {
    /// Conversion based unit and length unit.
    StepBasicConversionBasedUnitAndLengthUnit, StepBasicConversionBasedUnitAndLengthUnitState, "StepBasicConversionBasedUnitAndLengthUnit", StepBasicConversionBasedUnit, {
        length_unit: Handle<StepBasicLengthUnit>,
    }
}
impl StepBasicConversionBasedUnitAndLengthUnit {
    step_methods! {
        set set_length_unit = length_unit: Handle<StepBasicLengthUnit>;
        get length_unit = length_unit: Handle<StepBasicLengthUnit>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_dimensions: &Handle<StepBasicDimensionalExponents>,
        a_name: &Handle<HAsciiString>,
        a_conversion_factor: &Handle<StandardTransient>,
    ) {
        self.parent.init(a_dimensions, a_name, a_conversion_factor);
        self.state.write().length_unit = Handle::new(StepBasicLengthUnit::new());
        let field_length_unit = self.state.read().length_unit.clone();
        field_length_unit.init(a_dimensions);
    }
}

step_entity! {
    /// Conversion based unit and mass unit.
    StepBasicConversionBasedUnitAndMassUnit, StepBasicConversionBasedUnitAndMassUnitState, "StepBasicConversionBasedUnitAndMassUnit", StepBasicConversionBasedUnit, {
        mass_unit: Handle<StepBasicMassUnit>,
    }
}
impl StepBasicConversionBasedUnitAndMassUnit {
    step_methods! {
        set set_mass_unit = mass_unit: Handle<StepBasicMassUnit>;
        get mass_unit = mass_unit: Handle<StepBasicMassUnit>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_dimensions: &Handle<StepBasicDimensionalExponents>,
        a_name: &Handle<HAsciiString>,
        a_conversion_factor: &Handle<StandardTransient>,
    ) {
        self.parent.init(a_dimensions, a_name, a_conversion_factor);
        self.state.write().mass_unit = Handle::new(StepBasicMassUnit::new());
        let field_mass_unit = self.state.read().mass_unit.clone();
        field_mass_unit.init(a_dimensions);
    }
}

step_entity! {
    /// Conversion based unit and plane angle unit.
    StepBasicConversionBasedUnitAndPlaneAngleUnit, StepBasicConversionBasedUnitAndPlaneAngleUnitState, "StepBasicConversionBasedUnitAndPlaneAngleUnit", StepBasicConversionBasedUnit, {
        plane_angle_unit: Handle<StepBasicPlaneAngleUnit>,
    }
}
impl StepBasicConversionBasedUnitAndPlaneAngleUnit {
    step_methods! {
        set set_plane_angle_unit = plane_angle_unit: Handle<StepBasicPlaneAngleUnit>;
        get plane_angle_unit = plane_angle_unit: Handle<StepBasicPlaneAngleUnit>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_dimensions: &Handle<StepBasicDimensionalExponents>,
        a_name: &Handle<HAsciiString>,
        a_conversion_factor: &Handle<StandardTransient>,
    ) {
        self.parent.init(a_dimensions, a_name, a_conversion_factor);
        self.state.write().plane_angle_unit = Handle::new(StepBasicPlaneAngleUnit::new());
        let field_plane_angle_unit = self.state.read().plane_angle_unit.clone();
        field_plane_angle_unit.init(a_dimensions);
    }
}

step_entity! {
    /// Conversion based unit and ratio unit.
    StepBasicConversionBasedUnitAndRatioUnit, StepBasicConversionBasedUnitAndRatioUnitState, "StepBasicConversionBasedUnitAndRatioUnit", StepBasicConversionBasedUnit, {
        ratio_unit: Handle<StepBasicRatioUnit>,
    }
}
impl StepBasicConversionBasedUnitAndRatioUnit {
    step_methods! {
        set set_ratio_unit = ratio_unit: Handle<StepBasicRatioUnit>;
        get ratio_unit = ratio_unit: Handle<StepBasicRatioUnit>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_dimensions: &Handle<StepBasicDimensionalExponents>,
        a_name: &Handle<HAsciiString>,
        a_conversion_factor: &Handle<StandardTransient>,
    ) {
        self.parent.init(a_dimensions, a_name, a_conversion_factor);
        self.state.write().ratio_unit = Handle::new(StepBasicRatioUnit::new());
        let field_ratio_unit = self.state.read().ratio_unit.clone();
        field_ratio_unit.init(a_dimensions);
    }
}

step_entity! {
    /// Conversion based unit and solid angle unit.
    StepBasicConversionBasedUnitAndSolidAngleUnit, StepBasicConversionBasedUnitAndSolidAngleUnitState, "StepBasicConversionBasedUnitAndSolidAngleUnit", StepBasicConversionBasedUnit, {
        solid_angle_unit: Handle<StepBasicSolidAngleUnit>,
    }
}
impl StepBasicConversionBasedUnitAndSolidAngleUnit {
    step_methods! {
        set set_solid_angle_unit = solid_angle_unit: Handle<StepBasicSolidAngleUnit>;
        get solid_angle_unit = solid_angle_unit: Handle<StepBasicSolidAngleUnit>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_dimensions: &Handle<StepBasicDimensionalExponents>,
        a_name: &Handle<HAsciiString>,
        a_conversion_factor: &Handle<StandardTransient>,
    ) {
        self.parent.init(a_dimensions, a_name, a_conversion_factor);
        self.state.write().solid_angle_unit = Handle::new(StepBasicSolidAngleUnit::new());
        let field_solid_angle_unit = self.state.read().solid_angle_unit.clone();
        field_solid_angle_unit.init(a_dimensions);
    }
}

step_entity! {
    /// Conversion based unit and time unit.
    StepBasicConversionBasedUnitAndTimeUnit, StepBasicConversionBasedUnitAndTimeUnitState, "StepBasicConversionBasedUnitAndTimeUnit", StepBasicConversionBasedUnit, {
        time_unit: Handle<StepBasicTimeUnit>,
    }
}
impl StepBasicConversionBasedUnitAndTimeUnit {
    step_methods! {
        set set_time_unit = time_unit: Handle<StepBasicTimeUnit>;
        get time_unit = time_unit: Handle<StepBasicTimeUnit>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_dimensions: &Handle<StepBasicDimensionalExponents>,
        a_name: &Handle<HAsciiString>,
        a_conversion_factor: &Handle<StandardTransient>,
    ) {
        self.parent.init(a_dimensions, a_name, a_conversion_factor);
        self.state.write().time_unit = Handle::new(StepBasicTimeUnit::new());
        let field_time_unit = self.state.read().time_unit.clone();
        field_time_unit.init(a_dimensions);
    }
}

step_entity! {
    /// Conversion based unit and volume unit.
    StepBasicConversionBasedUnitAndVolumeUnit, StepBasicConversionBasedUnitAndVolumeUnitState, "StepBasicConversionBasedUnitAndVolumeUnit", StepBasicConversionBasedUnit, {
        volume_unit: Handle<StepBasicVolumeUnit>,
    }
}
impl StepBasicConversionBasedUnitAndVolumeUnit {
    step_methods! {
        set set_volume_unit = volume_unit: Handle<StepBasicVolumeUnit>;
        get volume_unit = volume_unit: Handle<StepBasicVolumeUnit>;
    }
}

step_entity! {
    /// Coordinated universal time offset.
    StepBasicCoordinatedUniversalTimeOffset, StepBasicCoordinatedUniversalTimeOffsetState, "StepBasicCoordinatedUniversalTimeOffset", StandardTransient, {
        hour_offset: i32,
        minute_offset: i32,
        sense: StepBasicAheadOrBehind,
        has_minute_offset: bool,
    }
}
impl StepBasicCoordinatedUniversalTimeOffset {
    step_methods! {
        set_copy set_hour_offset = hour_offset: i32;
        get hour_offset = hour_offset: i32;
        set_copy set_minute_offset = minute_offset flag has_minute_offset: i32;
        clear un_set_minute_offset = has_minute_offset;
        get minute_offset = minute_offset: i32;
        get has_minute_offset = has_minute_offset: bool;
        set_copy set_sense = sense: StepBasicAheadOrBehind;
        get sense = sense: StepBasicAheadOrBehind;
    }
    /// `init`.
    pub fn init(
        &self,
        a_hour_offset: i32,
        has_aminute_offset: bool,
        a_minute_offset: i32,
        a_sense: StepBasicAheadOrBehind,
    ) {
        let mut state = self.state.write();
        state.hour_offset = a_hour_offset;
        state.has_minute_offset = has_aminute_offset;
        state.minute_offset = a_minute_offset;
        state.sense = a_sense;
    }
}
