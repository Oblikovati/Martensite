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
fn runtime_type_step_ap214_applied_approval_assignment() {
    let object = Handle::new(StepAP214AppliedApprovalAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP214AppliedApprovalAssignment"
    );
    assert!(object.is_kind_name("StepBasicApprovalAssignment"));
}

#[test]
fn init_round_trip_step_ap214_applied_approval_assignment() {
    let a_assigned_approval = Handle::new(StepBasicApproval::new());
    let a_items = Handle::new(HArray1::<StepAP214ApprovalItem>::with_bounds(1, 2).unwrap());
    let object = StepAP214AppliedApprovalAssignment::new();
    object.init(&a_assigned_approval, &a_items);
    assert!(object.items() == a_items);
}

#[test]
fn runtime_type_step_ap214_applied_date_and_time_assignment() {
    let object = Handle::new(StepAP214AppliedDateAndTimeAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP214AppliedDateAndTimeAssignment"
    );
    assert!(object.is_kind_name("StepBasicDateAndTimeAssignment"));
}

#[test]
fn init_round_trip_step_ap214_applied_date_and_time_assignment() {
    let a_assigned_date_and_time = Handle::new(StepBasicDateAndTime::new());
    let a_role = Handle::new(StepBasicDateTimeRole::new());
    let a_items = Handle::new(HArray1::<StepAP214DateAndTimeItem>::with_bounds(1, 2).unwrap());
    let object = StepAP214AppliedDateAndTimeAssignment::new();
    object.init(&a_assigned_date_and_time, &a_role, &a_items);
    assert!(object.items() == a_items);
}

#[test]
fn runtime_type_step_ap214_applied_date_assignment() {
    let object = Handle::new(StepAP214AppliedDateAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP214AppliedDateAssignment"
    );
    assert!(object.is_kind_name("StepBasicDateAssignment"));
}

#[test]
fn init_round_trip_step_ap214_applied_date_assignment() {
    let a_assigned_date = Handle::new(StepBasicDate::new());
    let a_role = Handle::new(StepBasicDateRole::new());
    let a_items = Handle::new(HArray1::<StepAP214DateItem>::with_bounds(1, 2).unwrap());
    let object = StepAP214AppliedDateAssignment::new();
    object.init(&a_assigned_date, &a_role, &a_items);
    assert!(object.items() == a_items);
}

#[test]
fn runtime_type_step_ap214_applied_document_reference() {
    let object = Handle::new(StepAP214AppliedDocumentReference::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP214AppliedDocumentReference"
    );
    assert!(object.is_kind_name("StepBasicDocumentReference"));
}

#[test]
fn init_round_trip_step_ap214_applied_document_reference() {
    let a_assigned_document = Handle::new(StepBasicDocument::new());
    let a_source = Handle::new(HAsciiString::new("sample"));
    let a_items =
        Handle::new(HArray1::<StepAP214DocumentReferenceItem>::with_bounds(1, 2).unwrap());
    let object = StepAP214AppliedDocumentReference::new();
    object.init(&a_assigned_document, &a_source, &a_items);
    assert!(object.items() == a_items);
}

#[test]
fn runtime_type_step_ap214_applied_external_identification_assignment() {
    let object = Handle::new(StepAP214AppliedExternalIdentificationAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP214AppliedExternalIdentificationAssignment"
    );
    assert!(object.is_kind_name("StepBasicExternalIdentificationAssignment"));
}

#[test]
fn init_round_trip_step_ap214_applied_external_identification_assignment() {
    let a_identification_assignment_assigned_id = Handle::new(HAsciiString::new("sample"));
    let a_identification_assignment_role = Handle::new(StepBasicIdentificationRole::new());
    let a_external_identification_assignment_source = Handle::new(StepBasicExternalSource::new());
    let a_items =
        Handle::new(HArray1::<StepAP214ExternalIdentificationItem>::with_bounds(1, 2).unwrap());
    let object = StepAP214AppliedExternalIdentificationAssignment::new();
    object.init(
        &a_identification_assignment_assigned_id,
        &a_identification_assignment_role,
        &a_external_identification_assignment_source,
        &a_items,
    );
    assert!(object.items() == a_items);
}

#[test]
fn runtime_type_step_ap214_applied_group_assignment() {
    let object = Handle::new(StepAP214AppliedGroupAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP214AppliedGroupAssignment"
    );
    assert!(object.is_kind_name("StepBasicGroupAssignment"));
}

#[test]
fn init_round_trip_step_ap214_applied_group_assignment() {
    let a_group_assignment_assigned_group = Handle::new(StepBasicGroup::new());
    let a_items = Handle::new(HArray1::<StepAP214GroupItem>::with_bounds(1, 2).unwrap());
    let object = StepAP214AppliedGroupAssignment::new();
    object.init(&a_group_assignment_assigned_group, &a_items);
    assert!(object.items() == a_items);
}

#[test]
fn runtime_type_step_ap214_applied_organization_assignment() {
    let object = Handle::new(StepAP214AppliedOrganizationAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP214AppliedOrganizationAssignment"
    );
    assert!(object.is_kind_name("StepBasicOrganizationAssignment"));
}

#[test]
fn init_round_trip_step_ap214_applied_organization_assignment() {
    let a_assigned_organization = Handle::new(StepBasicOrganization::new());
    let a_role = Handle::new(StepBasicOrganizationRole::new());
    let a_items = Handle::new(HArray1::<StepAP214OrganizationItem>::with_bounds(1, 2).unwrap());
    let object = StepAP214AppliedOrganizationAssignment::new();
    object.init(&a_assigned_organization, &a_role, &a_items);
    assert!(object.items() == a_items);
}

#[test]
fn runtime_type_step_ap214_applied_person_and_organization_assignment() {
    let object = Handle::new(StepAP214AppliedPersonAndOrganizationAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP214AppliedPersonAndOrganizationAssignment"
    );
    assert!(object.is_kind_name("StepBasicPersonAndOrganizationAssignment"));
}

#[test]
fn init_round_trip_step_ap214_applied_person_and_organization_assignment() {
    let a_assigned_person_and_organization = Handle::new(StepBasicPersonAndOrganization::new());
    let a_role = Handle::new(StepBasicPersonAndOrganizationRole::new());
    let a_items =
        Handle::new(HArray1::<StepAP214PersonAndOrganizationItem>::with_bounds(1, 2).unwrap());
    let object = StepAP214AppliedPersonAndOrganizationAssignment::new();
    object.init(&a_assigned_person_and_organization, &a_role, &a_items);
    assert!(object.items() == a_items);
}

#[test]
fn runtime_type_step_ap214_applied_presented_item() {
    let object = Handle::new(StepAP214AppliedPresentedItem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP214AppliedPresentedItem"
    );
    assert!(object.is_kind_name("StepVisualPresentedItem"));
}

#[test]
fn init_round_trip_step_ap214_applied_presented_item() {
    let a_items = Handle::new(HArray1::<StepAP214PresentedItemSelect>::with_bounds(1, 2).unwrap());
    let object = StepAP214AppliedPresentedItem::new();
    object.init(&a_items);
    assert!(object.items() == a_items);
}

#[test]
fn runtime_type_step_ap214_applied_security_classification_assignment() {
    let object = Handle::new(StepAP214AppliedSecurityClassificationAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP214AppliedSecurityClassificationAssignment"
    );
    assert!(object.is_kind_name("StepBasicSecurityClassificationAssignment"));
}

#[test]
fn init_round_trip_step_ap214_applied_security_classification_assignment() {
    let a_assigned_security_classification = Handle::new(StepBasicSecurityClassification::new());
    let a_items =
        Handle::new(HArray1::<StepAP214SecurityClassificationItem>::with_bounds(1, 2).unwrap());
    let object = StepAP214AppliedSecurityClassificationAssignment::new();
    object.init(&a_assigned_security_classification, &a_items);
    assert!(object.items() == a_items);
}

#[test]
fn runtime_type_step_ap214_auto_design_actual_date_and_time_assignment() {
    let object = Handle::new(StepAP214AutoDesignActualDateAndTimeAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP214AutoDesignActualDateAndTimeAssignment"
    );
    assert!(object.is_kind_name("StepBasicDateAndTimeAssignment"));
}

#[test]
fn init_round_trip_step_ap214_auto_design_actual_date_and_time_assignment() {
    let a_assigned_date_and_time = Handle::new(StepBasicDateAndTime::new());
    let a_role = Handle::new(StepBasicDateTimeRole::new());
    let a_items =
        Handle::new(HArray1::<StepAP214AutoDesignDateAndTimeItem>::with_bounds(1, 2).unwrap());
    let object = StepAP214AutoDesignActualDateAndTimeAssignment::new();
    object.init(&a_assigned_date_and_time, &a_role, &a_items);
    assert!(object.items() == a_items);
}

#[test]
fn runtime_type_step_ap214_auto_design_actual_date_assignment() {
    let object = Handle::new(StepAP214AutoDesignActualDateAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP214AutoDesignActualDateAssignment"
    );
    assert!(object.is_kind_name("StepBasicDateAssignment"));
}

#[test]
fn init_round_trip_step_ap214_auto_design_actual_date_assignment() {
    let a_assigned_date = Handle::new(StepBasicDate::new());
    let a_role = Handle::new(StepBasicDateRole::new());
    let a_items = Handle::new(HArray1::<StepAP214AutoDesignDatedItem>::with_bounds(1, 2).unwrap());
    let object = StepAP214AutoDesignActualDateAssignment::new();
    object.init(&a_assigned_date, &a_role, &a_items);
    assert!(object.items() == a_items);
}

#[test]
fn runtime_type_step_ap214_auto_design_approval_assignment() {
    let object = Handle::new(StepAP214AutoDesignApprovalAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP214AutoDesignApprovalAssignment"
    );
    assert!(object.is_kind_name("StepBasicApprovalAssignment"));
}

#[test]
fn init_round_trip_step_ap214_auto_design_approval_assignment() {
    let a_assigned_approval = Handle::new(StepBasicApproval::new());
    let a_items =
        Handle::new(HArray1::<StepAP214AutoDesignGeneralOrgItem>::with_bounds(1, 2).unwrap());
    let object = StepAP214AutoDesignApprovalAssignment::new();
    object.init(&a_assigned_approval, &a_items);
    assert!(object.items() == a_items);
}

#[test]
fn runtime_type_step_ap214_auto_design_date_and_person_assignment() {
    let object = Handle::new(StepAP214AutoDesignDateAndPersonAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP214AutoDesignDateAndPersonAssignment"
    );
    assert!(object.is_kind_name("StepBasicPersonAndOrganizationAssignment"));
}

#[test]
fn init_round_trip_step_ap214_auto_design_date_and_person_assignment() {
    let a_assigned_person_and_organization = Handle::new(StepBasicPersonAndOrganization::new());
    let a_role = Handle::new(StepBasicPersonAndOrganizationRole::new());
    let a_items =
        Handle::new(HArray1::<StepAP214AutoDesignDateAndPersonItem>::with_bounds(1, 2).unwrap());
    let object = StepAP214AutoDesignDateAndPersonAssignment::new();
    object.init(&a_assigned_person_and_organization, &a_role, &a_items);
    assert!(object.items() == a_items);
}

#[test]
fn runtime_type_step_ap214_auto_design_document_reference() {
    let object = Handle::new(StepAP214AutoDesignDocumentReference::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP214AutoDesignDocumentReference"
    );
    assert!(object.is_kind_name("StepBasicDocumentReference"));
}

#[test]
fn init_round_trip_step_ap214_auto_design_document_reference() {
    let a_assigned_document = Handle::new(StepBasicDocument::new());
    let a_source = Handle::new(HAsciiString::new("sample"));
    let a_items =
        Handle::new(HArray1::<StepAP214AutoDesignReferencingItem>::with_bounds(1, 2).unwrap());
    let object = StepAP214AutoDesignDocumentReference::new();
    object.init(&a_assigned_document, &a_source, &a_items);
    assert!(object.items() == a_items);
}

#[test]
fn runtime_type_step_ap214_auto_design_group_assignment() {
    let object = Handle::new(StepAP214AutoDesignGroupAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP214AutoDesignGroupAssignment"
    );
    assert!(object.is_kind_name("StepBasicGroupAssignment"));
}

#[test]
fn init_round_trip_step_ap214_auto_design_group_assignment() {
    let a_assigned_group = Handle::new(StepBasicGroup::new());
    let a_items =
        Handle::new(HArray1::<StepAP214AutoDesignGroupedItem>::with_bounds(1, 2).unwrap());
    let object = StepAP214AutoDesignGroupAssignment::new();
    object.init(&a_assigned_group, &a_items);
    assert!(object.items() == a_items);
}

#[test]
fn runtime_type_step_ap214_auto_design_nominal_date_and_time_assignment() {
    let object = Handle::new(StepAP214AutoDesignNominalDateAndTimeAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP214AutoDesignNominalDateAndTimeAssignment"
    );
    assert!(object.is_kind_name("StepBasicDateAndTimeAssignment"));
}

#[test]
fn init_round_trip_step_ap214_auto_design_nominal_date_and_time_assignment() {
    let a_assigned_date_and_time = Handle::new(StepBasicDateAndTime::new());
    let a_role = Handle::new(StepBasicDateTimeRole::new());
    let a_items =
        Handle::new(HArray1::<StepAP214AutoDesignDateAndTimeItem>::with_bounds(1, 2).unwrap());
    let object = StepAP214AutoDesignNominalDateAndTimeAssignment::new();
    object.init(&a_assigned_date_and_time, &a_role, &a_items);
    assert!(object.items() == a_items);
}

#[test]
fn runtime_type_step_ap214_auto_design_nominal_date_assignment() {
    let object = Handle::new(StepAP214AutoDesignNominalDateAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP214AutoDesignNominalDateAssignment"
    );
    assert!(object.is_kind_name("StepBasicDateAssignment"));
}
