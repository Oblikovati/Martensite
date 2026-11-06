#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_basic::*;
#[allow(unused_imports)]
use crate::step_data::*;
#[allow(unused_imports)]
use crate::step_element::*;
use crate::step_entity::*;
#[allow(unused_imports)]
use crate::step_repr::*;

#[test]
fn runtime_type_step_fea_aligned_curve3d_element_coordinate_system() {
    let object = Handle::new(StepFEAAlignedCurve3dElementCoordinateSystem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEAAlignedCurve3dElementCoordinateSystem"
    );
    assert!(object.is_kind_name("StepFEAFeaRepresentationItem"));
}

#[test]
fn init_round_trip_step_fea_aligned_curve3d_element_coordinate_system() {
    let a_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let a_coordinate_system = Handle::new(StepFEAFeaAxis2Placement3d::new());
    let object = StepFEAAlignedCurve3dElementCoordinateSystem::new();
    object.init(&a_representation_item_name, &a_coordinate_system);
    assert!(object.coordinate_system() == a_coordinate_system);
}

#[test]
fn runtime_type_step_fea_aligned_surface3d_element_coordinate_system() {
    let object = Handle::new(StepFEAAlignedSurface3dElementCoordinateSystem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEAAlignedSurface3dElementCoordinateSystem"
    );
    assert!(object.is_kind_name("StepFEAFeaRepresentationItem"));
}

#[test]
fn init_round_trip_step_fea_aligned_surface3d_element_coordinate_system() {
    let a_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let a_coordinate_system = Handle::new(StepFEAFeaAxis2Placement3d::new());
    let object = StepFEAAlignedSurface3dElementCoordinateSystem::new();
    object.init(&a_representation_item_name, &a_coordinate_system);
    assert!(object.coordinate_system() == a_coordinate_system);
}

#[test]
fn runtime_type_step_fea_arbitrary_volume3d_element_coordinate_system() {
    let object = Handle::new(StepFEAArbitraryVolume3dElementCoordinateSystem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEAArbitraryVolume3dElementCoordinateSystem"
    );
    assert!(object.is_kind_name("StepFEAFeaRepresentationItem"));
}

#[test]
fn init_round_trip_step_fea_arbitrary_volume3d_element_coordinate_system() {
    let a_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let a_coordinate_system = Handle::new(StepFEAFeaAxis2Placement3d::new());
    let object = StepFEAArbitraryVolume3dElementCoordinateSystem::new();
    object.init(&a_representation_item_name, &a_coordinate_system);
    assert!(object.coordinate_system() == a_coordinate_system);
}

#[test]
fn runtime_type_step_fea_constant_surface3d_element_coordinate_system() {
    let object = Handle::new(StepFEAConstantSurface3dElementCoordinateSystem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEAConstantSurface3dElementCoordinateSystem"
    );
    assert!(object.is_kind_name("StepFEAFeaRepresentationItem"));
}

#[test]
fn init_round_trip_step_fea_constant_surface3d_element_coordinate_system() {
    let a_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let a_axis = 7;
    let a_angle = 2.5;
    let object = StepFEAConstantSurface3dElementCoordinateSystem::new();
    object.init(&a_representation_item_name, a_axis, a_angle);
    assert!(object.axis() == a_axis);
    assert!(object.angle() == a_angle);
}

#[test]
fn runtime_type_step_fea_curve3d_element_property() {
    let object = Handle::new(StepFEACurve3dElementProperty::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEACurve3dElementProperty"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_fea_curve3d_element_property() {
    let a_property_id = Handle::new(HAsciiString::new("sample"));
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_interval_definitions =
        Handle::new(HArray1::<Handle<StepFEACurveElementInterval>>::with_bounds(1, 2).unwrap());
    let a_end_offsets =
        Handle::new(HArray1::<Handle<StepFEACurveElementEndOffset>>::with_bounds(1, 2).unwrap());
    let a_end_releases =
        Handle::new(HArray1::<Handle<StepFEACurveElementEndRelease>>::with_bounds(1, 2).unwrap());
    let object = StepFEACurve3dElementProperty::new();
    object.init(
        &a_property_id,
        &a_description,
        &a_interval_definitions,
        &a_end_offsets,
        &a_end_releases,
    );
    assert!(object.property_id() == a_property_id);
    assert!(object.description() == a_description);
    assert!(object.interval_definitions() == a_interval_definitions);
    assert!(object.end_offsets() == a_end_offsets);
    assert!(object.end_releases() == a_end_releases);
}

#[test]
fn runtime_type_step_fea_curve3d_element_representation() {
    let object = Handle::new(StepFEACurve3dElementRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEACurve3dElementRepresentation"
    );
    assert!(object.is_kind_name("StepFEAElementRepresentation"));
}

#[test]
fn init_round_trip_step_fea_curve3d_element_representation() {
    let a_representation_name = Handle::new(HAsciiString::new("sample"));
    let a_representation_items =
        Handle::new(HArray1::<Handle<StepReprRepresentationItem>>::with_bounds(1, 2).unwrap());
    let a_representation_context_of_items = Handle::new(StepReprRepresentationContext::new());
    let a_element_representation_node_list =
        Handle::new(HArray1::<Handle<StepFEANodeRepresentation>>::with_bounds(1, 2).unwrap());
    let a_model_ref = Handle::new(StepFEAFeaModel3d::new());
    let a_element_descriptor = Handle::new(StepElementCurve3dElementDescriptor::new());
    let a_property = Handle::new(StepFEACurve3dElementProperty::new());
    let a_material = Handle::new(StepElementElementMaterial::new());
    let object = StepFEACurve3dElementRepresentation::new();
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
fn runtime_type_step_fea_curve_element_end_offset() {
    let object = Handle::new(StepFEACurveElementEndOffset::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEACurveElementEndOffset"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_fea_curve_element_end_offset() {
    let a_coordinate_system = StepFEACurveElementEndCoordinateSystem::default();
    let a_offset_vector = Handle::new(HArray1::<f64>::with_bounds(1, 2).unwrap());
    let object = StepFEACurveElementEndOffset::new();
    object.init(&a_coordinate_system, &a_offset_vector);
    assert!(object.offset_vector() == a_offset_vector);
}

#[test]
fn runtime_type_step_fea_curve_element_end_release() {
    let object = Handle::new(StepFEACurveElementEndRelease::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEACurveElementEndRelease"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_fea_curve_element_end_release() {
    let a_coordinate_system = StepFEACurveElementEndCoordinateSystem::default();
    let a_releases = Handle::new(
        HArray1::<Handle<StepElementCurveElementEndReleasePacket>>::with_bounds(1, 2).unwrap(),
    );
    let object = StepFEACurveElementEndRelease::new();
    object.init(&a_coordinate_system, &a_releases);
    assert!(object.releases() == a_releases);
}

#[test]
fn runtime_type_step_fea_curve_element_interval() {
    let object = Handle::new(StepFEACurveElementInterval::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEACurveElementInterval"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_fea_curve_element_interval() {
    let a_finish_position = Handle::new(StepFEACurveElementLocation::new());
    let a_eu_angles = Handle::new(StepBasicEulerAngles::new());
    let object = StepFEACurveElementInterval::new();
    object.init(&a_finish_position, &a_eu_angles);
    assert!(object.finish_position() == a_finish_position);
    assert!(object.eu_angles() == a_eu_angles);
}

#[test]
fn runtime_type_step_fea_curve_element_interval_constant() {
    let object = Handle::new(StepFEACurveElementIntervalConstant::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEACurveElementIntervalConstant"
    );
    assert!(object.is_kind_name("StepFEACurveElementInterval"));
}

#[test]
fn init_round_trip_step_fea_curve_element_interval_constant() {
    let a_curve_element_interval_finish_position = Handle::new(StepFEACurveElementLocation::new());
    let a_curve_element_interval_eu_angles = Handle::new(StepBasicEulerAngles::new());
    let a_section = Handle::new(StepElementCurveElementSectionDefinition::new());
    let object = StepFEACurveElementIntervalConstant::new();
    object.init(
        &a_curve_element_interval_finish_position,
        &a_curve_element_interval_eu_angles,
        &a_section,
    );
    assert!(object.section() == a_section);
}

#[test]
fn runtime_type_step_fea_curve_element_interval_linearly_varying() {
    let object = Handle::new(StepFEACurveElementIntervalLinearlyVarying::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEACurveElementIntervalLinearlyVarying"
    );
    assert!(object.is_kind_name("StepFEACurveElementInterval"));
}

#[test]
fn init_round_trip_step_fea_curve_element_interval_linearly_varying() {
    let a_curve_element_interval_finish_position = Handle::new(StepFEACurveElementLocation::new());
    let a_curve_element_interval_eu_angles = Handle::new(StepBasicEulerAngles::new());
    let a_sections = Handle::new(
        HArray1::<Handle<StepElementCurveElementSectionDefinition>>::with_bounds(1, 2).unwrap(),
    );
    let object = StepFEACurveElementIntervalLinearlyVarying::new();
    object.init(
        &a_curve_element_interval_finish_position,
        &a_curve_element_interval_eu_angles,
        &a_sections,
    );
    assert!(object.sections() == a_sections);
}

#[test]
fn runtime_type_step_fea_curve_element_location() {
    let object = Handle::new(StepFEACurveElementLocation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEACurveElementLocation"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_fea_curve_element_location() {
    let a_coordinate = Handle::new(StepFEAFeaParametricPoint::new());
    let object = StepFEACurveElementLocation::new();
    object.init(&a_coordinate);
    assert!(object.coordinate() == a_coordinate);
}

#[test]
fn runtime_type_step_fea_dummy_node() {
    let object = Handle::new(StepFEADummyNode::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepFEADummyNode");
    assert!(object.is_kind_name("StepFEANodeRepresentation"));
}

#[test]
fn runtime_type_step_fea_element_geometric_relationship() {
    let object = Handle::new(StepFEAElementGeometricRelationship::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEAElementGeometricRelationship"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_fea_element_geometric_relationship() {
    let a_element_ref = StepFEAElementOrElementGroup::default();
    let a_item = Handle::new(StepElementAnalysisItemWithinRepresentation::new());
    let a_aspect = StepElementElementAspect::default();
    let object = StepFEAElementGeometricRelationship::new();
    object.init(&a_element_ref, &a_item, &a_aspect);
    assert!(object.item() == a_item);
}

#[test]
fn runtime_type_step_fea_element_group() {
    let object = Handle::new(StepFEAElementGroup::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepFEAElementGroup");
    assert!(object.is_kind_name("StepFEAFeaGroup"));
}

#[test]
fn init_round_trip_step_fea_element_group() {
    let a_group_name = Handle::new(HAsciiString::new("sample"));
    let a_group_description = Handle::new(HAsciiString::new("sample"));
    let a_fea_group_model_ref = Handle::new(StepFEAFeaModel::new());
    let a_elements =
        Handle::new(HArray1::<Handle<StepFEAElementRepresentation>>::with_bounds(1, 2).unwrap());
    let object = StepFEAElementGroup::new();
    object.init(
        &a_group_name,
        &a_group_description,
        &a_fea_group_model_ref,
        &a_elements,
    );
    assert!(object.elements() == a_elements);
}

#[test]
fn runtime_type_step_fea_element_representation() {
    let object = Handle::new(StepFEAElementRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEAElementRepresentation"
    );
    assert!(object.is_kind_name("StepReprRepresentation"));
}

#[test]
fn init_round_trip_step_fea_element_representation() {
    let a_representation_name = Handle::new(HAsciiString::new("sample"));
    let a_representation_items =
        Handle::new(HArray1::<Handle<StepReprRepresentationItem>>::with_bounds(1, 2).unwrap());
    let a_representation_context_of_items = Handle::new(StepReprRepresentationContext::new());
    let a_node_list =
        Handle::new(HArray1::<Handle<StepFEANodeRepresentation>>::with_bounds(1, 2).unwrap());
    let object = StepFEAElementRepresentation::new();
    object.init(
        &a_representation_name,
        &a_representation_items,
        &a_representation_context_of_items,
        &a_node_list,
    );
    assert!(object.node_list() == a_node_list);
}

#[test]
fn runtime_type_step_fea_fea_area_density() {
    let object = Handle::new(StepFEAFeaAreaDensity::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEAFeaAreaDensity"
    );
    assert!(object.is_kind_name("StepFEAFeaMaterialPropertyRepresentationItem"));
}

#[test]
fn init_round_trip_step_fea_fea_area_density() {
    let a_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let a_fea_constant = 2.5;
    let object = StepFEAFeaAreaDensity::new();
    object.init(&a_representation_item_name, a_fea_constant);
    assert!(object.fea_constant() == a_fea_constant);
}
