#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

step_entity! {
    /// Representation of STEP entity ActionMethod
    StepBasicActionMethod, StepBasicActionMethodState, "StepBasicActionMethod", StandardTransient, {
        the_name: Handle<HAsciiString>,
        the_description: Handle<HAsciiString>,
        the_consequence: Handle<HAsciiString>,
        the_purpose: Handle<HAsciiString>,
        def_description: bool,
    }
}
impl StepBasicActionMethod {
    step_methods! {
        get name = the_name: Handle<HAsciiString>;
        set set_name = the_name: Handle<HAsciiString>;
        get description = the_description: Handle<HAsciiString>;
        set set_description = the_description: Handle<HAsciiString>;
        get has_description = def_description: bool;
        get consequence = the_consequence: Handle<HAsciiString>;
        set set_consequence = the_consequence: Handle<HAsciiString>;
        get purpose = the_purpose: Handle<HAsciiString>;
        set set_purpose = the_purpose: Handle<HAsciiString>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        has_description: bool,
        a_description: &Handle<HAsciiString>,
        a_consequence: &Handle<HAsciiString>,
        a_purpose: &Handle<HAsciiString>,
    ) {
        let mut state = self.state.write();
        state.the_name = a_name.clone();
        state.def_description = has_description;
        if state.def_description {
            state.the_description = a_description.clone();
        } else {
            state.the_description = Default::default();
        }
        state.the_consequence = a_consequence.clone();
        state.the_purpose = a_purpose.clone();
    }
}

step_entity! {
    /// Representation of STEP entity ActionRequestAssignment
    StepBasicActionRequestAssignment, StepBasicActionRequestAssignmentState, "StepBasicActionRequestAssignment", StandardTransient, {
        the_assigned_action_request: Handle<StepBasicVersionedActionRequest>,
    }
}
impl StepBasicActionRequestAssignment {
    step_methods! {
        set init = the_assigned_action_request: Handle<StepBasicVersionedActionRequest>;
        get assigned_action_request = the_assigned_action_request: Handle<StepBasicVersionedActionRequest>;
        set set_assigned_action_request = the_assigned_action_request: Handle<StepBasicVersionedActionRequest>;
    }
}

step_entity! {
    /// Representation of STEP entity ActionRequestSolution
    StepBasicActionRequestSolution, StepBasicActionRequestSolutionState, "StepBasicActionRequestSolution", StandardTransient, {
        the_method: Handle<StepBasicActionMethod>,
        the_request: Handle<StepBasicVersionedActionRequest>,
    }
}
impl StepBasicActionRequestSolution {
    step_methods! {
        get method = the_method: Handle<StepBasicActionMethod>;
        set set_method = the_method: Handle<StepBasicActionMethod>;
        get request = the_request: Handle<StepBasicVersionedActionRequest>;
        set set_request = the_request: Handle<StepBasicVersionedActionRequest>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_method: &Handle<StepBasicActionMethod>,
        a_request: &Handle<StepBasicVersionedActionRequest>,
    ) {
        let mut state = self.state.write();
        state.the_method = a_method.clone();
        state.the_request = a_request.clone();
    }
}

step_entity! {
    /// Address.
    StepBasicAddress, StepBasicAddressState, "StepBasicAddress", StandardTransient, {
        internal_location: Handle<HAsciiString>,
        street_number: Handle<HAsciiString>,
        street: Handle<HAsciiString>,
        postal_box: Handle<HAsciiString>,
        town: Handle<HAsciiString>,
        region: Handle<HAsciiString>,
        postal_code: Handle<HAsciiString>,
        country: Handle<HAsciiString>,
        facsimile_number: Handle<HAsciiString>,
        telephone_number: Handle<HAsciiString>,
        electronic_mail_address: Handle<HAsciiString>,
        telex_number: Handle<HAsciiString>,
        has_internal_location: bool,
        has_street_number: bool,
        has_street: bool,
        has_postal_box: bool,
        has_town: bool,
        has_region: bool,
        has_postal_code: bool,
        has_country: bool,
        has_facsimile_number: bool,
        has_telephone_number: bool,
        has_electronic_mail_address: bool,
        has_telex_number: bool,
    }
}
impl StepBasicAddress {
    step_methods! {
        set set_internal_location = internal_location flag has_internal_location: Handle<HAsciiString>;
        unset un_set_internal_location = internal_location flag has_internal_location;
        get internal_location = internal_location: Handle<HAsciiString>;
        get has_internal_location = has_internal_location: bool;
        set set_street_number = street_number flag has_street_number: Handle<HAsciiString>;
        unset un_set_street_number = street_number flag has_street_number;
        get street_number = street_number: Handle<HAsciiString>;
        get has_street_number = has_street_number: bool;
        set set_street = street flag has_street: Handle<HAsciiString>;
        unset un_set_street = street flag has_street;
        get street = street: Handle<HAsciiString>;
        get has_street = has_street: bool;
        set set_postal_box = postal_box flag has_postal_box: Handle<HAsciiString>;
        unset un_set_postal_box = postal_box flag has_postal_box;
        get postal_box = postal_box: Handle<HAsciiString>;
        get has_postal_box = has_postal_box: bool;
        set set_town = town flag has_town: Handle<HAsciiString>;
        unset un_set_town = town flag has_town;
        get town = town: Handle<HAsciiString>;
        get has_town = has_town: bool;
        set set_region = region flag has_region: Handle<HAsciiString>;
        unset un_set_region = region flag has_region;
        get region = region: Handle<HAsciiString>;
        get has_region = has_region: bool;
        set set_postal_code = postal_code flag has_postal_code: Handle<HAsciiString>;
        unset un_set_postal_code = postal_code flag has_postal_code;
        get postal_code = postal_code: Handle<HAsciiString>;
        get has_postal_code = has_postal_code: bool;
        set set_country = country flag has_country: Handle<HAsciiString>;
        unset un_set_country = country flag has_country;
        get country = country: Handle<HAsciiString>;
        get has_country = has_country: bool;
        set set_facsimile_number = facsimile_number flag has_facsimile_number: Handle<HAsciiString>;
        unset un_set_facsimile_number = facsimile_number flag has_facsimile_number;
        get facsimile_number = facsimile_number: Handle<HAsciiString>;
        get has_facsimile_number = has_facsimile_number: bool;
        set set_telephone_number = telephone_number flag has_telephone_number: Handle<HAsciiString>;
        unset un_set_telephone_number = telephone_number flag has_telephone_number;
        get telephone_number = telephone_number: Handle<HAsciiString>;
        get has_telephone_number = has_telephone_number: bool;
        set set_electronic_mail_address = electronic_mail_address flag has_electronic_mail_address: Handle<HAsciiString>;
        unset un_set_electronic_mail_address = electronic_mail_address flag has_electronic_mail_address;
        get electronic_mail_address = electronic_mail_address: Handle<HAsciiString>;
        get has_electronic_mail_address = has_electronic_mail_address: bool;
        set set_telex_number = telex_number flag has_telex_number: Handle<HAsciiString>;
        unset un_set_telex_number = telex_number flag has_telex_number;
        get telex_number = telex_number: Handle<HAsciiString>;
        get has_telex_number = has_telex_number: bool;
    }
    /// `init`.
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        has_ainternal_location: bool,
        a_internal_location: &Handle<HAsciiString>,
        has_astreet_number: bool,
        a_street_number: &Handle<HAsciiString>,
        has_astreet: bool,
        a_street: &Handle<HAsciiString>,
        has_apostal_box: bool,
        a_postal_box: &Handle<HAsciiString>,
        has_atown: bool,
        a_town: &Handle<HAsciiString>,
        has_aregion: bool,
        a_region: &Handle<HAsciiString>,
        has_apostal_code: bool,
        a_postal_code: &Handle<HAsciiString>,
        has_acountry: bool,
        a_country: &Handle<HAsciiString>,
        has_afacsimile_number: bool,
        a_facsimile_number: &Handle<HAsciiString>,
        has_atelephone_number: bool,
        a_telephone_number: &Handle<HAsciiString>,
        has_aelectronic_mail_address: bool,
        a_electronic_mail_address: &Handle<HAsciiString>,
        has_atelex_number: bool,
        a_telex_number: &Handle<HAsciiString>,
    ) {
        let mut state = self.state.write();
        state.has_internal_location = has_ainternal_location;
        state.internal_location = a_internal_location.clone();
        state.has_street_number = has_astreet_number;
        state.street_number = a_street_number.clone();
        state.has_street = has_astreet;
        state.street = a_street.clone();
        state.has_postal_box = has_apostal_box;
        state.postal_box = a_postal_box.clone();
        state.has_town = has_atown;
        state.town = a_town.clone();
        state.has_region = has_aregion;
        state.region = a_region.clone();
        state.has_postal_code = has_apostal_code;
        state.postal_code = a_postal_code.clone();
        state.has_country = has_acountry;
        state.country = a_country.clone();
        state.has_facsimile_number = has_afacsimile_number;
        state.facsimile_number = a_facsimile_number.clone();
        state.has_telephone_number = has_atelephone_number;
        state.telephone_number = a_telephone_number.clone();
        state.has_electronic_mail_address = has_aelectronic_mail_address;
        state.electronic_mail_address = a_electronic_mail_address.clone();
        state.has_telex_number = has_atelex_number;
        state.telex_number = a_telex_number.clone();
    }
}

step_entity! {
    /// Application context.
    StepBasicApplicationContext, StepBasicApplicationContextState, "StepBasicApplicationContext", StandardTransient, {
        application: Handle<HAsciiString>,
    }
}
impl StepBasicApplicationContext {
    step_methods! {
        set init = application: Handle<HAsciiString>;
        set set_application = application: Handle<HAsciiString>;
        get application = application: Handle<HAsciiString>;
    }
}

step_entity! {
    /// Application context element.
    StepBasicApplicationContextElement, StepBasicApplicationContextElementState, "StepBasicApplicationContextElement", StandardTransient, {
        name: Handle<HAsciiString>,
        frame_of_reference: Handle<StepBasicApplicationContext>,
    }
}
impl StepBasicApplicationContextElement {
    step_methods! {
        set set_name = name: Handle<HAsciiString>;
        get name = name: Handle<HAsciiString>;
        set set_frame_of_reference = frame_of_reference: Handle<StepBasicApplicationContext>;
        get frame_of_reference = frame_of_reference: Handle<StepBasicApplicationContext>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_frame_of_reference: &Handle<StepBasicApplicationContext>,
    ) {
        let mut state = self.state.write();
        state.name = a_name.clone();
        state.frame_of_reference = a_frame_of_reference.clone();
    }
}

step_entity! {
    /// Application protocol definition.
    StepBasicApplicationProtocolDefinition, StepBasicApplicationProtocolDefinitionState, "StepBasicApplicationProtocolDefinition", StandardTransient, {
        status: Handle<HAsciiString>,
        application_interpreted_model_schema_name: Handle<HAsciiString>,
        application_protocol_year: i32,
        application: Handle<StepBasicApplicationContext>,
    }
}
impl StepBasicApplicationProtocolDefinition {
    step_methods! {
        set set_status = status: Handle<HAsciiString>;
        get status = status: Handle<HAsciiString>;
        set set_application_interpreted_model_schema_name = application_interpreted_model_schema_name: Handle<HAsciiString>;
        get application_interpreted_model_schema_name = application_interpreted_model_schema_name: Handle<HAsciiString>;
        set_copy set_application_protocol_year = application_protocol_year: i32;
        get application_protocol_year = application_protocol_year: i32;
        set set_application = application: Handle<StepBasicApplicationContext>;
        get application = application: Handle<StepBasicApplicationContext>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_status: &Handle<HAsciiString>,
        a_application_interpreted_model_schema_name: &Handle<HAsciiString>,
        a_application_protocol_year: i32,
        a_application: &Handle<StepBasicApplicationContext>,
    ) {
        let mut state = self.state.write();
        state.status = a_status.clone();
        state.application_interpreted_model_schema_name =
            a_application_interpreted_model_schema_name.clone();
        state.application_protocol_year = a_application_protocol_year;
        state.application = a_application.clone();
    }
}
