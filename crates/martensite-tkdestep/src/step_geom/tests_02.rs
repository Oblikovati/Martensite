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
fn runtime_type_step_geom_boundary_curve() {
    let object = Handle::new(StepGeomBoundaryCurve::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomBoundaryCurve"
    );
    assert!(object.is_kind_name("StepGeomCompositeCurveOnSurface"));
}

#[test]
fn runtime_type_step_geom_bounded_curve() {
    let object = Handle::new(StepGeomBoundedCurve::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomBoundedCurve"
    );
    assert!(object.is_kind_name("StepGeomCurve"));
}

#[test]
fn runtime_type_step_geom_bounded_surface() {
    let object = Handle::new(StepGeomBoundedSurface::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomBoundedSurface"
    );
    assert!(object.is_kind_name("StepGeomSurface"));
}

#[test]
fn runtime_type_step_geom_cartesian_transformation_operator() {
    let object = Handle::new(StepGeomCartesianTransformationOperator::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomCartesianTransformationOperator"
    );
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}

#[test]
fn init_round_trip_step_geom_cartesian_transformation_operator() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let has_aaxis1 = true;
    let a_axis1 = Handle::new(StepGeomDirection::new());
    let has_aaxis2 = true;
    let a_axis2 = Handle::new(StepGeomDirection::new());
    let a_local_origin = Handle::new(StepGeomCartesianPoint::new());
    let has_ascale = true;
    let a_scale = 2.5;
    let object = StepGeomCartesianTransformationOperator::new();
    object.init(
        &a_name,
        has_aaxis1,
        &a_axis1,
        has_aaxis2,
        &a_axis2,
        &a_local_origin,
        has_ascale,
        a_scale,
    );
    assert!(object.has_axis1() == has_aaxis1);
    assert!(object.axis1() == a_axis1);
    assert!(object.has_axis2() == has_aaxis2);
    assert!(object.axis2() == a_axis2);
    assert!(object.local_origin() == a_local_origin);
    assert!(object.has_scale() == has_ascale);
    assert!(object.scale() == a_scale);
}

#[test]
fn runtime_type_step_geom_cartesian_transformation_operator2d() {
    let object = Handle::new(StepGeomCartesianTransformationOperator2d::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomCartesianTransformationOperator2d"
    );
    assert!(object.is_kind_name("StepGeomCartesianTransformationOperator"));
}

#[test]
fn runtime_type_step_geom_cartesian_transformation_operator3d() {
    let object = Handle::new(StepGeomCartesianTransformationOperator3d::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomCartesianTransformationOperator3d"
    );
    assert!(object.is_kind_name("StepGeomCartesianTransformationOperator"));
}

#[test]
fn init_round_trip_step_geom_cartesian_transformation_operator3d() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let has_aaxis1 = true;
    let a_axis1 = Handle::new(StepGeomDirection::new());
    let has_aaxis2 = true;
    let a_axis2 = Handle::new(StepGeomDirection::new());
    let a_local_origin = Handle::new(StepGeomCartesianPoint::new());
    let has_ascale = true;
    let a_scale = 2.5;
    let has_aaxis3 = true;
    let a_axis3 = Handle::new(StepGeomDirection::new());
    let object = StepGeomCartesianTransformationOperator3d::new();
    object.init(
        &a_name,
        has_aaxis1,
        &a_axis1,
        has_aaxis2,
        &a_axis2,
        &a_local_origin,
        has_ascale,
        a_scale,
        has_aaxis3,
        &a_axis3,
    );
    assert!(object.has_axis3() == has_aaxis3);
    assert!(object.axis3() == a_axis3);
}

#[test]
fn runtime_type_step_geom_circle() {
    let object = Handle::new(StepGeomCircle::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepGeomCircle");
    assert!(object.is_kind_name("StepGeomConic"));
}

#[test]
fn init_round_trip_step_geom_circle() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_position = StepGeomAxis2Placement::default();
    let a_radius = 2.5;
    let object = StepGeomCircle::new();
    object.init(&a_name, &a_position, a_radius);
    assert!(object.radius() == a_radius);
}

#[test]
fn runtime_type_step_geom_composite_curve() {
    let object = Handle::new(StepGeomCompositeCurve::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomCompositeCurve"
    );
    assert!(object.is_kind_name("StepGeomBoundedCurve"));
}

#[test]
fn init_round_trip_step_geom_composite_curve() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_segments =
        Handle::new(HArray1::<Handle<StepGeomCompositeCurveSegment>>::with_bounds(1, 2).unwrap());
    let a_self_intersect = StepDataLogical::default();
    let object = StepGeomCompositeCurve::new();
    object.init(&a_name, &a_segments, a_self_intersect);
    assert!(object.segments() == a_segments);
    assert!(object.self_intersect() == a_self_intersect);
}

#[test]
fn runtime_type_step_geom_composite_curve_on_surface() {
    let object = Handle::new(StepGeomCompositeCurveOnSurface::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomCompositeCurveOnSurface"
    );
    assert!(object.is_kind_name("StepGeomCompositeCurve"));
}

#[test]
fn runtime_type_step_geom_composite_curve_segment() {
    let object = Handle::new(StepGeomCompositeCurveSegment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomCompositeCurveSegment"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_geom_composite_curve_segment() {
    let a_transition = StepGeomTransitionCode::default();
    let a_same_sense = true;
    let a_parent_curve = Handle::new(StepGeomCurve::new());
    let object = StepGeomCompositeCurveSegment::new();
    object.init(a_transition, a_same_sense, &a_parent_curve);
    assert!(object.transition() == a_transition);
    assert!(object.same_sense() == a_same_sense);
    assert!(object.parent_curve() == a_parent_curve);
}

#[test]
fn runtime_type_step_geom_conic() {
    let object = Handle::new(StepGeomConic::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepGeomConic");
    assert!(object.is_kind_name("StepGeomCurve"));
}

#[test]
fn init_round_trip_step_geom_conic() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_position = StepGeomAxis2Placement::default();
    let object = StepGeomConic::new();
    object.init(&a_name, &a_position);
}

#[test]
fn runtime_type_step_geom_conical_surface() {
    let object = Handle::new(StepGeomConicalSurface::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomConicalSurface"
    );
    assert!(object.is_kind_name("StepGeomElementarySurface"));
}

#[test]
fn init_round_trip_step_geom_conical_surface() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_position = Handle::new(StepGeomAxis2Placement3d::new());
    let a_radius = 2.5;
    let a_semi_angle = 2.5;
    let object = StepGeomConicalSurface::new();
    object.init(&a_name, &a_position, a_radius, a_semi_angle);
    assert!(object.radius() == a_radius);
    assert!(object.semi_angle() == a_semi_angle);
}

#[test]
fn runtime_type_step_geom_curve() {
    let object = Handle::new(StepGeomCurve::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepGeomCurve");
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}

#[test]
fn runtime_type_step_geom_curve_bounded_surface() {
    let object = Handle::new(StepGeomCurveBoundedSurface::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomCurveBoundedSurface"
    );
    assert!(object.is_kind_name("StepGeomBoundedSurface"));
}

#[test]
fn init_round_trip_step_geom_curve_bounded_surface() {
    let a_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let a_basis_surface = Handle::new(StepGeomSurface::new());
    let a_boundaries = Handle::new(HArray1::<StepGeomSurfaceBoundary>::with_bounds(1, 2).unwrap());
    let a_implicit_outer = true;
    let object = StepGeomCurveBoundedSurface::new();
    object.init(
        &a_representation_item_name,
        &a_basis_surface,
        &a_boundaries,
        a_implicit_outer,
    );
    assert!(object.basis_surface() == a_basis_surface);
    assert!(object.boundaries() == a_boundaries);
    assert!(object.implicit_outer() == a_implicit_outer);
}

#[test]
fn runtime_type_step_geom_curve_replica() {
    let object = Handle::new(StepGeomCurveReplica::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomCurveReplica"
    );
    assert!(object.is_kind_name("StepGeomCurve"));
}

#[test]
fn init_round_trip_step_geom_curve_replica() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_parent_curve = Handle::new(StepGeomCurve::new());
    let a_transformation = Handle::new(StepGeomCartesianTransformationOperator::new());
    let object = StepGeomCurveReplica::new();
    object.init(&a_name, &a_parent_curve, &a_transformation);
    assert!(object.parent_curve() == a_parent_curve);
    assert!(object.transformation() == a_transformation);
}

#[test]
fn runtime_type_step_geom_cylindrical_surface() {
    let object = Handle::new(StepGeomCylindricalSurface::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomCylindricalSurface"
    );
    assert!(object.is_kind_name("StepGeomElementarySurface"));
}

#[test]
fn init_round_trip_step_geom_cylindrical_surface() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_position = Handle::new(StepGeomAxis2Placement3d::new());
    let a_radius = 2.5;
    let object = StepGeomCylindricalSurface::new();
    object.init(&a_name, &a_position, a_radius);
    assert!(object.radius() == a_radius);
}

#[test]
fn runtime_type_step_geom_degenerate_pcurve() {
    let object = Handle::new(StepGeomDegeneratePcurve::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomDegeneratePcurve"
    );
    assert!(object.is_kind_name("StepGeomPoint"));
}

#[test]
fn init_round_trip_step_geom_degenerate_pcurve() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_basis_surface = Handle::new(StepGeomSurface::new());
    let a_reference_to_curve = Handle::new(StepReprDefinitionalRepresentation::new());
    let object = StepGeomDegeneratePcurve::new();
    object.init(&a_name, &a_basis_surface, &a_reference_to_curve);
    assert!(object.basis_surface() == a_basis_surface);
    assert!(object.reference_to_curve() == a_reference_to_curve);
}

#[test]
fn runtime_type_step_geom_degenerate_toroidal_surface() {
    let object = Handle::new(StepGeomDegenerateToroidalSurface::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomDegenerateToroidalSurface"
    );
    assert!(object.is_kind_name("StepGeomToroidalSurface"));
}

#[test]
fn init_round_trip_step_geom_degenerate_toroidal_surface() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_position = Handle::new(StepGeomAxis2Placement3d::new());
    let a_major_radius = 2.5;
    let a_minor_radius = 2.5;
    let a_select_outer = true;
    let object = StepGeomDegenerateToroidalSurface::new();
    object.init(
        &a_name,
        &a_position,
        a_major_radius,
        a_minor_radius,
        a_select_outer,
    );
    assert!(object.select_outer() == a_select_outer);
}

#[test]
fn runtime_type_step_geom_elementary_surface() {
    let object = Handle::new(StepGeomElementarySurface::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomElementarySurface"
    );
    assert!(object.is_kind_name("StepGeomSurface"));
}

#[test]
fn init_round_trip_step_geom_elementary_surface() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_position = Handle::new(StepGeomAxis2Placement3d::new());
    let object = StepGeomElementarySurface::new();
    object.init(&a_name, &a_position);
    assert!(object.position() == a_position);
}
