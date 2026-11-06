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

step_entity! {
    /// Representation of STEP entity ConfigurationDesign
    StepReprConfigurationDesign, StepReprConfigurationDesignState, "StepReprConfigurationDesign", StandardTransient, {
        the_configuration: Handle<StepReprConfigurationItem>,
        the_design: StepReprConfigurationDesignItem,
    }
}
impl StepReprConfigurationDesign {
    step_methods! {
        get configuration = the_configuration: Handle<StepReprConfigurationItem>;
        set set_configuration = the_configuration: Handle<StepReprConfigurationItem>;
        get design = the_design: StepReprConfigurationDesignItem;
        set set_design = the_design: StepReprConfigurationDesignItem;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_configuration: &Handle<StepReprConfigurationItem>,
        a_design: &StepReprConfigurationDesignItem,
    ) {
        let mut state = self.state.write();
        state.the_configuration = a_configuration.clone();
        state.the_design = a_design.clone();
    }
}

step_entity! {
    /// Representation of STEP entity ConfigurationEffectivity
    StepReprConfigurationEffectivity, StepReprConfigurationEffectivityState, "StepReprConfigurationEffectivity", StepBasicProductDefinitionEffectivity, {
        the_configuration: Handle<StepReprConfigurationDesign>,
    }
}
impl StepReprConfigurationEffectivity {
    step_methods! {
        get configuration = the_configuration: Handle<StepReprConfigurationDesign>;
        set set_configuration = the_configuration: Handle<StepReprConfigurationDesign>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_effectivity_id: &Handle<HAsciiString>,
        a_product_definition_effectivity_usage: &Handle<StepBasicProductDefinitionRelationship>,
        a_configuration: &Handle<StepReprConfigurationDesign>,
    ) {
        self.parent
            .init(a_effectivity_id, a_product_definition_effectivity_usage);
        self.state.write().the_configuration = a_configuration.clone();
    }
}

step_entity! {
    /// Representation of STEP entity ConfigurationItem
    StepReprConfigurationItem, StepReprConfigurationItemState, "StepReprConfigurationItem", StandardTransient, {
        the_id: Handle<HAsciiString>,
        the_name: Handle<HAsciiString>,
        the_description: Handle<HAsciiString>,
        the_item_concept: Handle<StepReprProductConcept>,
        the_purpose: Handle<HAsciiString>,
        def_description: bool,
        def_purpose: bool,
    }
}
impl StepReprConfigurationItem {
    step_methods! {
        get id = the_id: Handle<HAsciiString>;
        set set_id = the_id: Handle<HAsciiString>;
        get name = the_name: Handle<HAsciiString>;
        set set_name = the_name: Handle<HAsciiString>;
        get description = the_description: Handle<HAsciiString>;
        set set_description = the_description: Handle<HAsciiString>;
        get has_description = def_description: bool;
        get item_concept = the_item_concept: Handle<StepReprProductConcept>;
        set set_item_concept = the_item_concept: Handle<StepReprProductConcept>;
        get purpose = the_purpose: Handle<HAsciiString>;
        set set_purpose = the_purpose: Handle<HAsciiString>;
        get has_purpose = def_purpose: bool;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        a_id: &Handle<HAsciiString>,
        a_name: &Handle<HAsciiString>,
        has_description: bool,
        a_description: &Handle<HAsciiString>,
        a_item_concept: &Handle<StepReprProductConcept>,
        has_purpose: bool,
        a_purpose: &Handle<HAsciiString>,
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
        state.the_item_concept = a_item_concept.clone();
        state.def_purpose = has_purpose;
        if state.def_purpose {
            state.the_purpose = a_purpose.clone();
        } else {
            state.the_purpose = Default::default();
        }
    }
}

step_entity! {
    /// Constructive geometry representation.
    StepReprConstructiveGeometryRepresentation, StepReprConstructiveGeometryRepresentationState, "StepReprConstructiveGeometryRepresentation", StepReprRepresentation, {
    }
}

step_entity! {
    /// Constructive geometry representation relationship.
    StepReprConstructiveGeometryRepresentationRelationship, StepReprConstructiveGeometryRepresentationRelationshipState, "StepReprConstructiveGeometryRepresentationRelationship", StepReprRepresentationRelationship, {
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    StepReprContinuosShapeAspect, StepReprContinuosShapeAspectState, "StepReprContinuosShapeAspect", StepReprCompositeShapeAspect, {
    }
}

step_entity! {
    /// Representation of STEP entity DataEnvironment
    StepReprDataEnvironment, StepReprDataEnvironmentState, "StepReprDataEnvironment", StandardTransient, {
        the_name: Handle<HAsciiString>,
        the_description: Handle<HAsciiString>,
        the_elements: Handle<HArray1<Handle<StepReprPropertyDefinitionRepresentation>>>,
    }
}
impl StepReprDataEnvironment {
    step_methods! {
        get name = the_name: Handle<HAsciiString>;
        set set_name = the_name: Handle<HAsciiString>;
        get description = the_description: Handle<HAsciiString>;
        set set_description = the_description: Handle<HAsciiString>;
        get elements = the_elements: Handle<HArray1<Handle<StepReprPropertyDefinitionRepresentation>>>;
        set set_elements = the_elements: Handle<HArray1<Handle<StepReprPropertyDefinitionRepresentation>>>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_description: &Handle<HAsciiString>,
        a_elements: &Handle<HArray1<Handle<StepReprPropertyDefinitionRepresentation>>>,
    ) {
        let mut state = self.state.write();
        state.the_name = a_name.clone();
        state.the_description = a_description.clone();
        state.the_elements = a_elements.clone();
    }
}

step_entity! {
    /// Definitional representation.
    StepReprDefinitionalRepresentation, StepReprDefinitionalRepresentationState, "StepReprDefinitionalRepresentation", StepReprRepresentation, {
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    StepReprDerivedShapeAspect, StepReprDerivedShapeAspectState, "StepReprDerivedShapeAspect", StepReprShapeAspect, {
    }
}

step_entity! {
    /// Descriptive representation item.
    StepReprDescriptiveRepresentationItem, StepReprDescriptiveRepresentationItemState, "StepReprDescriptiveRepresentationItem", StepReprRepresentationItem, {
        description: Handle<HAsciiString>,
    }
}
impl StepReprDescriptiveRepresentationItem {
    step_methods! {
        set set_description = description: Handle<HAsciiString>;
        get description = description: Handle<HAsciiString>;
    }
    /// `init`.
    pub fn init(&self, a_name: &Handle<HAsciiString>, a_description: &Handle<HAsciiString>) {
        self.state.write().description = a_description.clone();
        self.parent.init(a_name);
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    StepReprExtension, StepReprExtensionState, "StepReprExtension", StepReprDerivedShapeAspect, {
    }
}

step_entity! {
    /// Externally defined representation.
    StepReprExternallyDefinedRepresentation, StepReprExternallyDefinedRepresentationState, "StepReprExternallyDefinedRepresentation", StepReprRepresentation, {
    }
}

step_entity! {
    /// Representation of STEP entity DimensionalLocation
    StepReprFeatureForDatumTargetRelationship, StepReprFeatureForDatumTargetRelationshipState, "StepReprFeatureForDatumTargetRelationship", StepReprShapeAspectRelationship, {
    }
}

step_entity! {
    /// Functionally defined transformation.
    StepReprFunctionallyDefinedTransformation, StepReprFunctionallyDefinedTransformationState, "StepReprFunctionallyDefinedTransformation", StandardTransient, {
        name: Handle<HAsciiString>,
        description: Handle<HAsciiString>,
    }
}
impl StepReprFunctionallyDefinedTransformation {
    step_methods! {
        set set_name = name: Handle<HAsciiString>;
        get name = name: Handle<HAsciiString>;
        set set_description = description: Handle<HAsciiString>;
        get description = description: Handle<HAsciiString>;
    }
    /// `init`.
    pub fn init(&self, a_name: &Handle<HAsciiString>, a_description: &Handle<HAsciiString>) {
        let mut state = self.state.write();
        state.name = a_name.clone();
        state.description = a_description.clone();
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    StepReprGeometricAlignment, StepReprGeometricAlignmentState, "StepReprGeometricAlignment", StepReprDerivedShapeAspect, {
    }
}

step_entity! {
    /// Global uncertainty assigned context.
    StepReprGlobalUncertaintyAssignedContext, StepReprGlobalUncertaintyAssignedContextState, "StepReprGlobalUncertaintyAssignedContext", StepReprRepresentationContext, {
        uncertainty: Handle<HArray1<Handle<StepBasicUncertaintyMeasureWithUnit>>>,
    }
}
impl StepReprGlobalUncertaintyAssignedContext {
    step_methods! {
        set set_uncertainty = uncertainty: Handle<HArray1<Handle<StepBasicUncertaintyMeasureWithUnit>>>;
        get uncertainty = uncertainty: Handle<HArray1<Handle<StepBasicUncertaintyMeasureWithUnit>>>;
        item uncertainty_value = uncertainty: Handle<StepBasicUncertaintyMeasureWithUnit>;
        len nb_uncertainty = uncertainty;
    }
    /// `init`.
    pub fn init(
        &self,
        a_context_identifier: &Handle<HAsciiString>,
        a_context_type: &Handle<HAsciiString>,
        a_uncertainty: &Handle<HArray1<Handle<StepBasicUncertaintyMeasureWithUnit>>>,
    ) {
        self.state.write().uncertainty = a_uncertainty.clone();
        self.parent.init(a_context_identifier, a_context_type);
    }
}

step_entity! {
    /// Global unit assigned context.
    StepReprGlobalUnitAssignedContext, StepReprGlobalUnitAssignedContextState, "StepReprGlobalUnitAssignedContext", StepReprRepresentationContext, {
        units: Handle<HArray1<Handle<StepBasicNamedUnit>>>,
    }
}
impl StepReprGlobalUnitAssignedContext {
    step_methods! {
        set set_units = units: Handle<HArray1<Handle<StepBasicNamedUnit>>>;
        get units = units: Handle<HArray1<Handle<StepBasicNamedUnit>>>;
        item units_value = units: Handle<StepBasicNamedUnit>;
        len nb_units = units;
    }
    /// `init`.
    pub fn init(
        &self,
        a_context_identifier: &Handle<HAsciiString>,
        a_context_type: &Handle<HAsciiString>,
        a_units: &Handle<HArray1<Handle<StepBasicNamedUnit>>>,
    ) {
        self.state.write().units = a_units.clone();
        self.parent.init(a_context_identifier, a_context_type);
    }
}
