#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

step_entity! {
    /// Uncertainty measure with unit.
    StepBasicUncertaintyMeasureWithUnit, StepBasicUncertaintyMeasureWithUnitState, "StepBasicUncertaintyMeasureWithUnit", StepBasicMeasureWithUnit, {
        name: Handle<HAsciiString>,
        description: Handle<HAsciiString>,
    }
}
impl StepBasicUncertaintyMeasureWithUnit {
    step_methods! {
        set set_name = name: Handle<HAsciiString>;
        get name = name: Handle<HAsciiString>;
        set set_description = description: Handle<HAsciiString>;
        get description = description: Handle<HAsciiString>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_value_component: &Handle<StepBasicMeasureValueMember>,
        a_unit_component: &StepBasicUnit,
        a_name: &Handle<HAsciiString>,
        a_description: &Handle<HAsciiString>,
    ) {
        let mut state = self.state.write();
        state.name = a_name.clone();
        state.description = a_description.clone();
        drop(state);
        self.parent.init(a_value_component, a_unit_component);
    }
}

step_entity! {
    /// Representation of STEP entity VersionedActionRequest
    StepBasicVersionedActionRequest, StepBasicVersionedActionRequestState, "StepBasicVersionedActionRequest", StandardTransient, {
        the_id: Handle<HAsciiString>,
        the_version: Handle<HAsciiString>,
        the_purpose: Handle<HAsciiString>,
        the_description: Handle<HAsciiString>,
        def_description: bool,
    }
}
impl StepBasicVersionedActionRequest {
    step_methods! {
        get id = the_id: Handle<HAsciiString>;
        set set_id = the_id: Handle<HAsciiString>;
        get version = the_version: Handle<HAsciiString>;
        set set_version = the_version: Handle<HAsciiString>;
        get purpose = the_purpose: Handle<HAsciiString>;
        set set_purpose = the_purpose: Handle<HAsciiString>;
        get description = the_description: Handle<HAsciiString>;
        set set_description = the_description: Handle<HAsciiString>;
        get has_description = def_description: bool;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_id: &Handle<HAsciiString>,
        a_version: &Handle<HAsciiString>,
        a_purpose: &Handle<HAsciiString>,
        has_description: bool,
        a_description: &Handle<HAsciiString>,
    ) {
        let mut state = self.state.write();
        state.the_id = a_id.clone();
        state.the_version = a_version.clone();
        state.the_purpose = a_purpose.clone();
        state.def_description = has_description;
        if state.def_description {
            state.the_description = a_description.clone();
        } else {
            state.the_description = Default::default();
        }
    }
}

step_entity! {
    /// Volume unit.
    StepBasicVolumeUnit, StepBasicVolumeUnitState, "StepBasicVolumeUnit", StepBasicNamedUnit, {
    }
}

step_entity! {
    /// Week of year and day date.
    StepBasicWeekOfYearAndDayDate, StepBasicWeekOfYearAndDayDateState, "StepBasicWeekOfYearAndDayDate", StepBasicDate, {
        week_component: i32,
        day_component: i32,
        has_day_component: bool,
    }
}
impl StepBasicWeekOfYearAndDayDate {
    step_methods! {
        set_copy set_week_component = week_component: i32;
        get week_component = week_component: i32;
        set_copy set_day_component = day_component flag has_day_component: i32;
        clear un_set_day_component = has_day_component;
        get day_component = day_component: i32;
        get has_day_component = has_day_component: bool;
    }
    /// `init`.
    pub fn init(
        &self,
        a_year_component: i32,
        a_week_component: i32,
        has_aday_component: bool,
        a_day_component: i32,
    ) {
        let mut state = self.state.write();
        state.week_component = a_week_component;
        state.has_day_component = has_aday_component;
        state.day_component = a_day_component;
        drop(state);
        self.parent.init(a_year_component);
    }
}
