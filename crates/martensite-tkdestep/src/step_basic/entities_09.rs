#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

step_entity! {
    /// Organization assignment.
    StepBasicOrganizationAssignment, StepBasicOrganizationAssignmentState, "StepBasicOrganizationAssignment", StandardTransient, {
        assigned_organization: Handle<StepBasicOrganization>,
        role: Handle<StepBasicOrganizationRole>,
    }
}
impl StepBasicOrganizationAssignment {
    step_methods! {
        set set_assigned_organization = assigned_organization: Handle<StepBasicOrganization>;
        get assigned_organization = assigned_organization: Handle<StepBasicOrganization>;
        set set_role = role: Handle<StepBasicOrganizationRole>;
        get role = role: Handle<StepBasicOrganizationRole>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_assigned_organization: &Handle<StepBasicOrganization>,
        a_role: &Handle<StepBasicOrganizationRole>,
    ) {
        let mut state = self.state.write();
        state.assigned_organization = a_assigned_organization.clone();
        state.role = a_role.clone();
    }
}

step_entity! {
    /// Organization role.
    StepBasicOrganizationRole, StepBasicOrganizationRoleState, "StepBasicOrganizationRole", StandardTransient, {
        name: Handle<HAsciiString>,
    }
}
impl StepBasicOrganizationRole {
    step_methods! {
        set init = name: Handle<HAsciiString>;
        set set_name = name: Handle<HAsciiString>;
        get name = name: Handle<HAsciiString>;
    }
}

step_entity! {
    /// Organizational address.
    StepBasicOrganizationalAddress, StepBasicOrganizationalAddressState, "StepBasicOrganizationalAddress", StepBasicAddress, {
        organizations: Handle<HArray1<Handle<StepBasicOrganization>>>,
        description: Handle<HAsciiString>,
    }
}
impl StepBasicOrganizationalAddress {
    step_methods! {
        set set_organizations = organizations: Handle<HArray1<Handle<StepBasicOrganization>>>;
        get organizations = organizations: Handle<HArray1<Handle<StepBasicOrganization>>>;
        item organizations_value = organizations: Handle<StepBasicOrganization>;
        len nb_organizations = organizations;
        set set_description = description: Handle<HAsciiString>;
        get description = description: Handle<HAsciiString>;
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
        a_organizations: &Handle<HArray1<Handle<StepBasicOrganization>>>,
        a_description: &Handle<HAsciiString>,
    ) {
        let mut state = self.state.write();
        state.organizations = a_organizations.clone();
        state.description = a_description.clone();
        drop(state);
        self.parent.init(
            has_ainternal_location,
            a_internal_location,
            has_astreet_number,
            a_street_number,
            has_astreet,
            a_street,
            has_apostal_box,
            a_postal_box,
            has_atown,
            a_town,
            has_aregion,
            a_region,
            has_apostal_code,
            a_postal_code,
            has_acountry,
            a_country,
            has_afacsimile_number,
            a_facsimile_number,
            has_atelephone_number,
            a_telephone_number,
            has_aelectronic_mail_address,
            a_electronic_mail_address,
            has_atelex_number,
            a_telex_number,
        );
    }
}

step_entity! {
    /// Person.
    StepBasicPerson, StepBasicPersonState, "StepBasicPerson", StandardTransient, {
        id: Handle<HAsciiString>,
        last_name: Handle<HAsciiString>,
        first_name: Handle<HAsciiString>,
        middle_names: Handle<HArray1<Handle<HAsciiString>>>,
        prefix_titles: Handle<HArray1<Handle<HAsciiString>>>,
        suffix_titles: Handle<HArray1<Handle<HAsciiString>>>,
        has_last_name: bool,
        has_first_name: bool,
        has_middle_names: bool,
        has_prefix_titles: bool,
        has_suffix_titles: bool,
    }
}
impl StepBasicPerson {
    step_methods! {
        set set_id = id: Handle<HAsciiString>;
        get id = id: Handle<HAsciiString>;
        set set_last_name = last_name flag has_last_name: Handle<HAsciiString>;
        unset un_set_last_name = last_name flag has_last_name;
        get last_name = last_name: Handle<HAsciiString>;
        get has_last_name = has_last_name: bool;
        set set_first_name = first_name flag has_first_name: Handle<HAsciiString>;
        unset un_set_first_name = first_name flag has_first_name;
        get first_name = first_name: Handle<HAsciiString>;
        get has_first_name = has_first_name: bool;
        set set_middle_names = middle_names flag has_middle_names: Handle<HArray1<Handle<HAsciiString>>>;
        unset un_set_middle_names = middle_names flag has_middle_names;
        get middle_names = middle_names: Handle<HArray1<Handle<HAsciiString>>>;
        get has_middle_names = has_middle_names: bool;
        item middle_names_value = middle_names: Handle<HAsciiString>;
        len nb_middle_names = middle_names;
        set set_prefix_titles = prefix_titles flag has_prefix_titles: Handle<HArray1<Handle<HAsciiString>>>;
        unset un_set_prefix_titles = prefix_titles flag has_prefix_titles;
        get prefix_titles = prefix_titles: Handle<HArray1<Handle<HAsciiString>>>;
        get has_prefix_titles = has_prefix_titles: bool;
        item prefix_titles_value = prefix_titles: Handle<HAsciiString>;
        len nb_prefix_titles = prefix_titles;
        set set_suffix_titles = suffix_titles flag has_suffix_titles: Handle<HArray1<Handle<HAsciiString>>>;
        unset un_set_suffix_titles = suffix_titles flag has_suffix_titles;
        get suffix_titles = suffix_titles: Handle<HArray1<Handle<HAsciiString>>>;
        get has_suffix_titles = has_suffix_titles: bool;
        item suffix_titles_value = suffix_titles: Handle<HAsciiString>;
        len nb_suffix_titles = suffix_titles;
    }
    /// `init`.
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        a_id: &Handle<HAsciiString>,
        has_alast_name: bool,
        a_last_name: &Handle<HAsciiString>,
        has_afirst_name: bool,
        a_first_name: &Handle<HAsciiString>,
        has_amiddle_names: bool,
        a_middle_names: &Handle<HArray1<Handle<HAsciiString>>>,
        has_aprefix_titles: bool,
        a_prefix_titles: &Handle<HArray1<Handle<HAsciiString>>>,
        has_asuffix_titles: bool,
        a_suffix_titles: &Handle<HArray1<Handle<HAsciiString>>>,
    ) {
        let mut state = self.state.write();
        state.id = a_id.clone();
        state.has_last_name = has_alast_name;
        state.last_name = a_last_name.clone();
        state.has_first_name = has_afirst_name;
        state.first_name = a_first_name.clone();
        state.has_middle_names = has_amiddle_names;
        state.middle_names = a_middle_names.clone();
        state.has_prefix_titles = has_aprefix_titles;
        state.prefix_titles = a_prefix_titles.clone();
        state.has_suffix_titles = has_asuffix_titles;
        state.suffix_titles = a_suffix_titles.clone();
    }
}

step_entity! {
    /// Person and organization.
    StepBasicPersonAndOrganization, StepBasicPersonAndOrganizationState, "StepBasicPersonAndOrganization", StandardTransient, {
        the_person: Handle<StepBasicPerson>,
        the_organization: Handle<StepBasicOrganization>,
    }
}
impl StepBasicPersonAndOrganization {
    step_methods! {
        set set_the_person = the_person: Handle<StepBasicPerson>;
        get the_person = the_person: Handle<StepBasicPerson>;
        set set_the_organization = the_organization: Handle<StepBasicOrganization>;
        get the_organization = the_organization: Handle<StepBasicOrganization>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_the_person: &Handle<StepBasicPerson>,
        a_the_organization: &Handle<StepBasicOrganization>,
    ) {
        let mut state = self.state.write();
        state.the_person = a_the_person.clone();
        state.the_organization = a_the_organization.clone();
    }
}

step_entity! {
    /// Person and organization assignment.
    StepBasicPersonAndOrganizationAssignment, StepBasicPersonAndOrganizationAssignmentState, "StepBasicPersonAndOrganizationAssignment", StandardTransient, {
        assigned_person_and_organization: Handle<StepBasicPersonAndOrganization>,
        role: Handle<StepBasicPersonAndOrganizationRole>,
    }
}
impl StepBasicPersonAndOrganizationAssignment {
    step_methods! {
        set set_assigned_person_and_organization = assigned_person_and_organization: Handle<StepBasicPersonAndOrganization>;
        get assigned_person_and_organization = assigned_person_and_organization: Handle<StepBasicPersonAndOrganization>;
        set set_role = role: Handle<StepBasicPersonAndOrganizationRole>;
        get role = role: Handle<StepBasicPersonAndOrganizationRole>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_assigned_person_and_organization: &Handle<StepBasicPersonAndOrganization>,
        a_role: &Handle<StepBasicPersonAndOrganizationRole>,
    ) {
        let mut state = self.state.write();
        state.assigned_person_and_organization = a_assigned_person_and_organization.clone();
        state.role = a_role.clone();
    }
}

step_entity! {
    /// Person and organization role.
    StepBasicPersonAndOrganizationRole, StepBasicPersonAndOrganizationRoleState, "StepBasicPersonAndOrganizationRole", StandardTransient, {
        name: Handle<HAsciiString>,
    }
}
impl StepBasicPersonAndOrganizationRole {
    step_methods! {
        set init = name: Handle<HAsciiString>;
        set set_name = name: Handle<HAsciiString>;
        get name = name: Handle<HAsciiString>;
    }
}

step_entity! {
    /// Personal address.
    StepBasicPersonalAddress, StepBasicPersonalAddressState, "StepBasicPersonalAddress", StepBasicAddress, {
        people: Handle<HArray1<Handle<StepBasicPerson>>>,
        description: Handle<HAsciiString>,
    }
}
impl StepBasicPersonalAddress {
    step_methods! {
        set set_people = people: Handle<HArray1<Handle<StepBasicPerson>>>;
        get people = people: Handle<HArray1<Handle<StepBasicPerson>>>;
        item people_value = people: Handle<StepBasicPerson>;
        len nb_people = people;
        set set_description = description: Handle<HAsciiString>;
        get description = description: Handle<HAsciiString>;
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
        a_people: &Handle<HArray1<Handle<StepBasicPerson>>>,
        a_description: &Handle<HAsciiString>,
    ) {
        let mut state = self.state.write();
        state.people = a_people.clone();
        state.description = a_description.clone();
        drop(state);
        self.parent.init(
            has_ainternal_location,
            a_internal_location,
            has_astreet_number,
            a_street_number,
            has_astreet,
            a_street,
            has_apostal_box,
            a_postal_box,
            has_atown,
            a_town,
            has_aregion,
            a_region,
            has_apostal_code,
            a_postal_code,
            has_acountry,
            a_country,
            has_afacsimile_number,
            a_facsimile_number,
            has_atelephone_number,
            a_telephone_number,
            has_aelectronic_mail_address,
            a_electronic_mail_address,
            has_atelex_number,
            a_telex_number,
        );
    }
}

step_entity! {
    /// Physically modeled product definition.
    StepBasicPhysicallyModeledProductDefinition, StepBasicPhysicallyModeledProductDefinitionState, "StepBasicPhysicallyModeledProductDefinition", StepBasicProductDefinition, {
    }
}

step_entity! {
    /// Plane angle measure with unit.
    StepBasicPlaneAngleMeasureWithUnit, StepBasicPlaneAngleMeasureWithUnitState, "StepBasicPlaneAngleMeasureWithUnit", StepBasicMeasureWithUnit, {
    }
}

step_entity! {
    /// Plane angle unit.
    StepBasicPlaneAngleUnit, StepBasicPlaneAngleUnitState, "StepBasicPlaneAngleUnit", StepBasicNamedUnit, {
    }
}

step_entity! {
    /// Product.
    StepBasicProduct, StepBasicProductState, "StepBasicProduct", StandardTransient, {
        id: Handle<HAsciiString>,
        name: Handle<HAsciiString>,
        description: Handle<HAsciiString>,
        frame_of_reference: Handle<HArray1<Handle<StepBasicProductContext>>>,
    }
}
impl StepBasicProduct {
    step_methods! {
        set set_id = id: Handle<HAsciiString>;
        get id = id: Handle<HAsciiString>;
        set set_name = name: Handle<HAsciiString>;
        get name = name: Handle<HAsciiString>;
        set set_description = description: Handle<HAsciiString>;
        get description = description: Handle<HAsciiString>;
        set set_frame_of_reference = frame_of_reference: Handle<HArray1<Handle<StepBasicProductContext>>>;
        get frame_of_reference = frame_of_reference: Handle<HArray1<Handle<StepBasicProductContext>>>;
        item frame_of_reference_value = frame_of_reference: Handle<StepBasicProductContext>;
        len nb_frame_of_reference = frame_of_reference;
    }
    /// `init`.
    pub fn init(
        &self,
        a_id: &Handle<HAsciiString>,
        a_name: &Handle<HAsciiString>,
        a_description: &Handle<HAsciiString>,
        a_frame_of_reference: &Handle<HArray1<Handle<StepBasicProductContext>>>,
    ) {
        let mut state = self.state.write();
        state.id = a_id.clone();
        state.name = a_name.clone();
        state.description = a_description.clone();
        state.frame_of_reference = a_frame_of_reference.clone();
    }
}
