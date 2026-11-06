#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepGeomRwBezierCurve, "RwStepGeomRwBezierCurve");
impl RwStepGeomRwBezierCurve {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomBezierCurve>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 6, ach, "bezier_curve")? {
            return Ok(());
        }
        rw_block_0013!(
            a_closed_curve,
            a_control_points_list,
            a_curve_form,
            a_degree,
            a_name,
            a_self_intersect,
            ach,
            anent3,
            data,
            ent,
            i3,
            nb3,
            nsub3,
            num,
            text
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomBezierCurve>,
    ) -> Result<(), Failure> {
        rw_block_0061!(ent, i3, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomBezierCurve>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_control_points_list();
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.control_points_list_value(is1)?.cast());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepGeomRwBezierCurveAndRationalBSplineCurve,
    "RwStepGeomRwBezierCurveAndRationalBSplineCurve"
);
impl RwStepGeomRwBezierCurveAndRationalBSplineCurve {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomBezierCurveAndRationalBSplineCurve>,
    ) -> Result<(), Failure> {
        let mut num: i32 = num0;
        if !data.check_nb_params(num, 0, ach, "bezier_curve")? {
            return Ok(());
        }
        num = data.next_for_complex(num);
        rw_block_0001!(
            a_closed_curve,
            a_control_points_list,
            a_curve_form,
            a_degree,
            a_name,
            a_self_intersect,
            a_weights_data,
            a_weights_data_item,
            ach,
            anent2,
            data,
            i2,
            i6,
            nb2,
            nb6,
            nsub2,
            nsub6,
            num,
            text
        );
        ent.init_fields(
            &a_name,
            a_degree,
            &a_control_points_list,
            a_curve_form,
            a_closed_curve,
            a_self_intersect,
            &a_weights_data,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomBezierCurveAndRationalBSplineCurve>,
    ) -> Result<(), Failure> {
        step_start_entity(sw, "BEZIER_CURVE")?;
        rw_block_0011!(ent, i2, i6, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomBezierCurveAndRationalBSplineCurve>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_control_points_list();
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.control_points_list_value(is1)?.cast());
        }
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwBezierSurface, "RwStepGeomRwBezierSurface");
impl RwStepGeomRwBezierSurface {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomBezierSurface>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 8, ach, "bezier_surface")? {
            return Ok(());
        }
        rw_block_0008!(
            a_control_points_list,
            a_name,
            a_self_intersect,
            a_surface_form,
            a_u_closed,
            a_u_degree,
            a_v_closed,
            a_v_degree,
            ach,
            anent4,
            data,
            ent,
            i4,
            j4,
            nbi4,
            nbj4,
            nsi4,
            nsub4,
            num,
            text
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomBezierSurface>,
    ) -> Result<(), Failure> {
        rw_block_0035!(ent, i4, j4, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomBezierSurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0177!(ent, is1, iter, js1, nbi_elem1, nbj_elem1);
        Ok(())
    }
}

step_rw_class!(
    RwStepGeomRwBezierSurfaceAndRationalBSplineSurface,
    "RwStepGeomRwBezierSurfaceAndRationalBSplineSurface"
);
impl RwStepGeomRwBezierSurfaceAndRationalBSplineSurface {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomBezierSurfaceAndRationalBSplineSurface>,
    ) -> Result<(), Failure> {
        let mut num: i32 = num0;
        if !data.check_nb_params(num, 0, ach, "bezier_surface")? {
            return Ok(());
        }
        num = data.next_for_complex(num);
        rw_block_0002!(
            a_control_points_list,
            a_self_intersect,
            a_surface_form,
            a_u_closed,
            a_u_degree,
            a_v_closed,
            a_v_degree,
            a_weights_data,
            a_weights_data_item,
            ach,
            anent3,
            data,
            i3,
            j3,
            nbi3,
            nbj3,
            nsi3,
            nsub3,
            nsub8,
            num,
            text
        );
        if data.read_sub_list(num, 1, "items", ach, &mut nsub8, false, 0, 0)? {
            let nbi8: i32 = data.nb_params(nsub8)?;
            let nbj8: i32 = data.nb_params(data.param_number(nsub8, 1)?)?;
            a_weights_data = step_new_array2::<f64>(1, nbi8, 1, nbj8)?;
            for i8 in 1..=nbi8 {
                let mut nsi8: i32 = Default::default();
                if data.read_sub_list(
                    nsub8,
                    i8,
                    "sub-part(weights_data)",
                    ach,
                    &mut nsi8,
                    false,
                    0,
                    0,
                )? {
                    for j8 in 1..=nbj8 {
                        if data.read_real(
                            nsi8,
                            j8,
                            "weights_data",
                            ach,
                            &mut a_weights_data_item,
                        )? {
                            step_array2_set(
                                &a_weights_data,
                                i8,
                                j8,
                                a_weights_data_item,
                                "ReadStep",
                            )?;
                        }
                    }
                }
            }
        }
        rw_block_0089!(a_name, ach, data, num);
        ent.init_fields(
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
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomBezierSurfaceAndRationalBSplineSurface>,
    ) -> Result<(), Failure> {
        step_start_entity(sw, "BEZIER_SURFACE")?;
        rw_block_0009!(ent, i3, i8, j3, j8, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomBezierSurfaceAndRationalBSplineSurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0177!(ent, is1, iter, js1, nbi_elem1, nbj_elem1);
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwBoundaryCurve, "RwStepGeomRwBoundaryCurve");
impl RwStepGeomRwBoundaryCurve {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomBoundaryCurve>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "boundary_curve")? {
            return Ok(());
        }
        rw_block_0040!(
            a_name,
            a_segments,
            a_self_intersect,
            ach,
            anent2,
            data,
            ent,
            i2,
            nb2,
            nsub2,
            num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomBoundaryCurve>,
    ) -> Result<(), Failure> {
        rw_block_0117!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomBoundaryCurve>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_segments();
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.segments_value(is1)?.cast());
        }
        Ok(())
    }
}
