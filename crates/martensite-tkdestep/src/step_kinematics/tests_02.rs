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
fn runtime_type_step_kinematics_kinematic_link_representation_association() {
    let object = Handle::new(StepKinematicsKinematicLinkRepresentationAssociation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsKinematicLinkRepresentationAssociation"
    );
    assert!(object.is_kind_name("StepReprRepresentationRelationship"));
}

#[test]
fn runtime_type_step_kinematics_kinematic_pair() {
    let object = Handle::new(StepKinematicsKinematicPair::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsKinematicPair"
    );
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}

#[test]
fn runtime_type_step_kinematics_kinematic_property_definition_representation() {
    let object = Handle::new(StepKinematicsKinematicPropertyDefinitionRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsKinematicPropertyDefinitionRepresentation"
    );
    assert!(object.is_kind_name("StepReprPropertyDefinitionRepresentation"));
}

#[test]
fn runtime_type_step_kinematics_kinematic_property_mechanism_representation() {
    let object = Handle::new(StepKinematicsKinematicPropertyMechanismRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsKinematicPropertyMechanismRepresentation"
    );
    assert!(object.is_kind_name("StepKinematicsKinematicPropertyDefinitionRepresentation"));
}

#[test]
fn init_round_trip_step_kinematics_kinematic_property_mechanism_representation() {
    let the_property_definition_representation_definition =
        StepReprRepresentedDefinition::default();
    let the_property_definition_representation_used_representation =
        Handle::new(StepReprRepresentation::new());
    let the_base = Handle::new(StepKinematicsKinematicLinkRepresentation::new());
    let object = StepKinematicsKinematicPropertyMechanismRepresentation::new();
    object.init(
        &the_property_definition_representation_definition,
        &the_property_definition_representation_used_representation,
        &the_base,
    );
    assert!(object.base() == the_base);
}

#[test]
fn runtime_type_step_kinematics_kinematic_topology_directed_structure() {
    let object = Handle::new(StepKinematicsKinematicTopologyDirectedStructure::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsKinematicTopologyDirectedStructure"
    );
    assert!(object.is_kind_name("StepReprRepresentation"));
}

#[test]
fn init_round_trip_step_kinematics_kinematic_topology_directed_structure() {
    let the_representation_name = Handle::new(HAsciiString::new("sample"));
    let the_representation_items =
        Handle::new(HArray1::<Handle<StepReprRepresentationItem>>::with_bounds(1, 2).unwrap());
    let the_representation_context_of_items = Handle::new(StepReprRepresentationContext::new());
    let the_parent = Handle::new(StepKinematicsKinematicTopologyStructure::new());
    let object = StepKinematicsKinematicTopologyDirectedStructure::new();
    object.init(
        &the_representation_name,
        &the_representation_items,
        &the_representation_context_of_items,
        &the_parent,
    );
    assert!(object.parent() == the_parent);
}

#[test]
fn runtime_type_step_kinematics_kinematic_topology_network_structure() {
    let object = Handle::new(StepKinematicsKinematicTopologyNetworkStructure::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsKinematicTopologyNetworkStructure"
    );
    assert!(object.is_kind_name("StepReprRepresentation"));
}

#[test]
fn init_round_trip_step_kinematics_kinematic_topology_network_structure() {
    let the_representation_name = Handle::new(HAsciiString::new("sample"));
    let the_representation_items =
        Handle::new(HArray1::<Handle<StepReprRepresentationItem>>::with_bounds(1, 2).unwrap());
    let the_representation_context_of_items = Handle::new(StepReprRepresentationContext::new());
    let the_parent = Handle::new(StepKinematicsKinematicTopologyStructure::new());
    let object = StepKinematicsKinematicTopologyNetworkStructure::new();
    object.init(
        &the_representation_name,
        &the_representation_items,
        &the_representation_context_of_items,
        &the_parent,
    );
    assert!(object.parent() == the_parent);
}

#[test]
fn runtime_type_step_kinematics_kinematic_topology_structure() {
    let object = Handle::new(StepKinematicsKinematicTopologyStructure::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsKinematicTopologyStructure"
    );
    assert!(object.is_kind_name("StepReprRepresentation"));
}

#[test]
fn runtime_type_step_kinematics_linear_flexible_and_pinion_pair() {
    let object = Handle::new(StepKinematicsLinearFlexibleAndPinionPair::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsLinearFlexibleAndPinionPair"
    );
    assert!(object.is_kind_name("StepKinematicsLowOrderKinematicPairWithMotionCoupling"));
}

#[test]
fn init_round_trip_step_kinematics_linear_flexible_and_pinion_pair() {
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
    let object = StepKinematicsLinearFlexibleAndPinionPair::new();
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
fn runtime_type_step_kinematics_linear_flexible_and_planar_curve_pair() {
    let object = Handle::new(StepKinematicsLinearFlexibleAndPlanarCurvePair::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsLinearFlexibleAndPlanarCurvePair"
    );
    assert!(object.is_kind_name("StepKinematicsHighOrderKinematicPair"));
}

#[test]
fn init_round_trip_step_kinematics_linear_flexible_and_planar_curve_pair() {
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
    let object = StepKinematicsLinearFlexibleAndPlanarCurvePair::new();
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
fn runtime_type_step_kinematics_linear_flexible_link_representation() {
    let object = Handle::new(StepKinematicsLinearFlexibleLinkRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsLinearFlexibleLinkRepresentation"
    );
    assert!(object.is_kind_name("StepKinematicsKinematicLinkRepresentation"));
}

#[test]
fn runtime_type_step_kinematics_low_order_kinematic_pair() {
    let object = Handle::new(StepKinematicsLowOrderKinematicPair::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsLowOrderKinematicPair"
    );
    assert!(object.is_kind_name("StepKinematicsKinematicPair"));
}

#[test]
fn init_round_trip_step_kinematics_low_order_kinematic_pair() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_name = Handle::new(HAsciiString::new("sample"));
    let has_item_defined_transformation_description = true;
    let the_item_defined_transformation_description = Handle::new(HAsciiString::new("sample"));
    let the_item_defined_transformation_transform_item1 =
        Handle::new(StepReprRepresentationItem::new());
    let the_item_defined_transformation_transform_item2 =
        Handle::new(StepReprRepresentationItem::new());
    let the_kinematic_pair_joint = Handle::new(StepKinematicsKinematicJoint::new());
    let the_tx = true;
    let the_ty = true;
    let the_tz = true;
    let the_rx = true;
    let the_ry = true;
    let the_rz = true;
    let object = StepKinematicsLowOrderKinematicPair::new();
    object.init(
        &the_representation_item_name,
        &the_item_defined_transformation_name,
        has_item_defined_transformation_description,
        &the_item_defined_transformation_description,
        &the_item_defined_transformation_transform_item1,
        &the_item_defined_transformation_transform_item2,
        &the_kinematic_pair_joint,
        the_tx,
        the_ty,
        the_tz,
        the_rx,
        the_ry,
        the_rz,
    );
    assert!(object.tx() == the_tx);
    assert!(object.ty() == the_ty);
    assert!(object.tz() == the_tz);
    assert!(object.rx() == the_rx);
    assert!(object.ry() == the_ry);
    assert!(object.rz() == the_rz);
}

#[test]
fn runtime_type_step_kinematics_low_order_kinematic_pair_value() {
    let object = Handle::new(StepKinematicsLowOrderKinematicPairValue::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsLowOrderKinematicPairValue"
    );
    assert!(object.is_kind_name("StepKinematicsPairValue"));
}

#[test]
fn init_round_trip_step_kinematics_low_order_kinematic_pair_value() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_pair_value_applies_to_pair = Handle::new(StepKinematicsKinematicPair::new());
    let the_actual_translation_x = 2.5;
    let the_actual_translation_y = 2.5;
    let the_actual_translation_z = 2.5;
    let the_actual_rotation_x = 2.5;
    let the_actual_rotation_y = 2.5;
    let the_actual_rotation_z = 2.5;
    let object = StepKinematicsLowOrderKinematicPairValue::new();
    object.init(
        &the_representation_item_name,
        &the_pair_value_applies_to_pair,
        the_actual_translation_x,
        the_actual_translation_y,
        the_actual_translation_z,
        the_actual_rotation_x,
        the_actual_rotation_y,
        the_actual_rotation_z,
    );
    assert!(object.actual_translation_x() == the_actual_translation_x);
    assert!(object.actual_translation_y() == the_actual_translation_y);
    assert!(object.actual_translation_z() == the_actual_translation_z);
    assert!(object.actual_rotation_x() == the_actual_rotation_x);
    assert!(object.actual_rotation_y() == the_actual_rotation_y);
    assert!(object.actual_rotation_z() == the_actual_rotation_z);
}

#[test]
fn runtime_type_step_kinematics_low_order_kinematic_pair_with_motion_coupling() {
    let object = Handle::new(StepKinematicsLowOrderKinematicPairWithMotionCoupling::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsLowOrderKinematicPairWithMotionCoupling"
    );
    assert!(object.is_kind_name("StepKinematicsKinematicPair"));
}

#[test]
fn runtime_type_step_kinematics_low_order_kinematic_pair_with_range() {
    let object = Handle::new(StepKinematicsLowOrderKinematicPairWithRange::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsLowOrderKinematicPairWithRange"
    );
    assert!(object.is_kind_name("StepKinematicsLowOrderKinematicPair"));
}

#[test]
fn init_round_trip_step_kinematics_low_order_kinematic_pair_with_range() {
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
    let has_lower_limit_actual_rotation_x = true;
    let the_lower_limit_actual_rotation_x = 2.5;
    let has_upper_limit_actual_rotation_x = true;
    let the_upper_limit_actual_rotation_x = 2.5;
    let has_lower_limit_actual_rotation_y = true;
    let the_lower_limit_actual_rotation_y = 2.5;
    let has_upper_limit_actual_rotation_y = true;
    let the_upper_limit_actual_rotation_y = 2.5;
    let has_lower_limit_actual_rotation_z = true;
    let the_lower_limit_actual_rotation_z = 2.5;
    let has_upper_limit_actual_rotation_z = true;
    let the_upper_limit_actual_rotation_z = 2.5;
    let has_lower_limit_actual_translation_x = true;
    let the_lower_limit_actual_translation_x = 2.5;
    let has_upper_limit_actual_translation_x = true;
    let the_upper_limit_actual_translation_x = 2.5;
    let has_lower_limit_actual_translation_y = true;
    let the_lower_limit_actual_translation_y = 2.5;
    let has_upper_limit_actual_translation_y = true;
    let the_upper_limit_actual_translation_y = 2.5;
    let has_lower_limit_actual_translation_z = true;
    let the_lower_limit_actual_translation_z = 2.5;
    let has_upper_limit_actual_translation_z = true;
    let the_upper_limit_actual_translation_z = 2.5;
    let object = StepKinematicsLowOrderKinematicPairWithRange::new();
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
        has_lower_limit_actual_rotation_x,
        the_lower_limit_actual_rotation_x,
        has_upper_limit_actual_rotation_x,
        the_upper_limit_actual_rotation_x,
        has_lower_limit_actual_rotation_y,
        the_lower_limit_actual_rotation_y,
        has_upper_limit_actual_rotation_y,
        the_upper_limit_actual_rotation_y,
        has_lower_limit_actual_rotation_z,
        the_lower_limit_actual_rotation_z,
        has_upper_limit_actual_rotation_z,
        the_upper_limit_actual_rotation_z,
        has_lower_limit_actual_translation_x,
        the_lower_limit_actual_translation_x,
        has_upper_limit_actual_translation_x,
        the_upper_limit_actual_translation_x,
        has_lower_limit_actual_translation_y,
        the_lower_limit_actual_translation_y,
        has_upper_limit_actual_translation_y,
        the_upper_limit_actual_translation_y,
        has_lower_limit_actual_translation_z,
        the_lower_limit_actual_translation_z,
        has_upper_limit_actual_translation_z,
        the_upper_limit_actual_translation_z,
    );
    assert!(object.has_lower_limit_actual_rotation_x() == has_lower_limit_actual_rotation_x);
    assert!(object.lower_limit_actual_rotation_x() == the_lower_limit_actual_rotation_x);
    assert!(object.has_upper_limit_actual_rotation_x() == has_upper_limit_actual_rotation_x);
    assert!(object.upper_limit_actual_rotation_x() == the_upper_limit_actual_rotation_x);
    assert!(object.has_lower_limit_actual_rotation_y() == has_lower_limit_actual_rotation_y);
    assert!(object.lower_limit_actual_rotation_y() == the_lower_limit_actual_rotation_y);
    assert!(object.has_upper_limit_actual_rotation_y() == has_upper_limit_actual_rotation_y);
    assert!(object.upper_limit_actual_rotation_y() == the_upper_limit_actual_rotation_y);
    assert!(object.has_lower_limit_actual_rotation_z() == has_lower_limit_actual_rotation_z);
    assert!(object.lower_limit_actual_rotation_z() == the_lower_limit_actual_rotation_z);
    assert!(object.has_upper_limit_actual_rotation_z() == has_upper_limit_actual_rotation_z);
    assert!(object.upper_limit_actual_rotation_z() == the_upper_limit_actual_rotation_z);
    assert!(object.has_lower_limit_actual_translation_x() == has_lower_limit_actual_translation_x);
    assert!(object.lower_limit_actual_translation_x() == the_lower_limit_actual_translation_x);
    assert!(object.has_upper_limit_actual_translation_x() == has_upper_limit_actual_translation_x);
    assert!(object.upper_limit_actual_translation_x() == the_upper_limit_actual_translation_x);
    assert!(object.has_lower_limit_actual_translation_y() == has_lower_limit_actual_translation_y);
    assert!(object.lower_limit_actual_translation_y() == the_lower_limit_actual_translation_y);
    assert!(object.has_upper_limit_actual_translation_y() == has_upper_limit_actual_translation_y);
    assert!(object.upper_limit_actual_translation_y() == the_upper_limit_actual_translation_y);
    assert!(object.has_lower_limit_actual_translation_z() == has_lower_limit_actual_translation_z);
    assert!(object.lower_limit_actual_translation_z() == the_lower_limit_actual_translation_z);
    assert!(object.has_upper_limit_actual_translation_z() == has_upper_limit_actual_translation_z);
    assert!(object.upper_limit_actual_translation_z() == the_upper_limit_actual_translation_z);
}

#[test]
fn runtime_type_step_kinematics_mechanism_representation() {
    let object = Handle::new(StepKinematicsMechanismRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepKinematicsMechanismRepresentation"
    );
    assert!(object.is_kind_name("StepReprRepresentation"));
}
