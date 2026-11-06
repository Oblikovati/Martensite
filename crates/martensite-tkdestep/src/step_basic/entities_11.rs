#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

step_entity! {
    /// Representation of STEP entity Product_Definition_Reference
    StepBasicProductDefinitionReference, StepBasicProductDefinitionReferenceState, "StepBasicProductDefinitionReference", StandardTransient, {
        my_source: Handle<StepBasicExternalSource>,
        my_product_id: Handle<HAsciiString>,
        my_product_definition_formation_id: Handle<HAsciiString>,
        my_product_definition_id: Handle<HAsciiString>,
        my_id_owning_organization_name: Handle<HAsciiString>,
        has_id_owning_organization_name: bool,
    }
}
impl StepBasicProductDefinitionReference {
    step_methods! {
        get source = my_source: Handle<StepBasicExternalSource>;
        set set_source = my_source: Handle<StepBasicExternalSource>;
        get product_id = my_product_id: Handle<HAsciiString>;
        set set_product_id = my_product_id: Handle<HAsciiString>;
        get product_definition_formation_id = my_product_definition_formation_id: Handle<HAsciiString>;
        set set_product_definition_formation_id = my_product_definition_formation_id: Handle<HAsciiString>;
        get product_definition_id = my_product_definition_id: Handle<HAsciiString>;
        set set_product_definition_id = my_product_definition_id: Handle<HAsciiString>;
        get id_owning_organization_name = my_id_owning_organization_name: Handle<HAsciiString>;
        get has_id_owning_organization_name = has_id_owning_organization_name: bool;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_source: &Handle<StepBasicExternalSource>,
        the_product_id: &Handle<HAsciiString>,
        the_product_definition_formation_id: &Handle<HAsciiString>,
        the_product_definition_id: &Handle<HAsciiString>,
        the_id_owning_organization_name: &Handle<HAsciiString>,
    ) {
        let mut state = self.state.write();
        state.my_source = the_source.clone();
        state.my_product_id = the_product_id.clone();
        state.my_product_definition_formation_id = the_product_definition_formation_id.clone();
        state.my_product_definition_id = the_product_definition_id.clone();
        state.my_id_owning_organization_name = the_id_owning_organization_name.clone();
        state.has_id_owning_organization_name = !the_id_owning_organization_name.is_null();
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init_fields(
        &self,
        the_source: &Handle<StepBasicExternalSource>,
        the_product_id: &Handle<HAsciiString>,
        the_product_definition_formation_id: &Handle<HAsciiString>,
        the_product_definition_id: &Handle<HAsciiString>,
    ) {
        let mut state = self.state.write();
        state.my_source = the_source.clone();
        state.my_product_id = the_product_id.clone();
        state.my_product_definition_formation_id = the_product_definition_formation_id.clone();
        state.my_product_definition_id = the_product_definition_id.clone();
        state.has_id_owning_organization_name = false;
    }
    /// `set_id_owning_organization_name`.
    /// Set field IdOwningOrganizationName
    pub fn set_id_owning_organization_name(
        &self,
        the_id_owning_organization_name: &Handle<HAsciiString>,
    ) {
        let mut state = self.state.write();
        state.my_id_owning_organization_name = the_id_owning_organization_name.clone();
        state.has_id_owning_organization_name = !the_id_owning_organization_name.is_null();
    }
}

step_entity! {
    /// Product definition reference with local representation.
    StepBasicProductDefinitionReferenceWithLocalRepresentation, StepBasicProductDefinitionReferenceWithLocalRepresentationState, "StepBasicProductDefinitionReferenceWithLocalRepresentation", StepBasicProductDefinition, {
        my_source: Handle<StepBasicExternalSource>,
    }
}
impl StepBasicProductDefinitionReferenceWithLocalRepresentation {
    step_methods! {
        get source = my_source: Handle<StepBasicExternalSource>;
        set set_source = my_source: Handle<StepBasicExternalSource>;
    }
    /// `init`.
    pub fn init(
        &self,
        the_source: &Handle<StepBasicExternalSource>,
        the_id: &Handle<HAsciiString>,
        the_description: &Handle<HAsciiString>,
        the_formation: &Handle<StepBasicProductDefinitionFormation>,
        the_frame_of_reference: &Handle<StepBasicProductDefinitionContext>,
    ) {
        self.parent.init(
            the_id,
            the_description,
            the_formation,
            the_frame_of_reference,
        );
        self.state.write().my_source = the_source.clone();
    }
}

step_entity! {
    /// Representation of STEP entity ProductDefinitionRelationship
    StepBasicProductDefinitionRelationship, StepBasicProductDefinitionRelationshipState, "StepBasicProductDefinitionRelationship", StandardTransient, {
        the_id: Handle<HAsciiString>,
        the_name: Handle<HAsciiString>,
        the_description: Handle<HAsciiString>,
        the_relating_product_definition: StepBasicProductDefinitionOrReference,
        the_related_product_definition: StepBasicProductDefinitionOrReference,
        def_description: bool,
    }
}
impl StepBasicProductDefinitionRelationship {
    step_methods! {
        get id = the_id: Handle<HAsciiString>;
        set set_id = the_id: Handle<HAsciiString>;
        get name = the_name: Handle<HAsciiString>;
        set set_name = the_name: Handle<HAsciiString>;
        get description = the_description: Handle<HAsciiString>;
        set set_description = the_description: Handle<HAsciiString>;
        get has_description = def_description: bool;
        get relating_product_definition_ap242 = the_relating_product_definition: StepBasicProductDefinitionOrReference;
        get related_product_definition_ap242 = the_related_product_definition: StepBasicProductDefinitionOrReference;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn init(
        &self,
        a_id: &Handle<HAsciiString>,
        a_name: &Handle<HAsciiString>,
        has_description: bool,
        a_description: &Handle<HAsciiString>,
        a_relating_product_definition: &Handle<StepBasicProductDefinition>,
        a_related_product_definition: &Handle<StepBasicProductDefinition>,
    ) -> Result<(), Failure> {
        let mut state = self.state.write();
        state.the_id = a_id.clone();
        state.the_name = a_name.clone();
        state.def_description = has_description;
        if state.def_description {
            state.the_description = a_description.clone();
        } else {
            state.the_description = Default::default();
        }
        state
            .the_relating_product_definition
            .set_value(&a_relating_product_definition.cast())?;
        state
            .the_related_product_definition
            .set_value(&a_related_product_definition.cast())?;
        Ok(())
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init_product_definition_or_reference(
        &self,
        a_id: &Handle<HAsciiString>,
        a_name: &Handle<HAsciiString>,
        has_description: bool,
        a_description: &Handle<HAsciiString>,
        a_relating_product_definition: &StepBasicProductDefinitionOrReference,
        a_related_product_definition: &StepBasicProductDefinitionOrReference,
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
        state.the_relating_product_definition = a_relating_product_definition.clone();
        state.the_related_product_definition = a_related_product_definition.clone();
    }
    /// `relating_product_definition`.
    /// Returns field RelatingProductDefinition
    pub fn relating_product_definition(&self) -> Handle<StepBasicProductDefinition> {
        let field_the_relating_product_definition =
            self.state.read().the_relating_product_definition.clone();
        field_the_relating_product_definition.product_definition()
    }
    /// `set_relating_product_definition`.
    /// Set field RelatingProductDefinition
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_relating_product_definition(
        &self,
        a_relating_product_definition: &Handle<StepBasicProductDefinition>,
    ) -> Result<(), Failure> {
        self.state
            .write()
            .the_relating_product_definition
            .set_value(&a_relating_product_definition.cast())?;
        Ok(())
    }
    /// `set_relating_product_definition`.
    /// Set field RelatingProductDefinition in AP242
    pub fn set_relating_product_definition_product_definition_or_reference(
        &self,
        a_relating_product_definition: &StepBasicProductDefinitionOrReference,
    ) {
        self.state.write().the_relating_product_definition = a_relating_product_definition.clone();
    }
    /// `related_product_definition`.
    /// Returns field RelatedProductDefinition
    pub fn related_product_definition(&self) -> Handle<StepBasicProductDefinition> {
        let field_the_related_product_definition =
            self.state.read().the_related_product_definition.clone();
        field_the_related_product_definition.product_definition()
    }
    /// `set_related_product_definition`.
    /// Set field RelatedProductDefinition
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_related_product_definition(
        &self,
        a_related_product_definition: &Handle<StepBasicProductDefinition>,
    ) -> Result<(), Failure> {
        self.state
            .write()
            .the_related_product_definition
            .set_value(&a_related_product_definition.cast())?;
        Ok(())
    }
    /// `set_related_product_definition`.
    /// Set field RelatedProductDefinition in AP242
    pub fn set_related_product_definition_product_definition_or_reference(
        &self,
        a_related_product_definition: &StepBasicProductDefinitionOrReference,
    ) {
        self.state.write().the_related_product_definition = a_related_product_definition.clone();
    }
}

step_entity! {
    /// Product definition with associated documents.
    StepBasicProductDefinitionWithAssociatedDocuments, StepBasicProductDefinitionWithAssociatedDocumentsState, "StepBasicProductDefinitionWithAssociatedDocuments", StepBasicProductDefinition, {
        the_doc_ids: Handle<HArray1<Handle<StepBasicDocument>>>,
    }
}
impl StepBasicProductDefinitionWithAssociatedDocuments {
    step_methods! {
        get doc_ids = the_doc_ids: Handle<HArray1<Handle<StepBasicDocument>>>;
        set set_doc_ids = the_doc_ids: Handle<HArray1<Handle<StepBasicDocument>>>;
        len nb_doc_ids = the_doc_ids;
        item doc_ids_value = the_doc_ids: Handle<StepBasicDocument>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_id: &Handle<HAsciiString>,
        a_description: &Handle<HAsciiString>,
        a_formation: &Handle<StepBasicProductDefinitionFormation>,
        a_frame: &Handle<StepBasicProductDefinitionContext>,
        a_doc_ids: &Handle<HArray1<Handle<StepBasicDocument>>>,
    ) {
        self.parent.init(a_id, a_description, a_formation, a_frame);
        self.state.write().the_doc_ids = a_doc_ids.clone();
    }
    /// `set_doc_ids_value`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_doc_ids_value(
        &self,
        num: i32,
        adoc: &Handle<StepBasicDocument>,
    ) -> Result<(), Failure> {
        let field_the_doc_ids = self.state.read().the_doc_ids.clone();
        step_array1_set(
            &field_the_doc_ids,
            num,
            adoc.clone(),
            "StepBasicProductDefinitionWithAssociatedDocuments::SetDocIdsValue",
        )?;
        Ok(())
    }
}

step_entity! {
    /// Product related product category.
    StepBasicProductRelatedProductCategory, StepBasicProductRelatedProductCategoryState, "StepBasicProductRelatedProductCategory", StepBasicProductCategory, {
        products: Handle<HArray1<Handle<StepBasicProduct>>>,
    }
}
impl StepBasicProductRelatedProductCategory {
    step_methods! {
        set set_products = products: Handle<HArray1<Handle<StepBasicProduct>>>;
        get products = products: Handle<HArray1<Handle<StepBasicProduct>>>;
        item products_value = products: Handle<StepBasicProduct>;
        len nb_products = products;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        has_adescription: bool,
        a_description: &Handle<HAsciiString>,
        a_products: &Handle<HArray1<Handle<StepBasicProduct>>>,
    ) {
        self.state.write().products = a_products.clone();
        self.parent.init(a_name, has_adescription, a_description);
    }
}

step_entity! {
    /// Product type.
    StepBasicProductType, StepBasicProductTypeState, "StepBasicProductType", StepBasicProductRelatedProductCategory, {
    }
}

step_entity! {
    /// Ratio measure with unit.
    StepBasicRatioMeasureWithUnit, StepBasicRatioMeasureWithUnitState, "StepBasicRatioMeasureWithUnit", StepBasicMeasureWithUnit, {
    }
}

step_entity! {
    /// Ratio unit.
    StepBasicRatioUnit, StepBasicRatioUnitState, "StepBasicRatioUnit", StepBasicNamedUnit, {
    }
}

step_entity! {
    /// Representation of STEP entity RoleAssociation
    StepBasicRoleAssociation, StepBasicRoleAssociationState, "StepBasicRoleAssociation", StandardTransient, {
        the_role: Handle<StepBasicObjectRole>,
        the_item_with_role: StepBasicRoleSelect,
    }
}
impl StepBasicRoleAssociation {
    step_methods! {
        get role = the_role: Handle<StepBasicObjectRole>;
        set set_role = the_role: Handle<StepBasicObjectRole>;
        get item_with_role = the_item_with_role: StepBasicRoleSelect;
        set set_item_with_role = the_item_with_role: StepBasicRoleSelect;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_role: &Handle<StepBasicObjectRole>,
        a_item_with_role: &StepBasicRoleSelect,
    ) {
        let mut state = self.state.write();
        state.the_role = a_role.clone();
        state.the_item_with_role = a_item_with_role.clone();
    }
}
