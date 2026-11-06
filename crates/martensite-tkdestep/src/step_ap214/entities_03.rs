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
use crate::step_repr::*;
#[allow(unused_imports)]
use crate::step_visual::*;

step_entity! {
    /// Auto design nominal date and time assignment.
    StepAP214AutoDesignNominalDateAndTimeAssignment, StepAP214AutoDesignNominalDateAndTimeAssignmentState, "StepAP214AutoDesignNominalDateAndTimeAssignment", StepBasicDateAndTimeAssignment, {
        items: Handle<HArray1<StepAP214AutoDesignDateAndTimeItem>>,
    }
}
impl StepAP214AutoDesignNominalDateAndTimeAssignment {
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
    /// Auto design nominal date assignment.
    StepAP214AutoDesignNominalDateAssignment, StepAP214AutoDesignNominalDateAssignmentState, "StepAP214AutoDesignNominalDateAssignment", StepBasicDateAssignment, {
        items: Handle<HArray1<StepAP214AutoDesignDatedItem>>,
    }
}
impl StepAP214AutoDesignNominalDateAssignment {
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
    /// Auto design organization assignment.
    StepAP214AutoDesignOrganizationAssignment, StepAP214AutoDesignOrganizationAssignmentState, "StepAP214AutoDesignOrganizationAssignment", StepBasicOrganizationAssignment, {
        items: Handle<HArray1<StepAP214AutoDesignGeneralOrgItem>>,
    }
}
impl StepAP214AutoDesignOrganizationAssignment {
    step_methods! {
        set set_items = items: Handle<HArray1<StepAP214AutoDesignGeneralOrgItem>>;
        get items = items: Handle<HArray1<StepAP214AutoDesignGeneralOrgItem>>;
        item items_value = items: StepAP214AutoDesignGeneralOrgItem;
        len_checked nb_items = items;
    }
    /// `init`.
    pub fn init(
        &self,
        a_assigned_organization: &Handle<StepBasicOrganization>,
        a_role: &Handle<StepBasicOrganizationRole>,
        a_items: &Handle<HArray1<StepAP214AutoDesignGeneralOrgItem>>,
    ) {
        self.state.write().items = a_items.clone();
        self.parent.init(a_assigned_organization, a_role);
    }
}

step_entity! {
    /// Auto design person and organization assignment.
    StepAP214AutoDesignPersonAndOrganizationAssignment, StepAP214AutoDesignPersonAndOrganizationAssignmentState, "StepAP214AutoDesignPersonAndOrganizationAssignment", StepBasicPersonAndOrganizationAssignment, {
        items: Handle<HArray1<StepAP214AutoDesignGeneralOrgItem>>,
    }
}
impl StepAP214AutoDesignPersonAndOrganizationAssignment {
    step_methods! {
        set set_items = items: Handle<HArray1<StepAP214AutoDesignGeneralOrgItem>>;
        get items = items: Handle<HArray1<StepAP214AutoDesignGeneralOrgItem>>;
        item items_value = items: StepAP214AutoDesignGeneralOrgItem;
        len_checked nb_items = items;
    }
    /// `init`.
    pub fn init(
        &self,
        a_assigned_person_and_organization: &Handle<StepBasicPersonAndOrganization>,
        a_role: &Handle<StepBasicPersonAndOrganizationRole>,
        a_items: &Handle<HArray1<StepAP214AutoDesignGeneralOrgItem>>,
    ) {
        self.state.write().items = a_items.clone();
        self.parent.init(a_assigned_person_and_organization, a_role);
    }
}

step_entity! {
    /// Auto design presented item.
    StepAP214AutoDesignPresentedItem, StepAP214AutoDesignPresentedItemState, "StepAP214AutoDesignPresentedItem", StepVisualPresentedItem, {
        items: Handle<HArray1<StepAP214AutoDesignPresentedItemSelect>>,
    }
}
impl StepAP214AutoDesignPresentedItem {
    step_methods! {
        set init = items: Handle<HArray1<StepAP214AutoDesignPresentedItemSelect>>;
        set set_items = items: Handle<HArray1<StepAP214AutoDesignPresentedItemSelect>>;
        get items = items: Handle<HArray1<StepAP214AutoDesignPresentedItemSelect>>;
        item items_value = items: StepAP214AutoDesignPresentedItemSelect;
        len nb_items = items;
    }
}

step_entity! {
    /// Auto design security classification assignment.
    StepAP214AutoDesignSecurityClassificationAssignment, StepAP214AutoDesignSecurityClassificationAssignmentState, "StepAP214AutoDesignSecurityClassificationAssignment", StepBasicSecurityClassificationAssignment, {
        items: Handle<HArray1<Handle<StepBasicApproval>>>,
    }
}
impl StepAP214AutoDesignSecurityClassificationAssignment {
    step_methods! {
        set set_items = items: Handle<HArray1<Handle<StepBasicApproval>>>;
        get items = items: Handle<HArray1<Handle<StepBasicApproval>>>;
        item items_value = items: Handle<StepBasicApproval>;
        len nb_items = items;
    }
    /// `init`.
    pub fn init(
        &self,
        a_assigned_security_classification: &Handle<StepBasicSecurityClassification>,
        a_items: &Handle<HArray1<Handle<StepBasicApproval>>>,
    ) {
        self.state.write().items = a_items.clone();
        self.parent.init(a_assigned_security_classification);
    }
}

step_entity! {
    /// Representation of STEP entity Class
    StepAP214Class, StepAP214ClassState, "StepAP214Class", StepBasicGroup, {
    }
}

step_entity! {
    /// Representation of STEP entity ExternallyDefinedClass
    StepAP214ExternallyDefinedClass, StepAP214ExternallyDefinedClassState, "StepAP214ExternallyDefinedClass", StepAP214Class, {
        the_externally_defined_item: Handle<StepBasicExternallyDefinedItem>,
    }
}
impl StepAP214ExternallyDefinedClass {
    step_methods! {
        get externally_defined_item = the_externally_defined_item: Handle<StepBasicExternallyDefinedItem>;
        set set_externally_defined_item = the_externally_defined_item: Handle<StepBasicExternallyDefinedItem>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_group_name: &Handle<HAsciiString>,
        has_group_description: bool,
        a_group_description: &Handle<HAsciiString>,
        a_externally_defined_item_item_id: &StepBasicSourceItem,
        a_externally_defined_item_source: &Handle<StepBasicExternalSource>,
    ) {
        self.parent
            .init(a_group_name, has_group_description, a_group_description);
        let mut state = self.state.write();
        if state.the_externally_defined_item.is_null() {
            state.the_externally_defined_item = Handle::new(StepBasicExternallyDefinedItem::new());
        }
        drop(state);
        let field_the_externally_defined_item =
            self.state.read().the_externally_defined_item.clone();
        field_the_externally_defined_item.init(
            a_externally_defined_item_item_id,
            a_externally_defined_item_source,
        );
    }
}

step_entity! {
    /// Representation of STEP entity ExternallyDefinedGeneralProperty
    StepAP214ExternallyDefinedGeneralProperty, StepAP214ExternallyDefinedGeneralPropertyState, "StepAP214ExternallyDefinedGeneralProperty", StepBasicGeneralProperty, {
        the_externally_defined_item: Handle<StepBasicExternallyDefinedItem>,
    }
}
impl StepAP214ExternallyDefinedGeneralProperty {
    step_methods! {
        get externally_defined_item = the_externally_defined_item: Handle<StepBasicExternallyDefinedItem>;
        set set_externally_defined_item = the_externally_defined_item: Handle<StepBasicExternallyDefinedItem>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_general_property_id: &Handle<HAsciiString>,
        a_general_property_name: &Handle<HAsciiString>,
        has_general_property_description: bool,
        a_general_property_description: &Handle<HAsciiString>,
        a_externally_defined_item_item_id: &StepBasicSourceItem,
        a_externally_defined_item_source: &Handle<StepBasicExternalSource>,
    ) {
        self.parent.init(
            a_general_property_id,
            a_general_property_name,
            has_general_property_description,
            a_general_property_description,
        );
        let mut state = self.state.write();
        if state.the_externally_defined_item.is_null() {
            state.the_externally_defined_item = Handle::new(StepBasicExternallyDefinedItem::new());
        }
        drop(state);
        let field_the_externally_defined_item =
            self.state.read().the_externally_defined_item.clone();
        field_the_externally_defined_item.init(
            a_externally_defined_item_item_id,
            a_externally_defined_item_source,
        );
    }
}

step_entity! {
    /// Representation of STEP entity RepItemGroup
    StepAP214RepItemGroup, StepAP214RepItemGroupState, "StepAP214RepItemGroup", StepBasicGroup, {
        the_representation_item: Handle<StepReprRepresentationItem>,
    }
}
impl StepAP214RepItemGroup {
    step_methods! {
        get representation_item = the_representation_item: Handle<StepReprRepresentationItem>;
        set set_representation_item = the_representation_item: Handle<StepReprRepresentationItem>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_group_name: &Handle<HAsciiString>,
        has_group_description: bool,
        a_group_description: &Handle<HAsciiString>,
        a_representation_item_name: &Handle<HAsciiString>,
    ) {
        self.parent
            .init(a_group_name, has_group_description, a_group_description);
        let mut state = self.state.write();
        if state.the_representation_item.is_null() {
            state.the_representation_item = Handle::new(StepReprRepresentationItem::new());
        }
        drop(state);
        let field_the_representation_item = self.state.read().the_representation_item.clone();
        field_the_representation_item.init(a_representation_item_name);
    }
}
