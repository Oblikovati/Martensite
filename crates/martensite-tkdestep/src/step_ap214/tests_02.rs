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
fn init_round_trip_step_ap214_auto_design_nominal_date_assignment() {
    let a_assigned_date = Handle::new(StepBasicDate::new());
    let a_role = Handle::new(StepBasicDateRole::new());
    let a_items = Handle::new(HArray1::<StepAP214AutoDesignDatedItem>::with_bounds(1, 2).unwrap());
    let object = StepAP214AutoDesignNominalDateAssignment::new();
    object.init(&a_assigned_date, &a_role, &a_items);
    assert!(object.items() == a_items);
}

#[test]
fn runtime_type_step_ap214_auto_design_organization_assignment() {
    let object = Handle::new(StepAP214AutoDesignOrganizationAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP214AutoDesignOrganizationAssignment"
    );
    assert!(object.is_kind_name("StepBasicOrganizationAssignment"));
}

#[test]
fn init_round_trip_step_ap214_auto_design_organization_assignment() {
    let a_assigned_organization = Handle::new(StepBasicOrganization::new());
    let a_role = Handle::new(StepBasicOrganizationRole::new());
    let a_items =
        Handle::new(HArray1::<StepAP214AutoDesignGeneralOrgItem>::with_bounds(1, 2).unwrap());
    let object = StepAP214AutoDesignOrganizationAssignment::new();
    object.init(&a_assigned_organization, &a_role, &a_items);
    assert!(object.items() == a_items);
}

#[test]
fn runtime_type_step_ap214_auto_design_person_and_organization_assignment() {
    let object = Handle::new(StepAP214AutoDesignPersonAndOrganizationAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP214AutoDesignPersonAndOrganizationAssignment"
    );
    assert!(object.is_kind_name("StepBasicPersonAndOrganizationAssignment"));
}

#[test]
fn init_round_trip_step_ap214_auto_design_person_and_organization_assignment() {
    let a_assigned_person_and_organization = Handle::new(StepBasicPersonAndOrganization::new());
    let a_role = Handle::new(StepBasicPersonAndOrganizationRole::new());
    let a_items =
        Handle::new(HArray1::<StepAP214AutoDesignGeneralOrgItem>::with_bounds(1, 2).unwrap());
    let object = StepAP214AutoDesignPersonAndOrganizationAssignment::new();
    object.init(&a_assigned_person_and_organization, &a_role, &a_items);
    assert!(object.items() == a_items);
}

#[test]
fn runtime_type_step_ap214_auto_design_presented_item() {
    let object = Handle::new(StepAP214AutoDesignPresentedItem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP214AutoDesignPresentedItem"
    );
    assert!(object.is_kind_name("StepVisualPresentedItem"));
}

#[test]
fn init_round_trip_step_ap214_auto_design_presented_item() {
    let a_items =
        Handle::new(HArray1::<StepAP214AutoDesignPresentedItemSelect>::with_bounds(1, 2).unwrap());
    let object = StepAP214AutoDesignPresentedItem::new();
    object.init(&a_items);
    assert!(object.items() == a_items);
}

#[test]
fn runtime_type_step_ap214_auto_design_security_classification_assignment() {
    let object = Handle::new(StepAP214AutoDesignSecurityClassificationAssignment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP214AutoDesignSecurityClassificationAssignment"
    );
    assert!(object.is_kind_name("StepBasicSecurityClassificationAssignment"));
}

#[test]
fn init_round_trip_step_ap214_auto_design_security_classification_assignment() {
    let a_assigned_security_classification = Handle::new(StepBasicSecurityClassification::new());
    let a_items = Handle::new(HArray1::<Handle<StepBasicApproval>>::with_bounds(1, 2).unwrap());
    let object = StepAP214AutoDesignSecurityClassificationAssignment::new();
    object.init(&a_assigned_security_classification, &a_items);
    assert!(object.items() == a_items);
}

#[test]
fn runtime_type_step_ap214_class() {
    let object = Handle::new(StepAP214Class::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepAP214Class");
    assert!(object.is_kind_name("StepBasicGroup"));
}

#[test]
fn runtime_type_step_ap214_externally_defined_class() {
    let object = Handle::new(StepAP214ExternallyDefinedClass::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP214ExternallyDefinedClass"
    );
    assert!(object.is_kind_name("StepAP214Class"));
}

#[test]
fn init_round_trip_step_ap214_externally_defined_class() {
    let a_group_name = Handle::new(HAsciiString::new("sample"));
    let has_group_description = true;
    let a_group_description = Handle::new(HAsciiString::new("sample"));
    let a_externally_defined_item_item_id = StepBasicSourceItem::default();
    let a_externally_defined_item_source = Handle::new(StepBasicExternalSource::new());
    let object = StepAP214ExternallyDefinedClass::new();
    object.init(
        &a_group_name,
        has_group_description,
        &a_group_description,
        &a_externally_defined_item_item_id,
        &a_externally_defined_item_source,
    );
}

#[test]
fn runtime_type_step_ap214_externally_defined_general_property() {
    let object = Handle::new(StepAP214ExternallyDefinedGeneralProperty::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP214ExternallyDefinedGeneralProperty"
    );
    assert!(object.is_kind_name("StepBasicGeneralProperty"));
}

#[test]
fn init_round_trip_step_ap214_externally_defined_general_property() {
    let a_general_property_id = Handle::new(HAsciiString::new("sample"));
    let a_general_property_name = Handle::new(HAsciiString::new("sample"));
    let has_general_property_description = true;
    let a_general_property_description = Handle::new(HAsciiString::new("sample"));
    let a_externally_defined_item_item_id = StepBasicSourceItem::default();
    let a_externally_defined_item_source = Handle::new(StepBasicExternalSource::new());
    let object = StepAP214ExternallyDefinedGeneralProperty::new();
    object.init(
        &a_general_property_id,
        &a_general_property_name,
        has_general_property_description,
        &a_general_property_description,
        &a_externally_defined_item_item_id,
        &a_externally_defined_item_source,
    );
}

#[test]
fn runtime_type_step_ap214_rep_item_group() {
    let object = Handle::new(StepAP214RepItemGroup::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP214RepItemGroup"
    );
    assert!(object.is_kind_name("StepBasicGroup"));
}

#[test]
fn init_round_trip_step_ap214_rep_item_group() {
    let a_group_name = Handle::new(HAsciiString::new("sample"));
    let has_group_description = true;
    let a_group_description = Handle::new(HAsciiString::new("sample"));
    let a_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let object = StepAP214RepItemGroup::new();
    object.init(
        &a_group_name,
        has_group_description,
        &a_group_description,
        &a_representation_item_name,
    );
}
