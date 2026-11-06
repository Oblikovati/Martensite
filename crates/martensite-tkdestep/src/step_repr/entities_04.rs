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
    /// Representation of STEP entity ProductConcept
    StepReprProductConcept, StepReprProductConceptState, "StepReprProductConcept", StandardTransient, {
        the_id: Handle<HAsciiString>,
        the_name: Handle<HAsciiString>,
        the_description: Handle<HAsciiString>,
        the_market_context: Handle<StepBasicProductConceptContext>,
        def_description: bool,
    }
}
impl StepReprProductConcept {
    step_methods! {
        get id = the_id: Handle<HAsciiString>;
        set set_id = the_id: Handle<HAsciiString>;
        get name = the_name: Handle<HAsciiString>;
        set set_name = the_name: Handle<HAsciiString>;
        get description = the_description: Handle<HAsciiString>;
        set set_description = the_description: Handle<HAsciiString>;
        get has_description = def_description: bool;
        get market_context = the_market_context: Handle<StepBasicProductConceptContext>;
        set set_market_context = the_market_context: Handle<StepBasicProductConceptContext>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_id: &Handle<HAsciiString>,
        a_name: &Handle<HAsciiString>,
        has_description: bool,
        a_description: &Handle<HAsciiString>,
        a_market_context: &Handle<StepBasicProductConceptContext>,
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
        state.the_market_context = a_market_context.clone();
    }
}

step_entity! {
    /// Representation of STEP entity ProductDefinitionShape
    StepReprProductDefinitionShape, StepReprProductDefinitionShapeState, "StepReprProductDefinitionShape", StepReprPropertyDefinition, {
    }
}

step_entity! {
    /// Representation of STEP entity ProductDefinitionUsage
    StepReprProductDefinitionUsage, StepReprProductDefinitionUsageState, "StepReprProductDefinitionUsage", StepBasicProductDefinitionRelationship, {
    }
}

step_entity! {
    /// Promissory usage occurrence.
    StepReprPromissoryUsageOccurrence, StepReprPromissoryUsageOccurrenceState, "StepReprPromissoryUsageOccurrence", StepReprAssemblyComponentUsage, {
    }
}

step_entity! {
    /// Representation of STEP entity PropertyDefinition
    StepReprPropertyDefinition, StepReprPropertyDefinitionState, "StepReprPropertyDefinition", StandardTransient, {
        the_name: Handle<HAsciiString>,
        the_description: Handle<HAsciiString>,
        the_definition: StepReprCharacterizedDefinition,
        def_description: bool,
    }
}
impl StepReprPropertyDefinition {
    step_methods! {
        get name = the_name: Handle<HAsciiString>;
        set set_name = the_name: Handle<HAsciiString>;
        get description = the_description: Handle<HAsciiString>;
        set set_description = the_description: Handle<HAsciiString>;
        get has_description = def_description: bool;
        get definition = the_definition: StepReprCharacterizedDefinition;
        set set_definition = the_definition: StepReprCharacterizedDefinition;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        has_description: bool,
        a_description: &Handle<HAsciiString>,
        a_definition: &StepReprCharacterizedDefinition,
    ) {
        let mut state = self.state.write();
        state.the_name = a_name.clone();
        state.def_description = has_description;
        if state.def_description {
            state.the_description = a_description.clone();
        } else {
            state.the_description = Default::default();
        }
        state.the_definition = a_definition.clone();
    }
}

step_entity! {
    /// Representation of STEP entity PropertyDefinitionRelationship
    StepReprPropertyDefinitionRelationship, StepReprPropertyDefinitionRelationshipState, "StepReprPropertyDefinitionRelationship", StandardTransient, {
        the_name: Handle<HAsciiString>,
        the_description: Handle<HAsciiString>,
        the_relating_property_definition: Handle<StepReprPropertyDefinition>,
        the_related_property_definition: Handle<StepReprPropertyDefinition>,
    }
}
impl StepReprPropertyDefinitionRelationship {
    step_methods! {
        get name = the_name: Handle<HAsciiString>;
        set set_name = the_name: Handle<HAsciiString>;
        get description = the_description: Handle<HAsciiString>;
        set set_description = the_description: Handle<HAsciiString>;
        get relating_property_definition = the_relating_property_definition: Handle<StepReprPropertyDefinition>;
        set set_relating_property_definition = the_relating_property_definition: Handle<StepReprPropertyDefinition>;
        get related_property_definition = the_related_property_definition: Handle<StepReprPropertyDefinition>;
        set set_related_property_definition = the_related_property_definition: Handle<StepReprPropertyDefinition>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_description: &Handle<HAsciiString>,
        a_relating_property_definition: &Handle<StepReprPropertyDefinition>,
        a_related_property_definition: &Handle<StepReprPropertyDefinition>,
    ) {
        let mut state = self.state.write();
        state.the_name = a_name.clone();
        state.the_description = a_description.clone();
        state.the_relating_property_definition = a_relating_property_definition.clone();
        state.the_related_property_definition = a_related_property_definition.clone();
    }
}

step_entity! {
    /// Representation of STEP entity PropertyDefinitionRepresentation
    StepReprPropertyDefinitionRepresentation, StepReprPropertyDefinitionRepresentationState, "StepReprPropertyDefinitionRepresentation", StandardTransient, {
        the_definition: StepReprRepresentedDefinition,
        the_used_representation: Handle<StepReprRepresentation>,
    }
}
impl StepReprPropertyDefinitionRepresentation {
    step_methods! {
        get definition = the_definition: StepReprRepresentedDefinition;
        set set_definition = the_definition: StepReprRepresentedDefinition;
        get used_representation = the_used_representation: Handle<StepReprRepresentation>;
        set set_used_representation = the_used_representation: Handle<StepReprRepresentation>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_definition: &StepReprRepresentedDefinition,
        a_used_representation: &Handle<StepReprRepresentation>,
    ) {
        let mut state = self.state.write();
        state.the_definition = a_definition.clone();
        state.the_used_representation = a_used_representation.clone();
    }
}

step_entity! {
    /// Representation of STEP entity QuantifiedAssemblyComponentUsage
    StepReprQuantifiedAssemblyComponentUsage, StepReprQuantifiedAssemblyComponentUsageState, "StepReprQuantifiedAssemblyComponentUsage", StepReprAssemblyComponentUsage, {
        the_quantity: Handle<StandardTransient>,
    }
}
impl StepReprQuantifiedAssemblyComponentUsage {
    step_methods! {
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
        has_assembly_component_usage_reference_designator: bool,
        a_assembly_component_usage_reference_designator: &Handle<HAsciiString>,
        a_quantity: &Handle<StandardTransient>,
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
        self.state.write().the_quantity = a_quantity.clone();
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
        a_quantity: &Handle<StandardTransient>,
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
        self.state.write().the_quantity = a_quantity.clone();
    }
}

step_entity! {
    /// Real representation item.
    StepReprRealRepresentationItem, StepReprRealRepresentationItemState, "StepReprRealRepresentationItem", StepReprRepresentationItem, {
        value: f64,
    }
}
impl StepReprRealRepresentationItem {
    step_methods! {
        set_copy set_value = value: f64;
        get value = value: f64;
    }
    /// `init`.
    pub fn init(&self, the_name: &Handle<HAsciiString>, the_value: f64) {
        self.state.write().value = the_value;
        self.parent.init(the_name);
    }
}

step_entity! {
    /// Repr item and length measure with unit.
    custom_default StepReprReprItemAndLengthMeasureWithUnit, StepReprReprItemAndLengthMeasureWithUnitState, "StepReprReprItemAndLengthMeasureWithUnit", StepReprReprItemAndMeasureWithUnit, {
        my_length_measure_with_unit: Handle<StepBasicLengthMeasureWithUnit>,
    }
}
impl Default for StepReprReprItemAndLengthMeasureWithUnit {
    fn default() -> Self {
        let object = Self {
            parent: Default::default(),
            state: Default::default(),
        };
        {
            object.state.write().my_length_measure_with_unit =
                Handle::new(StepBasicLengthMeasureWithUnit::new());
        }
        object
    }
}
impl StepReprReprItemAndLengthMeasureWithUnit {
    /// `StepReprReprItemAndLengthMeasureWithUnit()`, which creates the sub-entities of the class.
    pub fn new() -> Self {
        Self::default()
    }
}
impl StepReprReprItemAndLengthMeasureWithUnit {
    step_methods! {
        set set_length_measure_with_unit = my_length_measure_with_unit: Handle<StepBasicLengthMeasureWithUnit>;
        get get_length_measure_with_unit = my_length_measure_with_unit: Handle<StepBasicLengthMeasureWithUnit>;
    }
}

step_entity! {
    /// Repr item and length measure with unit and QRI.
    custom_default StepReprReprItemAndLengthMeasureWithUnitAndQRI, StepReprReprItemAndLengthMeasureWithUnitAndQRIState, "StepReprReprItemAndLengthMeasureWithUnitAndQRI", StepReprReprItemAndMeasureWithUnitAndQRI, {
        my_length_measure_with_unit: Handle<StepBasicLengthMeasureWithUnit>,
    }
}
impl Default for StepReprReprItemAndLengthMeasureWithUnitAndQRI {
    fn default() -> Self {
        let object = Self {
            parent: Default::default(),
            state: Default::default(),
        };
        {
            object.state.write().my_length_measure_with_unit =
                Handle::new(StepBasicLengthMeasureWithUnit::new());
        }
        object
    }
}
impl StepReprReprItemAndLengthMeasureWithUnitAndQRI {
    /// `StepReprReprItemAndLengthMeasureWithUnitAndQRI()`, which creates the sub-entities of the class.
    pub fn new() -> Self {
        Self::default()
    }
}
impl StepReprReprItemAndLengthMeasureWithUnitAndQRI {
    step_methods! {
        set set_length_measure_with_unit = my_length_measure_with_unit: Handle<StepBasicLengthMeasureWithUnit>;
        get get_length_measure_with_unit = my_length_measure_with_unit: Handle<StepBasicLengthMeasureWithUnit>;
    }
}
