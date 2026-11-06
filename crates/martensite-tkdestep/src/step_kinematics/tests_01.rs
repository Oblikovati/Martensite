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
fn runtime_type_step_kinematics_actuated_kin_pair_and_order_kin_pair() {
    let object = Handle::new(StepKinematicsActuatedKinPairAndOrderKinPair::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsActuatedKinPairAndOrderKinPair"
    );
    assert!(object.is_kind_name("StepKinematicsKinematicPair"));
}

#[test]
fn init_round_trip_step_kinematics_actuated_kin_pair_and_order_kin_pair() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_name = Handle::new(HAsciiString::new("sample"));
    let has_item_defined_transformation_description = true;
    let the_item_defined_transformation_description = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_transform_item1 =
        Handle::new(StepReprRepresentationItem::new());
    let the_item_defined_transformation_transform_item2 =
        Handle::new(StepReprRepresentationItem::new());
    let the_joint = Handle::new(StepKinematicsKinematicJoint::new());
    let the_actuated_kinematic_pair = Handle::new(StepKinematicsActuatedKinematicPair::new());
    let the_order_kinematic_pair = Handle::new(StepKinematicsKinematicPair::new());
    let object = StepKinematicsActuatedKinPairAndOrderKinPair::new();
    object.init(
        &the_representation_item_name,
        &the_item_defined_transformation_name,
        has_item_defined_transformation_description,
        &the_item_defined_transformation_description,
        &the_item_defined_transformation_transform_item1,
        &the_item_defined_transformation_transform_item2,
        &the_joint,
        &the_actuated_kinematic_pair,
        &the_order_kinematic_pair,
    );
}

#[test]
fn runtime_type_step_kinematics_actuated_kinematic_pair() {
    let object = Handle::new(StepKinematicsActuatedKinematicPair::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsActuatedKinematicPair"
    );
    assert!(object.is_kind_name("StepKinematicsKinematicPair"));
}

#[test]
fn init_round_trip_step_kinematics_actuated_kinematic_pair() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_name = Handle::new(HAsciiString::new("sample"));
    let has_item_defined_transformation_description = true;
    let the_item_defined_transformation_description = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_transform_item1 =
        Handle::new(StepReprRepresentationItem::new());
    let the_item_defined_transformation_transform_item2 =
        Handle::new(StepReprRepresentationItem::new());
    let the_kinematic_pair_joint = Handle::new(StepKinematicsKinematicJoint::new());
    let has_tx = true;
    let the_tx = StepKinematicsActuatedDirection::default();
    let has_ty = true;
    let the_ty = StepKinematicsActuatedDirection::default();
    let has_tz = true;
    let the_tz = StepKinematicsActuatedDirection::default();
    let has_rx = true;
    let the_rx = StepKinematicsActuatedDirection::default();
    let has_ry = true;
    let the_ry = StepKinematicsActuatedDirection::default();
    let has_rz = true;
    let the_rz = StepKinematicsActuatedDirection::default();
    let object = StepKinematicsActuatedKinematicPair::new();
    object.init(
        &the_representation_item_name,
        &the_item_defined_transformation_name,
        has_item_defined_transformation_description,
        &the_item_defined_transformation_description,
        &the_item_defined_transformation_transform_item1,
        &the_item_defined_transformation_transform_item2,
        &the_kinematic_pair_joint,
        has_tx,
        the_tx,
        has_ty,
        the_ty,
        has_tz,
        the_tz,
        has_rx,
        the_rx,
        has_ry,
        the_ry,
        has_rz,
        the_rz,
    );
    assert!(object.has_tx() == has_tx);
    assert!(object.tx() == the_tx);
    assert!(object.has_ty() == has_ty);
    assert!(object.ty() == the_ty);
    assert!(object.has_tz() == has_tz);
    assert!(object.tz() == the_tz);
    assert!(object.has_rx() == has_rx);
    assert!(object.rx() == the_rx);
    assert!(object.has_ry() == has_ry);
    assert!(object.ry() == the_ry);
    assert!(object.has_rz() == has_rz);
    assert!(object.rz() == the_rz);
}

#[test]
fn runtime_type_step_kinematics_context_dependent_kinematic_link_representation() {
    let object = Handle::new(StepKinematicsContextDependentKinematicLinkRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsContextDependentKinematicLinkRepresentation"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_kinematics_context_dependent_kinematic_link_representation() {
    let the_representation_relation =
        Handle::new(StepKinematicsKinematicLinkRepresentationAssociation::new());
    let the_represented_product_relation =
        Handle::new(StepKinematicsProductDefinitionRelationshipKinematics::new());
    let object = StepKinematicsContextDependentKinematicLinkRepresentation::new();
    object.init(
        &the_representation_relation,
        &the_represented_product_relation,
    );
    assert!(object.representation_relation() == the_representation_relation);
    assert!(object.represented_product_relation() == the_represented_product_relation);
}

#[test]
fn runtime_type_step_kinematics_cylindrical_pair() {
    let object = Handle::new(StepKinematicsCylindricalPair::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsCylindricalPair"
    );
    assert!(object.is_kind_name("StepKinematicsLowOrderKinematicPair"));
}

#[test]
fn runtime_type_step_kinematics_cylindrical_pair_value() {
    let object = Handle::new(StepKinematicsCylindricalPairValue::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsCylindricalPairValue"
    );
    assert!(object.is_kind_name("StepKinematicsPairValue"));
}

#[test]
fn init_round_trip_step_kinematics_cylindrical_pair_value() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_pair_value_applies_to_pair = Handle::new(StepKinematicsKinematicPair::new());
    let the_actual_translation = 2.5;
    let the_actual_rotation = 2.5;
    let object = StepKinematicsCylindricalPairValue::new();
    object.init(
        &the_representation_item_name,
        &the_pair_value_applies_to_pair,
        the_actual_translation,
        the_actual_rotation,
    );
    assert!(object.actual_translation() == the_actual_translation);
    assert!(object.actual_rotation() == the_actual_rotation);
}

#[test]
fn runtime_type_step_kinematics_cylindrical_pair_with_range() {
    let object = Handle::new(StepKinematicsCylindricalPairWithRange::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsCylindricalPairWithRange"
    );
    assert!(object.is_kind_name("StepKinematicsCylindricalPair"));
}

#[test]
fn init_round_trip_step_kinematics_cylindrical_pair_with_range() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_name = Handle::new(HAsciiString::new("sample"));
    let has_item_defined_transformation_description = true;
    let the_item_defined_transformation_description = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_transform_item1 =
        Handle::new(StepReprRepresentationItem::new());
    let the_item_defined_transformation_transform_item2 =
        Handle::new(StepReprRepresentationItem::new());
    let the_kinematic_pair_joint = Handle::new(StepKinematicsKinematicJoint::new());
    let the_low_order_kinematic_pair_tx = true;
    let the_low_order_kinematic_pair_ty = true;
    let the_low_order_kinematic_pair_tz = true;
    let the_low_order_kinematic_pair_rx = true;
    let the_low_order_kinematic_pair_ry = true;
    let the_low_order_kinematic_pair_rz = true;
    let has_lower_limit_actual_translation = true;
    let the_lower_limit_actual_translation = 2.5;
    let has_upper_limit_actual_translation = true;
    let the_upper_limit_actual_translation = 2.5;
    let has_lower_limit_actual_rotation = true;
    let the_lower_limit_actual_rotation = 2.5;
    let has_upper_limit_actual_rotation = true;
    let the_upper_limit_actual_rotation = 2.5;
    let object = StepKinematicsCylindricalPairWithRange::new();
    object.init(
        &the_representation_item_name,
        &the_item_defined_transformation_name,
        has_item_defined_transformation_description,
        &the_item_defined_transformation_description,
        &the_item_defined_transformation_transform_item1,
        &the_item_defined_transformation_transform_item2,
        &the_kinematic_pair_joint,
        the_low_order_kinematic_pair_tx,
        the_low_order_kinematic_pair_ty,
        the_low_order_kinematic_pair_tz,
        the_low_order_kinematic_pair_rx,
        the_low_order_kinematic_pair_ry,
        the_low_order_kinematic_pair_rz,
        has_lower_limit_actual_translation,
        the_lower_limit_actual_translation,
        has_upper_limit_actual_translation,
        the_upper_limit_actual_translation,
        has_lower_limit_actual_rotation,
        the_lower_limit_actual_rotation,
        has_upper_limit_actual_rotation,
        the_upper_limit_actual_rotation,
    );
    assert!(object.has_lower_limit_actual_translation() == has_lower_limit_actual_translation);
    assert!(object.lower_limit_actual_translation() == the_lower_limit_actual_translation);
    assert!(object.has_upper_limit_actual_translation() == has_upper_limit_actual_translation);
    assert!(object.upper_limit_actual_translation() == the_upper_limit_actual_translation);
    assert!(object.has_lower_limit_actual_rotation() == has_lower_limit_actual_rotation);
    assert!(object.lower_limit_actual_rotation() == the_lower_limit_actual_rotation);
    assert!(object.has_upper_limit_actual_rotation() == has_upper_limit_actual_rotation);
    assert!(object.upper_limit_actual_rotation() == the_upper_limit_actual_rotation);
}

#[test]
fn runtime_type_step_kinematics_fully_constrained_pair() {
    let object = Handle::new(StepKinematicsFullyConstrainedPair::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsFullyConstrainedPair"
    );
    assert!(object.is_kind_name("StepKinematicsLowOrderKinematicPair"));
}

#[test]
fn runtime_type_step_kinematics_gear_pair() {
    let object = Handle::new(StepKinematicsGearPair::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsGearPair"
    );
    assert!(object.is_kind_name("StepKinematicsLowOrderKinematicPairWithMotionCoupling"));
}

#[test]
fn init_round_trip_step_kinematics_gear_pair() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_name = Handle::new(HAsciiString::new("sample"));
    let has_item_defined_transformation_description = true;
    let the_item_defined_transformation_description = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_transform_item1 =
        Handle::new(StepReprRepresentationItem::new());
    let the_item_defined_transformation_transform_item2 =
        Handle::new(StepReprRepresentationItem::new());
    let the_kinematic_pair_joint = Handle::new(StepKinematicsKinematicJoint::new());
    let the_radius_first_link = 2.5;
    let the_radius_second_link = 2.5;
    let the_bevel = 2.5;
    let the_helical_angle = 2.5;
    let the_gear_ratio = 2.5;
    let object = StepKinematicsGearPair::new();
    object.init(
        &the_representation_item_name,
        &the_item_defined_transformation_name,
        has_item_defined_transformation_description,
        &the_item_defined_transformation_description,
        &the_item_defined_transformation_transform_item1,
        &the_item_defined_transformation_transform_item2,
        &the_kinematic_pair_joint,
        the_radius_first_link,
        the_radius_second_link,
        the_bevel,
        the_helical_angle,
        the_gear_ratio,
    );
    assert!(object.radius_first_link() == the_radius_first_link);
    assert!(object.radius_second_link() == the_radius_second_link);
    assert!(object.bevel() == the_bevel);
    assert!(object.helical_angle() == the_helical_angle);
    assert!(object.gear_ratio() == the_gear_ratio);
}

#[test]
fn runtime_type_step_kinematics_gear_pair_value() {
    let object = Handle::new(StepKinematicsGearPairValue::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsGearPairValue"
    );
    assert!(object.is_kind_name("StepKinematicsPairValue"));
}

#[test]
fn init_round_trip_step_kinematics_gear_pair_value() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_pair_value_applies_to_pair = Handle::new(StepKinematicsKinematicPair::new());
    let the_actual_rotation1 = 2.5;
    let object = StepKinematicsGearPairValue::new();
    object.init(
        &the_representation_item_name,
        &the_pair_value_applies_to_pair,
        the_actual_rotation1,
    );
    assert!(object.actual_rotation1() == the_actual_rotation1);
}

#[test]
fn runtime_type_step_kinematics_gear_pair_with_range() {
    let object = Handle::new(StepKinematicsGearPairWithRange::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsGearPairWithRange"
    );
    assert!(object.is_kind_name("StepKinematicsGearPair"));
}

#[test]
fn init_round_trip_step_kinematics_gear_pair_with_range() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_name = Handle::new(HAsciiString::new("sample"));
    let has_item_defined_transformation_description = true;
    let the_item_defined_transformation_description = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_transform_item1 =
        Handle::new(StepReprRepresentationItem::new());
    let the_item_defined_transformation_transform_item2 =
        Handle::new(StepReprRepresentationItem::new());
    let the_kinematic_pair_joint = Handle::new(StepKinematicsKinematicJoint::new());
    let the_gear_pair_radius_first_link = 2.5;
    let the_gear_pair_radius_second_link = 2.5;
    let the_gear_pair_bevel = 2.5;
    let the_gear_pair_helical_angle = 2.5;
    let the_gear_pair_gear_ratio = 2.5;
    let has_lower_limit_actual_rotation1 = true;
    let the_lower_limit_actual_rotation1 = 2.5;
    let has_upper_limit_actual_rotation1 = true;
    let the_upper_limit_actual_rotation1 = 2.5;
    let object = StepKinematicsGearPairWithRange::new();
    object.init(
        &the_representation_item_name,
        &the_item_defined_transformation_name,
        has_item_defined_transformation_description,
        &the_item_defined_transformation_description,
        &the_item_defined_transformation_transform_item1,
        &the_item_defined_transformation_transform_item2,
        &the_kinematic_pair_joint,
        the_gear_pair_radius_first_link,
        the_gear_pair_radius_second_link,
        the_gear_pair_bevel,
        the_gear_pair_helical_angle,
        the_gear_pair_gear_ratio,
        has_lower_limit_actual_rotation1,
        the_lower_limit_actual_rotation1,
        has_upper_limit_actual_rotation1,
        the_upper_limit_actual_rotation1,
    );
    assert!(object.has_lower_limit_actual_rotation1() == has_lower_limit_actual_rotation1);
    assert!(object.lower_limit_actual_rotation1() == the_lower_limit_actual_rotation1);
    assert!(object.has_upper_limit_actual_rotation1() == has_upper_limit_actual_rotation1);
    assert!(object.upper_limit_actual_rotation1() == the_upper_limit_actual_rotation1);
}

#[test]
fn runtime_type_step_kinematics_high_order_kinematic_pair() {
    let object = Handle::new(StepKinematicsHighOrderKinematicPair::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsHighOrderKinematicPair"
    );
    assert!(object.is_kind_name("StepKinematicsKinematicPair"));
}

#[test]
fn runtime_type_step_kinematics_homokinetic_pair() {
    let object = Handle::new(StepKinematicsHomokineticPair::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsHomokineticPair"
    );
    assert!(object.is_kind_name("StepKinematicsUniversalPair"));
}

#[test]
fn runtime_type_step_kinematics_kinematic_joint() {
    let object = Handle::new(StepKinematicsKinematicJoint::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsKinematicJoint"
    );
    assert!(object.is_kind_name("StepShapeEdge"));
}

#[test]
fn runtime_type_step_kinematics_kinematic_link() {
    let object = Handle::new(StepKinematicsKinematicLink::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsKinematicLink"
    );
    assert!(object.is_kind_name("StepShapeVertex"));
}

#[test]
fn runtime_type_step_kinematics_kinematic_link_representation() {
    let object = Handle::new(StepKinematicsKinematicLinkRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsKinematicLinkRepresentation"
    );
    assert!(object.is_kind_name("StepReprRepresentation"));
}

#[test]
fn init_round_trip_step_kinematics_kinematic_link_representation() {
    let the_representation_name = Handle::new(HAsciiString::new("sample"));
    let the_representation_items =
        Handle::new(HArray1::<Handle<StepReprRepresentationItem>>::with_bounds(1, 2).unwrap());
    let the_representation_context_of_items = Handle::new(StepReprRepresentationContext::new());
    let the_represented_link = Handle::new(StepKinematicsKinematicLink::new());
    let object = StepKinematicsKinematicLinkRepresentation::new();
    object.init(
        &the_representation_name,
        &the_representation_items,
        &the_representation_context_of_items,
        &the_represented_link,
    );
    assert!(object.represented_link() == the_represented_link);
}
