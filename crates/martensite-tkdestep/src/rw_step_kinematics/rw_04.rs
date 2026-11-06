#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepKinematicsRwGearPair, "RwStepKinematicsRwGearPair");
impl RwStepKinematicsRwGearPair {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsGearPair>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 11, the_arch, "gear_pair")? {
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
        let mut a_radius_first_link: f64 = Default::default();
        the_data.read_real(
            the_num,
            7,
            "radius_first_link",
            the_arch,
            &mut a_radius_first_link,
        )?;
        let mut a_radius_second_link: f64 = Default::default();
        the_data.read_real(
            the_num,
            8,
            "radius_second_link",
            the_arch,
            &mut a_radius_second_link,
        )?;
        let mut a_bevel: f64 = Default::default();
        the_data.read_real(the_num, 9, "bevel", the_arch, &mut a_bevel)?;
        let mut a_helical_angle: f64 = Default::default();
        the_data.read_real(the_num, 10, "helical_angle", the_arch, &mut a_helical_angle)?;
        let mut a_gear_ratio: f64 = Default::default();
        the_data.read_real(the_num, 11, "gear_ratio", the_arch, &mut a_gear_ratio)?;
        the_ent.init(
            &a_representation_item_name,
            &a_item_defined_transformation_name,
            has_item_defined_transformation_description,
            &a_item_defined_transformation_description,
            &a_item_defined_transformation_transform_item1,
            &a_item_defined_transformation_transform_item2,
            &a_kinematic_pair_joint,
            a_radius_first_link,
            a_radius_second_link,
            a_bevel,
            a_helical_angle,
            a_gear_ratio,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsGearPair>,
    ) -> Result<(), Failure> {
        rw_block_b_0032!(the_ent, the_sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsGearPair>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0176!(iter, the_ent);
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwGearPairValue,
    "RwStepKinematicsRwGearPairValue"
);
impl RwStepKinematicsRwGearPairValue {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsGearPairValue>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 3, the_arch, "gear_pair_value")? {
            return Ok(());
        }
        rw_block_b_0139!(
            a_pair_value_applies_to_pair,
            a_representation_item_name,
            the_arch,
            the_data,
            the_num
        );
        let mut a_actual_rotation1: f64 = Default::default();
        the_data.read_real(
            the_num,
            3,
            "actual_rotation1",
            the_arch,
            &mut a_actual_rotation1,
        )?;
        the_ent.init(
            &a_representation_item_name,
            &a_pair_value_applies_to_pair,
            a_actual_rotation1,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsGearPairValue>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.name())?;
        the_sw.send_entity(&the_ent.applies_to_pair())?;
        the_sw.send_real(the_ent.actual_rotation1())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsGearPairValue>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&the_ent.parent.applies_to_pair().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwGearPairWithRange,
    "RwStepKinematicsRwGearPairWithRange"
);
impl RwStepKinematicsRwGearPairWithRange {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsGearPairWithRange>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 13, the_arch, "gear_pair_with_range")? {
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
        let mut a_gear_pair_radius_first_link: f64 = Default::default();
        the_data.read_real(
            the_num,
            7,
            "gear_pair.radius_first_link",
            the_arch,
            &mut a_gear_pair_radius_first_link,
        )?;
        let mut a_gear_pair_radius_second_link: f64 = Default::default();
        the_data.read_real(
            the_num,
            8,
            "gear_pair.radius_second_link",
            the_arch,
            &mut a_gear_pair_radius_second_link,
        )?;
        let mut a_gear_pair_bevel: f64 = Default::default();
        the_data.read_real(
            the_num,
            9,
            "gear_pair.bevel",
            the_arch,
            &mut a_gear_pair_bevel,
        )?;
        let mut a_gear_pair_helical_angle: f64 = Default::default();
        the_data.read_real(
            the_num,
            10,
            "gear_pair.helical_angle",
            the_arch,
            &mut a_gear_pair_helical_angle,
        )?;
        let mut a_gear_pair_gear_ratio: f64 = Default::default();
        the_data.read_real(
            the_num,
            11,
            "gear_pair.gear_ratio",
            the_arch,
            &mut a_gear_pair_gear_ratio,
        )?;
        let (a_lower_limit_actual_rotation1, has_lower_limit_actual_rotation1) =
            step_read_optional_real(
                the_data,
                the_num,
                12,
                "lower_limit_actual_rotation1",
                the_arch,
            )?;
        let (a_upper_limit_actual_rotation1, has_upper_limit_actual_rotation1) =
            step_read_optional_real(
                the_data,
                the_num,
                13,
                "upper_limit_actual_rotation1",
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
            a_gear_pair_radius_first_link,
            a_gear_pair_radius_second_link,
            a_gear_pair_bevel,
            a_gear_pair_helical_angle,
            a_gear_pair_gear_ratio,
            has_lower_limit_actual_rotation1,
            a_lower_limit_actual_rotation1,
            has_upper_limit_actual_rotation1,
            a_upper_limit_actual_rotation1,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsGearPairWithRange>,
    ) -> Result<(), Failure> {
        rw_block_b_0032!(the_ent, the_sw);
        step_send_optional_real(
            the_sw,
            the_ent.has_lower_limit_actual_rotation1(),
            the_ent.lower_limit_actual_rotation1(),
        )?;
        step_send_optional_real(
            the_sw,
            the_ent.has_upper_limit_actual_rotation1(),
            the_ent.upper_limit_actual_rotation1(),
        )?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsGearPairWithRange>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0175!(iter, the_ent);
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwHomokineticPair,
    "RwStepKinematicsRwHomokineticPair"
);
impl RwStepKinematicsRwHomokineticPair {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsHomokineticPair>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 13, the_arch, "homokinetic_pair")? {
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
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsHomokineticPair>,
    ) -> Result<(), Failure> {
        rw_block_b_0023!(the_ent, the_sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsHomokineticPair>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0175!(iter, the_ent);
        Ok(())
    }
}
