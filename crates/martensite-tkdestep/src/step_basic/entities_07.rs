#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;
#[allow(unused_imports)]
use crate::step_repr::*;

step_entity! {
    /// Representation of STEP entity ExternalIdentificationAssignment
    StepBasicExternalIdentificationAssignment, StepBasicExternalIdentificationAssignmentState, "StepBasicExternalIdentificationAssignment", StepBasicIdentificationAssignment, {
        the_source: Handle<StepBasicExternalSource>,
    }
}
impl StepBasicExternalIdentificationAssignment {
    step_methods! {
        get source = the_source: Handle<StepBasicExternalSource>;
        set set_source = the_source: Handle<StepBasicExternalSource>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_identification_assignment_assigned_id: &Handle<HAsciiString>,
        a_identification_assignment_role: &Handle<StepBasicIdentificationRole>,
        a_source: &Handle<StepBasicExternalSource>,
    ) {
        self.parent.init(
            a_identification_assignment_assigned_id,
            a_identification_assignment_role,
        );
        self.state.write().the_source = a_source.clone();
    }
}

step_entity! {
    /// Representation of STEP entity ExternalSource
    StepBasicExternalSource, StepBasicExternalSourceState, "StepBasicExternalSource", StandardTransient, {
        the_source_id: StepBasicSourceItem,
    }
}
impl StepBasicExternalSource {
    step_methods! {
        set init = the_source_id: StepBasicSourceItem;
        get source_id = the_source_id: StepBasicSourceItem;
        set set_source_id = the_source_id: StepBasicSourceItem;
    }
}

step_entity! {
    /// Representation of STEP entity ExternallyDefinedItem
    StepBasicExternallyDefinedItem, StepBasicExternallyDefinedItemState, "StepBasicExternallyDefinedItem", StandardTransient, {
        the_item_id: StepBasicSourceItem,
        the_source: Handle<StepBasicExternalSource>,
    }
}
impl StepBasicExternallyDefinedItem {
    step_methods! {
        get item_id = the_item_id: StepBasicSourceItem;
        set set_item_id = the_item_id: StepBasicSourceItem;
        get source = the_source: Handle<StepBasicExternalSource>;
        set set_source = the_source: Handle<StepBasicExternalSource>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_item_id: &StepBasicSourceItem,
        a_source: &Handle<StepBasicExternalSource>,
    ) {
        let mut state = self.state.write();
        state.the_item_id = a_item_id.clone();
        state.the_source = a_source.clone();
    }
}

step_entity! {
    /// Representation of STEP entity GeneralProperty
    StepBasicGeneralProperty, StepBasicGeneralPropertyState, "StepBasicGeneralProperty", StandardTransient, {
        the_id: Handle<HAsciiString>,
        the_name: Handle<HAsciiString>,
        the_description: Handle<HAsciiString>,
        def_description: bool,
    }
}
impl StepBasicGeneralProperty {
    step_methods! {
        get id = the_id: Handle<HAsciiString>;
        set set_id = the_id: Handle<HAsciiString>;
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
        a_id: &Handle<HAsciiString>,
        a_name: &Handle<HAsciiString>,
        has_description: bool,
        a_description: &Handle<HAsciiString>,
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
    }
}

step_entity! {
    /// Representation of STEP entity GeneralPropertyAssociation
    StepBasicGeneralPropertyAssociation, StepBasicGeneralPropertyAssociationState, "StepBasicGeneralPropertyAssociation", StandardTransient, {
        the_name: Handle<HAsciiString>,
        the_description: Handle<HAsciiString>,
        the_general_property: Handle<StepBasicGeneralProperty>,
        the_property_definition: Handle<StepReprPropertyDefinition>,
    }
}
impl StepBasicGeneralPropertyAssociation {
    step_methods! {
        get name = the_name: Handle<HAsciiString>;
        set set_name = the_name: Handle<HAsciiString>;
        get description = the_description: Handle<HAsciiString>;
        set set_description = the_description: Handle<HAsciiString>;
        get general_property = the_general_property: Handle<StepBasicGeneralProperty>;
        set set_general_property = the_general_property: Handle<StepBasicGeneralProperty>;
        get property_definition = the_property_definition: Handle<StepReprPropertyDefinition>;
        set set_property_definition = the_property_definition: Handle<StepReprPropertyDefinition>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_description: &Handle<HAsciiString>,
        a_general_property: &Handle<StepBasicGeneralProperty>,
        a_property_definition: &Handle<StepReprPropertyDefinition>,
    ) {
        let mut state = self.state.write();
        state.the_name = a_name.clone();
        state.the_description = a_description.clone();
        state.the_general_property = a_general_property.clone();
        state.the_property_definition = a_property_definition.clone();
    }
}

step_entity! {
    /// Representation of STEP entity GeneralPropertyRelationship
    StepBasicGeneralPropertyRelationship, StepBasicGeneralPropertyRelationshipState, "StepBasicGeneralPropertyRelationship", StandardTransient, {
        def_description: bool,
        the_name: Handle<HAsciiString>,
        the_description: Handle<HAsciiString>,
        the_relating_general_property: Handle<StepBasicGeneralProperty>,
        the_related_general_property: Handle<StepBasicGeneralProperty>,
    }
}
impl StepBasicGeneralPropertyRelationship {
    step_methods! {
        get name = the_name: Handle<HAsciiString>;
        set set_name = the_name: Handle<HAsciiString>;
        get has_description = def_description: bool;
        get description = the_description: Handle<HAsciiString>;
        get relating_general_property = the_relating_general_property: Handle<StepBasicGeneralProperty>;
        set set_relating_general_property = the_relating_general_property: Handle<StepBasicGeneralProperty>;
        get related_general_property = the_related_general_property: Handle<StepBasicGeneralProperty>;
        set set_related_general_property = the_related_general_property: Handle<StepBasicGeneralProperty>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        has_description: bool,
        a_description: &Handle<HAsciiString>,
        a_relating_general_property: &Handle<StepBasicGeneralProperty>,
        a_related_general_property: &Handle<StepBasicGeneralProperty>,
    ) {
        let mut state = self.state.write();
        state.the_name = a_name.clone();
        state.def_description = has_description;
        if state.def_description {
            state.the_description = a_description.clone();
        } else {
            state.the_description = Default::default();
        }
        state.the_relating_general_property = a_relating_general_property.clone();
        state.the_related_general_property = a_related_general_property.clone();
    }
    /// `set_description`.
    /// Set field Description
    pub fn set_description(&self, a_description: &Handle<HAsciiString>) {
        let mut state = self.state.write();
        state.def_description = true;
        if a_description.is_null() {
            state.def_description = false;
        }
        state.the_description = a_description.clone();
    }
}

step_entity! {
    /// Representation of STEP entity Group
    StepBasicGroup, StepBasicGroupState, "StepBasicGroup", StandardTransient, {
        the_name: Handle<HAsciiString>,
        the_description: Handle<HAsciiString>,
        def_description: bool,
    }
}
impl StepBasicGroup {
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
    /// Representation of STEP entity GroupAssignment
    StepBasicGroupAssignment, StepBasicGroupAssignmentState, "StepBasicGroupAssignment", StandardTransient, {
        the_assigned_group: Handle<StepBasicGroup>,
    }
}
impl StepBasicGroupAssignment {
    step_methods! {
        set init = the_assigned_group: Handle<StepBasicGroup>;
        get assigned_group = the_assigned_group: Handle<StepBasicGroup>;
        set set_assigned_group = the_assigned_group: Handle<StepBasicGroup>;
    }
}

step_entity! {
    /// Representation of STEP entity GroupRelationship
    StepBasicGroupRelationship, StepBasicGroupRelationshipState, "StepBasicGroupRelationship", StandardTransient, {
        the_name: Handle<HAsciiString>,
        the_description: Handle<HAsciiString>,
        the_relating_group: Handle<StepBasicGroup>,
        the_related_group: Handle<StepBasicGroup>,
        def_description: bool,
    }
}
impl StepBasicGroupRelationship {
    step_methods! {
        get name = the_name: Handle<HAsciiString>;
        set set_name = the_name: Handle<HAsciiString>;
        get description = the_description: Handle<HAsciiString>;
        set set_description = the_description: Handle<HAsciiString>;
        get has_description = def_description: bool;
        get relating_group = the_relating_group: Handle<StepBasicGroup>;
        set set_relating_group = the_relating_group: Handle<StepBasicGroup>;
        get related_group = the_related_group: Handle<StepBasicGroup>;
        set set_related_group = the_related_group: Handle<StepBasicGroup>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        has_description: bool,
        a_description: &Handle<HAsciiString>,
        a_relating_group: &Handle<StepBasicGroup>,
        a_related_group: &Handle<StepBasicGroup>,
    ) {
        let mut state = self.state.write();
        state.the_name = a_name.clone();
        state.def_description = has_description;
        if state.def_description {
            state.the_description = a_description.clone();
        } else {
            state.the_description = Default::default();
        }
        state.the_relating_group = a_relating_group.clone();
        state.the_related_group = a_related_group.clone();
    }
}
