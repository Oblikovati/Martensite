#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_ap214::*;
#[allow(unused_imports)]
use crate::step_basic::*;
#[allow(unused_imports)]
use crate::step_data::*;
#[allow(unused_imports)]
use crate::step_dim_tol::*;
use crate::step_entity::*;
#[allow(unused_imports)]
use crate::step_repr::*;
#[allow(unused_imports)]
use crate::step_shape::*;

step_select! {
    /// Id attribute select.
    StepAP242IdAttributeSelect, numbered [StepBasicAction = 1, StepBasicAddress = 2, StepBasicApplicationContext = 3, StepShapeDimensionalSize = 4, StepDimTolGeometricTolerance = 5, StepBasicGroup = 6, StepBasicProductCategory = 8, StepReprPropertyDefinition = 9, StepReprRepresentation = 10, StepReprShapeAspect = 11, StepReprShapeAspectRelationship = 12], {
        action: StepBasicAction,
        address: StepBasicAddress,
        application_context: StepBasicApplicationContext,
        dimensional_size: StepShapeDimensionalSize,
        geometric_tolerance: StepDimTolGeometricTolerance,
        group: StepBasicGroup,
        product_category: StepBasicProductCategory,
        property_definition: StepReprPropertyDefinition,
        representation: StepReprRepresentation,
        shape_aspect: StepReprShapeAspect,
        shape_aspect_relationship: StepReprShapeAspectRelationship,
    }
}

step_select! {
    /// Item identified representation usage definition.
    StepAP242ItemIdentifiedRepresentationUsageDefinition, [StepAP214AppliedApprovalAssignment, StepAP214AppliedDateAndTimeAssignment, StepAP214AppliedDateAssignment, StepAP214AppliedDocumentReference, StepAP214AppliedExternalIdentificationAssignment, StepAP214AppliedGroupAssignment, StepAP214AppliedOrganizationAssignment, StepAP214AppliedPersonAndOrganizationAssignment, StepAP214AppliedSecurityClassificationAssignment, StepShapeDimensionalSize, StepBasicGeneralProperty, StepDimTolGeometricTolerance, StepBasicProductDefinitionRelationship, StepReprPropertyDefinition, StepReprPropertyDefinitionRelationship, StepReprShapeAspect, StepReprShapeAspectRelationship], {
        applied_approval_assignment: StepAP214AppliedApprovalAssignment,
        applied_date_and_time_assignment: StepAP214AppliedDateAndTimeAssignment,
        applied_date_assignment: StepAP214AppliedDateAssignment,
        applied_document_reference: StepAP214AppliedDocumentReference,
        applied_external_identification_assignment: StepAP214AppliedExternalIdentificationAssignment,
        applied_group_assignment: StepAP214AppliedGroupAssignment,
        applied_organization_assignment: StepAP214AppliedOrganizationAssignment,
        applied_person_and_organization_assignment: StepAP214AppliedPersonAndOrganizationAssignment,
        applied_security_classification_assignment: StepAP214AppliedSecurityClassificationAssignment,
        dimensional_size: StepShapeDimensionalSize,
        general_property: StepBasicGeneralProperty,
        geometric_tolerance: StepDimTolGeometricTolerance,
        product_definition_relationship: StepBasicProductDefinitionRelationship,
        property_definition: StepReprPropertyDefinition,
        property_definition_relationship: StepReprPropertyDefinitionRelationship,
        shape_aspect: StepReprShapeAspect,
        shape_aspect_relationship: StepReprShapeAspectRelationship,
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    StepAP242DraughtingModelItemAssociation, StepAP242DraughtingModelItemAssociationState, "StepAP242DraughtingModelItemAssociation", StepAP242ItemIdentifiedRepresentationUsage, {
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    StepAP242GeometricItemSpecificUsage, StepAP242GeometricItemSpecificUsageState, "StepAP242GeometricItemSpecificUsage", StepAP242ItemIdentifiedRepresentationUsage, {
    }
}

step_entity! {
    /// Id attribute.
    StepAP242IdAttribute, StepAP242IdAttributeState, "StepAP242IdAttribute", StandardTransient, {
        attribute_value: Handle<HAsciiString>,
        identified_item: StepAP242IdAttributeSelect,
    }
}
impl StepAP242IdAttribute {
    step_methods! {
        set set_attribute_value = attribute_value: Handle<HAsciiString>;
        get attribute_value = attribute_value: Handle<HAsciiString>;
        set set_identified_item = identified_item: StepAP242IdAttributeSelect;
        get identified_item = identified_item: StepAP242IdAttributeSelect;
    }
    /// `init`.
    /// Init all field own and inherited
    pub fn init(
        &self,
        the_attribute_value: &Handle<HAsciiString>,
        the_identified_item: &StepAP242IdAttributeSelect,
    ) {
        let mut state = self.state.write();
        state.attribute_value = the_attribute_value.clone();
        state.identified_item = the_identified_item.clone();
    }
}

step_entity! {
    /// Item identified representation usage.
    StepAP242ItemIdentifiedRepresentationUsage, StepAP242ItemIdentifiedRepresentationUsageState, "StepAP242ItemIdentifiedRepresentationUsage", StandardTransient, {
        name: Handle<HAsciiString>,
        description: Handle<HAsciiString>,
        definition: StepAP242ItemIdentifiedRepresentationUsageDefinition,
        used_representation: Handle<StepReprRepresentation>,
        identified_item: Handle<HArray1<Handle<StepReprRepresentationItem>>>,
    }
}
impl StepAP242ItemIdentifiedRepresentationUsage {
    step_methods! {
        set set_name = name: Handle<HAsciiString>;
        get name = name: Handle<HAsciiString>;
        set set_description = description: Handle<HAsciiString>;
        get description = description: Handle<HAsciiString>;
        set set_definition = definition: StepAP242ItemIdentifiedRepresentationUsageDefinition;
        get definition = definition: StepAP242ItemIdentifiedRepresentationUsageDefinition;
        set set_used_representation = used_representation: Handle<StepReprRepresentation>;
        get used_representation = used_representation: Handle<StepReprRepresentation>;
        get identified_item = identified_item: Handle<HArray1<Handle<StepReprRepresentationItem>>>;
        len nb_identified_item = identified_item;
        set set_identified_item = identified_item: Handle<HArray1<Handle<StepReprRepresentationItem>>>;
        item identified_item_value = identified_item: Handle<StepReprRepresentationItem>;
    }
    /// `init`.
    /// Init all fields own and inherited
    pub fn init(
        &self,
        the_name: &Handle<HAsciiString>,
        the_description: &Handle<HAsciiString>,
        the_definition: &StepAP242ItemIdentifiedRepresentationUsageDefinition,
        the_used_representation: &Handle<StepReprRepresentation>,
        the_identified_item: &Handle<HArray1<Handle<StepReprRepresentationItem>>>,
    ) {
        let mut state = self.state.write();
        state.name = the_name.clone();
        state.description = the_description.clone();
        state.definition = the_definition.clone();
        state.used_representation = the_used_representation.clone();
        state.identified_item = the_identified_item.clone();
    }
    /// `set_identified_item_value`.
    /// Set identified item with given number
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_identified_item_value(
        &self,
        num: i32,
        the_item: &Handle<StepReprRepresentationItem>,
    ) -> Result<(), Failure> {
        let field_identified_item = self.state.read().identified_item.clone();
        step_array1_set(
            &field_identified_item,
            num,
            the_item.clone(),
            "StepAP242ItemIdentifiedRepresentationUsage::SetIdentifiedItemValue",
        )?;
        Ok(())
    }
}
