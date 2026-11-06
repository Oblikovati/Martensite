#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepKinematicsRwRollingSurfacePair,
    "RwStepKinematicsRwRollingSurfacePair"
);
impl RwStepKinematicsRwRollingSurfacePair {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsRollingSurfacePair>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 9, the_arch, "rolling_surface_pair")? {
            return Ok(());
        }
        rw_block_b_0007!(
            a_item_defined_transformation_description,
            a_item_defined_transformation_name,
            a_item_defined_transformation_transform_item1,
            a_item_defined_transformation_transform_item2,
            a_kinematic_pair_joint,
            a_representation_item_name,
            a_surface_pair_orientation,
            a_surface_pair_surface1,
            a_surface_pair_surface2,
            has_item_defined_transformation_description,
            the_arch,
            the_data,
            the_ent,
            the_num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsRollingSurfacePair>,
    ) -> Result<(), Failure> {
        rw_block_b_0049!(the_ent, the_sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsRollingSurfacePair>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0103!(iter, the_ent);
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwRollingSurfacePairValue,
    "RwStepKinematicsRwRollingSurfacePairValue"
);
impl RwStepKinematicsRwRollingSurfacePairValue {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsRollingSurfacePairValue>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 4, the_arch, "rolling_surface_pair_value")? {
            return Ok(());
        }
        rw_block_b_0089!(
            a_actual_point_on_surface,
            a_pair_value_applies_to_pair,
            a_representation_item_name,
            the_arch,
            the_data,
            the_num
        );
        let mut a_actual_rotation: f64 = Default::default();
        the_data.read_real(
            the_num,
            4,
            "actual_rotation",
            the_arch,
            &mut a_actual_rotation,
        )?;
        the_ent.init(
            &a_representation_item_name,
            &a_pair_value_applies_to_pair,
            &a_actual_point_on_surface,
            a_actual_rotation,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsRollingSurfacePairValue>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.name())?;
        the_sw.send_entity(&the_ent.applies_to_pair())?;
        the_sw.send_entity(&the_ent.actual_point_on_surface())?;
        the_sw.send_real(the_ent.actual_rotation())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsRollingSurfacePairValue>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&the_ent.parent.applies_to_pair().cast());
        iter.add_item(&the_ent.actual_point_on_surface().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwRotationAboutDirection,
    "RwStepKinematicsRwRotationAboutDirection"
);
impl RwStepKinematicsRwRotationAboutDirection {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsRotationAboutDirection>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 3, the_arch, "rotation_about_direction")? {
            return Ok(());
        }
        let mut a_representation_item_name: Handle<HAsciiString> = Default::default();
        the_data.read_string(
            the_num,
            1,
            "representation_item.name",
            the_arch,
            &mut a_representation_item_name,
        )?;
        let mut a_direction_of_axis: Handle<StepGeomDirection> = Default::default();
        the_data.read_entity(
            the_num,
            2,
            "direction_of_axis",
            the_arch,
            StepGeomDirection::static_type(),
            &mut a_direction_of_axis,
        )?;
        let mut a_rotation_angle: f64 = Default::default();
        the_data.read_real(
            the_num,
            3,
            "rotation_angle",
            the_arch,
            &mut a_rotation_angle,
        )?;
        the_ent.init(
            &a_representation_item_name,
            &a_direction_of_axis,
            a_rotation_angle,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsRotationAboutDirection>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.name())?;
        the_sw.send_entity(&the_ent.direction_of_axis())?;
        the_sw.send_real(the_ent.rotation_angle())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsRotationAboutDirection>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&the_ent.direction_of_axis().cast());
        Ok(())
    }
}

step_rw_class!(RwStepKinematicsRwScrewPair, "RwStepKinematicsRwScrewPair");
impl RwStepKinematicsRwScrewPair {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsScrewPair>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 7, the_arch, "screw_pair")? {
            return Ok(());
        }
        rw_block_b_0022!(
            a_item_defined_transformation_description,
            a_item_defined_transformation_name,
            a_item_defined_transformation_transform_item1,
            a_item_defined_transformation_transform_item2,
            a_kinematic_pair_joint,
            a_representation_item_name,
            has_item_defined_transformation_description,
            the_arch,
            the_data,
            the_num
        );
        let mut a_pitch: f64 = Default::default();
        the_data.read_real(the_num, 7, "pitch", the_arch, &mut a_pitch)?;
        the_ent.init(
            &a_representation_item_name,
            &a_item_defined_transformation_name,
            has_item_defined_transformation_description,
            &a_item_defined_transformation_description,
            &a_item_defined_transformation_transform_item1,
            &a_item_defined_transformation_transform_item2,
            &a_kinematic_pair_joint,
            a_pitch,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsScrewPair>,
    ) -> Result<(), Failure> {
        rw_block_b_0072!(the_ent, the_sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsScrewPair>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0176!(iter, the_ent);
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwScrewPairValue,
    "RwStepKinematicsRwScrewPairValue"
);
impl RwStepKinematicsRwScrewPairValue {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsScrewPairValue>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 3, the_arch, "screw_pair_value")? {
            return Ok(());
        }
        rw_block_b_0076!(
            a_actual_rotation,
            a_pair_value_applies_to_pair,
            a_representation_item_name,
            the_arch,
            the_data,
            the_ent,
            the_num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsScrewPairValue>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.name())?;
        the_sw.send_entity(&the_ent.applies_to_pair())?;
        the_sw.send_real(the_ent.actual_rotation())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsScrewPairValue>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&the_ent.parent.applies_to_pair().cast());
        Ok(())
    }
}
