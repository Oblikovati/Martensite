#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepGeomRwTrimmedCurve, "RwStepGeomRwTrimmedCurve");
impl RwStepGeomRwTrimmedCurve {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomTrimmedCurve>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 6, ach, "trimmed_curve")? {
            return Ok(());
        }
        rw_block_0132!(a_basis_curve, a_name, ach, data, num);
        let a_cartesian_point: Handle<StepGeomCartesianPoint> = Default::default();
        let mut a_trim1: Handle<HArray1<StepGeomTrimmingSelect>> = Default::default();
        let mut nsub3: i32 = Default::default();
        if data.read_sub_list(num, 3, "trim_1", ach, &mut nsub3, false, 0, 0)? {
            let nb3: i32 = data.nb_params(nsub3)?;
            a_trim1 = step_new_array1::<StepGeomTrimmingSelect>(1, nb3)?;
            for i3 in 1..=nb3 {
                let mut a_trim1_item: StepGeomTrimmingSelect = Default::default();
                if data.read_entity_select(nsub3, i3, "trim_1", ach, &mut a_trim1_item)? {
                    step_array1_set(&a_trim1, i3, a_trim1_item.clone(), "ReadStep")?;
                }
            }
        }
        let mut a_trim2: Handle<HArray1<StepGeomTrimmingSelect>> = Default::default();
        let mut nsub4: i32 = Default::default();
        if data.read_sub_list(num, 4, "trim_2", ach, &mut nsub4, false, 0, 0)? {
            let nb4: i32 = data.nb_params(nsub4)?;
            a_trim2 = step_new_array1::<StepGeomTrimmingSelect>(1, nb4)?;
            for i4 in 1..=nb4 {
                let mut a_trim2_item: StepGeomTrimmingSelect = Default::default();
                if data.read_entity_select(nsub4, i4, "trim_2", ach, &mut a_trim2_item)? {
                    step_array1_set(&a_trim2, i4, a_trim2_item.clone(), "ReadStep")?;
                }
            }
        }
        let mut a_sense_agreement: bool = Default::default();
        data.read_boolean(num, 5, "sense_agreement", ach, &mut a_sense_agreement)?;
        let mut a_master_representation: StepGeomTrimmingPreference =
            StepGeomTrimmingPreference::TpCartesian;
        if data.param_type(num, 6)? == InterfaceParamType::ParamEnum {
            let text: Vec<u8> = data.param_c_value(num, 6)?;
            if !RwStepGeomRwTrimmingPreference::convert_to_enum(&text, &mut a_master_representation)
            {
                ach.add_fail_text(
                    "Enumeration trimming_preference has not an allowed value",
                    "",
                );
            }
        } else {
            ach.add_fail_text(
                "Parameter #6 (master_representation) is not an enumeration",
                "",
            );
        }
        ent.init(
            &a_name,
            &a_basis_curve,
            &a_trim1,
            &a_trim2,
            a_sense_agreement,
            a_master_representation,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomTrimmedCurve>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.basis_curve())?;
        sw.open_sub()?;
        for i2 in 1..=ent.nb_trim1()? {
            sw.send_entity(&ent.trim1_value(i2)?.value())?;
        }
        sw.close_sub()?;
        sw.open_sub()?;
        for i3 in 1..=ent.nb_trim2()? {
            sw.send_entity(&ent.trim2_value(i3)?.value())?;
        }
        sw.close_sub()?;
        sw.send_boolean(ent.sense_agreement())?;
        sw.send_enum(RwStepGeomRwTrimmingPreference::convert_to_string(
            ent.master_representation(),
        ))?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomTrimmedCurve>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.basis_curve().cast());
        let nb_elem2: i32 = ent.nb_trim1()?;
        for is2 in 1..=nb_elem2 {
            if ent.trim1_value(is2)?.case_number() > 0 {
                iter.get_one_item(&ent.trim1_value(is2)?.value());
            }
        }
        let nb_elem3: i32 = ent.nb_trim2()?;
        for is3 in 1..=nb_elem3 {
            if ent.trim2_value(is3)?.case_number() > 0 {
                iter.get_one_item(&ent.trim2_value(is3)?.value());
            }
        }
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwUniformCurve, "RwStepGeomRwUniformCurve");
impl RwStepGeomRwUniformCurve {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomUniformCurve>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 6, ach, "uniform_curve")? {
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
        ent: &Handle<StepGeomUniformCurve>,
    ) -> Result<(), Failure> {
        rw_block_0061!(ent, i3, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomUniformCurve>,
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
    RwStepGeomRwUniformCurveAndRationalBSplineCurve,
    "RwStepGeomRwUniformCurveAndRationalBSplineCurve"
);
impl RwStepGeomRwUniformCurveAndRationalBSplineCurve {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomUniformCurveAndRationalBSplineCurve>,
    ) -> Result<(), Failure> {
        let mut num: i32 = num0;
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
        num = data.next_for_complex(num);
        if !data.check_nb_params(num, 0, ach, "uniform_curve")? {
            return Ok(());
        }
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
        ent: &Handle<StepGeomUniformCurveAndRationalBSplineCurve>,
    ) -> Result<(), Failure> {
        rw_block_0011!(ent, i2, i6, sw);
        step_start_entity(sw, "UNIFORM_CURVE")?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomUniformCurveAndRationalBSplineCurve>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_control_points_list();
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.control_points_list_value(is1)?.cast());
        }
        Ok(())
    }
}
