#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

step_entity! {
    /// Approval.
    StepBasicApproval, StepBasicApprovalState, "StepBasicApproval", StandardTransient, {
        status: Handle<StepBasicApprovalStatus>,
        level: Handle<HAsciiString>,
    }
}
impl StepBasicApproval {
    step_methods! {
        set set_status = status: Handle<StepBasicApprovalStatus>;
        get status = status: Handle<StepBasicApprovalStatus>;
        set set_level = level: Handle<HAsciiString>;
        get level = level: Handle<HAsciiString>;
    }
    /// `init`.
    pub fn init(&self, a_status: &Handle<StepBasicApprovalStatus>, a_level: &Handle<HAsciiString>) {
        let mut state = self.state.write();
        state.status = a_status.clone();
        state.level = a_level.clone();
    }
}

step_entity! {
    /// Approval assignment.
    StepBasicApprovalAssignment, StepBasicApprovalAssignmentState, "StepBasicApprovalAssignment", StandardTransient, {
        assigned_approval: Handle<StepBasicApproval>,
    }
}
impl StepBasicApprovalAssignment {
    step_methods! {
        set init = assigned_approval: Handle<StepBasicApproval>;
        set set_assigned_approval = assigned_approval: Handle<StepBasicApproval>;
        get assigned_approval = assigned_approval: Handle<StepBasicApproval>;
    }
}

step_entity! {
    /// Added from StepBasic Rev2 to Rev4
    StepBasicApprovalDateTime, StepBasicApprovalDateTimeState, "StepBasicApprovalDateTime", StandardTransient, {
        the_date_time: StepBasicDateTimeSelect,
        the_dated_approval: Handle<StepBasicApproval>,
    }
}
impl StepBasicApprovalDateTime {
    step_methods! {
        set set_date_time = the_date_time: StepBasicDateTimeSelect;
        get date_time = the_date_time: StepBasicDateTimeSelect;
        set set_dated_approval = the_dated_approval: Handle<StepBasicApproval>;
        get dated_approval = the_dated_approval: Handle<StepBasicApproval>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_date_time: &StepBasicDateTimeSelect,
        a_dated_approval: &Handle<StepBasicApproval>,
    ) {
        let mut state = self.state.write();
        state.the_date_time = a_date_time.clone();
        state.the_dated_approval = a_dated_approval.clone();
    }
}

step_entity! {
    /// Approval person organization.
    StepBasicApprovalPersonOrganization, StepBasicApprovalPersonOrganizationState, "StepBasicApprovalPersonOrganization", StandardTransient, {
        person_organization: StepBasicPersonOrganizationSelect,
        authorized_approval: Handle<StepBasicApproval>,
        role: Handle<StepBasicApprovalRole>,
    }
}
impl StepBasicApprovalPersonOrganization {
    step_methods! {
        set set_person_organization = person_organization: StepBasicPersonOrganizationSelect;
        get person_organization = person_organization: StepBasicPersonOrganizationSelect;
        set set_authorized_approval = authorized_approval: Handle<StepBasicApproval>;
        get authorized_approval = authorized_approval: Handle<StepBasicApproval>;
        set set_role = role: Handle<StepBasicApprovalRole>;
        get role = role: Handle<StepBasicApprovalRole>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_person_organization: &StepBasicPersonOrganizationSelect,
        a_authorized_approval: &Handle<StepBasicApproval>,
        a_role: &Handle<StepBasicApprovalRole>,
    ) {
        let mut state = self.state.write();
        state.person_organization = a_person_organization.clone();
        state.authorized_approval = a_authorized_approval.clone();
        state.role = a_role.clone();
    }
}

step_entity! {
    /// Approval relationship.
    StepBasicApprovalRelationship, StepBasicApprovalRelationshipState, "StepBasicApprovalRelationship", StandardTransient, {
        name: Handle<HAsciiString>,
        description: Handle<HAsciiString>,
        relating_approval: Handle<StepBasicApproval>,
        related_approval: Handle<StepBasicApproval>,
    }
}
impl StepBasicApprovalRelationship {
    step_methods! {
        set set_name = name: Handle<HAsciiString>;
        get name = name: Handle<HAsciiString>;
        set set_description = description: Handle<HAsciiString>;
        get description = description: Handle<HAsciiString>;
        set set_relating_approval = relating_approval: Handle<StepBasicApproval>;
        get relating_approval = relating_approval: Handle<StepBasicApproval>;
        set set_related_approval = related_approval: Handle<StepBasicApproval>;
        get related_approval = related_approval: Handle<StepBasicApproval>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_description: &Handle<HAsciiString>,
        a_relating_approval: &Handle<StepBasicApproval>,
        a_related_approval: &Handle<StepBasicApproval>,
    ) {
        let mut state = self.state.write();
        state.name = a_name.clone();
        state.description = a_description.clone();
        state.relating_approval = a_relating_approval.clone();
        state.related_approval = a_related_approval.clone();
    }
}

step_entity! {
    /// Approval role.
    StepBasicApprovalRole, StepBasicApprovalRoleState, "StepBasicApprovalRole", StandardTransient, {
        role: Handle<HAsciiString>,
    }
}
impl StepBasicApprovalRole {
    step_methods! {
        set init = role: Handle<HAsciiString>;
        set set_role = role: Handle<HAsciiString>;
        get role = role: Handle<HAsciiString>;
    }
}

step_entity! {
    /// Approval status.
    StepBasicApprovalStatus, StepBasicApprovalStatusState, "StepBasicApprovalStatus", StandardTransient, {
        name: Handle<HAsciiString>,
    }
}
impl StepBasicApprovalStatus {
    step_methods! {
        set init = name: Handle<HAsciiString>;
        set set_name = name: Handle<HAsciiString>;
        get name = name: Handle<HAsciiString>;
    }
}

step_entity! {
    /// Area unit.
    StepBasicAreaUnit, StepBasicAreaUnitState, "StepBasicAreaUnit", StepBasicNamedUnit, {
    }
}

step_entity! {
    /// Calendar date.
    StepBasicCalendarDate, StepBasicCalendarDateState, "StepBasicCalendarDate", StepBasicDate, {
        day_component: i32,
        month_component: i32,
    }
}
impl StepBasicCalendarDate {
    step_methods! {
        set_copy set_day_component = day_component: i32;
        get day_component = day_component: i32;
        set_copy set_month_component = month_component: i32;
        get month_component = month_component: i32;
    }
    /// `init`.
    pub fn init(&self, a_year_component: i32, a_day_component: i32, a_month_component: i32) {
        let mut state = self.state.write();
        state.day_component = a_day_component;
        state.month_component = a_month_component;
        drop(state);
        self.parent.init(a_year_component);
    }
}

step_entity! {
    /// Representation of STEP entity Certification
    StepBasicCertification, StepBasicCertificationState, "StepBasicCertification", StandardTransient, {
        the_name: Handle<HAsciiString>,
        the_purpose: Handle<HAsciiString>,
        the_kind: Handle<StepBasicCertificationType>,
    }
}
impl StepBasicCertification {
    step_methods! {
        get name = the_name: Handle<HAsciiString>;
        set set_name = the_name: Handle<HAsciiString>;
        get purpose = the_purpose: Handle<HAsciiString>;
        set set_purpose = the_purpose: Handle<HAsciiString>;
        get kind = the_kind: Handle<StepBasicCertificationType>;
        set set_kind = the_kind: Handle<StepBasicCertificationType>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_purpose: &Handle<HAsciiString>,
        a_kind: &Handle<StepBasicCertificationType>,
    ) {
        let mut state = self.state.write();
        state.the_name = a_name.clone();
        state.the_purpose = a_purpose.clone();
        state.the_kind = a_kind.clone();
    }
}

step_entity! {
    /// Representation of STEP entity CertificationAssignment
    StepBasicCertificationAssignment, StepBasicCertificationAssignmentState, "StepBasicCertificationAssignment", StandardTransient, {
        the_assigned_certification: Handle<StepBasicCertification>,
    }
}
impl StepBasicCertificationAssignment {
    step_methods! {
        set init = the_assigned_certification: Handle<StepBasicCertification>;
        get assigned_certification = the_assigned_certification: Handle<StepBasicCertification>;
        set set_assigned_certification = the_assigned_certification: Handle<StepBasicCertification>;
    }
}

step_entity! {
    /// Representation of STEP entity CertificationType
    StepBasicCertificationType, StepBasicCertificationTypeState, "StepBasicCertificationType", StandardTransient, {
        the_description: Handle<HAsciiString>,
    }
}
impl StepBasicCertificationType {
    step_methods! {
        set init = the_description: Handle<HAsciiString>;
        get description = the_description: Handle<HAsciiString>;
        set set_description = the_description: Handle<HAsciiString>;
    }
}

step_entity! {
    /// Representation of STEP entity CharacterizedObject
    StepBasicCharacterizedObject, StepBasicCharacterizedObjectState, "StepBasicCharacterizedObject", StandardTransient, {
        the_name: Handle<HAsciiString>,
        the_description: Handle<HAsciiString>,
        def_description: bool,
    }
}
impl StepBasicCharacterizedObject {
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
