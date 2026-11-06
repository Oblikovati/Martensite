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
use crate::step_repr::*;

step_select! {
    /// Representation of STEP SELECT type ApprovedItem
    StepAP203ApprovedItem, [StepBasicProductDefinitionFormation, StepBasicProductDefinition, StepReprConfigurationEffectivity, StepReprConfigurationItem, StepBasicSecurityClassification, StepAP203ChangeRequest, StepAP203Change, StepAP203StartRequest, StepAP203StartWork, StepBasicCertification, StepBasicContract], {
        product_definition_formation: StepBasicProductDefinitionFormation,
        product_definition: StepBasicProductDefinition,
        configuration_effectivity: StepReprConfigurationEffectivity,
        configuration_item: StepReprConfigurationItem,
        security_classification: StepBasicSecurityClassification,
        change_request: StepAP203ChangeRequest,
        change: StepAP203Change,
        start_request: StepAP203StartRequest,
        start_work: StepAP203StartWork,
        certification: StepBasicCertification,
        contract: StepBasicContract,
    }
}

step_select! {
    /// Representation of STEP SELECT type CertifiedItem
    StepAP203CertifiedItem, [StepReprSuppliedPartRelationship], {
        supplied_part_relationship: StepReprSuppliedPartRelationship,
    }
}

step_select! {
    /// Representation of STEP SELECT type ChangeRequestItem
    StepAP203ChangeRequestItem, [StepBasicProductDefinitionFormation], {
        product_definition_formation: StepBasicProductDefinitionFormation,
    }
}

step_select! {
    /// Representation of STEP SELECT type ClassifiedItem
    StepAP203ClassifiedItem, [StepBasicProductDefinitionFormation, StepReprAssemblyComponentUsage], {
        product_definition_formation: StepBasicProductDefinitionFormation,
        assembly_component_usage: StepReprAssemblyComponentUsage,
    }
}

step_select! {
    /// Representation of STEP SELECT type ContractedItem
    StepAP203ContractedItem, [StepBasicProductDefinitionFormation], {
        product_definition_formation: StepBasicProductDefinitionFormation,
    }
}

step_select! {
    /// Representation of STEP SELECT type DateTimeItem
    StepAP203DateTimeItem, [StepBasicProductDefinition, StepAP203ChangeRequest, StepAP203StartRequest, StepAP203Change, StepAP203StartWork, StepBasicApprovalPersonOrganization, StepBasicContract, StepBasicSecurityClassification, StepBasicCertification], {
        product_definition: StepBasicProductDefinition,
        change_request: StepAP203ChangeRequest,
        start_request: StepAP203StartRequest,
        change: StepAP203Change,
        start_work: StepAP203StartWork,
        approval_person_organization: StepBasicApprovalPersonOrganization,
        contract: StepBasicContract,
        security_classification: StepBasicSecurityClassification,
        certification: StepBasicCertification,
    }
}

step_select! {
    /// Representation of STEP SELECT type PersonOrganizationItem
    StepAP203PersonOrganizationItem, [StepAP203Change, StepAP203StartWork, StepAP203ChangeRequest, StepAP203StartRequest, StepReprConfigurationItem, StepBasicProduct, StepBasicProductDefinitionFormation, StepBasicProductDefinition, StepBasicContract, StepBasicSecurityClassification], {
        change: StepAP203Change,
        start_work: StepAP203StartWork,
        change_request: StepAP203ChangeRequest,
        start_request: StepAP203StartRequest,
        configuration_item: StepReprConfigurationItem,
        product: StepBasicProduct,
        product_definition_formation: StepBasicProductDefinitionFormation,
        product_definition: StepBasicProductDefinition,
        contract: StepBasicContract,
        security_classification: StepBasicSecurityClassification,
    }
}

step_select! {
    /// Representation of STEP SELECT type SpecifiedItem
    StepAP203SpecifiedItem, [StepBasicProductDefinition, StepReprShapeAspect], {
        product_definition: StepBasicProductDefinition,
        shape_aspect: StepReprShapeAspect,
    }
}

step_select! {
    /// Representation of STEP SELECT type StartRequestItem
    StepAP203StartRequestItem, [StepBasicProductDefinitionFormation], {
        product_definition_formation: StepBasicProductDefinitionFormation,
    }
}

step_select! {
    /// Representation of STEP SELECT type WorkItem
    StepAP203WorkItem, [StepBasicProductDefinitionFormation], {
        product_definition_formation: StepBasicProductDefinitionFormation,
    }
}

step_entity! {
    /// Representation of STEP entity CcDesignApproval
    StepAP203CcDesignApproval, StepAP203CcDesignApprovalState, "StepAP203CcDesignApproval", StepBasicApprovalAssignment, {
        the_items: Handle<HArray1<StepAP203ApprovedItem>>,
    }
}
impl StepAP203CcDesignApproval {
    step_methods! {
        get items = the_items: Handle<HArray1<StepAP203ApprovedItem>>;
        set set_items = the_items: Handle<HArray1<StepAP203ApprovedItem>>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_approval_assignment_assigned_approval: &Handle<StepBasicApproval>,
        a_items: &Handle<HArray1<StepAP203ApprovedItem>>,
    ) {
        self.parent.init(a_approval_assignment_assigned_approval);
        self.state.write().the_items = a_items.clone();
    }
}

step_entity! {
    /// Representation of STEP entity CcDesignCertification
    StepAP203CcDesignCertification, StepAP203CcDesignCertificationState, "StepAP203CcDesignCertification", StepBasicCertificationAssignment, {
        the_items: Handle<HArray1<StepAP203CertifiedItem>>,
    }
}
impl StepAP203CcDesignCertification {
    step_methods! {
        get items = the_items: Handle<HArray1<StepAP203CertifiedItem>>;
        set set_items = the_items: Handle<HArray1<StepAP203CertifiedItem>>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_certification_assignment_assigned_certification: &Handle<StepBasicCertification>,
        a_items: &Handle<HArray1<StepAP203CertifiedItem>>,
    ) {
        self.parent
            .init(a_certification_assignment_assigned_certification);
        self.state.write().the_items = a_items.clone();
    }
}

step_entity! {
    /// Representation of STEP entity CcDesignContract
    StepAP203CcDesignContract, StepAP203CcDesignContractState, "StepAP203CcDesignContract", StepBasicContractAssignment, {
        the_items: Handle<HArray1<StepAP203ContractedItem>>,
    }
}
impl StepAP203CcDesignContract {
    step_methods! {
        get items = the_items: Handle<HArray1<StepAP203ContractedItem>>;
        set set_items = the_items: Handle<HArray1<StepAP203ContractedItem>>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_contract_assignment_assigned_contract: &Handle<StepBasicContract>,
        a_items: &Handle<HArray1<StepAP203ContractedItem>>,
    ) {
        self.parent.init(a_contract_assignment_assigned_contract);
        self.state.write().the_items = a_items.clone();
    }
}

step_entity! {
    /// Representation of STEP entity CcDesignDateAndTimeAssignment
    StepAP203CcDesignDateAndTimeAssignment, StepAP203CcDesignDateAndTimeAssignmentState, "StepAP203CcDesignDateAndTimeAssignment", StepBasicDateAndTimeAssignment, {
        the_items: Handle<HArray1<StepAP203DateTimeItem>>,
    }
}
impl StepAP203CcDesignDateAndTimeAssignment {
    step_methods! {
        get items = the_items: Handle<HArray1<StepAP203DateTimeItem>>;
        set set_items = the_items: Handle<HArray1<StepAP203DateTimeItem>>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_date_and_time_assignment_assigned_date_and_time: &Handle<StepBasicDateAndTime>,
        a_date_and_time_assignment_role: &Handle<StepBasicDateTimeRole>,
        a_items: &Handle<HArray1<StepAP203DateTimeItem>>,
    ) {
        self.parent.init(
            a_date_and_time_assignment_assigned_date_and_time,
            a_date_and_time_assignment_role,
        );
        self.state.write().the_items = a_items.clone();
    }
}

step_entity! {
    /// Representation of STEP entity CcDesignPersonAndOrganizationAssignment
    StepAP203CcDesignPersonAndOrganizationAssignment, StepAP203CcDesignPersonAndOrganizationAssignmentState, "StepAP203CcDesignPersonAndOrganizationAssignment", StepBasicPersonAndOrganizationAssignment, {
        the_items: Handle<HArray1<StepAP203PersonOrganizationItem>>,
    }
}
impl StepAP203CcDesignPersonAndOrganizationAssignment {
    step_methods! {
        get items = the_items: Handle<HArray1<StepAP203PersonOrganizationItem>>;
        set set_items = the_items: Handle<HArray1<StepAP203PersonOrganizationItem>>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_person_and_organization_assignment_assigned_person_and_organization: &Handle<
            StepBasicPersonAndOrganization,
        >,
        a_person_and_organization_assignment_role: &Handle<StepBasicPersonAndOrganizationRole>,
        a_items: &Handle<HArray1<StepAP203PersonOrganizationItem>>,
    ) {
        self.parent.init(
            a_person_and_organization_assignment_assigned_person_and_organization,
            a_person_and_organization_assignment_role,
        );
        self.state.write().the_items = a_items.clone();
    }
}

step_entity! {
    /// Representation of STEP entity CcDesignSecurityClassification
    StepAP203CcDesignSecurityClassification, StepAP203CcDesignSecurityClassificationState, "StepAP203CcDesignSecurityClassification", StepBasicSecurityClassificationAssignment, {
        the_items: Handle<HArray1<StepAP203ClassifiedItem>>,
    }
}
impl StepAP203CcDesignSecurityClassification {
    step_methods! {
        get items = the_items: Handle<HArray1<StepAP203ClassifiedItem>>;
        set set_items = the_items: Handle<HArray1<StepAP203ClassifiedItem>>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_security_classification_assignment_assigned_security_classification: &Handle<
            StepBasicSecurityClassification,
        >,
        a_items: &Handle<HArray1<StepAP203ClassifiedItem>>,
    ) {
        self.parent
            .init(a_security_classification_assignment_assigned_security_classification);
        self.state.write().the_items = a_items.clone();
    }
}

step_entity! {
    /// Representation of STEP entity CcDesignSpecificationReference
    StepAP203CcDesignSpecificationReference, StepAP203CcDesignSpecificationReferenceState, "StepAP203CcDesignSpecificationReference", StepBasicDocumentReference, {
        the_items: Handle<HArray1<StepAP203SpecifiedItem>>,
    }
}
impl StepAP203CcDesignSpecificationReference {
    step_methods! {
        get items = the_items: Handle<HArray1<StepAP203SpecifiedItem>>;
        set set_items = the_items: Handle<HArray1<StepAP203SpecifiedItem>>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_document_reference_assigned_document: &Handle<StepBasicDocument>,
        a_document_reference_source: &Handle<HAsciiString>,
        a_items: &Handle<HArray1<StepAP203SpecifiedItem>>,
    ) {
        self.parent.init0(
            a_document_reference_assigned_document,
            a_document_reference_source,
        );
        self.state.write().the_items = a_items.clone();
    }
}
