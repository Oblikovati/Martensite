#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_enum_text! {
    RwStepGeomRwBSplineCurveForm, StepGeomBSplineCurveForm, "StepGeomBSplineCurveForm", {
        BscfEllipticArc => ".ELLIPTIC_ARC.",
        BscfPolylineForm => ".POLYLINE_FORM.",
        BscfParabolicArc => ".PARABOLIC_ARC.",
        BscfCircularArc => ".CIRCULAR_ARC.",
        BscfUnspecified => ".UNSPECIFIED.",
        BscfHyperbolicArc => ".HYPERBOLIC_ARC.",
    }
}

step_enum_text! {
    RwStepGeomRwBSplineSurfaceForm, StepGeomBSplineSurfaceForm, "StepGeomBSplineSurfaceForm", {
        BssfSurfOfLinearExtrusion => ".SURF_OF_LINEAR_EXTRUSION.",
        BssfPlaneSurf => ".PLANE_SURF.",
        BssfGeneralisedCone => ".GENERALISED_CONE.",
        BssfToroidalSurf => ".TOROIDAL_SURF.",
        BssfConicalSurf => ".CONICAL_SURF.",
        BssfSphericalSurf => ".SPHERICAL_SURF.",
        BssfUnspecified => ".UNSPECIFIED.",
        BssfRuledSurf => ".RULED_SURF.",
        BssfSurfOfRevolution => ".SURF_OF_REVOLUTION.",
        BssfCylindricalSurf => ".CYLINDRICAL_SURF.",
        BssfQuadricSurf => ".QUADRIC_SURF.",
    }
}

step_enum_text! {
    RwStepGeomRwKnotType, StepGeomKnotType, "StepGeomKnotType", {
        KtUniformKnots => ".UNIFORM_KNOTS.",
        KtQuasiUniformKnots => ".QUASI_UNIFORM_KNOTS.",
        KtPiecewiseBezierKnots => ".PIECEWISE_BEZIER_KNOTS.",
        KtUnspecified => ".UNSPECIFIED.",
    }
}

step_enum_text! {
    RwStepGeomRwPreferredSurfaceCurveRepresentation, StepGeomPreferredSurfaceCurveRepresentation, "StepGeomPreferredSurfaceCurveRepresentation", {
        PscrPcurveS2 => ".PCURVE_S2.",
        PscrPcurveS1 => ".PCURVE_S1.",
        PscrCurve3d => ".CURVE_3D.",
    }
}

step_enum_text! {
    RwStepGeomRwTransitionCode, StepGeomTransitionCode, "StepGeomTransitionCode", {
        TcDiscontinuous => ".DISCONTINUOUS.",
        TcContSameGradientSameCurvature => ".CONT_SAME_GRADIENT_SAME_CURVATURE.",
        TcContSameGradient => ".CONT_SAME_GRADIENT.",
        TcContinuous => ".CONTINUOUS.",
    }
}

step_enum_text! {
    RwStepGeomRwTrimmingPreference, StepGeomTrimmingPreference, "StepGeomTrimmingPreference", {
        TpParameter => ".PARAMETER.",
        TpUnspecified => ".UNSPECIFIED.",
        TpCartesian => ".CARTESIAN.",
    }
}

step_rw_class!(RwStepGeomRwAxis1Placement, "RwStepGeomRwAxis1Placement");
impl RwStepGeomRwAxis1Placement {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomAxis1Placement>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "axis1_placement")? {
            return Ok(());
        }
        rw_block_0105!(a_axis, a_location, a_name, ach, data, num);
        let mut has_aaxis: bool = true;
        if data.is_param_defined(num, 3)? {
            data.read_entity(
                num,
                3,
                "axis",
                ach,
                StepGeomDirection::static_type(),
                &mut a_axis,
            )?;
        } else {
            has_aaxis = false;
            a_axis = Default::default();
        }
        ent.init(&a_name, &a_location, has_aaxis, &a_axis);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomAxis1Placement>,
    ) -> Result<(), Failure> {
        rw_block_0154!(ent, has_aaxis, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomAxis1Placement>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.location().cast());
        if ent.has_axis() {
            iter.get_one_item(&ent.axis().cast());
        }
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwAxis2Placement2d, "RwStepGeomRwAxis2Placement2d");
impl RwStepGeomRwAxis2Placement2d {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomAxis2Placement2d>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "axis2_placement_2d")? {
            return Ok(());
        }
        rw_block_0130!(a_location, a_name, ach, data, num);
        let mut a_ref_direction: Handle<StepGeomDirection> = Default::default();
        let mut has_aref_direction: bool = true;
        if data.is_param_defined(num, 3)? {
            data.read_entity(
                num,
                3,
                "ref_direction",
                ach,
                StepGeomDirection::static_type(),
                &mut a_ref_direction,
            )?;
        } else {
            has_aref_direction = false;
            a_ref_direction = Default::default();
        }
        ent.init(&a_name, &a_location, has_aref_direction, &a_ref_direction);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomAxis2Placement2d>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.location())?;
        let has_aref_direction: bool = ent.has_ref_direction();
        if has_aref_direction {
            sw.send_entity(&ent.ref_direction())?;
        } else {
            sw.send_undef()?;
        }
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomAxis2Placement2d>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.location().cast());
        if ent.has_ref_direction() {
            iter.get_one_item(&ent.ref_direction().cast());
        }
        Ok(())
    }
}
