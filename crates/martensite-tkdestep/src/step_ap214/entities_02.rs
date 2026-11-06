#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_basic::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;
#[allow(unused_imports)]
use crate::step_visual::*;

step_entity! {
    /// Applied document reference.
    StepAP214AppliedDocumentReference, StepAP214AppliedDocumentReferenceState, "StepAP214AppliedDocumentReference", StepBasicDocumentReference, {
        items: Handle<HArray1<StepAP214DocumentReferenceItem>>,
    }
}
impl StepAP214AppliedDocumentReference {
    step_methods! {
        get items = items: Handle<HArray1<StepAP214DocumentReferenceItem>>;
        set set_items = items: Handle<HArray1<StepAP214DocumentReferenceItem>>;
        item items_value = items: StepAP214DocumentReferenceItem;
        len nb_items = items;
    }
    /// `init`.
    pub fn init(
        &self,
        a_assigned_document: &Handle<StepBasicDocument>,
        a_source: &Handle<HAsciiString>,
        a_items: &Handle<HArray1<StepAP214DocumentReferenceItem>>,
    ) {
        self.init0(a_assigned_document, a_source);
        self.state.write().items = a_items.clone();
    }
}

step_entity! {
    /// Representation of STEP entity AppliedExternalIdentificationAssignment
    StepAP214AppliedExternalIdentificationAssignment, StepAP214AppliedExternalIdentificationAssignmentState, "StepAP214AppliedExternalIdentificationAssignment", StepBasicExternalIdentificationAssignment, {
        the_items: Handle<HArray1<StepAP214ExternalIdentificationItem>>,
    }
}
impl StepAP214AppliedExternalIdentificationAssignment {
    step_methods! {
        get items = the_items: Handle<HArray1<StepAP214ExternalIdentificationItem>>;
        set set_items = the_items: Handle<HArray1<StepAP214ExternalIdentificationItem>>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_identification_assignment_assigned_id: &Handle<HAsciiString>,
        a_identification_assignment_role: &Handle<StepBasicIdentificationRole>,
        a_external_identification_assignment_source: &Handle<StepBasicExternalSource>,
        a_items: &Handle<HArray1<StepAP214ExternalIdentificationItem>>,
    ) {
        self.parent.init(
            a_identification_assignment_assigned_id,
            a_identification_assignment_role,
            a_external_identification_assignment_source,
        );
        self.state.write().the_items = a_items.clone();
    }
}

step_entity! {
    /// Representation of STEP entity AppliedGroupAssignment
    StepAP214AppliedGroupAssignment, StepAP214AppliedGroupAssignmentState, "StepAP214AppliedGroupAssignment", StepBasicGroupAssignment, {
        the_items: Handle<HArray1<StepAP214GroupItem>>,
    }
}
impl StepAP214AppliedGroupAssignment {
    step_methods! {
        get items = the_items: Handle<HArray1<StepAP214GroupItem>>;
        set set_items = the_items: Handle<HArray1<StepAP214GroupItem>>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_group_assignment_assigned_group: &Handle<StepBasicGroup>,
        a_items: &Handle<HArray1<StepAP214GroupItem>>,
    ) {
        self.parent.init(a_group_assignment_assigned_group);
        self.state.write().the_items = a_items.clone();
    }
}

step_entity! {
    /// Applied organization assignment.
    StepAP214AppliedOrganizationAssignment, StepAP214AppliedOrganizationAssignmentState, "StepAP214AppliedOrganizationAssignment", StepBasicOrganizationAssignment, {
        items: Handle<HArray1<StepAP214OrganizationItem>>,
    }
}
impl StepAP214AppliedOrganizationAssignment {
    step_methods! {
        set set_items = items: Handle<HArray1<StepAP214OrganizationItem>>;
        get items = items: Handle<HArray1<StepAP214OrganizationItem>>;
        item items_value = items: StepAP214OrganizationItem;
        len_checked nb_items = items;
    }
    /// `init`.
    pub fn init(
        &self,
        a_assigned_organization: &Handle<StepBasicOrganization>,
        a_role: &Handle<StepBasicOrganizationRole>,
        a_items: &Handle<HArray1<StepAP214OrganizationItem>>,
    ) {
        self.state.write().items = a_items.clone();
        self.parent.init(a_assigned_organization, a_role);
    }
}

step_entity! {
    /// Applied person and organization assignment.
    StepAP214AppliedPersonAndOrganizationAssignment, StepAP214AppliedPersonAndOrganizationAssignmentState, "StepAP214AppliedPersonAndOrganizationAssignment", StepBasicPersonAndOrganizationAssignment, {
        items: Handle<HArray1<StepAP214PersonAndOrganizationItem>>,
    }
}
impl StepAP214AppliedPersonAndOrganizationAssignment {
    step_methods! {
        set set_items = items: Handle<HArray1<StepAP214PersonAndOrganizationItem>>;
        get items = items: Handle<HArray1<StepAP214PersonAndOrganizationItem>>;
        item items_value = items: StepAP214PersonAndOrganizationItem;
        len_checked nb_items = items;
    }
    /// `init`.
    pub fn init(
        &self,
        a_assigned_person_and_organization: &Handle<StepBasicPersonAndOrganization>,
        a_role: &Handle<StepBasicPersonAndOrganizationRole>,
        a_items: &Handle<HArray1<StepAP214PersonAndOrganizationItem>>,
    ) {
        self.state.write().items = a_items.clone();
        self.parent.init(a_assigned_person_and_organization, a_role);
    }
}

step_entity! {
    /// Applied presented item.
    StepAP214AppliedPresentedItem, StepAP214AppliedPresentedItemState, "StepAP214AppliedPresentedItem", StepVisualPresentedItem, {
        items: Handle<HArray1<StepAP214PresentedItemSelect>>,
    }
}
impl StepAP214AppliedPresentedItem {
    step_methods! {
        set init = items: Handle<HArray1<StepAP214PresentedItemSelect>>;
        set set_items = items: Handle<HArray1<StepAP214PresentedItemSelect>>;
        get items = items: Handle<HArray1<StepAP214PresentedItemSelect>>;
        item items_value = items: StepAP214PresentedItemSelect;
        len nb_items = items;
    }
}

step_entity! {
    /// Applied security classification assignment.
    StepAP214AppliedSecurityClassificationAssignment, StepAP214AppliedSecurityClassificationAssignmentState, "StepAP214AppliedSecurityClassificationAssignment", StepBasicSecurityClassificationAssignment, {
        items: Handle<HArray1<StepAP214SecurityClassificationItem>>,
    }
}
impl StepAP214AppliedSecurityClassificationAssignment {
    step_methods! {
        set set_items = items: Handle<HArray1<StepAP214SecurityClassificationItem>>;
        get items = items: Handle<HArray1<StepAP214SecurityClassificationItem>>;
        item items_value = items: StepAP214SecurityClassificationItem;
        len nb_items = items;
    }
    /// `init`.
    pub fn init(
        &self,
        a_assigned_security_classification: &Handle<StepBasicSecurityClassification>,
        a_items: &Handle<HArray1<StepAP214SecurityClassificationItem>>,
    ) {
        self.state.write().items = a_items.clone();
        self.parent.init(a_assigned_security_classification);
    }
}

step_entity! {
    /// Auto design actual date and time assignment.
    StepAP214AutoDesignActualDateAndTimeAssignment, StepAP214AutoDesignActualDateAndTimeAssignmentState, "StepAP214AutoDesignActualDateAndTimeAssignment", StepBasicDateAndTimeAssignment, {
        items: Handle<HArray1<StepAP214AutoDesignDateAndTimeItem>>,
    }
}
impl StepAP214AutoDesignActualDateAndTimeAssignment {
    step_methods! {
        set set_items = items: Handle<HArray1<StepAP214AutoDesignDateAndTimeItem>>;
        get items = items: Handle<HArray1<StepAP214AutoDesignDateAndTimeItem>>;
        item items_value = items: StepAP214AutoDesignDateAndTimeItem;
        len_checked nb_items = items;
    }
    /// `init`.
    pub fn init(
        &self,
        a_assigned_date_and_time: &Handle<StepBasicDateAndTime>,
        a_role: &Handle<StepBasicDateTimeRole>,
        a_items: &Handle<HArray1<StepAP214AutoDesignDateAndTimeItem>>,
    ) {
        self.state.write().items = a_items.clone();
        self.parent.init(a_assigned_date_and_time, a_role);
    }
}

step_entity! {
    /// Auto design actual date assignment.
    StepAP214AutoDesignActualDateAssignment, StepAP214AutoDesignActualDateAssignmentState, "StepAP214AutoDesignActualDateAssignment", StepBasicDateAssignment, {
        items: Handle<HArray1<StepAP214AutoDesignDatedItem>>,
    }
}
impl StepAP214AutoDesignActualDateAssignment {
    step_methods! {
        set set_items = items: Handle<HArray1<StepAP214AutoDesignDatedItem>>;
        get items = items: Handle<HArray1<StepAP214AutoDesignDatedItem>>;
        item items_value = items: StepAP214AutoDesignDatedItem;
        len_checked nb_items = items;
    }
    /// `init`.
    pub fn init(
        &self,
        a_assigned_date: &Handle<StepBasicDate>,
        a_role: &Handle<StepBasicDateRole>,
        a_items: &Handle<HArray1<StepAP214AutoDesignDatedItem>>,
    ) {
        self.state.write().items = a_items.clone();
        self.parent.init(a_assigned_date, a_role);
    }
}

step_entity! {
    /// Auto design approval assignment.
    StepAP214AutoDesignApprovalAssignment, StepAP214AutoDesignApprovalAssignmentState, "StepAP214AutoDesignApprovalAssignment", StepBasicApprovalAssignment, {
        items: Handle<HArray1<StepAP214AutoDesignGeneralOrgItem>>,
    }
}
impl StepAP214AutoDesignApprovalAssignment {
    step_methods! {
        set set_items = items: Handle<HArray1<StepAP214AutoDesignGeneralOrgItem>>;
        get items = items: Handle<HArray1<StepAP214AutoDesignGeneralOrgItem>>;
        item items_value = items: StepAP214AutoDesignGeneralOrgItem;
        len_checked nb_items = items;
    }
    /// `init`.
    pub fn init(
        &self,
        a_assigned_approval: &Handle<StepBasicApproval>,
        a_items: &Handle<HArray1<StepAP214AutoDesignGeneralOrgItem>>,
    ) {
        self.state.write().items = a_items.clone();
        self.parent.init(a_assigned_approval);
    }
}

step_entity! {
    /// Auto design date and person assignment.
    StepAP214AutoDesignDateAndPersonAssignment, StepAP214AutoDesignDateAndPersonAssignmentState, "StepAP214AutoDesignDateAndPersonAssignment", StepBasicPersonAndOrganizationAssignment, {
        items: Handle<HArray1<StepAP214AutoDesignDateAndPersonItem>>,
    }
}
impl StepAP214AutoDesignDateAndPersonAssignment {
    step_methods! {
        set set_items = items: Handle<HArray1<StepAP214AutoDesignDateAndPersonItem>>;
        get items = items: Handle<HArray1<StepAP214AutoDesignDateAndPersonItem>>;
        item items_value = items: StepAP214AutoDesignDateAndPersonItem;
        len_checked nb_items = items;
    }
    /// `init`.
    pub fn init(
        &self,
        a_assigned_person_and_organization: &Handle<StepBasicPersonAndOrganization>,
        a_role: &Handle<StepBasicPersonAndOrganizationRole>,
        a_items: &Handle<HArray1<StepAP214AutoDesignDateAndPersonItem>>,
    ) {
        self.state.write().items = a_items.clone();
        self.parent.init(a_assigned_person_and_organization, a_role);
    }
}

step_entity! {
    /// Auto design document reference.
    StepAP214AutoDesignDocumentReference, StepAP214AutoDesignDocumentReferenceState, "StepAP214AutoDesignDocumentReference", StepBasicDocumentReference, {
        items: Handle<HArray1<StepAP214AutoDesignReferencingItem>>,
    }
}
impl StepAP214AutoDesignDocumentReference {
    step_methods! {
        get items = items: Handle<HArray1<StepAP214AutoDesignReferencingItem>>;
        set set_items = items: Handle<HArray1<StepAP214AutoDesignReferencingItem>>;
        item items_value = items: StepAP214AutoDesignReferencingItem;
        len nb_items = items;
    }
    /// `init`.
    pub fn init(
        &self,
        a_assigned_document: &Handle<StepBasicDocument>,
        a_source: &Handle<HAsciiString>,
        a_items: &Handle<HArray1<StepAP214AutoDesignReferencingItem>>,
    ) {
        self.init0(a_assigned_document, a_source);
        self.state.write().items = a_items.clone();
    }
}

step_entity! {
    /// Auto design group assignment.
    StepAP214AutoDesignGroupAssignment, StepAP214AutoDesignGroupAssignmentState, "StepAP214AutoDesignGroupAssignment", StepBasicGroupAssignment, {
        items: Handle<HArray1<StepAP214AutoDesignGroupedItem>>,
    }
}
impl StepAP214AutoDesignGroupAssignment {
    step_methods! {
        set set_items = items: Handle<HArray1<StepAP214AutoDesignGroupedItem>>;
        get items = items: Handle<HArray1<StepAP214AutoDesignGroupedItem>>;
        item items_value = items: StepAP214AutoDesignGroupedItem;
        len_checked nb_items = items;
    }
    /// `init`.
    pub fn init(
        &self,
        a_assigned_group: &Handle<StepBasicGroup>,
        a_items: &Handle<HArray1<StepAP214AutoDesignGroupedItem>>,
    ) {
        self.state.write().items = a_items.clone();
        self.parent.init(a_assigned_group);
    }
}
