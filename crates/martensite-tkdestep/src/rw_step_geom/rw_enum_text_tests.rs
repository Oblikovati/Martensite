use crate::step_rw::*;

#[test]
fn enum_text_round_trip_rw_step_geom_rwb_spline_curve_form() {
    for value in [
        StepGeomBSplineCurveForm::BscfEllipticArc,
        StepGeomBSplineCurveForm::BscfPolylineForm,
        StepGeomBSplineCurveForm::BscfParabolicArc,
        StepGeomBSplineCurveForm::BscfCircularArc,
        StepGeomBSplineCurveForm::BscfUnspecified,
        StepGeomBSplineCurveForm::BscfHyperbolicArc,
    ] {
        let mut read = StepGeomBSplineCurveForm::default();
        assert!(RwStepGeomRwBSplineCurveForm::convert_to_enum(
            RwStepGeomRwBSplineCurveForm::convert_to_string(value),
            &mut read
        ));
        assert_eq!(read, value);
    }
    let mut read = StepGeomBSplineCurveForm::default();
    assert!(!RwStepGeomRwBSplineCurveForm::convert_to_enum(
        b".NOT_A_VALUE.",
        &mut read
    ));
}

#[test]
fn enum_text_round_trip_rw_step_geom_rwb_spline_surface_form() {
    for value in [
        StepGeomBSplineSurfaceForm::BssfSurfOfLinearExtrusion,
        StepGeomBSplineSurfaceForm::BssfPlaneSurf,
        StepGeomBSplineSurfaceForm::BssfGeneralisedCone,
        StepGeomBSplineSurfaceForm::BssfToroidalSurf,
        StepGeomBSplineSurfaceForm::BssfConicalSurf,
        StepGeomBSplineSurfaceForm::BssfSphericalSurf,
        StepGeomBSplineSurfaceForm::BssfUnspecified,
        StepGeomBSplineSurfaceForm::BssfRuledSurf,
        StepGeomBSplineSurfaceForm::BssfSurfOfRevolution,
        StepGeomBSplineSurfaceForm::BssfCylindricalSurf,
        StepGeomBSplineSurfaceForm::BssfQuadricSurf,
    ] {
        let mut read = StepGeomBSplineSurfaceForm::default();
        assert!(RwStepGeomRwBSplineSurfaceForm::convert_to_enum(
            RwStepGeomRwBSplineSurfaceForm::convert_to_string(value),
            &mut read
        ));
        assert_eq!(read, value);
    }
    let mut read = StepGeomBSplineSurfaceForm::default();
    assert!(!RwStepGeomRwBSplineSurfaceForm::convert_to_enum(
        b".NOT_A_VALUE.",
        &mut read
    ));
}

#[test]
fn enum_text_round_trip_rw_step_geom_rw_knot_type() {
    for value in [
        StepGeomKnotType::KtUniformKnots,
        StepGeomKnotType::KtQuasiUniformKnots,
        StepGeomKnotType::KtPiecewiseBezierKnots,
        StepGeomKnotType::KtUnspecified,
    ] {
        let mut read = StepGeomKnotType::default();
        assert!(RwStepGeomRwKnotType::convert_to_enum(
            RwStepGeomRwKnotType::convert_to_string(value),
            &mut read
        ));
        assert_eq!(read, value);
    }
    let mut read = StepGeomKnotType::default();
    assert!(!RwStepGeomRwKnotType::convert_to_enum(
        b".NOT_A_VALUE.",
        &mut read
    ));
}

#[test]
fn enum_text_round_trip_rw_step_geom_rw_preferred_surface_curve_representation() {
    for value in [
        StepGeomPreferredSurfaceCurveRepresentation::PscrPcurveS2,
        StepGeomPreferredSurfaceCurveRepresentation::PscrPcurveS1,
        StepGeomPreferredSurfaceCurveRepresentation::PscrCurve3d,
    ] {
        let mut read = StepGeomPreferredSurfaceCurveRepresentation::default();
        assert!(
            RwStepGeomRwPreferredSurfaceCurveRepresentation::convert_to_enum(
                RwStepGeomRwPreferredSurfaceCurveRepresentation::convert_to_string(value),
                &mut read
            )
        );
        assert_eq!(read, value);
    }
    let mut read = StepGeomPreferredSurfaceCurveRepresentation::default();
    assert!(
        !RwStepGeomRwPreferredSurfaceCurveRepresentation::convert_to_enum(
            b".NOT_A_VALUE.",
            &mut read
        )
    );
}

#[test]
fn enum_text_round_trip_rw_step_geom_rw_transition_code() {
    for value in [
        StepGeomTransitionCode::TcDiscontinuous,
        StepGeomTransitionCode::TcContSameGradientSameCurvature,
        StepGeomTransitionCode::TcContSameGradient,
        StepGeomTransitionCode::TcContinuous,
    ] {
        let mut read = StepGeomTransitionCode::default();
        assert!(RwStepGeomRwTransitionCode::convert_to_enum(
            RwStepGeomRwTransitionCode::convert_to_string(value),
            &mut read
        ));
        assert_eq!(read, value);
    }
    let mut read = StepGeomTransitionCode::default();
    assert!(!RwStepGeomRwTransitionCode::convert_to_enum(
        b".NOT_A_VALUE.",
        &mut read
    ));
}

#[test]
fn enum_text_round_trip_rw_step_geom_rw_trimming_preference() {
    for value in [
        StepGeomTrimmingPreference::TpParameter,
        StepGeomTrimmingPreference::TpUnspecified,
        StepGeomTrimmingPreference::TpCartesian,
    ] {
        let mut read = StepGeomTrimmingPreference::default();
        assert!(RwStepGeomRwTrimmingPreference::convert_to_enum(
            RwStepGeomRwTrimmingPreference::convert_to_string(value),
            &mut read
        ));
        assert_eq!(read, value);
    }
    let mut read = StepGeomTrimmingPreference::default();
    assert!(!RwStepGeomRwTrimmingPreference::convert_to_enum(
        b".NOT_A_VALUE.",
        &mut read
    ));
}
