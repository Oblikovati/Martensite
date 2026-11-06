#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

step_entity! {
    /// Product category.
    StepBasicProductCategory, StepBasicProductCategoryState, "StepBasicProductCategory", StandardTransient, {
        name: Handle<HAsciiString>,
        description: Handle<HAsciiString>,
        has_description: bool,
    }
}
impl StepBasicProductCategory {
    step_methods! {
        set set_name = name: Handle<HAsciiString>;
        get name = name: Handle<HAsciiString>;
        set set_description = description flag has_description: Handle<HAsciiString>;
        unset un_set_description = description flag has_description;
        get description = description: Handle<HAsciiString>;
        get has_description = has_description: bool;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        has_adescription: bool,
        a_description: &Handle<HAsciiString>,
    ) {
        let mut state = self.state.write();
        state.name = a_name.clone();
        state.has_description = has_adescription;
        state.description = a_description.clone();
    }
}

step_entity! {
    /// Representation of STEP entity ProductCategoryRelationship
    StepBasicProductCategoryRelationship, StepBasicProductCategoryRelationshipState, "StepBasicProductCategoryRelationship", StandardTransient, {
        the_name: Handle<HAsciiString>,
        the_description: Handle<HAsciiString>,
        the_category: Handle<StepBasicProductCategory>,
        the_sub_category: Handle<StepBasicProductCategory>,
        def_description: bool,
    }
}
impl StepBasicProductCategoryRelationship {
    step_methods! {
        get name = the_name: Handle<HAsciiString>;
        set set_name = the_name: Handle<HAsciiString>;
        get description = the_description: Handle<HAsciiString>;
        set set_description = the_description: Handle<HAsciiString>;
        get has_description = def_description: bool;
        get category = the_category: Handle<StepBasicProductCategory>;
        set set_category = the_category: Handle<StepBasicProductCategory>;
        get sub_category = the_sub_category: Handle<StepBasicProductCategory>;
        set set_sub_category = the_sub_category: Handle<StepBasicProductCategory>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        has_description: bool,
        a_description: &Handle<HAsciiString>,
        a_category: &Handle<StepBasicProductCategory>,
        a_sub_category: &Handle<StepBasicProductCategory>,
    ) {
        let mut state = self.state.write();
        state.the_name = a_name.clone();
        state.def_description = has_description;
        if state.def_description {
            state.the_description = a_description.clone();
        } else {
            state.the_description = Default::default();
        }
        state.the_category = a_category.clone();
        state.the_sub_category = a_sub_category.clone();
    }
}

step_entity! {
    /// Representation of STEP entity ProductConceptContext
    StepBasicProductConceptContext, StepBasicProductConceptContextState, "StepBasicProductConceptContext", StepBasicApplicationContextElement, {
        the_market_segment_type: Handle<HAsciiString>,
    }
}
impl StepBasicProductConceptContext {
    step_methods! {
        get market_segment_type = the_market_segment_type: Handle<HAsciiString>;
        set set_market_segment_type = the_market_segment_type: Handle<HAsciiString>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_application_context_element_name: &Handle<HAsciiString>,
        a_application_context_element_frame_of_reference: &Handle<StepBasicApplicationContext>,
        a_market_segment_type: &Handle<HAsciiString>,
    ) {
        self.parent.init(
            a_application_context_element_name,
            a_application_context_element_frame_of_reference,
        );
        self.state.write().the_market_segment_type = a_market_segment_type.clone();
    }
}

step_entity! {
    /// Product context.
    StepBasicProductContext, StepBasicProductContextState, "StepBasicProductContext", StepBasicApplicationContextElement, {
        discipline_type: Handle<HAsciiString>,
    }
}
impl StepBasicProductContext {
    step_methods! {
        set set_discipline_type = discipline_type: Handle<HAsciiString>;
        get discipline_type = discipline_type: Handle<HAsciiString>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_frame_of_reference: &Handle<StepBasicApplicationContext>,
        a_discipline_type: &Handle<HAsciiString>,
    ) {
        self.state.write().discipline_type = a_discipline_type.clone();
        self.parent.init(a_name, a_frame_of_reference);
    }
}

step_entity! {
    /// Product definition.
    StepBasicProductDefinition, StepBasicProductDefinitionState, "StepBasicProductDefinition", StandardTransient, {
        id: Handle<HAsciiString>,
        description: Handle<HAsciiString>,
        formation: Handle<StepBasicProductDefinitionFormation>,
        frame_of_reference: Handle<StepBasicProductDefinitionContext>,
    }
}
impl StepBasicProductDefinition {
    step_methods! {
        set set_id = id: Handle<HAsciiString>;
        get id = id: Handle<HAsciiString>;
        set set_description = description: Handle<HAsciiString>;
        get description = description: Handle<HAsciiString>;
        set set_formation = formation: Handle<StepBasicProductDefinitionFormation>;
        get formation = formation: Handle<StepBasicProductDefinitionFormation>;
        set set_frame_of_reference = frame_of_reference: Handle<StepBasicProductDefinitionContext>;
        get frame_of_reference = frame_of_reference: Handle<StepBasicProductDefinitionContext>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_id: &Handle<HAsciiString>,
        a_description: &Handle<HAsciiString>,
        a_formation: &Handle<StepBasicProductDefinitionFormation>,
        a_frame_of_reference: &Handle<StepBasicProductDefinitionContext>,
    ) {
        let mut state = self.state.write();
        state.id = a_id.clone();
        state.description = a_description.clone();
        state.formation = a_formation.clone();
        state.frame_of_reference = a_frame_of_reference.clone();
    }
}

step_entity! {
    /// Product definition context.
    StepBasicProductDefinitionContext, StepBasicProductDefinitionContextState, "StepBasicProductDefinitionContext", StepBasicApplicationContextElement, {
        life_cycle_stage: Handle<HAsciiString>,
    }
}
impl StepBasicProductDefinitionContext {
    step_methods! {
        set set_life_cycle_stage = life_cycle_stage: Handle<HAsciiString>;
        get life_cycle_stage = life_cycle_stage: Handle<HAsciiString>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_frame_of_reference: &Handle<StepBasicApplicationContext>,
        a_life_cycle_stage: &Handle<HAsciiString>,
    ) {
        self.state.write().life_cycle_stage = a_life_cycle_stage.clone();
        self.parent.init(a_name, a_frame_of_reference);
    }
}

step_entity! {
    /// Product definition effectivity.
    StepBasicProductDefinitionEffectivity, StepBasicProductDefinitionEffectivityState, "StepBasicProductDefinitionEffectivity", StepBasicEffectivity, {
        the_usage: Handle<StepBasicProductDefinitionRelationship>,
    }
}
impl StepBasicProductDefinitionEffectivity {
    step_methods! {
        get usage = the_usage: Handle<StepBasicProductDefinitionRelationship>;
        set set_usage = the_usage: Handle<StepBasicProductDefinitionRelationship>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_id: &Handle<HAsciiString>,
        a_usage: &Handle<StepBasicProductDefinitionRelationship>,
    ) {
        self.set_id(a_id);
        self.state.write().the_usage = a_usage.clone();
    }
}

step_entity! {
    /// Product definition formation.
    StepBasicProductDefinitionFormation, StepBasicProductDefinitionFormationState, "StepBasicProductDefinitionFormation", StandardTransient, {
        id: Handle<HAsciiString>,
        description: Handle<HAsciiString>,
        of_product: Handle<StepBasicProduct>,
    }
}
impl StepBasicProductDefinitionFormation {
    step_methods! {
        set set_id = id: Handle<HAsciiString>;
        get id = id: Handle<HAsciiString>;
        set set_description = description: Handle<HAsciiString>;
        get description = description: Handle<HAsciiString>;
        set set_of_product = of_product: Handle<StepBasicProduct>;
        get of_product = of_product: Handle<StepBasicProduct>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_id: &Handle<HAsciiString>,
        a_description: &Handle<HAsciiString>,
        a_of_product: &Handle<StepBasicProduct>,
    ) {
        let mut state = self.state.write();
        state.id = a_id.clone();
        state.description = a_description.clone();
        state.of_product = a_of_product.clone();
    }
}

step_entity! {
    /// Representation of STEP entity ProductDefinitionFormationRelationship
    StepBasicProductDefinitionFormationRelationship, StepBasicProductDefinitionFormationRelationshipState, "StepBasicProductDefinitionFormationRelationship", StandardTransient, {
        the_id: Handle<HAsciiString>,
        the_name: Handle<HAsciiString>,
        the_description: Handle<HAsciiString>,
        the_relating_product_definition_formation: Handle<StepBasicProductDefinitionFormation>,
        the_related_product_definition_formation: Handle<StepBasicProductDefinitionFormation>,
    }
}
impl StepBasicProductDefinitionFormationRelationship {
    step_methods! {
        get id = the_id: Handle<HAsciiString>;
        set set_id = the_id: Handle<HAsciiString>;
        get name = the_name: Handle<HAsciiString>;
        set set_name = the_name: Handle<HAsciiString>;
        get description = the_description: Handle<HAsciiString>;
        set set_description = the_description: Handle<HAsciiString>;
        get relating_product_definition_formation = the_relating_product_definition_formation: Handle<StepBasicProductDefinitionFormation>;
        set set_relating_product_definition_formation = the_relating_product_definition_formation: Handle<StepBasicProductDefinitionFormation>;
        get related_product_definition_formation = the_related_product_definition_formation: Handle<StepBasicProductDefinitionFormation>;
        set set_related_product_definition_formation = the_related_product_definition_formation: Handle<StepBasicProductDefinitionFormation>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_id: &Handle<HAsciiString>,
        a_name: &Handle<HAsciiString>,
        a_description: &Handle<HAsciiString>,
        a_relating_product_definition_formation: &Handle<StepBasicProductDefinitionFormation>,
        a_related_product_definition_formation: &Handle<StepBasicProductDefinitionFormation>,
    ) {
        let mut state = self.state.write();
        state.the_id = a_id.clone();
        state.the_name = a_name.clone();
        state.the_description = a_description.clone();
        state.the_relating_product_definition_formation =
            a_relating_product_definition_formation.clone();
        state.the_related_product_definition_formation =
            a_related_product_definition_formation.clone();
    }
}

step_entity! {
    /// Product definition formation with specified source.
    StepBasicProductDefinitionFormationWithSpecifiedSource, StepBasicProductDefinitionFormationWithSpecifiedSourceState, "StepBasicProductDefinitionFormationWithSpecifiedSource", StepBasicProductDefinitionFormation, {
        make_or_buy: StepBasicSource,
    }
}
impl StepBasicProductDefinitionFormationWithSpecifiedSource {
    step_methods! {
        set_copy set_make_or_buy = make_or_buy: StepBasicSource;
        get make_or_buy = make_or_buy: StepBasicSource;
    }
    /// `init`.
    pub fn init(
        &self,
        a_id: &Handle<HAsciiString>,
        a_description: &Handle<HAsciiString>,
        a_of_product: &Handle<StepBasicProduct>,
        a_make_or_buy: StepBasicSource,
    ) {
        self.state.write().make_or_buy = a_make_or_buy;
        self.parent.init(a_id, a_description, a_of_product);
    }
}
