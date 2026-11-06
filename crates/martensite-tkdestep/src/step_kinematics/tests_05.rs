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
fn init_round_trip_step_kinematics_revolute_pair_with_range() {
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
    let object = StepKinematicsRevolutePairWithRange::new();
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
    );
    assert!(object.has_lower_limit_actual_rotation() == has_lower_limit_actual_rotation);
    assert!(object.lower_limit_actual_rotation() == the_lower_limit_actual_rotation);
    assert!(object.has_upper_limit_actual_rotation() == has_upper_limit_actual_rotation);
    assert!(object.upper_limit_actual_rotation() == the_upper_limit_actual_rotation);
}

#[test]
fn runtime_type_step_kinematics_rigid_link_representation() {
    let object = Handle::new(StepKinematicsRigidLinkRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsRigidLinkRepresentation"
    );
    assert!(object.is_kind_name("StepKinematicsKinematicLinkRepresentation"));
}

#[test]
fn runtime_type_step_kinematics_rolling_curve_pair() {
    let object = Handle::new(StepKinematicsRollingCurvePair::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsRollingCurvePair"
    );
    assert!(object.is_kind_name("StepKinematicsPlanarCurvePair"));
}

#[test]
fn runtime_type_step_kinematics_rolling_curve_pair_value() {
    let object = Handle::new(StepKinematicsRollingCurvePairValue::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsRollingCurvePairValue"
    );
    assert!(object.is_kind_name("StepKinematicsPairValue"));
}

#[test]
fn init_round_trip_step_kinematics_rolling_curve_pair_value() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_pair_value_applies_to_pair = Handle::new(StepKinematicsKinematicPair::new());
    let the_actual_point_on_curve1 = Handle::new(StepGeomPointOnCurve::new());
    let object = StepKinematicsRollingCurvePairValue::new();
    object.init(
        &the_representation_item_name,
        &the_pair_value_applies_to_pair,
        &the_actual_point_on_curve1,
    );
    assert!(object.actual_point_on_curve1() == the_actual_point_on_curve1);
}

#[test]
fn runtime_type_step_kinematics_rolling_surface_pair() {
    let object = Handle::new(StepKinematicsRollingSurfacePair::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsRollingSurfacePair"
    );
    assert!(object.is_kind_name("StepKinematicsSurfacePair"));
}

#[test]
fn runtime_type_step_kinematics_rolling_surface_pair_value() {
    let object = Handle::new(StepKinematicsRollingSurfacePairValue::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsRollingSurfacePairValue"
    );
    assert!(object.is_kind_name("StepKinematicsPairValue"));
}

#[test]
fn init_round_trip_step_kinematics_rolling_surface_pair_value() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_pair_value_applies_to_pair = Handle::new(StepKinematicsKinematicPair::new());
    let the_actual_point_on_surface = Handle::new(StepGeomPointOnSurface::new());
    let the_actual_rotation = 2.5;
    let object = StepKinematicsRollingSurfacePairValue::new();
    object.init(
        &the_representation_item_name,
        &the_pair_value_applies_to_pair,
        &the_actual_point_on_surface,
        the_actual_rotation,
    );
    assert!(object.actual_point_on_surface() == the_actual_point_on_surface);
    assert!(object.actual_rotation() == the_actual_rotation);
}

#[test]
fn runtime_type_step_kinematics_rotation_about_direction() {
    let object = Handle::new(StepKinematicsRotationAboutDirection::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsRotationAboutDirection"
    );
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}

#[test]
fn init_round_trip_step_kinematics_rotation_about_direction() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_direction_of_axis = Handle::new(StepGeomDirection::new());
    let the_rotation_angle = 2.5;
    let object = StepKinematicsRotationAboutDirection::new();
    object.init(
        &the_representation_item_name,
        &the_direction_of_axis,
        the_rotation_angle,
    );
    assert!(object.direction_of_axis() == the_direction_of_axis);
    assert!(object.rotation_angle() == the_rotation_angle);
}

#[test]
fn runtime_type_step_kinematics_screw_pair() {
    let object = Handle::new(StepKinematicsScrewPair::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsScrewPair"
    );
    assert!(object.is_kind_name("StepKinematicsLowOrderKinematicPairWithMotionCoupling"));
}

#[test]
fn init_round_trip_step_kinematics_screw_pair() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_name = Handle::new(HAsciiString::new("sample"));
    let has_item_defined_transformation_description = true;
    let the_item_defined_transformation_description = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_transform_item1 =
        Handle::new(StepReprRepresentationItem::new());
    let the_item_defined_transformation_transform_item2 =
        Handle::new(StepReprRepresentationItem::new());
    let the_kinematic_pair_joint = Handle::new(StepKinematicsKinematicJoint::new());
    let the_pitch = 2.5;
    let object = StepKinematicsScrewPair::new();
    object.init(
        &the_representation_item_name,
        &the_item_defined_transformation_name,
        has_item_defined_transformation_description,
        &the_item_defined_transformation_description,
        &the_item_defined_transformation_transform_item1,
        &the_item_defined_transformation_transform_item2,
        &the_kinematic_pair_joint,
        the_pitch,
    );
    assert!(object.pitch() == the_pitch);
}

#[test]
fn runtime_type_step_kinematics_screw_pair_value() {
    let object = Handle::new(StepKinematicsScrewPairValue::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsScrewPairValue"
    );
    assert!(object.is_kind_name("StepKinematicsPairValue"));
}

#[test]
fn init_round_trip_step_kinematics_screw_pair_value() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_pair_value_applies_to_pair = Handle::new(StepKinematicsKinematicPair::new());
    let the_actual_rotation = 2.5;
    let object = StepKinematicsScrewPairValue::new();
    object.init(
        &the_representation_item_name,
        &the_pair_value_applies_to_pair,
        the_actual_rotation,
    );
    assert!(object.actual_rotation() == the_actual_rotation);
}

#[test]
fn runtime_type_step_kinematics_screw_pair_with_range() {
    let object = Handle::new(StepKinematicsScrewPairWithRange::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsScrewPairWithRange"
    );
    assert!(object.is_kind_name("StepKinematicsScrewPair"));
}

#[test]
fn init_round_trip_step_kinematics_screw_pair_with_range() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_name = Handle::new(HAsciiString::new("sample"));
    let has_item_defined_transformation_description = true;
    let the_item_defined_transformation_description = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_transform_item1 =
        Handle::new(StepReprRepresentationItem::new());
    let the_item_defined_transformation_transform_item2 =
        Handle::new(StepReprRepresentationItem::new());
    let the_kinematic_pair_joint = Handle::new(StepKinematicsKinematicJoint::new());
    let the_screw_pair_pitch = 2.5;
    let has_lower_limit_actual_rotation = true;
    let the_lower_limit_actual_rotation = 2.5;
    let has_upper_limit_actual_rotation = true;
    let the_upper_limit_actual_rotation = 2.5;
    let object = StepKinematicsScrewPairWithRange::new();
    object.init(
        &the_representation_item_name,
        &the_item_defined_transformation_name,
        has_item_defined_transformation_description,
        &the_item_defined_transformation_description,
        &the_item_defined_transformation_transform_item1,
        &the_item_defined_transformation_transform_item2,
        &the_kinematic_pair_joint,
        the_screw_pair_pitch,
        has_lower_limit_actual_rotation,
        the_lower_limit_actual_rotation,
        has_upper_limit_actual_rotation,
        the_upper_limit_actual_rotation,
    );
    assert!(object.has_lower_limit_actual_rotation() == has_lower_limit_actual_rotation);
    assert!(object.lower_limit_actual_rotation() == the_lower_limit_actual_rotation);
    assert!(object.has_upper_limit_actual_rotation() == has_upper_limit_actual_rotation);
    assert!(object.upper_limit_actual_rotation() == the_upper_limit_actual_rotation);
}

#[test]
fn runtime_type_step_kinematics_sliding_curve_pair() {
    let object = Handle::new(StepKinematicsSlidingCurvePair::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsSlidingCurvePair"
    );
    assert!(object.is_kind_name("StepKinematicsPlanarCurvePair"));
}

#[test]
fn runtime_type_step_kinematics_sliding_curve_pair_value() {
    let object = Handle::new(StepKinematicsSlidingCurvePairValue::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsSlidingCurvePairValue"
    );
    assert!(object.is_kind_name("StepKinematicsPairValue"));
}

#[test]
fn init_round_trip_step_kinematics_sliding_curve_pair_value() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_pair_value_applies_to_pair = Handle::new(StepKinematicsKinematicPair::new());
    let the_actual_point_on_curve1 = Handle::new(StepGeomPointOnCurve::new());
    let the_actual_point_on_curve2 = Handle::new(StepGeomPointOnCurve::new());
    let object = StepKinematicsSlidingCurvePairValue::new();
    object.init(
        &the_representation_item_name,
        &the_pair_value_applies_to_pair,
        &the_actual_point_on_curve1,
        &the_actual_point_on_curve2,
    );
    assert!(object.actual_point_on_curve1() == the_actual_point_on_curve1);
    assert!(object.actual_point_on_curve2() == the_actual_point_on_curve2);
}

#[test]
fn runtime_type_step_kinematics_sliding_surface_pair() {
    let object = Handle::new(StepKinematicsSlidingSurfacePair::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsSlidingSurfacePair"
    );
    assert!(object.is_kind_name("StepKinematicsSurfacePair"));
}

#[test]
fn runtime_type_step_kinematics_sliding_surface_pair_value() {
    let object = Handle::new(StepKinematicsSlidingSurfacePairValue::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsSlidingSurfacePairValue"
    );
    assert!(object.is_kind_name("StepKinematicsPairValue"));
}

#[test]
fn init_round_trip_step_kinematics_sliding_surface_pair_value() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_pair_value_applies_to_pair = Handle::new(StepKinematicsKinematicPair::new());
    let the_actual_point_on_surface1 = Handle::new(StepGeomPointOnSurface::new());
    let the_actual_point_on_surface2 = Handle::new(StepGeomPointOnSurface::new());
    let the_actual_rotation = 2.5;
    let object = StepKinematicsSlidingSurfacePairValue::new();
    object.init(
        &the_representation_item_name,
        &the_pair_value_applies_to_pair,
        &the_actual_point_on_surface1,
        &the_actual_point_on_surface2,
        the_actual_rotation,
    );
    assert!(object.actual_point_on_surface1() == the_actual_point_on_surface1);
    assert!(object.actual_point_on_surface2() == the_actual_point_on_surface2);
    assert!(object.actual_rotation() == the_actual_rotation);
}

#[test]
fn runtime_type_step_kinematics_spherical_pair() {
    let object = Handle::new(StepKinematicsSphericalPair::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsSphericalPair"
    );
    assert!(object.is_kind_name("StepKinematicsLowOrderKinematicPair"));
}

#[test]
fn runtime_type_step_kinematics_spherical_pair_value() {
    let object = Handle::new(StepKinematicsSphericalPairValue::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsSphericalPairValue"
    );
    assert!(object.is_kind_name("StepKinematicsPairValue"));
}

#[test]
fn init_round_trip_step_kinematics_spherical_pair_value() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_pair_value_applies_to_pair = Handle::new(StepKinematicsKinematicPair::new());
    let the_input_orientation = StepKinematicsSpatialRotation::default();
    let object = StepKinematicsSphericalPairValue::new();
    object.init(
        &the_representation_item_name,
        &the_pair_value_applies_to_pair,
        &the_input_orientation,
    );
}

#[test]
fn runtime_type_step_kinematics_spherical_pair_with_pin() {
    let object = Handle::new(StepKinematicsSphericalPairWithPin::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsSphericalPairWithPin"
    );
    assert!(object.is_kind_name("StepKinematicsLowOrderKinematicPair"));
}

#[test]
fn runtime_type_step_kinematics_spherical_pair_with_pin_and_range() {
    let object = Handle::new(StepKinematicsSphericalPairWithPinAndRange::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsSphericalPairWithPinAndRange"
    );
    assert!(object.is_kind_name("StepKinematicsSphericalPairWithPin"));
}
