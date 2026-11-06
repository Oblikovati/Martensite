#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepFEARwAlignedCurve3dElementCoordinateSystem,
    "RwStepFEARwAlignedCurve3dElementCoordinateSystem"
);
impl RwStepFEARwAlignedCurve3dElementCoordinateSystem {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAAlignedCurve3dElementCoordinateSystem>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "aligned_curve3d_element_coordinate_system")? {
            return Ok(());
        }
        rw_block_b_0108!(
            a_coordinate_system,
            a_representation_item_name,
            ach,
            data,
            ent,
            num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEAAlignedCurve3dElementCoordinateSystem>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.name())?;
        sw.send_entity(&ent.coordinate_system())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEAAlignedCurve3dElementCoordinateSystem>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.coordinate_system().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepFEARwAlignedSurface3dElementCoordinateSystem,
    "RwStepFEARwAlignedSurface3dElementCoordinateSystem"
);
impl RwStepFEARwAlignedSurface3dElementCoordinateSystem {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAAlignedSurface3dElementCoordinateSystem>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "aligned_surface3d_element_coordinate_system")? {
            return Ok(());
        }
        rw_block_b_0108!(
            a_coordinate_system,
            a_representation_item_name,
            ach,
            data,
            ent,
            num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEAAlignedSurface3dElementCoordinateSystem>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.name())?;
        sw.send_entity(&ent.coordinate_system())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEAAlignedSurface3dElementCoordinateSystem>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.coordinate_system().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepFEARwArbitraryVolume3dElementCoordinateSystem,
    "RwStepFEARwArbitraryVolume3dElementCoordinateSystem"
);
impl RwStepFEARwArbitraryVolume3dElementCoordinateSystem {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAArbitraryVolume3dElementCoordinateSystem>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "arbitrary_volume3d_element_coordinate_system")? {
            return Ok(());
        }
        rw_block_b_0108!(
            a_coordinate_system,
            a_representation_item_name,
            ach,
            data,
            ent,
            num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEAArbitraryVolume3dElementCoordinateSystem>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.name())?;
        sw.send_entity(&ent.coordinate_system())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEAArbitraryVolume3dElementCoordinateSystem>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.coordinate_system().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepFEARwConstantSurface3dElementCoordinateSystem,
    "RwStepFEARwConstantSurface3dElementCoordinateSystem"
);
impl RwStepFEARwConstantSurface3dElementCoordinateSystem {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAConstantSurface3dElementCoordinateSystem>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "constant_surface3d_element_coordinate_system")? {
            return Ok(());
        }
        rw_block_b_0077!(
            a_angle,
            a_axis,
            a_representation_item_name,
            ach,
            data,
            ent,
            num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEAConstantSurface3dElementCoordinateSystem>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.name())?;
        sw.send_integer(ent.axis())?;
        sw.send_real(ent.angle())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepFEAConstantSurface3dElementCoordinateSystem>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}
