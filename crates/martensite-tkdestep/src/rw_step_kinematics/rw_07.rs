#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepKinematicsRwLowOrderKinematicPairValue,
    "RwStepKinematicsRwLowOrderKinematicPairValue"
);
impl RwStepKinematicsRwLowOrderKinematicPairValue {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsLowOrderKinematicPairValue>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 8, the_arch, "low_order_kinematic_pair_value")? {
            return Ok(());
        }
        rw_block_b_0139!(
            a_pair_value_applies_to_pair,
            a_representation_item_name,
            the_arch,
            the_data,
            the_num
        );
        let mut a_actual_translation_x: f64 = Default::default();
        the_data.read_real(
            the_num,
            3,
            "actual_translation_x",
            the_arch,
            &mut a_actual_translation_x,
        )?;
        let mut a_actual_translation_y: f64 = Default::default();
        the_data.read_real(
            the_num,
            4,
            "actual_translation_y",
            the_arch,
            &mut a_actual_translation_y,
        )?;
        let mut a_actual_translation_z: f64 = Default::default();
        the_data.read_real(
            the_num,
            5,
            "actual_translation_z",
            the_arch,
            &mut a_actual_translation_z,
        )?;
        let mut a_actual_rotation_x: f64 = Default::default();
        the_data.read_real(
            the_num,
            6,
            "actual_rotation_x",
            the_arch,
            &mut a_actual_rotation_x,
        )?;
        let mut a_actual_rotation_y: f64 = Default::default();
        the_data.read_real(
            the_num,
            7,
            "actual_rotation_y",
            the_arch,
            &mut a_actual_rotation_y,
        )?;
        let mut a_actual_rotation_z: f64 = Default::default();
        the_data.read_real(
            the_num,
            8,
            "actual_rotation_z",
            the_arch,
            &mut a_actual_rotation_z,
        )?;
        the_ent.init(
            &a_representation_item_name,
            &a_pair_value_applies_to_pair,
            a_actual_translation_x,
            a_actual_translation_y,
            a_actual_translation_z,
            a_actual_rotation_x,
            a_actual_rotation_y,
            a_actual_rotation_z,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsLowOrderKinematicPairValue>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.name())?;
        the_sw.send_entity(&the_ent.applies_to_pair())?;
        the_sw.send_real(the_ent.actual_translation_x())?;
        the_sw.send_real(the_ent.actual_translation_y())?;
        the_sw.send_real(the_ent.actual_translation_z())?;
        the_sw.send_real(the_ent.actual_rotation_x())?;
        the_sw.send_real(the_ent.actual_rotation_y())?;
        the_sw.send_real(the_ent.actual_rotation_z())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsLowOrderKinematicPairValue>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&the_ent.parent.applies_to_pair().cast());
        Ok(())
    }
}
