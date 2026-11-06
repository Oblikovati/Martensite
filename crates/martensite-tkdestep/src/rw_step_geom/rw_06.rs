#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepGeomRwBSplineSurfaceWithKnotsAndRationalBSplineSurface,
    "RwStepGeomRwBSplineSurfaceWithKnotsAndRationalBSplineSurface"
);
impl RwStepGeomRwBSplineSurfaceWithKnotsAndRationalBSplineSurface {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface>,
    ) -> Result<(), Failure> {
        let mut num: i32 = 0;
        data.named_for_complex_short("BOUNDED_SURFACE", "BNDSRF", num0, &mut num, ach);
        data.named_for_complex_short("B_SPLINE_SURFACE", "BSPSR", num0, &mut num, ach);
        rw_block_0058!(
            a_control_points_list,
            a_u_degree,
            a_v_degree,
            ach,
            anent3,
            data,
            nsub3,
            num
        );
        if data.read_sub_list(num, 3, "control_points_list", ach, &mut nsub3, false, 0, 0)? {
            let nbi3: i32 = data.nb_params(nsub3)?;
            let nbj3: i32 = data.nb_params(data.param_number(nsub3, 1)?)?;
            a_control_points_list =
                step_new_array2::<Handle<StepGeomCartesianPoint>>(1, nbi3, 1, nbj3)?;
            for i3 in 1..=nbi3 {
                let mut nsi3temp: i32 = Default::default();
                if data.read_sub_list(
                    nsub3,
                    i3,
                    "sub-part(control_points_list)",
                    ach,
                    &mut nsi3temp,
                    false,
                    0,
                    0,
                )? {
                    let nsi3: i32 = data.param_number(nsub3, i3)?;
                    for j3 in 1..=nbj3 {
                        if data.read_entity(
                            nsi3,
                            j3,
                            "cartesian_point",
                            ach,
                            StepGeomCartesianPoint::static_type(),
                            &mut anent3,
                        )? {
                            step_array2_set(
                                &a_control_points_list,
                                i3,
                                j3,
                                anent3.clone(),
                                "ReadStep",
                            )?;
                        }
                    }
                }
            }
        }
        rw_block_0054!(
            a_self_intersect,
            a_surface_form,
            a_u_closed,
            a_v_closed,
            ach,
            data,
            num,
            text
        );
        data.named_for_complex_short("B_SPLINE_SURFACE_WITH_KNOTS", "BSSWK", num0, &mut num, ach);
        if !data.check_nb_params(num, 5, ach, "b_spline_surface_with_knots")? {
            return Ok(());
        }
        let mut a_u_multiplicities: Handle<HArray1<i32>> = Default::default();
        let mut a_u_multiplicities_item: i32 = Default::default();
        let mut nsub8: i32 = Default::default();
        if data.read_sub_list(num, 1, "u_multiplicities", ach, &mut nsub8, false, 0, 0)? {
            let nb8: i32 = data.nb_params(nsub8)?;
            a_u_multiplicities = step_new_array1::<i32>(1, nb8)?;
            for i8 in 1..=nb8 {
                if data.read_integer(
                    nsub8,
                    i8,
                    "u_multiplicities",
                    ach,
                    &mut a_u_multiplicities_item,
                )? {
                    step_array1_set(&a_u_multiplicities, i8, a_u_multiplicities_item, "ReadStep")?;
                }
            }
        }
        let mut a_v_multiplicities: Handle<HArray1<i32>> = Default::default();
        let mut a_v_multiplicities_item: i32 = Default::default();
        let mut nsub9: i32 = Default::default();
        if data.read_sub_list(num, 2, "v_multiplicities", ach, &mut nsub9, false, 0, 0)? {
            let nb9: i32 = data.nb_params(nsub9)?;
            a_v_multiplicities = step_new_array1::<i32>(1, nb9)?;
            for i9 in 1..=nb9 {
                if data.read_integer(
                    nsub9,
                    i9,
                    "v_multiplicities",
                    ach,
                    &mut a_v_multiplicities_item,
                )? {
                    step_array1_set(&a_v_multiplicities, i9, a_v_multiplicities_item, "ReadStep")?;
                }
            }
        }
        let mut a_u_knots: Handle<HArray1<f64>> = Default::default();
        let mut a_u_knots_item: f64 = Default::default();
        let mut nsub10: i32 = Default::default();
        if data.read_sub_list(num, 3, "u_knots", ach, &mut nsub10, false, 0, 0)? {
            let nb10: i32 = data.nb_params(nsub10)?;
            a_u_knots = step_new_array1::<f64>(1, nb10)?;
            for i10 in 1..=nb10 {
                if data.read_real(nsub10, i10, "u_knots", ach, &mut a_u_knots_item)? {
                    step_array1_set(&a_u_knots, i10, a_u_knots_item, "ReadStep")?;
                }
            }
        }
        let mut a_v_knots: Handle<HArray1<f64>> = Default::default();
        let mut a_v_knots_item: f64 = Default::default();
        let mut nsub11: i32 = Default::default();
        if data.read_sub_list(num, 4, "v_knots", ach, &mut nsub11, false, 0, 0)? {
            let nb11: i32 = data.nb_params(nsub11)?;
            a_v_knots = step_new_array1::<f64>(1, nb11)?;
            for i11 in 1..=nb11 {
                if data.read_real(nsub11, i11, "v_knots", ach, &mut a_v_knots_item)? {
                    step_array1_set(&a_v_knots, i11, a_v_knots_item, "ReadStep")?;
                }
            }
        }
        let mut a_knot_spec: StepGeomKnotType = StepGeomKnotType::KtUniformKnots;
        if data.param_type(num, 5)? == InterfaceParamType::ParamEnum {
            let text: Vec<u8> = data.param_c_value(num, 5)?;
            if !RwStepGeomRwKnotType::convert_to_enum(&text, &mut a_knot_spec) {
                ach.add_fail_text("Enumeration knot_type has not an allowed value", "");
            }
        } else {
            ach.add_fail_text("Parameter #5 (knot_spec) is not an enumeration", "");
        }
        data.named_for_complex_short(
            "GEOMETRIC_REPRESENTATION_ITEM",
            "GMRPIT",
            num0,
            &mut num,
            ach,
        );
        data.named_for_complex_short("RATIONAL_B_SPLINE_SURFACE", "RBSS", num0, &mut num, ach);
        rw_block_0181!(a_weights_data, a_weights_data_item, ach, data, num);
        let mut nsub13: i32 = Default::default();
        if data.read_sub_list(num, 1, "weights_data", ach, &mut nsub13, false, 0, 0)? {
            let nbi13: i32 = data.nb_params(nsub13)?;
            let nbj13: i32 = data.nb_params(data.param_number(nsub13, 1)?)?;
            a_weights_data = step_new_array2::<f64>(1, nbi13, 1, nbj13)?;
            for i13 in 1..=nbi13 {
                let mut nsi13temp: i32 = Default::default();
                if data.read_sub_list(
                    nsub13,
                    i13,
                    "sub-part(weights_data)",
                    ach,
                    &mut nsi13temp,
                    false,
                    0,
                    0,
                )? {
                    let nsi13: i32 = data.param_number(nsub13, i13)?;
                    for j13 in 1..=nbj13 {
                        if data.read_real(
                            nsi13,
                            j13,
                            "weights_data",
                            ach,
                            &mut a_weights_data_item,
                        )? {
                            step_array2_set(
                                &a_weights_data,
                                i13,
                                j13,
                                a_weights_data_item,
                                "ReadStep",
                            )?;
                        }
                    }
                }
            }
        }
        data.named_for_complex_short("REPRESENTATION_ITEM", "RPRITM", num0, &mut num, ach);
        let mut a_name: Handle<HAsciiString> = Default::default();
        if !data.check_nb_params(num, 1, ach, "representation_item")? {
            a_name = Handle::new(HAsciiString::new(""));
        } else {
            data.read_string(num, 1, "name", ach, &mut a_name)?;
        }
        data.named_for_complex_short("SURFACE", "SRFC", num0, &mut num, ach);
        ent.init_fields(
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
            &a_weights_data,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface>,
    ) -> Result<(), Failure> {
        rw_block_0028!(ent, i3, j3, sw);
        step_start_entity(sw, "B_SPLINE_SURFACE_WITH_KNOTS")?;
        sw.open_sub()?;
        for i8 in 1..=ent.nb_u_multiplicities()? {
            sw.send_integer(ent.u_multiplicities_value(i8)?)?;
        }
        sw.close_sub()?;
        sw.open_sub()?;
        for i9 in 1..=ent.nb_v_multiplicities()? {
            sw.send_integer(ent.v_multiplicities_value(i9)?)?;
        }
        sw.close_sub()?;
        sw.open_sub()?;
        for i10 in 1..=ent.nb_u_knots()? {
            sw.send_real(ent.u_knots_value(i10)?)?;
        }
        sw.close_sub()?;
        sw.open_sub()?;
        for i11 in 1..=ent.nb_v_knots()? {
            sw.send_real(ent.v_knots_value(i11)?)?;
        }
        sw.close_sub()?;
        sw.send_enum(RwStepGeomRwKnotType::convert_to_string(ent.knot_spec()?))?;
        step_start_entity(sw, "GEOMETRIC_REPRESENTATION_ITEM")?;
        step_start_entity(sw, "RATIONAL_B_SPLINE_SURFACE")?;
        sw.open_sub()?;
        for i13 in 1..=ent.nb_weights_data_i()? {
            sw.new_line(false)?;
            sw.open_sub()?;
            for j13 in 1..=ent.nb_weights_data_j()? {
                sw.send_real(ent.weights_data_value(i13, j13)?)?;
                sw.join_last(false);
            }
            sw.close_sub()?;
        }
        sw.close_sub()?;
        step_start_entity(sw, "REPRESENTATION_ITEM")?;
        sw.send_entity(&ent.name())?;
        step_start_entity(sw, "SURFACE")?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0177!(ent, is1, iter, js1, nbi_elem1, nbj_elem1);
        Ok(())
    }
    /// `Check`.
    pub fn check(
        &self,
        ent: &Handle<StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface>,
        a_shto: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let a_rational_bss: Handle<StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface> =
            ent.clone();
        let a_bsswk: Handle<StepGeomBSplineSurfaceWithKnots> =
            step_live(&a_rational_bss, "Check")?.b_spline_surface_with_knots();
        let t1: RwStepGeomRwBSplineSurfaceWithKnots = Default::default();
        t1.check(&a_bsswk, a_shto, ach)?;
        let a_rbss: Handle<StepGeomRationalBSplineSurface> =
            step_live(&a_rational_bss, "Check")?.rational_b_spline_surface();
        let t2: RwStepGeomRwRationalBSplineSurface = Default::default();
        t2.check(&a_rbss, a_shto, ach)?;
        Ok(())
    }
}
