#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepGeomRwBSplineSurfaceWithKnots,
    "RwStepGeomRwBSplineSurfaceWithKnots"
);
impl RwStepGeomRwBSplineSurfaceWithKnots {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomBSplineSurfaceWithKnots>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 13, ach, "b_spline_surface_with_knots")? {
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
        let mut a_u_multiplicities: Handle<HArray1<i32>> = Default::default();
        let mut a_u_multiplicities_item: i32 = Default::default();
        let mut nsub9: i32 = Default::default();
        if data.read_sub_list(num, 9, "u_multiplicities", ach, &mut nsub9, false, 0, 0)? {
            let nb9: i32 = data.nb_params(nsub9)?;
            a_u_multiplicities = step_new_array1::<i32>(1, nb9)?;
            for i9 in 1..=nb9 {
                if data.read_integer(
                    nsub9,
                    i9,
                    "u_multiplicities",
                    ach,
                    &mut a_u_multiplicities_item,
                )? {
                    step_array1_set(&a_u_multiplicities, i9, a_u_multiplicities_item, "ReadStep")?;
                }
            }
        }
        let mut a_v_multiplicities: Handle<HArray1<i32>> = Default::default();
        let mut a_v_multiplicities_item: i32 = Default::default();
        let mut nsub10: i32 = Default::default();
        if data.read_sub_list(num, 10, "v_multiplicities", ach, &mut nsub10, false, 0, 0)? {
            let nb10: i32 = data.nb_params(nsub10)?;
            a_v_multiplicities = step_new_array1::<i32>(1, nb10)?;
            for i10 in 1..=nb10 {
                if data.read_integer(
                    nsub10,
                    i10,
                    "v_multiplicities",
                    ach,
                    &mut a_v_multiplicities_item,
                )? {
                    step_array1_set(
                        &a_v_multiplicities,
                        i10,
                        a_v_multiplicities_item,
                        "ReadStep",
                    )?;
                }
            }
        }
        let mut a_u_knots: Handle<HArray1<f64>> = Default::default();
        let mut a_u_knots_item: f64 = Default::default();
        let mut nsub11: i32 = Default::default();
        if data.read_sub_list(num, 11, "u_knots", ach, &mut nsub11, false, 0, 0)? {
            let nb11: i32 = data.nb_params(nsub11)?;
            a_u_knots = step_new_array1::<f64>(1, nb11)?;
            for i11 in 1..=nb11 {
                if data.read_real(nsub11, i11, "u_knots", ach, &mut a_u_knots_item)? {
                    step_array1_set(&a_u_knots, i11, a_u_knots_item, "ReadStep")?;
                }
            }
        }
        let mut a_v_knots: Handle<HArray1<f64>> = Default::default();
        let mut a_v_knots_item: f64 = Default::default();
        let mut nsub12: i32 = Default::default();
        if data.read_sub_list(num, 12, "v_knots", ach, &mut nsub12, false, 0, 0)? {
            let nb12: i32 = data.nb_params(nsub12)?;
            a_v_knots = step_new_array1::<f64>(1, nb12)?;
            for i12 in 1..=nb12 {
                if data.read_real(nsub12, i12, "v_knots", ach, &mut a_v_knots_item)? {
                    step_array1_set(&a_v_knots, i12, a_v_knots_item, "ReadStep")?;
                }
            }
        }
        let mut a_knot_spec: StepGeomKnotType = StepGeomKnotType::KtUniformKnots;
        if data.param_type(num, 13)? == InterfaceParamType::ParamEnum {
            let text: Vec<u8> = data.param_c_value(num, 13)?;
            if !RwStepGeomRwKnotType::convert_to_enum(&text, &mut a_knot_spec) {
                ach.add_fail_text("Enumeration knot_type has not an allowed value", "");
            }
        } else {
            ach.add_fail_text("Parameter #13 (knot_spec) is not an enumeration", "");
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
            &a_u_multiplicities,
            &a_v_multiplicities,
            &a_u_knots,
            &a_v_knots,
            a_knot_spec,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomBSplineSurfaceWithKnots>,
    ) -> Result<(), Failure> {
        rw_block_0027!(ent, i4, j4, sw);
        for i9 in 1..=ent.nb_u_multiplicities()? {
            sw.send_integer(ent.u_multiplicities_value(i9)?)?;
        }
        sw.close_sub()?;
        sw.open_sub()?;
        for i10 in 1..=ent.nb_v_multiplicities()? {
            sw.send_integer(ent.v_multiplicities_value(i10)?)?;
        }
        sw.close_sub()?;
        sw.open_sub()?;
        for i11 in 1..=ent.nb_u_knots()? {
            sw.send_real(ent.u_knots_value(i11)?)?;
        }
        sw.close_sub()?;
        sw.open_sub()?;
        for i12 in 1..=ent.nb_v_knots()? {
            sw.send_real(ent.v_knots_value(i12)?)?;
        }
        sw.close_sub()?;
        sw.send_enum(RwStepGeomRwKnotType::convert_to_string(ent.knot_spec()))?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomBSplineSurfaceWithKnots>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0177!(ent, is1, iter, js1, nbi_elem1, nbj_elem1);
        Ok(())
    }
    /// `Check`.
    pub fn check(
        &self,
        ent: &Handle<StepGeomBSplineSurfaceWithKnots>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let nb_cplu: i32 = ent.nb_control_points_list_i();
        let nb_cplv: i32 = ent.nb_control_points_list_j();
        let dg_bssu: i32 = ent.u_degree();
        let dg_bssv: i32 = ent.v_degree();
        let nb_mul_u: i32 = ent.nb_u_multiplicities()?;
        let nb_mul_v: i32 = ent.nb_v_multiplicities()?;
        let nb_kno_u: i32 = ent.nb_u_knots()?;
        let nb_kno_v: i32 = ent.nb_v_knots()?;
        let mut sum_mul_u: i32 = 0;
        let mut sum_mul_v: i32 = 0;
        let i: i32 = Default::default();
        if nb_mul_u != nb_kno_u {
            ach.add_fail_text(
                "ERROR: No.of KnotMultiplicities not equal No.of Knots in U",
                "",
            );
        }
        if nb_mul_v != nb_kno_v {
            ach.add_fail_text(
                "ERROR: No.of KnotMultiplicities not equal No.of Knots in V",
                "",
            );
        }
        if nb_mul_u == 0 {
            ach.add_warning_text("WARNING: No.of KnotMultiplicities in U is zero", "");
            return Ok(());
        }
        if nb_mul_v == 0 {
            ach.add_warning_text("WARNING: No.of KnotMultiplicities in V is zero", "");
            return Ok(());
        }
        for i in 1..=nb_mul_u - 1 {
            sum_mul_u = sum_mul_u + ent.u_multiplicities_value(i)?;
        }
        let sum_non_pu: i32 = nb_cplu + dg_bssu + 1;
        let mult1_u: i32 = ent.u_multiplicities_value(1)?;
        let mult_nu: i32 = ent.u_multiplicities_value(nb_mul_u)?;
        if (sum_mul_u + mult_nu) == sum_non_pu {
        } else {
            if (sum_mul_u == nb_cplu) && (mult1_u == mult_nu) {
            } else {
                ach.add_fail_text("ERROR: wrong number of Knot Multiplicities in U", "");
            }
        }
        for i in 2..=nb_kno_u {
            let dist_kn: f64 = ent.u_knots_value(i - 1)? - ent.u_knots_value(i)?;
            if dist_kn.abs() <= f64::EPSILON {
                ach.add_warning_text("WARNING: Surface contains identical KnotsValues in U", "");
            } else {
                if dist_kn > f64::EPSILON {
                    ach.add_fail_text("ERROR: Surface contains descending KnotsValues in U", "");
                }
            }
        }
        for i in 1..=nb_mul_v - 1 {
            sum_mul_v = sum_mul_v + ent.v_multiplicities_value(i)?;
        }
        let sum_non_pv: i32 = nb_cplv + dg_bssv + 1;
        let mult1_v: i32 = ent.v_multiplicities_value(1)?;
        let mult_nv: i32 = ent.v_multiplicities_value(nb_mul_v)?;
        if (sum_mul_v + mult_nv) == sum_non_pv {
        } else {
            if (sum_mul_v == nb_cplv) && (mult1_v == mult_nv) {
            } else {
                ach.add_fail_text("ERROR: wrong number of Knot Multiplicities in V", "");
            }
        }
        for i in 2..=nb_kno_v {
            let dist_kn: f64 = ent.v_knots_value(i - 1)? - ent.v_knots_value(i)?;
            if dist_kn.abs() <= f64::EPSILON {
                ach.add_warning_text("WARNING: Surface contains identical KnotsValues in V", "");
            } else {
                if dist_kn > f64::EPSILON {
                    ach.add_fail_text("ERROR: Surface contains descending KnotsValues in V", "");
                }
            }
        }
        Ok(())
    }
}
