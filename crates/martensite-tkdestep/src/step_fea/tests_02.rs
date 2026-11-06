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
fn runtime_type_step_fea_fea_axis2_placement3d() {
    let object = Handle::new(StepFEAFeaAxis2Placement3d::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEAFeaAxis2Placement3d"
    );
    assert!(object.is_kind_name("StepGeomAxis2Placement3d"));
}

#[test]
fn init_round_trip_step_fea_fea_axis2_placement3d() {
    let a_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let a_placement_location = Handle::new(StepGeomCartesianPoint::new());
    let has_axis2_placement3d_axis = true;
    let a_axis2_placement3d_axis = Handle::new(StepGeomDirection::new());
    let has_axis2_placement3d_ref_direction = true;
    let a_axis2_placement3d_ref_direction = Handle::new(StepGeomDirection::new());
    let a_system_type = StepFEACoordinateSystemType::default();
    let a_description = Handle::new(HAsciiString::new("sample"));
    let object = StepFEAFeaAxis2Placement3d::new();
    object.init(
        &a_representation_item_name,
        &a_placement_location,
        has_axis2_placement3d_axis,
        &a_axis2_placement3d_axis,
        has_axis2_placement3d_ref_direction,
        &a_axis2_placement3d_ref_direction,
        a_system_type,
        &a_description,
    );
    assert!(object.system_type() == a_system_type);
    assert!(object.description() == a_description);
}

#[test]
fn runtime_type_step_fea_fea_curve_section_geometric_relationship() {
    let object = Handle::new(StepFEAFeaCurveSectionGeometricRelationship::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEAFeaCurveSectionGeometricRelationship"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_fea_fea_curve_section_geometric_relationship() {
    let a_section_ref = Handle::new(StepElementCurveElementSectionDefinition::new());
    let a_item = Handle::new(StepElementAnalysisItemWithinRepresentation::new());
    let object = StepFEAFeaCurveSectionGeometricRelationship::new();
    object.init(&a_section_ref, &a_item);
    assert!(object.section_ref() == a_section_ref);
    assert!(object.item() == a_item);
}

#[test]
fn runtime_type_step_fea_fea_group() {
    let object = Handle::new(StepFEAFeaGroup::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepFEAFeaGroup");
    assert!(object.is_kind_name("StepBasicGroup"));
}

#[test]
fn init_round_trip_step_fea_fea_group() {
    let a_group_name = Handle::new(HAsciiString::new("sample"));
    let a_group_description = Handle::new(HAsciiString::new("sample"));
    let a_model_ref = Handle::new(StepFEAFeaModel::new());
    let object = StepFEAFeaGroup::new();
    object.init(&a_group_name, &a_group_description, &a_model_ref);
    assert!(object.model_ref() == a_model_ref);
}

#[test]
fn runtime_type_step_fea_fea_linear_elasticity() {
    let object = Handle::new(StepFEAFeaLinearElasticity::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEAFeaLinearElasticity"
    );
    assert!(object.is_kind_name("StepFEAFeaMaterialPropertyRepresentationItem"));
}

#[test]
fn init_round_trip_step_fea_fea_linear_elasticity() {
    let a_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let a_fea_constants = StepFEASymmetricTensor43d::default();
    let object = StepFEAFeaLinearElasticity::new();
    object.init(&a_representation_item_name, &a_fea_constants);
}

#[test]
fn runtime_type_step_fea_fea_mass_density() {
    let object = Handle::new(StepFEAFeaMassDensity::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEAFeaMassDensity"
    );
    assert!(object.is_kind_name("StepFEAFeaMaterialPropertyRepresentationItem"));
}

#[test]
fn init_round_trip_step_fea_fea_mass_density() {
    let a_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let a_fea_constant = 2.5;
    let object = StepFEAFeaMassDensity::new();
    object.init(&a_representation_item_name, a_fea_constant);
    assert!(object.fea_constant() == a_fea_constant);
}

#[test]
fn runtime_type_step_fea_fea_material_property_representation() {
    let object = Handle::new(StepFEAFeaMaterialPropertyRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEAFeaMaterialPropertyRepresentation"
    );
    assert!(object.is_kind_name("StepReprMaterialPropertyRepresentation"));
}

#[test]
fn runtime_type_step_fea_fea_material_property_representation_item() {
    let object = Handle::new(StepFEAFeaMaterialPropertyRepresentationItem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEAFeaMaterialPropertyRepresentationItem"
    );
    assert!(object.is_kind_name("StepReprRepresentationItem"));
}

#[test]
fn runtime_type_step_fea_fea_model() {
    let object = Handle::new(StepFEAFeaModel::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepFEAFeaModel");
    assert!(object.is_kind_name("StepReprRepresentation"));
}

#[test]
fn init_round_trip_step_fea_fea_model() {
    let a_representation_name = Handle::new(HAsciiString::new("sample"));
    let a_representation_items =
        Handle::new(HArray1::<Handle<StepReprRepresentationItem>>::with_bounds(1, 2).unwrap());
    let a_representation_context_of_items = Handle::new(StepReprRepresentationContext::new());
    let a_creating_software = Handle::new(HAsciiString::new("sample"));
    let a_intended_analysis_code = Handle::new(HArray1::<AsciiString>::with_bounds(1, 2).unwrap());
    let a_description = Handle::new(HAsciiString::new("sample"));
    let a_analysis_type = Handle::new(HAsciiString::new("sample"));
    let object = StepFEAFeaModel::new();
    object.init(
        &a_representation_name,
        &a_representation_items,
        &a_representation_context_of_items,
        &a_creating_software,
        &a_intended_analysis_code,
        &a_description,
        &a_analysis_type,
    );
    assert!(object.creating_software() == a_creating_software);
    assert!(object.intended_analysis_code() == a_intended_analysis_code);
    assert!(object.description() == a_description);
    assert!(object.analysis_type() == a_analysis_type);
}

#[test]
fn runtime_type_step_fea_fea_model3d() {
    let object = Handle::new(StepFEAFeaModel3d::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepFEAFeaModel3d");
    assert!(object.is_kind_name("StepFEAFeaModel"));
}

#[test]
fn runtime_type_step_fea_fea_model_definition() {
    let object = Handle::new(StepFEAFeaModelDefinition::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEAFeaModelDefinition"
    );
    assert!(object.is_kind_name("StepReprShapeAspect"));
}

#[test]
fn runtime_type_step_fea_fea_moisture_absorption() {
    let object = Handle::new(StepFEAFeaMoistureAbsorption::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEAFeaMoistureAbsorption"
    );
    assert!(object.is_kind_name("StepFEAFeaMaterialPropertyRepresentationItem"));
}

#[test]
fn init_round_trip_step_fea_fea_moisture_absorption() {
    let a_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let a_fea_constants = StepFEASymmetricTensor23d::default();
    let object = StepFEAFeaMoistureAbsorption::new();
    object.init(&a_representation_item_name, &a_fea_constants);
}

#[test]
fn runtime_type_step_fea_fea_parametric_point() {
    let object = Handle::new(StepFEAFeaParametricPoint::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEAFeaParametricPoint"
    );
    assert!(object.is_kind_name("StepGeomPoint"));
}

#[test]
fn init_round_trip_step_fea_fea_parametric_point() {
    let a_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let a_coordinates = Handle::new(HArray1::<f64>::with_bounds(1, 2).unwrap());
    let object = StepFEAFeaParametricPoint::new();
    object.init(&a_representation_item_name, &a_coordinates);
    assert!(object.coordinates() == a_coordinates);
}

#[test]
fn runtime_type_step_fea_fea_representation_item() {
    let object = Handle::new(StepFEAFeaRepresentationItem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEAFeaRepresentationItem"
    );
    assert!(object.is_kind_name("StepReprRepresentationItem"));
}

#[test]
fn runtime_type_step_fea_fea_secant_coefficient_of_linear_thermal_expansion() {
    let object = Handle::new(StepFEAFeaSecantCoefficientOfLinearThermalExpansion::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEAFeaSecantCoefficientOfLinearThermalExpansion"
    );
    assert!(object.is_kind_name("StepFEAFeaMaterialPropertyRepresentationItem"));
}

#[test]
fn init_round_trip_step_fea_fea_secant_coefficient_of_linear_thermal_expansion() {
    let a_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let a_fea_constants = StepFEASymmetricTensor23d::default();
    let a_reference_temperature = 2.5;
    let object = StepFEAFeaSecantCoefficientOfLinearThermalExpansion::new();
    object.init(
        &a_representation_item_name,
        &a_fea_constants,
        a_reference_temperature,
    );
    assert!(object.reference_temperature() == a_reference_temperature);
}

#[test]
fn runtime_type_step_fea_fea_shell_bending_stiffness() {
    let object = Handle::new(StepFEAFeaShellBendingStiffness::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEAFeaShellBendingStiffness"
    );
    assert!(object.is_kind_name("StepFEAFeaMaterialPropertyRepresentationItem"));
}

#[test]
fn init_round_trip_step_fea_fea_shell_bending_stiffness() {
    let a_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let a_fea_constants = StepFEASymmetricTensor42d::default();
    let object = StepFEAFeaShellBendingStiffness::new();
    object.init(&a_representation_item_name, &a_fea_constants);
}

#[test]
fn runtime_type_step_fea_fea_shell_membrane_bending_coupling_stiffness() {
    let object = Handle::new(StepFEAFeaShellMembraneBendingCouplingStiffness::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEAFeaShellMembraneBendingCouplingStiffness"
    );
    assert!(object.is_kind_name("StepFEAFeaMaterialPropertyRepresentationItem"));
}

#[test]
fn init_round_trip_step_fea_fea_shell_membrane_bending_coupling_stiffness() {
    let a_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let a_fea_constants = StepFEASymmetricTensor42d::default();
    let object = StepFEAFeaShellMembraneBendingCouplingStiffness::new();
    object.init(&a_representation_item_name, &a_fea_constants);
}

#[test]
fn runtime_type_step_fea_fea_shell_membrane_stiffness() {
    let object = Handle::new(StepFEAFeaShellMembraneStiffness::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEAFeaShellMembraneStiffness"
    );
    assert!(object.is_kind_name("StepFEAFeaMaterialPropertyRepresentationItem"));
}

#[test]
fn init_round_trip_step_fea_fea_shell_membrane_stiffness() {
    let a_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let a_fea_constants = StepFEASymmetricTensor42d::default();
    let object = StepFEAFeaShellMembraneStiffness::new();
    object.init(&a_representation_item_name, &a_fea_constants);
}

#[test]
fn runtime_type_step_fea_fea_shell_shear_stiffness() {
    let object = Handle::new(StepFEAFeaShellShearStiffness::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEAFeaShellShearStiffness"
    );
    assert!(object.is_kind_name("StepFEAFeaMaterialPropertyRepresentationItem"));
}

#[test]
fn init_round_trip_step_fea_fea_shell_shear_stiffness() {
    let a_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let a_fea_constants = StepFEASymmetricTensor22d::default();
    let object = StepFEAFeaShellShearStiffness::new();
    object.init(&a_representation_item_name, &a_fea_constants);
}

#[test]
fn runtime_type_step_fea_fea_surface_section_geometric_relationship() {
    let object = Handle::new(StepFEAFeaSurfaceSectionGeometricRelationship::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEAFeaSurfaceSectionGeometricRelationship"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_fea_fea_surface_section_geometric_relationship() {
    let a_section_ref = Handle::new(StepElementSurfaceSection::new());
    let a_item = Handle::new(StepElementAnalysisItemWithinRepresentation::new());
    let object = StepFEAFeaSurfaceSectionGeometricRelationship::new();
    object.init(&a_section_ref, &a_item);
    assert!(object.section_ref() == a_section_ref);
    assert!(object.item() == a_item);
}

#[test]
fn runtime_type_step_fea_fea_tangential_coefficient_of_linear_thermal_expansion() {
    let object = Handle::new(StepFEAFeaTangentialCoefficientOfLinearThermalExpansion::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEAFeaTangentialCoefficientOfLinearThermalExpansion"
    );
    assert!(object.is_kind_name("StepFEAFeaMaterialPropertyRepresentationItem"));
}

#[test]
fn init_round_trip_step_fea_fea_tangential_coefficient_of_linear_thermal_expansion() {
    let a_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let a_fea_constants = StepFEASymmetricTensor23d::default();
    let object = StepFEAFeaTangentialCoefficientOfLinearThermalExpansion::new();
    object.init(&a_representation_item_name, &a_fea_constants);
}

#[test]
fn runtime_type_step_fea_freedom_and_coefficient() {
    let object = Handle::new(StepFEAFreedomAndCoefficient::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepFEAFreedomAndCoefficient"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}
