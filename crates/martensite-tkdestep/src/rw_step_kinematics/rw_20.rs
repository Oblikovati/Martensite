#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepKinematicsRwUniversalPair,
    "RwStepKinematicsRwUniversalPair"
);
impl RwStepKinematicsRwUniversalPair {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsUniversalPair>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 13, the_arch, "universal_pair")? {
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
        let (a_input_skew_angle, has_input_skew_angle) =
            step_read_optional_real(the_data, the_num, 13, "input_skew_angle", the_arch)?;
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
            has_input_skew_angle,
            a_input_skew_angle,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsUniversalPair>,
    ) -> Result<(), Failure> {
        rw_block_b_0023!(the_ent, the_sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsUniversalPair>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0176!(iter, the_ent);
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwUniversalPairValue,
    "RwStepKinematicsRwUniversalPairValue"
);
impl RwStepKinematicsRwUniversalPairValue {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsUniversalPairValue>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 4, the_arch, "universal_pair_value")? {
            return Ok(());
        }
        rw_block_b_0139!(
            a_pair_value_applies_to_pair,
            a_representation_item_name,
            the_arch,
            the_data,
            the_num
        );
        let mut a_first_rotation_angle: f64 = Default::default();
        the_data.read_real(
            the_num,
            3,
            "first_rotation_angle",
            the_arch,
            &mut a_first_rotation_angle,
        )?;
        let mut a_second_rotation_angle: f64 = Default::default();
        the_data.read_real(
            the_num,
            4,
            "second_rotation_angle",
            the_arch,
            &mut a_second_rotation_angle,
        )?;
        the_ent.init(
            &a_representation_item_name,
            &a_pair_value_applies_to_pair,
            a_first_rotation_angle,
            a_second_rotation_angle,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsUniversalPairValue>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.name())?;
        the_sw.send_entity(&the_ent.applies_to_pair())?;
        the_sw.send_real(the_ent.first_rotation_angle())?;
        the_sw.send_real(the_ent.second_rotation_angle())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsUniversalPairValue>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&the_ent.parent.applies_to_pair().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwUniversalPairWithRange,
    "RwStepKinematicsRwUniversalPairWithRange"
);
impl RwStepKinematicsRwUniversalPairWithRange {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsUniversalPairWithRange>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 17, the_arch, "universal_pair_with_range")? {
            return Ok(());
        }
        rw_block_b_0003!(
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
            a_universal_pair_input_skew_angle,
            has_item_defined_transformation_description,
            has_universal_pair_input_skew_angle,
            the_arch,
            the_data,
            the_num
        );
        let (a_lower_limit_first_rotation, has_lower_limit_first_rotation) =
            step_read_optional_real(
                the_data,
                the_num,
                14,
                "lower_limit_first_rotation",
                the_arch,
            )?;
        let (a_upper_limit_first_rotation, has_upper_limit_first_rotation) =
            step_read_optional_real(
                the_data,
                the_num,
                15,
                "upper_limit_first_rotation",
                the_arch,
            )?;
        let (a_lower_limit_second_rotation, has_lower_limit_second_rotation) =
            step_read_optional_real(
                the_data,
                the_num,
                16,
                "lower_limit_second_rotation",
                the_arch,
            )?;
        let (a_upper_limit_second_rotation, has_upper_limit_second_rotation) =
            step_read_optional_real(
                the_data,
                the_num,
                17,
                "upper_limit_second_rotation",
                the_arch,
            )?;
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
            has_universal_pair_input_skew_angle,
            a_universal_pair_input_skew_angle,
            has_lower_limit_first_rotation,
            a_lower_limit_first_rotation,
            has_upper_limit_first_rotation,
            a_upper_limit_first_rotation,
            has_lower_limit_second_rotation,
            a_lower_limit_second_rotation,
            has_upper_limit_second_rotation,
            a_upper_limit_second_rotation,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsUniversalPairWithRange>,
    ) -> Result<(), Failure> {
        rw_block_b_0023!(the_ent, the_sw);
        step_send_optional_real(
            the_sw,
            the_ent.has_lower_limit_first_rotation(),
            the_ent.lower_limit_first_rotation(),
        )?;
        step_send_optional_real(
            the_sw,
            the_ent.has_upper_limit_first_rotation(),
            the_ent.upper_limit_first_rotation(),
        )?;
        step_send_optional_real(
            the_sw,
            the_ent.has_lower_limit_second_rotation(),
            the_ent.lower_limit_second_rotation(),
        )?;
        step_send_optional_real(
            the_sw,
            the_ent.has_upper_limit_second_rotation(),
            the_ent.upper_limit_second_rotation(),
        )?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsUniversalPairWithRange>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0175!(iter, the_ent);
        Ok(())
    }
}
