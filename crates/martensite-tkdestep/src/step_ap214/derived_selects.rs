#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_basic::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_members::step_derived_select;
#[allow(unused_imports)]
use crate::step_repr::*;
#[allow(unused_imports)]
use crate::step_shape::*;
#[allow(unused_imports)]
use crate::step_visual::*;

step_derived_select! {
    /// Date and time item.
    StepAP214DateAndTimeItem, StepAP214ApprovalItem, [StepBasicApprovalPersonOrganization, StepAP214AppliedPersonAndOrganizationAssignment, StepAP214AppliedOrganizationAssignment, StepReprAssemblyComponentUsageSubstitute, StepBasicDocumentFile, StepBasicEffectivity, StepReprMaterialDesignation, StepVisualMechanicalDesignGeometricPresentationRepresentation, StepVisualPresentationArea, StepBasicProduct, StepBasicProductDefinition, StepBasicProductDefinitionFormation, StepBasicProductDefinitionRelationship, StepReprPropertyDefinition, StepShapeShapeRepresentation, StepBasicSecurityClassification], {
        approval_person_organization: StepBasicApprovalPersonOrganization,
        applied_person_and_organization_assignment: StepAP214AppliedPersonAndOrganizationAssignment,
        applied_organization_assignment: StepAP214AppliedOrganizationAssignment,
    }
}

step_derived_select! {
    /// Date item.
    StepAP214DateItem, StepAP214ApprovalItem, [StepBasicApprovalPersonOrganization, StepAP214AppliedPersonAndOrganizationAssignment, StepAP214AppliedOrganizationAssignment, StepReprAssemblyComponentUsageSubstitute, StepBasicDocumentFile, StepBasicEffectivity, StepReprMaterialDesignation, StepVisualMechanicalDesignGeometricPresentationRepresentation, StepVisualPresentationArea, StepBasicProduct, StepBasicProductDefinition, StepBasicProductDefinitionFormation, StepBasicProductDefinitionRelationship, StepReprPropertyDefinition, StepShapeShapeRepresentation, StepAP214AppliedSecurityClassificationAssignment, StepBasicDocument], {
        approval_person_organization: StepBasicApprovalPersonOrganization,
        applied_person_and_organization_assignment: StepAP214AppliedPersonAndOrganizationAssignment,
        applied_organization_assignment: StepAP214AppliedOrganizationAssignment,
        applied_security_classification_assignment: StepAP214AppliedSecurityClassificationAssignment,
    }
}

step_derived_select! {
    /// Organization item.
    StepAP214OrganizationItem, StepAP214ApprovalItem, [StepAP214AppliedOrganizationAssignment, StepBasicApproval, StepReprAssemblyComponentUsageSubstitute, StepBasicDocumentFile, StepReprMaterialDesignation, StepVisualMechanicalDesignGeometricPresentationRepresentation, StepVisualPresentationArea, StepBasicProduct, StepBasicProductDefinition, StepBasicProductDefinitionFormation, StepBasicProductDefinitionRelationship, StepReprPropertyDefinition, StepShapeShapeRepresentation, StepBasicSecurityClassification, StepAP214AppliedSecurityClassificationAssignment], {
        applied_organization_assignment: StepAP214AppliedOrganizationAssignment,
        approval: StepBasicApproval,
        applied_security_classification_assignment: StepAP214AppliedSecurityClassificationAssignment,
    }
}

step_derived_select! {
    /// Person and organization item.
    StepAP214PersonAndOrganizationItem, StepAP214ApprovalItem, [StepAP214AppliedOrganizationAssignment, StepReprAssemblyComponentUsageSubstitute, StepBasicDocumentFile, StepReprMaterialDesignation, StepVisualMechanicalDesignGeometricPresentationRepresentation, StepVisualPresentationArea, StepBasicProduct, StepBasicProductDefinition, StepBasicProductDefinitionFormation, StepBasicProductDefinitionRelationship, StepReprPropertyDefinition, StepShapeShapeRepresentation, StepBasicSecurityClassification, StepAP214AppliedSecurityClassificationAssignment, StepBasicApproval], {
        applied_organization_assignment: StepAP214AppliedOrganizationAssignment,
        applied_security_classification_assignment: StepAP214AppliedSecurityClassificationAssignment,
        approval: StepBasicApproval,
    }
}

step_derived_select! {
    /// Security classification item.
    StepAP214SecurityClassificationItem, StepAP214ApprovalItem, [StepBasicAction, StepReprAssemblyComponentUsage, StepReprAssemblyComponentUsageSubstitute, StepReprConfigurationDesign, StepReprConfigurationEffectivity, StepBasicDocument, StepBasicDocumentFile, StepVisualDraughtingModel, StepBasicGeneralProperty, StepReprMakeFromUsageOption, StepReprMaterialDesignation, StepVisualMechanicalDesignGeometricPresentationRepresentation, StepVisualPresentationArea, StepBasicProduct, StepReprProductConcept, StepBasicProductDefinition, StepBasicProductDefinitionFormation, StepBasicProductDefinitionRelationship, StepReprProductDefinitionUsage, StepReprPropertyDefinition, StepShapeShapeRepresentation, StepBasicVersionedActionRequest], {
        action: StepBasicAction,
        assembly_component_usage: StepReprAssemblyComponentUsage,
        configuration_design: StepReprConfigurationDesign,
        configuration_effectivity: StepReprConfigurationEffectivity,
        draughting_model: StepVisualDraughtingModel,
        general_property: StepBasicGeneralProperty,
        make_from_usage_option: StepReprMakeFromUsageOption,
        product_concept: StepReprProductConcept,
        product_definition_usage: StepReprProductDefinitionUsage,
        versioned_action_request: StepBasicVersionedActionRequest,
    }
}
