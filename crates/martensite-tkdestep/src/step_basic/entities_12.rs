#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

step_entity! {
    /// Security classification.
    StepBasicSecurityClassification, StepBasicSecurityClassificationState, "StepBasicSecurityClassification", StandardTransient, {
        name: Handle<HAsciiString>,
        purpose: Handle<HAsciiString>,
        security_level: Handle<StepBasicSecurityClassificationLevel>,
    }
}
impl StepBasicSecurityClassification {
    step_methods! {
        set set_name = name: Handle<HAsciiString>;
        get name = name: Handle<HAsciiString>;
        set set_purpose = purpose: Handle<HAsciiString>;
        get purpose = purpose: Handle<HAsciiString>;
        set set_security_level = security_level: Handle<StepBasicSecurityClassificationLevel>;
        get security_level = security_level: Handle<StepBasicSecurityClassificationLevel>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_purpose: &Handle<HAsciiString>,
        a_security_level: &Handle<StepBasicSecurityClassificationLevel>,
    ) {
        let mut state = self.state.write();
        state.name = a_name.clone();
        state.purpose = a_purpose.clone();
        state.security_level = a_security_level.clone();
    }
}

step_entity! {
    /// Security classification assignment.
    StepBasicSecurityClassificationAssignment, StepBasicSecurityClassificationAssignmentState, "StepBasicSecurityClassificationAssignment", StandardTransient, {
        assigned_security_classification: Handle<StepBasicSecurityClassification>,
    }
}
impl StepBasicSecurityClassificationAssignment {
    step_methods! {
        set init = assigned_security_classification: Handle<StepBasicSecurityClassification>;
        set set_assigned_security_classification = assigned_security_classification: Handle<StepBasicSecurityClassification>;
        get assigned_security_classification = assigned_security_classification: Handle<StepBasicSecurityClassification>;
    }
}

step_entity! {
    /// Security classification level.
    StepBasicSecurityClassificationLevel, StepBasicSecurityClassificationLevelState, "StepBasicSecurityClassificationLevel", StandardTransient, {
        name: Handle<HAsciiString>,
    }
}
impl StepBasicSecurityClassificationLevel {
    step_methods! {
        set init = name: Handle<HAsciiString>;
        set set_name = name: Handle<HAsciiString>;
        get name = name: Handle<HAsciiString>;
    }
}

step_entity! {
    /// Si unit and area unit.
    StepBasicSiUnitAndAreaUnit, StepBasicSiUnitAndAreaUnitState, "StepBasicSiUnitAndAreaUnit", StepBasicSiUnit, {
        area_unit: Handle<StepBasicAreaUnit>,
    }
}
impl StepBasicSiUnitAndAreaUnit {
    step_methods! {
        set set_area_unit = area_unit: Handle<StepBasicAreaUnit>;
        get area_unit = area_unit: Handle<StepBasicAreaUnit>;
    }
}

step_entity! {
    /// Si unit and length unit.
    StepBasicSiUnitAndLengthUnit, StepBasicSiUnitAndLengthUnitState, "StepBasicSiUnitAndLengthUnit", StepBasicSiUnit, {
        length_unit: Handle<StepBasicLengthUnit>,
    }
}
impl StepBasicSiUnitAndLengthUnit {
    step_methods! {
        set set_length_unit = length_unit: Handle<StepBasicLengthUnit>;
        get length_unit = length_unit: Handle<StepBasicLengthUnit>;
    }
    /// `init`.
    pub fn init(
        &self,
        has_aprefix: bool,
        a_prefix: StepBasicSiPrefix,
        a_name: StepBasicSiUnitName,
    ) {
        self.state.write().length_unit = Handle::new(StepBasicLengthUnit::new());
        let a_dimensions = Handle::<StepBasicDimensionalExponents>::default();
        let field_length_unit = self.state.read().length_unit.clone();
        field_length_unit.init(&a_dimensions);
        self.parent.init(has_aprefix, a_prefix, a_name);
    }
}

step_entity! {
    /// Si unit and mass unit.
    StepBasicSiUnitAndMassUnit, StepBasicSiUnitAndMassUnitState, "StepBasicSiUnitAndMassUnit", StepBasicSiUnit, {
        mass_unit: Handle<StepBasicMassUnit>,
    }
}
impl StepBasicSiUnitAndMassUnit {
    step_methods! {
        set set_mass_unit = mass_unit: Handle<StepBasicMassUnit>;
        get mass_unit = mass_unit: Handle<StepBasicMassUnit>;
    }
    /// `init`.
    pub fn init(
        &self,
        has_aprefix: bool,
        a_prefix: StepBasicSiPrefix,
        a_name: StepBasicSiUnitName,
    ) {
        self.state.write().mass_unit = Handle::new(StepBasicMassUnit::new());
        let a_dimensions = Handle::<StepBasicDimensionalExponents>::default();
        let field_mass_unit = self.state.read().mass_unit.clone();
        field_mass_unit.init(&a_dimensions);
        self.parent.init(has_aprefix, a_prefix, a_name);
    }
}

step_entity! {
    /// Si unit and plane angle unit.
    StepBasicSiUnitAndPlaneAngleUnit, StepBasicSiUnitAndPlaneAngleUnitState, "StepBasicSiUnitAndPlaneAngleUnit", StepBasicSiUnit, {
        plane_angle_unit: Handle<StepBasicPlaneAngleUnit>,
    }
}
impl StepBasicSiUnitAndPlaneAngleUnit {
    step_methods! {
        set set_plane_angle_unit = plane_angle_unit: Handle<StepBasicPlaneAngleUnit>;
        get plane_angle_unit = plane_angle_unit: Handle<StepBasicPlaneAngleUnit>;
    }
    /// `init`.
    pub fn init(
        &self,
        has_aprefix: bool,
        a_prefix: StepBasicSiPrefix,
        a_name: StepBasicSiUnitName,
    ) {
        self.state.write().plane_angle_unit = Handle::new(StepBasicPlaneAngleUnit::new());
        let a_dimensions = Handle::<StepBasicDimensionalExponents>::default();
        let field_plane_angle_unit = self.state.read().plane_angle_unit.clone();
        field_plane_angle_unit.init(&a_dimensions);
        self.parent.init(has_aprefix, a_prefix, a_name);
    }
}

step_entity! {
    /// Si unit and ratio unit.
    StepBasicSiUnitAndRatioUnit, StepBasicSiUnitAndRatioUnitState, "StepBasicSiUnitAndRatioUnit", StepBasicSiUnit, {
        ratio_unit: Handle<StepBasicRatioUnit>,
    }
}
impl StepBasicSiUnitAndRatioUnit {
    step_methods! {
        set set_ratio_unit = ratio_unit: Handle<StepBasicRatioUnit>;
        get ratio_unit = ratio_unit: Handle<StepBasicRatioUnit>;
    }
    /// `init`.
    pub fn init(
        &self,
        has_aprefix: bool,
        a_prefix: StepBasicSiPrefix,
        a_name: StepBasicSiUnitName,
    ) {
        self.parent.init(has_aprefix, a_prefix, a_name);
        self.state.write().ratio_unit = Handle::new(StepBasicRatioUnit::new());
        let a_dimensions = Handle::<StepBasicDimensionalExponents>::default();
        let field_ratio_unit = self.state.read().ratio_unit.clone();
        field_ratio_unit.init(&a_dimensions);
    }
}

step_entity! {
    /// Si unit and solid angle unit.
    StepBasicSiUnitAndSolidAngleUnit, StepBasicSiUnitAndSolidAngleUnitState, "StepBasicSiUnitAndSolidAngleUnit", StepBasicSiUnit, {
        solid_angle_unit: Handle<StepBasicSolidAngleUnit>,
    }
}
impl StepBasicSiUnitAndSolidAngleUnit {
    step_methods! {
        set set_solid_angle_unit = solid_angle_unit: Handle<StepBasicSolidAngleUnit>;
        get solid_angle_unit = solid_angle_unit: Handle<StepBasicSolidAngleUnit>;
    }
    /// `init`.
    pub fn init(
        &self,
        has_aprefix: bool,
        a_prefix: StepBasicSiPrefix,
        a_name: StepBasicSiUnitName,
    ) {
        self.parent.init(has_aprefix, a_prefix, a_name);
        self.state.write().solid_angle_unit = Handle::new(StepBasicSolidAngleUnit::new());
        let a_dimensions = Handle::<StepBasicDimensionalExponents>::default();
        let field_solid_angle_unit = self.state.read().solid_angle_unit.clone();
        field_solid_angle_unit.init(&a_dimensions);
    }
}

step_entity! {
    /// Si unit and thermodynamic temperature unit.
    StepBasicSiUnitAndThermodynamicTemperatureUnit, StepBasicSiUnitAndThermodynamicTemperatureUnitState, "StepBasicSiUnitAndThermodynamicTemperatureUnit", StepBasicSiUnit, {
        thermodynamic_temperature_unit: Handle<StepBasicThermodynamicTemperatureUnit>,
    }
}
impl StepBasicSiUnitAndThermodynamicTemperatureUnit {
    step_methods! {
        set set_thermodynamic_temperature_unit = thermodynamic_temperature_unit: Handle<StepBasicThermodynamicTemperatureUnit>;
        get thermodynamic_temperature_unit = thermodynamic_temperature_unit: Handle<StepBasicThermodynamicTemperatureUnit>;
    }
    /// `init`.
    pub fn init(
        &self,
        has_aprefix: bool,
        a_prefix: StepBasicSiPrefix,
        a_name: StepBasicSiUnitName,
    ) {
        self.parent.init(has_aprefix, a_prefix, a_name);
        self.state.write().thermodynamic_temperature_unit =
            Handle::new(StepBasicThermodynamicTemperatureUnit::new());
        let a_dimensions = Handle::<StepBasicDimensionalExponents>::default();
        let field_thermodynamic_temperature_unit =
            self.state.read().thermodynamic_temperature_unit.clone();
        field_thermodynamic_temperature_unit.init(&a_dimensions);
    }
}

step_entity! {
    /// Si unit and time unit.
    StepBasicSiUnitAndTimeUnit, StepBasicSiUnitAndTimeUnitState, "StepBasicSiUnitAndTimeUnit", StepBasicSiUnit, {
        time_unit: Handle<StepBasicTimeUnit>,
    }
}
impl StepBasicSiUnitAndTimeUnit {
    step_methods! {
        set set_time_unit = time_unit: Handle<StepBasicTimeUnit>;
        get time_unit = time_unit: Handle<StepBasicTimeUnit>;
    }
    /// `init`.
    pub fn init(
        &self,
        has_aprefix: bool,
        a_prefix: StepBasicSiPrefix,
        a_name: StepBasicSiUnitName,
    ) {
        self.parent.init(has_aprefix, a_prefix, a_name);
        self.state.write().time_unit = Handle::new(StepBasicTimeUnit::new());
        let a_dimensions = Handle::<StepBasicDimensionalExponents>::default();
        let field_time_unit = self.state.read().time_unit.clone();
        field_time_unit.init(&a_dimensions);
    }
}

step_entity! {
    /// Si unit and volume unit.
    StepBasicSiUnitAndVolumeUnit, StepBasicSiUnitAndVolumeUnitState, "StepBasicSiUnitAndVolumeUnit", StepBasicSiUnit, {
        volume_unit: Handle<StepBasicVolumeUnit>,
    }
}
impl StepBasicSiUnitAndVolumeUnit {
    step_methods! {
        set set_volume_unit = volume_unit: Handle<StepBasicVolumeUnit>;
        get volume_unit = volume_unit: Handle<StepBasicVolumeUnit>;
    }
}

step_entity! {
    /// Solid angle measure with unit.
    StepBasicSolidAngleMeasureWithUnit, StepBasicSolidAngleMeasureWithUnitState, "StepBasicSolidAngleMeasureWithUnit", StepBasicMeasureWithUnit, {
    }
}

step_entity! {
    /// Solid angle unit.
    StepBasicSolidAngleUnit, StepBasicSolidAngleUnitState, "StepBasicSolidAngleUnit", StepBasicNamedUnit, {
    }
}

step_entity! {
    /// Representation of STEP entity ThermodynamicTemperatureUnit
    StepBasicThermodynamicTemperatureUnit, StepBasicThermodynamicTemperatureUnitState, "StepBasicThermodynamicTemperatureUnit", StepBasicNamedUnit, {
    }
}

step_entity! {
    /// Time measure with unit.
    StepBasicTimeMeasureWithUnit, StepBasicTimeMeasureWithUnitState, "StepBasicTimeMeasureWithUnit", StepBasicMeasureWithUnit, {
    }
}

step_entity! {
    /// Time unit.
    StepBasicTimeUnit, StepBasicTimeUnitState, "StepBasicTimeUnit", StepBasicNamedUnit, {
    }
}
