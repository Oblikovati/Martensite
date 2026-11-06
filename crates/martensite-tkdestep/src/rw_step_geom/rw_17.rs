#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepGeomRwRationalBSplineCurve,
    "RwStepGeomRwRationalBSplineCurve"
);
impl RwStepGeomRwRationalBSplineCurve {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomRationalBSplineCurve>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 7, ach, "rational_b_spline_curve")? {
            return Ok(());
        }
        rw_block_0014!(
            a_closed_curve,
            a_control_points_list,
            a_curve_form,
            a_degree,
            a_name,
            a_self_intersect,
            ach,
            anent3,
            data,
            i3,
            nb3,
            nsub3,
            num,
            text
        );
        let mut a_weights_data: Handle<HArray1<f64>> = Default::default();
        let mut a_weights_data_item: f64 = Default::default();
        let mut nsub7: i32 = Default::default();
        if data.read_sub_list(num, 7, "weights_data", ach, &mut nsub7, false, 0, 0)? {
            let nb7: i32 = data.nb_params(nsub7)?;
            a_weights_data = step_new_array1::<f64>(1, nb7)?;
            for i7 in 1..=nb7 {
                if data.read_real(nsub7, i7, "weights_data", ach, &mut a_weights_data_item)? {
                    step_array1_set(&a_weights_data, i7, a_weights_data_item, "ReadStep")?;
                }
            }
        }
        ent.init(
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
        ent: &Handle<StepGeomRationalBSplineCurve>,
    ) -> Result<(), Failure> {
        rw_block_0049!(ent, i3, sw);
        for i7 in 1..=ent.nb_weights_data()? {
            sw.send_real(ent.weights_data_value(i7)?)?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomRationalBSplineCurve>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_control_points_list();
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.control_points_list_value(is1)?.cast());
        }
        Ok(())
    }
    /// `Check`.
    pub fn check(
        &self,
        ent: &Handle<StepGeomRationalBSplineCurve>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let nb_wght: i32 = ent.nb_weights_data()?;
        let nb_cpl: i32 = ent.nb_control_points_list();
        if nb_wght != nb_cpl {
            ach.add_fail_text("ERROR: No.of ControlPoints not equal No.of Weights", "");
        }
        for i in 1..=nb_wght {
            if ent.weights_data_value(i)? < f64::EPSILON {
                ach.add_fail_text("ERROR: WeightsData Value not greater than 0.0", "");
            }
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepGeomRwRationalBSplineSurface,
    "RwStepGeomRwRationalBSplineSurface"
);
impl RwStepGeomRwRationalBSplineSurface {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomRationalBSplineSurface>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 9, ach, "rational_b_spline_surface")? {
            return Ok(());
        }
        rw_block_0010!(
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
            i4,
            j4,
            nbi4,
            nbj4,
            nsi4,
            nsub4,
            num,
            text
        );
        let mut a_weights_data: Handle<HArray2<f64>> = Default::default();
        let mut a_weights_data_item: f64 = Default::default();
        let mut nsub9: i32 = Default::default();
        if data.read_sub_list(num, 9, "weights_data", ach, &mut nsub9, false, 0, 0)? {
            let nbi9: i32 = data.nb_params(nsub9)?;
            let nbj9: i32 = data.nb_params(data.param_number(nsub9, 1)?)?;
            a_weights_data = step_new_array2::<f64>(1, nbi9, 1, nbj9)?;
            for i9 in 1..=nbi9 {
                let mut nsi9: i32 = Default::default();
                if data.read_sub_list(
                    nsub9,
                    i9,
                    "sub-part(weights_data)",
                    ach,
                    &mut nsi9,
                    false,
                    0,
                    0,
                )? {
                    for j9 in 1..=nbj9 {
                        if data.read_real(
                            nsi9,
                            j9,
                            "weights_data",
                            ach,
                            &mut a_weights_data_item,
                        )? {
                            step_array2_set(
                                &a_weights_data,
                                i9,
                                j9,
                                a_weights_data_item,
                                "ReadStep",
                            )?;
                        }
                    }
                }
            }
        }
        ent.init(
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
        ent: &Handle<StepGeomRationalBSplineSurface>,
    ) -> Result<(), Failure> {
        rw_block_0027!(ent, i4, j4, sw);
        for i9 in 1..=ent.nb_weights_data_i()? {
            sw.new_line(false)?;
            sw.open_sub()?;
            for j9 in 1..=ent.nb_weights_data_j()? {
                sw.send_real(ent.weights_data_value(i9, j9)?)?;
                sw.join_last(false);
            }
            sw.close_sub()?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomRationalBSplineSurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0177!(ent, is1, iter, js1, nbi_elem1, nbj_elem1);
        Ok(())
    }
    /// `Check`.
    pub fn check(
        &self,
        ent: &Handle<StepGeomRationalBSplineSurface>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let nb_wgt_u: i32 = ent.nb_weights_data_i()?;
        let nb_wgt_v: i32 = ent.nb_weights_data_j()?;
        let nb_cplu: i32 = ent.nb_control_points_list_i();
        let nb_cplv: i32 = ent.nb_control_points_list_j();
        let i: i32 = Default::default();
        let j: i32 = Default::default();
        if nb_wgt_u != nb_cplu {
            ach.add_fail_text(
                "ERROR: No.of ControlPoints not equal No.of Weights in U",
                "",
            );
        }
        if nb_wgt_v != nb_cplv {
            ach.add_fail_text(
                "ERROR: No.of ControlPoints not equal No.of Weights in V",
                "",
            );
        }
        for i in 1..=nb_wgt_u {
            for j in 1..=nb_wgt_v {
                if ent.weights_data_value(i, j)? < f64::EPSILON {
                    ach.add_fail_text("ERROR: WeightsData Value not greater than 0.0", "");
                }
            }
        }
        Ok(())
    }
}
