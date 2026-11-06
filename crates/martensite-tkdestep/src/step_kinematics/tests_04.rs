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
fn runtime_type_step_kinematics_point_on_surface_pair() {
    let object = Handle::new(StepKinematicsPointOnSurfacePair::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsPointOnSurfacePair"
    );
    assert!(object.is_kind_name("StepKinematicsHighOrderKinematicPair"));
}

#[test]
fn init_round_trip_step_kinematics_point_on_surface_pair() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_name = Handle::new(HAsciiString::new("sample"));
    let has_item_defined_transformation_description = true;
    let the_item_defined_transformation_description = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_transform_item1 =
        Handle::new(StepReprRepresentationItem::new());
    let the_item_defined_transformation_transform_item2 =
        Handle::new(StepReprRepresentationItem::new());
    let the_kinematic_pair_joint = Handle::new(StepKinematicsKinematicJoint::new());
    let the_pair_surface = Handle::new(StepGeomSurface::new());
    let object = StepKinematicsPointOnSurfacePair::new();
    object.init(
        &the_representation_item_name,
        &the_item_defined_transformation_name,
        has_item_defined_transformation_description,
        &the_item_defined_transformation_description,
        &the_item_defined_transformation_transform_item1,
        &the_item_defined_transformation_transform_item2,
        &the_kinematic_pair_joint,
        &the_pair_surface,
    );
    assert!(object.pair_surface() == the_pair_surface);
}

#[test]
fn runtime_type_step_kinematics_point_on_surface_pair_value() {
    let object = Handle::new(StepKinematicsPointOnSurfacePairValue::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsPointOnSurfacePairValue"
    );
    assert!(object.is_kind_name("StepKinematicsPairValue"));
}

#[test]
fn init_round_trip_step_kinematics_point_on_surface_pair_value() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_pair_value_applies_to_pair = Handle::new(StepKinematicsKinematicPair::new());
    let the_actual_point_on_surface = Handle::new(StepGeomPointOnSurface::new());
    let the_input_orientation = StepKinematicsSpatialRotation::default();
    let object = StepKinematicsPointOnSurfacePairValue::new();
    object.init(
        &the_representation_item_name,
        &the_pair_value_applies_to_pair,
        &the_actual_point_on_surface,
        &the_input_orientation,
    );
    assert!(object.actual_point_on_surface() == the_actual_point_on_surface);
}

#[test]
fn runtime_type_step_kinematics_point_on_surface_pair_with_range() {
    let object = Handle::new(StepKinematicsPointOnSurfacePairWithRange::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsPointOnSurfacePairWithRange"
    );
    assert!(object.is_kind_name("StepKinematicsPointOnSurfacePair"));
}

#[test]
fn init_round_trip_step_kinematics_point_on_surface_pair_with_range() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_name = Handle::new(HAsciiString::new("sample"));
    let has_item_defined_transformation_description = true;
    let the_item_defined_transformation_description = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_transform_item1 =
        Handle::new(StepReprRepresentationItem::new());
    let the_item_defined_transformation_transform_item2 =
        Handle::new(StepReprRepresentationItem::new());
    let the_kinematic_pair_joint = Handle::new(StepKinematicsKinematicJoint::new());
    let the_point_on_surface_pair_pair_surface = Handle::new(StepGeomSurface::new());
    let the_range_on_pair_surface = Handle::new(StepGeomRectangularTrimmedSurface::new());
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
    let object = StepKinematicsPointOnSurfacePairWithRange::new();
    object.init(
        &the_representation_item_name,
        &the_item_defined_transformation_name,
        has_item_defined_transformation_description,
        &the_item_defined_transformation_description,
        &the_item_defined_transformation_transform_item1,
        &the_item_defined_transformation_transform_item2,
        &the_kinematic_pair_joint,
        &the_point_on_surface_pair_pair_surface,
        &the_range_on_pair_surface,
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
    assert!(object.range_on_pair_surface() == the_range_on_pair_surface);
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
fn runtime_type_step_kinematics_prismatic_pair() {
    let object = Handle::new(StepKinematicsPrismaticPair::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsPrismaticPair"
    );
    assert!(object.is_kind_name("StepKinematicsLowOrderKinematicPair"));
}

#[test]
fn runtime_type_step_kinematics_prismatic_pair_value() {
    let object = Handle::new(StepKinematicsPrismaticPairValue::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsPrismaticPairValue"
    );
    assert!(object.is_kind_name("StepKinematicsPairValue"));
}

#[test]
fn init_round_trip_step_kinematics_prismatic_pair_value() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_pair_value_applies_to_pair = Handle::new(StepKinematicsKinematicPair::new());
    let the_actual_translation = 2.5;
    let object = StepKinematicsPrismaticPairValue::new();
    object.init(
        &the_representation_item_name,
        &the_pair_value_applies_to_pair,
        the_actual_translation,
    );
    assert!(object.actual_translation() == the_actual_translation);
}

#[test]
fn runtime_type_step_kinematics_prismatic_pair_with_range() {
    let object = Handle::new(StepKinematicsPrismaticPairWithRange::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsPrismaticPairWithRange"
    );
    assert!(object.is_kind_name("StepKinematicsPrismaticPair"));
}

#[test]
fn init_round_trip_step_kinematics_prismatic_pair_with_range() {
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
    let object = StepKinematicsPrismaticPairWithRange::new();
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
    );
    assert!(object.has_lower_limit_actual_translation() == has_lower_limit_actual_translation);
    assert!(object.lower_limit_actual_translation() == the_lower_limit_actual_translation);
    assert!(object.has_upper_limit_actual_translation() == has_upper_limit_actual_translation);
    assert!(object.upper_limit_actual_translation() == the_upper_limit_actual_translation);
}

#[test]
fn runtime_type_step_kinematics_product_definition_kinematics() {
    let object = Handle::new(StepKinematicsProductDefinitionKinematics::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsProductDefinitionKinematics"
    );
    assert!(object.is_kind_name("StepReprPropertyDefinition"));
}

#[test]
fn runtime_type_step_kinematics_product_definition_relationship_kinematics() {
    let object = Handle::new(StepKinematicsProductDefinitionRelationshipKinematics::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsProductDefinitionRelationshipKinematics"
    );
    assert!(object.is_kind_name("StepReprPropertyDefinition"));
}

#[test]
fn runtime_type_step_kinematics_rack_and_pinion_pair() {
    let object = Handle::new(StepKinematicsRackAndPinionPair::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsRackAndPinionPair"
    );
    assert!(object.is_kind_name("StepKinematicsLowOrderKinematicPairWithMotionCoupling"));
}

#[test]
fn init_round_trip_step_kinematics_rack_and_pinion_pair() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_name = Handle::new(HAsciiString::new("sample"));
    let has_item_defined_transformation_description = true;
    let the_item_defined_transformation_description = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_transform_item1 =
        Handle::new(StepReprRepresentationItem::new());
    let the_item_defined_transformation_transform_item2 =
        Handle::new(StepReprRepresentationItem::new());
    let the_kinematic_pair_joint = Handle::new(StepKinematicsKinematicJoint::new());
    let the_pinion_radius = 2.5;
    let object = StepKinematicsRackAndPinionPair::new();
    object.init(
        &the_representation_item_name,
        &the_item_defined_transformation_name,
        has_item_defined_transformation_description,
        &the_item_defined_transformation_description,
        &the_item_defined_transformation_transform_item1,
        &the_item_defined_transformation_transform_item2,
        &the_kinematic_pair_joint,
        the_pinion_radius,
    );
    assert!(object.pinion_radius() == the_pinion_radius);
}

#[test]
fn runtime_type_step_kinematics_rack_and_pinion_pair_value() {
    let object = Handle::new(StepKinematicsRackAndPinionPairValue::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsRackAndPinionPairValue"
    );
    assert!(object.is_kind_name("StepKinematicsPairValue"));
}

#[test]
fn init_round_trip_step_kinematics_rack_and_pinion_pair_value() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_pair_value_applies_to_pair = Handle::new(StepKinematicsKinematicPair::new());
    let the_actual_displacement = 2.5;
    let object = StepKinematicsRackAndPinionPairValue::new();
    object.init(
        &the_representation_item_name,
        &the_pair_value_applies_to_pair,
        the_actual_displacement,
    );
    assert!(object.actual_displacement() == the_actual_displacement);
}

#[test]
fn runtime_type_step_kinematics_rack_and_pinion_pair_with_range() {
    let object = Handle::new(StepKinematicsRackAndPinionPairWithRange::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsRackAndPinionPairWithRange"
    );
    assert!(object.is_kind_name("StepKinematicsRackAndPinionPair"));
}

#[test]
fn init_round_trip_step_kinematics_rack_and_pinion_pair_with_range() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_name = Handle::new(HAsciiString::new("sample"));
    let has_item_defined_transformation_description = true;
    let the_item_defined_transformation_description = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_transform_item1 =
        Handle::new(StepReprRepresentationItem::new());
    let the_item_defined_transformation_transform_item2 =
        Handle::new(StepReprRepresentationItem::new());
    let the_kinematic_pair_joint = Handle::new(StepKinematicsKinematicJoint::new());
    let the_rack_and_pinion_pair_pinion_radius = 2.5;
    let has_lower_limit_rack_displacement = true;
    let the_lower_limit_rack_displacement = 2.5;
    let has_upper_limit_rack_displacement = true;
    let the_upper_limit_rack_displacement = 2.5;
    let object = StepKinematicsRackAndPinionPairWithRange::new();
    object.init(
        &the_representation_item_name,
        &the_item_defined_transformation_name,
        has_item_defined_transformation_description,
        &the_item_defined_transformation_description,
        &the_item_defined_transformation_transform_item1,
        &the_item_defined_transformation_transform_item2,
        &the_kinematic_pair_joint,
        the_rack_and_pinion_pair_pinion_radius,
        has_lower_limit_rack_displacement,
        the_lower_limit_rack_displacement,
        has_upper_limit_rack_displacement,
        the_upper_limit_rack_displacement,
    );
    assert!(object.has_lower_limit_rack_displacement() == has_lower_limit_rack_displacement);
    assert!(object.lower_limit_rack_displacement() == the_lower_limit_rack_displacement);
    assert!(object.has_upper_limit_rack_displacement() == has_upper_limit_rack_displacement);
    assert!(object.upper_limit_rack_displacement() == the_upper_limit_rack_displacement);
}

#[test]
fn runtime_type_step_kinematics_revolute_pair() {
    let object = Handle::new(StepKinematicsRevolutePair::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsRevolutePair"
    );
    assert!(object.is_kind_name("StepKinematicsLowOrderKinematicPair"));
}

#[test]
fn runtime_type_step_kinematics_revolute_pair_value() {
    let object = Handle::new(StepKinematicsRevolutePairValue::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsRevolutePairValue"
    );
    assert!(object.is_kind_name("StepKinematicsPairValue"));
}

#[test]
fn init_round_trip_step_kinematics_revolute_pair_value() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_pair_value_applies_to_pair = Handle::new(StepKinematicsKinematicPair::new());
    let the_actual_rotation = 2.5;
    let object = StepKinematicsRevolutePairValue::new();
    object.init(
        &the_representation_item_name,
        &the_pair_value_applies_to_pair,
        the_actual_rotation,
    );
    assert!(object.actual_rotation() == the_actual_rotation);
}

#[test]
fn runtime_type_step_kinematics_revolute_pair_with_range() {
    let object = Handle::new(StepKinematicsRevolutePairWithRange::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsRevolutePairWithRange"
    );
    assert!(object.is_kind_name("StepKinematicsRevolutePair"));
}
