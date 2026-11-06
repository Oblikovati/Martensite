#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepGeomRwAxis2Placement3d, "RwStepGeomRwAxis2Placement3d");
impl RwStepGeomRwAxis2Placement3d {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomAxis2Placement3d>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "axis2_placement_3d")? {
            return Ok(());
        }
        rw_block_0105!(a_axis, a_location, a_name, ach, data, num);
        let mut has_aaxis: bool = false;
        if data.is_param_defined(num, 3)? {
            has_aaxis = data.read_entity(
                num,
                3,
                "axis",
                ach,
                StepGeomDirection::static_type(),
                &mut a_axis,
            )?;
        } else {
            a_axis = Default::default();
        }
        let mut a_ref_direction: Handle<StepGeomDirection> = Default::default();
        let mut has_aref_direction: bool = false;
        if data.is_param_defined(num, 4)? {
            has_aref_direction = data.read_entity(
                num,
                4,
                "ref_direction",
                ach,
                StepGeomDirection::static_type(),
                &mut a_ref_direction,
            )?;
        } else {
            a_ref_direction = Default::default();
        }
        ent.init(
            &a_name,
            &a_location,
            has_aaxis,
            &a_axis,
            has_aref_direction,
            &a_ref_direction,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomAxis2Placement3d>,
    ) -> Result<(), Failure> {
        rw_block_0154!(ent, has_aaxis, sw);
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
        ent: &Handle<StepGeomAxis2Placement3d>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.location().cast());
        if ent.has_axis() {
            iter.get_one_item(&ent.axis().cast());
        }
        if ent.has_ref_direction() {
            iter.get_one_item(&ent.ref_direction().cast());
        }
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwBSplineCurve, "RwStepGeomRwBSplineCurve");
impl RwStepGeomRwBSplineCurve {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomBSplineCurve>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 6, ach, "b_spline_curve")? {
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
        ent: &Handle<StepGeomBSplineCurve>,
    ) -> Result<(), Failure> {
        rw_block_0061!(ent, i3, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomBSplineCurve>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_control_points_list();
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.control_points_list_value(is1)?.cast());
        }
        Ok(())
    }
}
