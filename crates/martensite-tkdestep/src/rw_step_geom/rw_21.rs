#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepGeomRwSurfacePatch, "RwStepGeomRwSurfacePatch");
impl RwStepGeomRwSurfacePatch {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomSurfacePatch>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "surface_patch")? {
            return Ok(());
        }
        let mut a_parent_surface: Handle<StepGeomBoundedSurface> = Default::default();
        data.read_entity(
            num,
            1,
            "parent_surface",
            ach,
            StepGeomBoundedSurface::static_type(),
            &mut a_parent_surface,
        )?;
        let mut a_u_transition: StepGeomTransitionCode = StepGeomTransitionCode::TcDiscontinuous;
        if data.param_type(num, 2)? == InterfaceParamType::ParamEnum {
            let text: Vec<u8> = data.param_c_value(num, 2)?;
            if !RwStepGeomRwTransitionCode::convert_to_enum(&text, &mut a_u_transition) {
                ach.add_fail_text("Enumeration transition_code has not an allowed value", "");
            }
        } else {
            ach.add_fail_text("Parameter #2 (u_transition) is not an enumeration", "");
        }
        let mut a_v_transition: StepGeomTransitionCode = StepGeomTransitionCode::TcDiscontinuous;
        if data.param_type(num, 3)? == InterfaceParamType::ParamEnum {
            let text: Vec<u8> = data.param_c_value(num, 3)?;
            if !RwStepGeomRwTransitionCode::convert_to_enum(&text, &mut a_v_transition) {
                ach.add_fail_text("Enumeration transition_code has not an allowed value", "");
            }
        } else {
            ach.add_fail_text("Parameter #3 (v_transition) is not an enumeration", "");
        }
        let mut a_u_sense: bool = Default::default();
        data.read_boolean(num, 4, "u_sense", ach, &mut a_u_sense)?;
        let mut a_v_sense: bool = Default::default();
        data.read_boolean(num, 5, "v_sense", ach, &mut a_v_sense)?;
        ent.init(
            &a_parent_surface,
            a_u_transition,
            a_v_transition,
            a_u_sense,
            a_v_sense,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomSurfacePatch>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent_surface())?;
        sw.send_enum(RwStepGeomRwTransitionCode::convert_to_string(
            ent.u_transition(),
        ))?;
        sw.send_enum(RwStepGeomRwTransitionCode::convert_to_string(
            ent.v_transition(),
        ))?;
        sw.send_boolean(ent.u_sense())?;
        sw.send_boolean(ent.v_sense())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomSurfacePatch>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.parent_surface().cast());
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwSurfaceReplica, "RwStepGeomRwSurfaceReplica");
impl RwStepGeomRwSurfaceReplica {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomSurfaceReplica>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "surface_replica")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_parent_surface: Handle<StepGeomSurface> = Default::default();
        data.read_entity(
            num,
            2,
            "parent_surface",
            ach,
            StepGeomSurface::static_type(),
            &mut a_parent_surface,
        )?;
        let mut a_transformation: Handle<StepGeomCartesianTransformationOperator3d> =
            Default::default();
        data.read_entity(
            num,
            3,
            "transformation",
            ach,
            StepGeomCartesianTransformationOperator3d::static_type(),
            &mut a_transformation,
        )?;
        ent.init(&a_name, &a_parent_surface, &a_transformation);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomSurfaceReplica>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.parent_surface())?;
        sw.send_entity(&ent.transformation())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomSurfaceReplica>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.parent_surface().cast());
        iter.get_one_item(&ent.transformation().cast());
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwSweptSurface, "RwStepGeomRwSweptSurface");
impl RwStepGeomRwSweptSurface {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomSweptSurface>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "swept_surface")? {
            return Ok(());
        }
        rw_block_0134!(a_name, a_swept_curve, ach, data, num);
        ent.init(&a_name, &a_swept_curve);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomSweptSurface>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.swept_curve())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomSweptSurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.swept_curve().cast());
        Ok(())
    }
}

step_rw_class!(RwStepGeomRwToroidalSurface, "RwStepGeomRwToroidalSurface");
impl RwStepGeomRwToroidalSurface {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomToroidalSurface>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "toroidal_surface")? {
            return Ok(());
        }
        rw_block_0059!(
            a_major_radius,
            a_minor_radius,
            a_name,
            a_position,
            ach,
            data,
            num
        );
        ent.init(&a_name, &a_position, a_major_radius, a_minor_radius);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomToroidalSurface>,
    ) -> Result<(), Failure> {
        rw_block_0167!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomToroidalSurface>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.position().cast());
        Ok(())
    }
    /// `Check`.
    pub fn check(
        &self,
        ent: &Handle<StepGeomToroidalSurface>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if ent.major_radius() < 0.0 {
            ach.add_fail_text("ERROR: ToroidalSurface: MajorRadius < 0.0", "");
        }
        if ent.minor_radius() < 0.0 {
            ach.add_fail_text("ERROR: ToroidalSurface: MinorRadius < 0.0", "");
        }
        if ent.major_radius() < ent.minor_radius() {
            ach.add_warning_text("ToroidalSurface: MajorRadius smaller than MinorRadius", "");
        }
        Ok(())
    }
}
