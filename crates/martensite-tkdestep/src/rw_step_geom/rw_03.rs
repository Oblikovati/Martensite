#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepGeomRwBSplineCurveWithKnots,
    "RwStepGeomRwBSplineCurveWithKnots"
);
impl RwStepGeomRwBSplineCurveWithKnots {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomBSplineCurveWithKnots>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 9, ach, "b_spline_curve_with_knots")? {
            return Ok(());
        }
        rw_block_0074!(
            a_control_points_list,
            a_degree,
            a_name,
            ach,
            anent3,
            data,
            nsub3,
            num
        );
        if data.read_sub_list(num, 3, "control_points_list", ach, &mut nsub3, false, 0, 0)? {
            let nb3: i32 = data.nb_params(nsub3)?;
            if nb3 < 1 {
                ach.add_fail_text(
                    "Number of control points of the b_spline_curve_form is equal to 0",
                    "",
                );
            } else {
                a_control_points_list = step_new_array1::<Handle<StepGeomCartesianPoint>>(1, nb3)?;
                for i3 in 1..=nb3 {
                    if data.read_entity(
                        nsub3,
                        i3,
                        "cartesian_point",
                        ach,
                        StepGeomCartesianPoint::static_type(),
                        &mut anent3,
                    )? {
                        step_array1_set(&a_control_points_list, i3, anent3.clone(), "ReadStep")?;
                    }
                }
            }
        }
        rw_block_0080!(
            a_closed_curve,
            a_curve_form,
            a_self_intersect,
            ach,
            data,
            num,
            text
        );
        let mut a_knot_multiplicities: Handle<HArray1<i32>> = Default::default();
        let mut a_knot_multiplicities_item: i32 = Default::default();
        let mut nsub7: i32 = Default::default();
        if data.read_sub_list(num, 7, "knot_multiplicities", ach, &mut nsub7, false, 0, 0)? {
            let nb7: i32 = data.nb_params(nsub7)?;
            a_knot_multiplicities = step_new_array1::<i32>(1, nb7)?;
            for i7 in 1..=nb7 {
                if data.read_integer(
                    nsub7,
                    i7,
                    "knot_multiplicities",
                    ach,
                    &mut a_knot_multiplicities_item,
                )? {
                    step_array1_set(
                        &a_knot_multiplicities,
                        i7,
                        a_knot_multiplicities_item,
                        "ReadStep",
                    )?;
                }
            }
        }
        let mut a_knots: Handle<HArray1<f64>> = Default::default();
        let mut a_knots_item: f64 = Default::default();
        let mut nsub8: i32 = Default::default();
        if data.read_sub_list(num, 8, "knots", ach, &mut nsub8, false, 0, 0)? {
            let nb8: i32 = data.nb_params(nsub8)?;
            a_knots = step_new_array1::<f64>(1, nb8)?;
            for i8 in 1..=nb8 {
                if data.read_real(nsub8, i8, "knots", ach, &mut a_knots_item)? {
                    step_array1_set(&a_knots, i8, a_knots_item, "ReadStep")?;
                }
            }
        }
        let mut a_knot_spec: StepGeomKnotType = StepGeomKnotType::KtUniformKnots;
        if data.param_type(num, 9)? == InterfaceParamType::ParamEnum {
            let text: Vec<u8> = data.param_c_value(num, 9)?;
            if !RwStepGeomRwKnotType::convert_to_enum(&text, &mut a_knot_spec) {
                ach.add_fail_text("Enumeration knot_type has not an allowed value", "");
            }
        } else {
            ach.add_fail_text("Parameter #9 (knot_spec) is not an enumeration", "");
        }
        ent.init(
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
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomBSplineCurveWithKnots>,
    ) -> Result<(), Failure> {
        rw_block_0049!(ent, i3, sw);
        for i7 in 1..=ent.nb_knot_multiplicities()? {
            sw.send_integer(ent.knot_multiplicities_value(i7)?)?;
        }
        sw.close_sub()?;
        sw.open_sub()?;
        for i8 in 1..=ent.nb_knots()? {
            sw.send_real(ent.knots_value(i8)?)?;
        }
        sw.close_sub()?;
        sw.send_enum(RwStepGeomRwKnotType::convert_to_string(ent.knot_spec()))?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomBSplineCurveWithKnots>,
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
        ent: &Handle<StepGeomBSplineCurveWithKnots>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let nb_cpl: i32 = ent.nb_control_points_list();
        let dg_bsc: i32 = ent.degree();
        let nb_mult: i32 = ent.nb_knot_multiplicities()?;
        let nb_kno: i32 = ent.nb_knots()?;
        let mut sum_mult: i32 = 0;
        if nb_mult != nb_kno {
            ach.add_fail_text("ERROR: No.of KnotMultiplicities not equal No.of Knots", "");
        }
        if nb_mult == 0 {
            ach.add_fail_text("ERROR: No.of KnotMultiplicities is equal to 0", "");
            return Ok(());
        }
        let i: i32 = Default::default();
        for i in 1..=nb_mult - 1 {
            sum_mult = sum_mult + ent.knot_multiplicities_value(i)?;
        }
        let sum_non_p: i32 = nb_cpl + dg_bsc + 1;
        let mult1: i32 = ent.knot_multiplicities_value(1)?;
        let mult_n: i32 = ent.knot_multiplicities_value(nb_mult)?;
        if (sum_mult + mult_n) == sum_non_p {
        } else {
            if (sum_mult == nb_cpl) && (mult1 == mult_n) {
            } else {
                ach.add_fail_text("ERROR: wrong number of Knot Multiplicities", "");
            }
        }
        for i in 2..=nb_kno {
            let dist_kn: f64 = ent.knots_value(i - 1)? - ent.knots_value(i)?;
            if dist_kn.abs() <= f64::EPSILON {
                ach.add_warning_text("WARNING: Curve contains identical KnotsValues", "");
            } else {
                if dist_kn > f64::EPSILON {
                    ach.add_fail_text("ERROR: Curve contains descending KnotsValues", "");
                }
            }
        }
        Ok(())
    }
}
