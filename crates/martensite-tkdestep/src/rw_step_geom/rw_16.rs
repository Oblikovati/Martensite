#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepGeomRwQuasiUniformCurve,
    "RwStepGeomRwQuasiUniformCurve"
);
impl RwStepGeomRwQuasiUniformCurve {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomQuasiUniformCurve>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 6, ach, "quasi_uniform_curve")? {
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
        ent: &Handle<StepGeomQuasiUniformCurve>,
    ) -> Result<(), Failure> {
        rw_block_0061!(ent, i3, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomQuasiUniformCurve>,
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
    RwStepGeomRwQuasiUniformCurveAndRationalBSplineCurve,
    "RwStepGeomRwQuasiUniformCurveAndRationalBSplineCurve"
);
impl RwStepGeomRwQuasiUniformCurveAndRationalBSplineCurve {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomQuasiUniformCurveAndRationalBSplineCurve>,
    ) -> Result<(), Failure> {
        let mut num: i32 = num0;
        rw_block_0007!(
            a_closed_curve,
            a_control_points_list,
            a_curve_form,
            a_degree,
            a_self_intersect,
            ach,
            anent2,
            data,
            i2,
            nb2,
            nsub2,
            num,
            text
        );
        if !data.check_nb_params(num, 0, ach, "quasi_uniform_curve")? {
            return Ok(());
        }
        num = data.next_for_complex(num);
        rw_block_0041!(
            a_name,
            a_weights_data,
            a_weights_data_item,
            ach,
            data,
            i6,
            nb6,
            nsub6,
            num
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
        ent: &Handle<StepGeomQuasiUniformCurveAndRationalBSplineCurve>,
    ) -> Result<(), Failure> {
        rw_block_0029!(ent, i2, sw);
        step_start_entity(sw, "QUASI_UNIFORM_CURVE")?;
        rw_block_0095!(ent, i6, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomQuasiUniformCurveAndRationalBSplineCurve>,
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
    RwStepGeomRwQuasiUniformSurface,
    "RwStepGeomRwQuasiUniformSurface"
);
impl RwStepGeomRwQuasiUniformSurface {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomQuasiUniformSurface>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 8, ach, "quasi_uniform_surface")? {
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
        ent: &Handle<StepGeomQuasiUniformSurface>,
    ) -> Result<(), Failure> {
        rw_block_0035!(ent, i4, j4, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomQuasiUniformSurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0177!(ent, is1, iter, js1, nbi_elem1, nbj_elem1);
        Ok(())
    }
}

step_rw_class!(
    RwStepGeomRwQuasiUniformSurfaceAndRationalBSplineSurface,
    "RwStepGeomRwQuasiUniformSurfaceAndRationalBSplineSurface"
);
impl RwStepGeomRwQuasiUniformSurfaceAndRationalBSplineSurface {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomQuasiUniformSurfaceAndRationalBSplineSurface>,
    ) -> Result<(), Failure> {
        let mut num: i32 = num0;
        rw_block_0006!(
            a_control_points_list,
            a_self_intersect,
            a_surface_form,
            a_u_closed,
            a_u_degree,
            a_v_closed,
            a_v_degree,
            ach,
            anent3,
            data,
            i3,
            j3,
            nbi3,
            nbj3,
            nsi3,
            nsub3,
            num,
            text
        );
        if !data.check_nb_params(num, 0, ach, "quasi_uniform_surface")? {
            return Ok(());
        }
        num = data.next_for_complex(num);
        rw_block_0146!(a_weights_data, a_weights_data_item, ach, data, nsub8, num);
        rw_block_0062!(
            a_name,
            a_weights_data,
            a_weights_data_item,
            ach,
            data,
            i8,
            j8,
            nbi8,
            nbj8,
            nsi8,
            nsub8,
            num
        );
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
        ent: &Handle<StepGeomQuasiUniformSurfaceAndRationalBSplineSurface>,
    ) -> Result<(), Failure> {
        rw_block_0021!(ent, i3, j3, sw);
        step_start_entity(sw, "QUASI_UNIFORM_SURFACE")?;
        rw_block_0070!(ent, i8, j8, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomQuasiUniformSurfaceAndRationalBSplineSurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0177!(ent, is1, iter, js1, nbi_elem1, nbj_elem1);
        Ok(())
    }
}
