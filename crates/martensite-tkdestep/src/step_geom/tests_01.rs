#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

#[test]
fn runtime_type_step_geom_axis1_placement() {
    let object = Handle::new(StepGeomAxis1Placement::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomAxis1Placement"
    );
    assert!(object.is_kind_name("StepGeomPlacement"));
}

#[test]
fn init_round_trip_step_geom_axis1_placement() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_location = Handle::new(StepGeomCartesianPoint::new());
    let has_aaxis = true;
    let a_axis = Handle::new(StepGeomDirection::new());
    let object = StepGeomAxis1Placement::new();
    object.init(&a_name, &a_location, has_aaxis, &a_axis);
    assert!(object.has_axis() == has_aaxis);
    assert!(object.axis() == a_axis);
}

#[test]
fn runtime_type_step_geom_axis2_placement2d() {
    let object = Handle::new(StepGeomAxis2Placement2d::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomAxis2Placement2d"
    );
    assert!(object.is_kind_name("StepGeomPlacement"));
}

#[test]
fn init_round_trip_step_geom_axis2_placement2d() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_location = Handle::new(StepGeomCartesianPoint::new());
    let has_aref_direction = true;
    let a_ref_direction = Handle::new(StepGeomDirection::new());
    let object = StepGeomAxis2Placement2d::new();
    object.init(&a_name, &a_location, has_aref_direction, &a_ref_direction);
    assert!(object.has_ref_direction() == has_aref_direction);
    assert!(object.ref_direction() == a_ref_direction);
}

#[test]
fn runtime_type_step_geom_axis2_placement3d() {
    let object = Handle::new(StepGeomAxis2Placement3d::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomAxis2Placement3d"
    );
    assert!(object.is_kind_name("StepGeomPlacement"));
}

#[test]
fn init_round_trip_step_geom_axis2_placement3d() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_location = Handle::new(StepGeomCartesianPoint::new());
    let has_aaxis = true;
    let a_axis = Handle::new(StepGeomDirection::new());
    let has_aref_direction = true;
    let a_ref_direction = Handle::new(StepGeomDirection::new());
    let object = StepGeomAxis2Placement3d::new();
    object.init(
        &a_name,
        &a_location,
        has_aaxis,
        &a_axis,
        has_aref_direction,
        &a_ref_direction,
    );
    assert!(object.has_axis() == has_aaxis);
    assert!(object.axis() == a_axis);
    assert!(object.has_ref_direction() == has_aref_direction);
    assert!(object.ref_direction() == a_ref_direction);
}

#[test]
fn runtime_type_step_geom_b_spline_curve() {
    let object = Handle::new(StepGeomBSplineCurve::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomBSplineCurve"
    );
    assert!(object.is_kind_name("StepGeomBoundedCurve"));
}

#[test]
fn init_round_trip_step_geom_b_spline_curve() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_degree = 7;
    let a_control_points_list =
        Handle::new(HArray1::<Handle<StepGeomCartesianPoint>>::with_bounds(1, 2).unwrap());
    let a_curve_form = StepGeomBSplineCurveForm::default();
    let a_closed_curve = StepDataLogical::default();
    let a_self_intersect = StepDataLogical::default();
    let object = StepGeomBSplineCurve::new();
    object.init(
        &a_name,
        a_degree,
        &a_control_points_list,
        a_curve_form,
        a_closed_curve,
        a_self_intersect,
    );
    assert!(object.degree() == a_degree);
    assert!(object.control_points_list() == a_control_points_list);
    assert!(object.curve_form() == a_curve_form);
    assert!(object.closed_curve() == a_closed_curve);
    assert!(object.self_intersect() == a_self_intersect);
}

#[test]
fn runtime_type_step_geom_b_spline_curve_with_knots() {
    let object = Handle::new(StepGeomBSplineCurveWithKnots::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomBSplineCurveWithKnots"
    );
    assert!(object.is_kind_name("StepGeomBSplineCurve"));
}

#[test]
fn init_round_trip_step_geom_b_spline_curve_with_knots() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_degree = 7;
    let a_control_points_list =
        Handle::new(HArray1::<Handle<StepGeomCartesianPoint>>::with_bounds(1, 2).unwrap());
    let a_curve_form = StepGeomBSplineCurveForm::default();
    let a_closed_curve = StepDataLogical::default();
    let a_self_intersect = StepDataLogical::default();
    let a_knot_multiplicities = Handle::new(HArray1::<i32>::with_bounds(1, 2).unwrap());
    let a_knots = Handle::new(HArray1::<f64>::with_bounds(1, 2).unwrap());
    let a_knot_spec = StepGeomKnotType::default();
    let object = StepGeomBSplineCurveWithKnots::new();
    object.init(
        &a_name,
        a_degree,
        &a_control_points_list,
        a_curve_form,
        a_closed_curve,
        a_self_intersect,
        &a_knot_multiplicities,
        &a_knots,
        a_knot_spec,
    );
    assert!(object.knot_multiplicities() == a_knot_multiplicities);
    assert!(object.knots() == a_knots);
    assert!(object.knot_spec() == a_knot_spec);
}

#[test]
fn runtime_type_step_geom_b_spline_curve_with_knots_and_rational_b_spline_curve() {
    let object = Handle::new(StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve"
    );
    assert!(object.is_kind_name("StepGeomBSplineCurve"));
}

#[test]
fn init_round_trip_step_geom_b_spline_curve_with_knots_and_rational_b_spline_curve() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_degree = 7;
    let a_control_points_list =
        Handle::new(HArray1::<Handle<StepGeomCartesianPoint>>::with_bounds(1, 2).unwrap());
    let a_curve_form = StepGeomBSplineCurveForm::default();
    let a_closed_curve = StepDataLogical::default();
    let a_self_intersect = StepDataLogical::default();
    let a_b_spline_curve_with_knots = Handle::new(StepGeomBSplineCurveWithKnots::new());
    let a_rational_b_spline_curve = Handle::new(StepGeomRationalBSplineCurve::new());
    let object = StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve::new();
    object.init(
        &a_name,
        a_degree,
        &a_control_points_list,
        a_curve_form,
        a_closed_curve,
        a_self_intersect,
        &a_b_spline_curve_with_knots,
        &a_rational_b_spline_curve,
    );
    assert!(object.b_spline_curve_with_knots() == a_b_spline_curve_with_knots);
    assert!(object.rational_b_spline_curve() == a_rational_b_spline_curve);
}

#[test]
fn runtime_type_step_geom_b_spline_surface() {
    let object = Handle::new(StepGeomBSplineSurface::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomBSplineSurface"
    );
    assert!(object.is_kind_name("StepGeomBoundedSurface"));
}

#[test]
fn init_round_trip_step_geom_b_spline_surface() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_u_degree = 7;
    let a_v_degree = 7;
    let a_control_points_list =
        Handle::new(HArray2::<Handle<StepGeomCartesianPoint>>::with_bounds(1, 2, 1, 2).unwrap());
    let a_surface_form = StepGeomBSplineSurfaceForm::default();
    let a_u_closed = StepDataLogical::default();
    let a_v_closed = StepDataLogical::default();
    let a_self_intersect = StepDataLogical::default();
    let object = StepGeomBSplineSurface::new();
    object.init(
        &a_name,
        a_u_degree,
        a_v_degree,
        &a_control_points_list,
        a_surface_form,
        a_u_closed,
        a_v_closed,
        a_self_intersect,
    );
    assert!(object.u_degree() == a_u_degree);
    assert!(object.v_degree() == a_v_degree);
    assert!(object.control_points_list() == a_control_points_list);
    assert!(object.surface_form() == a_surface_form);
    assert!(object.u_closed() == a_u_closed);
    assert!(object.v_closed() == a_v_closed);
    assert!(object.self_intersect() == a_self_intersect);
}

#[test]
fn runtime_type_step_geom_b_spline_surface_with_knots() {
    let object = Handle::new(StepGeomBSplineSurfaceWithKnots::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomBSplineSurfaceWithKnots"
    );
    assert!(object.is_kind_name("StepGeomBSplineSurface"));
}

#[test]
fn init_round_trip_step_geom_b_spline_surface_with_knots() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_u_degree = 7;
    let a_v_degree = 7;
    let a_control_points_list =
        Handle::new(HArray2::<Handle<StepGeomCartesianPoint>>::with_bounds(1, 2, 1, 2).unwrap());
    let a_surface_form = StepGeomBSplineSurfaceForm::default();
    let a_u_closed = StepDataLogical::default();
    let a_v_closed = StepDataLogical::default();
    let a_self_intersect = StepDataLogical::default();
    let a_u_multiplicities = Handle::new(HArray1::<i32>::with_bounds(1, 2).unwrap());
    let a_v_multiplicities = Handle::new(HArray1::<i32>::with_bounds(1, 2).unwrap());
    let a_u_knots = Handle::new(HArray1::<f64>::with_bounds(1, 2).unwrap());
    let a_v_knots = Handle::new(HArray1::<f64>::with_bounds(1, 2).unwrap());
    let a_knot_spec = StepGeomKnotType::default();
    let object = StepGeomBSplineSurfaceWithKnots::new();
    object.init(
        &a_name,
        a_u_degree,
        a_v_degree,
        &a_control_points_list,
        a_surface_form,
        a_u_closed,
        a_v_closed,
        a_self_intersect,
        &a_u_multiplicities,
        &a_v_multiplicities,
        &a_u_knots,
        &a_v_knots,
        a_knot_spec,
    );
    assert!(object.u_multiplicities() == a_u_multiplicities);
    assert!(object.v_multiplicities() == a_v_multiplicities);
    assert!(object.u_knots() == a_u_knots);
    assert!(object.v_knots() == a_v_knots);
    assert!(object.knot_spec() == a_knot_spec);
}

#[test]
fn runtime_type_step_geom_b_spline_surface_with_knots_and_rational_b_spline_surface() {
    let object = Handle::new(StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface"
    );
    assert!(object.is_kind_name("StepGeomBSplineSurface"));
}

#[test]
fn init_round_trip_step_geom_b_spline_surface_with_knots_and_rational_b_spline_surface() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_u_degree = 7;
    let a_v_degree = 7;
    let a_control_points_list =
        Handle::new(HArray2::<Handle<StepGeomCartesianPoint>>::with_bounds(1, 2, 1, 2).unwrap());
    let a_surface_form = StepGeomBSplineSurfaceForm::default();
    let a_u_closed = StepDataLogical::default();
    let a_v_closed = StepDataLogical::default();
    let a_self_intersect = StepDataLogical::default();
    let a_b_spline_surface_with_knots = Handle::new(StepGeomBSplineSurfaceWithKnots::new());
    let a_rational_b_spline_surface = Handle::new(StepGeomRationalBSplineSurface::new());
    let object = StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface::new();
    object.init(
        &a_name,
        a_u_degree,
        a_v_degree,
        &a_control_points_list,
        a_surface_form,
        a_u_closed,
        a_v_closed,
        a_self_intersect,
        &a_b_spline_surface_with_knots,
        &a_rational_b_spline_surface,
    );
    assert!(object.b_spline_surface_with_knots() == a_b_spline_surface_with_knots);
    assert!(object.rational_b_spline_surface() == a_rational_b_spline_surface);
}

#[test]
fn runtime_type_step_geom_bezier_curve() {
    let object = Handle::new(StepGeomBezierCurve::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepGeomBezierCurve");
    assert!(object.is_kind_name("StepGeomBSplineCurve"));
}

#[test]
fn runtime_type_step_geom_bezier_curve_and_rational_b_spline_curve() {
    let object = Handle::new(StepGeomBezierCurveAndRationalBSplineCurve::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomBezierCurveAndRationalBSplineCurve"
    );
    assert!(object.is_kind_name("StepGeomBSplineCurve"));
}

#[test]
fn init_round_trip_step_geom_bezier_curve_and_rational_b_spline_curve() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_degree = 7;
    let a_control_points_list =
        Handle::new(HArray1::<Handle<StepGeomCartesianPoint>>::with_bounds(1, 2).unwrap());
    let a_curve_form = StepGeomBSplineCurveForm::default();
    let a_closed_curve = StepDataLogical::default();
    let a_self_intersect = StepDataLogical::default();
    let a_bezier_curve = Handle::new(StepGeomBezierCurve::new());
    let a_rational_b_spline_curve = Handle::new(StepGeomRationalBSplineCurve::new());
    let object = StepGeomBezierCurveAndRationalBSplineCurve::new();
    object.init(
        &a_name,
        a_degree,
        &a_control_points_list,
        a_curve_form,
        a_closed_curve,
        a_self_intersect,
        &a_bezier_curve,
        &a_rational_b_spline_curve,
    );
    assert!(object.bezier_curve() == a_bezier_curve);
    assert!(object.rational_b_spline_curve() == a_rational_b_spline_curve);
}

#[test]
fn runtime_type_step_geom_bezier_surface() {
    let object = Handle::new(StepGeomBezierSurface::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomBezierSurface"
    );
    assert!(object.is_kind_name("StepGeomBSplineSurface"));
}

#[test]
fn runtime_type_step_geom_bezier_surface_and_rational_b_spline_surface() {
    let object = Handle::new(StepGeomBezierSurfaceAndRationalBSplineSurface::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomBezierSurfaceAndRationalBSplineSurface"
    );
    assert!(object.is_kind_name("StepGeomBSplineSurface"));
}

#[test]
fn init_round_trip_step_geom_bezier_surface_and_rational_b_spline_surface() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_u_degree = 7;
    let a_v_degree = 7;
    let a_control_points_list =
        Handle::new(HArray2::<Handle<StepGeomCartesianPoint>>::with_bounds(1, 2, 1, 2).unwrap());
    let a_surface_form = StepGeomBSplineSurfaceForm::default();
    let a_u_closed = StepDataLogical::default();
    let a_v_closed = StepDataLogical::default();
    let a_self_intersect = StepDataLogical::default();
    let a_bezier_surface = Handle::new(StepGeomBezierSurface::new());
    let a_rational_b_spline_surface = Handle::new(StepGeomRationalBSplineSurface::new());
    let object = StepGeomBezierSurfaceAndRationalBSplineSurface::new();
    object.init(
        &a_name,
        a_u_degree,
        a_v_degree,
        &a_control_points_list,
        a_surface_form,
        a_u_closed,
        a_v_closed,
        a_self_intersect,
        &a_bezier_surface,
        &a_rational_b_spline_surface,
    );
    assert!(object.bezier_surface() == a_bezier_surface);
    assert!(object.rational_b_spline_surface() == a_rational_b_spline_surface);
}
