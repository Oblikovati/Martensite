#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

step_entity! {
    /// Representation of STEP entity IdentificationAssignment
    StepBasicIdentificationAssignment, StepBasicIdentificationAssignmentState, "StepBasicIdentificationAssignment", StandardTransient, {
        the_assigned_id: Handle<HAsciiString>,
        the_role: Handle<StepBasicIdentificationRole>,
    }
}
impl StepBasicIdentificationAssignment {
    step_methods! {
        get assigned_id = the_assigned_id: Handle<HAsciiString>;
        set set_assigned_id = the_assigned_id: Handle<HAsciiString>;
        get role = the_role: Handle<StepBasicIdentificationRole>;
        set set_role = the_role: Handle<StepBasicIdentificationRole>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_assigned_id: &Handle<HAsciiString>,
        a_role: &Handle<StepBasicIdentificationRole>,
    ) {
        let mut state = self.state.write();
        state.the_assigned_id = a_assigned_id.clone();
        state.the_role = a_role.clone();
    }
}

step_entity! {
    /// Representation of STEP entity IdentificationRole
    StepBasicIdentificationRole, StepBasicIdentificationRoleState, "StepBasicIdentificationRole", StandardTransient, {
        the_name: Handle<HAsciiString>,
        the_description: Handle<HAsciiString>,
        def_description: bool,
    }
}
impl StepBasicIdentificationRole {
    step_methods! {
        get name = the_name: Handle<HAsciiString>;
        set set_name = the_name: Handle<HAsciiString>;
        get description = the_description: Handle<HAsciiString>;
        set set_description = the_description: Handle<HAsciiString>;
        get has_description = def_description: bool;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        has_description: bool,
        a_description: &Handle<HAsciiString>,
    ) {
        let mut state = self.state.write();
        state.the_name = a_name.clone();
        state.def_description = has_description;
        if state.def_description {
            state.the_description = a_description.clone();
        } else {
            state.the_description = Default::default();
        }
    }
}

step_entity! {
    /// Length measure with unit.
    StepBasicLengthMeasureWithUnit, StepBasicLengthMeasureWithUnitState, "StepBasicLengthMeasureWithUnit", StepBasicMeasureWithUnit, {
    }
}

step_entity! {
    /// Length unit.
    StepBasicLengthUnit, StepBasicLengthUnitState, "StepBasicLengthUnit", StepBasicNamedUnit, {
    }
}

step_entity! {
    /// Local time.
    StepBasicLocalTime, StepBasicLocalTimeState, "StepBasicLocalTime", StandardTransient, {
        hour_component: i32,
        minute_component: i32,
        second_component: f64,
        zone: Handle<StepBasicCoordinatedUniversalTimeOffset>,
        has_minute_component: bool,
        has_second_component: bool,
    }
}
impl StepBasicLocalTime {
    step_methods! {
        set_copy set_hour_component = hour_component: i32;
        get hour_component = hour_component: i32;
        set_copy set_minute_component = minute_component flag has_minute_component: i32;
        clear un_set_minute_component = has_minute_component;
        get minute_component = minute_component: i32;
        get has_minute_component = has_minute_component: bool;
        set_copy set_second_component = second_component flag has_second_component: f64;
        clear un_set_second_component = has_second_component;
        get second_component = second_component: f64;
        get has_second_component = has_second_component: bool;
        set set_zone = zone: Handle<StepBasicCoordinatedUniversalTimeOffset>;
        get zone = zone: Handle<StepBasicCoordinatedUniversalTimeOffset>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_hour_component: i32,
        has_aminute_component: bool,
        a_minute_component: i32,
        has_asecond_component: bool,
        a_second_component: f64,
        a_zone: &Handle<StepBasicCoordinatedUniversalTimeOffset>,
    ) {
        let mut state = self.state.write();
        state.hour_component = a_hour_component;
        state.has_minute_component = has_aminute_component;
        state.minute_component = a_minute_component;
        state.has_second_component = has_asecond_component;
        state.second_component = a_second_component;
        state.zone = a_zone.clone();
    }
}

step_entity! {
    /// Mass measure with unit.
    StepBasicMassMeasureWithUnit, StepBasicMassMeasureWithUnitState, "StepBasicMassMeasureWithUnit", StepBasicMeasureWithUnit, {
    }
}

step_entity! {
    /// Representation of STEP entity MassUnit
    StepBasicMassUnit, StepBasicMassUnitState, "StepBasicMassUnit", StepBasicNamedUnit, {
    }
}

step_entity! {
    /// Measure with unit.
    StepBasicMeasureWithUnit, StepBasicMeasureWithUnitState, "StepBasicMeasureWithUnit", StandardTransient, {
        value_component: Handle<StepBasicMeasureValueMember>,
        unit_component: StepBasicUnit,
    }
}
impl StepBasicMeasureWithUnit {
    step_methods! {
        get value_component_member = value_component: Handle<StepBasicMeasureValueMember>;
        set set_value_component_member = value_component: Handle<StepBasicMeasureValueMember>;
        set set_unit_component = unit_component: StepBasicUnit;
        get unit_component = unit_component: StepBasicUnit;
    }
    /// `init`.
    pub fn init(
        &self,
        a_value_component: &Handle<StepBasicMeasureValueMember>,
        a_unit_component: &StepBasicUnit,
    ) {
        let mut state = self.state.write();
        state.value_component = a_value_component.clone();
        state.unit_component = a_unit_component.clone();
    }
    /// `set_value_component`.
    pub fn set_value_component(&self, a_value_component: f64) {
        let mut state = self.state.write();
        if state.value_component.is_null() {
            state.value_component = Handle::new(StepBasicMeasureValueMember::new());
        }
        drop(state);
        let field_value_component = self.state.read().value_component.clone();
        field_value_component.set_real(a_value_component);
    }
    /// `value_component`.
    pub fn value_component(&self) -> f64 {
        let field_value_component = self.state.read().value_component.clone();
        if self.state.read().value_component.is_null() {
            0.0
        } else {
            field_value_component.real()
        }
    }
}

step_entity! {
    /// Mechanical context.
    StepBasicMechanicalContext, StepBasicMechanicalContextState, "StepBasicMechanicalContext", StepBasicProductContext, {
    }
}

step_entity! {
    /// Representation of STEP entity NameAssignment
    StepBasicNameAssignment, StepBasicNameAssignmentState, "StepBasicNameAssignment", StandardTransient, {
        the_assigned_name: Handle<HAsciiString>,
    }
}
impl StepBasicNameAssignment {
    step_methods! {
        set init = the_assigned_name: Handle<HAsciiString>;
        get assigned_name = the_assigned_name: Handle<HAsciiString>;
        set set_assigned_name = the_assigned_name: Handle<HAsciiString>;
    }
}

step_entity! {
    /// Representation of STEP entity ObjectRole
    StepBasicObjectRole, StepBasicObjectRoleState, "StepBasicObjectRole", StandardTransient, {
        the_name: Handle<HAsciiString>,
        the_description: Handle<HAsciiString>,
        def_description: bool,
    }
}
impl StepBasicObjectRole {
    step_methods! {
        get name = the_name: Handle<HAsciiString>;
        set set_name = the_name: Handle<HAsciiString>;
        get description = the_description: Handle<HAsciiString>;
        set set_description = the_description: Handle<HAsciiString>;
        get has_description = def_description: bool;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        has_description: bool,
        a_description: &Handle<HAsciiString>,
    ) {
        let mut state = self.state.write();
        state.the_name = a_name.clone();
        state.def_description = has_description;
        if state.def_description {
            state.the_description = a_description.clone();
        } else {
            state.the_description = Default::default();
        }
    }
}

step_entity! {
    /// Ordinal date.
    StepBasicOrdinalDate, StepBasicOrdinalDateState, "StepBasicOrdinalDate", StepBasicDate, {
        day_component: i32,
    }
}
impl StepBasicOrdinalDate {
    step_methods! {
        set_copy set_day_component = day_component: i32;
        get day_component = day_component: i32;
    }
    /// `init`.
    pub fn init(&self, a_year_component: i32, a_day_component: i32) {
        self.state.write().day_component = a_day_component;
        self.parent.init(a_year_component);
    }
}

step_entity! {
    /// Organization.
    StepBasicOrganization, StepBasicOrganizationState, "StepBasicOrganization", StandardTransient, {
        id: Handle<HAsciiString>,
        name: Handle<HAsciiString>,
        description: Handle<HAsciiString>,
        has_id: bool,
    }
}
impl StepBasicOrganization {
    step_methods! {
        set set_id = id flag has_id: Handle<HAsciiString>;
        unset un_set_id = id flag has_id;
        get id = id: Handle<HAsciiString>;
        get has_id = has_id: bool;
        set set_name = name: Handle<HAsciiString>;
        get name = name: Handle<HAsciiString>;
        set set_description = description: Handle<HAsciiString>;
        get description = description: Handle<HAsciiString>;
    }
    /// `init`.
    pub fn init(
        &self,
        has_aid: bool,
        a_id: &Handle<HAsciiString>,
        a_name: &Handle<HAsciiString>,
        a_description: &Handle<HAsciiString>,
    ) {
        let mut state = self.state.write();
        state.has_id = has_aid;
        state.id = a_id.clone();
        state.name = a_name.clone();
        state.description = a_description.clone();
    }
}
