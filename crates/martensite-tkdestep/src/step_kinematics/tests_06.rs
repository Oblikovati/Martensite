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
fn init_round_trip_step_kinematics_spherical_pair_with_pin_and_range() {
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
    let has_lower_limit_yaw = true;
    let the_lower_limit_yaw = 2.5;
    let has_upper_limit_yaw = true;
    let the_upper_limit_yaw = 2.5;
    let has_lower_limit_roll = true;
    let the_lower_limit_roll = 2.5;
    let has_upper_limit_roll = true;
    let the_upper_limit_roll = 2.5;
    let object = StepKinematicsSphericalPairWithPinAndRange::new();
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
        has_lower_limit_yaw,
        the_lower_limit_yaw,
        has_upper_limit_yaw,
        the_upper_limit_yaw,
        has_lower_limit_roll,
        the_lower_limit_roll,
        has_upper_limit_roll,
        the_upper_limit_roll,
    );
    assert!(object.has_lower_limit_yaw() == has_lower_limit_yaw);
    assert!(object.lower_limit_yaw() == the_lower_limit_yaw);
    assert!(object.has_upper_limit_yaw() == has_upper_limit_yaw);
    assert!(object.upper_limit_yaw() == the_upper_limit_yaw);
    assert!(object.has_lower_limit_roll() == has_lower_limit_roll);
    assert!(object.lower_limit_roll() == the_lower_limit_roll);
    assert!(object.has_upper_limit_roll() == has_upper_limit_roll);
    assert!(object.upper_limit_roll() == the_upper_limit_roll);
}

#[test]
fn runtime_type_step_kinematics_spherical_pair_with_range() {
    let object = Handle::new(StepKinematicsSphericalPairWithRange::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsSphericalPairWithRange"
    );
    assert!(object.is_kind_name("StepKinematicsSphericalPair"));
}

#[test]
fn init_round_trip_step_kinematics_spherical_pair_with_range() {
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
    let object = StepKinematicsSphericalPairWithRange::new();
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

#[test]
fn runtime_type_step_kinematics_surface_pair() {
    let object = Handle::new(StepKinematicsSurfacePair::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsSurfacePair"
    );
    assert!(object.is_kind_name("StepKinematicsHighOrderKinematicPair"));
}

#[test]
fn init_round_trip_step_kinematics_surface_pair() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_name = Handle::new(HAsciiString::new("sample"));
    let has_item_defined_transformation_description = true;
    let the_item_defined_transformation_description = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_transform_item1 =
        Handle::new(StepReprRepresentationItem::new());
    let the_item_defined_transformation_transform_item2 =
        Handle::new(StepReprRepresentationItem::new());
    let the_kinematic_pair_joint = Handle::new(StepKinematicsKinematicJoint::new());
    let the_surface1 = Handle::new(StepGeomSurface::new());
    let the_surface2 = Handle::new(StepGeomSurface::new());
    let the_orientation = true;
    let object = StepKinematicsSurfacePair::new();
    object.init(
        &the_representation_item_name,
        &the_item_defined_transformation_name,
        has_item_defined_transformation_description,
        &the_item_defined_transformation_description,
        &the_item_defined_transformation_transform_item1,
        &the_item_defined_transformation_transform_item2,
        &the_kinematic_pair_joint,
        &the_surface1,
        &the_surface2,
        the_orientation,
    );
    assert!(object.surface1() == the_surface1);
    assert!(object.surface2() == the_surface2);
    assert!(object.orientation() == the_orientation);
}

#[test]
fn runtime_type_step_kinematics_surface_pair_with_range() {
    let object = Handle::new(StepKinematicsSurfacePairWithRange::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsSurfacePairWithRange"
    );
    assert!(object.is_kind_name("StepKinematicsSurfacePair"));
}

#[test]
fn init_round_trip_step_kinematics_surface_pair_with_range() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_name = Handle::new(HAsciiString::new("sample"));
    let has_item_defined_transformation_description = true;
    let the_item_defined_transformation_description = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_transform_item1 =
        Handle::new(StepReprRepresentationItem::new());
    let the_item_defined_transformation_transform_item2 =
        Handle::new(StepReprRepresentationItem::new());
    let the_kinematic_pair_joint = Handle::new(StepKinematicsKinematicJoint::new());
    let the_surface_pair_surface1 = Handle::new(StepGeomSurface::new());
    let the_surface_pair_surface2 = Handle::new(StepGeomSurface::new());
    let the_surface_pair_orientation = true;
    let the_range_on_surface1 = Handle::new(StepGeomRectangularTrimmedSurface::new());
    let the_range_on_surface2 = Handle::new(StepGeomRectangularTrimmedSurface::new());
    let has_lower_limit_actual_rotation = true;
    let the_lower_limit_actual_rotation = 2.5;
    let has_upper_limit_actual_rotation = true;
    let the_upper_limit_actual_rotation = 2.5;
    let object = StepKinematicsSurfacePairWithRange::new();
    object.init(
        &the_representation_item_name,
        &the_item_defined_transformation_name,
        has_item_defined_transformation_description,
        &the_item_defined_transformation_description,
        &the_item_defined_transformation_transform_item1,
        &the_item_defined_transformation_transform_item2,
        &the_kinematic_pair_joint,
        &the_surface_pair_surface1,
        &the_surface_pair_surface2,
        the_surface_pair_orientation,
        &the_range_on_surface1,
        &the_range_on_surface2,
        has_lower_limit_actual_rotation,
        the_lower_limit_actual_rotation,
        has_upper_limit_actual_rotation,
        the_upper_limit_actual_rotation,
    );
    assert!(object.range_on_surface1() == the_range_on_surface1);
    assert!(object.range_on_surface2() == the_range_on_surface2);
    assert!(object.has_lower_limit_actual_rotation() == has_lower_limit_actual_rotation);
    assert!(object.lower_limit_actual_rotation() == the_lower_limit_actual_rotation);
    assert!(object.has_upper_limit_actual_rotation() == has_upper_limit_actual_rotation);
    assert!(object.upper_limit_actual_rotation() == the_upper_limit_actual_rotation);
}

#[test]
fn runtime_type_step_kinematics_unconstrained_pair() {
    let object = Handle::new(StepKinematicsUnconstrainedPair::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsUnconstrainedPair"
    );
    assert!(object.is_kind_name("StepKinematicsLowOrderKinematicPair"));
}

#[test]
fn runtime_type_step_kinematics_unconstrained_pair_value() {
    let object = Handle::new(StepKinematicsUnconstrainedPairValue::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsUnconstrainedPairValue"
    );
    assert!(object.is_kind_name("StepKinematicsPairValue"));
}

#[test]
fn init_round_trip_step_kinematics_unconstrained_pair_value() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_pair_value_applies_to_pair = Handle::new(StepKinematicsKinematicPair::new());
    let the_actual_placement = Handle::new(StepGeomAxis2Placement3d::new());
    let object = StepKinematicsUnconstrainedPairValue::new();
    object.init(
        &the_representation_item_name,
        &the_pair_value_applies_to_pair,
        &the_actual_placement,
    );
    assert!(object.actual_placement() == the_actual_placement);
}

#[test]
fn runtime_type_step_kinematics_universal_pair() {
    let object = Handle::new(StepKinematicsUniversalPair::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsUniversalPair"
    );
    assert!(object.is_kind_name("StepKinematicsLowOrderKinematicPair"));
}

#[test]
fn init_round_trip_step_kinematics_universal_pair() {
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
    let has_input_skew_angle = true;
    let the_input_skew_angle = 2.5;
    let object = StepKinematicsUniversalPair::new();
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
        has_input_skew_angle,
        the_input_skew_angle,
    );
    assert!(object.has_input_skew_angle() == has_input_skew_angle);
    assert!(object.input_skew_angle() == the_input_skew_angle);
}

#[test]
fn runtime_type_step_kinematics_universal_pair_value() {
    let object = Handle::new(StepKinematicsUniversalPairValue::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsUniversalPairValue"
    );
    assert!(object.is_kind_name("StepKinematicsPairValue"));
}

#[test]
fn init_round_trip_step_kinematics_universal_pair_value() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_pair_value_applies_to_pair = Handle::new(StepKinematicsKinematicPair::new());
    let the_first_rotation_angle = 2.5;
    let the_second_rotation_angle = 2.5;
    let object = StepKinematicsUniversalPairValue::new();
    object.init(
        &the_representation_item_name,
        &the_pair_value_applies_to_pair,
        the_first_rotation_angle,
        the_second_rotation_angle,
    );
    assert!(object.first_rotation_angle() == the_first_rotation_angle);
    assert!(object.second_rotation_angle() == the_second_rotation_angle);
}

#[test]
fn runtime_type_step_kinematics_universal_pair_with_range() {
    let object = Handle::new(StepKinematicsUniversalPairWithRange::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsUniversalPairWithRange"
    );
    assert!(object.is_kind_name("StepKinematicsUniversalPair"));
}

#[test]
fn init_round_trip_step_kinematics_universal_pair_with_range() {
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
    let has_universal_pair_input_skew_angle = true;
    let the_universal_pair_input_skew_angle = 2.5;
    let has_lower_limit_first_rotation = true;
    let the_lower_limit_first_rotation = 2.5;
    let has_upper_limit_first_rotation = true;
    let the_upper_limit_first_rotation = 2.5;
    let has_lower_limit_second_rotation = true;
    let the_lower_limit_second_rotation = 2.5;
    let has_upper_limit_second_rotation = true;
    let the_upper_limit_second_rotation = 2.5;
    let object = StepKinematicsUniversalPairWithRange::new();
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
        has_universal_pair_input_skew_angle,
        the_universal_pair_input_skew_angle,
        has_lower_limit_first_rotation,
        the_lower_limit_first_rotation,
        has_upper_limit_first_rotation,
        the_upper_limit_first_rotation,
        has_lower_limit_second_rotation,
        the_lower_limit_second_rotation,
        has_upper_limit_second_rotation,
        the_upper_limit_second_rotation,
    );
    assert!(object.has_lower_limit_first_rotation() == has_lower_limit_first_rotation);
    assert!(object.lower_limit_first_rotation() == the_lower_limit_first_rotation);
    assert!(object.has_upper_limit_first_rotation() == has_upper_limit_first_rotation);
    assert!(object.upper_limit_first_rotation() == the_upper_limit_first_rotation);
    assert!(object.has_lower_limit_second_rotation() == has_lower_limit_second_rotation);
    assert!(object.lower_limit_second_rotation() == the_lower_limit_second_rotation);
    assert!(object.has_upper_limit_second_rotation() == has_upper_limit_second_rotation);
    assert!(object.upper_limit_second_rotation() == the_upper_limit_second_rotation);
}
