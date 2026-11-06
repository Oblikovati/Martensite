#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
#[allow(unused_imports)]
use crate::step_element::*;
use crate::step_entity::*;
#[allow(unused_imports)]
use crate::step_geom::*;
#[allow(unused_imports)]
use crate::step_repr::*;

#[test]
fn init_round_trip_step_fea_freedom_and_coefficient() {
    let a_freedom = StepFEADegreeOfFreedom::default();
    let a_a = StepElementMeasureOrUnspecifiedValue::default();
    let object = StepFEAFreedomAndCoefficient::new();
    object.init(&a_freedom, &a_a);
}

#[test]
fn runtime_type_step_fea_freedoms_list() {
    let object = Handle::new(StepFEAFreedomsList::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepFEAFreedomsList");
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_fea_freedoms_list() {
    let a_freedoms = Handle::new(HArray1::<StepFEADegreeOfFreedom>::with_bounds(1, 2).unwrap());
    let object = StepFEAFreedomsList::new();
    object.init(&a_freedoms);
    assert!(object.freedoms() == a_freedoms);
}

#[test]
fn runtime_type_step_fea_geometric_node() {
    let object = Handle::new(StepFEAGeometricNode::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEAGeometricNode"
    );
    assert!(object.is_kind_name("StepFEANodeRepresentation"));
}

#[test]
fn runtime_type_step_fea_node() {
    let object = Handle::new(StepFEANode::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepFEANode");
    assert!(object.is_kind_name("StepFEANodeRepresentation"));
}

#[test]
fn runtime_type_step_fea_node_definition() {
    let object = Handle::new(StepFEANodeDefinition::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEANodeDefinition"
    );
    assert!(object.is_kind_name("StepReprShapeAspect"));
}

#[test]
fn runtime_type_step_fea_node_group() {
    let object = Handle::new(StepFEANodeGroup::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepFEANodeGroup");
    assert!(object.is_kind_name("StepFEAFeaGroup"));
}

#[test]
fn init_round_trip_step_fea_node_group() {
    let a_group_name = Handle::new(HAsciiString::new("sample"));
    let a_group_description = Handle::new(HAsciiString::new("sample"));
    let a_fea_group_model_ref = Handle::new(StepFEAFeaModel::new());
    let a_nodes =
        Handle::new(HArray1::<Handle<StepFEANodeRepresentation>>::with_bounds(1, 2).unwrap());
    let object = StepFEANodeGroup::new();
    object.init(
        &a_group_name,
        &a_group_description,
        &a_fea_group_model_ref,
        &a_nodes,
    );
    assert!(object.nodes() == a_nodes);
}

#[test]
fn runtime_type_step_fea_node_representation() {
    let object = Handle::new(StepFEANodeRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEANodeRepresentation"
    );
    assert!(object.is_kind_name("StepReprRepresentation"));
}

#[test]
fn init_round_trip_step_fea_node_representation() {
    let a_representation_name = Handle::new(HAsciiString::new("sample"));
    let a_representation_items =
        Handle::new(HArray1::<Handle<StepReprRepresentationItem>>::with_bounds(1, 2).unwrap());
    let a_representation_context_of_items = Handle::new(StepReprRepresentationContext::new());
    let a_model_ref = Handle::new(StepFEAFeaModel::new());
    let object = StepFEANodeRepresentation::new();
    object.init(
        &a_representation_name,
        &a_representation_items,
        &a_representation_context_of_items,
        &a_model_ref,
    );
    assert!(object.model_ref() == a_model_ref);
}

#[test]
fn runtime_type_step_fea_node_set() {
    let object = Handle::new(StepFEANodeSet::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepFEANodeSet");
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}

#[test]
fn init_round_trip_step_fea_node_set() {
    let a_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let a_nodes =
        Handle::new(HArray1::<Handle<StepFEANodeRepresentation>>::with_bounds(1, 2).unwrap());
    let object = StepFEANodeSet::new();
    object.init(&a_representation_item_name, &a_nodes);
    assert!(object.nodes() == a_nodes);
}

#[test]
fn runtime_type_step_fea_node_with_solution_coordinate_system() {
    let object = Handle::new(StepFEANodeWithSolutionCoordinateSystem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEANodeWithSolutionCoordinateSystem"
    );
    assert!(object.is_kind_name("StepFEANode"));
}

#[test]
fn runtime_type_step_fea_node_with_vector() {
    let object = Handle::new(StepFEANodeWithVector::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEANodeWithVector"
    );
    assert!(object.is_kind_name("StepFEANode"));
}

#[test]
fn runtime_type_step_fea_parametric_curve3d_element_coordinate_direction() {
    let object = Handle::new(StepFEAParametricCurve3dElementCoordinateDirection::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEAParametricCurve3dElementCoordinateDirection"
    );
    assert!(object.is_kind_name("StepFEAFeaRepresentationItem"));
}

#[test]
fn init_round_trip_step_fea_parametric_curve3d_element_coordinate_direction() {
    let a_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let a_orientation = Handle::new(StepGeomDirection::new());
    let object = StepFEAParametricCurve3dElementCoordinateDirection::new();
    object.init(&a_representation_item_name, &a_orientation);
    assert!(object.orientation() == a_orientation);
}

#[test]
fn runtime_type_step_fea_parametric_curve3d_element_coordinate_system() {
    let object = Handle::new(StepFEAParametricCurve3dElementCoordinateSystem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEAParametricCurve3dElementCoordinateSystem"
    );
    assert!(object.is_kind_name("StepFEAFeaRepresentationItem"));
}

#[test]
fn init_round_trip_step_fea_parametric_curve3d_element_coordinate_system() {
    let a_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let a_direction = Handle::new(StepFEAParametricCurve3dElementCoordinateDirection::new());
    let object = StepFEAParametricCurve3dElementCoordinateSystem::new();
    object.init(&a_representation_item_name, &a_direction);
    assert!(object.direction() == a_direction);
}

#[test]
fn runtime_type_step_fea_parametric_surface3d_element_coordinate_system() {
    let object = Handle::new(StepFEAParametricSurface3dElementCoordinateSystem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEAParametricSurface3dElementCoordinateSystem"
    );
    assert!(object.is_kind_name("StepFEAFeaRepresentationItem"));
}

#[test]
fn init_round_trip_step_fea_parametric_surface3d_element_coordinate_system() {
    let a_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let a_axis = 7;
    let a_angle = 2.5;
    let object = StepFEAParametricSurface3dElementCoordinateSystem::new();
    object.init(&a_representation_item_name, a_axis, a_angle);
    assert!(object.axis() == a_axis);
    assert!(object.angle() == a_angle);
}

#[test]
fn runtime_type_step_fea_surface3d_element_representation() {
    let object = Handle::new(StepFEASurface3dElementRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEASurface3dElementRepresentation"
    );
    assert!(object.is_kind_name("StepFEAElementRepresentation"));
}

#[test]
fn init_round_trip_step_fea_surface3d_element_representation() {
    let a_representation_name = Handle::new(HAsciiString::new("sample"));
    let a_representation_items =
        Handle::new(HArray1::<Handle<StepReprRepresentationItem>>::with_bounds(1, 2).unwrap());
    let a_representation_context_of_items = Handle::new(StepReprRepresentationContext::new());
    let a_element_representation_node_list =
        Handle::new(HArray1::<Handle<StepFEANodeRepresentation>>::with_bounds(1, 2).unwrap());
    let a_model_ref = Handle::new(StepFEAFeaModel3d::new());
    let a_element_descriptor = Handle::new(StepElementSurface3dElementDescriptor::new());
    let a_property = Handle::new(StepElementSurfaceElementProperty::new());
    let a_material = Handle::new(StepElementElementMaterial::new());
    let object = StepFEASurface3dElementRepresentation::new();
    object.init(
        &a_representation_name,
        &a_representation_items,
        &a_representation_context_of_items,
        &a_element_representation_node_list,
        &a_model_ref,
        &a_element_descriptor,
        &a_property,
        &a_material,
    );
    assert!(object.model_ref() == a_model_ref);
    assert!(object.element_descriptor() == a_element_descriptor);
    assert!(object.property() == a_property);
    assert!(object.material() == a_material);
}

#[test]
fn runtime_type_step_fea_volume3d_element_representation() {
    let object = Handle::new(StepFEAVolume3dElementRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEAVolume3dElementRepresentation"
    );
    assert!(object.is_kind_name("StepFEAElementRepresentation"));
}

#[test]
fn init_round_trip_step_fea_volume3d_element_representation() {
    let a_representation_name = Handle::new(HAsciiString::new("sample"));
    let a_representation_items =
        Handle::new(HArray1::<Handle<StepReprRepresentationItem>>::with_bounds(1, 2).unwrap());
    let a_representation_context_of_items = Handle::new(StepReprRepresentationContext::new());
    let a_element_representation_node_list =
        Handle::new(HArray1::<Handle<StepFEANodeRepresentation>>::with_bounds(1, 2).unwrap());
    let a_model_ref = Handle::new(StepFEAFeaModel3d::new());
    let a_element_descriptor = Handle::new(StepElementVolume3dElementDescriptor::new());
    let a_material = Handle::new(StepElementElementMaterial::new());
    let object = StepFEAVolume3dElementRepresentation::new();
    object.init(
        &a_representation_name,
        &a_representation_items,
        &a_representation_context_of_items,
        &a_element_representation_node_list,
        &a_model_ref,
        &a_element_descriptor,
        &a_material,
    );
    assert!(object.model_ref() == a_model_ref);
    assert!(object.element_descriptor() == a_element_descriptor);
    assert!(object.material() == a_material);
}
