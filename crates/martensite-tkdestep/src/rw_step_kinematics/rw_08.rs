#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepKinematicsRwLowOrderKinematicPairWithRange,
    "RwStepKinematicsRwLowOrderKinematicPairWithRange"
);
impl RwStepKinematicsRwLowOrderKinematicPairWithRange {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsLowOrderKinematicPairWithRange>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(
            the_num,
            24,
            the_arch,
            "low_order_kinematic_pair_with_range",
        )? {
            return Ok(());
        }
        rw_block_b_0005!(
            a_item_defined_transformation_description,
            a_item_defined_transformation_name,
            a_item_defined_transformation_transform_item1,
            a_item_defined_transformation_transform_item2,
            a_kinematic_pair_joint,
            a_low_order_kinematic_pair_rx,
            a_low_order_kinematic_pair_ry,
            a_low_order_kinematic_pair_rz,
            a_low_order_kinematic_pair_tx,
            a_low_order_kinematic_pair_ty,
            a_low_order_kinematic_pair_tz,
            a_representation_item_name,
            has_item_defined_transformation_description,
            the_arch,
            the_data,
            the_num
        );
        let mut a_lower_limit_actual_rotation_x: f64 = Default::default();
        let mut has_lower_limit_actual_rotation_x: bool = true;
        if the_data.is_param_defined(the_num, 13)? {
            the_data.read_real(
                the_num,
                13,
                "lower_limit_actual_rotation_x",
                the_arch,
                &mut a_lower_limit_actual_rotation_x,
            )?;
        } else {
            has_lower_limit_actual_rotation_x = false;
            a_lower_limit_actual_rotation_x = 0.0;
        }
        let mut a_upper_limit_actual_rotation_x: f64 = Default::default();
        let mut has_upper_limit_actual_rotation_x: bool = true;
        if the_data.is_param_defined(the_num, 14)? {
            the_data.read_real(
                the_num,
                14,
                "upper_limit_actual_rotation_x",
                the_arch,
                &mut a_upper_limit_actual_rotation_x,
            )?;
        } else {
            has_upper_limit_actual_rotation_x = false;
            a_upper_limit_actual_rotation_x = 0.0;
        }
        let mut a_lower_limit_actual_rotation_y: f64 = Default::default();
        let mut has_lower_limit_actual_rotation_y: bool = true;
        if the_data.is_param_defined(the_num, 15)? {
            the_data.read_real(
                the_num,
                15,
                "lower_limit_actual_rotation_y",
                the_arch,
                &mut a_lower_limit_actual_rotation_y,
            )?;
        } else {
            has_lower_limit_actual_rotation_y = false;
            a_lower_limit_actual_rotation_y = 0.0;
        }
        let mut a_upper_limit_actual_rotation_y: f64 = Default::default();
        let mut has_upper_limit_actual_rotation_y: bool = true;
        if the_data.is_param_defined(the_num, 16)? {
            the_data.read_real(
                the_num,
                16,
                "upper_limit_actual_rotation_y",
                the_arch,
                &mut a_upper_limit_actual_rotation_y,
            )?;
        } else {
            has_upper_limit_actual_rotation_y = false;
            a_upper_limit_actual_rotation_y = 0.0;
        }
        let mut a_lower_limit_actual_rotation_z: f64 = Default::default();
        let mut has_lower_limit_actual_rotation_z: bool = true;
        if the_data.is_param_defined(the_num, 17)? {
            the_data.read_real(
                the_num,
                17,
                "lower_limit_actual_rotation_z",
                the_arch,
                &mut a_lower_limit_actual_rotation_z,
            )?;
        } else {
            has_lower_limit_actual_rotation_z = false;
            a_lower_limit_actual_rotation_z = 0.0;
        }
        let mut a_upper_limit_actual_rotation_z: f64 = Default::default();
        let mut has_upper_limit_actual_rotation_z: bool = true;
        if the_data.is_param_defined(the_num, 18)? {
            the_data.read_real(
                the_num,
                18,
                "upper_limit_actual_rotation_z",
                the_arch,
                &mut a_upper_limit_actual_rotation_z,
            )?;
        } else {
            has_upper_limit_actual_rotation_z = false;
            a_upper_limit_actual_rotation_z = 0.0;
        }
        let mut a_lower_limit_actual_translation_x: f64 = Default::default();
        let mut has_lower_limit_actual_translation_x: bool = true;
        if the_data.is_param_defined(the_num, 19)? {
            the_data.read_real(
                the_num,
                19,
                "lower_limit_actual_translation_x",
                the_arch,
                &mut a_lower_limit_actual_translation_x,
            )?;
        } else {
            has_lower_limit_actual_translation_x = false;
            a_lower_limit_actual_translation_x = 0.0;
        }
        let mut a_upper_limit_actual_translation_x: f64 = Default::default();
        let mut has_upper_limit_actual_translation_x: bool = true;
        if the_data.is_param_defined(the_num, 20)? {
            the_data.read_real(
                the_num,
                20,
                "upper_limit_actual_translation_x",
                the_arch,
                &mut a_upper_limit_actual_translation_x,
            )?;
        } else {
            has_upper_limit_actual_translation_x = false;
            a_upper_limit_actual_translation_x = 0.0;
        }
        let mut a_lower_limit_actual_translation_y: f64 = Default::default();
        let mut has_lower_limit_actual_translation_y: bool = true;
        if the_data.is_param_defined(the_num, 21)? {
            the_data.read_real(
                the_num,
                21,
                "lower_limit_actual_translation_y",
                the_arch,
                &mut a_lower_limit_actual_translation_y,
            )?;
        } else {
            has_lower_limit_actual_translation_y = false;
            a_lower_limit_actual_translation_y = 0.0;
        }
        let mut a_upper_limit_actual_translation_y: f64 = Default::default();
        let mut has_upper_limit_actual_translation_y: bool = true;
        if the_data.is_param_defined(the_num, 22)? {
            the_data.read_real(
                the_num,
                22,
                "upper_limit_actual_translation_y",
                the_arch,
                &mut a_upper_limit_actual_translation_y,
            )?;
        } else {
            has_upper_limit_actual_translation_y = false;
            a_upper_limit_actual_translation_y = 0.0;
        }
        let mut a_lower_limit_actual_translation_z: f64 = Default::default();
        let mut has_lower_limit_actual_translation_z: bool = true;
        if the_data.is_param_defined(the_num, 23)? {
            the_data.read_real(
                the_num,
                23,
                "lower_limit_actual_translation_z",
                the_arch,
                &mut a_lower_limit_actual_translation_z,
            )?;
        } else {
            has_lower_limit_actual_translation_z = false;
            a_lower_limit_actual_translation_z = 0.0;
        }
        let mut a_upper_limit_actual_translation_z: f64 = Default::default();
        let mut has_upper_limit_actual_translation_z: bool = true;
        if the_data.is_param_defined(the_num, 24)? {
            the_data.read_real(
                the_num,
                24,
                "upper_limit_actual_translation_z",
                the_arch,
                &mut a_upper_limit_actual_translation_z,
            )?;
        } else {
            has_upper_limit_actual_translation_z = false;
            a_upper_limit_actual_translation_z = 0.0;
        }
        the_ent.init(
            &a_representation_item_name,
            &a_item_defined_transformation_name,
            has_item_defined_transformation_description,
            &a_item_defined_transformation_description,
            &a_item_defined_transformation_transform_item1,
            &a_item_defined_transformation_transform_item2,
            &a_kinematic_pair_joint,
            a_low_order_kinematic_pair_tx,
            a_low_order_kinematic_pair_ty,
            a_low_order_kinematic_pair_tz,
            a_low_order_kinematic_pair_rx,
            a_low_order_kinematic_pair_ry,
            a_low_order_kinematic_pair_rz,
            has_lower_limit_actual_rotation_x,
            a_lower_limit_actual_rotation_x,
            has_upper_limit_actual_rotation_x,
            a_upper_limit_actual_rotation_x,
            has_lower_limit_actual_rotation_y,
            a_lower_limit_actual_rotation_y,
            has_upper_limit_actual_rotation_y,
            a_upper_limit_actual_rotation_y,
            has_lower_limit_actual_rotation_z,
            a_lower_limit_actual_rotation_z,
            has_upper_limit_actual_rotation_z,
            a_upper_limit_actual_rotation_z,
            has_lower_limit_actual_translation_x,
            a_lower_limit_actual_translation_x,
            has_upper_limit_actual_translation_x,
            a_upper_limit_actual_translation_x,
            has_lower_limit_actual_translation_y,
            a_lower_limit_actual_translation_y,
            has_upper_limit_actual_translation_y,
            a_upper_limit_actual_translation_y,
            has_lower_limit_actual_translation_z,
            a_lower_limit_actual_translation_z,
            has_upper_limit_actual_translation_z,
            a_upper_limit_actual_translation_z,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsLowOrderKinematicPairWithRange>,
    ) -> Result<(), Failure> {
        rw_block_b_0029!(the_ent, the_sw);
        if the_ent.has_lower_limit_actual_rotation_x() {
            the_sw.send_real(the_ent.lower_limit_actual_rotation_x())?;
        } else {
            the_sw.send_undef()?;
        }
        if the_ent.has_upper_limit_actual_rotation_x() {
            the_sw.send_real(the_ent.upper_limit_actual_rotation_x())?;
        } else {
            the_sw.send_undef()?;
        }
        if the_ent.has_lower_limit_actual_rotation_y() {
            the_sw.send_real(the_ent.lower_limit_actual_rotation_y())?;
        } else {
            the_sw.send_undef()?;
        }
        if the_ent.has_upper_limit_actual_rotation_y() {
            the_sw.send_real(the_ent.upper_limit_actual_rotation_y())?;
        } else {
            the_sw.send_undef()?;
        }
        if the_ent.has_lower_limit_actual_rotation_z() {
            the_sw.send_real(the_ent.lower_limit_actual_rotation_z())?;
        } else {
            the_sw.send_undef()?;
        }
        if the_ent.has_upper_limit_actual_rotation_z() {
            the_sw.send_real(the_ent.upper_limit_actual_rotation_z())?;
        } else {
            the_sw.send_undef()?;
        }
        rw_block_b_0126!(the_ent, the_sw);
        if the_ent.has_lower_limit_actual_translation_z() {
            the_sw.send_real(the_ent.lower_limit_actual_translation_z())?;
        } else {
            the_sw.send_undef()?;
        }
        if the_ent.has_upper_limit_actual_translation_z() {
            the_sw.send_real(the_ent.upper_limit_actual_translation_z())?;
        } else {
            the_sw.send_undef()?;
        }
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsLowOrderKinematicPairWithRange>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0176!(iter, the_ent);
        Ok(())
    }
}
