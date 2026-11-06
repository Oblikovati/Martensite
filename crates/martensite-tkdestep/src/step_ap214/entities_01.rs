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
#[allow(unused_imports)]
use crate::step_repr::*;
#[allow(unused_imports)]
use crate::step_shape::*;
#[allow(unused_imports)]
use crate::step_visual::*;

step_select! {
    /// Approval item.
    StepAP214ApprovalItem, [StepReprAssemblyComponentUsageSubstitute, StepBasicDocumentFile, StepReprMaterialDesignation, StepVisualMechanicalDesignGeometricPresentationRepresentation, StepVisualPresentationArea, StepBasicProduct, StepBasicProductDefinition, StepBasicProductDefinitionFormation, StepBasicProductDefinitionRelationship, StepReprPropertyDefinition, StepShapeShapeRepresentation, StepBasicSecurityClassification, StepReprConfigurationItem, StepBasicDate, StepBasicDocument, StepBasicEffectivity, StepBasicGroup, StepBasicGroupRelationship, StepBasicProductDefinitionFormationRelationship, StepReprRepresentation, StepReprShapeAspectRelationship], {
        assembly_component_usage_substitute: StepReprAssemblyComponentUsageSubstitute,
        document_file: StepBasicDocumentFile,
        material_designation: StepReprMaterialDesignation,
        mechanical_design_geometric_presentation_representation: StepVisualMechanicalDesignGeometricPresentationRepresentation,
        presentation_area: StepVisualPresentationArea,
        product: StepBasicProduct,
        product_definition: StepBasicProductDefinition,
        product_definition_formation: StepBasicProductDefinitionFormation,
        product_definition_relationship: StepBasicProductDefinitionRelationship,
        property_definition: StepReprPropertyDefinition,
        shape_representation: StepShapeShapeRepresentation,
        security_classification: StepBasicSecurityClassification,
        configuration_item: StepReprConfigurationItem,
        date: StepBasicDate,
        document: StepBasicDocument,
        effectivity: StepBasicEffectivity,
        group: StepBasicGroup,
        group_relationship: StepBasicGroupRelationship,
        product_definition_formation_relationship: StepBasicProductDefinitionFormationRelationship,
        representation: StepReprRepresentation,
        shape_aspect_relationship: StepReprShapeAspectRelationship,
    }
}

step_select! {
    /// Auto design date and person item.
    StepAP214AutoDesignDateAndPersonItem, [StepAP214AutoDesignOrganizationAssignment, StepBasicProduct, StepBasicProductDefinition, StepBasicProductDefinitionFormation, StepReprRepresentation, StepAP214AutoDesignDocumentReference, StepReprExternallyDefinedRepresentation, StepBasicProductDefinitionRelationship, StepBasicProductDefinitionWithAssociatedDocuments], {
        auto_design_organization_assignment: StepAP214AutoDesignOrganizationAssignment,
        product: StepBasicProduct,
        product_definition: StepBasicProductDefinition,
        product_definition_formation: StepBasicProductDefinitionFormation,
        representation: StepReprRepresentation,
        auto_design_document_reference: StepAP214AutoDesignDocumentReference,
        externally_defined_representation: StepReprExternallyDefinedRepresentation,
        product_definition_relationship: StepBasicProductDefinitionRelationship,
        product_definition_with_associated_documents: StepBasicProductDefinitionWithAssociatedDocuments,
    }
}

step_select! {
    /// Auto design date and time item.
    StepAP214AutoDesignDateAndTimeItem, [StepBasicApprovalPersonOrganization, StepAP214AutoDesignDateAndPersonAssignment, StepBasicProductDefinitionEffectivity], {
        approval_person_organization: StepBasicApprovalPersonOrganization,
        auto_design_date_and_person_assignment: StepAP214AutoDesignDateAndPersonAssignment,
        product_definition_effectivity: StepBasicProductDefinitionEffectivity,
    }
}

step_select! {
    /// Auto design dated item.
    StepAP214AutoDesignDatedItem, [StepBasicApprovalPersonOrganization, StepAP214AutoDesignDateAndPersonAssignment, StepBasicProductDefinitionEffectivity], {
        approval_person_organization: StepBasicApprovalPersonOrganization,
        auto_design_date_and_person_assignment: StepAP214AutoDesignDateAndPersonAssignment,
        product_definition_effectivity: StepBasicProductDefinitionEffectivity,
    }
}

step_select! {
    /// Auto design general org item.
    StepAP214AutoDesignGeneralOrgItem, [StepBasicProduct, StepBasicProductDefinition, StepBasicProductDefinitionFormation, StepBasicProductDefinitionRelationship, StepBasicProductDefinitionWithAssociatedDocuments, StepReprRepresentation, StepReprExternallyDefinedRepresentation, StepAP214AutoDesignDocumentReference], {
        product: StepBasicProduct,
        product_definition: StepBasicProductDefinition,
        product_definition_formation: StepBasicProductDefinitionFormation,
        product_definition_relationship: StepBasicProductDefinitionRelationship,
        product_definition_with_associated_documents: StepBasicProductDefinitionWithAssociatedDocuments,
        representation: StepReprRepresentation,
        externally_defined_representation: StepReprExternallyDefinedRepresentation,
        auto_design_document_reference: StepAP214AutoDesignDocumentReference,
    }
}

step_select! {
    /// Auto design grouped item.
    StepAP214AutoDesignGroupedItem, [StepShapeAdvancedBrepShapeRepresentation, StepShapeCsgShapeRepresentation, StepShapeFacetedBrepShapeRepresentation, StepShapeGeometricallyBoundedSurfaceShapeRepresentation, StepShapeGeometricallyBoundedWireframeShapeRepresentation, StepShapeManifoldSurfaceShapeRepresentation, StepReprRepresentation, StepReprRepresentationItem, StepShapeShapeRepresentation, StepReprShapeAspect, StepVisualTemplateInstance], {
        advanced_brep_shape_representation: StepShapeAdvancedBrepShapeRepresentation,
        csg_shape_representation: StepShapeCsgShapeRepresentation,
        faceted_brep_shape_representation: StepShapeFacetedBrepShapeRepresentation,
        geometrically_bounded_surface_shape_representation: StepShapeGeometricallyBoundedSurfaceShapeRepresentation,
        geometrically_bounded_wireframe_shape_representation: StepShapeGeometricallyBoundedWireframeShapeRepresentation,
        manifold_surface_shape_representation: StepShapeManifoldSurfaceShapeRepresentation,
        representation: StepReprRepresentation,
        representation_item: StepReprRepresentationItem,
        shape_aspect: StepReprShapeAspect,
        shape_representation: StepShapeShapeRepresentation,
        template_instance: StepVisualTemplateInstance,
    }
}

step_select! {
    /// Auto design presented item select.
    StepAP214AutoDesignPresentedItemSelect, [StepBasicProductDefinition, StepBasicProductDefinitionRelationship, StepReprProductDefinitionShape, StepReprRepresentationRelationship, StepReprShapeAspect, StepBasicDocumentRelationship], {
        product_definition_relationship: StepBasicProductDefinitionRelationship,
        product_definition: StepBasicProductDefinition,
        product_definition_shape: StepReprProductDefinitionShape,
        representation_relationship: StepReprRepresentationRelationship,
        shape_aspect: StepReprShapeAspect,
        document_relationship: StepBasicDocumentRelationship,
    }
}

step_select! {
    /// Auto design referencing item.
    StepAP214AutoDesignReferencingItem, [StepBasicApproval, StepBasicDocumentRelationship, StepReprExternallyDefinedRepresentation, StepReprMappedItem, StepReprMaterialDesignation, StepVisualPresentationArea, StepVisualPresentationView, StepBasicProductCategory, StepBasicProductDefinition, StepBasicProductDefinitionRelationship, StepReprPropertyDefinition, StepReprRepresentation, StepReprRepresentationRelationship, StepReprShapeAspect], {
        approval: StepBasicApproval,
        document_relationship: StepBasicDocumentRelationship,
        externally_defined_representation: StepReprExternallyDefinedRepresentation,
        mapped_item: StepReprMappedItem,
        material_designation: StepReprMaterialDesignation,
        presentation_area: StepVisualPresentationArea,
        presentation_view: StepVisualPresentationView,
        product_category: StepBasicProductCategory,
        product_definition: StepBasicProductDefinition,
        product_definition_relationship: StepBasicProductDefinitionRelationship,
        property_definition: StepReprPropertyDefinition,
        representation: StepReprRepresentation,
        representation_relationship: StepReprRepresentationRelationship,
        shape_aspect: StepReprShapeAspect,
    }
}

step_select! {
    /// Document reference item.
    StepAP214DocumentReferenceItem, [StepBasicApproval, StepReprDescriptiveRepresentationItem, StepReprMaterialDesignation, StepBasicProductDefinition, StepBasicProductDefinitionRelationship, StepReprPropertyDefinition, StepReprRepresentation, StepReprShapeAspect, StepReprShapeAspectRelationship, StepAP214AppliedExternalIdentificationAssignment, StepReprAssemblyComponentUsage, StepBasicCharacterizedObject, StepShapeDimensionalSize, StepBasicExternallyDefinedItem, StepBasicGroup, StepBasicGroupRelationship, StepReprMeasureRepresentationItem, StepBasicProductCategory, StepBasicProductDefinitionContext, StepReprRepresentationItem], {
        approval: StepBasicApproval,
        descriptive_representation_item: StepReprDescriptiveRepresentationItem,
        material_designation: StepReprMaterialDesignation,
        product_definition: StepBasicProductDefinition,
        product_definition_relationship: StepBasicProductDefinitionRelationship,
        property_definition: StepReprPropertyDefinition,
        representation: StepReprRepresentation,
        shape_aspect: StepReprShapeAspect,
        shape_aspect_relationship: StepReprShapeAspectRelationship,
        applied_external_identification_assignment: StepAP214AppliedExternalIdentificationAssignment,
        assembly_component_usage: StepReprAssemblyComponentUsage,
        characterized_object: StepBasicCharacterizedObject,
        dimensional_size: StepShapeDimensionalSize,
        externally_defined_item: StepBasicExternallyDefinedItem,
        group: StepBasicGroup,
        group_relationship: StepBasicGroupRelationship,
        measure_representation_item: StepReprMeasureRepresentationItem,
        product_category: StepBasicProductCategory,
        product_definition_context: StepBasicProductDefinitionContext,
        representation_item: StepReprRepresentationItem,
    }
}

step_select! {
    /// Representation of STEP SELECT type ExternalIdentificationItem
    StepAP214ExternalIdentificationItem, [StepBasicDocumentFile, StepAP214ExternallyDefinedClass, StepAP214ExternallyDefinedGeneralProperty, StepBasicProductDefinition, StepAP214AppliedOrganizationAssignment, StepAP214AppliedPersonAndOrganizationAssignment, StepBasicApproval, StepBasicApprovalStatus, StepBasicExternalSource, StepBasicOrganizationalAddress, StepBasicSecurityClassification, StepGeomTrimmedCurve, StepBasicVersionedActionRequest, StepBasicDateAndTimeAssignment, StepBasicDateAssignment], {
        document_file: StepBasicDocumentFile,
        externally_defined_class: StepAP214ExternallyDefinedClass,
        externally_defined_general_property: StepAP214ExternallyDefinedGeneralProperty,
        product_definition: StepBasicProductDefinition,
        applied_organization_assignment: StepAP214AppliedOrganizationAssignment,
        applied_person_and_organization_assignment: StepAP214AppliedPersonAndOrganizationAssignment,
        approval: StepBasicApproval,
        approval_status: StepBasicApprovalStatus,
        external_source: StepBasicExternalSource,
        organizational_address: StepBasicOrganizationalAddress,
        security_classification: StepBasicSecurityClassification,
        trimmed_curve: StepGeomTrimmedCurve,
        versioned_action_request: StepBasicVersionedActionRequest,
        date_and_time_assignment: StepBasicDateAndTimeAssignment,
        date_assignment: StepBasicDateAssignment,
    }
}

step_select! {
    /// Group item.
    StepAP214GroupItem, [StepGeomGeometricRepresentationItem, StepBasicGroupRelationship, StepReprMappedItem, StepBasicProductDefinition, StepBasicProductDefinitionFormation, StepReprPropertyDefinitionRepresentation, StepReprRepresentation, StepReprRepresentationItem, StepReprRepresentationRelationshipWithTransformation, StepReprShapeAspect, StepReprShapeAspectRelationship, StepReprShapeRepresentationRelationship, StepVisualStyledItem, StepShapeTopologicalRepresentationItem], {
        geometric_representation_item: StepGeomGeometricRepresentationItem,
        group_relationship: StepBasicGroupRelationship,
        mapped_item: StepReprMappedItem,
        product_definition: StepBasicProductDefinition,
        product_definition_formation: StepBasicProductDefinitionFormation,
        property_definition_representation: StepReprPropertyDefinitionRepresentation,
        representation: StepReprRepresentation,
        representation_item: StepReprRepresentationItem,
        representation_relationship_with_transformation: StepReprRepresentationRelationshipWithTransformation,
        shape_aspect: StepReprShapeAspect,
        shape_aspect_relationship: StepReprShapeAspectRelationship,
        shape_representation_relationship: StepReprShapeRepresentationRelationship,
        styled_item: StepVisualStyledItem,
        topological_representation_item: StepShapeTopologicalRepresentationItem,
    }
}

step_select! {
    /// Presented item select.
    StepAP214PresentedItemSelect, [StepBasicProductDefinition, StepBasicProductDefinitionRelationship], {
        product_definition_relationship: StepBasicProductDefinitionRelationship,
        product_definition: StepBasicProductDefinition,
    }
}

step_entity! {
    /// Applied approval assignment.
    StepAP214AppliedApprovalAssignment, StepAP214AppliedApprovalAssignmentState, "StepAP214AppliedApprovalAssignment", StepBasicApprovalAssignment, {
        items: Handle<HArray1<StepAP214ApprovalItem>>,
    }
}
impl StepAP214AppliedApprovalAssignment {
    step_methods! {
        set set_items = items: Handle<HArray1<StepAP214ApprovalItem>>;
        get items = items: Handle<HArray1<StepAP214ApprovalItem>>;
        item items_value = items: StepAP214ApprovalItem;
        len_checked nb_items = items;
    }
    /// `init`.
    pub fn init(
        &self,
        a_assigned_approval: &Handle<StepBasicApproval>,
        a_items: &Handle<HArray1<StepAP214ApprovalItem>>,
    ) {
        self.state.write().items = a_items.clone();
        self.parent.init(a_assigned_approval);
    }
}

step_entity! {
    /// Applied date and time assignment.
    StepAP214AppliedDateAndTimeAssignment, StepAP214AppliedDateAndTimeAssignmentState, "StepAP214AppliedDateAndTimeAssignment", StepBasicDateAndTimeAssignment, {
        items: Handle<HArray1<StepAP214DateAndTimeItem>>,
    }
}
impl StepAP214AppliedDateAndTimeAssignment {
    step_methods! {
        set set_items = items: Handle<HArray1<StepAP214DateAndTimeItem>>;
        get items = items: Handle<HArray1<StepAP214DateAndTimeItem>>;
        item items_value = items: StepAP214DateAndTimeItem;
        len_checked nb_items = items;
    }
    /// `init`.
    pub fn init(
        &self,
        a_assigned_date_and_time: &Handle<StepBasicDateAndTime>,
        a_role: &Handle<StepBasicDateTimeRole>,
        a_items: &Handle<HArray1<StepAP214DateAndTimeItem>>,
    ) {
        self.state.write().items = a_items.clone();
        self.parent.init(a_assigned_date_and_time, a_role);
    }
}

step_entity! {
    /// Applied date assignment.
    StepAP214AppliedDateAssignment, StepAP214AppliedDateAssignmentState, "StepAP214AppliedDateAssignment", StepBasicDateAssignment, {
        items: Handle<HArray1<StepAP214DateItem>>,
    }
}
impl StepAP214AppliedDateAssignment {
    step_methods! {
        set set_items = items: Handle<HArray1<StepAP214DateItem>>;
        get items = items: Handle<HArray1<StepAP214DateItem>>;
        item items_value = items: StepAP214DateItem;
        len_checked nb_items = items;
    }
    /// `init`.
    pub fn init(
        &self,
        a_assigned_date: &Handle<StepBasicDate>,
        a_role: &Handle<StepBasicDateRole>,
        a_items: &Handle<HArray1<StepAP214DateItem>>,
    ) {
        self.state.write().items = a_items.clone();
        self.parent.init(a_assigned_date, a_role);
    }
}
