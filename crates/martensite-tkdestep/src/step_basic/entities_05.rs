#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

step_entity! {
    /// Date.
    StepBasicDate, StepBasicDateState, "StepBasicDate", StandardTransient, {
        year_component: i32,
    }
}
impl StepBasicDate {
    step_methods! {
        set_copy init = year_component: i32;
        set_copy set_year_component = year_component: i32;
        get year_component = year_component: i32;
    }
}

step_entity! {
    /// Date and time.
    StepBasicDateAndTime, StepBasicDateAndTimeState, "StepBasicDateAndTime", StandardTransient, {
        date_component: Handle<StepBasicDate>,
        time_component: Handle<StepBasicLocalTime>,
    }
}
impl StepBasicDateAndTime {
    step_methods! {
        set set_date_component = date_component: Handle<StepBasicDate>;
        get date_component = date_component: Handle<StepBasicDate>;
        set set_time_component = time_component: Handle<StepBasicLocalTime>;
        get time_component = time_component: Handle<StepBasicLocalTime>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_date_component: &Handle<StepBasicDate>,
        a_time_component: &Handle<StepBasicLocalTime>,
    ) {
        let mut state = self.state.write();
        state.date_component = a_date_component.clone();
        state.time_component = a_time_component.clone();
    }
}

step_entity! {
    /// Date and time assignment.
    StepBasicDateAndTimeAssignment, StepBasicDateAndTimeAssignmentState, "StepBasicDateAndTimeAssignment", StandardTransient, {
        assigned_date_and_time: Handle<StepBasicDateAndTime>,
        role: Handle<StepBasicDateTimeRole>,
    }
}
impl StepBasicDateAndTimeAssignment {
    step_methods! {
        set set_assigned_date_and_time = assigned_date_and_time: Handle<StepBasicDateAndTime>;
        get assigned_date_and_time = assigned_date_and_time: Handle<StepBasicDateAndTime>;
        set set_role = role: Handle<StepBasicDateTimeRole>;
        get role = role: Handle<StepBasicDateTimeRole>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_assigned_date_and_time: &Handle<StepBasicDateAndTime>,
        a_role: &Handle<StepBasicDateTimeRole>,
    ) {
        let mut state = self.state.write();
        state.assigned_date_and_time = a_assigned_date_and_time.clone();
        state.role = a_role.clone();
    }
}

step_entity! {
    /// Date assignment.
    StepBasicDateAssignment, StepBasicDateAssignmentState, "StepBasicDateAssignment", StandardTransient, {
        assigned_date: Handle<StepBasicDate>,
        role: Handle<StepBasicDateRole>,
    }
}
impl StepBasicDateAssignment {
    step_methods! {
        set set_assigned_date = assigned_date: Handle<StepBasicDate>;
        get assigned_date = assigned_date: Handle<StepBasicDate>;
        set set_role = role: Handle<StepBasicDateRole>;
        get role = role: Handle<StepBasicDateRole>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_assigned_date: &Handle<StepBasicDate>,
        a_role: &Handle<StepBasicDateRole>,
    ) {
        let mut state = self.state.write();
        state.assigned_date = a_assigned_date.clone();
        state.role = a_role.clone();
    }
}

step_entity! {
    /// Date role.
    StepBasicDateRole, StepBasicDateRoleState, "StepBasicDateRole", StandardTransient, {
        name: Handle<HAsciiString>,
    }
}
impl StepBasicDateRole {
    step_methods! {
        set init = name: Handle<HAsciiString>;
        set set_name = name: Handle<HAsciiString>;
        get name = name: Handle<HAsciiString>;
    }
}

step_entity! {
    /// Date time role.
    StepBasicDateTimeRole, StepBasicDateTimeRoleState, "StepBasicDateTimeRole", StandardTransient, {
        name: Handle<HAsciiString>,
    }
}
impl StepBasicDateTimeRole {
    step_methods! {
        set init = name: Handle<HAsciiString>;
        set set_name = name: Handle<HAsciiString>;
        get name = name: Handle<HAsciiString>;
    }
}

step_entity! {
    /// Added from StepBasic Rev2 to Rev4
    StepBasicDerivedUnit, StepBasicDerivedUnitState, "StepBasicDerivedUnit", StandardTransient, {
        the_elements: Handle<HArray1<Handle<StepBasicDerivedUnitElement>>>,
    }
}
impl StepBasicDerivedUnit {
    step_methods! {
        set init = the_elements: Handle<HArray1<Handle<StepBasicDerivedUnitElement>>>;
        set set_elements = the_elements: Handle<HArray1<Handle<StepBasicDerivedUnitElement>>>;
        get elements = the_elements: Handle<HArray1<Handle<StepBasicDerivedUnitElement>>>;
        len_checked nb_elements = the_elements;
        item elements_value = the_elements: Handle<StepBasicDerivedUnitElement>;
    }
}

step_entity! {
    /// Added from StepBasic Rev2 to Rev4
    StepBasicDerivedUnitElement, StepBasicDerivedUnitElementState, "StepBasicDerivedUnitElement", StandardTransient, {
        the_unit: Handle<StepBasicNamedUnit>,
        the_exponent: f64,
    }
}
impl StepBasicDerivedUnitElement {
    step_methods! {
        set set_unit = the_unit: Handle<StepBasicNamedUnit>;
        get unit = the_unit: Handle<StepBasicNamedUnit>;
        set_copy set_exponent = the_exponent: f64;
        get exponent = the_exponent: f64;
    }
    /// `init`.
    pub fn init(&self, a_unit: &Handle<StepBasicNamedUnit>, a_exponent: f64) {
        let mut state = self.state.write();
        state.the_unit = a_unit.clone();
        state.the_exponent = a_exponent;
    }
}

step_entity! {
    /// class added to Schema AP214 around April 1996
    StepBasicDesignContext, StepBasicDesignContextState, "StepBasicDesignContext", StepBasicProductDefinitionContext, {
    }
}

step_entity! {
    /// Digital document.
    StepBasicDigitalDocument, StepBasicDigitalDocumentState, "StepBasicDigitalDocument", StepBasicDocument, {
    }
}

step_entity! {
    /// Dimensional exponents.
    StepBasicDimensionalExponents, StepBasicDimensionalExponentsState, "StepBasicDimensionalExponents", StandardTransient, {
        length_exponent: f64,
        mass_exponent: f64,
        time_exponent: f64,
        electric_current_exponent: f64,
        thermodynamic_temperature_exponent: f64,
        amount_of_substance_exponent: f64,
        luminous_intensity_exponent: f64,
    }
}
impl StepBasicDimensionalExponents {
    step_methods! {
        set_copy set_length_exponent = length_exponent: f64;
        get length_exponent = length_exponent: f64;
        set_copy set_mass_exponent = mass_exponent: f64;
        get mass_exponent = mass_exponent: f64;
        set_copy set_time_exponent = time_exponent: f64;
        get time_exponent = time_exponent: f64;
        set_copy set_electric_current_exponent = electric_current_exponent: f64;
        get electric_current_exponent = electric_current_exponent: f64;
        set_copy set_thermodynamic_temperature_exponent = thermodynamic_temperature_exponent: f64;
        get thermodynamic_temperature_exponent = thermodynamic_temperature_exponent: f64;
        set_copy set_amount_of_substance_exponent = amount_of_substance_exponent: f64;
        get amount_of_substance_exponent = amount_of_substance_exponent: f64;
        set_copy set_luminous_intensity_exponent = luminous_intensity_exponent: f64;
        get luminous_intensity_exponent = luminous_intensity_exponent: f64;
    }
    /// `init`.
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        a_length_exponent: f64,
        a_mass_exponent: f64,
        a_time_exponent: f64,
        a_electric_current_exponent: f64,
        a_thermodynamic_temperature_exponent: f64,
        a_amount_of_substance_exponent: f64,
        a_luminous_intensity_exponent: f64,
    ) {
        let mut state = self.state.write();
        state.length_exponent = a_length_exponent;
        state.mass_exponent = a_mass_exponent;
        state.time_exponent = a_time_exponent;
        state.electric_current_exponent = a_electric_current_exponent;
        state.thermodynamic_temperature_exponent = a_thermodynamic_temperature_exponent;
        state.amount_of_substance_exponent = a_amount_of_substance_exponent;
        state.luminous_intensity_exponent = a_luminous_intensity_exponent;
    }
}

step_entity! {
    /// Representation of STEP entity Document
    StepBasicDocument, StepBasicDocumentState, "StepBasicDocument", StandardTransient, {
        the_id: Handle<HAsciiString>,
        the_name: Handle<HAsciiString>,
        the_description: Handle<HAsciiString>,
        the_kind: Handle<StepBasicDocumentType>,
        def_description: bool,
    }
}
impl StepBasicDocument {
    step_methods! {
        get id = the_id: Handle<HAsciiString>;
        set set_id = the_id: Handle<HAsciiString>;
        get name = the_name: Handle<HAsciiString>;
        set set_name = the_name: Handle<HAsciiString>;
        get description = the_description: Handle<HAsciiString>;
        set set_description = the_description: Handle<HAsciiString>;
        get has_description = def_description: bool;
        get kind = the_kind: Handle<StepBasicDocumentType>;
        set set_kind = the_kind: Handle<StepBasicDocumentType>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_id: &Handle<HAsciiString>,
        a_name: &Handle<HAsciiString>,
        has_description: bool,
        a_description: &Handle<HAsciiString>,
        a_kind: &Handle<StepBasicDocumentType>,
    ) {
        let mut state = self.state.write();
        state.the_id = a_id.clone();
        state.the_name = a_name.clone();
        state.def_description = has_description;
        if state.def_description {
            state.the_description = a_description.clone();
        } else {
            state.the_description = Default::default();
        }
        state.the_kind = a_kind.clone();
    }
}
