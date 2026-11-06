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
    /// Representation of STEP entity RepresentationReference
    StepReprRepresentationReference, StepReprRepresentationReferenceState, "StepReprRepresentationReference", StandardTransient, {
        my_id: Handle<HAsciiString>,
        my_context_of_items: Handle<StepReprRepresentationContextReference>,
    }
}
impl StepReprRepresentationReference {
    step_methods! {
        get id = my_id: Handle<HAsciiString>;
        set set_id = my_id: Handle<HAsciiString>;
        get context_of_items = my_context_of_items: Handle<StepReprRepresentationContextReference>;
        set set_context_of_items = my_context_of_items: Handle<StepReprRepresentationContextReference>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_id: &Handle<HAsciiString>,
        the_context_of_items: &Handle<StepReprRepresentationContextReference>,
    ) {
        let mut state = self.state.write();
        state.my_id = the_id.clone();
        state.my_context_of_items = the_context_of_items.clone();
    }
}

step_entity! {
    /// Representation relationship.
    StepReprRepresentationRelationship, StepReprRepresentationRelationshipState, "StepReprRepresentationRelationship", StandardTransient, {
        name: Handle<HAsciiString>,
        description: Handle<HAsciiString>,
        rep1: Handle<StepReprRepresentation>,
        rep2: Handle<StepReprRepresentation>,
    }
}
impl StepReprRepresentationRelationship {
    step_methods! {
        set set_name = name: Handle<HAsciiString>;
        get name = name: Handle<HAsciiString>;
        set set_description = description: Handle<HAsciiString>;
        get description = description: Handle<HAsciiString>;
        set set_rep1 = rep1: Handle<StepReprRepresentation>;
        get rep1 = rep1: Handle<StepReprRepresentation>;
        set set_rep2 = rep2: Handle<StepReprRepresentation>;
        get rep2 = rep2: Handle<StepReprRepresentation>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_description: &Handle<HAsciiString>,
        a_rep1: &Handle<StepReprRepresentation>,
        a_rep2: &Handle<StepReprRepresentation>,
    ) {
        let mut state = self.state.write();
        state.name = a_name.clone();
        state.description = a_description.clone();
        state.rep1 = a_rep1.clone();
        state.rep2 = a_rep2.clone();
    }
    /// `has_description`.
    pub fn has_description(&self) -> bool {
        !self.state.read().description.is_null()
    }
}

step_entity! {
    /// Representation relationship with transformation.
    StepReprRepresentationRelationshipWithTransformation, StepReprRepresentationRelationshipWithTransformationState, "StepReprRepresentationRelationshipWithTransformation", StepReprShapeRepresentationRelationship, {
        the_trans: StepReprTransformation,
    }
}
impl StepReprRepresentationRelationshipWithTransformation {
    step_methods! {
        get transformation_operator = the_trans: StepReprTransformation;
        set set_transformation_operator = the_trans: StepReprTransformation;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_description: &Handle<HAsciiString>,
        a_rep1: &Handle<StepReprRepresentation>,
        a_rep2: &Handle<StepReprRepresentation>,
        a_trans: &StepReprTransformation,
    ) {
        self.parent
            .parent
            .init(a_name, a_description, a_rep1, a_rep2);
        self.state.write().the_trans = a_trans.clone();
    }
}

step_entity! {
    /// Shape aspect.
    StepReprShapeAspect, StepReprShapeAspectState, "StepReprShapeAspect", StandardTransient, {
        name: Handle<HAsciiString>,
        description: Handle<HAsciiString>,
        of_shape: Handle<StepReprProductDefinitionShape>,
        product_definitional: StepDataLogical,
    }
}
impl StepReprShapeAspect {
    step_methods! {
        set set_name = name: Handle<HAsciiString>;
        get name = name: Handle<HAsciiString>;
        set set_description = description: Handle<HAsciiString>;
        get description = description: Handle<HAsciiString>;
        set set_of_shape = of_shape: Handle<StepReprProductDefinitionShape>;
        get of_shape = of_shape: Handle<StepReprProductDefinitionShape>;
        set_copy set_product_definitional = product_definitional: StepDataLogical;
        get product_definitional = product_definitional: StepDataLogical;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_description: &Handle<HAsciiString>,
        a_of_shape: &Handle<StepReprProductDefinitionShape>,
        a_product_definitional: StepDataLogical,
    ) {
        let mut state = self.state.write();
        state.name = a_name.clone();
        state.description = a_description.clone();
        state.of_shape = a_of_shape.clone();
        state.product_definitional = a_product_definitional;
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    StepReprShapeAspectDerivingRelationship, StepReprShapeAspectDerivingRelationshipState, "StepReprShapeAspectDerivingRelationship", StepReprShapeAspectRelationship, {
    }
}

step_entity! {
    /// Representation of STEP entity ShapeAspectRelationship
    StepReprShapeAspectRelationship, StepReprShapeAspectRelationshipState, "StepReprShapeAspectRelationship", StandardTransient, {
        the_name: Handle<HAsciiString>,
        the_description: Handle<HAsciiString>,
        the_relating_shape_aspect: Handle<StepReprShapeAspect>,
        the_related_shape_aspect: Handle<StepReprShapeAspect>,
        def_description: bool,
    }
}
impl StepReprShapeAspectRelationship {
    step_methods! {
        get name = the_name: Handle<HAsciiString>;
        set set_name = the_name: Handle<HAsciiString>;
        get description = the_description: Handle<HAsciiString>;
        set set_description = the_description: Handle<HAsciiString>;
        get has_description = def_description: bool;
        get relating_shape_aspect = the_relating_shape_aspect: Handle<StepReprShapeAspect>;
        set set_relating_shape_aspect = the_relating_shape_aspect: Handle<StepReprShapeAspect>;
        get related_shape_aspect = the_related_shape_aspect: Handle<StepReprShapeAspect>;
        set set_related_shape_aspect = the_related_shape_aspect: Handle<StepReprShapeAspect>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        has_description: bool,
        a_description: &Handle<HAsciiString>,
        a_relating_shape_aspect: &Handle<StepReprShapeAspect>,
        a_related_shape_aspect: &Handle<StepReprShapeAspect>,
    ) {
        let mut state = self.state.write();
        state.the_name = a_name.clone();
        state.def_description = has_description;
        if state.def_description {
            state.the_description = a_description.clone();
        } else {
            state.the_description = Default::default();
        }
        state.the_relating_shape_aspect = a_relating_shape_aspect.clone();
        state.the_related_shape_aspect = a_related_shape_aspect.clone();
    }
}

step_entity! {
    /// Representation of STEP entity ShapeAspectTransition
    StepReprShapeAspectTransition, StepReprShapeAspectTransitionState, "StepReprShapeAspectTransition", StepReprShapeAspectRelationship, {
    }
}

step_entity! {
    /// Shape representation relationship.
    StepReprShapeRepresentationRelationship, StepReprShapeRepresentationRelationshipState, "StepReprShapeRepresentationRelationship", StepReprRepresentationRelationship, {
    }
}

step_entity! {
    /// Shape representation relationship with transformation.
    StepReprShapeRepresentationRelationshipWithTransformation, StepReprShapeRepresentationRelationshipWithTransformationState, "StepReprShapeRepresentationRelationshipWithTransformation", StepReprRepresentationRelationshipWithTransformation, {
    }
}

step_entity! {
    /// Representation of STEP entity SpecifiedHigherUsageOccurrence
    StepReprSpecifiedHigherUsageOccurrence, StepReprSpecifiedHigherUsageOccurrenceState, "StepReprSpecifiedHigherUsageOccurrence", StepReprAssemblyComponentUsage, {
        the_upper_usage: Handle<StepReprAssemblyComponentUsage>,
        the_next_usage: Handle<StepReprNextAssemblyUsageOccurrence>,
    }
}
impl StepReprSpecifiedHigherUsageOccurrence {
    step_methods! {
        get upper_usage = the_upper_usage: Handle<StepReprAssemblyComponentUsage>;
        set set_upper_usage = the_upper_usage: Handle<StepReprAssemblyComponentUsage>;
        get next_usage = the_next_usage: Handle<StepReprNextAssemblyUsageOccurrence>;
        set set_next_usage = the_next_usage: Handle<StepReprNextAssemblyUsageOccurrence>;
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
        has_assembly_component_usage_reference_designator: bool,
        a_assembly_component_usage_reference_designator: &Handle<HAsciiString>,
        a_upper_usage: &Handle<StepReprAssemblyComponentUsage>,
        a_next_usage: &Handle<StepReprNextAssemblyUsageOccurrence>,
    ) -> Result<(), Failure> {
        self.parent.init(
            a_product_definition_relationship_id,
            a_product_definition_relationship_name,
            has_product_definition_relationship_description,
            a_product_definition_relationship_description,
            a_product_definition_relationship_relating_product_definition,
            a_product_definition_relationship_related_product_definition,
            has_assembly_component_usage_reference_designator,
            a_assembly_component_usage_reference_designator,
        )?;
        let mut state = self.state.write();
        state.the_upper_usage = a_upper_usage.clone();
        state.the_next_usage = a_next_usage.clone();
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
        has_assembly_component_usage_reference_designator: bool,
        a_assembly_component_usage_reference_designator: &Handle<HAsciiString>,
        a_upper_usage: &Handle<StepReprAssemblyComponentUsage>,
        a_next_usage: &Handle<StepReprNextAssemblyUsageOccurrence>,
    ) {
        self.parent.init_product_definition_or_reference(
            a_product_definition_relationship_id,
            a_product_definition_relationship_name,
            has_product_definition_relationship_description,
            a_product_definition_relationship_description,
            a_product_definition_relationship_relating_product_definition,
            a_product_definition_relationship_related_product_definition,
            has_assembly_component_usage_reference_designator,
            a_assembly_component_usage_reference_designator,
        );
        let mut state = self.state.write();
        state.the_upper_usage = a_upper_usage.clone();
        state.the_next_usage = a_next_usage.clone();
    }
}

step_entity! {
    /// Representation of STEP entity StructuralResponseProperty
    StepReprStructuralResponseProperty, StepReprStructuralResponsePropertyState, "StepReprStructuralResponseProperty", StepReprPropertyDefinition, {
    }
}

step_entity! {
    /// Representation of STEP entity StructuralResponsePropertyDefinitionRepresentation
    StepReprStructuralResponsePropertyDefinitionRepresentation, StepReprStructuralResponsePropertyDefinitionRepresentationState, "StepReprStructuralResponsePropertyDefinitionRepresentation", StepReprPropertyDefinitionRepresentation, {
    }
}

step_entity! {
    /// Supplied part relationship.
    StepReprSuppliedPartRelationship, StepReprSuppliedPartRelationshipState, "StepReprSuppliedPartRelationship", StepBasicProductDefinitionRelationship, {
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    StepReprTangent, StepReprTangentState, "StepReprTangent", StepReprDerivedShapeAspect, {
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    StepReprValueRange, StepReprValueRangeState, "StepReprValueRange", StepReprCompoundRepresentationItem, {
    }
}

step_entity! {
    /// Value representation item.
    StepReprValueRepresentationItem, StepReprValueRepresentationItemState, "StepReprValueRepresentationItem", StepReprRepresentationItem, {
        value_component_member: Handle<StepBasicMeasureValueMember>,
    }
}
impl StepReprValueRepresentationItem {
    step_methods! {
        set set_value_component_member = value_component_member: Handle<StepBasicMeasureValueMember>;
        get value_component_member = value_component_member: Handle<StepBasicMeasureValueMember>;
    }
    /// `init`.
    pub fn init(
        &self,
        the_name: &Handle<HAsciiString>,
        the_value_component_member: &Handle<StepBasicMeasureValueMember>,
    ) {
        self.state.write().value_component_member = the_value_component_member.clone();
        self.parent.init(the_name);
    }
}
