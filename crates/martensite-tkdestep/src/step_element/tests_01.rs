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
fn runtime_type_step_element_analysis_item_within_representation() {
    let object = Handle::new(StepElementAnalysisItemWithinRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepElementAnalysisItemWithinRepresentation"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_element_analysis_item_within_representation() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_item = Handle::new(StepReprRepresentationItem::new());
    let a_rep = Handle::new(StepReprRepresentation::new());
    let object = StepElementAnalysisItemWithinRepresentation::new();
    object.init(&a_name, &a_description, &a_item, &a_rep);
    assert!(object.name() == a_name);
    assert!(object.description() == a_description);
    assert!(object.item() == a_item);
    assert!(object.rep() == a_rep);
}

#[test]
fn runtime_type_step_element_curve3d_element_descriptor() {
    let object = Handle::new(StepElementCurve3dElementDescriptor::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepElementCurve3dElementDescriptor"
    );
    assert!(object.is_kind_name("StepElementElementDescriptor"));
}

#[test]
fn init_round_trip_step_element_curve3d_element_descriptor() {
    let a_element_descriptor_topology_order = StepElementElementOrder::default();
    let a_element_descriptor_description = Handle::new(HAsciiString::new("sample"));
    let a_purpose = Handle::new(
        HArray1::<Handle<HSequence<Handle<StepElementCurveElementPurposeMember>>>>::with_bounds(
            1, 2,
        )
        .unwrap(),
    );
    let object = StepElementCurve3dElementDescriptor::new();
    object.init(
        a_element_descriptor_topology_order,
        &a_element_descriptor_description,
        &a_purpose,
    );
    assert!(object.purpose() == a_purpose);
}

#[test]
fn runtime_type_step_element_curve_element_end_release_packet() {
    let object = Handle::new(StepElementCurveElementEndReleasePacket::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepElementCurveElementEndReleasePacket"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_element_curve_element_end_release_packet() {
    let a_release_freedom = StepElementCurveElementFreedom::default();
    let a_release_stiffness = 2.5;
    let object = StepElementCurveElementEndReleasePacket::new();
    object.init(&a_release_freedom, a_release_stiffness);
    assert!(object.release_stiffness() == a_release_stiffness);
}

#[test]
fn runtime_type_step_element_curve_element_section_definition() {
    let object = Handle::new(StepElementCurveElementSectionDefinition::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepElementCurveElementSectionDefinition"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_element_curve_element_section_definition() {
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_section_angle = 2.5;
    let object = StepElementCurveElementSectionDefinition::new();
    object.init(&a_description, a_section_angle);
    assert!(object.description() == a_description);
    assert!(object.section_angle() == a_section_angle);
}

#[test]
fn runtime_type_step_element_curve_element_section_derived_definitions() {
    let object = Handle::new(StepElementCurveElementSectionDerivedDefinitions::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepElementCurveElementSectionDerivedDefinitions"
    );
    assert!(object.is_kind_name("StepElementCurveElementSectionDefinition"));
}

#[test]
fn init_round_trip_step_element_curve_element_section_derived_definitions() {
    let a_curve_element_section_definition_description = Handle::new(HAsciiString::new("sample"));
    let a_curve_element_section_definition_section_angle = 2.5;
    let a_cross_sectional_area = 2.5;
    let a_shear_area =
        Handle::new(HArray1::<StepElementMeasureOrUnspecifiedValue>::with_bounds(1, 2).unwrap());
    let a_second_moment_of_area = Handle::new(HArray1::<f64>::with_bounds(1, 2).unwrap());
    let a_torsional_constant = 2.5;
    let a_warping_constant = StepElementMeasureOrUnspecifiedValue::default();
    let a_location_of_centroid =
        Handle::new(HArray1::<StepElementMeasureOrUnspecifiedValue>::with_bounds(1, 2).unwrap());
    let a_location_of_shear_centre =
        Handle::new(HArray1::<StepElementMeasureOrUnspecifiedValue>::with_bounds(1, 2).unwrap());
    let a_location_of_non_structural_mass =
        Handle::new(HArray1::<StepElementMeasureOrUnspecifiedValue>::with_bounds(1, 2).unwrap());
    let a_non_structural_mass = StepElementMeasureOrUnspecifiedValue::default();
    let a_polar_moment = StepElementMeasureOrUnspecifiedValue::default();
    let object = StepElementCurveElementSectionDerivedDefinitions::new();
    object.init(
        &a_curve_element_section_definition_description,
        a_curve_element_section_definition_section_angle,
        a_cross_sectional_area,
        &a_shear_area,
        &a_second_moment_of_area,
        a_torsional_constant,
        &a_warping_constant,
        &a_location_of_centroid,
        &a_location_of_shear_centre,
        &a_location_of_non_structural_mass,
        &a_non_structural_mass,
        &a_polar_moment,
    );
    assert!(object.cross_sectional_area() == a_cross_sectional_area);
    assert!(object.shear_area() == a_shear_area);
    assert!(object.second_moment_of_area() == a_second_moment_of_area);
    assert!(object.torsional_constant() == a_torsional_constant);
    assert!(object.location_of_centroid() == a_location_of_centroid);
    assert!(object.location_of_shear_centre() == a_location_of_shear_centre);
    assert!(object.location_of_non_structural_mass() == a_location_of_non_structural_mass);
}

#[test]
fn runtime_type_step_element_element_descriptor() {
    let object = Handle::new(StepElementElementDescriptor::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepElementElementDescriptor"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_element_element_descriptor() {
    let a_topology_order = StepElementElementOrder::default();
    let a_description = Handle::new(HAsciiString::new("sample"));
    let object = StepElementElementDescriptor::new();
    object.init(a_topology_order, &a_description);
    assert!(object.topology_order() == a_topology_order);
    assert!(object.description() == a_description);
}

#[test]
fn runtime_type_step_element_element_material() {
    let object = Handle::new(StepElementElementMaterial::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepElementElementMaterial"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_element_element_material() {
    let a_material_id = Handle::new(HAsciiString::new("sample"));
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_properties = Handle::new(
        HArray1::<Handle<StepReprMaterialPropertyRepresentation>>::with_bounds(1, 2).unwrap(),
    );
    let object = StepElementElementMaterial::new();
    object.init(&a_material_id, &a_description, &a_properties);
    assert!(object.material_id() == a_material_id);
    assert!(object.description() == a_description);
    assert!(object.properties() == a_properties);
}

#[test]
fn runtime_type_step_element_surface3d_element_descriptor() {
    let object = Handle::new(StepElementSurface3dElementDescriptor::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepElementSurface3dElementDescriptor"
    );
    assert!(object.is_kind_name("StepElementElementDescriptor"));
}

#[test]
fn init_round_trip_step_element_surface3d_element_descriptor() {
    let a_element_descriptor_topology_order = StepElementElementOrder::default();
    let a_element_descriptor_description = Handle::new(HAsciiString::new("sample"));
    let a_purpose = Handle::new(
        HArray1::<Handle<HSequence<Handle<StepElementSurfaceElementPurposeMember>>>>::with_bounds(
            1, 2,
        )
        .unwrap(),
    );
    let a_shape = StepElementElement2dShape::default();
    let object = StepElementSurface3dElementDescriptor::new();
    object.init(
        a_element_descriptor_topology_order,
        &a_element_descriptor_description,
        &a_purpose,
        a_shape,
    );
    assert!(object.purpose() == a_purpose);
    assert!(object.shape() == a_shape);
}

#[test]
fn runtime_type_step_element_surface_element_property() {
    let object = Handle::new(StepElementSurfaceElementProperty::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepElementSurfaceElementProperty"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_element_surface_element_property() {
    let a_property_id = Handle::new(HAsciiString::new("sample"));
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_section = Handle::new(StepElementSurfaceSectionField::new());
    let object = StepElementSurfaceElementProperty::new();
    object.init(&a_property_id, &a_description, &a_section);
    assert!(object.property_id() == a_property_id);
    assert!(object.description() == a_description);
    assert!(object.section() == a_section);
}

#[test]
fn runtime_type_step_element_surface_section() {
    let object = Handle::new(StepElementSurfaceSection::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepElementSurfaceSection"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_element_surface_section() {
    let a_offset = StepElementMeasureOrUnspecifiedValue::default();
    let a_non_structural_mass = StepElementMeasureOrUnspecifiedValue::default();
    let a_non_structural_mass_offset = StepElementMeasureOrUnspecifiedValue::default();
    let object = StepElementSurfaceSection::new();
    object.init(
        &a_offset,
        &a_non_structural_mass,
        &a_non_structural_mass_offset,
    );
}

#[test]
fn runtime_type_step_element_surface_section_field() {
    let object = Handle::new(StepElementSurfaceSectionField::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepElementSurfaceSectionField"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn runtime_type_step_element_surface_section_field_constant() {
    let object = Handle::new(StepElementSurfaceSectionFieldConstant::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepElementSurfaceSectionFieldConstant"
    );
    assert!(object.is_kind_name("StepElementSurfaceSectionField"));
}

#[test]
fn init_round_trip_step_element_surface_section_field_constant() {
    let a_definition = Handle::new(StepElementSurfaceSection::new());
    let object = StepElementSurfaceSectionFieldConstant::new();
    object.init(&a_definition);
    assert!(object.definition() == a_definition);
}

#[test]
fn runtime_type_step_element_surface_section_field_varying() {
    let object = Handle::new(StepElementSurfaceSectionFieldVarying::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepElementSurfaceSectionFieldVarying"
    );
    assert!(object.is_kind_name("StepElementSurfaceSectionField"));
}

#[test]
fn init_round_trip_step_element_surface_section_field_varying() {
    let a_definitions =
        Handle::new(HArray1::<Handle<StepElementSurfaceSection>>::with_bounds(1, 2).unwrap());
    let a_additional_node_values = true;
    let object = StepElementSurfaceSectionFieldVarying::new();
    object.init(&a_definitions, a_additional_node_values);
    assert!(object.definitions() == a_definitions);
    assert!(object.additional_node_values() == a_additional_node_values);
}

#[test]
fn runtime_type_step_element_uniform_surface_section() {
    let object = Handle::new(StepElementUniformSurfaceSection::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepElementUniformSurfaceSection"
    );
    assert!(object.is_kind_name("StepElementSurfaceSection"));
}

#[test]
fn init_round_trip_step_element_uniform_surface_section() {
    let a_surface_section_offset = StepElementMeasureOrUnspecifiedValue::default();
    let a_surface_section_non_structural_mass = StepElementMeasureOrUnspecifiedValue::default();
    let a_surface_section_non_structural_mass_offset =
        StepElementMeasureOrUnspecifiedValue::default();
    let a_thickness = 2.5;
    let a_bending_thickness = StepElementMeasureOrUnspecifiedValue::default();
    let a_shear_thickness = StepElementMeasureOrUnspecifiedValue::default();
    let object = StepElementUniformSurfaceSection::new();
    object.init(
        &a_surface_section_offset,
        &a_surface_section_non_structural_mass,
        &a_surface_section_non_structural_mass_offset,
        a_thickness,
        &a_bending_thickness,
        &a_shear_thickness,
    );
    assert!(object.thickness() == a_thickness);
}

#[test]
fn runtime_type_step_element_volume3d_element_descriptor() {
    let object = Handle::new(StepElementVolume3dElementDescriptor::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepElementVolume3dElementDescriptor"
    );
    assert!(object.is_kind_name("StepElementElementDescriptor"));
}

#[test]
fn init_round_trip_step_element_volume3d_element_descriptor() {
    let a_element_descriptor_topology_order = StepElementElementOrder::default();
    let a_element_descriptor_description = Handle::new(HAsciiString::new("sample"));
    let a_purpose = Handle::new(
        HArray1::<Handle<StepElementVolumeElementPurposeMember>>::with_bounds(1, 2).unwrap(),
    );
    let a_shape = StepElementVolume3dElementShape::default();
    let object = StepElementVolume3dElementDescriptor::new();
    object.init(
        a_element_descriptor_topology_order,
        &a_element_descriptor_description,
        &a_purpose,
        a_shape,
    );
    assert!(object.purpose() == a_purpose);
    assert!(object.shape() == a_shape);
}
