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
use crate::step_geom::*;

step_select! {
    /// Representation of STEP SELECT type CharacterizedDefinition
    StepReprCharacterizedDefinition, [StepBasicCharacterizedObject, StepBasicProductDefinition, StepBasicProductDefinitionRelationship, StepReprProductDefinitionShape, StepReprShapeAspect, StepReprShapeAspectRelationship, StepBasicDocumentFile], {
        characterized_object: StepBasicCharacterizedObject,
        product_definition: StepBasicProductDefinition,
        product_definition_relationship: StepBasicProductDefinitionRelationship,
        product_definition_shape: StepReprProductDefinitionShape,
        shape_aspect: StepReprShapeAspect,
        shape_aspect_relationship: StepReprShapeAspectRelationship,
        document_file: StepBasicDocumentFile,
    }
}

step_select! {
    /// Representation of STEP SELECT type ConfigurationDesignItem
    StepReprConfigurationDesignItem, [StepBasicProductDefinition, StepBasicProductDefinitionFormation], {
        product_definition: StepBasicProductDefinition,
        product_definition_formation: StepBasicProductDefinitionFormation,
    }
}

step_select! {
    /// Representation of STEP SELECT type RepresentationOrRepresentationReference
    StepReprRepresentationOrRepresentationReference, [StepReprRepresentation, StepReprRepresentationReference], {
        representation: StepReprRepresentation,
        representation_reference: StepReprRepresentationReference,
    }
}

step_select! {
    /// Representation of STEP SELECT type RepresentedDefinition
    StepReprRepresentedDefinition, [StepBasicGeneralProperty, StepReprPropertyDefinition, StepReprPropertyDefinitionRelationship, StepReprShapeAspect, StepReprShapeAspectRelationship], {
        general_property: StepBasicGeneralProperty,
        property_definition: StepReprPropertyDefinition,
        property_definition_relationship: StepReprPropertyDefinitionRelationship,
        shape_aspect: StepReprShapeAspect,
        shape_aspect_relationship: StepReprShapeAspectRelationship,
    }
}

step_select! {
    /// Transformation.
    StepReprTransformation, numbered [StepReprItemDefinedTransformation = 1, StepReprFunctionallyDefinedTransformation = 2, StepGeomGeometricRepresentationItem = 1], {
        item_defined_transformation: StepReprItemDefinedTransformation,
        functionally_defined_transformation: StepReprFunctionallyDefinedTransformation,
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    StepReprAllAroundShapeAspect, StepReprAllAroundShapeAspectState, "StepReprAllAroundShapeAspect", StepReprContinuosShapeAspect, {
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    StepReprApex, StepReprApexState, "StepReprApex", StepReprDerivedShapeAspect, {
    }
}

step_entity! {
    /// Representation of STEP entity AssemblyComponentUsage
    StepReprAssemblyComponentUsage, StepReprAssemblyComponentUsageState, "StepReprAssemblyComponentUsage", StepReprProductDefinitionUsage, {
        the_reference_designator: Handle<HAsciiString>,
        def_reference_designator: bool,
    }
}
impl StepReprAssemblyComponentUsage {
    step_methods! {
        get reference_designator = the_reference_designator: Handle<HAsciiString>;
        set set_reference_designator = the_reference_designator: Handle<HAsciiString>;
        get has_reference_designator = def_reference_designator: bool;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    ///
    /// # Errors
    /// The null dereference or array index failure.
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        a_product_definition_relationship_id: &Handle<HAsciiString>,
        a_product_definition_relationship_name: &Handle<HAsciiString>,
        has_product_definition_relationship_description: bool,
        a_product_definition_relationship_description: &Handle<HAsciiString>,
        a_product_definition_relationship_relating_product_definition: &Handle<
            StepBasicProductDefinition,
        >,
        a_product_definition_relationship_related_product_definition: &Handle<
            StepBasicProductDefinition,
        >,
        has_reference_designator: bool,
        a_reference_designator: &Handle<HAsciiString>,
    ) -> Result<(), Failure> {
        self.parent.init(
            a_product_definition_relationship_id,
            a_product_definition_relationship_name,
            has_product_definition_relationship_description,
            a_product_definition_relationship_description,
            a_product_definition_relationship_relating_product_definition,
            a_product_definition_relationship_related_product_definition,
        )?;
        let mut state = self.state.write();
        state.def_reference_designator = has_reference_designator;
        if state.def_reference_designator {
            state.the_reference_designator = a_reference_designator.clone();
        } else {
            state.the_reference_designator = Default::default();
        }
        Ok(())
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init_product_definition_or_reference(
        &self,
        a_product_definition_relationship_id: &Handle<HAsciiString>,
        a_product_definition_relationship_name: &Handle<HAsciiString>,
        has_product_definition_relationship_description: bool,
        a_product_definition_relationship_description: &Handle<HAsciiString>,
        a_product_definition_relationship_relating_product_definition: &StepBasicProductDefinitionOrReference,
        a_product_definition_relationship_related_product_definition: &StepBasicProductDefinitionOrReference,
        has_reference_designator: bool,
        a_reference_designator: &Handle<HAsciiString>,
    ) {
        self.parent.init_product_definition_or_reference(
            a_product_definition_relationship_id,
            a_product_definition_relationship_name,
            has_product_definition_relationship_description,
            a_product_definition_relationship_description,
            a_product_definition_relationship_relating_product_definition,
            a_product_definition_relationship_related_product_definition,
        );
        let mut state = self.state.write();
        state.def_reference_designator = has_reference_designator;
        if state.def_reference_designator {
            state.the_reference_designator = a_reference_designator.clone();
        } else {
            state.the_reference_designator = Default::default();
        }
    }
}

step_entity! {
    /// Assembly component usage substitute.
    StepReprAssemblyComponentUsageSubstitute, StepReprAssemblyComponentUsageSubstituteState, "StepReprAssemblyComponentUsageSubstitute", StandardTransient, {
        the_name: Handle<HAsciiString>,
        the_def: Handle<HAsciiString>,
        the_base: Handle<StepReprAssemblyComponentUsage>,
        the_subs: Handle<StepReprAssemblyComponentUsage>,
    }
}
impl StepReprAssemblyComponentUsageSubstitute {
    step_methods! {
        get name = the_name: Handle<HAsciiString>;
        set set_name = the_name: Handle<HAsciiString>;
        get definition = the_def: Handle<HAsciiString>;
        set set_definition = the_def: Handle<HAsciiString>;
        get base = the_base: Handle<StepReprAssemblyComponentUsage>;
        set set_base = the_base: Handle<StepReprAssemblyComponentUsage>;
        get substitute = the_subs: Handle<StepReprAssemblyComponentUsage>;
        set set_substitute = the_subs: Handle<StepReprAssemblyComponentUsage>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_def: &Handle<HAsciiString>,
        a_base: &Handle<StepReprAssemblyComponentUsage>,
        a_subs: &Handle<StepReprAssemblyComponentUsage>,
    ) {
        let mut state = self.state.write();
        state.the_name = a_name.clone();
        state.the_def = a_def.clone();
        state.the_base = a_base.clone();
        state.the_subs = a_subs.clone();
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    StepReprBetweenShapeAspect, StepReprBetweenShapeAspectState, "StepReprBetweenShapeAspect", StepReprContinuosShapeAspect, {
    }
}

step_entity! {
    /// Boolean representation item.
    StepReprBooleanRepresentationItem, StepReprBooleanRepresentationItemState, "StepReprBooleanRepresentationItem", StepReprRepresentationItem, {
        value: bool,
    }
}
impl StepReprBooleanRepresentationItem {
    step_methods! {
        set_copy set_value = value: bool;
        get value = value: bool;
    }
    /// `init`.
    pub fn init(&self, the_name: &Handle<HAsciiString>, the_value: bool) {
        self.state.write().value = the_value;
        self.parent.init(the_name);
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    StepReprCentreOfSymmetry, StepReprCentreOfSymmetryState, "StepReprCentreOfSymmetry", StepReprDerivedShapeAspect, {
    }
}

step_entity! {
    /// Characterized representation.
    StepReprCharacterizedRepresentation, StepReprCharacterizedRepresentationState, "StepReprCharacterizedRepresentation", StepReprRepresentation, {
        my_description: Handle<HAsciiString>,
    }
}
impl StepReprCharacterizedRepresentation {
    step_methods! {
        set set_description = my_description: Handle<HAsciiString>;
        get description = my_description: Handle<HAsciiString>;
    }
    /// `init`.
    /// Returns a CharacterizedRepresentation
    pub fn init(
        &self,
        the_name: &Handle<HAsciiString>,
        the_description: &Handle<HAsciiString>,
        the_items: &Handle<HArray1<Handle<StepReprRepresentationItem>>>,
        the_context_of_items: &Handle<StepReprRepresentationContext>,
    ) {
        self.state.write().my_description = the_description.clone();
        self.parent.init(the_name, the_items, the_context_of_items);
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    StepReprCompGroupShAspAndCompShAspAndDatumFeatAndShAsp, StepReprCompGroupShAspAndCompShAspAndDatumFeatAndShAspState, "StepReprCompGroupShAspAndCompShAspAndDatumFeatAndShAsp", StepReprCompShAspAndDatumFeatAndShAsp, {
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    StepReprCompShAspAndDatumFeatAndShAsp, StepReprCompShAspAndDatumFeatAndShAspState, "StepReprCompShAspAndDatumFeatAndShAsp", StepReprShapeAspect, {
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    StepReprCompositeGroupShapeAspect, StepReprCompositeGroupShapeAspectState, "StepReprCompositeGroupShapeAspect", StepReprCompositeShapeAspect, {
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    StepReprCompositeShapeAspect, StepReprCompositeShapeAspectState, "StepReprCompositeShapeAspect", StepReprShapeAspect, {
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    StepReprCompoundRepresentationItem, StepReprCompoundRepresentationItemState, "StepReprCompoundRepresentationItem", StepReprRepresentationItem, {
        the_item_element: Handle<HArray1<Handle<StepReprRepresentationItem>>>,
    }
}
impl StepReprCompoundRepresentationItem {
    step_methods! {
        get item_element = the_item_element: Handle<HArray1<Handle<StepReprRepresentationItem>>>;
        len nb_item_element = the_item_element;
        set set_item_element = the_item_element: Handle<HArray1<Handle<StepReprRepresentationItem>>>;
        item item_element_value = the_item_element: Handle<StepReprRepresentationItem>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        item_element: &Handle<HArray1<Handle<StepReprRepresentationItem>>>,
    ) {
        self.parent.init(a_name);
        self.state.write().the_item_element = item_element.clone();
    }
    /// `set_item_element_value`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_item_element_value(
        &self,
        num: i32,
        anelement: &Handle<StepReprRepresentationItem>,
    ) -> Result<(), Failure> {
        let field_the_item_element = self.state.read().the_item_element.clone();
        step_array1_set(
            &field_the_item_element,
            num,
            anelement.clone(),
            "StepReprCompoundRepresentationItem::SetItemElementValue",
        )?;
        Ok(())
    }
}
