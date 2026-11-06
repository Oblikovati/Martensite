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
use crate::step_geom::*;

#[test]
fn init_round_trip_step_shape_limits_and_fits() {
    let form_variance = Handle::new(HAsciiString::new("sample"));
    let zone_variance = Handle::new(HAsciiString::new("sample"));
    let grade = Handle::new(HAsciiString::new("sample"));
    let source = Handle::new(HAsciiString::new("sample"));
    let object = StepShapeLimitsAndFits::new();
    object.init(&form_variance, &zone_variance, &grade, &source);
    assert!(object.form_variance() == form_variance);
    assert!(object.zone_variance() == zone_variance);
    assert!(object.grade() == grade);
    assert!(object.source() == source);
}

#[test]
fn runtime_type_step_shape_loop() {
    let object = Handle::new(StepShapeLoop::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepShapeLoop");
    assert!(object.is_kind_name("StepShapeTopologicalRepresentationItem"));
}

#[test]
fn runtime_type_step_shape_loop_and_path() {
    let object = Handle::new(StepShapeLoopAndPath::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeLoopAndPath"
    );
    assert!(object.is_kind_name("StepShapeTopologicalRepresentationItem"));
}

#[test]
fn init_round_trip_step_shape_loop_and_path() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_loop = Handle::new(StepShapeLoop::new());
    let a_path = Handle::new(StepShapePath::new());
    let object = StepShapeLoopAndPath::new();
    object.init(&a_name, &a_loop, &a_path);
    assert!(object.loop_() == a_loop);
    assert!(object.path() == a_path);
}

#[test]
fn runtime_type_step_shape_manifold_solid_brep() {
    let object = Handle::new(StepShapeManifoldSolidBrep::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeManifoldSolidBrep"
    );
    assert!(object.is_kind_name("StepShapeSolidModel"));
}

#[test]
fn init_round_trip_step_shape_manifold_solid_brep() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_outer = Handle::new(StepShapeClosedShell::new());
    let object = StepShapeManifoldSolidBrep::new();
    object.init(&a_name, &a_outer);
    assert!(object.outer() == a_outer);
}

#[test]
fn runtime_type_step_shape_manifold_surface_shape_representation() {
    let object = Handle::new(StepShapeManifoldSurfaceShapeRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeManifoldSurfaceShapeRepresentation"
    );
    assert!(object.is_kind_name("StepShapeShapeRepresentation"));
}

#[test]
fn runtime_type_step_shape_measure_qualification() {
    let object = Handle::new(StepShapeMeasureQualification::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeMeasureQualification"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn runtime_type_step_shape_measure_representation_item_and_qualified_representation_item() {
    let object =
        Handle::new(StepShapeMeasureRepresentationItemAndQualifiedRepresentationItem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeMeasureRepresentationItemAndQualifiedRepresentationItem"
    );
    assert!(object.is_kind_name("StepReprRepresentationItem"));
}

#[test]
fn init_round_trip_step_shape_measure_representation_item_and_qualified_representation_item() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_value_component = Handle::new(StepBasicMeasureValueMember::new());
    let a_unit_component = StepBasicUnit::default();
    let qualifiers = Handle::new(HArray1::<StepShapeValueQualifier>::with_bounds(1, 2).unwrap());
    let object = StepShapeMeasureRepresentationItemAndQualifiedRepresentationItem::new();
    object
        .init(&a_name, &a_value_component, &a_unit_component, &qualifiers)
        .unwrap();
    assert!(object.qualifiers() == qualifiers);
}

#[test]
fn runtime_type_step_shape_non_manifold_surface_shape_representation() {
    let object = Handle::new(StepShapeNonManifoldSurfaceShapeRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeNonManifoldSurfaceShapeRepresentation"
    );
    assert!(object.is_kind_name("StepShapeShapeRepresentation"));
}

#[test]
fn runtime_type_step_shape_open_shell() {
    let object = Handle::new(StepShapeOpenShell::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepShapeOpenShell");
    assert!(object.is_kind_name("StepShapeConnectedFaceSet"));
}

#[test]
fn runtime_type_step_shape_plus_minus_tolerance() {
    let object = Handle::new(StepShapePlusMinusTolerance::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapePlusMinusTolerance"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_shape_plus_minus_tolerance() {
    let range = StepShapeToleranceMethodDefinition::default();
    let toleranced_dimension = StepShapeDimensionalCharacteristic::default();
    let object = StepShapePlusMinusTolerance::new();
    object.init(&range, &toleranced_dimension);
}

#[test]
fn runtime_type_step_shape_point_representation() {
    let object = Handle::new(StepShapePointRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapePointRepresentation"
    );
    assert!(object.is_kind_name("StepShapeShapeRepresentation"));
}

#[test]
fn runtime_type_step_shape_poly_loop() {
    let object = Handle::new(StepShapePolyLoop::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepShapePolyLoop");
    assert!(object.is_kind_name("StepShapeLoop"));
}

#[test]
fn init_round_trip_step_shape_poly_loop() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_polygon =
        Handle::new(HArray1::<Handle<StepGeomCartesianPoint>>::with_bounds(1, 2).unwrap());
    let object = StepShapePolyLoop::new();
    object.init(&a_name, &a_polygon);
    assert!(object.polygon() == a_polygon);
}

#[test]
fn runtime_type_step_shape_precision_qualifier() {
    let object = Handle::new(StepShapePrecisionQualifier::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapePrecisionQualifier"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_shape_precision_qualifier() {
    let precision_value = 7;
    let object = StepShapePrecisionQualifier::new();
    object.init(precision_value);
    assert!(object.precision_value() == precision_value);
}

#[test]
fn runtime_type_step_shape_qualified_representation_item() {
    let object = Handle::new(StepShapeQualifiedRepresentationItem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeQualifiedRepresentationItem"
    );
    assert!(object.is_kind_name("StepReprRepresentationItem"));
}

#[test]
fn init_round_trip_step_shape_qualified_representation_item() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let qualifiers = Handle::new(HArray1::<StepShapeValueQualifier>::with_bounds(1, 2).unwrap());
    let object = StepShapeQualifiedRepresentationItem::new();
    object.init(&a_name, &qualifiers);
    assert!(object.qualifiers() == qualifiers);
}

#[test]
fn runtime_type_step_shape_revolved_area_solid() {
    let object = Handle::new(StepShapeRevolvedAreaSolid::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeRevolvedAreaSolid"
    );
    assert!(object.is_kind_name("StepShapeSweptAreaSolid"));
}

#[test]
fn init_round_trip_step_shape_revolved_area_solid() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_swept_area = Handle::new(StepGeomCurveBoundedSurface::new());
    let a_axis = Handle::new(StepGeomAxis1Placement::new());
    let a_angle = 2.5;
    let object = StepShapeRevolvedAreaSolid::new();
    object.init(&a_name, &a_swept_area, &a_axis, a_angle);
    assert!(object.axis() == a_axis);
    assert!(object.angle() == a_angle);
}

#[test]
fn runtime_type_step_shape_revolved_face_solid() {
    let object = Handle::new(StepShapeRevolvedFaceSolid::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeRevolvedFaceSolid"
    );
    assert!(object.is_kind_name("StepShapeSweptFaceSolid"));
}

#[test]
fn init_round_trip_step_shape_revolved_face_solid() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_swept_area = Handle::new(StepShapeFaceSurface::new());
    let object = StepShapeRevolvedFaceSolid::new();
    object.init(&a_name, &a_swept_area);
}

#[test]
fn runtime_type_step_shape_right_angular_wedge() {
    let object = Handle::new(StepShapeRightAngularWedge::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeRightAngularWedge"
    );
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}

#[test]
fn init_round_trip_step_shape_right_angular_wedge() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_position = Handle::new(StepGeomAxis2Placement3d::new());
    let a_x = 2.5;
    let a_y = 2.5;
    let a_z = 2.5;
    let a_ltx = 2.5;
    let object = StepShapeRightAngularWedge::new();
    object.init(&a_name, &a_position, a_x, a_y, a_z, a_ltx);
    assert!(object.position() == a_position);
    assert!(object.x() == a_x);
    assert!(object.y() == a_y);
    assert!(object.z() == a_z);
    assert!(object.ltx() == a_ltx);
}

#[test]
fn runtime_type_step_shape_right_circular_cone() {
    let object = Handle::new(StepShapeRightCircularCone::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeRightCircularCone"
    );
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}

#[test]
fn init_round_trip_step_shape_right_circular_cone() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_position = Handle::new(StepGeomAxis1Placement::new());
    let a_height = 2.5;
    let a_radius = 2.5;
    let a_semi_angle = 2.5;
    let object = StepShapeRightCircularCone::new();
    object.init(&a_name, &a_position, a_height, a_radius, a_semi_angle);
    assert!(object.position() == a_position);
    assert!(object.height() == a_height);
    assert!(object.radius() == a_radius);
    assert!(object.semi_angle() == a_semi_angle);
}

#[test]
fn runtime_type_step_shape_right_circular_cylinder() {
    let object = Handle::new(StepShapeRightCircularCylinder::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeRightCircularCylinder"
    );
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}

#[test]
fn init_round_trip_step_shape_right_circular_cylinder() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_position = Handle::new(StepGeomAxis1Placement::new());
    let a_height = 2.5;
    let a_radius = 2.5;
    let object = StepShapeRightCircularCylinder::new();
    object.init(&a_name, &a_position, a_height, a_radius);
    assert!(object.position() == a_position);
    assert!(object.height() == a_height);
    assert!(object.radius() == a_radius);
}

#[test]
fn runtime_type_step_shape_seam_edge() {
    let object = Handle::new(StepShapeSeamEdge::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepShapeSeamEdge");
    assert!(object.is_kind_name("StepShapeOrientedEdge"));
}

#[test]
fn init_round_trip_step_shape_seam_edge() {
    let a_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let a_oriented_edge_edge_element = Handle::new(StepShapeEdge::new());
    let a_oriented_edge_orientation = true;
    let a_pcurve_reference = Handle::new(StepGeomPcurve::new());
    let object = StepShapeSeamEdge::new();
    object.init(
        &a_representation_item_name,
        &a_oriented_edge_edge_element,
        a_oriented_edge_orientation,
        &a_pcurve_reference,
    );
    assert!(object.pcurve_reference() == a_pcurve_reference);
}
