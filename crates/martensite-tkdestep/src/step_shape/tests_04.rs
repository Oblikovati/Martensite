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
fn runtime_type_step_shape_shape_definition_representation() {
    let object = Handle::new(StepShapeShapeDefinitionRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeShapeDefinitionRepresentation"
    );
    assert!(object.is_kind_name("StepReprPropertyDefinitionRepresentation"));
}

#[test]
fn runtime_type_step_shape_shape_dimension_representation() {
    let object = Handle::new(StepShapeShapeDimensionRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeShapeDimensionRepresentation"
    );
    assert!(object.is_kind_name("StepShapeShapeRepresentation"));
}

#[test]
fn init_round_trip_step_shape_shape_dimension_representation() {
    let the_name = Handle::new(HAsciiString::new("sample"));
    let the_items =
        Handle::new(HArray1::<Handle<StepReprRepresentationItem>>::with_bounds(1, 2).unwrap());
    let the_context_of_items = Handle::new(StepReprRepresentationContext::new());
    let object = StepShapeShapeDimensionRepresentation::new();
    object.init(&the_name, &the_items, &the_context_of_items);
}

#[test]
fn runtime_type_step_shape_shape_representation() {
    let object = Handle::new(StepShapeShapeRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeShapeRepresentation"
    );
    assert!(object.is_kind_name("StepReprRepresentation"));
}

#[test]
fn runtime_type_step_shape_shape_representation_with_parameters() {
    let object = Handle::new(StepShapeShapeRepresentationWithParameters::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeShapeRepresentationWithParameters"
    );
    assert!(object.is_kind_name("StepShapeShapeRepresentation"));
}

#[test]
fn runtime_type_step_shape_shell_based_surface_model() {
    let object = Handle::new(StepShapeShellBasedSurfaceModel::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeShellBasedSurfaceModel"
    );
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}

#[test]
fn init_round_trip_step_shape_shell_based_surface_model() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_sbsm_boundary = Handle::new(HArray1::<StepShapeShell>::with_bounds(1, 2).unwrap());
    let object = StepShapeShellBasedSurfaceModel::new();
    object.init(&a_name, &a_sbsm_boundary);
    assert!(object.sbsm_boundary() == a_sbsm_boundary);
}

#[test]
fn runtime_type_step_shape_solid_model() {
    let object = Handle::new(StepShapeSolidModel::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepShapeSolidModel");
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}

#[test]
fn runtime_type_step_shape_solid_replica() {
    let object = Handle::new(StepShapeSolidReplica::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeSolidReplica"
    );
    assert!(object.is_kind_name("StepShapeSolidModel"));
}

#[test]
fn init_round_trip_step_shape_solid_replica() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_parent_solid = Handle::new(StepShapeSolidModel::new());
    let a_transformation = Handle::new(StepGeomCartesianTransformationOperator3d::new());
    let object = StepShapeSolidReplica::new();
    object.init(&a_name, &a_parent_solid, &a_transformation);
    assert!(object.parent_solid() == a_parent_solid);
    assert!(object.transformation() == a_transformation);
}

#[test]
fn runtime_type_step_shape_sphere() {
    let object = Handle::new(StepShapeSphere::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepShapeSphere");
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}

#[test]
fn init_round_trip_step_shape_sphere() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_radius = 2.5;
    let a_centre = Handle::new(StepGeomPoint::new());
    let object = StepShapeSphere::new();
    object.init(&a_name, a_radius, &a_centre);
    assert!(object.radius() == a_radius);
    assert!(object.centre() == a_centre);
}

#[test]
fn runtime_type_step_shape_subedge() {
    let object = Handle::new(StepShapeSubedge::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepShapeSubedge");
    assert!(object.is_kind_name("StepShapeEdge"));
}

#[test]
fn init_round_trip_step_shape_subedge() {
    let a_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let a_edge_edge_start = Handle::new(StepShapeVertex::new());
    let a_edge_edge_end = Handle::new(StepShapeVertex::new());
    let a_parent_edge = Handle::new(StepShapeEdge::new());
    let object = StepShapeSubedge::new();
    object.init(
        &a_representation_item_name,
        &a_edge_edge_start,
        &a_edge_edge_end,
        &a_parent_edge,
    );
    assert!(object.parent_edge() == a_parent_edge);
}

#[test]
fn runtime_type_step_shape_subface() {
    let object = Handle::new(StepShapeSubface::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepShapeSubface");
    assert!(object.is_kind_name("StepShapeFace"));
}

#[test]
fn init_round_trip_step_shape_subface() {
    let a_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let a_face_bounds =
        Handle::new(HArray1::<Handle<StepShapeFaceBound>>::with_bounds(1, 2).unwrap());
    let a_parent_face = Handle::new(StepShapeFace::new());
    let object = StepShapeSubface::new();
    object.init(&a_representation_item_name, &a_face_bounds, &a_parent_face);
    assert!(object.parent_face() == a_parent_face);
}

#[test]
fn runtime_type_step_shape_swept_area_solid() {
    let object = Handle::new(StepShapeSweptAreaSolid::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeSweptAreaSolid"
    );
    assert!(object.is_kind_name("StepShapeSolidModel"));
}

#[test]
fn init_round_trip_step_shape_swept_area_solid() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_swept_area = Handle::new(StepGeomCurveBoundedSurface::new());
    let object = StepShapeSweptAreaSolid::new();
    object.init(&a_name, &a_swept_area);
    assert!(object.swept_area() == a_swept_area);
}

#[test]
fn runtime_type_step_shape_swept_face_solid() {
    let object = Handle::new(StepShapeSweptFaceSolid::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeSweptFaceSolid"
    );
    assert!(object.is_kind_name("StepShapeSolidModel"));
}

#[test]
fn init_round_trip_step_shape_swept_face_solid() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_swept_area = Handle::new(StepShapeFaceSurface::new());
    let object = StepShapeSweptFaceSolid::new();
    object.init(&a_name, &a_swept_area);
    assert!(object.swept_face() == a_swept_area);
}

#[test]
fn runtime_type_step_shape_tolerance_value() {
    let object = Handle::new(StepShapeToleranceValue::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeToleranceValue"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn runtime_type_step_shape_topological_representation_item() {
    let object = Handle::new(StepShapeTopologicalRepresentationItem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeTopologicalRepresentationItem"
    );
    assert!(object.is_kind_name("StepReprRepresentationItem"));
}

#[test]
fn runtime_type_step_shape_torus() {
    let object = Handle::new(StepShapeTorus::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepShapeTorus");
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}

#[test]
fn init_round_trip_step_shape_torus() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_position = Handle::new(StepGeomAxis1Placement::new());
    let a_major_radius = 2.5;
    let a_minor_radius = 2.5;
    let object = StepShapeTorus::new();
    object.init(&a_name, &a_position, a_major_radius, a_minor_radius);
    assert!(object.position() == a_position);
    assert!(object.major_radius() == a_major_radius);
    assert!(object.minor_radius() == a_minor_radius);
}

#[test]
fn runtime_type_step_shape_transitional_shape_representation() {
    let object = Handle::new(StepShapeTransitionalShapeRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeTransitionalShapeRepresentation"
    );
    assert!(object.is_kind_name("StepShapeShapeRepresentation"));
}

#[test]
fn runtime_type_step_shape_type_qualifier() {
    let object = Handle::new(StepShapeTypeQualifier::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeTypeQualifier"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_shape_type_qualifier() {
    let name = Handle::new(HAsciiString::new("sample"));
    let object = StepShapeTypeQualifier::new();
    object.init(&name);
    assert!(object.name() == name);
}

#[test]
fn runtime_type_step_shape_value_format_type_qualifier() {
    let object = Handle::new(StepShapeValueFormatTypeQualifier::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeValueFormatTypeQualifier"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_shape_value_format_type_qualifier() {
    let the_format_type = Handle::new(HAsciiString::new("sample"));
    let object = StepShapeValueFormatTypeQualifier::new();
    object.init(&the_format_type);
    assert!(object.format_type() == the_format_type);
}

#[test]
fn runtime_type_step_shape_vertex() {
    let object = Handle::new(StepShapeVertex::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepShapeVertex");
    assert!(object.is_kind_name("StepShapeTopologicalRepresentationItem"));
}

#[test]
fn runtime_type_step_shape_vertex_loop() {
    let object = Handle::new(StepShapeVertexLoop::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepShapeVertexLoop");
    assert!(object.is_kind_name("StepShapeLoop"));
}

#[test]
fn init_round_trip_step_shape_vertex_loop() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_loop_vertex = Handle::new(StepShapeVertex::new());
    let object = StepShapeVertexLoop::new();
    object.init(&a_name, &a_loop_vertex);
    assert!(object.loop_vertex() == a_loop_vertex);
}

#[test]
fn runtime_type_step_shape_vertex_point() {
    let object = Handle::new(StepShapeVertexPoint::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeVertexPoint"
    );
    assert!(object.is_kind_name("StepShapeVertex"));
}

#[test]
fn init_round_trip_step_shape_vertex_point() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_vertex_geometry = Handle::new(StepGeomPoint::new());
    let object = StepShapeVertexPoint::new();
    object.init(&a_name, &a_vertex_geometry);
    assert!(object.vertex_geometry() == a_vertex_geometry);
}
