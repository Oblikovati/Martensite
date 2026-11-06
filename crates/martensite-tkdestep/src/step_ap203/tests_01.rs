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

#[test]
fn runtime_type_step_ap203_cc_design_approval() {
    let object = Handle::new(StepAP203CcDesignApproval::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP203CcDesignApproval"
    );
    assert!(object.is_kind_name("StepBasicApprovalAssignment"));
}

#[test]
fn init_round_trip_step_ap203_cc_design_approval() {
    let a_approval_assignment_assigned_approval = Handle::new(StepBasicApproval::new());
    let a_items = Handle::new(HArray1::<StepAP203ApprovedItem>::with_bounds(1, 2).unwrap());
    let object = StepAP203CcDesignApproval::new();
    object.init(&a_approval_assignment_assigned_approval, &a_items);
    assert!(object.items() == a_items);
}

#[test]
fn runtime_type_step_ap203_cc_design_certification() {
    let object = Handle::new(StepAP203CcDesignCertification::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP203CcDesignCertification"
    );
    assert!(object.is_kind_name("StepBasicCertificationAssignment"));
}

#[test]
fn init_round_trip_step_ap203_cc_design_certification() {
    let a_certification_assignment_assigned_certification =
        Handle::new(StepBasicCertification::new());
    let a_items = Handle::new(HArray1::<StepAP203CertifiedItem>::with_bounds(1, 2).unwrap());
    let object = StepAP203CcDesignCertification::new();
    object.init(&a_certification_assignment_assigned_certification, &a_items);
    assert!(object.items() == a_items);
}

#[test]
fn runtime_type_step_ap203_cc_design_contract() {
    let object = Handle::new(StepAP203CcDesignContract::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP203CcDesignContract"
    );
    assert!(object.is_kind_name("StepBasicContractAssignment"));
}

#[test]
fn init_round_trip_step_ap203_cc_design_contract() {
    let a_contract_assignment_assigned_contract = Handle::new(StepBasicContract::new());
    let a_items = Handle::new(HArray1::<StepAP203ContractedItem>::with_bounds(1, 2).unwrap());
    let object = StepAP203CcDesignContract::new();
    object.init(&a_contract_assignment_assigned_contract, &a_items);
    assert!(object.items() == a_items);
}

#[test]
fn runtime_type_step_ap203_cc_design_date_and_time_assignment() {
    let object = Handle::new(StepAP203CcDesignDateAndTimeAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP203CcDesignDateAndTimeAssignment"
    );
    assert!(object.is_kind_name("StepBasicDateAndTimeAssignment"));
}

#[test]
fn init_round_trip_step_ap203_cc_design_date_and_time_assignment() {
    let a_date_and_time_assignment_assigned_date_and_time =
        Handle::new(StepBasicDateAndTime::new());
    let a_date_and_time_assignment_role = Handle::new(StepBasicDateTimeRole::new());
    let a_items = Handle::new(HArray1::<StepAP203DateTimeItem>::with_bounds(1, 2).unwrap());
    let object = StepAP203CcDesignDateAndTimeAssignment::new();
    object.init(
        &a_date_and_time_assignment_assigned_date_and_time,
        &a_date_and_time_assignment_role,
        &a_items,
    );
    assert!(object.items() == a_items);
}

#[test]
fn runtime_type_step_ap203_cc_design_person_and_organization_assignment() {
    let object = Handle::new(StepAP203CcDesignPersonAndOrganizationAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP203CcDesignPersonAndOrganizationAssignment"
    );
    assert!(object.is_kind_name("StepBasicPersonAndOrganizationAssignment"));
}

#[test]
fn init_round_trip_step_ap203_cc_design_person_and_organization_assignment() {
    let a_person_and_organization_assignment_assigned_person_and_organization =
        Handle::new(StepBasicPersonAndOrganization::new());
    let a_person_and_organization_assignment_role =
        Handle::new(StepBasicPersonAndOrganizationRole::new());
    let a_items =
        Handle::new(HArray1::<StepAP203PersonOrganizationItem>::with_bounds(1, 2).unwrap());
    let object = StepAP203CcDesignPersonAndOrganizationAssignment::new();
    object.init(
        &a_person_and_organization_assignment_assigned_person_and_organization,
        &a_person_and_organization_assignment_role,
        &a_items,
    );
    assert!(object.items() == a_items);
}

#[test]
fn runtime_type_step_ap203_cc_design_security_classification() {
    let object = Handle::new(StepAP203CcDesignSecurityClassification::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP203CcDesignSecurityClassification"
    );
    assert!(object.is_kind_name("StepBasicSecurityClassificationAssignment"));
}

#[test]
fn init_round_trip_step_ap203_cc_design_security_classification() {
    let a_security_classification_assignment_assigned_security_classification =
        Handle::new(StepBasicSecurityClassification::new());
    let a_items = Handle::new(HArray1::<StepAP203ClassifiedItem>::with_bounds(1, 2).unwrap());
    let object = StepAP203CcDesignSecurityClassification::new();
    object.init(
        &a_security_classification_assignment_assigned_security_classification,
        &a_items,
    );
    assert!(object.items() == a_items);
}

#[test]
fn runtime_type_step_ap203_cc_design_specification_reference() {
    let object = Handle::new(StepAP203CcDesignSpecificationReference::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP203CcDesignSpecificationReference"
    );
    assert!(object.is_kind_name("StepBasicDocumentReference"));
}

#[test]
fn init_round_trip_step_ap203_cc_design_specification_reference() {
    let a_document_reference_assigned_document = Handle::new(StepBasicDocument::new());
    let a_document_reference_source = Handle::new(HAsciiString::new("sample"));
    let a_items = Handle::new(HArray1::<StepAP203SpecifiedItem>::with_bounds(1, 2).unwrap());
    let object = StepAP203CcDesignSpecificationReference::new();
    object.init(
        &a_document_reference_assigned_document,
        &a_document_reference_source,
        &a_items,
    );
    assert!(object.items() == a_items);
}

#[test]
fn runtime_type_step_ap203_change() {
    let object = Handle::new(StepAP203Change::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepAP203Change");
    assert!(object.is_kind_name("StepBasicActionAssignment"));
}

#[test]
fn init_round_trip_step_ap203_change() {
    let a_action_assignment_assigned_action = Handle::new(StepBasicAction::new());
    let a_items = Handle::new(HArray1::<StepAP203WorkItem>::with_bounds(1, 2).unwrap());
    let object = StepAP203Change::new();
    object.init(&a_action_assignment_assigned_action, &a_items);
    assert!(object.items() == a_items);
}

#[test]
fn runtime_type_step_ap203_change_request() {
    let object = Handle::new(StepAP203ChangeRequest::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP203ChangeRequest"
    );
    assert!(object.is_kind_name("StepBasicActionRequestAssignment"));
}

#[test]
fn init_round_trip_step_ap203_change_request() {
    let a_action_request_assignment_assigned_action_request =
        Handle::new(StepBasicVersionedActionRequest::new());
    let a_items = Handle::new(HArray1::<StepAP203ChangeRequestItem>::with_bounds(1, 2).unwrap());
    let object = StepAP203ChangeRequest::new();
    object.init(
        &a_action_request_assignment_assigned_action_request,
        &a_items,
    );
    assert!(object.items() == a_items);
}

#[test]
fn runtime_type_step_ap203_start_request() {
    let object = Handle::new(StepAP203StartRequest::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP203StartRequest"
    );
    assert!(object.is_kind_name("StepBasicActionRequestAssignment"));
}

#[test]
fn init_round_trip_step_ap203_start_request() {
    let a_action_request_assignment_assigned_action_request =
        Handle::new(StepBasicVersionedActionRequest::new());
    let a_items = Handle::new(HArray1::<StepAP203StartRequestItem>::with_bounds(1, 2).unwrap());
    let object = StepAP203StartRequest::new();
    object.init(
        &a_action_request_assignment_assigned_action_request,
        &a_items,
    );
    assert!(object.items() == a_items);
}

#[test]
fn runtime_type_step_ap203_start_work() {
    let object = Handle::new(StepAP203StartWork::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepAP203StartWork");
    assert!(object.is_kind_name("StepBasicActionAssignment"));
}

#[test]
fn init_round_trip_step_ap203_start_work() {
    let a_action_assignment_assigned_action = Handle::new(StepBasicAction::new());
    let a_items = Handle::new(HArray1::<StepAP203WorkItem>::with_bounds(1, 2).unwrap());
    let object = StepAP203StartWork::new();
    object.init(&a_action_assignment_assigned_action, &a_items);
    assert!(object.items() == a_items);
}
