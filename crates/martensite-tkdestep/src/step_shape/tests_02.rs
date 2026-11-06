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
fn runtime_type_step_shape_dimensional_size() {
    let object = Handle::new(StepShapeDimensionalSize::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeDimensionalSize"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_shape_dimensional_size() {
    let a_applies_to = Handle::new(StepReprShapeAspect::new());
    let a_name = Handle::new(HAsciiString::new("sample"));
    let object = StepShapeDimensionalSize::new();
    object.init(&a_applies_to, &a_name);
    assert!(object.applies_to() == a_applies_to);
    assert!(object.name() == a_name);
}

#[test]
fn runtime_type_step_shape_dimensional_size_with_path() {
    let object = Handle::new(StepShapeDimensionalSizeWithPath::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeDimensionalSizeWithPath"
    );
    assert!(object.is_kind_name("StepShapeDimensionalSize"));
}

#[test]
fn init_round_trip_step_shape_dimensional_size_with_path() {
    let a_dimensional_size_applies_to = Handle::new(StepReprShapeAspect::new());
    let a_dimensional_size_name = Handle::new(HAsciiString::new("sample"));
    let a_path = Handle::new(StepReprShapeAspect::new());
    let object = StepShapeDimensionalSizeWithPath::new();
    object.init(
        &a_dimensional_size_applies_to,
        &a_dimensional_size_name,
        &a_path,
    );
    assert!(object.path() == a_path);
}

#[test]
fn runtime_type_step_shape_directed_dimensional_location() {
    let object = Handle::new(StepShapeDirectedDimensionalLocation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeDirectedDimensionalLocation"
    );
    assert!(object.is_kind_name("StepShapeDimensionalLocation"));
}

#[test]
fn runtime_type_step_shape_edge_based_wireframe_model() {
    let object = Handle::new(StepShapeEdgeBasedWireframeModel::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeEdgeBasedWireframeModel"
    );
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}

#[test]
fn init_round_trip_step_shape_edge_based_wireframe_model() {
    let a_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let a_ebwm_boundary =
        Handle::new(HArray1::<Handle<StepShapeConnectedEdgeSet>>::with_bounds(1, 2).unwrap());
    let object = StepShapeEdgeBasedWireframeModel::new();
    object.init(&a_representation_item_name, &a_ebwm_boundary);
    assert!(object.ebwm_boundary() == a_ebwm_boundary);
}

#[test]
fn runtime_type_step_shape_edge_based_wireframe_shape_representation() {
    let object = Handle::new(StepShapeEdgeBasedWireframeShapeRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeEdgeBasedWireframeShapeRepresentation"
    );
    assert!(object.is_kind_name("StepShapeShapeRepresentation"));
}

#[test]
fn runtime_type_step_shape_edge_curve() {
    let object = Handle::new(StepShapeEdgeCurve::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepShapeEdgeCurve");
    assert!(object.is_kind_name("StepShapeEdge"));
}

#[test]
fn init_round_trip_step_shape_edge_curve() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_edge_start = Handle::new(StepShapeVertex::new());
    let a_edge_end = Handle::new(StepShapeVertex::new());
    let a_edge_geometry = Handle::new(StepGeomCurve::new());
    let a_same_sense = true;
    let object = StepShapeEdgeCurve::new();
    object.init(
        &a_name,
        &a_edge_start,
        &a_edge_end,
        &a_edge_geometry,
        a_same_sense,
    );
    assert!(object.edge_geometry() == a_edge_geometry);
    assert!(object.same_sense() == a_same_sense);
}

#[test]
fn runtime_type_step_shape_edge_loop() {
    let object = Handle::new(StepShapeEdgeLoop::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepShapeEdgeLoop");
    assert!(object.is_kind_name("StepShapeLoop"));
}

#[test]
fn init_round_trip_step_shape_edge_loop() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_edge_list =
        Handle::new(HArray1::<Handle<StepShapeOrientedEdge>>::with_bounds(1, 2).unwrap());
    let object = StepShapeEdgeLoop::new();
    object.init(&a_name, &a_edge_list);
    assert!(object.edge_list() == a_edge_list);
}

#[test]
fn runtime_type_step_shape_extruded_area_solid() {
    let object = Handle::new(StepShapeExtrudedAreaSolid::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeExtrudedAreaSolid"
    );
    assert!(object.is_kind_name("StepShapeSweptAreaSolid"));
}

#[test]
fn init_round_trip_step_shape_extruded_area_solid() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_swept_area = Handle::new(StepGeomCurveBoundedSurface::new());
    let a_extruded_direction = Handle::new(StepGeomDirection::new());
    let a_depth = 2.5;
    let object = StepShapeExtrudedAreaSolid::new();
    object.init(&a_name, &a_swept_area, &a_extruded_direction, a_depth);
    assert!(object.extruded_direction() == a_extruded_direction);
    assert!(object.depth() == a_depth);
}

#[test]
fn runtime_type_step_shape_extruded_face_solid() {
    let object = Handle::new(StepShapeExtrudedFaceSolid::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeExtrudedFaceSolid"
    );
    assert!(object.is_kind_name("StepShapeSweptFaceSolid"));
}

#[test]
fn init_round_trip_step_shape_extruded_face_solid() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_swept_area = Handle::new(StepShapeFaceSurface::new());
    let a_extruded_direction = Handle::new(StepGeomDirection::new());
    let a_depth = 2.5;
    let object = StepShapeExtrudedFaceSolid::new();
    object.init(&a_name, &a_swept_area, &a_extruded_direction, a_depth);
    assert!(object.extruded_direction() == a_extruded_direction);
    assert!(object.depth() == a_depth);
}

#[test]
fn runtime_type_step_shape_face_based_surface_model() {
    let object = Handle::new(StepShapeFaceBasedSurfaceModel::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeFaceBasedSurfaceModel"
    );
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}

#[test]
fn init_round_trip_step_shape_face_based_surface_model() {
    let a_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let a_fbsm_faces =
        Handle::new(HArray1::<Handle<StepShapeConnectedFaceSet>>::with_bounds(1, 2).unwrap());
    let object = StepShapeFaceBasedSurfaceModel::new();
    object.init(&a_representation_item_name, &a_fbsm_faces);
    assert!(object.fbsm_faces() == a_fbsm_faces);
}

#[test]
fn runtime_type_step_shape_face_bound() {
    let object = Handle::new(StepShapeFaceBound::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepShapeFaceBound");
    assert!(object.is_kind_name("StepShapeTopologicalRepresentationItem"));
}

#[test]
fn init_round_trip_step_shape_face_bound() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_bound = Handle::new(StepShapeLoop::new());
    let a_orientation = true;
    let object = StepShapeFaceBound::new();
    object.init(&a_name, &a_bound, a_orientation);
    assert!(object.bound() == a_bound);
    assert!(object.orientation() == a_orientation);
}

#[test]
fn runtime_type_step_shape_face_outer_bound() {
    let object = Handle::new(StepShapeFaceOuterBound::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeFaceOuterBound"
    );
    assert!(object.is_kind_name("StepShapeFaceBound"));
}

#[test]
fn runtime_type_step_shape_face_surface() {
    let object = Handle::new(StepShapeFaceSurface::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeFaceSurface"
    );
    assert!(object.is_kind_name("StepShapeFace"));
}

#[test]
fn init_round_trip_step_shape_face_surface() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_bounds = Handle::new(HArray1::<Handle<StepShapeFaceBound>>::with_bounds(1, 2).unwrap());
    let a_face_geometry = Handle::new(StepGeomSurface::new());
    let a_same_sense = true;
    let object = StepShapeFaceSurface::new();
    object.init(&a_name, &a_bounds, &a_face_geometry, a_same_sense);
    assert!(object.face_geometry() == a_face_geometry);
    assert!(object.same_sense() == a_same_sense);
}

#[test]
fn runtime_type_step_shape_faceted_brep() {
    let object = Handle::new(StepShapeFacetedBrep::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeFacetedBrep"
    );
    assert!(object.is_kind_name("StepShapeManifoldSolidBrep"));
}

#[test]
fn runtime_type_step_shape_faceted_brep_and_brep_with_voids() {
    let object = Handle::new(StepShapeFacetedBrepAndBrepWithVoids::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeFacetedBrepAndBrepWithVoids"
    );
    assert!(object.is_kind_name("StepShapeManifoldSolidBrep"));
}

#[test]
fn init_round_trip_step_shape_faceted_brep_and_brep_with_voids() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_outer = Handle::new(StepShapeClosedShell::new());
    let a_faceted_brep = Handle::new(StepShapeFacetedBrep::new());
    let a_brep_with_voids = Handle::new(StepShapeBrepWithVoids::new());
    let object = StepShapeFacetedBrepAndBrepWithVoids::new();
    object.init(&a_name, &a_outer, &a_faceted_brep, &a_brep_with_voids);
    assert!(object.faceted_brep() == a_faceted_brep);
    assert!(object.brep_with_voids() == a_brep_with_voids);
}

#[test]
fn runtime_type_step_shape_faceted_brep_shape_representation() {
    let object = Handle::new(StepShapeFacetedBrepShapeRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeFacetedBrepShapeRepresentation"
    );
    assert!(object.is_kind_name("StepShapeShapeRepresentation"));
}

#[test]
fn runtime_type_step_shape_geometric_curve_set() {
    let object = Handle::new(StepShapeGeometricCurveSet::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeGeometricCurveSet"
    );
    assert!(object.is_kind_name("StepShapeGeometricSet"));
}

#[test]
fn runtime_type_step_shape_geometric_set() {
    let object = Handle::new(StepShapeGeometricSet::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeGeometricSet"
    );
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}

#[test]
fn init_round_trip_step_shape_geometric_set() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_elements =
        Handle::new(HArray1::<StepShapeGeometricSetSelect>::with_bounds(1, 2).unwrap());
    let object = StepShapeGeometricSet::new();
    object.init(&a_name, &a_elements);
    assert!(object.elements() == a_elements);
}

#[test]
fn runtime_type_step_shape_geometrically_bounded_surface_shape_representation() {
    let object = Handle::new(StepShapeGeometricallyBoundedSurfaceShapeRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeGeometricallyBoundedSurfaceShapeRepresentation"
    );
    assert!(object.is_kind_name("StepShapeShapeRepresentation"));
}

#[test]
fn runtime_type_step_shape_geometrically_bounded_wireframe_shape_representation() {
    let object = Handle::new(StepShapeGeometricallyBoundedWireframeShapeRepresentation::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeGeometricallyBoundedWireframeShapeRepresentation"
    );
    assert!(object.is_kind_name("StepShapeShapeRepresentation"));
}

#[test]
fn runtime_type_step_shape_half_space_solid() {
    let object = Handle::new(StepShapeHalfSpaceSolid::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeHalfSpaceSolid"
    );
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}

#[test]
fn init_round_trip_step_shape_half_space_solid() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_base_surface = Handle::new(StepGeomSurface::new());
    let a_agreement_flag = true;
    let object = StepShapeHalfSpaceSolid::new();
    object.init(&a_name, &a_base_surface, a_agreement_flag);
    assert!(object.base_surface() == a_base_surface);
    assert!(object.agreement_flag() == a_agreement_flag);
}

#[test]
fn runtime_type_step_shape_limits_and_fits() {
    let object = Handle::new(StepShapeLimitsAndFits::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepShapeLimitsAndFits"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}
