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
fn runtime_type_step_geom_ellipse() {
    let object = Handle::new(StepGeomEllipse::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepGeomEllipse");
    assert!(object.is_kind_name("StepGeomConic"));
}

#[test]
fn init_round_trip_step_geom_ellipse() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_position = StepGeomAxis2Placement::default();
    let a_semi_axis1 = 2.5;
    let a_semi_axis2 = 2.5;
    let object = StepGeomEllipse::new();
    object.init(&a_name, &a_position, a_semi_axis1, a_semi_axis2);
    assert!(object.semi_axis1() == a_semi_axis1);
    assert!(object.semi_axis2() == a_semi_axis2);
}

#[test]
fn runtime_type_step_geom_evaluated_degenerate_pcurve() {
    let object = Handle::new(StepGeomEvaluatedDegeneratePcurve::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomEvaluatedDegeneratePcurve"
    );
    assert!(object.is_kind_name("StepGeomDegeneratePcurve"));
}

#[test]
fn init_round_trip_step_geom_evaluated_degenerate_pcurve() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_basis_surface = Handle::new(StepGeomSurface::new());
    let a_reference_to_curve = Handle::new(StepReprDefinitionalRepresentation::new());
    let a_equivalent_point = Handle::new(StepGeomCartesianPoint::new());
    let object = StepGeomEvaluatedDegeneratePcurve::new();
    object.init(
        &a_name,
        &a_basis_surface,
        &a_reference_to_curve,
        &a_equivalent_point,
    );
    assert!(object.equivalent_point() == a_equivalent_point);
}

#[test]
fn runtime_type_step_geom_geom_rep_context_and_glob_unit_ass_ctx_and_glob_uncertainty_ass_ctx() {
    let object =
        Handle::new(StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx"
    );
    assert!(object.is_kind_name("StepReprRepresentationContext"));
}

#[test]
fn init_round_trip_step_geom_geom_rep_context_and_glob_unit_ass_ctx_and_glob_uncertainty_ass_ctx() {
    let a_context_identifier = Handle::new(HAsciiString::new("sample"));
    let a_context_type = Handle::new(HAsciiString::new("sample"));
    let a_geometric_representation_context =
        Handle::new(StepGeomGeometricRepresentationContext::new());
    let a_global_unit_assigned_context = Handle::new(StepReprGlobalUnitAssignedContext::new());
    let a_global_uncertainty_assigned_ctx =
        Handle::new(StepReprGlobalUncertaintyAssignedContext::new());
    let object = StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx::new();
    object.init(
        &a_context_identifier,
        &a_context_type,
        &a_geometric_representation_context,
        &a_global_unit_assigned_context,
        &a_global_uncertainty_assigned_ctx,
    );
    assert!(object.geometric_representation_context() == a_geometric_representation_context);
    assert!(object.global_unit_assigned_context() == a_global_unit_assigned_context);
    assert!(object.global_uncertainty_assigned_context() == a_global_uncertainty_assigned_ctx);
}

#[test]
fn runtime_type_step_geom_geometric_representation_context() {
    let object = Handle::new(StepGeomGeometricRepresentationContext::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomGeometricRepresentationContext"
    );
    assert!(object.is_kind_name("StepReprRepresentationContext"));
}

#[test]
fn init_round_trip_step_geom_geometric_representation_context() {
    let a_context_identifier = Handle::new(HAsciiString::new("sample"));
    let a_context_type = Handle::new(HAsciiString::new("sample"));
    let a_coordinate_space_dimension = 7;
    let object = StepGeomGeometricRepresentationContext::new();
    object.init(
        &a_context_identifier,
        &a_context_type,
        a_coordinate_space_dimension,
    );
    assert!(object.coordinate_space_dimension() == a_coordinate_space_dimension);
}

#[test]
fn runtime_type_step_geom_geometric_representation_context_and_global_unit_assigned_context() {
    let object =
        Handle::new(StepGeomGeometricRepresentationContextAndGlobalUnitAssignedContext::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomGeometricRepresentationContextAndGlobalUnitAssignedContext"
    );
    assert!(object.is_kind_name("StepReprRepresentationContext"));
}

#[test]
fn init_round_trip_step_geom_geometric_representation_context_and_global_unit_assigned_context() {
    let a_context_identifier = Handle::new(HAsciiString::new("sample"));
    let a_context_type = Handle::new(HAsciiString::new("sample"));
    let a_geometric_representation_context =
        Handle::new(StepGeomGeometricRepresentationContext::new());
    let a_global_unit_assigned_context = Handle::new(StepReprGlobalUnitAssignedContext::new());
    let object = StepGeomGeometricRepresentationContextAndGlobalUnitAssignedContext::new();
    object.init(
        &a_context_identifier,
        &a_context_type,
        &a_geometric_representation_context,
        &a_global_unit_assigned_context,
    );
    assert!(object.geometric_representation_context() == a_geometric_representation_context);
    assert!(object.global_unit_assigned_context() == a_global_unit_assigned_context);
}

#[test]
fn runtime_type_step_geom_geometric_representation_context_and_parametric_representation_context() {
    let object = Handle::new(
        StepGeomGeometricRepresentationContextAndParametricRepresentationContext::new(),
    );
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomGeometricRepresentationContextAndParametricRepresentationContext"
    );
    assert!(object.is_kind_name("StepReprRepresentationContext"));
}

#[test]
fn init_round_trip_step_geom_geometric_representation_context_and_parametric_representation_context()
 {
    let a_context_identifier = Handle::new(HAsciiString::new("sample"));
    let a_context_type = Handle::new(HAsciiString::new("sample"));
    let a_geometric_representation_context =
        Handle::new(StepGeomGeometricRepresentationContext::new());
    let a_parametric_representation_context =
        Handle::new(StepReprParametricRepresentationContext::new());
    let object = StepGeomGeometricRepresentationContextAndParametricRepresentationContext::new();
    object.init(
        &a_context_identifier,
        &a_context_type,
        &a_geometric_representation_context,
        &a_parametric_representation_context,
    );
    assert!(object.geometric_representation_context() == a_geometric_representation_context);
    assert!(object.parametric_representation_context() == a_parametric_representation_context);
}

#[test]
fn runtime_type_step_geom_geometric_representation_item() {
    let object = Handle::new(StepGeomGeometricRepresentationItem::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomGeometricRepresentationItem"
    );
    assert!(object.is_kind_name("StepReprRepresentationItem"));
}

#[test]
fn runtime_type_step_geom_hyperbola() {
    let object = Handle::new(StepGeomHyperbola::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepGeomHyperbola");
    assert!(object.is_kind_name("StepGeomConic"));
}

#[test]
fn init_round_trip_step_geom_hyperbola() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_position = StepGeomAxis2Placement::default();
    let a_semi_axis = 2.5;
    let a_semi_imag_axis = 2.5;
    let object = StepGeomHyperbola::new();
    object.init(&a_name, &a_position, a_semi_axis, a_semi_imag_axis);
    assert!(object.semi_axis() == a_semi_axis);
    assert!(object.semi_imag_axis() == a_semi_imag_axis);
}

#[test]
fn runtime_type_step_geom_intersection_curve() {
    let object = Handle::new(StepGeomIntersectionCurve::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomIntersectionCurve"
    );
    assert!(object.is_kind_name("StepGeomSurfaceCurve"));
}

#[test]
fn runtime_type_step_geom_line() {
    let object = Handle::new(StepGeomLine::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepGeomLine");
    assert!(object.is_kind_name("StepGeomCurve"));
}

#[test]
fn init_round_trip_step_geom_line() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_pnt = Handle::new(StepGeomCartesianPoint::new());
    let a_dir = Handle::new(StepGeomVector::new());
    let object = StepGeomLine::new();
    object.init(&a_name, &a_pnt, &a_dir);
    assert!(object.pnt() == a_pnt);
    assert!(object.dir() == a_dir);
}

#[test]
fn runtime_type_step_geom_offset_curve3d() {
    let object = Handle::new(StepGeomOffsetCurve3d::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomOffsetCurve3d"
    );
    assert!(object.is_kind_name("StepGeomCurve"));
}

#[test]
fn init_round_trip_step_geom_offset_curve3d() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_basis_curve = Handle::new(StepGeomCurve::new());
    let a_distance = 2.5;
    let a_self_intersect = StepDataLogical::default();
    let a_ref_direction = Handle::new(StepGeomDirection::new());
    let object = StepGeomOffsetCurve3d::new();
    object.init(
        &a_name,
        &a_basis_curve,
        a_distance,
        a_self_intersect,
        &a_ref_direction,
    );
    assert!(object.basis_curve() == a_basis_curve);
    assert!(object.distance() == a_distance);
    assert!(object.self_intersect() == a_self_intersect);
    assert!(object.ref_direction() == a_ref_direction);
}

#[test]
fn runtime_type_step_geom_offset_surface() {
    let object = Handle::new(StepGeomOffsetSurface::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomOffsetSurface"
    );
    assert!(object.is_kind_name("StepGeomSurface"));
}

#[test]
fn init_round_trip_step_geom_offset_surface() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_basis_surface = Handle::new(StepGeomSurface::new());
    let a_distance = 2.5;
    let a_self_intersect = StepDataLogical::default();
    let object = StepGeomOffsetSurface::new();
    object.init(&a_name, &a_basis_surface, a_distance, a_self_intersect);
    assert!(object.basis_surface() == a_basis_surface);
    assert!(object.distance() == a_distance);
    assert!(object.self_intersect() == a_self_intersect);
}

#[test]
fn runtime_type_step_geom_oriented_surface() {
    let object = Handle::new(StepGeomOrientedSurface::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomOrientedSurface"
    );
    assert!(object.is_kind_name("StepGeomSurface"));
}

#[test]
fn init_round_trip_step_geom_oriented_surface() {
    let a_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let a_orientation = true;
    let object = StepGeomOrientedSurface::new();
    object.init(&a_representation_item_name, a_orientation);
    assert!(object.orientation() == a_orientation);
}

#[test]
fn runtime_type_step_geom_outer_boundary_curve() {
    let object = Handle::new(StepGeomOuterBoundaryCurve::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomOuterBoundaryCurve"
    );
    assert!(object.is_kind_name("StepGeomBoundaryCurve"));
}

#[test]
fn runtime_type_step_geom_parabola() {
    let object = Handle::new(StepGeomParabola::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepGeomParabola");
    assert!(object.is_kind_name("StepGeomConic"));
}

#[test]
fn init_round_trip_step_geom_parabola() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_position = StepGeomAxis2Placement::default();
    let a_focal_dist = 2.5;
    let object = StepGeomParabola::new();
    object.init(&a_name, &a_position, a_focal_dist);
    assert!(object.focal_dist() == a_focal_dist);
}

#[test]
fn runtime_type_step_geom_pcurve() {
    let object = Handle::new(StepGeomPcurve::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepGeomPcurve");
    assert!(object.is_kind_name("StepGeomCurve"));
}

#[test]
fn init_round_trip_step_geom_pcurve() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_basis_surface = Handle::new(StepGeomSurface::new());
    let a_reference_to_curve = Handle::new(StepReprDefinitionalRepresentation::new());
    let object = StepGeomPcurve::new();
    object.init(&a_name, &a_basis_surface, &a_reference_to_curve);
    assert!(object.basis_surface() == a_basis_surface);
    assert!(object.reference_to_curve() == a_reference_to_curve);
}

#[test]
fn runtime_type_step_geom_placement() {
    let object = Handle::new(StepGeomPlacement::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepGeomPlacement");
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}

#[test]
fn init_round_trip_step_geom_placement() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_location = Handle::new(StepGeomCartesianPoint::new());
    let object = StepGeomPlacement::new();
    object.init(&a_name, &a_location);
    assert!(object.location() == a_location);
}

#[test]
fn runtime_type_step_geom_plane() {
    let object = Handle::new(StepGeomPlane::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepGeomPlane");
    assert!(object.is_kind_name("StepGeomElementarySurface"));
}
