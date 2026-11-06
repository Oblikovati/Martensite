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
    /// Integer representation item.
    StepReprIntegerRepresentationItem, StepReprIntegerRepresentationItemState, "StepReprIntegerRepresentationItem", StepReprRepresentationItem, {
        value: i32,
    }
}
impl StepReprIntegerRepresentationItem {
    step_methods! {
        set_copy set_value = value: i32;
        get value = value: i32;
    }
    /// `init`.
    pub fn init(&self, the_name: &Handle<HAsciiString>, the_value: i32) {
        self.state.write().value = the_value;
        self.parent.init(the_name);
    }
}

step_entity! {
    /// Added from StepRepr Rev2 to Rev4
    StepReprItemDefinedTransformation, StepReprItemDefinedTransformationState, "StepReprItemDefinedTransformation", StandardTransient, {
        the_name: Handle<HAsciiString>,
        the_description: Handle<HAsciiString>,
        the_transform_item1: Handle<StepReprRepresentationItem>,
        the_transform_item2: Handle<StepReprRepresentationItem>,
    }
}
impl StepReprItemDefinedTransformation {
    step_methods! {
        set set_name = the_name: Handle<HAsciiString>;
        get name = the_name: Handle<HAsciiString>;
        set set_description = the_description: Handle<HAsciiString>;
        get description = the_description: Handle<HAsciiString>;
        set set_transform_item1 = the_transform_item1: Handle<StepReprRepresentationItem>;
        get transform_item1 = the_transform_item1: Handle<StepReprRepresentationItem>;
        set set_transform_item2 = the_transform_item2: Handle<StepReprRepresentationItem>;
        get transform_item2 = the_transform_item2: Handle<StepReprRepresentationItem>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_description: &Handle<HAsciiString>,
        a_transform_item1: &Handle<StepReprRepresentationItem>,
        a_transform_item2: &Handle<StepReprRepresentationItem>,
    ) {
        let mut state = self.state.write();
        state.the_name = a_name.clone();
        state.the_description = a_description.clone();
        state.the_transform_item1 = a_transform_item1.clone();
        state.the_transform_item2 = a_transform_item2.clone();
    }
    /// `has_description`.
    pub fn has_description(&self) -> bool {
        !self.state.read().the_description.is_null()
    }
}

step_entity! {
    /// Representation of STEP entity MakeFromUsageOption
    StepReprMakeFromUsageOption, StepReprMakeFromUsageOptionState, "StepReprMakeFromUsageOption", StepReprProductDefinitionUsage, {
        the_ranking: i32,
        the_ranking_rationale: Handle<HAsciiString>,
        the_quantity: Handle<StandardTransient>,
    }
}
impl StepReprMakeFromUsageOption {
    step_methods! {
        get ranking = the_ranking: i32;
        set_copy set_ranking = the_ranking: i32;
        get ranking_rationale = the_ranking_rationale: Handle<HAsciiString>;
        set set_ranking_rationale = the_ranking_rationale: Handle<HAsciiString>;
        get quantity = the_quantity: Handle<StandardTransient>;
        set set_quantity = the_quantity: Handle<StandardTransient>;
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
        a_ranking: i32,
        a_ranking_rationale: &Handle<HAsciiString>,
        a_quantity: &Handle<StandardTransient>,
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
        state.the_ranking = a_ranking;
        state.the_ranking_rationale = a_ranking_rationale.clone();
        state.the_quantity = a_quantity.clone();
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
        a_ranking: i32,
        a_ranking_rationale: &Handle<HAsciiString>,
        a_quantity: &Handle<StandardTransient>,
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
        state.the_ranking = a_ranking;
        state.the_ranking_rationale = a_ranking_rationale.clone();
        state.the_quantity = a_quantity.clone();
    }
}

step_entity! {
    /// Mapped item.
    StepReprMappedItem, StepReprMappedItemState, "StepReprMappedItem", StepReprRepresentationItem, {
        mapping_source: Handle<StepReprRepresentationMap>,
        mapping_target: Handle<StepReprRepresentationItem>,
    }
}
impl StepReprMappedItem {
    step_methods! {
        set set_mapping_source = mapping_source: Handle<StepReprRepresentationMap>;
        get mapping_source = mapping_source: Handle<StepReprRepresentationMap>;
        set set_mapping_target = mapping_target: Handle<StepReprRepresentationItem>;
        get mapping_target = mapping_target: Handle<StepReprRepresentationItem>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_mapping_source: &Handle<StepReprRepresentationMap>,
        a_mapping_target: &Handle<StepReprRepresentationItem>,
    ) {
        let mut state = self.state.write();
        state.mapping_source = a_mapping_source.clone();
        state.mapping_target = a_mapping_target.clone();
        drop(state);
        self.parent.init(a_name);
    }
}

step_entity! {
    /// Material designation.
    StepReprMaterialDesignation, StepReprMaterialDesignationState, "StepReprMaterialDesignation", StandardTransient, {
        name: Handle<HAsciiString>,
        of_definition: StepReprCharacterizedDefinition,
    }
}
impl StepReprMaterialDesignation {
    step_methods! {
        set set_name = name: Handle<HAsciiString>;
        get name = name: Handle<HAsciiString>;
        set set_of_definition = of_definition: StepReprCharacterizedDefinition;
        get of_definition = of_definition: StepReprCharacterizedDefinition;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_of_definition: &StepReprCharacterizedDefinition,
    ) {
        let mut state = self.state.write();
        state.name = a_name.clone();
        state.of_definition = a_of_definition.clone();
    }
}

step_entity! {
    /// Representation of STEP entity MaterialProperty
    StepReprMaterialProperty, StepReprMaterialPropertyState, "StepReprMaterialProperty", StepReprPropertyDefinition, {
    }
}

step_entity! {
    /// Representation of STEP entity MaterialPropertyRepresentation
    StepReprMaterialPropertyRepresentation, StepReprMaterialPropertyRepresentationState, "StepReprMaterialPropertyRepresentation", StepReprPropertyDefinitionRepresentation, {
        the_dependent_environment: Handle<StepReprDataEnvironment>,
    }
}
impl StepReprMaterialPropertyRepresentation {
    step_methods! {
        get dependent_environment = the_dependent_environment: Handle<StepReprDataEnvironment>;
        set set_dependent_environment = the_dependent_environment: Handle<StepReprDataEnvironment>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_property_definition_representation_definition: &StepReprRepresentedDefinition,
        a_property_definition_representation_used_representation: &Handle<StepReprRepresentation>,
        a_dependent_environment: &Handle<StepReprDataEnvironment>,
    ) {
        self.parent.init(
            a_property_definition_representation_definition,
            a_property_definition_representation_used_representation,
        );
        self.state.write().the_dependent_environment = a_dependent_environment.clone();
    }
}

step_entity! {
    /// Implements a measure_representation_item entity
    /// which is used for storing validation properties
    /// (e.g. area) for shapes
    custom_default StepReprMeasureRepresentationItem, StepReprMeasureRepresentationItemState, "StepReprMeasureRepresentationItem", StepReprRepresentationItem, {
        my_measure: Handle<StepBasicMeasureWithUnit>,
    }
}
impl Default for StepReprMeasureRepresentationItem {
    fn default() -> Self {
        let object = Self {
            parent: Default::default(),
            state: Default::default(),
        };
        {
            object.state.write().my_measure = Handle::new(StepBasicMeasureWithUnit::new());
        }
        object
    }
}
impl StepReprMeasureRepresentationItem {
    /// `StepReprMeasureRepresentationItem()`, which creates the sub-entities of the class.
    pub fn new() -> Self {
        Self::default()
    }
}
impl StepReprMeasureRepresentationItem {
    step_methods! {
        set set_measure = my_measure: Handle<StepBasicMeasureWithUnit>;
        get measure = my_measure: Handle<StepBasicMeasureWithUnit>;
    }
    /// `init`.
    /// Init all fields
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_value_component: &Handle<StepBasicMeasureValueMember>,
        a_unit_component: &StepBasicUnit,
    ) -> Result<(), Failure> {
        self.parent.init(a_name);
        let field_my_measure = self.state.read().my_measure.clone();
        step_live(&field_my_measure, "StepReprMeasureRepresentationItem::Init")?
            .init(a_value_component, a_unit_component);
        Ok(())
    }
}

step_entity! {
    /// Mechanical design and draughting relationship.
    StepReprMechanicalDesignAndDraughtingRelationship, StepReprMechanicalDesignAndDraughtingRelationshipState, "StepReprMechanicalDesignAndDraughtingRelationship", StepReprRepresentationRelationship, {
    }
}

step_entity! {
    /// Representation of STEP entity NextAssemblyUsageOccurrence
    StepReprNextAssemblyUsageOccurrence, StepReprNextAssemblyUsageOccurrenceState, "StepReprNextAssemblyUsageOccurrence", StepReprAssemblyComponentUsage, {
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    StepReprParallelOffset, StepReprParallelOffsetState, "StepReprParallelOffset", StepReprDerivedShapeAspect, {
        offset: Handle<StandardTransient>,
    }
}
impl StepReprParallelOffset {
    step_methods! {
        get offset = offset: Handle<StandardTransient>;
        set set_offset = offset: Handle<StandardTransient>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_shape_aspect_name: &Handle<HAsciiString>,
        the_shape_aspect_description: &Handle<HAsciiString>,
        the_shape_aspect_of_shape: &Handle<StepReprProductDefinitionShape>,
        the_shape_aspect_product_definitional: StepDataLogical,
        the_offset: &Handle<StandardTransient>,
    ) {
        self.parent.parent.init(
            the_shape_aspect_name,
            the_shape_aspect_description,
            the_shape_aspect_of_shape,
            the_shape_aspect_product_definitional,
        );
        self.state.write().offset = the_offset.clone();
    }
}

step_entity! {
    /// Parametric representation context.
    StepReprParametricRepresentationContext, StepReprParametricRepresentationContextState, "StepReprParametricRepresentationContext", StepReprRepresentationContext, {
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    StepReprPerpendicularTo, StepReprPerpendicularToState, "StepReprPerpendicularTo", StepReprDerivedShapeAspect, {
    }
}
