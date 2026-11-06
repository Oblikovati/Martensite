#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepShapeRwRevolvedAreaSolid,
    "RwStepShapeRwRevolvedAreaSolid"
);
impl RwStepShapeRwRevolvedAreaSolid {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeRevolvedAreaSolid>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "revolved_area_solid")? {
            return Ok(());
        }
        rw_block_0137!(a_name, a_swept_area, ach, data, num);
        let mut a_axis: Handle<StepGeomAxis1Placement> = Default::default();
        rw_block_0184!(a_angle, a_axis, ach, data, num);
        ent.init(&a_name, &a_swept_area, &a_axis, a_angle);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeRevolvedAreaSolid>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.swept_area())?;
        sw.send_entity(&ent.axis())?;
        sw.send_real(ent.angle())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeRevolvedAreaSolid>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.swept_area().cast());
        iter.get_one_item(&ent.axis().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwRevolvedFaceSolid,
    "RwStepShapeRwRevolvedFaceSolid"
);
impl RwStepShapeRwRevolvedFaceSolid {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeRevolvedFaceSolid>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "revolved_face_solid")? {
            return Ok(());
        }
        rw_block_0138!(a_name, a_swept_face, ach, data, num);
        let mut a_axis: Handle<StepGeomAxis1Placement> = Default::default();
        rw_block_0184!(a_angle, a_axis, ach, data, num);
        ent.init_axis1_placement(&a_name, &a_swept_face, &a_axis, a_angle);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeRevolvedFaceSolid>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.swept_face())?;
        sw.send_entity(&ent.axis())?;
        sw.send_real(ent.angle())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeRevolvedFaceSolid>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.swept_face().cast());
        iter.get_one_item(&ent.axis().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwRightAngularWedge,
    "RwStepShapeRwRightAngularWedge"
);
impl RwStepShapeRwRightAngularWedge {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeRightAngularWedge>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 6, ach, "right_angular_wedge")? {
            return Ok(());
        }
        rw_block_0036!(a_name, a_position, a_x, a_y, a_z, ach, data, num);
        let mut a_ltx: f64 = Default::default();
        data.read_real(num, 6, "ltx", ach, &mut a_ltx)?;
        ent.init(&a_name, &a_position, a_x, a_y, a_z, a_ltx);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeRightAngularWedge>,
    ) -> Result<(), Failure> {
        rw_block_0119!(ent, sw);
        sw.send_real(ent.ltx())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeRightAngularWedge>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.position().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwRightCircularCone,
    "RwStepShapeRwRightCircularCone"
);
impl RwStepShapeRwRightCircularCone {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeRightCircularCone>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "right_circular_cone")? {
            return Ok(());
        }
        rw_block_0060!(a_height, a_name, a_position, a_radius, ach, data, num);
        let mut a_semi_angle: f64 = Default::default();
        data.read_real(num, 5, "semi_angle", ach, &mut a_semi_angle)?;
        ent.init(&a_name, &a_position, a_height, a_radius, a_semi_angle);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeRightCircularCone>,
    ) -> Result<(), Failure> {
        rw_block_0168!(ent, sw);
        sw.send_real(ent.semi_angle())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeRightCircularCone>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.position().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwRightCircularCylinder,
    "RwStepShapeRwRightCircularCylinder"
);
impl RwStepShapeRwRightCircularCylinder {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeRightCircularCylinder>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "right_circular_cylinder")? {
            return Ok(());
        }
        rw_block_0060!(a_height, a_name, a_position, a_radius, ach, data, num);
        ent.init(&a_name, &a_position, a_height, a_radius);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeRightCircularCylinder>,
    ) -> Result<(), Failure> {
        rw_block_0168!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeRightCircularCylinder>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.position().cast());
        Ok(())
    }
}
