#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_basic::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;
#[allow(unused_imports)]
use crate::step_repr::*;

#[test]
fn runtime_type_step_dim_tol_angularity_tolerance() {
    let object = Handle::new(StepDimTolAngularityTolerance::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolAngularityTolerance"
    );
    assert!(object.is_kind_name("StepDimTolGeometricToleranceWithDatumReference"));
}

#[test]
fn runtime_type_step_dim_tol_circular_runout_tolerance() {
    let object = Handle::new(StepDimTolCircularRunoutTolerance::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolCircularRunoutTolerance"
    );
    assert!(object.is_kind_name("StepDimTolGeometricToleranceWithDatumReference"));
}

#[test]
fn runtime_type_step_dim_tol_coaxiality_tolerance() {
    let object = Handle::new(StepDimTolCoaxialityTolerance::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolCoaxialityTolerance"
    );
    assert!(object.is_kind_name("StepDimTolGeometricToleranceWithDatumReference"));
}

#[test]
fn runtime_type_step_dim_tol_common_datum() {
    let object = Handle::new(StepDimTolCommonDatum::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolCommonDatum"
    );
    assert!(object.is_kind_name("StepReprCompositeShapeAspect"));
}

#[test]
fn init_round_trip_step_dim_tol_common_datum() {
    let the_shape_aspect_name = Handle::new(HAsciiString::new("sample"));
    let the_shape_aspect_description = Handle::new(HAsciiString::new("sample"));
    let the_shape_aspect_of_shape = Handle::new(StepReprProductDefinitionShape::new());
    let the_shape_aspect_product_definitional = StepDataLogical::default();
    let the_datum_name = Handle::new(HAsciiString::new("sample"));
    let the_datum_description = Handle::new(HAsciiString::new("sample"));
    let the_datum_of_shape = Handle::new(StepReprProductDefinitionShape::new());
    let the_datum_product_definitional = StepDataLogical::default();
    let the_datum_identification = Handle::new(HAsciiString::new("sample"));
    let object = StepDimTolCommonDatum::new();
    object.init(
        &the_shape_aspect_name,
        &the_shape_aspect_description,
        &the_shape_aspect_of_shape,
        the_shape_aspect_product_definitional,
        &the_datum_name,
        &the_datum_description,
        &the_datum_of_shape,
        the_datum_product_definitional,
        &the_datum_identification,
    );
}

#[test]
fn runtime_type_step_dim_tol_concentricity_tolerance() {
    let object = Handle::new(StepDimTolConcentricityTolerance::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolConcentricityTolerance"
    );
    assert!(object.is_kind_name("StepDimTolGeometricToleranceWithDatumReference"));
}

#[test]
fn runtime_type_step_dim_tol_cylindricity_tolerance() {
    let object = Handle::new(StepDimTolCylindricityTolerance::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolCylindricityTolerance"
    );
    assert!(object.is_kind_name("StepDimTolGeometricTolerance"));
}

#[test]
fn runtime_type_step_dim_tol_datum() {
    let object = Handle::new(StepDimTolDatum::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepDimTolDatum");
    assert!(object.is_kind_name("StepReprShapeAspect"));
}

#[test]
fn init_round_trip_step_dim_tol_datum() {
    let the_shape_aspect_name = Handle::new(HAsciiString::new("sample"));
    let the_shape_aspect_description = Handle::new(HAsciiString::new("sample"));
    let the_shape_aspect_of_shape = Handle::new(StepReprProductDefinitionShape::new());
    let the_shape_aspect_product_definitional = StepDataLogical::default();
    let the_identification = Handle::new(HAsciiString::new("sample"));
    let object = StepDimTolDatum::new();
    object.init(
        &the_shape_aspect_name,
        &the_shape_aspect_description,
        &the_shape_aspect_of_shape,
        the_shape_aspect_product_definitional,
        &the_identification,
    );
    assert!(object.identification() == the_identification);
}

#[test]
fn runtime_type_step_dim_tol_datum_feature() {
    let object = Handle::new(StepDimTolDatumFeature::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolDatumFeature"
    );
    assert!(object.is_kind_name("StepReprShapeAspect"));
}

#[test]
fn runtime_type_step_dim_tol_datum_reference() {
    let object = Handle::new(StepDimTolDatumReference::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolDatumReference"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_dim_tol_datum_reference() {
    let the_precedence = 7;
    let the_referenced_datum = Handle::new(StepDimTolDatum::new());
    let object = StepDimTolDatumReference::new();
    object.init(the_precedence, &the_referenced_datum);
    assert!(object.precedence() == the_precedence);
    assert!(object.referenced_datum() == the_referenced_datum);
}

#[test]
fn runtime_type_step_dim_tol_datum_reference_compartment() {
    let object = Handle::new(StepDimTolDatumReferenceCompartment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolDatumReferenceCompartment"
    );
    assert!(object.is_kind_name("StepDimTolGeneralDatumReference"));
}

#[test]
fn runtime_type_step_dim_tol_datum_reference_element() {
    let object = Handle::new(StepDimTolDatumReferenceElement::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolDatumReferenceElement"
    );
    assert!(object.is_kind_name("StepDimTolGeneralDatumReference"));
}

#[test]
fn runtime_type_step_dim_tol_datum_reference_modifier_with_value() {
    let object = Handle::new(StepDimTolDatumReferenceModifierWithValue::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolDatumReferenceModifierWithValue"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_dim_tol_datum_reference_modifier_with_value() {
    let the_modifier_type = StepDimTolDatumReferenceModifierType::default();
    let the_modifier_value = Handle::new(StepBasicLengthMeasureWithUnit::new());
    let object = StepDimTolDatumReferenceModifierWithValue::new();
    object.init(the_modifier_type, &the_modifier_value);
    assert!(object.modifier_type() == the_modifier_type);
    assert!(object.modifier_value() == the_modifier_value);
}

#[test]
fn runtime_type_step_dim_tol_datum_system() {
    let object = Handle::new(StepDimTolDatumSystem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolDatumSystem"
    );
    assert!(object.is_kind_name("StepReprShapeAspect"));
}

#[test]
fn init_round_trip_step_dim_tol_datum_system() {
    let the_name = Handle::new(HAsciiString::new("sample"));
    let the_description = Handle::new(HAsciiString::new("sample"));
    let the_of_shape = Handle::new(StepReprProductDefinitionShape::new());
    let the_product_definitional = StepDataLogical::default();
    let the_constituents = Handle::new(
        HArray1::<Handle<StepDimTolDatumReferenceCompartment>>::with_bounds(1, 2).unwrap(),
    );
    let object = StepDimTolDatumSystem::new();
    object.init(
        &the_name,
        &the_description,
        &the_of_shape,
        the_product_definitional,
        &the_constituents,
    );
    assert!(object.constituents() == the_constituents);
}

#[test]
fn runtime_type_step_dim_tol_datum_target() {
    let object = Handle::new(StepDimTolDatumTarget::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolDatumTarget"
    );
    assert!(object.is_kind_name("StepReprShapeAspect"));
}

#[test]
fn init_round_trip_step_dim_tol_datum_target() {
    let the_shape_aspect_name = Handle::new(HAsciiString::new("sample"));
    let the_shape_aspect_description = Handle::new(HAsciiString::new("sample"));
    let the_shape_aspect_of_shape = Handle::new(StepReprProductDefinitionShape::new());
    let the_shape_aspect_product_definitional = StepDataLogical::default();
    let the_target_id = Handle::new(HAsciiString::new("sample"));
    let object = StepDimTolDatumTarget::new();
    object.init(
        &the_shape_aspect_name,
        &the_shape_aspect_description,
        &the_shape_aspect_of_shape,
        the_shape_aspect_product_definitional,
        &the_target_id,
    );
    assert!(object.target_id() == the_target_id);
}

#[test]
fn runtime_type_step_dim_tol_flatness_tolerance() {
    let object = Handle::new(StepDimTolFlatnessTolerance::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolFlatnessTolerance"
    );
    assert!(object.is_kind_name("StepDimTolGeometricTolerance"));
}

#[test]
fn runtime_type_step_dim_tol_general_datum_reference() {
    let object = Handle::new(StepDimTolGeneralDatumReference::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolGeneralDatumReference"
    );
    assert!(object.is_kind_name("StepReprShapeAspect"));
}

#[test]
fn init_round_trip_step_dim_tol_general_datum_reference() {
    let the_name = Handle::new(HAsciiString::new("sample"));
    let the_description = Handle::new(HAsciiString::new("sample"));
    let the_of_shape = Handle::new(StepReprProductDefinitionShape::new());
    let the_product_definitional = StepDataLogical::default();
    let the_base = StepDimTolDatumOrCommonDatum::default();
    let the_has_modifiers = true;
    let the_modifiers =
        Handle::new(HArray1::<StepDimTolDatumReferenceModifier>::with_bounds(1, 2).unwrap());
    let object = StepDimTolGeneralDatumReference::new();
    object.init(
        &the_name,
        &the_description,
        &the_of_shape,
        the_product_definitional,
        &the_base,
        the_has_modifiers,
        &the_modifiers,
    );
    assert!(object.modifiers() == the_modifiers);
}

#[test]
fn runtime_type_step_dim_tol_geo_tol_and_geo_tol_wth_dat_ref() {
    let object = Handle::new(StepDimTolGeoTolAndGeoTolWthDatRef::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolGeoTolAndGeoTolWthDatRef"
    );
    assert!(object.is_kind_name("StepDimTolGeometricTolerance"));
}

#[test]
fn runtime_type_step_dim_tol_geo_tol_and_geo_tol_wth_dat_ref_and_geo_tol_wth_max_tol() {
    let object = Handle::new(StepDimTolGeoTolAndGeoTolWthDatRefAndGeoTolWthMaxTol::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolGeoTolAndGeoTolWthDatRefAndGeoTolWthMaxTol"
    );
    assert!(object.is_kind_name("StepDimTolGeoTolAndGeoTolWthDatRefAndGeoTolWthMod"));
}

#[test]
fn runtime_type_step_dim_tol_geo_tol_and_geo_tol_wth_dat_ref_and_geo_tol_wth_mod() {
    let object = Handle::new(StepDimTolGeoTolAndGeoTolWthDatRefAndGeoTolWthMod::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolGeoTolAndGeoTolWthDatRefAndGeoTolWthMod"
    );
    assert!(object.is_kind_name("StepDimTolGeometricTolerance"));
}

#[test]
fn runtime_type_step_dim_tol_geo_tol_and_geo_tol_wth_dat_ref_and_mod_geo_tol_and_pos_tol() {
    let object = Handle::new(StepDimTolGeoTolAndGeoTolWthDatRefAndModGeoTolAndPosTol::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolGeoTolAndGeoTolWthDatRefAndModGeoTolAndPosTol"
    );
    assert!(object.is_kind_name("StepDimTolGeometricTolerance"));
}

#[test]
fn runtime_type_step_dim_tol_geo_tol_and_geo_tol_wth_dat_ref_and_uneq_dis_geo_tol() {
    let object = Handle::new(StepDimTolGeoTolAndGeoTolWthDatRefAndUneqDisGeoTol::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolGeoTolAndGeoTolWthDatRefAndUneqDisGeoTol"
    );
    assert!(object.is_kind_name("StepDimTolGeoTolAndGeoTolWthDatRef"));
}

#[test]
fn runtime_type_step_dim_tol_geo_tol_and_geo_tol_wth_max_tol() {
    let object = Handle::new(StepDimTolGeoTolAndGeoTolWthMaxTol::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolGeoTolAndGeoTolWthMaxTol"
    );
    assert!(object.is_kind_name("StepDimTolGeoTolAndGeoTolWthMod"));
}

#[test]
fn runtime_type_step_dim_tol_geo_tol_and_geo_tol_wth_mod() {
    let object = Handle::new(StepDimTolGeoTolAndGeoTolWthMod::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolGeoTolAndGeoTolWthMod"
    );
    assert!(object.is_kind_name("StepDimTolGeometricTolerance"));
}

#[test]
fn runtime_type_step_dim_tol_geometric_tolerance() {
    let object = Handle::new(StepDimTolGeometricTolerance::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolGeometricTolerance"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn runtime_type_step_dim_tol_geometric_tolerance_relationship() {
    let object = Handle::new(StepDimTolGeometricToleranceRelationship::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolGeometricToleranceRelationship"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_dim_tol_geometric_tolerance_relationship() {
    let the_name = Handle::new(HAsciiString::new("sample"));
    let the_description = Handle::new(HAsciiString::new("sample"));
    let the_relating_geometric_tolerance = Handle::new(StepDimTolGeometricTolerance::new());
    let the_related_geometric_tolerance = Handle::new(StepDimTolGeometricTolerance::new());
    let object = StepDimTolGeometricToleranceRelationship::new();
    object.init(
        &the_name,
        &the_description,
        &the_relating_geometric_tolerance,
        &the_related_geometric_tolerance,
    );
    assert!(object.name() == the_name);
    assert!(object.description() == the_description);
    assert!(object.relating_geometric_tolerance() == the_relating_geometric_tolerance);
    assert!(object.related_geometric_tolerance() == the_related_geometric_tolerance);
}

#[test]
fn runtime_type_step_dim_tol_geometric_tolerance_with_datum_reference() {
    let object = Handle::new(StepDimTolGeometricToleranceWithDatumReference::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolGeometricToleranceWithDatumReference"
    );
    assert!(object.is_kind_name("StepDimTolGeometricTolerance"));
}

#[test]
fn runtime_type_step_dim_tol_geometric_tolerance_with_defined_area_unit() {
    let object = Handle::new(StepDimTolGeometricToleranceWithDefinedAreaUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolGeometricToleranceWithDefinedAreaUnit"
    );
    assert!(object.is_kind_name("StepDimTolGeometricToleranceWithDefinedUnit"));
}

#[test]
fn runtime_type_step_dim_tol_geometric_tolerance_with_defined_unit() {
    let object = Handle::new(StepDimTolGeometricToleranceWithDefinedUnit::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolGeometricToleranceWithDefinedUnit"
    );
    assert!(object.is_kind_name("StepDimTolGeometricTolerance"));
}
