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
fn runtime_type_step_dim_tol_geometric_tolerance_with_maximum_tolerance() {
    let object = Handle::new(StepDimTolGeometricToleranceWithMaximumTolerance::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolGeometricToleranceWithMaximumTolerance"
    );
    assert!(object.is_kind_name("StepDimTolGeometricToleranceWithModifiers"));
}

#[test]
fn runtime_type_step_dim_tol_geometric_tolerance_with_modifiers() {
    let object = Handle::new(StepDimTolGeometricToleranceWithModifiers::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolGeometricToleranceWithModifiers"
    );
    assert!(object.is_kind_name("StepDimTolGeometricTolerance"));
}

#[test]
fn runtime_type_step_dim_tol_line_profile_tolerance() {
    let object = Handle::new(StepDimTolLineProfileTolerance::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolLineProfileTolerance"
    );
    assert!(object.is_kind_name("StepDimTolGeometricTolerance"));
}

#[test]
fn runtime_type_step_dim_tol_modified_geometric_tolerance() {
    let object = Handle::new(StepDimTolModifiedGeometricTolerance::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolModifiedGeometricTolerance"
    );
    assert!(object.is_kind_name("StepDimTolGeometricTolerance"));
}

#[test]
fn runtime_type_step_dim_tol_non_uniform_zone_definition() {
    let object = Handle::new(StepDimTolNonUniformZoneDefinition::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolNonUniformZoneDefinition"
    );
    assert!(object.is_kind_name("StepDimTolToleranceZoneDefinition"));
}

#[test]
fn runtime_type_step_dim_tol_parallelism_tolerance() {
    let object = Handle::new(StepDimTolParallelismTolerance::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolParallelismTolerance"
    );
    assert!(object.is_kind_name("StepDimTolGeometricToleranceWithDatumReference"));
}

#[test]
fn runtime_type_step_dim_tol_perpendicularity_tolerance() {
    let object = Handle::new(StepDimTolPerpendicularityTolerance::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolPerpendicularityTolerance"
    );
    assert!(object.is_kind_name("StepDimTolGeometricToleranceWithDatumReference"));
}

#[test]
fn runtime_type_step_dim_tol_placed_datum_target_feature() {
    let object = Handle::new(StepDimTolPlacedDatumTargetFeature::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolPlacedDatumTargetFeature"
    );
    assert!(object.is_kind_name("StepDimTolDatumTarget"));
}

#[test]
fn runtime_type_step_dim_tol_position_tolerance() {
    let object = Handle::new(StepDimTolPositionTolerance::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolPositionTolerance"
    );
    assert!(object.is_kind_name("StepDimTolGeometricTolerance"));
}

#[test]
fn runtime_type_step_dim_tol_projected_zone_definition() {
    let object = Handle::new(StepDimTolProjectedZoneDefinition::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolProjectedZoneDefinition"
    );
    assert!(object.is_kind_name("StepDimTolToleranceZoneDefinition"));
}

#[test]
fn init_round_trip_step_dim_tol_projected_zone_definition() {
    let the_zone = Handle::new(StepDimTolToleranceZone::new());
    let the_boundaries =
        Handle::new(HArray1::<Handle<StepReprShapeAspect>>::with_bounds(1, 2).unwrap());
    let the_projection_end = Handle::new(StepReprShapeAspect::new());
    let the_projection_length = Handle::new(StepBasicLengthMeasureWithUnit::new());
    let object = StepDimTolProjectedZoneDefinition::new();
    object.init(
        &the_zone,
        &the_boundaries,
        &the_projection_end,
        &the_projection_length,
    );
    assert!(object.projection_end() == the_projection_end);
    assert!(object.projection_length() == the_projection_length);
}

#[test]
fn runtime_type_step_dim_tol_roundness_tolerance() {
    let object = Handle::new(StepDimTolRoundnessTolerance::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolRoundnessTolerance"
    );
    assert!(object.is_kind_name("StepDimTolGeometricTolerance"));
}

#[test]
fn runtime_type_step_dim_tol_runout_zone_definition() {
    let object = Handle::new(StepDimTolRunoutZoneDefinition::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolRunoutZoneDefinition"
    );
    assert!(object.is_kind_name("StepDimTolToleranceZoneDefinition"));
}

#[test]
fn init_round_trip_step_dim_tol_runout_zone_definition() {
    let the_zone = Handle::new(StepDimTolToleranceZone::new());
    let the_boundaries =
        Handle::new(HArray1::<Handle<StepReprShapeAspect>>::with_bounds(1, 2).unwrap());
    let the_orientation = Handle::new(StepDimTolRunoutZoneOrientation::new());
    let object = StepDimTolRunoutZoneDefinition::new();
    object.init(&the_zone, &the_boundaries, &the_orientation);
    assert!(object.orientation() == the_orientation);
}

#[test]
fn runtime_type_step_dim_tol_runout_zone_orientation() {
    let object = Handle::new(StepDimTolRunoutZoneOrientation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolRunoutZoneOrientation"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_dim_tol_runout_zone_orientation() {
    let the_angle = Handle::new(StepBasicPlaneAngleMeasureWithUnit::new());
    let object = StepDimTolRunoutZoneOrientation::new();
    object.init(&the_angle);
    assert!(object.angle() == the_angle);
}

#[test]
fn runtime_type_step_dim_tol_straightness_tolerance() {
    let object = Handle::new(StepDimTolStraightnessTolerance::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolStraightnessTolerance"
    );
    assert!(object.is_kind_name("StepDimTolGeometricTolerance"));
}

#[test]
fn runtime_type_step_dim_tol_surface_profile_tolerance() {
    let object = Handle::new(StepDimTolSurfaceProfileTolerance::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolSurfaceProfileTolerance"
    );
    assert!(object.is_kind_name("StepDimTolGeometricTolerance"));
}

#[test]
fn runtime_type_step_dim_tol_symmetry_tolerance() {
    let object = Handle::new(StepDimTolSymmetryTolerance::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolSymmetryTolerance"
    );
    assert!(object.is_kind_name("StepDimTolGeometricToleranceWithDatumReference"));
}

#[test]
fn runtime_type_step_dim_tol_tolerance_zone() {
    let object = Handle::new(StepDimTolToleranceZone::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolToleranceZone"
    );
    assert!(object.is_kind_name("StepReprShapeAspect"));
}

#[test]
fn init_round_trip_step_dim_tol_tolerance_zone() {
    let the_name = Handle::new(HAsciiString::new("sample"));
    let the_description = Handle::new(HAsciiString::new("sample"));
    let the_of_shape = Handle::new(StepReprProductDefinitionShape::new());
    let the_product_definitional = StepDataLogical::default();
    let the_defining_tolerance =
        Handle::new(HArray1::<StepDimTolToleranceZoneTarget>::with_bounds(1, 2).unwrap());
    let the_form = Handle::new(StepDimTolToleranceZoneForm::new());
    let object = StepDimTolToleranceZone::new();
    object.init(
        &the_name,
        &the_description,
        &the_of_shape,
        the_product_definitional,
        &the_defining_tolerance,
        &the_form,
    );
    assert!(object.defining_tolerance() == the_defining_tolerance);
    assert!(object.form() == the_form);
}

#[test]
fn runtime_type_step_dim_tol_tolerance_zone_definition() {
    let object = Handle::new(StepDimTolToleranceZoneDefinition::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolToleranceZoneDefinition"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_dim_tol_tolerance_zone_definition() {
    let the_zone = Handle::new(StepDimTolToleranceZone::new());
    let the_boundaries =
        Handle::new(HArray1::<Handle<StepReprShapeAspect>>::with_bounds(1, 2).unwrap());
    let object = StepDimTolToleranceZoneDefinition::new();
    object.init(&the_zone, &the_boundaries);
    assert!(object.boundaries() == the_boundaries);
    assert!(object.zone() == the_zone);
}

#[test]
fn runtime_type_step_dim_tol_tolerance_zone_form() {
    let object = Handle::new(StepDimTolToleranceZoneForm::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolToleranceZoneForm"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_dim_tol_tolerance_zone_form() {
    let the_name = Handle::new(HAsciiString::new("sample"));
    let object = StepDimTolToleranceZoneForm::new();
    object.init(&the_name);
    assert!(object.name() == the_name);
}

#[test]
fn runtime_type_step_dim_tol_total_runout_tolerance() {
    let object = Handle::new(StepDimTolTotalRunoutTolerance::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolTotalRunoutTolerance"
    );
    assert!(object.is_kind_name("StepDimTolGeometricToleranceWithDatumReference"));
}

#[test]
fn runtime_type_step_dim_tol_unequally_disposed_geometric_tolerance() {
    let object = Handle::new(StepDimTolUnequallyDisposedGeometricTolerance::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepDimTolUnequallyDisposedGeometricTolerance"
    );
    assert!(object.is_kind_name("StepDimTolGeometricTolerance"));
}
