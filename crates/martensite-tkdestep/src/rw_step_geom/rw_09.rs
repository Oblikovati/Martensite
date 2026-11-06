#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepGeomRwCompositeCurve, "RwStepGeomRwCompositeCurve");
impl RwStepGeomRwCompositeCurve {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomCompositeCurve>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "composite_curve")? {
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
        ent: &Handle<StepGeomCompositeCurve>,
    ) -> Result<(), Failure> {
        rw_block_0117!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomCompositeCurve>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_segments();
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.segments_value(is1)?.cast());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepGeomRwCompositeCurveOnSurface,
    "RwStepGeomRwCompositeCurveOnSurface"
);
impl RwStepGeomRwCompositeCurveOnSurface {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomCompositeCurveOnSurface>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "composite_curve_on_surface")? {
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
        ent: &Handle<StepGeomCompositeCurveOnSurface>,
    ) -> Result<(), Failure> {
        rw_block_0117!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomCompositeCurveOnSurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_segments();
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.segments_value(is1)?.cast());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepGeomRwCompositeCurveSegment,
    "RwStepGeomRwCompositeCurveSegment"
);
impl RwStepGeomRwCompositeCurveSegment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomCompositeCurveSegment>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "composite_curve_segment")? {
            return Ok(());
        }
        rw_block_0079!(
            a_parent_curve,
            a_same_sense,
            a_transition,
            ach,
            data,
            num,
            text
        );
        ent.init(a_transition, a_same_sense, &a_parent_curve);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomCompositeCurveSegment>,
    ) -> Result<(), Failure> {
        sw.send_enum(RwStepGeomRwTransitionCode::convert_to_string(
            ent.transition(),
        ))?;
        sw.send_boolean(ent.same_sense())?;
        sw.send_entity(&ent.parent_curve())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomCompositeCurveSegment>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.parent_curve().cast());
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwConic, "RwStepGeomRwConic");
impl RwStepGeomRwConic {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomConic>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "conic")? {
            return Ok(());
        }
        rw_block_0153!(a_name, a_position, ach, data, num);
        ent.init(&a_name, &a_position);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomConic>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.position().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomConic>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.position().value());
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwConicalSurface, "RwStepGeomRwConicalSurface");
impl RwStepGeomRwConicalSurface {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomConicalSurface>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "conical_surface")? {
            return Ok(());
        }
        rw_block_0085!(a_name, a_position, a_radius, ach, data, num);
        let mut a_semi_angle: f64 = Default::default();
        data.read_real(num, 4, "semi_angle", ach, &mut a_semi_angle)?;
        ent.init(&a_name, &a_position, a_radius, a_semi_angle);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomConicalSurface>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.position())?;
        sw.send_real(ent.radius())?;
        sw.send_real(ent.semi_angle())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomConicalSurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.position().cast());
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwCurve, "RwStepGeomRwCurve");
impl RwStepGeomRwCurve {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomCurve>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "curve")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        ent.init(&a_name);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomCurve>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        Ok(())
    }
}
