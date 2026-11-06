#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

step_entity! {
    /// Representation of STEP entity DocumentFile
    custom_default StepBasicDocumentFile, StepBasicDocumentFileState, "StepBasicDocumentFile", StepBasicDocument, {
        the_characterized_object: Handle<StepBasicCharacterizedObject>,
    }
}
impl Default for StepBasicDocumentFile {
    fn default() -> Self {
        let object = Self {
            parent: Default::default(),
            state: Default::default(),
        };
        {
            object.state.write().the_characterized_object =
                Handle::new(StepBasicCharacterizedObject::new());
        }
        object
    }
}
impl StepBasicDocumentFile {
    /// `StepBasicDocumentFile()`, which creates the sub-entities of the class.
    pub fn new() -> Self {
        Self::default()
    }
}
impl StepBasicDocumentFile {
    step_methods! {
        get characterized_object = the_characterized_object: Handle<StepBasicCharacterizedObject>;
        set set_characterized_object = the_characterized_object: Handle<StepBasicCharacterizedObject>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    ///
    /// # Errors
    /// The null dereference or array index failure.
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        a_document_id: &Handle<HAsciiString>,
        a_document_name: &Handle<HAsciiString>,
        has_document_description: bool,
        a_document_description: &Handle<HAsciiString>,
        a_document_kind: &Handle<StepBasicDocumentType>,
        a_characterized_object_name: &Handle<HAsciiString>,
        has_characterized_object_description: bool,
        a_characterized_object_description: &Handle<HAsciiString>,
    ) -> Result<(), Failure> {
        self.parent.init(
            a_document_id,
            a_document_name,
            has_document_description,
            a_document_description,
            a_document_kind,
        );
        let field_the_characterized_object = self.state.read().the_characterized_object.clone();
        step_live(
            &field_the_characterized_object,
            "StepBasicDocumentFile::Init",
        )?
        .init(
            a_characterized_object_name,
            has_characterized_object_description,
            a_characterized_object_description,
        );
        Ok(())
    }
}

step_entity! {
    /// Representation of STEP entity DocumentProductAssociation
    StepBasicDocumentProductAssociation, StepBasicDocumentProductAssociationState, "StepBasicDocumentProductAssociation", StandardTransient, {
        the_name: Handle<HAsciiString>,
        the_description: Handle<HAsciiString>,
        the_relating_document: Handle<StepBasicDocument>,
        the_related_product: StepBasicProductOrFormationOrDefinition,
        def_description: bool,
    }
}
impl StepBasicDocumentProductAssociation {
    step_methods! {
        get name = the_name: Handle<HAsciiString>;
        set set_name = the_name: Handle<HAsciiString>;
        get description = the_description: Handle<HAsciiString>;
        set set_description = the_description: Handle<HAsciiString>;
        get has_description = def_description: bool;
        get relating_document = the_relating_document: Handle<StepBasicDocument>;
        set set_relating_document = the_relating_document: Handle<StepBasicDocument>;
        get related_product = the_related_product: StepBasicProductOrFormationOrDefinition;
        set set_related_product = the_related_product: StepBasicProductOrFormationOrDefinition;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        has_description: bool,
        a_description: &Handle<HAsciiString>,
        a_relating_document: &Handle<StepBasicDocument>,
        a_related_product: &StepBasicProductOrFormationOrDefinition,
    ) {
        let mut state = self.state.write();
        state.the_name = a_name.clone();
        state.def_description = has_description;
        if state.def_description {
            state.the_description = a_description.clone();
        } else {
            state.the_description = Default::default();
        }
        state.the_relating_document = a_relating_document.clone();
        state.the_related_product = a_related_product.clone();
    }
}

step_entity! {
    /// Representation of STEP entity DocumentProductEquivalence
    StepBasicDocumentProductEquivalence, StepBasicDocumentProductEquivalenceState, "StepBasicDocumentProductEquivalence", StepBasicDocumentProductAssociation, {
    }
}

step_entity! {
    /// Document reference.
    StepBasicDocumentReference, StepBasicDocumentReferenceState, "StepBasicDocumentReference", StandardTransient, {
        the_assigned_document: Handle<StepBasicDocument>,
        the_source: Handle<HAsciiString>,
    }
}
impl StepBasicDocumentReference {
    step_methods! {
        get assigned_document = the_assigned_document: Handle<StepBasicDocument>;
        set set_assigned_document = the_assigned_document: Handle<StepBasicDocument>;
        get source = the_source: Handle<HAsciiString>;
        set set_source = the_source: Handle<HAsciiString>;
    }
    /// `init0`.
    pub fn init0(
        &self,
        a_assigned_document: &Handle<StepBasicDocument>,
        a_source: &Handle<HAsciiString>,
    ) {
        let mut state = self.state.write();
        state.the_assigned_document = a_assigned_document.clone();
        state.the_source = a_source.clone();
    }
}

step_entity! {
    /// Document relationship.
    StepBasicDocumentRelationship, StepBasicDocumentRelationshipState, "StepBasicDocumentRelationship", StandardTransient, {
        the_name: Handle<HAsciiString>,
        the_description: Handle<HAsciiString>,
        the_relating: Handle<StepBasicDocument>,
        the_related: Handle<StepBasicDocument>,
    }
}
impl StepBasicDocumentRelationship {
    step_methods! {
        get name = the_name: Handle<HAsciiString>;
        set set_name = the_name: Handle<HAsciiString>;
        get description = the_description: Handle<HAsciiString>;
        set set_description = the_description: Handle<HAsciiString>;
        get relating_document = the_relating: Handle<StepBasicDocument>;
        set set_relating_document = the_relating: Handle<StepBasicDocument>;
        get related_document = the_related: Handle<StepBasicDocument>;
        set set_related_document = the_related: Handle<StepBasicDocument>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_description: &Handle<HAsciiString>,
        a_relating: &Handle<StepBasicDocument>,
        a_related: &Handle<StepBasicDocument>,
    ) {
        let mut state = self.state.write();
        state.the_name = a_name.clone();
        state.the_description = a_description.clone();
        state.the_relating = a_relating.clone();
        state.the_related = a_related.clone();
    }
}

step_entity! {
    /// Representation of STEP entity DocumentRepresentationType
    StepBasicDocumentRepresentationType, StepBasicDocumentRepresentationTypeState, "StepBasicDocumentRepresentationType", StandardTransient, {
        the_name: Handle<HAsciiString>,
        the_represented_document: Handle<StepBasicDocument>,
    }
}
impl StepBasicDocumentRepresentationType {
    step_methods! {
        get name = the_name: Handle<HAsciiString>;
        set set_name = the_name: Handle<HAsciiString>;
        get represented_document = the_represented_document: Handle<StepBasicDocument>;
        set set_represented_document = the_represented_document: Handle<StepBasicDocument>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_represented_document: &Handle<StepBasicDocument>,
    ) {
        let mut state = self.state.write();
        state.the_name = a_name.clone();
        state.the_represented_document = a_represented_document.clone();
    }
}

step_entity! {
    /// Document type.
    StepBasicDocumentType, StepBasicDocumentTypeState, "StepBasicDocumentType", StandardTransient, {
        thepdt: Handle<HAsciiString>,
    }
}
impl StepBasicDocumentType {
    step_methods! {
        set init = thepdt: Handle<HAsciiString>;
        get product_data_type = thepdt: Handle<HAsciiString>;
        set set_product_data_type = thepdt: Handle<HAsciiString>;
    }
}

step_entity! {
    /// Document usage constraint.
    StepBasicDocumentUsageConstraint, StepBasicDocumentUsageConstraintState, "StepBasicDocumentUsageConstraint", StandardTransient, {
        the_source: Handle<StepBasicDocument>,
        the_se: Handle<HAsciiString>,
        the_sev: Handle<HAsciiString>,
    }
}
impl StepBasicDocumentUsageConstraint {
    step_methods! {
        get source = the_source: Handle<StepBasicDocument>;
        set set_source = the_source: Handle<StepBasicDocument>;
        get subject_element = the_se: Handle<HAsciiString>;
        set set_subject_element = the_se: Handle<HAsciiString>;
        get subject_element_value = the_sev: Handle<HAsciiString>;
        set set_subject_element_value = the_sev: Handle<HAsciiString>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_source: &Handle<StepBasicDocument>,
        ase: &Handle<HAsciiString>,
        asev: &Handle<HAsciiString>,
    ) {
        let mut state = self.state.write();
        state.the_source = a_source.clone();
        state.the_se = ase.clone();
        state.the_sev = asev.clone();
    }
}

step_entity! {
    /// Effectivity.
    StepBasicEffectivity, StepBasicEffectivityState, "StepBasicEffectivity", StandardTransient, {
        theid: Handle<HAsciiString>,
    }
}
impl StepBasicEffectivity {
    step_methods! {
        set init = theid: Handle<HAsciiString>;
        get id = theid: Handle<HAsciiString>;
        set set_id = theid: Handle<HAsciiString>;
    }
}

step_entity! {
    /// Representation of STEP entity EffectivityAssignment
    StepBasicEffectivityAssignment, StepBasicEffectivityAssignmentState, "StepBasicEffectivityAssignment", StandardTransient, {
        the_assigned_effectivity: Handle<StepBasicEffectivity>,
    }
}
impl StepBasicEffectivityAssignment {
    step_methods! {
        set init = the_assigned_effectivity: Handle<StepBasicEffectivity>;
        get assigned_effectivity = the_assigned_effectivity: Handle<StepBasicEffectivity>;
        set set_assigned_effectivity = the_assigned_effectivity: Handle<StepBasicEffectivity>;
    }
}

step_entity! {
    /// Representation of STEP entity EulerAngles
    StepBasicEulerAngles, StepBasicEulerAnglesState, "StepBasicEulerAngles", StandardTransient, {
        the_angles: Handle<HArray1<f64>>,
    }
}
impl StepBasicEulerAngles {
    step_methods! {
        set init = the_angles: Handle<HArray1<f64>>;
        get angles = the_angles: Handle<HArray1<f64>>;
        set set_angles = the_angles: Handle<HArray1<f64>>;
    }
}
