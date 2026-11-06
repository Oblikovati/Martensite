#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;
#[allow(unused_imports)]
use crate::step_geom::*;
#[allow(unused_imports)]
use crate::step_repr::*;

#[test]
fn init_round_trip_step_kinematics_mechanism_representation() {
    let the_representation_name = Handle::new(HAsciiString::new("sample"));
    let the_representation_items =
        Handle::new(HArray1::<Handle<StepReprRepresentationItem>>::with_bounds(1, 2).unwrap());
    let the_representation_context_of_items = Handle::new(StepReprRepresentationContext::new());
    let the_represented_topology = StepKinematicsKinematicTopologyRepresentationSelect::default();
    let object = StepKinematicsMechanismRepresentation::new();
    object.init(
        &the_representation_name,
        &the_representation_items,
        &the_representation_context_of_items,
        &the_represented_topology,
    );
}

#[test]
fn runtime_type_step_kinematics_mechanism_state_representation() {
    let object = Handle::new(StepKinematicsMechanismStateRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsMechanismStateRepresentation"
    );
    assert!(object.is_kind_name("StepReprRepresentation"));
}

#[test]
fn init_round_trip_step_kinematics_mechanism_state_representation() {
    let the_name = Handle::new(HAsciiString::new("sample"));
    let the_items =
        Handle::new(HArray1::<Handle<StepReprRepresentationItem>>::with_bounds(1, 2).unwrap());
    let the_context_of_items = Handle::new(StepReprRepresentationContext::new());
    let the_mechanism = Handle::new(StepKinematicsMechanismRepresentation::new());
    let object = StepKinematicsMechanismStateRepresentation::new();
    object.init(&the_name, &the_items, &the_context_of_items, &the_mechanism);
    assert!(object.mechanism() == the_mechanism);
}

#[test]
fn runtime_type_step_kinematics_oriented_joint() {
    let object = Handle::new(StepKinematicsOrientedJoint::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsOrientedJoint"
    );
    assert!(object.is_kind_name("StepShapeOrientedEdge"));
}

#[test]
fn runtime_type_step_kinematics_pair_representation_relationship() {
    let object = Handle::new(StepKinematicsPairRepresentationRelationship::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsPairRepresentationRelationship"
    );
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}

#[test]
fn runtime_type_step_kinematics_pair_value() {
    let object = Handle::new(StepKinematicsPairValue::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsPairValue"
    );
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}

#[test]
fn init_round_trip_step_kinematics_pair_value() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_applies_to_pair = Handle::new(StepKinematicsKinematicPair::new());
    let object = StepKinematicsPairValue::new();
    object.init(&the_representation_item_name, &the_applies_to_pair);
    assert!(object.applies_to_pair() == the_applies_to_pair);
}

#[test]
fn runtime_type_step_kinematics_planar_curve_pair() {
    let object = Handle::new(StepKinematicsPlanarCurvePair::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsPlanarCurvePair"
    );
    assert!(object.is_kind_name("StepKinematicsHighOrderKinematicPair"));
}

#[test]
fn init_round_trip_step_kinematics_planar_curve_pair() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_name = Handle::new(HAsciiString::new("sample"));
    let has_item_defined_transformation_description = true;
    let the_item_defined_transformation_description = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_transform_item1 =
        Handle::new(StepReprRepresentationItem::new());
    let the_item_defined_transformation_transform_item2 =
        Handle::new(StepReprRepresentationItem::new());
    let the_kinematic_pair_joint = Handle::new(StepKinematicsKinematicJoint::new());
    let the_curve1 = Handle::new(StepGeomCurve::new());
    let the_curve2 = Handle::new(StepGeomCurve::new());
    let the_orientation = true;
    let object = StepKinematicsPlanarCurvePair::new();
    object.init(
        &the_representation_item_name,
        &the_item_defined_transformation_name,
        has_item_defined_transformation_description,
        &the_item_defined_transformation_description,
        &the_item_defined_transformation_transform_item1,
        &the_item_defined_transformation_transform_item2,
        &the_kinematic_pair_joint,
        &the_curve1,
        &the_curve2,
        the_orientation,
    );
    assert!(object.curve1() == the_curve1);
    assert!(object.curve2() == the_curve2);
    assert!(object.orientation() == the_orientation);
}

#[test]
fn runtime_type_step_kinematics_planar_curve_pair_range() {
    let object = Handle::new(StepKinematicsPlanarCurvePairRange::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsPlanarCurvePairRange"
    );
    assert!(object.is_kind_name("StepKinematicsPlanarCurvePair"));
}

#[test]
fn init_round_trip_step_kinematics_planar_curve_pair_range() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_name = Handle::new(HAsciiString::new("sample"));
    let has_item_defined_transformation_description = true;
    let the_item_defined_transformation_description = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_transform_item1 =
        Handle::new(StepReprRepresentationItem::new());
    let the_item_defined_transformation_transform_item2 =
        Handle::new(StepReprRepresentationItem::new());
    let the_kinematic_pair_joint = Handle::new(StepKinematicsKinematicJoint::new());
    let the_planar_curve_pair_curve1 = Handle::new(StepGeomCurve::new());
    let the_planar_curve_pair_curve2 = Handle::new(StepGeomCurve::new());
    let the_planar_curve_pair_orientation = true;
    let the_range_on_curve1 = Handle::new(StepGeomTrimmedCurve::new());
    let the_range_on_curve2 = Handle::new(StepGeomTrimmedCurve::new());
    let object = StepKinematicsPlanarCurvePairRange::new();
    object.init(
        &the_representation_item_name,
        &the_item_defined_transformation_name,
        has_item_defined_transformation_description,
        &the_item_defined_transformation_description,
        &the_item_defined_transformation_transform_item1,
        &the_item_defined_transformation_transform_item2,
        &the_kinematic_pair_joint,
        &the_planar_curve_pair_curve1,
        &the_planar_curve_pair_curve2,
        the_planar_curve_pair_orientation,
        &the_range_on_curve1,
        &the_range_on_curve2,
    );
    assert!(object.range_on_curve1() == the_range_on_curve1);
    assert!(object.range_on_curve2() == the_range_on_curve2);
}

#[test]
fn runtime_type_step_kinematics_planar_pair() {
    let object = Handle::new(StepKinematicsPlanarPair::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsPlanarPair"
    );
    assert!(object.is_kind_name("StepKinematicsLowOrderKinematicPair"));
}

#[test]
fn runtime_type_step_kinematics_planar_pair_value() {
    let object = Handle::new(StepKinematicsPlanarPairValue::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsPlanarPairValue"
    );
    assert!(object.is_kind_name("StepKinematicsPairValue"));
}

#[test]
fn init_round_trip_step_kinematics_planar_pair_value() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_pair_value_applies_to_pair = Handle::new(StepKinematicsKinematicPair::new());
    let the_actual_rotation = 2.5;
    let the_actual_translation_x = 2.5;
    let the_actual_translation_y = 2.5;
    let object = StepKinematicsPlanarPairValue::new();
    object.init(
        &the_representation_item_name,
        &the_pair_value_applies_to_pair,
        the_actual_rotation,
        the_actual_translation_x,
        the_actual_translation_y,
    );
    assert!(object.actual_rotation() == the_actual_rotation);
    assert!(object.actual_translation_x() == the_actual_translation_x);
    assert!(object.actual_translation_y() == the_actual_translation_y);
}

#[test]
fn runtime_type_step_kinematics_planar_pair_with_range() {
    let object = Handle::new(StepKinematicsPlanarPairWithRange::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsPlanarPairWithRange"
    );
    assert!(object.is_kind_name("StepKinematicsPlanarPair"));
}

#[test]
fn init_round_trip_step_kinematics_planar_pair_with_range() {
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
    let has_lower_limit_actual_rotation = true;
    let the_lower_limit_actual_rotation = 2.5;
    let has_upper_limit_actual_rotation = true;
    let the_upper_limit_actual_rotation = 2.5;
    let has_lower_limit_actual_translation_x = true;
    let the_lower_limit_actual_translation_x = 2.5;
    let has_upper_limit_actual_translation_x = true;
    let the_upper_limit_actual_translation_x = 2.5;
    let has_lower_limit_actual_translation_y = true;
    let the_lower_limit_actual_translation_y = 2.5;
    let has_upper_limit_actual_translation_y = true;
    let the_upper_limit_actual_translation_y = 2.5;
    let object = StepKinematicsPlanarPairWithRange::new();
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
        has_lower_limit_actual_rotation,
        the_lower_limit_actual_rotation,
        has_upper_limit_actual_rotation,
        the_upper_limit_actual_rotation,
        has_lower_limit_actual_translation_x,
        the_lower_limit_actual_translation_x,
        has_upper_limit_actual_translation_x,
        the_upper_limit_actual_translation_x,
        has_lower_limit_actual_translation_y,
        the_lower_limit_actual_translation_y,
        has_upper_limit_actual_translation_y,
        the_upper_limit_actual_translation_y,
    );
    assert!(object.has_lower_limit_actual_rotation() == has_lower_limit_actual_rotation);
    assert!(object.lower_limit_actual_rotation() == the_lower_limit_actual_rotation);
    assert!(object.has_upper_limit_actual_rotation() == has_upper_limit_actual_rotation);
    assert!(object.upper_limit_actual_rotation() == the_upper_limit_actual_rotation);
    assert!(object.has_lower_limit_actual_translation_x() == has_lower_limit_actual_translation_x);
    assert!(object.lower_limit_actual_translation_x() == the_lower_limit_actual_translation_x);
    assert!(object.has_upper_limit_actual_translation_x() == has_upper_limit_actual_translation_x);
    assert!(object.upper_limit_actual_translation_x() == the_upper_limit_actual_translation_x);
    assert!(object.has_lower_limit_actual_translation_y() == has_lower_limit_actual_translation_y);
    assert!(object.lower_limit_actual_translation_y() == the_lower_limit_actual_translation_y);
    assert!(object.has_upper_limit_actual_translation_y() == has_upper_limit_actual_translation_y);
    assert!(object.upper_limit_actual_translation_y() == the_upper_limit_actual_translation_y);
}

#[test]
fn runtime_type_step_kinematics_point_on_planar_curve_pair() {
    let object = Handle::new(StepKinematicsPointOnPlanarCurvePair::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsPointOnPlanarCurvePair"
    );
    assert!(object.is_kind_name("StepKinematicsHighOrderKinematicPair"));
}

#[test]
fn init_round_trip_step_kinematics_point_on_planar_curve_pair() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_name = Handle::new(HAsciiString::new("sample"));
    let has_item_defined_transformation_description = true;
    let the_item_defined_transformation_description = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_transform_item1 =
        Handle::new(StepReprRepresentationItem::new());
    let the_item_defined_transformation_transform_item2 =
        Handle::new(StepReprRepresentationItem::new());
    let the_kinematic_pair_joint = Handle::new(StepKinematicsKinematicJoint::new());
    let the_pair_curve = Handle::new(StepGeomCurve::new());
    let the_orientation = true;
    let object = StepKinematicsPointOnPlanarCurvePair::new();
    object.init(
        &the_representation_item_name,
        &the_item_defined_transformation_name,
        has_item_defined_transformation_description,
        &the_item_defined_transformation_description,
        &the_item_defined_transformation_transform_item1,
        &the_item_defined_transformation_transform_item2,
        &the_kinematic_pair_joint,
        &the_pair_curve,
        the_orientation,
    );
    assert!(object.pair_curve() == the_pair_curve);
    assert!(object.orientation() == the_orientation);
}

#[test]
fn runtime_type_step_kinematics_point_on_planar_curve_pair_value() {
    let object = Handle::new(StepKinematicsPointOnPlanarCurvePairValue::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsPointOnPlanarCurvePairValue"
    );
    assert!(object.is_kind_name("StepKinematicsPairValue"));
}

#[test]
fn init_round_trip_step_kinematics_point_on_planar_curve_pair_value() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_pair_value_applies_to_pair = Handle::new(StepKinematicsKinematicPair::new());
    let the_actual_point_on_curve = Handle::new(StepGeomPointOnCurve::new());
    let the_input_orientation = StepKinematicsSpatialRotation::default();
    let object = StepKinematicsPointOnPlanarCurvePairValue::new();
    object.init(
        &the_representation_item_name,
        &the_pair_value_applies_to_pair,
        &the_actual_point_on_curve,
        &the_input_orientation,
    );
    assert!(object.actual_point_on_curve() == the_actual_point_on_curve);
}

#[test]
fn runtime_type_step_kinematics_point_on_planar_curve_pair_with_range() {
    let object = Handle::new(StepKinematicsPointOnPlanarCurvePairWithRange::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsPointOnPlanarCurvePairWithRange"
    );
    assert!(object.is_kind_name("StepKinematicsPointOnPlanarCurvePair"));
}

#[test]
fn init_round_trip_step_kinematics_point_on_planar_curve_pair_with_range() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_name = Handle::new(HAsciiString::new("sample"));
    let has_item_defined_transformation_description = true;
    let the_item_defined_transformation_description = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_transform_item1 =
        Handle::new(StepReprRepresentationItem::new());
    let the_item_defined_transformation_transform_item2 =
        Handle::new(StepReprRepresentationItem::new());
    let the_kinematic_pair_joint = Handle::new(StepKinematicsKinematicJoint::new());
    let the_point_on_planar_curve_pair_pair_curve = Handle::new(StepGeomCurve::new());
    let the_point_on_planar_curve_pair_orientation = true;
    let the_range_on_pair_curve = Handle::new(StepGeomTrimmedCurve::new());
    let has_lower_limit_yaw = true;
    let the_lower_limit_yaw = 2.5;
    let has_upper_limit_yaw = true;
    let the_upper_limit_yaw = 2.5;
    let has_lower_limit_pitch = true;
    let the_lower_limit_pitch = 2.5;
    let has_upper_limit_pitch = true;
    let the_upper_limit_pitch = 2.5;
    let has_lower_limit_roll = true;
    let the_lower_limit_roll = 2.5;
    let has_upper_limit_roll = true;
    let the_upper_limit_roll = 2.5;
    let object = StepKinematicsPointOnPlanarCurvePairWithRange::new();
    object.init(
        &the_representation_item_name,
        &the_item_defined_transformation_name,
        has_item_defined_transformation_description,
        &the_item_defined_transformation_description,
        &the_item_defined_transformation_transform_item1,
        &the_item_defined_transformation_transform_item2,
        &the_kinematic_pair_joint,
        &the_point_on_planar_curve_pair_pair_curve,
        the_point_on_planar_curve_pair_orientation,
        &the_range_on_pair_curve,
        has_lower_limit_yaw,
        the_lower_limit_yaw,
        has_upper_limit_yaw,
        the_upper_limit_yaw,
        has_lower_limit_pitch,
        the_lower_limit_pitch,
        has_upper_limit_pitch,
        the_upper_limit_pitch,
        has_lower_limit_roll,
        the_lower_limit_roll,
        has_upper_limit_roll,
        the_upper_limit_roll,
    );
    assert!(object.range_on_pair_curve() == the_range_on_pair_curve);
    assert!(object.has_lower_limit_yaw() == has_lower_limit_yaw);
    assert!(object.lower_limit_yaw() == the_lower_limit_yaw);
    assert!(object.has_upper_limit_yaw() == has_upper_limit_yaw);
    assert!(object.upper_limit_yaw() == the_upper_limit_yaw);
    assert!(object.has_lower_limit_pitch() == has_lower_limit_pitch);
    assert!(object.lower_limit_pitch() == the_lower_limit_pitch);
    assert!(object.has_upper_limit_pitch() == has_upper_limit_pitch);
    assert!(object.upper_limit_pitch() == the_upper_limit_pitch);
    assert!(object.has_lower_limit_roll() == has_lower_limit_roll);
    assert!(object.lower_limit_roll() == the_lower_limit_roll);
    assert!(object.has_upper_limit_roll() == has_upper_limit_roll);
    assert!(object.upper_limit_roll() == the_upper_limit_roll);
}
