#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

#[test]
fn runtime_type_step_geom_point() {
    let object = Handle::new(StepGeomPoint::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepGeomPoint");
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}

#[test]
fn runtime_type_step_geom_point_on_curve() {
    let object = Handle::new(StepGeomPointOnCurve::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomPointOnCurve"
    );
    assert!(object.is_kind_name("StepGeomPoint"));
}

#[test]
fn init_round_trip_step_geom_point_on_curve() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_basis_curve = Handle::new(StepGeomCurve::new());
    let a_point_parameter = 2.5;
    let object = StepGeomPointOnCurve::new();
    object.init(&a_name, &a_basis_curve, a_point_parameter);
    assert!(object.basis_curve() == a_basis_curve);
    assert!(object.point_parameter() == a_point_parameter);
}

#[test]
fn runtime_type_step_geom_point_on_surface() {
    let object = Handle::new(StepGeomPointOnSurface::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomPointOnSurface"
    );
    assert!(object.is_kind_name("StepGeomPoint"));
}

#[test]
fn init_round_trip_step_geom_point_on_surface() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_basis_surface = Handle::new(StepGeomSurface::new());
    let a_point_parameter_u = 2.5;
    let a_point_parameter_v = 2.5;
    let object = StepGeomPointOnSurface::new();
    object.init(
        &a_name,
        &a_basis_surface,
        a_point_parameter_u,
        a_point_parameter_v,
    );
    assert!(object.basis_surface() == a_basis_surface);
    assert!(object.point_parameter_u() == a_point_parameter_u);
    assert!(object.point_parameter_v() == a_point_parameter_v);
}

#[test]
fn runtime_type_step_geom_point_replica() {
    let object = Handle::new(StepGeomPointReplica::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomPointReplica"
    );
    assert!(object.is_kind_name("StepGeomPoint"));
}

#[test]
fn init_round_trip_step_geom_point_replica() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_parent_pt = Handle::new(StepGeomPoint::new());
    let a_transformation = Handle::new(StepGeomCartesianTransformationOperator::new());
    let object = StepGeomPointReplica::new();
    object.init(&a_name, &a_parent_pt, &a_transformation);
    assert!(object.parent_pt() == a_parent_pt);
    assert!(object.transformation() == a_transformation);
}

#[test]
fn runtime_type_step_geom_polyline() {
    let object = Handle::new(StepGeomPolyline::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepGeomPolyline");
    assert!(object.is_kind_name("StepGeomBoundedCurve"));
}

#[test]
fn init_round_trip_step_geom_polyline() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_points =
        Handle::new(HArray1::<Handle<StepGeomCartesianPoint>>::with_bounds(1, 2).unwrap());
    let object = StepGeomPolyline::new();
    object.init(&a_name, &a_points);
    assert!(object.points() == a_points);
}

#[test]
fn runtime_type_step_geom_quasi_uniform_curve() {
    let object = Handle::new(StepGeomQuasiUniformCurve::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomQuasiUniformCurve"
    );
    assert!(object.is_kind_name("StepGeomBSplineCurve"));
}

#[test]
fn runtime_type_step_geom_quasi_uniform_curve_and_rational_b_spline_curve() {
    let object = Handle::new(StepGeomQuasiUniformCurveAndRationalBSplineCurve::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomQuasiUniformCurveAndRationalBSplineCurve"
    );
    assert!(object.is_kind_name("StepGeomBSplineCurve"));
}

#[test]
fn init_round_trip_step_geom_quasi_uniform_curve_and_rational_b_spline_curve() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_degree = 7;
    let a_control_points_list =
        Handle::new(HArray1::<Handle<StepGeomCartesianPoint>>::with_bounds(1, 2).unwrap());
    let a_curve_form = StepGeomBSplineCurveForm::default();
    let a_closed_curve = StepDataLogical::default();
    let a_self_intersect = StepDataLogical::default();
    let a_quasi_uniform_curve = Handle::new(StepGeomQuasiUniformCurve::new());
    let a_rational_b_spline_curve = Handle::new(StepGeomRationalBSplineCurve::new());
    let object = StepGeomQuasiUniformCurveAndRationalBSplineCurve::new();
    object.init(
        &a_name,
        a_degree,
        &a_control_points_list,
        a_curve_form,
        a_closed_curve,
        a_self_intersect,
        &a_quasi_uniform_curve,
        &a_rational_b_spline_curve,
    );
    assert!(object.quasi_uniform_curve() == a_quasi_uniform_curve);
    assert!(object.rational_b_spline_curve() == a_rational_b_spline_curve);
}

#[test]
fn runtime_type_step_geom_quasi_uniform_surface() {
    let object = Handle::new(StepGeomQuasiUniformSurface::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomQuasiUniformSurface"
    );
    assert!(object.is_kind_name("StepGeomBSplineSurface"));
}

#[test]
fn runtime_type_step_geom_quasi_uniform_surface_and_rational_b_spline_surface() {
    let object = Handle::new(StepGeomQuasiUniformSurfaceAndRationalBSplineSurface::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomQuasiUniformSurfaceAndRationalBSplineSurface"
    );
    assert!(object.is_kind_name("StepGeomBSplineSurface"));
}

#[test]
fn init_round_trip_step_geom_quasi_uniform_surface_and_rational_b_spline_surface() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_u_degree = 7;
    let a_v_degree = 7;
    let a_control_points_list =
        Handle::new(HArray2::<Handle<StepGeomCartesianPoint>>::with_bounds(1, 2, 1, 2).unwrap());
    let a_surface_form = StepGeomBSplineSurfaceForm::default();
    let a_u_closed = StepDataLogical::default();
    let a_v_closed = StepDataLogical::default();
    let a_self_intersect = StepDataLogical::default();
    let a_quasi_uniform_surface = Handle::new(StepGeomQuasiUniformSurface::new());
    let a_rational_b_spline_surface = Handle::new(StepGeomRationalBSplineSurface::new());
    let object = StepGeomQuasiUniformSurfaceAndRationalBSplineSurface::new();
    object.init(
        &a_name,
        a_u_degree,
        a_v_degree,
        &a_control_points_list,
        a_surface_form,
        a_u_closed,
        a_v_closed,
        a_self_intersect,
        &a_quasi_uniform_surface,
        &a_rational_b_spline_surface,
    );
    assert!(object.quasi_uniform_surface() == a_quasi_uniform_surface);
    assert!(object.rational_b_spline_surface() == a_rational_b_spline_surface);
}

#[test]
fn runtime_type_step_geom_rational_b_spline_curve() {
    let object = Handle::new(StepGeomRationalBSplineCurve::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomRationalBSplineCurve"
    );
    assert!(object.is_kind_name("StepGeomBSplineCurve"));
}

#[test]
fn init_round_trip_step_geom_rational_b_spline_curve() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_degree = 7;
    let a_control_points_list =
        Handle::new(HArray1::<Handle<StepGeomCartesianPoint>>::with_bounds(1, 2).unwrap());
    let a_curve_form = StepGeomBSplineCurveForm::default();
    let a_closed_curve = StepDataLogical::default();
    let a_self_intersect = StepDataLogical::default();
    let a_weights_data = Handle::new(HArray1::<f64>::with_bounds(1, 2).unwrap());
    let object = StepGeomRationalBSplineCurve::new();
    object.init(
        &a_name,
        a_degree,
        &a_control_points_list,
        a_curve_form,
        a_closed_curve,
        a_self_intersect,
        &a_weights_data,
    );
    assert!(object.weights_data() == a_weights_data);
}

#[test]
fn runtime_type_step_geom_rational_b_spline_surface() {
    let object = Handle::new(StepGeomRationalBSplineSurface::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomRationalBSplineSurface"
    );
    assert!(object.is_kind_name("StepGeomBSplineSurface"));
}

#[test]
fn init_round_trip_step_geom_rational_b_spline_surface() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_u_degree = 7;
    let a_v_degree = 7;
    let a_control_points_list =
        Handle::new(HArray2::<Handle<StepGeomCartesianPoint>>::with_bounds(1, 2, 1, 2).unwrap());
    let a_surface_form = StepGeomBSplineSurfaceForm::default();
    let a_u_closed = StepDataLogical::default();
    let a_v_closed = StepDataLogical::default();
    let a_self_intersect = StepDataLogical::default();
    let a_weights_data = Handle::new(HArray2::<f64>::with_bounds(1, 2, 1, 2).unwrap());
    let object = StepGeomRationalBSplineSurface::new();
    object.init(
        &a_name,
        a_u_degree,
        a_v_degree,
        &a_control_points_list,
        a_surface_form,
        a_u_closed,
        a_v_closed,
        a_self_intersect,
        &a_weights_data,
    );
    assert!(object.weights_data() == a_weights_data);
}

#[test]
fn runtime_type_step_geom_rectangular_composite_surface() {
    let object = Handle::new(StepGeomRectangularCompositeSurface::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomRectangularCompositeSurface"
    );
    assert!(object.is_kind_name("StepGeomBoundedSurface"));
}

#[test]
fn init_round_trip_step_geom_rectangular_composite_surface() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_segments =
        Handle::new(HArray2::<Handle<StepGeomSurfacePatch>>::with_bounds(1, 2, 1, 2).unwrap());
    let object = StepGeomRectangularCompositeSurface::new();
    object.init(&a_name, &a_segments);
    assert!(object.segments() == a_segments);
}

#[test]
fn runtime_type_step_geom_rectangular_trimmed_surface() {
    let object = Handle::new(StepGeomRectangularTrimmedSurface::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomRectangularTrimmedSurface"
    );
    assert!(object.is_kind_name("StepGeomBoundedSurface"));
}

#[test]
fn init_round_trip_step_geom_rectangular_trimmed_surface() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_basis_surface = Handle::new(StepGeomSurface::new());
    let a_u1 = 2.5;
    let a_u2 = 2.5;
    let a_v1 = 2.5;
    let a_v2 = 2.5;
    let a_usense = true;
    let a_vsense = true;
    let object = StepGeomRectangularTrimmedSurface::new();
    object.init(
        &a_name,
        &a_basis_surface,
        a_u1,
        a_u2,
        a_v1,
        a_v2,
        a_usense,
        a_vsense,
    );
    assert!(object.basis_surface() == a_basis_surface);
    assert!(object.u1() == a_u1);
    assert!(object.u2() == a_u2);
    assert!(object.v1() == a_v1);
    assert!(object.v2() == a_v2);
    assert!(object.usense() == a_usense);
    assert!(object.vsense() == a_vsense);
}

#[test]
fn runtime_type_step_geom_reparametrised_composite_curve_segment() {
    let object = Handle::new(StepGeomReparametrisedCompositeCurveSegment::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomReparametrisedCompositeCurveSegment"
    );
    assert!(object.is_kind_name("StepGeomCompositeCurveSegment"));
}

#[test]
fn init_round_trip_step_geom_reparametrised_composite_curve_segment() {
    let a_transition = StepGeomTransitionCode::default();
    let a_same_sense = true;
    let a_parent_curve = Handle::new(StepGeomCurve::new());
    let a_param_length = 2.5;
    let object = StepGeomReparametrisedCompositeCurveSegment::new();
    object.init(a_transition, a_same_sense, &a_parent_curve, a_param_length);
    assert!(object.param_length() == a_param_length);
}

#[test]
fn runtime_type_step_geom_seam_curve() {
    let object = Handle::new(StepGeomSeamCurve::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepGeomSeamCurve");
    assert!(object.is_kind_name("StepGeomSurfaceCurve"));
}

#[test]
fn runtime_type_step_geom_spherical_surface() {
    let object = Handle::new(StepGeomSphericalSurface::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomSphericalSurface"
    );
    assert!(object.is_kind_name("StepGeomElementarySurface"));
}

#[test]
fn init_round_trip_step_geom_spherical_surface() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_position = Handle::new(StepGeomAxis2Placement3d::new());
    let a_radius = 2.5;
    let object = StepGeomSphericalSurface::new();
    object.init(&a_name, &a_position, a_radius);
    assert!(object.radius() == a_radius);
}

#[test]
fn runtime_type_step_geom_su_parameters() {
    let object = Handle::new(StepGeomSuParameters::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomSuParameters"
    );
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}
