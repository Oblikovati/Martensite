#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepGeomRwBSplineCurveWithKnotsAndRationalBSplineCurve,
    "RwStepGeomRwBSplineCurveWithKnotsAndRationalBSplineCurve"
);
impl RwStepGeomRwBSplineCurveWithKnotsAndRationalBSplineCurve {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve>,
    ) -> Result<(), Failure> {
        let mut num: i32 = 0;
        data.named_for_complex_short("BOUNDED_CURVE", "BNDCRV", num0, &mut num, ach);
        data.named_for_complex_short("B_SPLINE_CURVE", "BSPCR", num0, &mut num, ach);
        rw_block_0016!(
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
        data.named_for_complex_short("B_SPLINE_CURVE_WITH_KNOTS", "BSCWK", num0, &mut num, ach);
        if !data.check_nb_params(num, 3, ach, "b_spline_curve_with_knots")? {
            return Ok(());
        }
        let mut a_knot_multiplicities: Handle<HArray1<i32>> = Default::default();
        let mut a_knot_multiplicities_item: i32 = Default::default();
        let mut nsub6: i32 = Default::default();
        if data.read_sub_list(num, 1, "knot_multiplicities", ach, &mut nsub6, false, 0, 0)? {
            let nb6: i32 = data.nb_params(nsub6)?;
            a_knot_multiplicities = step_new_array1::<i32>(1, nb6)?;
            for i6 in 1..=nb6 {
                if data.read_integer(
                    nsub6,
                    i6,
                    "knot_multiplicities",
                    ach,
                    &mut a_knot_multiplicities_item,
                )? {
                    step_array1_set(
                        &a_knot_multiplicities,
                        i6,
                        a_knot_multiplicities_item,
                        "ReadStep",
                    )?;
                }
            }
        }
        let mut a_knots: Handle<HArray1<f64>> = Default::default();
        let mut a_knots_item: f64 = Default::default();
        let mut nsub7: i32 = Default::default();
        if data.read_sub_list(num, 2, "knots", ach, &mut nsub7, false, 0, 0)? {
            let nb7: i32 = data.nb_params(nsub7)?;
            a_knots = step_new_array1::<f64>(1, nb7)?;
            for i7 in 1..=nb7 {
                if data.read_real(nsub7, i7, "knots", ach, &mut a_knots_item)? {
                    step_array1_set(&a_knots, i7, a_knots_item, "ReadStep")?;
                }
            }
        }
        let mut a_knot_spec: StepGeomKnotType = StepGeomKnotType::KtUniformKnots;
        if data.param_type(num, 3)? == InterfaceParamType::ParamEnum {
            let text: Vec<u8> = data.param_c_value(num, 3)?;
            if !RwStepGeomRwKnotType::convert_to_enum(&text, &mut a_knot_spec) {
                ach.add_fail_text("Enumeration knot_type has not an allowed value", "");
            }
        } else {
            ach.add_fail_text("Parameter #3 (knot_spec) is not an enumeration", "");
        }
        data.named_for_complex("CURVE", num0, &mut num, ach);
        data.named_for_complex_short(
            "GEOMETRIC_REPRESENTATION_ITEM",
            "GMRPIT",
            num0,
            &mut num,
            ach,
        );
        data.named_for_complex_short("RATIONAL_B_SPLINE_CURVE", "RBSC", num0, &mut num, ach);
        rw_block_0180!(a_weights_data, a_weights_data_item, ach, data, num);
        let mut nsub9: i32 = Default::default();
        if data.read_sub_list(num, 1, "weights_data", ach, &mut nsub9, false, 0, 0)? {
            let nb9: i32 = data.nb_params(nsub9)?;
            a_weights_data = step_new_array1::<f64>(1, nb9)?;
            for i9 in 1..=nb9 {
                if data.read_real(nsub9, i9, "weights_data", ach, &mut a_weights_data_item)? {
                    step_array1_set(&a_weights_data, i9, a_weights_data_item, "ReadStep")?;
                }
            }
        }
        rw_block_0131!(a_name, ach, data, num, num0);
        ent.init_fields(
            &a_name,
            a_degree,
            &a_control_points_list,
            a_curve_form,
            a_closed_curve,
            a_self_intersect,
            &a_knot_multiplicities,
            &a_knots,
            a_knot_spec,
            &a_weights_data,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve>,
    ) -> Result<(), Failure> {
        rw_block_0050!(ent, i2, sw);
        step_start_entity(sw, "B_SPLINE_CURVE_WITH_KNOTS")?;
        sw.open_sub()?;
        for i6 in 1..=ent.nb_knot_multiplicities()? {
            sw.send_integer(ent.knot_multiplicities_value(i6)?)?;
        }
        sw.close_sub()?;
        sw.open_sub()?;
        for i7 in 1..=ent.nb_knots()? {
            sw.send_real(ent.knots_value(i7)?)?;
        }
        sw.close_sub()?;
        sw.send_enum(RwStepGeomRwKnotType::convert_to_string(ent.knot_spec()?))?;
        step_start_entity(sw, "CURVE")?;
        step_start_entity(sw, "GEOMETRIC_REPRESENTATION_ITEM")?;
        step_start_entity(sw, "RATIONAL_B_SPLINE_CURVE")?;
        sw.open_sub()?;
        for i9 in 1..=ent.nb_weights_data()? {
            sw.send_real(ent.weights_data_value(i9)?)?;
        }
        sw.close_sub()?;
        step_start_entity(sw, "REPRESENTATION_ITEM")?;
        sw.send_entity(&ent.name())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve>,
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
        ent: &Handle<StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve>,
        a_shto: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let a_rational_bsc: Handle<StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve> =
            ent.clone();
        let a_bscwk: Handle<StepGeomBSplineCurveWithKnots> =
            step_live(&a_rational_bsc, "Check")?.b_spline_curve_with_knots();
        let t1: RwStepGeomRwBSplineCurveWithKnots = Default::default();
        t1.check(&a_bscwk, a_shto, ach)?;
        let a_rbsc: Handle<StepGeomRationalBSplineCurve> =
            step_live(&a_rational_bsc, "Check")?.rational_b_spline_curve();
        let t2: RwStepGeomRwRationalBSplineCurve = Default::default();
        t2.check(&a_rbsc, a_shto, ach)?;
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwBSplineSurface, "RwStepGeomRwBSplineSurface");
impl RwStepGeomRwBSplineSurface {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomBSplineSurface>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 8, ach, "b_spline_surface")? {
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
        ent: &Handle<StepGeomBSplineSurface>,
    ) -> Result<(), Failure> {
        rw_block_0035!(ent, i4, j4, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomBSplineSurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0177!(ent, is1, iter, js1, nbi_elem1, nbj_elem1);
        Ok(())
    }
}
