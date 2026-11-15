use super::*;
use martensite_tkmath::gp::{Axis1, Axis2, Circle3, Direction3, Point3};
use martensite_tkv3d::{
    select_basics::SelectBasicsPickResult,
    select_mgr::{SelectMgrEntityOwner, SelectMgrSelectingVolumeManager},
    select3d::{Select3DSensitiveCircle, Select3DSensitiveEntity, Select3DSensitiveWire},
};

fn circle_entity() -> Handle<dyn Select3DSensitiveEntity> {
    let owner = SelectMgrEntityOwner::new(0).cast();
    let circle = Circle3::new(Axis2::default(), 1.).unwrap();
    Handle::new(Select3DSensitiveCircle::new(&owner, &circle, true)).cast()
}

fn wire_entity() -> Handle<Select3DSensitiveWire> {
    let owner = SelectMgrEntityOwner::new(0).cast();
    let wire = Handle::new(Select3DSensitiveWire::new(&owner));
    wire.add(&circle_entity()).unwrap();
    wire
}

#[test]
fn plain_real_entity_emits_only_its_actual_root_type() {
    let circle = circle_entity();
    assert_eq!(
        entity_state_type_rows(&circle).unwrap(),
        ["Select3DSensitiveCircle"]
    );
    assert!(entity_state_type_rows(&Handle::default()).is_err());
}

#[test]
fn real_wire_match_emits_root_then_actual_detected_circle_type() {
    let wire = wire_entity();
    let mut manager = SelectMgrSelectingVolumeManager::new();
    manager.init_axis_selecting_volume(Axis1::new(Point3::new(0., 0., -2.), Direction3::Z));
    manager.build_selecting_volume().unwrap();
    let mut pick = SelectBasicsPickResult::default();
    assert!(wire.matches(&manager, &mut pick).unwrap());
    assert_eq!(wire.get_last_detected(), wire.get_edges()[0]);
    let types = entity_state_type_rows(&wire.cast()).unwrap();
    assert_eq!(types, ["Select3DSensitiveWire", "Select3DSensitiveCircle"]);
}

#[test]
fn unmatched_wire_cannot_invent_a_detected_child_descriptor() {
    let wire = wire_entity();
    assert!(wire.get_last_detected().is_null());
    let failure = entity_state_type_rows(&wire.cast()).unwrap_err();
    assert!(
        failure
            .message()
            .contains("expected actual last detected entity")
    );
}
