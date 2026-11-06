#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;
#[allow(unused_imports)]
use crate::step_repr::*;

#[test]
fn runtime_type_step_ap242_draughting_model_item_association() {
    let object = Handle::new(StepAP242DraughtingModelItemAssociation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP242DraughtingModelItemAssociation"
    );
    assert!(object.is_kind_name("StepAP242ItemIdentifiedRepresentationUsage"));
}

#[test]
fn runtime_type_step_ap242_geometric_item_specific_usage() {
    let object = Handle::new(StepAP242GeometricItemSpecificUsage::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP242GeometricItemSpecificUsage"
    );
    assert!(object.is_kind_name("StepAP242ItemIdentifiedRepresentationUsage"));
}

#[test]
fn runtime_type_step_ap242_id_attribute() {
    let object = Handle::new(StepAP242IdAttribute::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP242IdAttribute"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_ap242_id_attribute() {
    let the_attribute_value = Handle::new(HAsciiString::new("sample"));
    let the_identified_item = StepAP242IdAttributeSelect::default();
    let object = StepAP242IdAttribute::new();
    object.init(&the_attribute_value, &the_identified_item);
    assert!(object.attribute_value() == the_attribute_value);
}

#[test]
fn runtime_type_step_ap242_item_identified_representation_usage() {
    let object = Handle::new(StepAP242ItemIdentifiedRepresentationUsage::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepAP242ItemIdentifiedRepresentationUsage"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_ap242_item_identified_representation_usage() {
    let the_name = Handle::new(HAsciiString::new("sample"));
    let the_description = Handle::new(HAsciiString::new("sample"));
    let the_definition = StepAP242ItemIdentifiedRepresentationUsageDefinition::default();
    let the_used_representation = Handle::new(StepReprRepresentation::new());
    let the_identified_item =
        Handle::new(HArray1::<Handle<StepReprRepresentationItem>>::with_bounds(1, 2).unwrap());
    let object = StepAP242ItemIdentifiedRepresentationUsage::new();
    object.init(
        &the_name,
        &the_description,
        &the_definition,
        &the_used_representation,
        &the_identified_item,
    );
    assert!(object.name() == the_name);
    assert!(object.description() == the_description);
    assert!(object.used_representation() == the_used_representation);
    assert!(object.identified_item() == the_identified_item);
}
