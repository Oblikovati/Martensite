#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

#[test]
fn init_round_trip_step_geom_su_parameters() {
    let the_representation_item_name = Handle::new(HAsciiString::new("sample"));
    let the_a = 2.5;
    let the_alpha = 2.5;
    let the_b = 2.5;
    let the_beta = 2.5;
    let the_c = 2.5;
    let the_gamma = 2.5;
    let object = StepGeomSuParameters::new();
    object.init(
        &the_representation_item_name,
        the_a,
        the_alpha,
        the_b,
        the_beta,
        the_c,
        the_gamma,
    );
    assert!(object.a() == the_a);
    assert!(object.alpha() == the_alpha);
    assert!(object.b() == the_b);
    assert!(object.beta() == the_beta);
    assert!(object.c() == the_c);
    assert!(object.gamma() == the_gamma);
}

#[test]
fn runtime_type_step_geom_surface() {
    let object = Handle::new(StepGeomSurface::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepGeomSurface");
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}

#[test]
fn runtime_type_step_geom_surface_curve() {
    let object = Handle::new(StepGeomSurfaceCurve::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomSurfaceCurve"
    );
    assert!(object.is_kind_name("StepGeomCurve"));
}

#[test]
fn init_round_trip_step_geom_surface_curve() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_curve3d = Handle::new(StepGeomCurve::new());
    let a_associated_geometry =
        Handle::new(HArray1::<StepGeomPcurveOrSurface>::with_bounds(1, 2).unwrap());
    let a_master_representation = StepGeomPreferredSurfaceCurveRepresentation::default();
    let object = StepGeomSurfaceCurve::new();
    object.init(
        &a_name,
        &a_curve3d,
        &a_associated_geometry,
        a_master_representation,
    );
    assert!(object.curve3d() == a_curve3d);
    assert!(object.associated_geometry() == a_associated_geometry);
    assert!(object.master_representation() == a_master_representation);
}

#[test]
fn runtime_type_step_geom_surface_curve_and_bounded_curve() {
    let object = Handle::new(StepGeomSurfaceCurveAndBoundedCurve::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomSurfaceCurveAndBoundedCurve"
    );
    assert!(object.is_kind_name("StepGeomSurfaceCurve"));
}

#[test]
fn runtime_type_step_geom_surface_of_linear_extrusion() {
    let object = Handle::new(StepGeomSurfaceOfLinearExtrusion::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomSurfaceOfLinearExtrusion"
    );
    assert!(object.is_kind_name("StepGeomSweptSurface"));
}

#[test]
fn init_round_trip_step_geom_surface_of_linear_extrusion() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_swept_curve = Handle::new(StepGeomCurve::new());
    let a_extrusion_axis = Handle::new(StepGeomVector::new());
    let object = StepGeomSurfaceOfLinearExtrusion::new();
    object.init(&a_name, &a_swept_curve, &a_extrusion_axis);
    assert!(object.extrusion_axis() == a_extrusion_axis);
}

#[test]
fn runtime_type_step_geom_surface_of_revolution() {
    let object = Handle::new(StepGeomSurfaceOfRevolution::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomSurfaceOfRevolution"
    );
    assert!(object.is_kind_name("StepGeomSweptSurface"));
}

#[test]
fn init_round_trip_step_geom_surface_of_revolution() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_swept_curve = Handle::new(StepGeomCurve::new());
    let a_axis_position = Handle::new(StepGeomAxis1Placement::new());
    let object = StepGeomSurfaceOfRevolution::new();
    object.init(&a_name, &a_swept_curve, &a_axis_position);
    assert!(object.axis_position() == a_axis_position);
}

#[test]
fn runtime_type_step_geom_surface_patch() {
    let object = Handle::new(StepGeomSurfacePatch::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomSurfacePatch"
    );
    assert!(object.is_kind_name("Standard_Transient"));
}

#[test]
fn init_round_trip_step_geom_surface_patch() {
    let a_parent_surface = Handle::new(StepGeomBoundedSurface::new());
    let a_u_transition = StepGeomTransitionCode::default();
    let a_v_transition = StepGeomTransitionCode::default();
    let a_u_sense = true;
    let a_v_sense = true;
    let object = StepGeomSurfacePatch::new();
    object.init(
        &a_parent_surface,
        a_u_transition,
        a_v_transition,
        a_u_sense,
        a_v_sense,
    );
    assert!(object.parent_surface() == a_parent_surface);
    assert!(object.u_transition() == a_u_transition);
    assert!(object.v_transition() == a_v_transition);
    assert!(object.u_sense() == a_u_sense);
    assert!(object.v_sense() == a_v_sense);
}

#[test]
fn runtime_type_step_geom_surface_replica() {
    let object = Handle::new(StepGeomSurfaceReplica::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomSurfaceReplica"
    );
    assert!(object.is_kind_name("StepGeomSurface"));
}

#[test]
fn init_round_trip_step_geom_surface_replica() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_parent_surface = Handle::new(StepGeomSurface::new());
    let a_transformation = Handle::new(StepGeomCartesianTransformationOperator3d::new());
    let object = StepGeomSurfaceReplica::new();
    object.init(&a_name, &a_parent_surface, &a_transformation);
    assert!(object.parent_surface() == a_parent_surface);
    assert!(object.transformation() == a_transformation);
}

#[test]
fn runtime_type_step_geom_swept_surface() {
    let object = Handle::new(StepGeomSweptSurface::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomSweptSurface"
    );
    assert!(object.is_kind_name("StepGeomSurface"));
}

#[test]
fn init_round_trip_step_geom_swept_surface() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_swept_curve = Handle::new(StepGeomCurve::new());
    let object = StepGeomSweptSurface::new();
    object.init(&a_name, &a_swept_curve);
    assert!(object.swept_curve() == a_swept_curve);
}

#[test]
fn runtime_type_step_geom_toroidal_surface() {
    let object = Handle::new(StepGeomToroidalSurface::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomToroidalSurface"
    );
    assert!(object.is_kind_name("StepGeomElementarySurface"));
}

#[test]
fn init_round_trip_step_geom_toroidal_surface() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_position = Handle::new(StepGeomAxis2Placement3d::new());
    let a_major_radius = 2.5;
    let a_minor_radius = 2.5;
    let object = StepGeomToroidalSurface::new();
    object.init(&a_name, &a_position, a_major_radius, a_minor_radius);
    assert!(object.major_radius() == a_major_radius);
    assert!(object.minor_radius() == a_minor_radius);
}

#[test]
fn runtime_type_step_geom_trimmed_curve() {
    let object = Handle::new(StepGeomTrimmedCurve::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomTrimmedCurve"
    );
    assert!(object.is_kind_name("StepGeomBoundedCurve"));
}

#[test]
fn init_round_trip_step_geom_trimmed_curve() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_basis_curve = Handle::new(StepGeomCurve::new());
    let a_trim1 = Handle::new(HArray1::<StepGeomTrimmingSelect>::with_bounds(1, 2).unwrap());
    let a_trim2 = Handle::new(HArray1::<StepGeomTrimmingSelect>::with_bounds(1, 2).unwrap());
    let a_sense_agreement = true;
    let a_master_representation = StepGeomTrimmingPreference::default();
    let object = StepGeomTrimmedCurve::new();
    object.init(
        &a_name,
        &a_basis_curve,
        &a_trim1,
        &a_trim2,
        a_sense_agreement,
        a_master_representation,
    );
    assert!(object.basis_curve() == a_basis_curve);
    assert!(object.trim1() == a_trim1);
    assert!(object.trim2() == a_trim2);
    assert!(object.sense_agreement() == a_sense_agreement);
    assert!(object.master_representation() == a_master_representation);
}

#[test]
fn runtime_type_step_geom_uniform_curve() {
    let object = Handle::new(StepGeomUniformCurve::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomUniformCurve"
    );
    assert!(object.is_kind_name("StepGeomBSplineCurve"));
}

#[test]
fn runtime_type_step_geom_uniform_curve_and_rational_b_spline_curve() {
    let object = Handle::new(StepGeomUniformCurveAndRationalBSplineCurve::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomUniformCurveAndRationalBSplineCurve"
    );
    assert!(object.is_kind_name("StepGeomBSplineCurve"));
}

#[test]
fn init_round_trip_step_geom_uniform_curve_and_rational_b_spline_curve() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_degree = 7;
    let a_control_points_list =
        Handle::new(HArray1::<Handle<StepGeomCartesianPoint>>::with_bounds(1, 2).unwrap());
    let a_curve_form = StepGeomBSplineCurveForm::default();
    let a_closed_curve = StepDataLogical::default();
    let a_self_intersect = StepDataLogical::default();
    let a_uniform_curve = Handle::new(StepGeomUniformCurve::new());
    let a_rational_b_spline_curve = Handle::new(StepGeomRationalBSplineCurve::new());
    let object = StepGeomUniformCurveAndRationalBSplineCurve::new();
    object.init(
        &a_name,
        a_degree,
        &a_control_points_list,
        a_curve_form,
        a_closed_curve,
        a_self_intersect,
        &a_uniform_curve,
        &a_rational_b_spline_curve,
    );
    assert!(object.uniform_curve() == a_uniform_curve);
    assert!(object.rational_b_spline_curve() == a_rational_b_spline_curve);
}

#[test]
fn runtime_type_step_geom_uniform_surface() {
    let object = Handle::new(StepGeomUniformSurface::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomUniformSurface"
    );
    assert!(object.is_kind_name("StepGeomBSplineSurface"));
}

#[test]
fn runtime_type_step_geom_uniform_surface_and_rational_b_spline_surface() {
    let object = Handle::new(StepGeomUniformSurfaceAndRationalBSplineSurface::new());
    assert_eq!(
        object.dynamic_type().unwrap().name(),
        "StepGeomUniformSurfaceAndRationalBSplineSurface"
    );
    assert!(object.is_kind_name("StepGeomBSplineSurface"));
}

#[test]
fn init_round_trip_step_geom_uniform_surface_and_rational_b_spline_surface() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_u_degree = 7;
    let a_v_degree = 7;
    let a_control_points_list =
        Handle::new(HArray2::<Handle<StepGeomCartesianPoint>>::with_bounds(1, 2, 1, 2).unwrap());
    let a_surface_form = StepGeomBSplineSurfaceForm::default();
    let a_u_closed = StepDataLogical::default();
    let a_v_closed = StepDataLogical::default();
    let a_self_intersect = StepDataLogical::default();
    let a_uniform_surface = Handle::new(StepGeomUniformSurface::new());
    let a_rational_b_spline_surface = Handle::new(StepGeomRationalBSplineSurface::new());
    let object = StepGeomUniformSurfaceAndRationalBSplineSurface::new();
    object.init(
        &a_name,
        a_u_degree,
        a_v_degree,
        &a_control_points_list,
        a_surface_form,
        a_u_closed,
        a_v_closed,
        a_self_intersect,
        &a_uniform_surface,
        &a_rational_b_spline_surface,
    );
    assert!(object.uniform_surface() == a_uniform_surface);
    assert!(object.rational_b_spline_surface() == a_rational_b_spline_surface);
}

#[test]
fn runtime_type_step_geom_vector() {
    let object = Handle::new(StepGeomVector::new());
    assert_eq!(object.dynamic_type().unwrap().name(), "StepGeomVector");
    assert!(object.is_kind_name("StepGeomGeometricRepresentationItem"));
}

#[test]
fn init_round_trip_step_geom_vector() {
    let a_name = Handle::new(HAsciiString::new("sample"));
    let a_orientation = Handle::new(StepGeomDirection::new());
    let a_magnitude = 2.5;
    let object = StepGeomVector::new();
    object.init(&a_name, &a_orientation, a_magnitude);
    assert!(object.orientation() == a_orientation);
    assert!(object.magnitude() == a_magnitude);
}
