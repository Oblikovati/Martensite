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
fn runtime_type_step_shape_advanced_brep_shape_representation() {
    let object = Handle::new(StepShapeAdvancedBrepShapeRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeAdvancedBrepShapeRepresentation"
    );
    assert!(object.is_kind_name("StepShapeShapeRepresentation"));
}

#[test]
fn runtime_type_step_shape_advanced_face() {
    let object = Handle::new(StepShapeAdvancedFace::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeAdvancedFace"
    );
    assert!(object.is_kind_name("StepShapeFaceSurface"));
}

#[test]
fn runtime_type_step_shape_angular_location() {
    let object = Handle::new(StepShapeAngularLocation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeAngularLocation"
    );
    assert!(object.is_kind_name("StepShapeDimensionalLocation"));
}

#[test]
fn init_round_trip_step_shape_angular_location() {
    let a_shape_aspect_relationship_name = Handle::new(HAsciiString::new("sample"));
    let has_shape_aspect_relationship_description = true;
    let a_shape_aspect_relationship_description = Handle::new(HAsciiString::new("sample"));
    let a_shape_aspect_relationship_relating_shape_aspect = Handle::new(StepReprShapeAspect::new());
    let a_shape_aspect_relationship_related_shape_aspect = Handle::new(StepReprShapeAspect::new());
    let a_angle_selection = StepShapeAngleRelator::default();
    let object = StepShapeAngularLocation::new();
    object.init(
        &a_shape_aspect_relationship_name,
        has_shape_aspect_relationship_description,
        &a_shape_aspect_relationship_description,
        &a_shape_aspect_relationship_relating_shape_aspect,
        &a_shape_aspect_relationship_related_shape_aspect,
        a_angle_selection,
    );
    assert!(object.angle_selection() == a_angle_selection);
}

#[test]
fn runtime_type_step_shape_angular_size() {
    let object = Handle::new(StepShapeAngularSize::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeAngularSize"
    );
    assert!(object.is_kind_name("StepShapeDimensionalSize"));
}

#[test]
fn init_round_trip_step_shape_angular_size() {
    let a_dimensional_size_applies_to = Handle::new(StepReprShapeAspect::new());
    let a_dimensional_size_name = Handle::new(HAsciiString::new("sample"));
    let a_angle_selection = StepShapeAngleRelator::default();
    let object = StepShapeAngularSize::new();
    object.init(
        &a_dimensional_size_applies_to,
        &a_dimensional_size_name,
        a_angle_selection,
    );
    assert!(object.angle_selection() == a_angle_selection);
}

#[test]
fn runtime_type_step_shape_block() {
    let object = Handle::new(StepShapeBlock::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepShapeBlock");
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}

#[test]
fn init_round_trip_step_shape_block() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_position = Handle::new(StepGeomAxis2Placement3d::new());
    let a_x = 2.5;
    let a_y = 2.5;
    let a_z = 2.5;
    let object = StepShapeBlock::new();
    object.init(&a_name, &a_position, a_x, a_y, a_z);
    assert!(object.position() == a_position);
    assert!(object.x() == a_x);
    assert!(object.y() == a_y);
    assert!(object.z() == a_z);
}

#[test]
fn runtime_type_step_shape_boolean_result() {
    let object = Handle::new(StepShapeBooleanResult::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeBooleanResult"
    );
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}

#[test]
fn init_round_trip_step_shape_boolean_result() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_operator = StepShapeBooleanOperator::default();
    let a_first_operand = StepShapeBooleanOperand::default();
    let a_second_operand = StepShapeBooleanOperand::default();
    let object = StepShapeBooleanResult::new();
    object.init(&a_name, a_operator, &a_first_operand, &a_second_operand);
    assert!(object.operator() == a_operator);
}

#[test]
fn runtime_type_step_shape_box_domain() {
    let object = Handle::new(StepShapeBoxDomain::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepShapeBoxDomain");
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_shape_box_domain() {
    let a_corner = Handle::new(StepGeomCartesianPoint::new());
    let a_xlength = 2.5;
    let a_ylength = 2.5;
    let a_zlength = 2.5;
    let object = StepShapeBoxDomain::new();
    object.init(&a_corner, a_xlength, a_ylength, a_zlength);
    assert!(object.corner() == a_corner);
    assert!(object.xlength() == a_xlength);
    assert!(object.ylength() == a_ylength);
    assert!(object.zlength() == a_zlength);
}

#[test]
fn runtime_type_step_shape_boxed_half_space() {
    let object = Handle::new(StepShapeBoxedHalfSpace::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeBoxedHalfSpace"
    );
    assert!(object.is_kind_name("StepShapeHalfSpaceSolid"));
}

#[test]
fn init_round_trip_step_shape_boxed_half_space() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_base_surface = Handle::new(StepGeomSurface::new());
    let a_agreement_flag = true;
    let a_enclosure = Handle::new(StepShapeBoxDomain::new());
    let object = StepShapeBoxedHalfSpace::new();
    object.init(&a_name, &a_base_surface, a_agreement_flag, &a_enclosure);
    assert!(object.enclosure() == a_enclosure);
}

#[test]
fn runtime_type_step_shape_brep_with_voids() {
    let object = Handle::new(StepShapeBrepWithVoids::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeBrepWithVoids"
    );
    assert!(object.is_kind_name("StepShapeManifoldSolidBrep"));
}

#[test]
fn init_round_trip_step_shape_brep_with_voids() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_outer = Handle::new(StepShapeClosedShell::new());
    let a_voids =
        Handle::new(HArray1::<Handle<StepShapeOrientedClosedShell>>::with_bounds(1, 2).unwrap());
    let object = StepShapeBrepWithVoids::new();
    object.init(&a_name, &a_outer, &a_voids);
    assert!(object.voids() == a_voids);
}

#[test]
fn runtime_type_step_shape_closed_shell() {
    let object = Handle::new(StepShapeClosedShell::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeClosedShell"
    );
    assert!(object.is_kind_name("StepShapeConnectedFaceSet"));
}

#[test]
fn runtime_type_step_shape_compound_shape_representation() {
    let object = Handle::new(StepShapeCompoundShapeRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeCompoundShapeRepresentation"
    );
    assert!(object.is_kind_name("StepShapeShapeRepresentation"));
}

#[test]
fn runtime_type_step_shape_connected_edge_set() {
    let object = Handle::new(StepShapeConnectedEdgeSet::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeConnectedEdgeSet"
    );
    assert!(object.is_kind_name("StepShapeTopologicalRepresentationItem"));
}

#[test]
fn init_round_trip_step_shape_connected_edge_set() {
    let a_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let a_ces_edges = Handle::new(HArray1::<Handle<StepShapeEdge>>::with_bounds(1, 2).unwrap());
    let object = StepShapeConnectedEdgeSet::new();
    object.init(&a_representation_item_name, &a_ces_edges);
    assert!(object.ces_edges() == a_ces_edges);
}

#[test]
fn runtime_type_step_shape_connected_face_shape_representation() {
    let object = Handle::new(StepShapeConnectedFaceShapeRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeConnectedFaceShapeRepresentation"
    );
    assert!(object.is_kind_name("StepReprRepresentation"));
}

#[test]
fn runtime_type_step_shape_connected_face_sub_set() {
    let object = Handle::new(StepShapeConnectedFaceSubSet::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeConnectedFaceSubSet"
    );
    assert!(object.is_kind_name("StepShapeConnectedFaceSet"));
}

#[test]
fn init_round_trip_step_shape_connected_face_sub_set() {
    let a_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let a_connected_face_set_cfs_faces =
        Handle::new(HArray1::<Handle<StepShapeFace>>::with_bounds(1, 2).unwrap());
    let a_parent_face_set = Handle::new(StepShapeConnectedFaceSet::new());
    let object = StepShapeConnectedFaceSubSet::new();
    object.init(
        &a_representation_item_name,
        &a_connected_face_set_cfs_faces,
        &a_parent_face_set,
    );
    assert!(object.parent_face_set() == a_parent_face_set);
}

#[test]
fn runtime_type_step_shape_context_dependent_shape_representation() {
    let object = Handle::new(StepShapeContextDependentShapeRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeContextDependentShapeRepresentation"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_shape_context_dependent_shape_representation() {
    let a_rep_rel = Handle::new(StepReprShapeRepresentationRelationship::new());
    let a_pro_rel = Handle::new(StepReprProductDefinitionShape::new());
    let object = StepShapeContextDependentShapeRepresentation::new();
    object.init(&a_rep_rel, &a_pro_rel);
    assert!(object.representation_relation() == a_rep_rel);
    assert!(object.represented_product_relation() == a_pro_rel);
}

#[test]
fn runtime_type_step_shape_csg_shape_representation() {
    let object = Handle::new(StepShapeCsgShapeRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeCsgShapeRepresentation"
    );
    assert!(object.is_kind_name("StepShapeShapeRepresentation"));
}

#[test]
fn runtime_type_step_shape_csg_solid() {
    let object = Handle::new(StepShapeCsgSolid::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepShapeCsgSolid");
    assert!(object.is_kind_name("StepShapeSolidModel"));
}

#[test]
fn init_round_trip_step_shape_csg_solid() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_tree_root_expression = StepShapeCsgSelect::default();
    let object = StepShapeCsgSolid::new();
    object.init(&a_name, &a_tree_root_expression);
}

#[test]
fn runtime_type_step_shape_definitional_representation_and_shape_representation() {
    let object = Handle::new(StepShapeDefinitionalRepresentationAndShapeRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeDefinitionalRepresentationAndShapeRepresentation"
    );
    assert!(object.is_kind_name("StepReprDefinitionalRepresentation"));
}

#[test]
fn runtime_type_step_shape_dimensional_characteristic_representation() {
    let object = Handle::new(StepShapeDimensionalCharacteristicRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeDimensionalCharacteristicRepresentation"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_shape_dimensional_characteristic_representation() {
    let a_dimension = StepShapeDimensionalCharacteristic::default();
    let a_representation = Handle::new(StepShapeShapeDimensionRepresentation::new());
    let object = StepShapeDimensionalCharacteristicRepresentation::new();
    object.init(&a_dimension, &a_representation);
    assert!(object.representation() == a_representation);
}

#[test]
fn runtime_type_step_shape_dimensional_location() {
    let object = Handle::new(StepShapeDimensionalLocation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeDimensionalLocation"
    );
    assert!(object.is_kind_name("StepReprShapeAspectRelationship"));
}

#[test]
fn runtime_type_step_shape_dimensional_location_with_path() {
    let object = Handle::new(StepShapeDimensionalLocationWithPath::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeDimensionalLocationWithPath"
    );
    assert!(object.is_kind_name("StepShapeDimensionalLocation"));
}

#[test]
fn init_round_trip_step_shape_dimensional_location_with_path() {
    let a_shape_aspect_relationship_name = Handle::new(HAsciiString::new("sample"));
    let has_shape_aspect_relationship_description = true;
    let a_shape_aspect_relationship_description = Handle::new(HAsciiString::new("sample"));
    let a_shape_aspect_relationship_relating_shape_aspect = Handle::new(StepReprShapeAspect::new());
    let a_shape_aspect_relationship_related_shape_aspect = Handle::new(StepReprShapeAspect::new());
    let a_path = Handle::new(StepReprShapeAspect::new());
    let object = StepShapeDimensionalLocationWithPath::new();
    object.init(
        &a_shape_aspect_relationship_name,
        has_shape_aspect_relationship_description,
        &a_shape_aspect_relationship_description,
        &a_shape_aspect_relationship_relating_shape_aspect,
        &a_shape_aspect_relationship_related_shape_aspect,
        &a_path,
    );
    assert!(object.path() == a_path);
}
